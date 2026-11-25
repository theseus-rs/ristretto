//! Reusable owning-root registrations. The collector supplies publication/deletion barriers.
//! No thread-local ownership: a registration can move with its handle or suspended future.
use arc_swap::ArcSwap;
#[path = "root_overflow.rs"]
mod overflow;
use overflow::Overflow;
pub(crate) use overflow::Record;
use portable_atomic::AtomicU64;
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU64;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

const SLOT_BITS: usize = 6;
const SLOTS: usize = 1 << SLOT_BITS;
const SHARD_BITS: usize = 4;
const SHARDS: usize = 1 << SHARD_BITS;
const LOCATION_BITS: usize = SLOT_BITS + SHARD_BITS;
const MAX_PAGE: usize = usize::MAX >> LOCATION_BITS;
const MAX_GENERATION: u64 = u64::MAX >> SLOT_BITS;
// The high bit distinguishes owning/internal registrations from public manual
// roots without growing TreeEntry or its generation-bearing removal token.
const OWNED_TREE_ROOT: u64 = 1 << 63;
const MAX_TREE_GENERATION: u64 = OWNED_TREE_ROOT - 1;

/// Root registry occupancy. Page bytes exclude map nodes and owning-handle allocations.
#[derive(Clone, Copy, Debug, Default)]
pub struct RootStatistics {
    pub slot_roots: usize,
    /// Slots prepared before admission, with no published address yet.
    pub reserved_slots: usize,
    pub compatibility_roots: usize,
    pub pages: usize,
    pub slot_capacity: usize,
    /// Single-registration records in the contended writer fallback directory.
    pub overflow_records: usize,
    /// Bytes in the indexed records, excluding directory nodes and Arc headers.
    pub overflow_record_bytes: usize,
    pub page_bytes: usize,
    /// Physical pages, including pages pinned by in-progress scans or preparation.
    pub resident_pages: usize,
    /// Page payload bytes; excludes map nodes and Arc allocation headers.
    pub resident_page_bytes: usize,
    pub cached_empty_pages: usize,
    /// Pages installed in the registry, including pages later released.
    pub pages_created: u64,
    pub pages_released: u64,
    pub slot_reuses: u64,
}

impl RootStatistics {
    pub(crate) fn merge(&mut self, other: Self) {
        self.slot_roots += other.slot_roots;
        self.reserved_slots += other.reserved_slots;
        self.compatibility_roots += other.compatibility_roots;
        self.pages += other.pages;
        self.slot_capacity += other.slot_capacity;
        self.overflow_records += other.overflow_records;
        self.overflow_record_bytes += other.overflow_record_bytes;
        self.page_bytes += other.page_bytes;
        self.resident_pages += other.resident_pages;
        self.resident_page_bytes += other.resident_page_bytes;
        self.cached_empty_pages += other.cached_empty_pages;
        self.pages_created += other.pages_created;
        self.pages_released += other.pages_released;
        self.slot_reuses += other.slot_reuses;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SlotId {
    location: usize,
    generation: NonZeroU64,
}

impl SlotId {
    #[expect(
        clippy::expect_used,
        reason = "occupied slots always carry a checked nonzero generation"
    )]
    fn new(shard: usize, page: usize, slot: usize, generation: u64) -> Self {
        Self {
            location: (page << LOCATION_BITS) | (shard << SLOT_BITS) | slot,
            generation: NonZeroU64::new(generation).expect("occupied root generation"),
        }
    }
    fn shard(self) -> usize {
        (self.location >> SLOT_BITS) & (SHARDS - 1)
    }
    fn page(self) -> usize {
        self.location >> LOCATION_BITS
    }
    fn slot(self) -> usize {
        self.location & (SLOTS - 1)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Registration {
    Tree(TreeRoot),
    Slot(SlotRoot),
    Context(Arc<crate::context::State>, SlotRoot),
    Overflow(Arc<Record>),
    ContextOverflow(Arc<crate::context::State>, Arc<Record>),
}

/// Owning tree roots must not follow a recycled public numeric ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TreeRoot {
    pub(crate) id: usize,
    pub(crate) generation: NonZeroU64,
}

#[derive(Debug)]
struct TreeEntry {
    generation: NonZeroU64,
    address: usize,
}

#[derive(Debug, Default)]
pub(crate) struct TreeRoots {
    entries: BTreeMap<usize, TreeEntry>,
    generation: u64,
}

impl TreeRoots {
    pub(crate) fn insert(&mut self, next: &AtomicUsize, address: usize) -> TreeRoot {
        self.insert_kind(next, address, true)
    }

    pub(crate) fn insert_manual(&mut self, next: &AtomicUsize, address: usize) -> TreeRoot {
        self.insert_kind(next, address, false)
    }

