//! Portable region storage. Blocks own ranges; regions are released only after all ranges retire.
use crate::{Error, Result};
use portable_atomic::AtomicU64;
use std::alloc::{Layout, alloc, dealloc};
use std::collections::{BTreeMap, BTreeSet};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

#[path = "evacuation.rs"]
mod evacuation;
use crate::collector::try_lock;
use evacuation::{Evacuation, Selection};

#[cfg(test)]
#[path = "region_shard_tests.rs"]
mod shard_tests;

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Physical storage statistics, distinct from logical Java allocation accounting.
#[derive(Clone, Copy, Debug, Default)]
pub struct RegionStatistics {
    /// Number of independently locked allocation registries.
    pub registry_shards: usize,
    pub committed_bytes: usize,
    pub occupied_bytes: usize,
    pub regions: usize,
    pub bytes_copied: u64,
    /// Bytes copied from host storage during payload preparation.
    pub bytes_staged: u64,
    /// Relocation bytes copied again after mutation invalidated an earlier copy.
    pub bytes_recopied: u64,
    pub destination_failures: u64,
    pub relocations: u64,
    pub regions_released: u64,
    /// Shared payload/descriptor backing, including in-progress reservations.
    pub shared_committed_bytes: usize,
    pub peak_shared_committed_bytes: usize,
    pub relocation_reserve_bytes: usize,
    pub relocation_reserve_used_bytes: usize,
    pub peak_relocation_reserve_used_bytes: usize,
    pub regions_selected: u64,
    pub selected_occupied_bytes: u64,
    pub selection_skips: u64,
    /// Retained cost-index vector storage, outside the arena backing cap.
    pub selection_capacity_bytes: usize,
    pub pinned_region_skips: u64,
    pub hot_region_skips: u64,
    /// Gross source backing freed; destination backing remains in committed bytes.
    pub evacuation_regions_released: u64,
    pub evacuation_bytes_released: u64,
}

/// Physical backing can outlive its last registered range. Its destructor
/// updates this accounting after deallocation without reacquiring the registry.
#[derive(Debug, Default)]
struct BackingTotals {
    committed: AtomicUsize,
    released: AtomicU64,
    evacuated_regions: AtomicU64,
    evacuated_bytes: AtomicU64,
}

#[derive(Debug)]
struct Registry {
    state: Weak<Mutex<State>>,
    totals: Arc<BackingTotals>,
}

#[derive(Debug)]
struct State {
    regions: BTreeMap<u64, Arc<Region>>,
    // Largest free range first narrows candidates; Region::allocate checks alignment.
    available: BTreeSet<(usize, u64)>,
    next: u64,
    backing: Arc<BackingTotals>,
    registered: usize,
    occupied: usize,
    trim_cursor: u64,
    evacuation: Option<Evacuation>,
}

impl State {
    /// Unregister empty backing without releasing its physical charge. An escaped
    /// range owner can still retain it; `Region::drop` accounts for the actual free.
    fn remove_region(&mut self, id: u64, only_selected: bool) -> Option<Arc<Region>> {
        let region = self.regions.get(&id)?;
        debug_assert_eq!(region.occupied(), 0);
        // End can invalidate the epoch while this registry is locked. Use one
        // source decision for both eager retirement and its physical-release metrics.
        let selected = self
            .evacuation
            .as_ref()
            .is_some_and(|plan| plan.source(region));
        if only_selected && !selected {
            return None;
        }
        let region = self.regions.remove(&id)?;
        // All ranges have returned and coalesced across the whole backing.
        self.available.remove(&(region.layout.size(), id));
        self.registered -= region.layout.size();
        if selected {
            region.evacuated.store(true, Ordering::Relaxed);
        }
        Some(region)
    }
}

/// An indexed, nonblocking sample for collection scheduling. Free extents
/// remain subject to alignment and concurrent allocation; this is not a promise.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Pressure {
    pub(crate) committed: usize,
    pub(crate) free: usize,
    pub(crate) largest_free: usize,
}

/// Shared cap for arena backing, including reservations not yet registered.
#[derive(Debug)]
pub(crate) struct Budget {
    used: AtomicUsize,
    released: AtomicU64,
    limit: usize,
    reserve: usize,
    peak_reserve_used: AtomicUsize,
    peak_used: AtomicUsize,
}
impl Budget {
    pub(crate) fn new(limit: usize) -> Arc<Self> {
        Self::with_reserve(limit, 0)
    }
    pub(crate) fn with_reserve(limit: usize, reserve: usize) -> Arc<Self> {
        Arc::new(Self {
            used: AtomicUsize::new(0),
            released: AtomicU64::new(0),
            limit,
            reserve: reserve.min(limit / 2),
            peak_reserve_used: AtomicUsize::new(0),
            peak_used: AtomicUsize::new(0),
        })
    }
    pub(crate) fn reserve(
        self: &Arc<Self>,
        size: usize,
        relocation: bool,
    ) -> Result<BackingCredit> {
        let limit = if relocation {
            self.limit
        } else {
            self.limit.saturating_sub(self.reserve)
        };
        let previous = self
            .used
            .try_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(size).filter(|total| *total <= limit)
            })
            .map_err(|_| Error::OutOfMemory)?;
        self.peak_used.fetch_max(previous + size, Ordering::Relaxed);
        if relocation && self.reserve != 0 {
            self.peak_reserve_used.fetch_max(
                (previous + size).saturating_sub(self.limit - self.reserve),
                Ordering::Relaxed,
            );
        }
        Ok(BackingCredit {
            budget: self.clone(),
            size,
            reclamation: false,
        })
    }
}
#[derive(Debug)]
pub(crate) struct BackingCredit {
    budget: Arc<Budget>,
    size: usize,
    reclamation: bool,
}
impl Drop for BackingCredit {
    fn drop(&mut self) {
        self.budget.used.fetch_sub(self.size, Ordering::AcqRel);
        if self.reclamation {
            self.budget
                .released
                .fetch_add(self.size as u64, Ordering::Release);
        }
    }
}

/// Rolls back a backing allocation reservation on failure or unwinding.
struct Reservation {
    totals: Arc<BackingTotals>,
    bytes: usize,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        if self.bytes != 0 {
            self.totals
                .committed
                .fetch_sub(self.bytes, Ordering::AcqRel);
        }
    }
}

