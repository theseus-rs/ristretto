//! Reference handlers register before checking the linked pending queue. A
//! broadcast records the last eligible ticket on each page. Cancellation and
//! delivery compete for the same ticket before moving its callback.
use super::{GarbageCollector, Poll, Recovery, WakeFailures};
use arc_swap::{ArcSwap, ArcSwapOption};
use once_cell::race::OnceBox;
use portable_atomic::AtomicU64;
use std::cell::UnsafeCell;
use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Waker;

const SLOTS: usize = 63;
const SEALED: u64 = 1 << SLOTS;

#[derive(Default)]
struct Slot {
    ticket: AtomicU64,
    callback: UnsafeCell<Option<Waker>>,
}

// SAFETY: only the owner of an unpublished occupied bit writes the callback.
// Once the ticket is published, only its terminal CAS winner accesses it. That
// winner empties the cell before releasing the bit for the next reservation.
unsafe impl Sync for Slot {}

struct Page {
    occupied: AtomicU64,
    issued: AtomicU64,
    slots: [Slot; SLOTS],
}

impl fmt::Debug for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Page")
            .field("occupied", &self.occupied)
            .field("issued", &self.issued)
            .finish_non_exhaustive()
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            occupied: AtomicU64::new(0),
            issued: AtomicU64::new(0),
            slots: std::array::from_fn(|_| Slot::default()),
        }
    }
}

impl Page {
    #[expect(
        clippy::indexing_slicing,
        reason = "only bits below SEALED designate slots"
    )]
    fn slot(&self, index: usize) -> &Slot {
        &self.slots[index]
    }

    fn reserve(&self) -> Option<(usize, NonZeroU64)> {
        let mut used = self.occupied.load(Ordering::Relaxed);
        loop {
            let index = used.trailing_ones() as usize;
            if index >= SLOTS || used & SEALED != 0 {
                return None;
            }
            let bit = 1 << index;
            match self.occupied.compare_exchange_weak(
                used,
                used | bit,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    if let Ok(last) =
                        self.issued
                            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                                v.checked_add(1)
                            })
                    {
                        return NonZeroU64::new(last + 1).map(|ticket| (index, ticket));
                    }
                    // Exhausted ticket values retire this page permanently.
                    self.occupied.fetch_or(SEALED, Ordering::Relaxed);
                    self.occupied.fetch_and(!bit, Ordering::Release);
                    return None;
                }
                Err(actual) => used = actual,
            }
        }
    }
}

#[derive(Debug)]
struct Broadcast(Vec<(Arc<Page>, u64)>);

#[derive(Debug, Default)]
struct Directory {
    first: Arc<Page>,
    extra: ArcSwap<Vec<Arc<Page>>>,
    ready: ArcSwapOption<Broadcast>,
}

#[derive(Debug, Default)]
pub(super) struct ReferenceWaiters {
    directory: OnceBox<Directory>,
    // Enrollment and notification share a modification order even on the
    // first page, before the notifier knows that a directory exists.
    enrolled: AtomicBool,
}

struct Registration<'a> {
    directory: &'a Directory,
    extra: Option<Arc<Page>>,
    index: usize,
    ticket: NonZeroU64,
}

impl Registration<'_> {
    fn page(&self) -> &Arc<Page> {
        self.extra.as_ref().unwrap_or(&self.directory.first)
    }
}

impl Drop for Registration<'_> {
    fn drop(&mut self) {
        self.directory.take(self.page(), self.index, self.ticket);
    }
}

impl Directory {
    fn reserve(&self) -> (Option<Arc<Page>>, usize, NonZeroU64) {
        if let Some((index, ticket)) = self.first.reserve() {
            return (None, index, ticket);
        }
        loop {
            let snapshot = self.extra.load();
            for page in snapshot.iter().rev() {
                if let Some((index, ticket)) = page.reserve() {
                    return (Some(page.clone()), index, ticket);
                }
            }
            let mut next: Vec<_> = snapshot
                .iter()
                .filter(|p| p.occupied.load(Ordering::Acquire) != SEALED)
                .cloned()
                .collect();
            next.push(Arc::new(Page::default()));
            self.extra.compare_and_swap(&*snapshot, Arc::new(next));
        }
    }

