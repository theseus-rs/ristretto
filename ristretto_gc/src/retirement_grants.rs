//! Transfer physically returned storage charges to already waiting allocations.
use crate::GarbageCollector;
use crate::collector::AllocationReservation;
use crate::publication_directory::Directory;
use arc_swap::{ArcSwap, ArcSwapOption};
use once_cell::race::OnceBox;
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Weak};

// A direct Global layout is limited to isize::MAX bytes on both native and
// Wasm. The two larger sentinels cannot alias a sum of its disjoint shares.
const PAID: usize = usize::MAX;
const ABANDONED: usize = usize::MAX - 1;

#[derive(Debug, Default)]
struct Donation {
    // Until physical release, counts the shares canceled by their recipients.
    canceled: AtomicUsize,
    private: AtomicBool,
}

// Unselected entries close with one atomic operation. After the first selector
// arrives, each attempt keeps its irrevocable winner even if the owner cancels.
// A reopened request installs a fresh generation, so an old helper cannot bind it.
const OPEN: u8 = 0;
const ACTIVE: u8 = 1;
const CLOSED: u8 = 2;
const SEALED: usize = usize::MAX - 2;

#[derive(Debug, Default)]
struct Generation {
    spare: Arc<Donation>,
    winner: ArcSwapOption<Donation>,
}

impl Generation {
    fn claim(&self, donor: &Arc<Donation>) -> bool {
        let previous = self
            .winner
            .compare_and_swap(std::ptr::null::<Donation>(), Some(donor.clone()));
        previous
            .as_ref()
            .is_none_or(|winner| Arc::ptr_eq(winner, donor))
    }

    fn claim_new(&self, donor: &Arc<Donation>) -> bool {
        self.winner
            .compare_and_swap(std::ptr::null::<Donation>(), Some(donor.clone()))
            .is_none()
    }

    fn seal(&self) -> Option<Arc<Donation>> {
        if let Some(winner) = self.winner.load_full() {
            return Some(winner);
        }
        let closed = Donation {
            canceled: AtomicUsize::new(SEALED),
            ..Donation::default()
        };
        let previous = self
            .winner
            .compare_and_swap(std::ptr::null::<Donation>(), Some(Arc::new(closed)));
        previous.as_ref().cloned()
    }
}

#[derive(Debug, Default)]
struct Entry {
    bytes: AtomicUsize,
    stage: AtomicU8,
    generations: OnceBox<Generations>,
}

#[derive(Debug, Default)]
struct Generations {
    first: Generation,
    retry: ArcSwapOption<Generation>,
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    page: Arc<Page>,
    slot: usize,
}

#[derive(Debug, Clone)]
struct Attempt {
    request: Request,
    retry: Option<Arc<Generation>>,
}

impl Attempt {
    fn generation(&self) -> &Generation {
        self.retry
            .as_deref()
            .unwrap_or_else(|| &self.request.generations().first)
    }
}

impl Request {
    #[expect(
        clippy::indexing_slicing,
        reason = "created from the winning page offset"
    )]
    fn entry(&self) -> &Entry {
        &self.page.slots[self.slot]
    }

    pub(crate) fn arm(&self) -> Option<&crate::recovery::grant_wake::Arm> {
        self.page.sleepers.get()?.slots.get(self.slot)?.get()
    }

    pub(crate) fn install_first_arm(&self, arm: crate::recovery::grant_wake::Arm) -> bool {
        let page = self.page.sleepers.get_or_init(Box::default);
        let Some(slot) = page.slots.get(self.slot) else {
            return false;
        };
        let (ticket, bytes) = (arm.ticket, arm.bytes);
        if slot.set(arm).is_err() {
            return false;
        }
        if ticket < page.first_ticket.load(Ordering::Relaxed) {
            page.first_ticket.fetch_min(ticket, Ordering::Relaxed);
        }
        if bytes < page.smallest.load(Ordering::Relaxed) {
            page.smallest.fetch_min(bytes, Ordering::Relaxed);
        }
        page.occupied.fetch_or(1 << self.slot, Ordering::Release);
        true
    }

    pub(crate) fn mark_arm_ready(&self) {
        if let Some(page) = self.page.sleepers.get() {
            page.ready.fetch_or(1 << self.slot, Ordering::Release);
        }
    }

    pub(crate) fn clear_arm(&self) {
        if let Some(page) = self.page.sleepers.get() {
            page.occupied
                .fetch_and(!(1 << self.slot), Ordering::Relaxed);
        }
    }

    // All competing helpers may construct a candidate. OnceBox publishes one
    // allocation and has no initializer lock that a parked preparer can retain.
    pub(crate) fn prepare(&self) {
        if self.entry().generations.get().is_some() {
            return;
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        crate::refund_tests::park_selection(crate::refund_tests::SelectionPoint::BeforePrepare);
        self.entry().generations.get_or_init(Box::default);
    }

    #[expect(
        clippy::expect_used,
        reason = "generation storage is published before activating a request"
    )]
    fn generations(&self) -> &Generations {
        self.entry()
            .generations
            .get()
            .expect("active generation storage")
    }

    fn attempt(&self) -> Attempt {
        Attempt {
            request: self.clone(),
            retry: self.generations().retry.load_full(),
        }
    }

    fn activate(&self) -> Option<Attempt> {
        match self
            .entry()
            .stage
            .compare_exchange(OPEN, ACTIVE, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) | Err(ACTIVE) => Some(self.attempt()),
            _ => None,
        }
    }

    fn donor(&self) -> Option<Arc<Donation>> {
        if self.entry().stage.load(Ordering::Acquire) != ACTIVE {
            return None;
        }
        self.attempt().generation().winner.load_full()
    }

    pub(crate) fn paid(&self) -> bool {
        self.donor()
            .is_some_and(|d| d.canceled.load(Ordering::Acquire) == PAID)
    }

    pub(crate) fn private(&self) -> bool {
        self.donor()
            .is_some_and(|d| d.private.load(Ordering::Acquire))
    }

    pub(crate) fn waiting(&self) -> bool {
        self.entry().stage.load(Ordering::Acquire) != CLOSED
            && self
                .donor()
                .is_none_or(|d| d.canceled.load(Ordering::Acquire) == ABANDONED)
    }
}