/// Region allocator shared by guarded Java payloads. Does not depend on virtual memory remapping.
#[derive(Debug)]
pub struct RegionHeap {
    states: Box<[Arc<Mutex<State>>]>,
    selection_shard: AtomicUsize,
    trim_shard: Mutex<usize>,
    selection: Option<Arc<Selection>>,
    budget: Arc<Budget>,
    returned_bytes: Arc<AtomicU64>,
    region_size: usize,
    limit: usize,
    reserve: usize,
    copied: AtomicU64,
    staged: AtomicU64,
    recopied: AtomicU64,
    destination_failures: AtomicU64,
    relocations: AtomicU64,
}

impl RegionHeap {
    /// Creates a bounded arena. `reserve` bytes remain unavailable to ordinary allocation.
    #[must_use]
    pub fn new(region_size: usize, limit: usize, reserve: usize) -> Arc<Self> {
        let budget = if reserve == 0 {
            Budget::new(limit)
        } else {
            Budget::with_reserve(limit, reserve)
        };
        Self::with_budget(region_size, limit, reserve, budget)
    }

    pub(crate) fn with_budget(
        region_size: usize,
        limit: usize,
        reserve: usize,
        budget: Arc<Budget>,
    ) -> Arc<Self> {
        Self::with_policy(region_size, limit, reserve, budget, false)
    }

    pub(crate) fn with_policy(
        region_size: usize,
        limit: usize,
        reserve: usize,
        budget: Arc<Budget>,
        cost_aware: bool,
    ) -> Arc<Self> {
        Self::with_shards(region_size, limit, reserve, budget, cost_aware, 1)
    }

