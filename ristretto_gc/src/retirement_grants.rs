//! Transfer physically returned storage charges to already waiting allocations.
use crate::GarbageCollector;
use crate::collector::AllocationReservation;
use crate::publication_directory::Directory;
use arc_swap::{ArcSwap, ArcSwapOption};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Weak};

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

// CASing a request to this unique Arc permanently ends its registration.
static CLOSED: LazyLock<Arc<Donation>> = LazyLock::new(|| Arc::new(Donation::default()));

#[derive(Debug)]
pub(crate) struct Request {
    bytes: usize,
    donor: ArcSwapOption<Donation>,
    // Prepared on the allocating thread. The sweep coordinator never allocates a receipt.
    spare: ArcSwapOption<Donation>,
}

impl Request {
    pub(crate) fn paid(&self) -> bool {
        self.donor
            .load()
            .as_ref()
            .is_some_and(|d| d.canceled.load(Ordering::Acquire) == PAID)
    }

    pub(crate) fn private(&self) -> bool {
        self.donor
            .load()
            .as_ref()
            .is_some_and(|donor| donor.private.load(Ordering::Acquire))
    }

    pub(crate) fn waiting(&self) -> bool {
        self.donor
            .load()
            .as_ref()
            .is_none_or(|d| d.canceled.load(Ordering::Acquire) == ABANDONED)
    }
}

#[derive(Debug, Default)]
struct Membership {
    directory: Directory<Weak<Request>>,
    last_id: u64,
}

#[derive(Debug, Default)]
pub(crate) struct Registry {
    entries: ArcSwap<Membership>,
    active: AtomicUsize,
}

pub(crate) struct Demand<'a> {
    gc: &'a GarbageCollector,
    id: u64,
    pub(crate) request: Arc<Request>,
}

impl Registry {
    pub(crate) fn register<'a>(
        &self,
        gc: &'a GarbageCollector,
        bytes: usize,
    ) -> crate::Result<Demand<'a>> {
        let request = Arc::new(Request {
            bytes,
            donor: ArcSwapOption::empty(),
            spare: ArcSwapOption::from(Some(Arc::new(Donation::default()))),
        });
        let mut head = self.entries.load();
        loop {
            let id = head
                .last_id
                .checked_add(1)
                .ok_or(crate::Error::OutOfMemory)?;
            let (directory, _) = head
                .directory
                .insert(id, Arc::downgrade(&request))
                .ok_or(crate::Error::OutOfMemory)?;
            let current = self.entries.compare_and_swap(
                &*head,
                Arc::new(Membership {
                    directory,
                    last_id: id,
                }),
            );
            if Arc::ptr_eq(&head, &current) {
                self.active.fetch_add(1, Ordering::Release);
                return Ok(Demand { gc, id, request });
            }
            head = current;
        }
    }

    pub(crate) fn capture(&self, metadata: &crate::metadata::ObjectMetadata) -> Option<Selection> {
        if self.active.load(Ordering::Acquire) == 0 || !metadata.can_transfer_retired_charge() {
            return None;
        }
        let entries = self.entries.load_full();
        let size = metadata.size();
        debug_assert!(isize::try_from(size).is_ok());
        Some(Selection {
            collector: metadata.collector.clone(),
            entries,
            donor: None,
            cursor: None,
            size,
            remaining: size,
            settled: false,
        })
    }
}

impl<'a> Demand<'a> {
    pub(crate) fn take(&self) -> Option<AllocationReservation<'a>> {
        let current = self.request.donor.load();
        let donor = current.as_ref()?;
        let state = donor.canceled.load(Ordering::Acquire);
        if state == ABANDONED {
            let spare = Arc::new(Donation::default());
            self.request
                .spare
                .compare_and_swap(std::ptr::null::<Donation>(), Some(spare));
            self.request.donor.compare_and_swap(&*current, None);
            return None;
        }
        if state != PAID {
            return None;
        }
        let previous = self
            .request
            .donor
            .compare_and_swap(&*current, Some(CLOSED.clone()));
        previous
            .as_ref()
            .filter(|old| Arc::ptr_eq(old, donor))
            .map(|_| AllocationReservation::transferred(self.gc, self.request.bytes))
    }

    // Races the donor's selection CAS. Only one can commit.
    pub(crate) fn finish_uncovered(&self) -> bool {
        self.request
            .donor
            .compare_and_swap(std::ptr::null::<Donation>(), Some(CLOSED.clone()))
            .is_none()
    }
}

impl Drop for Demand<'_> {
    fn drop(&mut self) {
        let registry = &self.gc.grant_registry;
        if let Some(donor) = self.request.donor.swap(Some(CLOSED.clone()))
            && !Arc::ptr_eq(&donor, &CLOSED)
        {
            let canceled = donor
                .canceled
                .try_update(Ordering::AcqRel, Ordering::Acquire, |old| {
                    (old < ABANDONED).then(|| old + self.request.bytes)
                });
            if canceled == Err(PAID) {
                self.gc.cancel_allocation(self.request.bytes);
            }
        }
        registry.entries.rcu(|head| {
            let (directory, _) = head.directory.remove(self.id);
            Arc::new(Membership {
                directory,
                last_id: head.last_id,
            })
        });
        registry.active.fetch_sub(1, Ordering::Release);
    }
}