const PAGE_WIDTH: usize = 32;
type Pages = Directory<Arc<Page>>;

#[derive(Debug)]
struct Sleepers {
    slots: [std::sync::OnceLock<crate::recovery::grant_wake::Arm>; PAGE_WIDTH],
    occupied: AtomicU32,
    ready: AtomicU32,
    smallest: AtomicUsize,
    first_ticket: AtomicU64,
    grants: AtomicU64,
}

impl Default for Sleepers {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| std::sync::OnceLock::new()),
            occupied: AtomicU32::new(0),
            ready: AtomicU32::new(0),
            smallest: AtomicUsize::new(usize::MAX),
            first_ticket: AtomicU64::new(u64::MAX),
            grants: AtomicU64::new(0),
        }
    }
}

#[derive(Debug)]
struct Page {
    sleepers: OnceBox<Sleepers>,
    slots: [Entry; PAGE_WIDTH],
    // Includes unissued positions. A page can only be removed after every
    // position has been committed and its own demand has closed.
    remaining: AtomicUsize,
}

impl Page {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the final partial page has at most PAGE_WIDTH entries"
    )]
    fn new(key: u64) -> Self {
        Self {
            sleepers: OnceBox::new(),
            slots: std::array::from_fn(|_| Entry::default()),
            remaining: AtomicUsize::new(
                (u64::MAX - key * PAGE_WIDTH as u64).min(PAGE_WIDTH as u64) as usize,
            ),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Registry {
    pages: ArcSwap<Pages>,
    // Each increment commits the already linked slot at the prior position.
    published: AtomicU64,
    active: AtomicUsize,
}

pub(crate) struct Demand<'a> {
    gc: &'a GarbageCollector,
    page_key: u64,
    pub(crate) request: Request,
}

impl Registry {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn has_demand(&self) -> bool {
        self.active.load(Ordering::Acquire) != 0
    }

    pub(crate) fn visit_arms(
        &self,
        scan: crate::recovery::grant_wake::Scan,
        mut visit: impl FnMut(&Request, &crate::recovery::grant_wake::Arm),
    ) {
        let pages = self.pages.load_full();
        let mut after = None;
        while let Some((key, page)) = pages.next_after(after) {
            after = Some(key);
            let Some(sleepers) = page.sleepers.get() else {
                continue;
            };
            let mut occupied = sleepers.occupied.load(Ordering::Acquire);
            if !scan.all
                && scan.available < sleepers.smallest.load(Ordering::Relaxed)
                && scan.ticket < sleepers.first_ticket.load(Ordering::Relaxed)
                && scan.grants != u64::MAX
                && scan.grants <= sleepers.grants.load(Ordering::Acquire)
            {
                occupied &= sleepers.ready.load(Ordering::Acquire);
            }
            while occupied != 0 {
                let slot = occupied.trailing_zeros() as usize;
                occupied &= occupied - 1;
                if let Some(arm) = sleepers.slots.get(slot).and_then(std::sync::OnceLock::get) {
                    visit(
                        &Request {
                            page: page.clone(),
                            slot,
                        },
                        arm,
                    );
                }
            }
            if scan.grants > sleepers.grants.load(Ordering::Relaxed) {
                sleepers.grants.fetch_max(scan.grants, Ordering::Release);
            }
        }
    }

    fn page(&self, position: u64) -> crate::Result<Option<Arc<Page>>> {
        let key = position / PAGE_WIDTH as u64;
        let head = self.pages.load();
        if let Some(page) = head.get(key) {
            return Ok(Some(page.clone()));
        }
        // A stale publisher may still hold an old position after the entire
        // page was removed. Never resurrect it in a newer directory.
        if self.published.load(Ordering::Acquire) != position {
            return Ok(None);
        }
        let page = Arc::new(Page::new(key));
        let (next, _) = head
            .insert(key, page.clone())
            .ok_or(crate::Error::OutOfMemory)?;
        let previous = self.pages.compare_and_swap(&*head, Arc::new(next));
        Ok(Arc::ptr_eq(&previous, &head).then_some(page))
    }

    pub(crate) fn register<'a>(
        &self,
        gc: &'a GarbageCollector,
        bytes: usize,
    ) -> crate::Result<Demand<'a>> {
        self.active.fetch_add(1, Ordering::AcqRel);
        let result = self.publish(gc, bytes);
        if result.is_err() {
            self.active.fetch_sub(1, Ordering::Release);
        }
        result
    }

    #[expect(
        clippy::indexing_slicing,
        clippy::cast_possible_truncation,
        reason = "the remainder is a page offset on every supported target"
    )]
    fn publish<'a>(&self, gc: &'a GarbageCollector, bytes: usize) -> crate::Result<Demand<'a>> {
        // A zero-byte reservation always succeeds in the ordinary budget.
        let bytes = std::num::NonZeroUsize::new(bytes).ok_or(crate::Error::OutOfMemory)?;
        loop {
            let position = self.published.load(Ordering::Acquire);
            let next = position.checked_add(1).ok_or(crate::Error::OutOfMemory)?;
            let Some(page) = self.page(position)? else {
                continue;
            };
            let index = (position % PAGE_WIDTH as u64) as usize;
            let ours = page.slots[index]
                .bytes
                .compare_exchange(0, bytes.get(), Ordering::AcqRel, Ordering::Acquire)
                .is_ok();
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            if ours {
                tests::after_link();
            }
            // An enroller paused after linking cannot delay peers. Exactly one
            // helper commits this position; the owning request remains pinned.
            let _ = self.published.compare_exchange(
                position,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            if ours {
                return Ok(Demand {
                    gc,
                    page_key: position / PAGE_WIDTH as u64,
                    request: Request { page, slot: index },
                });
            }
        }
    }

    fn release(&self, page_key: u64, page: &Arc<Page>) {
        if page.remaining.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.pages.rcu(|head| {
                let (directory, _) = head.remove(page_key);
                Arc::new(directory)
            });
        }
        self.active.fetch_sub(1, Ordering::Release);
    }

    fn selection(&self, collector: Weak<GarbageCollector>, size: usize) -> Selection {
        // If capture races a registrar, the counter alone determines which
        // side it falls on. Every position below it is already materialized.
        let limit = self.published.load(Ordering::Acquire);
        let pages = self.pages.load_full();
        Selection {
            collector,
            pages,
            limit,
            donor: None,
            cursor: 0,
            size,
            remaining: size,
            settled: false,
        }
    }

    pub(crate) fn capture(&self, metadata: &crate::metadata::ObjectMetadata) -> Option<Selection> {
        if self.active.load(Ordering::Acquire) == 0 || !metadata.can_transfer_retired_charge() {
            return None;
        }
        let size = metadata.size();
        debug_assert!(isize::try_from(size).is_ok());
        Some(self.selection(metadata.collector.clone(), size))
    }
}

