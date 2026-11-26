//! Transfer physically returned storage charges to already waiting allocations.
use crate::GarbageCollector;
use crate::collector::AllocationReservation;
use crate::publication_directory::Directory;
use arc_swap::{ArcSwap, ArcSwapOption};
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
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
        self.retry.as_deref().unwrap_or(&self.request.entry().first)
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

    fn attempt(&self) -> Attempt {
        Attempt {
            request: self.clone(),
            retry: self.entry().retry.load_full(),
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
struct Page {
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
                .entry()
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

    pub(crate) fn advance(&mut self, budget: &mut usize) -> bool {
        let steps = budget.saturating_add(1).min(32);
        for i in 0..steps {
            let Some((next, request)) = self.next_request(self.cursor) else {
                return true;
            };
            self.cursor = next;
            if i != 0 {
                *budget -= 1;
            }
            let bytes = request.entry().bytes.load(Ordering::Relaxed);
            if bytes <= self.remaining
                && let Some(attempt) = request.activate()
                && attempt.generation().winner.load().is_none()
            {
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                tests::before_binding_commit();
                let donor = self
                    .donor
                    .get_or_insert_with(|| Arc::new(Donation::default()));
                if attempt.generation().claim(donor) {
                    self.remaining -= bytes;
                }
            }
            if self.remaining == 0 || self.cursor == self.limit {
                return true;
            }
        }
        false
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