    fn register(&self, waker: &Waker) -> Registration<'_> {
        // Clone before reserving a bit. User clone/drop and wake may reenter.
        let callback = waker.clone();
        let (extra, index, ticket) = self.reserve();
        let registration = Registration {
            directory: self,
            extra,
            index,
            ticket,
        };
        let slot = registration.page().slot(index);
        // SAFETY: reserve acquired this bit after its previous owner emptied
        // the cell. No current ticket can claim it until the following store.
        unsafe { *slot.callback.get() = Some(callback) };
        slot.ticket.store(ticket.get(), Ordering::Release);
        registration
    }

    fn take(&self, page: &Arc<Page>, index: usize, ticket: NonZeroU64) -> Option<Waker> {
        let slot = page.slot(index);
        slot.ticket
            .compare_exchange(ticket.get(), 0, Ordering::AcqRel, Ordering::Relaxed)
            .ok()?;
        // The reservation and then its single winning claimant have exclusive
        // access to the callback until the occupancy bit is returned.
        // SAFETY: the acquire CAS won this distinct nonzero ticket and sees
        // its installed waker. Reuse waits for the release below.
        let callback = unsafe { (*slot.callback.get()).take() };
        let bit = 1 << index;
        let old = page.occupied.fetch_and(!bit, Ordering::AcqRel);
        if old & !SEALED == bit && !Arc::ptr_eq(page, &self.first) {
            self.trim(page);
        }
        callback
    }

    fn trim(&self, page: &Arc<Page>) {
        if let Err(other) =
            page.occupied
                .compare_exchange(0, SEALED, Ordering::AcqRel, Ordering::Relaxed)
            && other != SEALED
        {
            return;
        }
        self.extra.rcu(|old| {
            old.iter()
                .filter(|p| !Arc::ptr_eq(p, page))
                .cloned()
                .collect::<Vec<_>>()
        });
    }

    fn signal(&self, gc: &GarbageCollector) {
        loop {
            let previous = self.ready.load();
            let extra = self.extra.load();
            let cutoffs: Vec<_> = std::iter::once(&self.first)
                .chain(extra.iter())
                .filter(|p| p.occupied.load(Ordering::Acquire) & !SEALED != 0)
                .map(|p| (p.clone(), p.issued.load(Ordering::Acquire)))
                .collect();
            if cutoffs.is_empty() || !gc.has_pending_references() {
                return;
            }
            let old = self
                .ready
                .compare_and_swap(&*previous, Some(Arc::new(Broadcast(cutoffs))));
            let replaced = match (&*previous, &*old) {
                (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            };
            if replaced {
                return;
            }
        }
    }

    fn deliver(&self, failures: &mut WakeFailures) {
        let Some(notice) = self.ready.load_full() else {
            return;
        };
        for (page, cutoff) in &notice.0 {
            let mut occupied = page.occupied.load(Ordering::Acquire) & !SEALED;
            while occupied != 0 {
                let index = occupied.trailing_zeros() as usize;
                occupied &= occupied - 1;
                let ticket = page.slot(index).ticket.load(Ordering::Acquire);
                if let Some(ticket) = NonZeroU64::new(ticket)
                    && ticket.get() <= *cutoff
                    && let Some(waker) = self.take(page, index, ticket)
                {
                    failures.deliver(|| waker.wake());
                }
            }
        }
        self.ready.compare_and_swap(&notice, None);
    }
}

// A synchronous executor may reenter the collector from its wake callback.
// A second thread can still claim callbacks from the same published notice.
thread_local! {
    static DELIVERING: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(Vec::new()) };
}

struct Delivery(usize);
impl Delivery {
    fn enter(directory: &Directory) -> Option<Self> {
        let key = std::ptr::from_ref(directory).addr();
        DELIVERING.with_borrow_mut(|active| {
            if active.contains(&key) {
                None
            } else {
                active.push(key);
                Some(Self(key))
            }
        })
    }
}
impl Drop for Delivery {
    fn drop(&mut self) {
        DELIVERING.with_borrow_mut(|active| active.retain(|key| *key != self.0));
    }
}