    pub(crate) fn with_shards(
        region_size: usize,
        limit: usize,
        reserve: usize,
        budget: Arc<Budget>,
        cost_aware: bool,
        shards: usize,
    ) -> Arc<Self> {
        // All arena shards use the same physical cap as the shared payload and
        // descriptor budget. Per-shard reservations must not multiply that cap.
        let reserve = reserve.min(limit / 2);
        assert_eq!(limit, budget.limit);
        assert!(reserve <= budget.reserve);
        let count = crate::index::shard_count(shards);
        let selection = cost_aware.then(|| Arc::new(Selection::default()));
        Arc::new(Self {
            budget,
            returned_bytes: Arc::new(AtomicU64::new(0)),
            states: (0..count)
                .map(|index| {
                    Arc::new(Mutex::new(State {
                        regions: BTreeMap::new(),
                        available: BTreeSet::new(),
                        next: 1 + index as u64,
                        backing: Arc::default(),
                        registered: 0,
                        occupied: 0,
                        trim_cursor: 0,
                        evacuation: selection.as_ref().map(|s| Evacuation::new(s.clone())),
                    }))
                })
                .collect(),
            selection,
            selection_shard: AtomicUsize::new(0),
            trim_shard: Mutex::new(0),
            region_size: region_size.max(1).min(limit.saturating_sub(reserve).max(1)),
            limit,
            reserve,
            copied: AtomicU64::new(0),
            staged: AtomicU64::new(0),
            recopied: AtomicU64::new(0),
            destination_failures: AtomicU64::new(0),
            relocations: AtomicU64::new(0),
        })
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "only the low four bits select a registry on every target"
    )]
    fn index(&self, id: u64) -> usize {
        id.wrapping_sub(1) as usize & (self.states.len() - 1)
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "nonempty power-of-two registry count"
    )]
    fn shard(&self, index: usize) -> &Arc<Mutex<State>> {
        &self.states[index & (self.states.len() - 1)]
    }

    fn preferred_shard(&self) -> usize {
        // A stable per-thread preference reuses a partially filled region. It
        // owns no storage; migration and thread exit need no handoff or cleanup.
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        thread_local! {
            static INDEX: usize = NEXT.fetch_add(1, Ordering::Relaxed);
        }
        INDEX.with(|index| *index & (self.states.len() - 1))
    }

    pub(crate) fn allocate(&self, layout: Layout, source: Option<&Block>) -> Result<Block> {
        if source.is_some_and(|source| !self.owns(source) || source.len() == 0) {
            return Err(Error::OutOfMemory);
        }
        let result = self.allocate_range(layout, source).map(|mut block| {
            block.relocation = source.is_some();
            block
        });
        if source.is_some() && result.is_err() {
            self.destination_failures.fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    fn allocate_range(&self, layout: Layout, source: Option<&Block>) -> Result<Block> {
        let evacuating = if let Some(source) = source {
            // A nonempty block owns its range and prevents registry removal.
            // Validate its current role before admitting a destination.
            let epoch = self
                .selection
                .as_ref()
                .and_then(|selection| selection.epoch());
            if epoch.is_some_and(|epoch| !Evacuation::source_at(&source.region, epoch)) {
                return Err(Error::OutOfMemory);
            }
            epoch.is_some()
        } else {
            false
        };
        let before = source.map(Block::id);
        let preferred = self.preferred_shard();
        let busy = loop {
            let mut busy = 0_u16;
            // Look for existing space in every shard before allocating new backing.
            // A busy preferred shard does not block an available unrelated shard.
            for offset in 0..self.states.len() {
                let index = (preferred + offset) & (self.states.len() - 1);
                let mut state = match self.shard(index).try_lock() {
                    Ok(state) => state,
                    Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
                    Err(std::sync::TryLockError::WouldBlock) => {
                        busy |= 1 << index;
                        continue;
                    }
                };
                if let Some(block) = Self::allocate_existing(layout, before, evacuating, &mut state)
                {
                    return Ok(block);
                }
            }
            // Ordinary allocation can open backing on an accessible registry while a
            // peer is parked. Probe existing holes first, and leave relocation's
            // source and destination selection unchanged.
            if before.is_none() && busy != 0 {
                for offset in 0..self.states.len() {
                    let index = (preferred + offset) & (self.states.len() - 1);
                    if let Ok(state) = self.shard(index).try_lock() {
                        if let Ok(block) =
                            self.allocate_with_state(layout, None, false, index, state)
                        {
                            return Ok(block);
                        }
                        // At the shared cap a skipped shard may still have a hole.
                        break;
                    }
                }
            }
            if cfg!(not(target_family = "wasm"))
                && before.is_none()
                && busy != 0
                && self.states.len() > 1
            {
                // There may be no backing credit until a block is dropped. Waiting
                // on a particular busy shard would miss holes returned to its peers.
                // Back off with no registry lock held, then recheck all registries.
                #[cfg(all(test, not(target_family = "wasm")))]
                shard_tests::record_contended_retry();
                std::thread::sleep(std::time::Duration::from_micros(100));
                continue;
            }
            break busy;
        };
        // Contention is not evidence of exhaustion. Recheck skipped shards once
        // before reserving backing or returning OOM; never hold two shard locks.
        for index in 0..self.states.len() {
            if busy & (1 << index) != 0 {
                let mut state = lock(self.shard(index));
                if let Some(block) = Self::allocate_existing(layout, before, evacuating, &mut state)
                {
                    return Ok(block);
                }
            }
        }
        self.allocate_selected(layout, before, evacuating, preferred)
    }

    fn allocate_existing(
        layout: Layout,
        before: Option<u64>,
        evacuating: bool,
        state: &mut State,
    ) -> Option<Block> {
        for &(largest, id) in state.available.range((layout.size(), 0)..) {
            if before.is_some_and(|before| id == before || (!evacuating && id >= before)) {
                continue;
            }
            let Some(region) = state.regions.get(&id) else {
                continue;
            };
            if state
                .evacuation
                .as_ref()
                .is_some_and(|plan| plan.source(region))
            {
                continue;
            }
            if let Some(block) = region.allocate(layout) {
                let remaining = block.region.largest_free();
                state.occupied += layout.size();
                state.available.remove(&(largest, id));
                if remaining != 0 {
                    state.available.insert((remaining, id));
                }
                if let Some(plan) = &mut state.evacuation {
                    plan.destination(&block.region);
                }
                return Some(block);
            }
        }
        None
    }

    fn allocate_selected(
        &self,
        layout: Layout,
        before: Option<u64>,
        evacuating: bool,
        index: usize,
    ) -> Result<Block> {
        self.allocate_with_state(layout, before, evacuating, index, lock(self.shard(index)))
    }

    fn allocate_with_state(
        &self,
        layout: Layout,
        before: Option<u64>,
        evacuating: bool,
        index: usize,
        mut state: MutexGuard<'_, State>,
    ) -> Result<Block> {
        // Another allocator may have returned space since the bounded probe.
        if let Some(block) = Self::allocate_existing(layout, before, evacuating, &mut state) {
            return Ok(block);
        }
        // Packing only moves toward earlier regions. A failed selection leaves the source
        // untouched and cannot initiate an endless relocation between equally empty regions.
        if before.is_some() && !evacuating {
            return Err(Error::OutOfMemory);
        }
        let size = self.region_size.max(layout.size());
        let limit = if evacuating {
            self.limit
        } else {
            self.limit.saturating_sub(self.reserve)
        };
        let id = state.next;
        let next = id
            .checked_add(self.states.len() as u64)
            .ok_or(Error::OutOfMemory)?;
        state
            .backing
            .committed
            .try_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(size).filter(|total| *total <= limit)
            })
            .map_err(|_| Error::OutOfMemory)?;
        let mut reservation = Reservation {
            totals: state.backing.clone(),
            bytes: size,
        };
        let backing = self.budget.reserve(size, evacuating)?;
        state.next = next;
        let track_relocation = state.evacuation.is_some();
        drop(state);
        let region = Arc::new(Region::new(
            id,
            Layout::from_size_align(size, layout.align().max(64))
                .map_err(|_| Error::OutOfMemory)?,
            self.returned_bytes.clone(),
            Registry {
                state: Arc::downgrade(self.shard(index)),
                totals: reservation.totals.clone(),
            },
            backing,
            track_relocation,
            size > self.region_size,
        )?);
        let block = region.allocate(layout).ok_or(Error::OutOfMemory)?;
        let mut state = lock(self.shard(index));
        let remaining = region.largest_free();
        if remaining != 0 {
            state.available.insert((remaining, id));
        }
        state.regions.insert(id, region);
        state.registered += size;
        state.occupied += layout.size();
        block.region.registered.store(true, Ordering::Release);
        if let Some(plan) = &mut state.evacuation {
            plan.destination(&block.region);
        }
        reservation.bytes = 0;
        Ok(block)
    }

    pub(crate) fn owns(&self, block: &Block) -> bool {
        Arc::ptr_eq(&self.returned_bytes, &block.region.returned_bytes)
    }

    /// Conservative, nonblocking readiness for a physical waiter. This offers no
    /// ownership: another allocator can consume the range before the retry.
    pub(crate) fn can_allocate(&self, layout: Layout) -> bool {
        if let Some(span) = layout.size().checked_add(layout.align() - 1) {
            for shard in &self.states {
                let Ok(state) = shard.try_lock() else {
                    continue;
                };
                if state.available.range((span, 0)..).take(32).any(|(_, id)| {
                    state.regions.get(id).is_some_and(|region| {
                        !state
                            .evacuation
                            .as_ref()
                            .is_some_and(|plan| plan.source(region))
                    })
                }) {
                    return true;
                }
            }
        }
        let size = self.region_size.max(layout.size());
        Layout::from_size_align(size, layout.align().max(64)).is_ok()
            && self
                .budget
                .used
                .load(Ordering::Acquire)
                .checked_add(size)
                .is_some_and(|used| used <= self.budget.limit.saturating_sub(self.budget.reserve))
    }

    /// A partial destination from an older selection can now be a source itself.
    /// Restart it before copying, rather than committing into a current source.
    pub(crate) fn continue_relocation(&self, source: &Block, destination: &Block) -> bool {
        if !self.owns(source) || !self.owns(destination) {
            return false;
        }
        let Some(selection) = &self.selection else {
            return true;
        };
        // Both blocks retain their regions. Within one epoch a selected source
        // cannot be a destination and an assigned destination cannot be selected.
        selection.epoch().is_none_or(|epoch| {
            Evacuation::source_at(&source.region, epoch)
                && Evacuation::destination_at(&destination.region, epoch)
        })
    }

    pub(crate) fn begin_evacuation(&self) -> bool {
        self.selection.as_ref().is_some_and(|selection| {
            self.selection_shard.store(0, Ordering::Relaxed);
            selection.begin()
        })
    }

    /// A busy shard yields the helper. A successful probe reports whether selection is complete.
    pub(crate) fn select_evacuation(&self) -> Option<bool> {
        let Some(selection) = &self.selection else {
            return Some(true);
        };
        let Some(epoch) = selection.epoch() else {
            return Some(true);
        };
        let index = self.selection_shard.load(Ordering::Acquire);
        if let Some(shard) = self.states.get(index) {
            let mut state = try_lock(shard)?;
            let State {
                regions,
                evacuation,
                ..
            } = &mut *state;
            if let Some(plan) = evacuation {
                if plan.select(regions, epoch) {
                    self.end_evacuation();
                    return Some(true);
                }
                if plan.building() {
                    return Some(false);
                }
            }
            self.selection_shard.store(index + 1, Ordering::Release);
            return Some(false);
        }
        if selection.remaining.load(Ordering::Acquire) == 0 {
            return Some(true);
        }
        // Ranking requires a sample from every shard. Retry without choosing
        // a winner when one is busy, retaining the shared source quota.
        let mut next = None;
        for (index, shard) in self.states.iter().enumerate() {
            let state = try_lock(shard)?;
            if let Some(cost) = state.evacuation.as_ref().and_then(Evacuation::next_cost) {
                next = Some(next.map_or((cost, index), |best| std::cmp::min(best, (cost, index))));
            }
        }
        let Some((_, index)) = next else {
            return Some(true);
        };
        let mut state = try_lock(self.shard(index))?;
        let State {
            regions,
            evacuation,
            ..
        } = &mut *state;
        if let Some(plan) = evacuation {
            plan.select(regions, epoch);
        }
        Some(false)
    }

    pub(crate) fn end_evacuation(&self) {
        if let Some(selection) = &self.selection {
            selection.end();
        }
    }

    #[cfg(all(test, not(target_family = "wasm")))]
    pub(crate) fn evacuation_active(&self) -> bool {
        self.selection.as_ref().is_some_and(|s| s.epoch().is_some())
    }

    pub(crate) fn record_pin(&self, id: u64) {
        let state = lock(self.shard(self.index(id)));
        if let Some(plan) = &state.evacuation
            && let Some(region) = state.regions.get(&id)
        {
            plan.pin(region);
        }
    }

    /// Inspects at most `budget` regions, releasing empty ones. Returns true at the end of a pass.
    pub fn trim(&self, budget: usize) -> bool {
        self.trim_available(budget).unwrap_or(false)
    }

    fn trim_available(&self, budget: usize) -> Option<bool> {
        let mut index = try_lock(&self.trim_shard)?;
        let mut state = try_lock(self.shard(*index))?;
        for _ in 0..budget {
            let next = state
                .regions
                .range((
                    std::ops::Bound::Excluded(state.trim_cursor),
                    std::ops::Bound::Unbounded,
                ))
                .next()
                .map(|(id, region)| {
                    (
                        *id,
                        region.occupied() == 0 && Arc::strong_count(region) == 1,
                    )
                });
            let Some((id, empty)) = next else {
                state.trim_cursor = 0;
                *index += 1;
                if *index == self.states.len() {
                    *index = 0;
                    return Some(true);
                }
                drop(state);
                state = try_lock(self.shard(*index))?;
                continue;
            };
            state.trim_cursor = id;
            if empty && let Some(region) = state.remove_region(id, false) {
                // Backing deallocation can block in the system allocator.
                drop(state);
                drop(region);
                state = try_lock(self.shard(*index))?;
            }
        }
        Some(false)
    }

    /// Returns physical occupancy and relocation counters.
    #[must_use]
    pub fn statistics(&self) -> RegionStatistics {
        let mut result = RegionStatistics {
            registry_shards: self.states.len(),
            bytes_copied: self.copied.load(Ordering::Relaxed),
            bytes_staged: self.staged.load(Ordering::Relaxed),
            bytes_recopied: self.recopied.load(Ordering::Relaxed),
            destination_failures: self.destination_failures.load(Ordering::Relaxed),
            relocations: self.relocations.load(Ordering::Relaxed),
            shared_committed_bytes: self.budget.used.load(Ordering::Acquire),
            peak_shared_committed_bytes: self.budget.peak_used.load(Ordering::Relaxed),
            relocation_reserve_bytes: self.budget.reserve,
            relocation_reserve_used_bytes: self
                .budget
                .used
                .load(Ordering::Acquire)
                .saturating_sub(self.budget.limit - self.budget.reserve),
            peak_relocation_reserve_used_bytes: self
                .budget
                .peak_reserve_used
                .load(Ordering::Relaxed),
            ..RegionStatistics::default()
        };
        for shard in &self.states {
            let state = lock(shard);
            result.committed_bytes += state.backing.committed.load(Ordering::Acquire);
            result.occupied_bytes += state.occupied;
            result.regions += state.regions.len();
            result.regions_released += state.backing.released.load(Ordering::Acquire);
            result.regions_selected += state
                .evacuation
                .as_ref()
                .map_or(0, |plan| plan.selected_regions);
            result.selected_occupied_bytes += state
                .evacuation
                .as_ref()
                .map_or(0, |plan| plan.selected_bytes);
            result.selection_capacity_bytes += state
                .evacuation
                .as_ref()
                .map_or(0, Evacuation::capacity_bytes);
            result.selection_skips += state
                .evacuation
                .as_ref()
                .map_or(0, |plan| plan.selection_skips);
            result.pinned_region_skips +=
                state.evacuation.as_ref().map_or(0, |plan| plan.pin_skips);
            result.hot_region_skips += state.evacuation.as_ref().map_or(0, |plan| plan.hot_skips);
            result.evacuation_regions_released +=
                state.backing.evacuated_regions.load(Ordering::Acquire);
            result.evacuation_bytes_released +=
                state.backing.evacuated_bytes.load(Ordering::Acquire);
        }
        result
    }

    pub(crate) fn pressure(&self) -> Option<Pressure> {
        let mut result = Pressure::default();
        for shard in &self.states {
            let state = shard.try_lock().ok()?;
            result.committed += state.backing.committed.load(Ordering::Acquire);
            result.free += state.registered.saturating_sub(state.occupied);
            result.largest_free = result
                .largest_free
                .max(state.available.last().map_or(0, |(size, _)| *size));
        }
        Some(result)
    }

    /// Advisory progress survives another allocator consuming a returned range.
    /// Backing refunds from the descriptor arena also make payload capacity available.
    pub(crate) fn recovery_progress(&self) -> (u64, u64) {
        (
            self.returned_bytes.load(Ordering::Acquire),
            self.budget.released.load(Ordering::Acquire),
        )
    }

    pub(crate) fn record_staging(&self, bytes: usize) {
        self.staged.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    pub(crate) fn record_recopy(&self, bytes: usize) {
        if bytes != 0 {
            self.recopied.fetch_add(bytes as u64, Ordering::Relaxed);
        }
    }

    pub(crate) fn record_copy(&self, bytes: usize) {
        self.copied.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    pub(crate) fn record_relocation(&self) {
        self.relocations.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Debug)]
struct Region {
    registry: Registry,
    returned_bytes: Arc<AtomicU64>,
    id: u64,
    ptr: NonNull<u8>,
    layout: Layout,
    free: Mutex<FreeRanges>,
    occupied: AtomicUsize,
    registered: AtomicBool,
    relocation_role: AtomicU64,
    evacuated: AtomicBool,
    reclaimed: AtomicBool,
    oversized: bool,
    track_relocation: bool,
    mutations: AtomicU64,
    sampled_mutations: AtomicU64,
    pin_epoch: AtomicU64,
    backing: BackingCredit,
}

// SAFETY: the free map serializes allocation; each Block has exclusive ownership of its
// disjoint byte range. Payload access and relocation require the owner's managed guard.
unsafe impl Send for Region {}
unsafe impl Sync for Region {}

#[derive(Debug, Default)]
struct FreeRanges {
    by_offset: BTreeMap<usize, usize>,
    by_size: BTreeSet<(usize, usize)>,
}
impl FreeRanges {
    fn insert(&mut self, offset: usize, length: usize) {
        self.by_offset.insert(offset, length);
        self.by_size.insert((length, offset));
    }
    fn remove(&mut self, offset: usize) -> Option<usize> {
        let length = self.by_offset.remove(&offset)?;
        self.by_size.remove(&(length, offset));
        Some(length)
    }
}

impl Region {
    fn new(
        id: u64,
        layout: Layout,
        returned_bytes: Arc<AtomicU64>,
        registry: Registry,
        backing: BackingCredit,
        track_relocation: bool,
        oversized: bool,
    ) -> Result<Self> {
        let size = layout.size();
        // SAFETY: a valid, nonzero allocation layout.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or(Error::OutOfMemory)?;
        Ok(Self {
            registry,
            returned_bytes,
            id,
            ptr,
            layout,
            free: Mutex::new(FreeRanges {
                by_offset: BTreeMap::from([(0, size)]),
                by_size: BTreeSet::from([(size, 0)]),
            }),
            occupied: AtomicUsize::new(0),
            registered: AtomicBool::new(false),
            relocation_role: AtomicU64::new(0),
            evacuated: AtomicBool::new(false),
            reclaimed: AtomicBool::new(false),
            oversized,
            track_relocation,
            mutations: AtomicU64::new(0),
            sampled_mutations: AtomicU64::new(0),
            pin_epoch: AtomicU64::new(0),
            backing,
        })
    }

    fn occupied(&self) -> usize {
        self.occupied.load(Ordering::Acquire)
    }

    fn largest_free(&self) -> usize {
        lock(&self.free).by_size.last().map_or(0, |(size, _)| *size)
    }

    fn allocate(self: &Arc<Self>, layout: Layout) -> Option<Block> {
        let mut free = lock(&self.free);
        let (start, length, aligned) =
            free.by_size
                .range((layout.size(), 0)..)
                .find_map(|(length, start)| {
                    let absolute = (self.ptr.as_ptr() as usize).checked_add(*start)?;
                    let aligned = absolute.checked_add(layout.align() - 1)? & !(layout.align() - 1);
                    let aligned = aligned.checked_sub(self.ptr.as_ptr() as usize)?;
                    let end = aligned.checked_add(layout.size())?;
                    (end <= start.checked_add(*length)?).then_some((*start, *length, aligned))
                })?;
        free.remove(start);
        if aligned > start {
            free.insert(start, aligned - start);
        }
        let end = aligned + layout.size();
        if end < start + length {
            free.insert(end, start + length - end);
        }
        self.occupied.fetch_add(layout.size(), Ordering::Release);
        Some(Block {
            region: self.clone(),
            offset: aligned,
            length: layout.size(),
            relocation: false,
            returned: false,
        })
    }

    fn release(&self, free: &mut FreeRanges, offset: usize, length: usize) {
        let mut start = offset;
        let mut size = length;
        if let Some((&previous, &length)) = free.by_offset.range(..offset).next_back()
            && previous + length == offset
        {
            free.remove(previous);
            start = previous;
            size += length;
        }
        if let Some(length) = free.remove(start + size) {
            size += length;
        }
        free.insert(start, size);
        self.occupied.fetch_sub(length, Ordering::Release);
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        // SAFETY: all Block owners are gone; only Region owns this allocation and layout.
        unsafe { dealloc(self.ptr.as_ptr(), self.layout) };
        self.backing.reclamation = self.reclaimed.load(Ordering::Relaxed);
        if self.registered.load(Ordering::Acquire) {
            let totals = &self.registry.totals;
            totals
                .committed
                .fetch_sub(self.layout.size(), Ordering::AcqRel);
            totals.released.fetch_add(1, Ordering::Relaxed);
            if self.evacuated.load(Ordering::Relaxed) {
                totals.evacuated_regions.fetch_add(1, Ordering::Relaxed);
                totals
                    .evacuated_bytes
                    .fetch_add(self.layout.size() as u64, Ordering::Relaxed);
            }
        }
        // The shared BackingCredit field refunds after actual deallocation too.
    }
}

#[derive(Debug)]
pub(crate) struct Block {
    region: Arc<Region>,
    offset: usize,
    length: usize,
    // Raw destinations and transferred sources do not represent new capacity
    // from dead payloads. Counting them can make OOM recovery retry forever.
    relocation: bool,
    returned: bool,
}

impl Block {
    pub(crate) fn set_staging(&mut self, staging: bool) {
        self.relocation = staging;
    }

    pub(crate) fn transfer_ownership(&mut self, destination: &mut Self) {
        self.relocation = true;
        destination.relocation = false;
    }

    pub(crate) fn record_mutation(&self) {
        if self.region.track_relocation {
            self.region.mutations.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.length
    }

    /// Partition ownership without revisiting the heap's free indexes. Cached
    /// ranges start at 64-byte boundaries and charge padding to each partition.
    pub(crate) fn take_prefix(&mut self, layout: Layout) -> Option<Self> {
        if layout.align() > 64 || layout.size() == 0 || (self.ptr::<u8>() as usize) & 63 != 0 {
            return None;
        }
        let length = layout.size().checked_add(63)? & !63;
        if length > self.length {
            return None;
        }
        let block = Self {
            region: self.region.clone(),
            offset: self.offset,
            length,
            relocation: self.relocation,
            returned: false,
        };
        self.offset += length;
        self.length -= length;
        Some(block)
    }
    pub(crate) fn id(&self) -> u64 {
        self.region.id
    }
    pub(crate) fn ptr<T>(&self) -> *mut T {
        // SAFETY: this block is an allocated range inside the live region.
        unsafe { self.region.ptr.as_ptr().add(self.offset).cast() }
    }
}

impl Block {
    /// Return a fully destroyed payload's range. A busy registry or range map
    /// leaves both physical ownership and its accounting with the caller.
    pub(crate) fn try_return(&mut self) -> bool {
        if self.returned {
            return true;
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        crate::refund_tests::before_region_return();
        let registry = self.region.registry.state.upgrade();
        let state = if let Some(registry) = &registry {
            let Some(state) = try_lock(registry) else {
                return false;
            };
            Some(state)
        } else {
            None
        };
        let Some(free) = try_lock(&self.region.free) else {
            return false;
        };
        self.release_locked(state, free);
        self.returned = true;
        true
    }

    fn release_locked(
        &self,
        mut state: Option<MutexGuard<'_, State>>,
        mut free: MutexGuard<'_, FreeRanges>,
    ) {
        let previous = free.by_size.last().map_or(0, |(size, _)| *size);
        if self.length != 0 {
            self.region.release(&mut free, self.offset, self.length);
            if !self.relocation {
                self.region.reclaimed.store(true, Ordering::Relaxed);
            }
        }
        let mut recorded = false;
        if let Some(state) = &mut state
            && state.regions.contains_key(&self.region.id)
        {
            recorded = !self.relocation && self.length != 0;
            state.occupied -= self.length;
            state.available.remove(&(previous, self.region.id));
            let largest = free.by_size.last().map_or(0, |(size, _)| *size);
            state.available.insert((largest, self.region.id));
            if self.region.occupied() == 0 && (state.evacuation.is_some() || self.region.oversized)
            {
                state.remove_region(self.region.id, !self.region.oversized);
            }
        }
        drop(free);
        drop(state);
        if recorded {
            // Only published payload space, now reusable by other allocators,
            // counts as progress for an exact physical recovery retry.
            self.region
                .returned_bytes
                .fetch_add(self.length as u64, Ordering::Release);
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        if self.returned {
            return;
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        crate::refund_tests::before_region_return();
        let registry = self.region.registry.state.upgrade();
        let state = registry.as_ref().map(|registry| lock(registry));
        let free = lock(&self.region.free);
        self.release_locked(state, free);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_preserves_mutator_space_and_rechecks_refilled_candidates() {
        let heap = RegionHeap::with_policy(1024, 10 * 1024, 0, Budget::new(10 * 1024), true);
        let layout = Layout::from_size_align(512, 64).expect("payload layout");
        let blocks: Vec<_> = (0..16)
            .map(|_| heap.allocate(layout, None).expect("payload"))
            .collect();
        let live: Vec<_> = blocks
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| index.is_multiple_of(2).then_some(block))
            .collect();
        assert!(heap.begin_evacuation());
        assert!(!heap.select_evacuation().expect("uncontended selection")); // Snapshot the first sparse candidate.
        let small = Layout::from_size_align(64, 64).expect("small layout");
        let refill = heap
            .allocate(small, None)
            .expect("refill candidate after sampling");
        let mut steps = 1;
        while !heap.select_evacuation().expect("uncontended selection") {
            steps += 1;
            assert!(steps <= 18); // At most eight entries plus eight candidates and boundaries.
        }
        assert_eq!(heap.statistics().regions_selected, 4);
        {
            let state = lock(&heap.states[0]);
            let plan = state.evacuation.as_ref().expect("policy");
            assert!(plan.is_destination(&refill.region));
            assert!(!plan.source(&refill.region));
        }
        let ordinary = heap
            .allocate(layout, None)
            .expect("unselected holes remain available");
        assert_eq!(heap.statistics().committed_bytes, 8 * 1024);
        drop(ordinary);
        drop(refill);
        drop(live);
        heap.end_evacuation();
        assert!(heap.trim(32));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn retiring_a_partially_prepared_buffer_retries_each_owned_registry() {
        use crate::{HeapBuffer, Trace, TraceProgress};
        let source = RegionHeap::new(1024, 4096, 0);
        let destination = RegionHeap::new(1024, 4096, 0);
        let mut buffer = HeapBuffer::try_with_fn(Some(&source), 256, |_| Ok(7_u8)).expect("source");
        assert_eq!(
            buffer.prepare_step(&destination, 1).expect("begin staging"),
            TraceProgress::Continue(1)
        );
        {
            let _parked = lock(destination.states.first().expect("destination registry"));
            assert_eq!(buffer.clear_step(0, 1), TraceProgress::Busy);
            assert_eq!(buffer.len(), 256);
        }
        assert_eq!(destination.statistics().occupied_bytes, 256);
        {
            let _parked = lock(source.states.first().expect("source registry"));
            assert_eq!(buffer.clear_step(0, 1), TraceProgress::Busy);
            assert!(buffer.is_empty());
            assert_eq!(destination.statistics().occupied_bytes, 0);
        }
        assert_eq!(source.statistics().occupied_bytes, 256);
        assert_eq!(buffer.clear_step(0, 1), TraceProgress::Complete);
        assert_eq!(source.statistics().occupied_bytes, 0);
        drop(buffer);
        assert_eq!(source.statistics().occupied_bytes, 0);
    }

    #[test]
    fn empty_oversized_backing_cannot_be_repopulated_by_tiny_copies() {
        let heap = RegionHeap::with_policy(1024, 4096, 0, Budget::new(4096), true);
        let mut small =
            crate::HeapBuffer::try_with_fn(Some(&heap), 512, |_| Ok(7_u8)).expect("small payload");
        let large =
            crate::HeapBuffer::try_with_fn(Some(&heap), 3072, |_| Ok(9_u8)).expect("large payload");
        assert_eq!(heap.statistics().shared_committed_bytes, 4096);
        drop(large);
        assert_eq!(heap.statistics().shared_committed_bytes, 1024);
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        small.compact_step(&heap, 512);
        assert_eq!(heap.statistics().shared_committed_bytes, 1024);
        assert!(small.iter().all(|value| *value == 7));
        heap.end_evacuation();
        drop(small);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn region_size_respects_the_ordinary_limit_on_small_reserved_heaps() {
        let heap = RegionHeap::new(2048, 1024, 128);
        let buffer = crate::HeapBuffer::try_with_fn(Some(&heap), 128, |_| Ok(7_u8))
            .expect("payload fits the ordinary allowance");
        assert_eq!(heap.statistics().shared_committed_bytes, 896);
        assert_eq!(heap.statistics().relocation_reserve_bytes, 128);
        assert_eq!(heap.statistics().relocation_reserve_used_bytes, 0);
        drop(buffer);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn evacuation_without_destination_headroom_preserves_aligned_sources() {
        #[repr(align(256))]
        struct Aligned([u8; 256]);

        // Every sparse region is selected, but there is no destination backing
        // allowance. Failure must preserve the initialized source and its charge.
        let heap = RegionHeap::with_policy(1024, 3072, 0, Budget::new(3072), true);
        let buffers: Vec<_> = (0_u8..6)
            .map(|value| {
                crate::HeapBuffer::try_with_fn(Some(&heap), 2, |_| Ok(Aligned([value; 256])))
                    .expect("aligned payload")
            })
            .collect();
        let mut live: Vec<_> = buffers
            .into_iter()
            .enumerate()
            .filter_map(|(index, buffer)| index.is_multiple_of(2).then_some(buffer))
            .collect();
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        for (index, buffer) in live.iter_mut().enumerate() {
            let address = buffer.as_ptr();
            assert_eq!(address.addr() % 256, 0);
            assert_eq!(
                buffer.compact_step(&heap, 512),
                crate::TraceProgress::Complete
            );
            assert_eq!(buffer.as_ptr(), address);
            let expected = u8::try_from(index).expect("three sources") * 2;
            assert!(buffer.iter().all(|value| value.0 == [expected; 256]));
        }
        let stats = heap.statistics();
        assert_eq!(stats.destination_failures, 3);
        assert_eq!(stats.bytes_copied, 0);
        assert_eq!(stats.shared_committed_bytes, 3072);
        heap.end_evacuation();
        // Legacy packing can still use these holes after the failed selection.
        let buffer = live.last_mut().expect("later source");
        buffer.compact_step(&heap, 512);
        assert_eq!(buffer.as_ptr().addr() % 256, 0);
        assert!(buffer.iter().all(|value| value.0 == [4; 256]));
        drop(live);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn a_new_selection_restarts_destinations_reserved_by_an_older_copy() {
        let heap = RegionHeap::with_policy(1024, 4096, 0, Budget::new(4096), true);
        let buffers: Vec<_> = (0_u8..6)
            .map(|value| {
                crate::HeapBuffer::try_with_fn(Some(&heap), 512, |_| Ok(value)).expect("payload")
            })
            .collect();
        let mut live: Vec<_> = buffers
            .into_iter()
            .enumerate()
            .filter_map(|(index, buffer)| index.is_multiple_of(2).then_some(buffer))
            .collect();
        let source = live.last_mut().expect("later source");
        assert!(matches!(
            source.compact_step(&heap, 128),
            crate::TraceProgress::Continue(_)
        ));
        assert_eq!(heap.statistics().bytes_copied, 128);
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        for _ in 0..4 {
            source.compact_step(&heap, 128);
        }
        assert_eq!(heap.statistics().bytes_copied, 640);
        assert!(source.iter().all(|value| *value == 4));
        let copied = heap.statistics().bytes_copied;
        source.compact_step(&heap, 512);
        assert_eq!(heap.statistics().bytes_copied, copied);
        heap.end_evacuation();
        drop(live);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn cold_sources_move_while_hot_and_recently_pinned_regions_are_deferred() {
        let heap =
            RegionHeap::with_policy(1024, 4096, 1024, Budget::with_reserve(4096, 1024), true);
        let buffers: Vec<_> = (0_u8..6)
            .map(|value| {
                crate::HeapBuffer::try_with_fn(Some(&heap), 512, |_| Ok(value)).expect("payload")
            })
            .collect();
        let mut live: Vec<_> = buffers
            .into_iter()
            .enumerate()
            .filter_map(|(index, buffer)| index.is_multiple_of(2).then_some(buffer))
            .collect();
        let hot = live.first_mut().expect("hot payload");
        for value in 0..9 {
            *hot.get_mut(0).expect("element") = value;
        }
        let hot_address = hot.as_ptr();
        let pinned = live.get(1).expect("pinned payload");
        let pinned_address = pinned.as_ptr();
        let pinned_region = pinned.relocation_region(&heap).expect("region");
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        heap.record_pin(pinned_region);
        heap.end_evacuation();
        for value in 0..9 {
            *live
                .first_mut()
                .expect("hot payload")
                .get_mut(0)
                .expect("element") = value;
        }
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        let cold = live.get_mut(2).expect("cold payload");
        let cold_address = cold.as_ptr();
        for _ in 0..4 {
            cold.compact_step(&heap, 128);
        }
        assert_ne!(cold_address, cold.as_ptr());
        assert!(cold.iter().all(|value| *value == 4));
        assert_eq!(live.first().expect("hot payload").as_ptr(), hot_address);
        assert_eq!(
            live.get(1).expect("pinned payload").as_ptr(),
            pinned_address
        );
        assert_eq!(heap.statistics().hot_region_skips, 2);
        assert_eq!(heap.statistics().pinned_region_skips, 1);
        heap.end_evacuation();
        // Pin feedback lasts one selection, not forever after the guard is gone.
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        let released = live.get_mut(1).expect("released payload");
        for _ in 0..4 {
            released.compact_step(&heap, 128);
        }
        assert_ne!(released.as_ptr(), pinned_address);
        assert!(released.iter().all(|value| *value == 2));
        heap.end_evacuation();
        drop(live);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
    }

    #[test]
    fn one_shared_reserve_evacuates_multiple_sources_without_waiting_for_final_trim() {
        let budget = Budget::with_reserve(4096, 1024);
        let heap = RegionHeap::with_policy(1024, 4096, 1024, budget.clone(), true);
        let descriptors = RegionHeap::with_budget(1024, 4096, 0, budget.clone());
        let buffers: Vec<_> = (0_u8..6)
            .map(|value| {
                crate::HeapBuffer::try_with_fn(Some(&heap), 512, |_| Ok(value)).expect("payload")
            })
            .collect();
        let mut live: Vec<_> = buffers
            .into_iter()
            .enumerate()
            .filter_map(|(index, buffer)| index.is_multiple_of(2).then_some(buffer))
            .collect();
        assert_eq!(heap.statistics().committed_bytes, 3072);
        assert!(heap.begin_evacuation());
        let mut selection_steps = 1;
        while !heap.select_evacuation().expect("uncontended selection") {
            selection_steps += 1;
            assert!(selection_steps <= 8); // Three build units, three selections, two boundaries.
        }
        assert_eq!(heap.statistics().regions_selected, 3);
        let tiny = Layout::from_size_align(64, 64).expect("layout");
        assert!(!heap.can_allocate(tiny));
        assert!(heap.allocate(tiny, None).is_err()); // Sources cannot be refilled.
        assert!(descriptors.allocate(tiny, None).is_err()); // Shared reserve is protected.
        for (index, buffer) in live.iter_mut().enumerate() {
            for _ in 0..4 {
                buffer.compact_step(&heap, 128);
                assert!(budget.used.load(Ordering::Acquire) <= 4096);
            }
            assert!(buffer.iter().all(|value| usize::from(*value) == index * 2));
            let copied = heap.statistics().bytes_copied;
            assert_eq!(
                buffer.compact_step(&heap, 512),
                crate::TraceProgress::Complete
            );
            assert_eq!(heap.statistics().bytes_copied, copied); // Never move a destination again.
        }
        let stats = heap.statistics();
        assert_eq!(stats.regions, 2);
        assert_eq!(stats.committed_bytes, 2048);
        assert_eq!(stats.evacuation_regions_released, 3);
        assert_eq!(stats.evacuation_bytes_released, 3072);
        assert_eq!(stats.bytes_copied, 1536);
        assert_eq!(stats.peak_relocation_reserve_used_bytes, 1024);
        assert_eq!(stats.relocation_reserve_used_bytes, 0);
        heap.end_evacuation();
        let descriptor = descriptors
            .allocate(tiny, None)
            .expect("ordinary capacity returned");
        drop(descriptor);
        drop(live);
        assert!(heap.trim(16));
        assert!(descriptors.trim(16));
        assert_eq!(budget.used.load(Ordering::Acquire), 0);
    }

    #[test]
    fn finishing_selection_restores_unmoved_source_space_to_mutators() {
        let heap = RegionHeap::with_policy(512, 1024, 0, Budget::new(1024), true);
        let layout = Layout::from_size_align(256, 64).expect("layout");
        let blocks: Vec<_> = (0..4)
            .map(|_| heap.allocate(layout, None).expect("range"))
            .collect();
        let live: Vec<_> = blocks
            .into_iter()
            .enumerate()
            .filter_map(|(index, block)| index.is_multiple_of(2).then_some(block))
            .collect();
        assert!(heap.begin_evacuation());
        while !heap.select_evacuation().expect("uncontended selection") {}
        let tiny = Layout::from_size_align(64, 64).expect("layout");
        assert!(heap.allocate(tiny, None).is_err());
        // A pinned/hot source may remain unmoved; its spare space must not stay
        // excluded from allocation after the cycle completes.
        heap.end_evacuation();
        let ordinary = heap.allocate(tiny, None).expect("source space restored");
        drop(ordinary);
        drop(live);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().committed_bytes, 0);
    }

    #[test]
    fn unregistering_empty_backing_keeps_credit_until_its_last_owner_drops() {
        let heap = RegionHeap::new(1024, 1024, 0);
        let layout = Layout::from_size_align(1024, 64).expect("layout");
        let mut tail = heap.allocate(layout, None).expect("range");
        let live = tail.take_prefix(layout).expect("whole prefix");
        drop(live);
        assert_eq!(tail.len(), 0);
        // Model evacuation unregistering empty source backing while a zero-length
        // local cache tail still owns the allocation. Dropping a registry Arc is
        // not the physical reclamation event.
        let removed = lock(&heap.states[0])
            .remove_region(tail.id(), false)
            .expect("registered source");
        drop(removed);
        let held = heap.statistics();
        assert_eq!(held.regions, 0);
        assert_eq!(held.occupied_bytes, 0);
        assert_eq!(held.committed_bytes, 1024);
        assert_eq!(held.regions_released, 0);
        assert!(heap.allocate(layout, None).is_err());
        drop(tail);
        let freed = heap.statistics();
        assert_eq!(freed.committed_bytes, 0);
        assert_eq!(freed.regions_released, 1);
        let replacement = heap.allocate(layout, None).expect("actual refund");
        drop(replacement);
        assert!(heap.trim(16));
        assert_eq!(heap.statistics().committed_bytes, 0);
        assert_eq!(heap.statistics().regions_released, 2);
    }

    #[test]
    fn independent_arenas_share_backing_credit_until_the_last_block_retires() {
        let budget = Budget::new(1024);
        let first = RegionHeap::with_budget(1024, 1024, 0, budget.clone());
        let second = RegionHeap::with_budget(1024, 1024, 0, budget.clone());
        let layout = Layout::from_size_align(64, 64).expect("layout");
        let block = first.allocate(layout, None).expect("range");
        assert_eq!(budget.used.load(Ordering::Acquire), 1024);
        assert!(second.allocate(layout, None).is_err());
        drop(first);
        // An escaped block still owns its arena allocation after heap destruction.
        assert!(second.allocate(layout, None).is_err());
        drop(block);
        assert_eq!(budget.used.load(Ordering::Acquire), 0);
        let replacement = second.allocate(layout, None).expect("refunded credit");
        drop(replacement);
        assert!(second.trim(16));
        assert_eq!(budget.used.load(Ordering::Acquire), 0);
    }

    #[test]
    fn pressure_tracks_registered_free_extents_and_trim() {
        let heap = RegionHeap::new(1024, 2048, 0);
        let layout = Layout::from_size_align(256, 256).expect("valid layout");
        let first = heap.allocate(layout, None).expect("range");
        let second = heap.allocate(layout, None).expect("range");
        let third = heap.allocate(layout, None).expect("range");
        drop(second);
        let pressure = heap.pressure().expect("unlocked registry");
        assert_eq!(pressure.committed, 1024);
        assert_eq!(pressure.free, 512);
        assert_eq!(pressure.largest_free, 256);
        assert_eq!(heap.statistics().occupied_bytes, 512);
        drop(first);
        drop(third);
        assert_eq!(heap.pressure().expect("registry").largest_free, 1024);
        assert!(heap.trim(16));
        let pressure = heap.pressure().expect("registry");
        assert_eq!(pressure.committed, 0);
        assert_eq!(pressure.free, 0);
        assert_eq!(pressure.largest_free, 0);
        assert_eq!(heap.statistics().occupied_bytes, 0);
    }

    #[test]
    fn pressure_skips_busy_registry_and_excludes_unregistered_credits() {
        let heap = RegionHeap::new(1024, 2048, 0);
        let state = lock(&heap.states[0]);
        assert!(heap.pressure().is_none());
        // Simulate a reserved backing allocation not yet installed in the registry.
        state.backing.committed.store(1024, Ordering::Release);
        drop(state);
        let pressure = heap.pressure().expect("registry");
        assert_eq!(pressure.committed, 1024);
        assert_eq!(pressure.free, 0);
        assert_eq!(pressure.largest_free, 0);
    }
}
