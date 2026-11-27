//! Native allocations that have already exhausted their collection attempts may
//! sleep for a single callback dispatch. Only the allocating task acquires funds.
use super::{GarbageCollector, Ordering, RefundNotifier, Result, WaiterId, index::Index};
use crate::{
    collector::{AllocationReservation, CallbackTurn},
    retirement_grants::{Demand, Request},
};
use arc_swap::{ArcSwap, ArcSwapOption};
use std::sync::Arc;
use std::task::{Poll, Waker};

#[derive(Debug)]
struct Wait {
    turn: Arc<CallbackTurn>,
    request: Request,
    bytes: usize,
    waker: ArcSwapOption<Waker>,
}

impl Wait {
    fn ready(&self, gc: &GarbageCollector) -> bool {
        !self.turn.pending()
            || self.request.settled_donor()
            || self.bytes <= gc.allocation_headroom()
            || gc.recovery.worker_failed()
    }

    fn refresh(&self, incoming: &Waker) -> bool {
        let current = self.waker.load();
        current.as_ref().is_some_and(|old| {
            old.will_wake(incoming)
                || self
                    .waker
                    .compare_and_swap(&*current, Some(Arc::new(incoming.clone())))
                    .is_some()
        })
    }
}

#[derive(Debug, Default)]
pub(super) struct Registrations(ArcSwap<Index<Arc<Wait>>>);

#[derive(Default)]
pub(super) struct Scan {
    last: Option<WaiterId>,
    end: WaiterId,
}

impl Registrations {
    fn remove(&self, id: WaiterId) {
        if self.0.load().get(id).is_none() {
            return;
        }
        self.0.rcu(|current| {
            let mut next = (**current).clone();
            next.remove(id);
            next
        });
    }

    pub(super) fn dispatch(&self, gc: &GarbageCollector, scan: &mut Scan, closed: bool) -> bool {
        let snapshot = self.0.load_full();
        if scan.last.is_none() {
            // Finish this finite pass even under sustained enrollment.
            scan.end = gc.recovery.next_id.load(Ordering::Acquire);
        }
        for _ in 0..32 {
            let Some((id, wait)) = snapshot.next(scan.last).filter(|(id, _)| *id <= scan.end)
            else {
                scan.last = None;
                break;
            };
            scan.last = Some(id);
            if closed || wait.ready(gc) {
                let wait = wait.clone();
                // Secure delivery before removing the registration. A failed
                // thread spawn leaves the exact waker available for a later pass.
                if std::thread::Builder::new()
                    .name("gc-allocation-wake".into())
                    .spawn(move || {
                        if let Some(waker) = wait.waker.swap(None) {
                            Arc::unwrap_or_clone(waker).wake();
                        }
                    })
                    .is_ok()
                {
                    self.remove(id);
                }
            }
        }
        !self.0.load().is_empty()
    }
}

struct Registration<'a> {
    list: &'a Registrations,
    id: WaiterId,
    wait: Arc<Wait>,
}

impl Drop for Registration<'_> {
    fn drop(&mut self) {
        self.wait.waker.swap(None);
        self.list.remove(self.id);
    }
}

impl RefundNotifier {
    pub(crate) async fn wait_for_turn<'a>(
        &self,
        gc: &'a GarbageCollector,
        turn: Arc<CallbackTurn>,
        demand: &Demand<'a>,
        bytes: usize,
    ) -> Result<Option<AllocationReservation<'a>>> {
        let mut registration: Option<Registration<'_>> = None;
        let mut interest = None;
        let reserve = || demand.take().or_else(|| gc.reserve_bytes(bytes).ok());
        std::future::poll_fn(|cx| {
            // Context-local credit must remain visible if the executor migrates.
            gc.refresh_allocation_interest(&mut interest)?;
            if let Some(reservation) = reserve() {
                return Poll::Ready(Ok(Some(reservation)));
            }
            if crate::cell::has_guards() {
                return Poll::Ready(Err(crate::Error::OutOfMemory));
            }
            if self.event.closed.load(Ordering::Acquire) || !turn.pending() {
                return Poll::Ready(Ok(None));
            }
            if gc.recovery.worker_failed() {
                return Poll::Ready(Err(crate::Error::SyncError(
                    "collector worker panicked".into(),
                )));
            }
            if registration
                .as_ref()
                .is_some_and(|arm| arm.wait.refresh(cx.waker()))
            {
                return Poll::Pending;
            }
            // A stolen offer can exhaust the old callback. Enroll a fresh ID so
            // delayed cleanup cannot remove the replacement, then try funds once.
            let id = gc.recovery.new_id()?;
            let wait = Arc::new(Wait {
                turn: turn.clone(),
                request: demand.request.clone(),
                bytes,
                waker: ArcSwapOption::from(Some(Arc::new(cx.waker().clone()))),
            });
            let list = &self.event.turns;
            list.0.rcu(|current| {
                let mut next = (**current).clone();
                next.insert(id, wait.clone());
                next
            });
            registration = Some(Registration { list, id, wait });
            self.signal();
            if (bytes <= gc.headroom_after_registration() || demand.request.settled_donor())
                && let Some(reservation) = reserve()
            {
                return Poll::Ready(Ok(Some(reservation)));
            }
            if self.event.closed.load(Ordering::Acquire) || !turn.pending() {
                return Poll::Ready(Ok(None));
            }
            Poll::Pending
        })
        .await
    }
}