#[derive(Debug)]
pub(crate) struct Selection {
    collector: Weak<GarbageCollector>,
    entries: Arc<Membership>,
    donor: Option<Arc<Donation>>,
    size: usize,
    settled: bool,
    cursor: Option<u64>,
    remaining: usize,
}

impl Selection {
    // The task stays at the front of the public sweep queue through all batches.
    pub(crate) fn advance(&mut self, budget: &mut usize) -> bool {
        let steps = budget.saturating_add(1).min(32);
        for i in 0..steps {
            let Some((id, weak)) = self.entries.directory.next_after(self.cursor) else {
                return true;
            };
            self.cursor = Some(id);
            if i != 0 {
                *budget -= 1;
            }
            if let Some(request) = weak.upgrade()
                && request.bytes <= self.remaining
                && request.donor.load().is_none()
            {
                if self.donor.is_none() {
                    self.donor = request.spare.swap(None);
                }
                if let Some(donor) = &self.donor
                    && request
                        .donor
                        .compare_and_swap(std::ptr::null::<Donation>(), Some(donor.clone()))
                        .is_none()
                {
                    self.remaining -= request.bytes;
                }
            }
            if self.remaining == 0 || id == self.entries.last_id {
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
mod tests {
    use super::*;
    use crate::Configuration;
    use std::future::Future;
    use std::task::{Context, Wake, Waker};

    // Stand in for the exclusively owned, already charged allocation. The
    // native integration fixture separately exercises the actual destructor.
    fn fixture() -> crate::Result<Arc<GarbageCollector>> {
        let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(32).build());
        gc.reserve_allocation(32)?;
        Ok(gc)
    }

    fn capture(gc: &Arc<GarbageCollector>) -> Selection {
        Selection {
            entries: gc.grant_registry.entries.load_full(),
            collector: Arc::downgrade(gc),
            donor: None,
            size: 32,
            remaining: 32,
            cursor: None,
            settled: false,
        }
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify ledger transitions alongside fallible allocations"
    )]
    fn cancellations_and_consumption_refund_each_share_once() -> crate::Result<()> {
        let gc = fixture()?;
        let early = gc.grant_registry.register(&gc, 8)?;
        let late = gc.grant_registry.register(&gc, 16)?;
        let spender = gc.grant_registry.register(&gc, 8)?;
        let mut selected = capture(&gc);
        assert!(selected.advance(&mut 31));
        selected.mark_private();
        assert!(spender.take().is_none());
        drop(early);
        assert_eq!(gc.managed_bytes(), 32);
        let paid = selected.pay();
        assert_eq!(paid, 24);
        gc.cancel_allocation(32 - paid);
        drop(late);
        assert_eq!(gc.managed_bytes(), 8);
        let credit = spender.take().ok_or(crate::Error::OutOfMemory)?;
        assert!(spender.take().is_none());
        drop(spender);
        assert_eq!(gc.managed_bytes(), 8);
        drop(credit);
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[derive(Default)]
    struct Runnable(AtomicBool);
    impl Wake for Runnable {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "check poll and accounting results on the same collector"
    )]
    fn covered_workerless_waiter_assists_until_the_donor_becomes_private() -> crate::Result<()> {
        let gc = fixture()?;
        let first = gc.grant_registry.register(&gc, 8)?;
        let second = gc.grant_registry.register(&gc, 8)?;
        let mut selected = capture(&gc);
        assert!(!selected.advance(&mut 0));
        let mut opportunity = gc.observe_space(8)?;
        let mut wait = std::pin::pin!(
            crate::recovery::CollectionWait::for_space(&gc, u64::MAX, &mut opportunity)
                .with_grant(&first.request, true)
        );
        let runnable = Arc::new(Runnable::default());
        let waker = Waker::from(runnable.clone());
        let mut cx = Context::from_waker(&waker);
        assert!(wait.as_mut().poll(&mut cx).is_pending());
        assert!(runnable.0.swap(false, Ordering::AcqRel));
        assert!(selected.advance(&mut 0));
        selected.mark_private();
        assert!(wait.as_mut().poll(&mut cx).is_pending());
        assert!(!runnable.0.load(Ordering::Acquire));
        drop(selected);
        gc.recovery.notify_progress(&gc);
        assert!(wait.as_mut().poll(&mut cx).is_ready());
        assert!(first.take().is_none());
        assert!(first.finish_uncovered());
        drop(second);
        assert_eq!(gc.managed_bytes(), 32);
        gc.cancel_allocation(32);
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }
}