impl<'a> Demand<'a> {
    pub(crate) fn take(&self) -> Option<AllocationReservation<'a>> {
        let donor = self.request.donor()?;
        let state = donor.canceled.load(Ordering::Acquire);
        if state == ABANDONED {
            // Only the live demand can reopen its current attempt.
            self.request
                .generations()
                .retry
                .store(Some(Arc::new(Generation::default())));
            return None;
        }
        if state != PAID {
            return None;
        }
        self.request.entry().stage.store(CLOSED, Ordering::Release);
        Some(AllocationReservation::transferred(
            self.gc,
            self.request.entry().bytes.load(Ordering::Relaxed),
        ))
    }

    pub(crate) fn finish_uncovered(&self) -> bool {
        let entry = self.request.entry();
        match entry
            .stage
            .compare_exchange(OPEN, CLOSED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => true,
            Err(ACTIVE) if self.request.attempt().generation().seal().is_none() => {
                entry.stage.store(CLOSED, Ordering::Release);
                true
            }
            _ => false,
        }
    }
}

impl Drop for Demand<'_> {
    fn drop(&mut self) {
        let entry = self.request.entry();
        if entry.stage.swap(CLOSED, Ordering::AcqRel) == ACTIVE
            && let Some(donor) = self.request.attempt().generation().seal()
        {
            let bytes = entry.bytes.load(Ordering::Relaxed);
            let canceled = donor
                .canceled
                .try_update(Ordering::AcqRel, Ordering::Acquire, |old| {
                    (old < SEALED).then(|| old + bytes)
                });
            if canceled == Err(PAID) {
                self.gc.cancel_allocation(bytes);
            }
        }
        self.gc
            .grant_registry
            .release(self.page_key, &self.request.page);
    }
}

