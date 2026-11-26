//! Transfer physically returned storage charges to already waiting allocations.
use crate::GarbageCollector;
use crate::collector::AllocationReservation;
use crate::publication_directory::Directory;
use arc_swap::{ArcSwap, ArcSwapOption};
use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, Weak};

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

// Slots are issued once. The publishing CAS supplies the byte count and the
// winning caller owns the only Demand. Receipts are only touched for a grant.
const OPEN: u8 = 0;
const ASSIGNING: u8 = 1;
const BOUND: u8 = 2;
const CLOSED: u8 = 3;

#[derive(Debug)]
struct Entry {
    bytes: AtomicUsize,
    stage: AtomicU8,
    binding: OnceLock<Binding>,
}

// Receipt storage is separate from membership: a bound entry can retain
// another entry's receipt without keeping either membership page alive.
#[derive(Clone, Debug)]
struct Receipt {
    bank: Arc<[Donation]>,
    slot: usize,
}

impl std::ops::Deref for Receipt {
    type Target = Donation;
    #[expect(
        clippy::indexing_slicing,
        reason = "the slot belongs to this receipt bank"
    )]
    fn deref(&self) -> &Donation {
        &self.bank[self.slot]
    }
}

#[derive(Debug)]
struct Binding {
    first: Receipt,
    retry: ArcSwapOption<Retry>,
}

#[derive(Debug)]
struct Retry {
    spare: Receipt,
    donor: OnceLock<Receipt>,
}

impl Binding {
    fn current(&self) -> Option<Receipt> {
        match self.retry.load().as_ref() {
            Some(retry) => retry.donor.get().cloned(),
            None => Some(self.first.clone()),
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            bytes: AtomicUsize::new(0),
            stage: AtomicU8::new(OPEN),
            binding: OnceLock::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    page: Arc<Page>,
    slot: usize,
}

impl Request {
    #[expect(
        clippy::indexing_slicing,
        reason = "created from the winning page offset"
    )]
    fn entry(&self) -> &Entry {
        &self.page.slots[self.slot]
    }

    fn bound(&self, ready: impl FnOnce(&Donation) -> bool) -> bool {
        let entry = self.entry();
        entry.stage.load(Ordering::Acquire) == BOUND
            && entry
                .binding
                .get()
                .and_then(Binding::current)
                .is_some_and(|donor| ready(&donor))
    }

    pub(crate) fn paid(&self) -> bool {
        self.bound(|donor| donor.canceled.load(Ordering::Acquire) == PAID)
    }

    pub(crate) fn private(&self) -> bool {
        self.bound(|donor| donor.private.load(Ordering::Acquire))
    }

    pub(crate) fn waiting(&self) -> bool {
        self.entry().stage.load(Ordering::Acquire) == OPEN
            || self.bound(|donor| donor.canceled.load(Ordering::Acquire) == ABANDONED)
    }
}

const PAGE_WIDTH: usize = 32;
type Pages = Directory<Arc<Page>>;

#[derive(Debug)]
struct Page {
    bank: Arc<[Donation]>,
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
            bank: Arc::new(std::array::from_fn::<_, PAGE_WIDTH, _>(|_| {
                Donation::default()
            })),
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
        let entry = self.request.entry();
        if entry.stage.load(Ordering::Acquire) != BOUND {
            return None;
        }
        let binding = entry.binding.get()?;
        let state = binding.current()?.canceled.load(Ordering::Acquire);
        if state == ABANDONED {
            // Only the owner reopens a request. Prepare both the unused
            // receipt and the next one-shot binding before publishing OPEN.
            binding.retry.store(Some(Arc::new(Retry {
                spare: Receipt {
                    bank: Arc::new([Donation::default()]),
                    slot: 0,
                },
                donor: OnceLock::new(),
            })));
            entry.stage.store(OPEN, Ordering::Release);
            return None;
        }
        if state != PAID {
            return None;
        }
        // Only the owning Demand consumes or cancels a bound request.
        entry.stage.store(CLOSED, Ordering::Release);
        Some(AllocationReservation::transferred(
            self.gc,
            entry.bytes.load(Ordering::Relaxed),
        ))
    }

    pub(crate) fn finish_uncovered(&self) -> bool {
        self.request
            .entry()
            .stage
            .compare_exchange(OPEN, CLOSED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

impl Drop for Demand<'_> {
    fn drop(&mut self) {
        let entry = self.request.entry();
        if entry.stage.swap(CLOSED, Ordering::AcqRel) == BOUND
            && let Some(binding) = entry.binding.get()
            && let Some(donor) = binding.current()
        {
            let bytes = entry.bytes.load(Ordering::Relaxed);
            let canceled = donor
                .canceled
                .try_update(Ordering::AcqRel, Ordering::Acquire, |old| {
                    (old < ABANDONED).then(|| old + bytes)
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
    donor: Option<Receipt>,
    size: usize,
    settled: bool,
    cursor: u64,
    remaining: usize,
}

impl Selection {
    // The task stays at the front of the public sweep queue through all batches.
    #[expect(
        clippy::indexing_slicing,
        clippy::cast_possible_truncation,
        clippy::expect_used,
        reason = "page offsets fit usize and only the exclusive assignment owner initializes a binding"
    )]
    pub(crate) fn advance(&mut self, budget: &mut usize) -> bool {
        let steps = budget.saturating_add(1).min(32);
        for i in 0..steps {
            if self.cursor >= self.limit {
                return true;
            }
            let page_key = self.cursor / PAGE_WIDTH as u64;
            let Some((key, page)) = self.pages.next_after(page_key.checked_sub(1)) else {
                return true;
            };
            if key != page_key {
                self.cursor = key * PAGE_WIDTH as u64;
                if self.cursor >= self.limit {
                    return true;
                }
            }
            let slot = (self.cursor % PAGE_WIDTH as u64) as usize;
            self.cursor += 1;
            if i != 0 {
                *budget -= 1;
            }
            let entry = &page.slots[slot];
            let bytes = entry.bytes.load(Ordering::Relaxed);
            if bytes <= self.remaining
                && entry
                    .stage
                    .compare_exchange(OPEN, ASSIGNING, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
            {
                // ASSIGNING gives one helper exclusive initialization rights.
                // Both the default bank and any retry storage are preallocated.
                if let Some(binding) = entry.binding.get() {
                    let retry = binding.retry.load();
                    let retry = retry.as_ref().expect("reopened binding has a retry");
                    let donor = self.donor.get_or_insert_with(|| retry.spare.clone());
                    retry
                        .donor
                        .set(donor.clone())
                        .expect("unique retry assignment");
                } else {
                    let donor = self.donor.get_or_insert_with(|| Receipt {
                        bank: page.bank.clone(),
                        slot,
                    });
                    entry
                        .binding
                        .set(Binding {
                            first: donor.clone(),
                            retry: ArcSwapOption::empty(),
                        })
                        .expect("unique first assignment");
                }
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                tests::before_binding_commit();
                if entry
                    .stage
                    .compare_exchange(ASSIGNING, BOUND, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
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