impl Recovery {
    pub(crate) async fn wait_for_references(&self, gc: &GarbageCollector) {
        if gc.has_pending_references() {
            return;
        }
        let directory = self
            .references
            .directory
            .get_or_init(|| Box::new(Directory::default()));
        let mut registration = None;
        std::future::poll_fn(|cx| {
            drop(registration.take());
            registration = Some(directory.register(cx.waker()));
            self.references.enrolled.swap(true, Ordering::AcqRel);
            if gc.has_pending_references() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    pub(crate) fn reference_delivery_pending(&self) -> bool {
        self.references
            .directory
            .get()
            .is_some_and(|directory| directory.ready.load().is_some())
    }

    pub(super) fn notify_reference_handlers(
        &self,
        gc: &GarbageCollector,
        failures: &mut WakeFailures,
    ) {
        if gc.has_pending_references()
            && self.references.enrolled.fetch_or(false, Ordering::AcqRel)
            && let Some(directory) = self.references.directory.get()
        {
            directory.signal(gc);
        }
        if let Some(directory) = self.references.directory.get()
            && directory.ready.load().is_some()
            && let Some(_delivery) = Delivery::enter(directory)
        {
            while directory.ready.load().is_some() {
                directory.deliver(failures);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::task::Wake;

    #[derive(Default)]
    struct Count(AtomicUsize);
    impl Wake for Count {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Release);
        }
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn panicking_reference_handler_does_not_consume_a_peers_wake() {
        struct Panics;
        impl Wake for Panics {
            fn wake(self: Arc<Self>) {
                panic!("executor stopped");
            }
        }
        let directory = Directory::default();
        let first = directory.register(&Waker::from(Arc::new(Panics)));
        let healthy = Arc::new(Count::default());
        let second = directory.register(&Waker::from(healthy.clone()));
        assert!(Arc::ptr_eq(first.page(), second.page()));
        directory.ready.store(Some(Arc::new(Broadcast(vec![(
            second.page().clone(),
            second.ticket.get(),
        )]))));
        let mut failures = WakeFailures::default();
        directory.deliver(&mut failures);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| failures.finish())).is_err()
        );
        assert_eq!(healthy.0.load(Ordering::Acquire), 1);
        assert!(directory.ready.load().is_none());
    }

    #[test]
    fn old_broadcast_and_cancel_cannot_take_a_reused_reference_slot() {
        let directory = Directory::default();
        let first_waker = Arc::new(Count::default());
        let second_waker = Arc::new(Count::default());
        let first = directory.register(&Waker::from(first_waker.clone()));
        let notice = Arc::new(Broadcast(vec![(first.page().clone(), first.ticket.get())]));
        directory.ready.store(Some(notice.clone()));
        directory.deliver(&mut WakeFailures::default());
        assert_eq!(first_waker.0.load(Ordering::Acquire), 1);

        let second = directory.register(&Waker::from(second_waker.clone()));
        assert_eq!(first.index, second.index);
        assert!(Arc::ptr_eq(first.page(), second.page()));
        assert_ne!(first.ticket, second.ticket);
        drop(first);
        // A helper retains the original cutoff across a complete slot reuse.
        directory.ready.store(Some(notice));
        directory.deliver(&mut WakeFailures::default());
        assert_eq!(second_waker.0.load(Ordering::Acquire), 0);
        assert!(
            directory
                .take(second.page(), second.index, second.ticket)
                .is_some()
        );
    }

    #[test]
    fn stale_page_cannot_enrol_a_reference_after_its_last_cancellation() {
        let directory = Directory::default();
        let mut registrations: Vec<_> = (0..=SLOTS)
            .map(|_| directory.register(Waker::noop()))
            .collect();
        let extra = directory.extra.load()[0].clone();
        drop(registrations.pop());
        assert!(directory.extra.load().is_empty());
        assert!(extra.reserve().is_none());
        let next = directory.register(Waker::noop());
        assert!(!Arc::ptr_eq(&extra, next.page()));
    }

    #[test]
    fn ticket_exhaustion_keeps_live_callbacks_and_rejects_old_claims() {
        let directory = Directory::default();
        directory
            .first
            .issued
            .store(u64::MAX - 1, Ordering::Relaxed);
        let last = directory.register(Waker::noop());
        let next = directory.register(Waker::noop());
        assert_eq!(last.ticket.get(), u64::MAX);
        assert!(!Arc::ptr_eq(last.page(), next.page()));
        assert_eq!(next.ticket.get(), 1);
        drop(last);
        assert!(directory.first.reserve().is_none());
        assert!(
            directory
                .take(next.page(), next.index, next.ticket)
                .is_some()
        );
    }
}