#[derive(Debug)]
pub(crate) struct Selection {
    collector: Weak<GarbageCollector>,
    pages: Arc<Pages>,
    limit: u64,
    donor: Option<Arc<Donation>>,
    size: usize,
    settled: bool,
    cursor: u64,
    remaining: usize,
}

pub(crate) enum ScanProgress {
    More,
    Done,
    Prepare(Request),
}

impl Selection {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the remainder is a page offset"
    )]
    fn next_request(&self, cursor: u64) -> Option<(u64, Request)> {
        if cursor >= self.limit {
            return None;
        }
        let key = cursor / PAGE_WIDTH as u64;
        let (found, page) = self.pages.next_after(key.checked_sub(1))?;
        let position = cursor.max(found * PAGE_WIDTH as u64);
        if position >= self.limit {
            return None;
        }
        Some((
            position + 1,
            Request {
                page: page.clone(),
                slot: (position % PAGE_WIDTH as u64) as usize,
            },
        ))
    }

    #[cfg(test)]
    pub(crate) fn advance(&mut self, budget: &mut usize) -> bool {
        loop {
            match self.scan(budget) {
                ScanProgress::More => return false,
                ScanProgress::Done => return true,
                ScanProgress::Prepare(request) => request.prepare(),
            }
        }
    }

    pub(crate) fn scan(&mut self, budget: &mut usize) -> ScanProgress {
        let steps = budget.saturating_add(1).min(32);
        for i in 0..steps {
            let Some((next, request)) = self.next_request(self.cursor) else {
                return ScanProgress::Done;
            };
            let entry = request.entry();
            let bytes = entry.bytes.load(Ordering::Relaxed);
            if bytes <= self.remaining && entry.stage.load(Ordering::Acquire) != CLOSED {
                if entry.generations.get().is_none() {
                    return ScanProgress::Prepare(request);
                }
                if let Some(attempt) = request.activate() {
                    let generation = attempt.generation();
                    if generation.winner.load().is_none() {
                        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                        tests::before_binding_commit();
                        let donor = self.donor.as_ref().unwrap_or(&generation.spare);
                        // Two ordinary selectors racing the first slot propose
                        // the same spare. Only its successful NULL CAS owns it.
                        if generation.claim_new(donor) {
                            self.donor = Some(donor.clone());
                            self.remaining -= bytes;
                        }
                    }
                }
            }
            self.cursor = next;
            if i != 0 {
                *budget -= 1;
            }
            if self.remaining == 0 || self.cursor == self.limit {
                return ScanProgress::Done;
            }
        }
        ScanProgress::More
    }

    pub(crate) fn mark_private(&self) {
        if let Some(donor) = &self.donor {
            donor.private.store(true, Ordering::Release);
        }
    }

    // Every assigned share is either still chargeable to its exact request or
    // already recorded as canceled. One atomic publication settles them all.
    pub(crate) fn pay(&mut self) -> usize {
        self.settled = true;
        self.donor.as_ref().map_or(0, |donor| {
            let canceled = donor.canceled.swap(PAID, Ordering::AcqRel);
            self.size - self.remaining - canceled
        })
    }
}

impl Drop for Selection {
    fn drop(&mut self) {
        if !self.settled
            && let Some(donor) = &self.donor
        {
            donor.canceled.store(ABANDONED, Ordering::Release);
            if self.remaining < self.size
                && let Some(collector) = self.collector.upgrade()
            {
                collector.signal_grants();
            }
        }
    }
}

#[cfg(test)]
#[path = "retirement_grants_tests.rs"]
mod tests;

#[path = "released_donors.rs"]
mod released;
pub(crate) use released::{ReleasedCheck, ReleasedDonors, ReleasedWork};