    #[expect(
        clippy::expect_used,
        reason = "root generations fail closed before reuse"
    )]
    fn insert_kind(&mut self, next: &AtomicUsize, address: usize, owned: bool) -> TreeRoot {
        debug_assert_ne!(address, 0);
        loop {
            let id = next.fetch_add(1, Ordering::Relaxed);
            if id == 0 {
                continue;
            }
            if let std::collections::btree_map::Entry::Vacant(entry) = self.entries.entry(id) {
                let generation = self
                    .generation
                    .checked_add(1)
                    .filter(|&generation| generation <= MAX_TREE_GENERATION)
                    .expect("tree root generation exhausted");
                self.generation = generation;
                let generation =
                    NonZeroU64::new(generation | if owned { OWNED_TREE_ROOT } else { 0 })
                        .expect("nonzero tree root generation");
                entry.insert(TreeEntry {
                    generation,
                    address,
                });
                return TreeRoot { id, generation };
            }
        }
    }

    pub(crate) fn remove(&mut self, id: usize, generation: Option<NonZeroU64>) -> Option<usize> {
        let std::collections::btree_map::Entry::Occupied(entry) = self.entries.entry(id) else {
            return None;
        };
        let actual = entry.get().generation;
        if generation.map_or_else(
            || actual.get() & OWNED_TREE_ROOT != 0,
            |expected| expected != actual,
        ) {
            return None;
        }
        Some(entry.remove().address)
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn next(&self, after: usize) -> Option<(usize, usize)> {
        self.entries
            .range((Excluded(after), Unbounded))
            .next()
            .map(|(&id, entry)| (id, entry.address))
    }

    pub(crate) fn matching(&self, address: usize) -> Vec<TreeRoot> {
        self.entries
            .iter()
            .filter_map(|(&id, entry)| {
                (entry.address == address).then_some(TreeRoot {
                    id,
                    generation: entry.generation,
                })
            })
            .collect()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Cursor {
    shard: usize,
    page: usize,
    slot: usize,
    current: Option<Arc<Page>>,
}

impl Cursor {
    /// Only immediately after `Scan::Root`: re-read that slot on the next unit.
    /// Its page remains pinned, but its address/generation may change meanwhile.
    #[expect(clippy::expect_used, reason = "only a selected root can be retried")]
    pub(crate) fn retry_root(&mut self) {
        if self.shard == SHARDS {
            // Every overflow key is a non-null allocation address. Revisit this
            // key if it survived, or select its successor if it was removed.
            self.page = self.page.checked_sub(1).expect("selected overflow key");
        } else {
            self.slot = self.slot.checked_sub(1).expect("selected root slot");
        }
    }
}

#[derive(Debug)]
pub(crate) enum Scan {
    Root(usize),
    Advance,
    Done,
}

#[path = "root_slot_state.rs"]
mod root_slot_state;
use root_slot_state::{PENDING_ROOT, Slot, TAKEN, scan_address};

#[derive(Debug)]
struct Page {
    resident: Arc<AtomicUsize>,
    location: usize,
    keep_empty: AtomicBool,
    occupied: AtomicU64,
    slots: [Slot; SLOTS],
}
impl Page {
    fn new(resident: &Arc<AtomicUsize>) -> Self {
        resident.fetch_add(1, Ordering::Relaxed);
        Self {
            resident: resident.clone(),
            location: 0,
            keep_empty: AtomicBool::new(false),
            occupied: AtomicU64::new(0),
            slots: std::array::from_fn(|_| Slot::default()),
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        self.resident.fetch_sub(1, Ordering::Relaxed);
    }
}

/// A registration pins its slot page so deletion needs no registry lookup.
#[derive(Clone, Debug)]
pub(crate) struct SlotRoot {
    slot_and_generation: NonZeroU64,
    page: Arc<Page>,
}

impl SlotRoot {
    #[expect(
        clippy::expect_used,
        reason = "checked nonzero generation fits above six slot bits"
    )]
    fn new(id: SlotId, page: Arc<Page>) -> Self {
        debug_assert!(id.generation.get() <= MAX_GENERATION);
        debug_assert_eq!(id.location & !(SLOTS - 1), page.location);
        Self {
            slot_and_generation: NonZeroU64::new(
                (id.generation.get() << SLOT_BITS) | (id.slot() as u64),
            )
            .expect("packed slot generation"),
            page,
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "only the lowest six bits form the slot index"
    )]
    fn slot(&self) -> usize {
        (self.slot_and_generation.get() as usize) & (SLOTS - 1)
    }
    fn shard(&self) -> usize {
        (self.page.location >> SLOT_BITS) & (SHARDS - 1)
    }
    fn page_id(&self) -> usize {
        self.page.location >> LOCATION_BITS
    }

    #[cfg(test)]
    fn id(&self) -> SlotId {
        SlotId::new(
            self.shard(),
            self.page_id(),
            self.slot(),
            self.slot_and_generation.get() >> SLOT_BITS,
        )
    }

    #[expect(clippy::indexing_slicing, reason = "private slot index is bounded")]
    pub(crate) fn take_address(&self) -> Option<usize> {
        self.page.slots[self.slot()].take(self.slot_and_generation.get() >> SLOT_BITS)
    }

    /// Called only after address removal and its deletion barrier complete.
    /// Release makes the slot reusable; the writer's acquire observes both.
    pub(crate) fn release(&self) -> bool {
        let bit = 1_u64 << self.slot();
        let previous = self.page.occupied.fetch_and(!bit, Ordering::Release);
        // A designated cache page remains indexed while empty or nonempty.
        // Ordinary releases therefore change no writer-owned metadata. A full
        // page becoming available, or a noncached page becoming empty, does.
        previous == u64::MAX || (previous == bit && !self.page.keep_empty.load(Ordering::Acquire))
    }
}

/// An invisible slot owned by the collector's cancellation guard. Holding this
/// page reference makes commit independent of the registry writer lock.
#[derive(Debug)]
pub(crate) struct Reservation {
    id: SlotId,
    page: Arc<Page>,
}

pub(crate) enum PreparedSlot {
    Paged(Reservation),
    Overflow(Arc<Record>),
}

impl PreparedSlot {
    #[expect(clippy::indexing_slicing, reason = "reserved slot index is bounded")]
    fn address(&self) -> &AtomicUsize {
        match self {
            Self::Paged(slot) => &slot.page.slots[slot.id.slot()].address,
            Self::Overflow(record) => &record.address,
        }
    }

    pub(crate) fn arm(&self) {
        root_slot_state::arm_root(self.address());
    }

    pub(crate) fn activate(&self, address: usize) -> bool {
        root_slot_state::activate_root(self.address(), address)
    }

    pub(crate) fn commit(
        self,
        address: usize,
        context: Option<Arc<crate::context::State>>,
    ) -> Registration {
        self.address().store(address, Ordering::Release);
        self.finish(context)
    }

    // Once activation exposes the address a revoker can take the generation.
    // Transferring ownership must never write to that address a second time.
    pub(crate) fn finish(self, context: Option<Arc<crate::context::State>>) -> Registration {
        match self {
            Self::Paged(slot) => {
                let root = SlotRoot::new(slot.id, slot.page);
                match context {
                    Some(context) => Registration::Context(context, root),
                    None => Registration::Slot(root),
                }
            }
            Self::Overflow(root) => {
                root.published.store(true, Ordering::Release);
                match context {
                    Some(context) => Registration::ContextOverflow(context, root),
                    None => Registration::Overflow(root),
                }
            }
        }
    }
}

#[cfg(test)]
impl Reservation {
    #[expect(clippy::indexing_slicing, reason = "reserved slot index is bounded")]
    fn commit(self, address: usize) -> SlotRoot {
        self.page.slots[self.id.slot()]
            .address
            .store(address, Ordering::Release);
        SlotRoot::new(self.id, self.page)
    }
}

#[derive(Debug, Default)]
struct State {
    pages: BTreeMap<usize, Arc<Page>>,
    available: BTreeSet<usize>,
    cached_page: Option<usize>,
    next_page: usize,
    generation: u64,
    created: u64,
    released: u64,
    reused: u64,
}

#[derive(Debug, Default)]
#[repr(align(64))]
struct Shard {
    state: Mutex<State>,
    // Updated only when page membership changes, under `state`. A scan never
    // takes that mutex or retains this map across work units. The current page
    // has its own Arc, so retirement does not invalidate a resumable cursor.
    published: ArcSwap<BTreeMap<usize, Arc<Page>>>,
    cleanup_pending: AtomicBool,
}

#[derive(Debug, Default)]
pub(crate) struct RootSlots {
    shards: [Shard; SHARDS],
    overflow: Overflow,
    resident: Arc<AtomicUsize>,
    #[cfg(test)]
    pub(crate) prepare_pause:
        Mutex<Option<(std::sync::mpsc::Sender<()>, std::sync::mpsc::Receiver<()>)>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[expect(
    clippy::indexing_slicing,
    reason = "private locations and bitmap positions are bounded by fixed arrays"
)]
impl RootSlots {
    #[expect(
        clippy::expect_used,
        reason = "registry invariants and generation exhaustion must fail before reusing an identity"
    )]
    pub(crate) fn prepare(&self, address: usize) -> PreparedSlot {
        // Mix independently of the metadata index: its sweep order can drive
        // descriptor reuse, so sharing that bucket function causes page churn.
        let mut key = address >> 4;
        key ^= key >> 16;
        key = key.wrapping_mul(0x7feb_352d);
        key ^= key >> 15;
        key = key.wrapping_mul(0x846c_a68b);
        key ^= key >> 16;
        let preferred = key & (SHARDS - 1);
        let mut spare: Option<Arc<Page>> = None;
        loop {
            // The address selects a preferred shard. The registration itself
            // records its physical shard, so any available writer can host it.
            let Some((shard, mut state)) = self.available_writer(preferred) else {
                return PreparedSlot::Overflow(self.overflow.reserve(None));
            };
            let registry = &self.shards[shard];
            let old_view = if registry.cleanup_pending.swap(false, Ordering::AcqRel) {
                Self::clean_all(registry, &mut state)
            } else {
                None
            };
            let page_id = if let Some(&id) = state.available.first() {
                id
            } else if let Some(mut page) = spare.take() {
                let id = loop {
                    let id = state.next_page;
                    state.next_page = id.wrapping_add(1) & MAX_PAGE;
                    if !state.pages.contains_key(&id) {
                        break id;
                    }
                };
                Arc::get_mut(&mut page).expect("unpublished page").location =
                    (id << LOCATION_BITS) | (shard << SLOT_BITS);
                state.pages.insert(id, page);
                self.shards[shard]
                    .published
                    .store(Arc::new(state.pages.clone()));
                state.available.insert(id);
                state.created += 1;
                id
            } else {
                // System allocation does not hold a shard lock. A racing insertion
                // can make this spare unnecessary; it is dropped after unlocking.
                drop(state);
                drop(old_view);
                self.finish_deferred(shard);
                spare = Some(Arc::new(Page::new(&self.resident)));
                continue;
            };
            if state.cached_page.is_none() {
                state.cached_page = Some(page_id);
                state
                    .pages
                    .get(&page_id)
                    .expect("available page")
                    .keep_empty
                    .store(true, Ordering::Release);
            }
            #[cfg(test)]
            {
                let pause = lock(&self.prepare_pause).take();
                if let Some((entered, resume)) = pause {
                    entered.send(()).expect("root preparation observer");
                    resume.recv().expect("root preparation resume");
                }
            }
            // Unlike public usize IDs, generations never wrap, including on wasm32.
            let generation = state
                .generation
                .checked_add(1)
                .filter(|generation| *generation <= MAX_GENERATION)
                .expect("root slot generation exhausted");
            state.generation = generation;
            let page = state.pages.get_mut(&page_id).expect("available root page");
            let occupied = page.occupied.load(Ordering::Acquire);
            let slot = (!occupied).trailing_zeros() as usize;
            let reused = page.slots[slot].generation.load(Ordering::Relaxed) != 0;
            page.slots[slot].prepare_generation(generation);
            let occupied =
                page.occupied.fetch_or(1_u64 << slot, Ordering::AcqRel) | (1_u64 << slot);
            let page = page.clone();
            if occupied == u64::MAX {
                state.available.remove(&page_id);
            }
            state.reused += u64::from(reused);
            let reservation = Reservation {
                id: SlotId::new(shard, page_id, slot, generation),
                page,
            };
            drop(state);
            drop(old_view);
            self.finish_deferred(shard);
            return PreparedSlot::Paged(reservation);
        }
    }

    #[cfg(test)]
    fn reserve(&self, address: usize) -> Reservation {
        let PreparedSlot::Paged(slot) = self.prepare(address) else {
            panic!("expected an available page writer")
        };
        slot
    }

    pub(crate) fn cancel_prepared(&self, slot: PreparedSlot) {
        // The caller still exclusively owns this unpublished generation.
        slot.address().store(0, Ordering::Release);
        match slot {
            PreparedSlot::Paged(slot) => self.cancel(slot),
            PreparedSlot::Overflow(record) => {
                record.cancel_typed();
                self.finish_overflow(&record);
            }
        }
    }

    pub(crate) fn prepare_typed_attempt(&self, address: usize) -> Arc<Record> {
        self.overflow.reserve(Some(address))
    }

    pub(crate) fn finish_overflow(&self, record: &Arc<Record>) {
        self.overflow.remove(record);
    }

    pub(crate) fn matching_overflow(&self, address: usize) -> Vec<Arc<Record>> {
        self.overflow.matching(address)
    }

    fn cancel(&self, reservation: Reservation) {
        let root = SlotRoot::new(reservation.id, reservation.page);
        let address = root.take_address();
        debug_assert_eq!(address, Some(0));
        if root.release() {
            self.finish_removal(&root);
        }
    }

    #[cfg(test)]
    pub(crate) fn insert(&self, address: usize) -> SlotId {
        self.reserve(address).commit(address).id()
    }

    #[cfg(test)]
    pub(crate) fn remove(&self, id: SlotId) -> Option<usize> {
        let page = self.shards[id.shard()]
            .published
            .load()
            .get(&id.page())
            .cloned()?;
        let root = SlotRoot::new(id, page);
        let address = root.take_address()?;
        if root.release() {
            self.finish_removal(&root);
        }
        Some(address)
    }

    fn try_state(registry: &Shard) -> Option<MutexGuard<'_, State>> {
        match registry.state.try_lock() {
            Ok(state) => Some(state),
            Err(TryLockError::Poisoned(error)) => Some(error.into_inner()),
            Err(TryLockError::WouldBlock) => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_all_writers_held<R>(&self, operation: impl FnOnce() -> R) -> R {
        let _writers = self.shards.each_ref().map(|shard| lock(&shard.state));
        operation()
    }

    fn available_writer(&self, preferred: usize) -> Option<(usize, MutexGuard<'_, State>)> {
        (0..SHARDS).find_map(|offset| {
            let shard = (preferred + offset) & (SHARDS - 1);
            Self::try_state(&self.shards[shard]).map(|state| (shard, state))
        })
    }

    // Caller owns the writer mutex. Atomic removers only clear occupied bits;
    // no new reservation can appear while this mutex is held.
    fn clean_page(state: &mut State, id: usize) -> bool {
        let Some(page) = state.pages.get(&id) else {
            return false;
        };
        let occupied = page.occupied.load(Ordering::Acquire);
        if occupied != u64::MAX {
            state.available.insert(id);
        }
        if occupied != 0 || state.cached_page == Some(id) {
            return false;
        }
        if state.cached_page.is_none() {
            state.cached_page = Some(id);
            page.keep_empty.store(true, Ordering::Release);
            return false;
        }
        state.available.remove(&id);
        state.released += 1;
        state.pages.remove(&id);
        true
    }

    fn clean_all(registry: &Shard, state: &mut State) -> Option<Arc<BTreeMap<usize, Arc<Page>>>> {
        let mut next = state.pages.first_key_value().map(|(&id, _)| id);
        let mut changed = false;
        while let Some(id) = next {
            changed |= Self::clean_page(state, id);
            next = state
                .pages
                .range((Excluded(id), Unbounded))
                .next()
                .map(|(&id, _)| id);
        }
        changed.then(|| registry.published.swap(Arc::new(state.pages.clone())))
    }

    /// Metadata cleanup never waits for the writer mutex and runs after the
    /// caller's deletion admission. A busy writer receives a deferred request.
    pub(crate) fn finish_removal(&self, root: &SlotRoot) {
        let registry = &self.shards[root.shard()];
        if let Some(mut state) = Self::try_state(registry) {
            let same_page = state
                .pages
                .get(&root.page_id())
                .is_some_and(|page| Arc::ptr_eq(page, &root.page));
            let changed = same_page && Self::clean_page(&mut state, root.page_id());
            let old_view = changed.then(|| registry.published.swap(Arc::new(state.pages.clone())));
            drop(state);
            drop(old_view);
        } else {
            registry.cleanup_pending.store(true, Ordering::Release);
        }
        // Retry after setting the flag: the writer may have unlocked just before
        // that store. Either this retry or that writer's exit observes the work.
        self.finish_deferred(root.shard());
    }

    fn finish_deferred(&self, shard: usize) {
        let registry = &self.shards[shard];
        while registry.cleanup_pending.load(Ordering::Acquire) {
            let Some(mut state) = Self::try_state(registry) else {
                return;
            };
            let old_view = if registry.cleanup_pending.swap(false, Ordering::AcqRel) {
                Self::clean_all(registry, &mut state)
            } else {
                None
            };
            drop(state);
            drop(old_view);
        }
    }

    /// At most one root or one page/shard advance per work unit. Page membership
    /// is read without the writer mutex. A cursor pins only its current page;
    /// insertions behind it still require the collector's insertion barrier.
    pub(crate) fn scan(&self, cursor: &mut Cursor) -> Scan {
        let Some(shard) = self.shards.get(cursor.shard) else {
            return self.overflow.scan(&mut cursor.page);
        };
        if cursor.current.is_none() && cursor.slot == 0 {
            cursor.current = shard.published.load().get(&cursor.page).cloned();
        }
        if let Some(page) = &cursor.current {
            let mask = if cursor.slot < SLOTS {
                u64::MAX << cursor.slot
            } else {
                0
            };
            let remaining = page.occupied.load(Ordering::Acquire) & mask;
            if remaining != 0 {
                let slot = remaining.trailing_zeros() as usize;
                cursor.slot = slot + 1;
                let address = scan_address(&page.slots[slot].address);
                // A concurrent removal can clear the address after the bitmap
                // read. Its deletion barrier preserves the active snapshot.
                return if address == 0 {
                    Scan::Advance
                } else {
                    Scan::Root(address)
                };
            }
        }
        let next = shard
            .published
            .load()
            .range((Excluded(cursor.page), Unbounded))
            .next()
            .map(|(&id, page)| (id, page.clone()));
        cursor.current = None;
        if let Some((id, page)) = next {
            cursor.page = id;
            cursor.slot = 0;
            cursor.current = Some(page);
        } else {
            cursor.shard += 1;
            cursor.page = 0;
            cursor.slot = 0;
        }
        Scan::Advance
    }

    /// Cold compatibility API: snapshot checked generations without the writer
    /// mutex. A later removal cannot erase a slot reused since this observation.
    pub(crate) fn matching(&self, address: usize) -> Vec<SlotRoot> {
        let mut found = Vec::new();
        for (shard, registry) in self.shards.iter().enumerate() {
            let pages = registry.published.load();
            for (&id, page) in pages.iter() {
                let mut occupied = page.occupied.load(Ordering::Acquire);
                while occupied != 0 {
                    let slot = occupied.trailing_zeros() as usize;
                    occupied &= occupied - 1;
                    if let Some(generation) = page.slots[slot].matching_generation(address) {
                        found.push(SlotRoot::new(
                            SlotId::new(shard, id, slot, generation),
                            page.clone(),
                        ));
                    }
                }
            }
        }
        found
    }

    /// Release empty pages, including deferred removals; preserve live registrations. Live registrations remain usable.
    pub(crate) fn trim_empty(&self) {
        for (index, shard) in self.shards.iter().enumerate() {
            let mut state = lock(&shard.state);
            let old_view = Self::clean_all(shard, &mut state);
            let empty_cached = state.cached_page.filter(|id| {
                state
                    .pages
                    .get(id)
                    .is_some_and(|page| page.occupied.load(Ordering::Acquire) == 0)
            });
            let retired = empty_cached.and_then(|id| {
                state.cached_page = None;
                state.available.remove(&id);
                state.released += 1;
                state.pages.remove(&id)
            });
            if retired.is_some() {
                shard.published.store(Arc::new(state.pages.clone()));
            }
            drop(state);
            drop(retired);
            drop(old_view);
            self.finish_deferred(index);
        }
    }

    pub(crate) fn resident_counter(&self) -> std::sync::Weak<AtomicUsize> {
        Arc::downgrade(&self.resident)
    }

    pub(crate) fn resident_bytes(pages: usize) -> usize {
        pages * size_of::<Page>()
    }

    pub(crate) fn statistics(&self) -> RootStatistics {
        let mut result = RootStatistics::default();
        for (index, shard) in self.shards.iter().enumerate() {
            let state = lock(&shard.state);
            for page in state.pages.values() {
                let mut occupied = page.occupied.load(Ordering::Acquire);
                while occupied != 0 {
                    let slot = occupied.trailing_zeros() as usize;
                    occupied &= occupied - 1;
                    let address = page.slots[slot].address.load(Ordering::Acquire);
                    result.slot_roots += usize::from(address != 0 && address != PENDING_ROOT);
                    result.reserved_slots += usize::from(
                        (address == 0 || address == PENDING_ROOT)
                            && page.slots[slot].generation.load(Ordering::Acquire) != TAKEN,
                    );
                }
            }
            result.pages += state.pages.len();
            result.cached_empty_pages += usize::from(
                state
                    .cached_page
                    .and_then(|id| state.pages.get(&id))
                    .is_some_and(|page| page.occupied.load(Ordering::Acquire) == 0),
            );
            result.pages_created += state.created;
            result.pages_released += state.released;
            result.slot_reuses += state.reused;
            drop(state);
            self.finish_deferred(index);
        }
        result.slot_capacity = result.pages * SLOTS;
        result.page_bytes = result.pages * size_of::<Page>();
        result.resident_pages = self.resident.load(Ordering::Relaxed);
        result.resident_page_bytes = result.resident_pages * size_of::<Page>();
        self.overflow.count(&mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_retry_and_cancellation() {
        let roots = RootSlots::default();
        let (first, next, pending) = roots.with_all_writers_held(|| {
            let record = |value| {
                let PreparedSlot::Overflow(record) = roots.prepare(value) else {
                    panic!("overflow required")
                };
                record
            };
            let first = record(0x1000);
            let next = record(0x2000);
            let pending = roots.prepare(0x3000);
            (first, next, pending)
        });
        assert_eq!(roots.statistics().reserved_slots, 3);
        roots.cancel_prepared(pending);
        let (first, next) = if Record::key(&first) < Record::key(&next) {
            (first, next)
        } else {
            (next, first)
        };
        first.publish(0x1000);
        next.publish(0x2000);
        let mut cursor = Cursor::default();
        while matches!(roots.scan(&mut cursor), Scan::Advance) {}
        cursor.retry_root();
        assert!(matches!(roots.scan(&mut cursor), Scan::Root(0x1000)));
        cursor.retry_root();
        assert_eq!(first.take_address(), Some(0x1000));
        roots.finish_overflow(&first);
        assert!(matches!(roots.scan(&mut cursor), Scan::Root(0x2000)));
        assert!(matches!(roots.scan(&mut cursor), Scan::Done));
        assert!(roots.matching_overflow(0x1000).is_empty());
        assert_eq!(roots.matching_overflow(0x2000).len(), 1);
        assert_eq!(next.take_address(), Some(0x2000));
        roots.finish_overflow(&next);
        assert_eq!(next.take_address(), None);
        assert_eq!(roots.statistics().slot_capacity, 0);
    }

    #[test]
    fn manual_removal_and_owning_tokens_stay_separate_across_id_reuse() {
        let mut roots = TreeRoots::default();
        let next = AtomicUsize::new(usize::MAX);
        let owned = roots.insert(&next, 0x1000);
        assert_eq!(roots.remove(owned.id, None), None);
        assert_eq!(roots.remove(owned.id, Some(owned.generation)), Some(0x1000));

        next.store(owned.id, Ordering::Relaxed);
        let manual = roots.insert_manual(&next, 0x2000);
        assert_eq!(manual.id, owned.id);
        assert_eq!(roots.remove(owned.id, Some(owned.generation)), None);
        assert_eq!(roots.remove(manual.id, None), Some(0x2000));

        next.store(manual.id, Ordering::Relaxed);
        let replacement = roots.insert(&next, 0x3000);
        assert_eq!(roots.remove(manual.id, None), None);
        assert_eq!(roots.remove(manual.id, Some(manual.generation)), None);
        assert_eq!(roots.matching(0x3000), vec![replacement]);
        assert_eq!(
            roots.remove(replacement.id, Some(replacement.generation)),
            Some(0x3000)
        );
        assert_eq!(roots.len(), 0);
    }

    #[test]
    fn manual_generations_exhaust_before_the_owning_bit() {
        let mut roots = TreeRoots {
            generation: MAX_TREE_GENERATION - 1,
            ..TreeRoots::default()
        };
        let next = AtomicUsize::new(1);
        let last = roots.insert_manual(&next, 0x1000);
        assert_eq!(last.generation.get(), MAX_TREE_GENERATION);
        #[cfg(panic = "unwind")]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                roots.insert_manual(&next, 0x2000);
            }));
            assert!(result.is_err());
        }
        assert_eq!(roots.len(), 1);
        assert_eq!(roots.remove(last.id, None), Some(0x1000));
    }

    #[test]
    fn tree_generations_reject_stale_removal_and_exhaust_without_reuse() {
        let mut roots = TreeRoots::default();
        let next = AtomicUsize::new(1);
        let old = roots.insert(&next, 0x1000);
        assert_eq!(roots.remove(old.id, Some(old.generation)), Some(0x1000));
        next.store(old.id, Ordering::Relaxed);
        roots.generation = MAX_TREE_GENERATION - 1;
        let last = roots.insert(&next, 0x2000);
        assert_eq!(old.id, last.id);
        assert_eq!(last.generation.get(), u64::MAX);
        assert_eq!(roots.remove(old.id, Some(old.generation)), None);
        assert_eq!(roots.next(0), Some((last.id, 0x2000)));
        assert_eq!(roots.matching(0x2000), vec![last]);
        #[cfg(panic = "unwind")]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                roots.insert(&next, 0x3000);
            }));
            assert!(result.is_err());
        }
        assert_eq!(roots.len(), 1);
        assert_eq!(roots.remove(last.id, Some(last.generation)), Some(0x2000));
        assert_eq!(roots.len(), 0);
        assert_eq!(size_of::<TreeEntry>(), 16);
        assert!(size_of::<Registration>() <= 32);
    }

    #[test]
    fn packed_last_generation_and_slot_preserve_identity_without_widening_registrations() {
        assert_eq!(size_of::<SlotRoot>(), 16);
        assert!(size_of::<Registration>() <= 32);
        let roots = RootSlots::default();
        let ids: Vec<_> = (0..(SLOTS - 1)).map(|_| roots.insert(0x1000)).collect();
        let shard = ids[0].shard();
        lock(&roots.shards[shard].state).generation = MAX_GENERATION - 1;
        let last = roots.reserve(0x1000).commit(0x1000);
        assert_eq!(last.slot_and_generation.get(), u64::MAX);
        assert_eq!(last.id().generation.get(), MAX_GENERATION);
        assert_eq!(last.take_address(), Some(0x1000));
        if last.release() {
            roots.finish_removal(&last);
        }
        assert_eq!(last.take_address(), None);
        for id in ids {
            roots.remove(id);
        }
        drop(last);
        #[cfg(panic = "unwind")]
        {
            let exhausted =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| roots.reserve(0x1000)));
            assert!(exhausted.is_err());
        }
        roots.trim_empty();
        assert_eq!(roots.statistics().resident_pages, 0);
    }

    #[test]
    fn an_uncommitted_reservation_is_invisible_and_cancellation_releases_it() {
        let roots = RootSlots::default();
        let reserved = roots.reserve(0x1000);
        assert_eq!(roots.statistics().reserved_slots, 1);
        assert_eq!(roots.statistics().slot_roots, 0);
        let mut cursor = Cursor::default();
        loop {
            match roots.scan(&mut cursor) {
                Scan::Root(_) => panic!("reservation published an address"),
                Scan::Done => break,
                Scan::Advance => {}
            }
        }
        roots.cancel(reserved);
        assert_eq!(roots.statistics().reserved_slots, 0);
        roots.trim_empty();
        assert_eq!(roots.statistics().resident_pages, 0);
        let reserved = roots.reserve(0x1000);
        let id = reserved.commit(0x1000).id();
        assert_eq!(roots.statistics().slot_roots, 1);
        assert_eq!(roots.statistics().reserved_slots, 0);
        assert_eq!(roots.remove(id), Some(0x1000));
    }

    #[test]
    fn retried_root_reads_slot_reuse_and_skips_removal_without_replaying_other_slots() {
        for selected in [0, SLOTS - 1] {
            let roots = RootSlots::default();
            let ids: Vec<_> = (0..SLOTS).map(|_| roots.insert(0x1000)).collect();
            let mut cursor = Cursor::default();
            let mut scanned = 0;
            for _ in 0..SLOTS + SHARDS + 2 {
                if let Scan::Root(address) = roots.scan(&mut cursor) {
                    assert_eq!(address, 0x1000);
                    scanned += 1;
                    if scanned == selected + 1 {
                        break;
                    }
                }
            }
            assert_eq!(scanned, selected + 1);
            cursor.retry_root();
            let old = ids[selected];
            assert_eq!(roots.remove(old), Some(0x1000));
            // Reserve using the same shard hint but publish a different address.
            let replacement = roots.reserve(0x1000).commit(0x2000).id();
            assert_eq!(replacement.location, old.location);
            assert_ne!(replacement.generation, old.generation);
            assert!(matches!(roots.scan(&mut cursor), Scan::Root(0x2000)));
            cursor.retry_root();
            assert_eq!(roots.remove(replacement), Some(0x2000));
            let mut remaining = 0;
            for _ in 0..SLOTS + SHARDS + 2 {
                match roots.scan(&mut cursor) {
                    Scan::Root(address) => {
                        assert_eq!(address, 0x1000);
                        remaining += 1;
                    }
                    Scan::Advance => {}
                    Scan::Done => break,
                }
            }
            assert_eq!(remaining, SLOTS - selected - 1);
            assert!(matches!(roots.scan(&mut cursor), Scan::Done));
            for id in ids {
                roots.remove(id);
            }
            roots.trim_empty();
            drop(cursor);
            assert_eq!(roots.statistics().resident_pages, 0);
        }
    }

    #[test]
    fn a_cursor_pins_only_its_current_retired_page() {
        let roots = RootSlots::default();
        let ids: Vec<_> = (0..(SLOTS * 3)).map(|_| roots.insert(0x1000)).collect();
        let mut cursor = Cursor::default();
        while !matches!(roots.scan(&mut cursor), Scan::Root(_)) {}
        for id in ids {
            roots.remove(id);
        }
        roots.trim_empty();
        assert_eq!(roots.statistics().pages, 0);
        assert_eq!(roots.statistics().resident_pages, 1);
        let replacement = roots.insert(0x2000);
        assert_eq!(roots.statistics().resident_pages, 2);
        roots.remove(replacement);
        roots.trim_empty();
        assert_eq!(roots.statistics().resident_pages, 1);
        drop(cursor);
        assert_eq!(roots.statistics().resident_pages, 0);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn scan_completes_while_the_page_writer_is_stalled() {
        let roots = RootSlots::default();
        let root = roots.insert(0x1000);
        std::thread::scope(|scope| {
            let writer = lock(&roots.shards[root.shard()].state);
            let (send, receive) = std::sync::mpsc::channel();
            let roots = &roots;
            let scanner = scope.spawn(move || {
                let mut cursor = Cursor::default();
                let mut found = Vec::new();
                loop {
                    match roots.scan(&mut cursor) {
                        Scan::Root(address) => found.push(address),
                        Scan::Advance => {}
                        Scan::Done => break,
                    }
                }
                let matching = roots
                    .matching(0x1000)
                    .iter()
                    .map(SlotRoot::id)
                    .collect::<Vec<_>>();
                send.send((found, matching)).unwrap();
            });
            let result = receive.recv_timeout(std::time::Duration::from_secs(3));
            drop(writer); // release before asserting so regression cannot strand the thread
            scanner.join().unwrap();
            assert_eq!(result.unwrap(), (vec![0x1000], vec![root]));
        });
    }

    #[test]
    fn a_reservation_can_use_the_only_available_writer_and_keeps_its_physical_identity() {
        let roots = RootSlots::default();
        let original = roots.insert(0x1000);
        let free = (original.shard() + SHARDS - 1) & (SHARDS - 1);
        let mut writers = roots
            .shards
            .iter()
            .map(|shard| Some(lock(&shard.state)))
            .collect::<Vec<_>>();
        assert!(roots.available_writer(original.shard()).is_none());
        drop(writers[free].take());
        let alternate = roots.reserve(0x1000).commit(0x1000).id();
        assert_eq!(alternate.shard(), free);
        assert_ne!(alternate, original);
        drop(writers);
        assert_eq!(roots.remove(original), Some(0x1000));
        assert_eq!(roots.remove(original), None);
        assert_eq!(roots.remove(alternate), Some(0x1000));
        roots.trim_empty();
        assert_eq!(roots.statistics().resident_pages, 0);
    }

    #[test]
    fn reuse_rejects_stale_ids_and_duplicate_removal() {
        let roots = RootSlots::default();
        let old = roots.insert(0x1000);
        assert_eq!(roots.remove(old), Some(0x1000));
        let new = roots.insert(0x1000);
        assert_eq!(old.location, new.location);
        assert_ne!(old.generation, new.generation);
        assert_eq!(roots.remove(old), None);
        assert_eq!(roots.remove(new), Some(0x1000));
        assert_eq!(roots.remove(new), None);
        assert_eq!(roots.statistics().slot_reuses, 1);
    }

    #[test]
    fn scanning_counts_sparse_pages_and_empty_shards_with_bounded_steps() {
        let roots = RootSlots::default();
        let ids: Vec<_> = (0..(SLOTS * 3)).map(|_| roots.insert(0x1000)).collect();
        for &id in &ids[..(SLOTS * 2)] {
            assert_eq!(roots.remove(id), Some(0x1000));
        }
        let mut cursor = Cursor::default();
        let mut found = 0;
        let mut advances = 0;
        loop {
            match roots.scan(&mut cursor) {
                Scan::Root(address) => {
                    assert_eq!(address, 0x1000);
                    found += 1;
                }
                Scan::Advance => advances += 1,
                Scan::Done => break,
            }
            assert!(found + advances <= SLOTS + SHARDS + 2);
        }
        assert_eq!(found, SLOTS);
        assert_eq!(roots.statistics().pages, 2); // one live page and one cached empty page
        for id in ids {
            roots.remove(id);
        }
        assert_eq!(roots.statistics().pages, 1);
        roots.trim_empty();
        assert_eq!(roots.statistics().pages, 0);
        assert_eq!(
            roots.statistics().pages_created,
            roots.statistics().pages_released
        );
    }

    #[test]
    fn page_number_wrap_does_not_alias_live_pages_or_old_registrations() {
        let roots = RootSlots::default();
        let first = roots.insert(0x1000);
        let shard = first.shard();
        lock(&roots.shards[shard].state).next_page = MAX_PAGE;
        let ids: Vec<_> = (0..(2 * SLOTS)).map(|_| roots.insert(0x1000)).collect();
        assert!(ids.iter().any(|id| id.page() == MAX_PAGE));
        assert!(ids.iter().any(|id| id.page() == 1));
        assert_eq!(roots.remove(first), Some(0x1000));
        roots.trim_empty(); // never removes a live page
        for id in ids {
            assert_eq!(roots.remove(id), Some(0x1000));
        }
        roots.trim_empty();
        lock(&roots.shards[shard].state).next_page = 0;
        let new = roots.insert(0x1000);
        assert_eq!(new.location, first.location);
        assert_eq!(roots.remove(first), None);
        assert_eq!(roots.remove(new), Some(0x1000));
    }

    #[test]
    fn metadata_grouped_reuse_does_not_churn_root_pages() {
        // Sweep visits descriptors grouped by metadata shard. Reuse in this order
        // must not concentrate an entire rolling live set in one root shard.
        let roots = RootSlots::default();
        let mut live = std::collections::VecDeque::new();
        for _ in 0..4 {
            for shard in 0..SHARDS {
                for address in (1..=8192).map(|i| i * 16).filter(|address| {
                    crate::index::address_bucket(*address) & (SHARDS - 1) == shard
                }) {
                    live.push_back(roots.insert(address));
                    if live.len() > 512
                        && let Some(id) = live.pop_front()
                    {
                        roots.remove(id);
                    }
                }
            }
        }
        let statistics = roots.statistics();
        assert_eq!(statistics.slot_roots, 512);
        assert!(
            statistics.pages_created <= 32,
            "grouped descriptor reuse allocated {} root pages",
            statistics.pages_created
        );
        for id in live {
            roots.remove(id);
        }
        roots.trim_empty();
        let statistics = roots.statistics();
        assert_eq!(statistics.pages, 0);
        assert_eq!(statistics.pages_created, statistics.pages_released);
    }

    #[test]
    fn matching_and_trimming_preserve_other_roots() {
        let roots = RootSlots::default();
        let first = roots.insert(0x1000);
        let second = roots.insert(0x1000);
        let other = roots.insert(0x2000);
        assert_eq!(
            roots
                .matching(0x1000)
                .iter()
                .map(SlotRoot::id)
                .collect::<Vec<_>>(),
            vec![first, second]
        );
        roots.trim_empty();
        assert_eq!(roots.statistics().slot_roots, 3);
        assert_eq!(roots.remove(other), Some(0x2000));
    }
}
