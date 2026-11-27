//! Native allocations that have already exhausted their collection attempts may
//! sleep for a single callback dispatch. Only the allocating task acquires funds.
use super::{GarbageCollector, Ordering, RefundNotifier, Result, WaiterId, index::Index};
use crate::{collector::CallbackTurn, retirement_grants::Request};
use arc_swap::{ArcSwap, ArcSwapOption};
use std::sync::Arc;
use std::task::{Poll, Waker};

#[derive(Debug)]
struct Wait {
    turn: CallbackTurn,
    request: Request,
    bytes: usize,
    waker: ArcSwapOption<Waker>,
}

impl Wait {
    fn ready(&self, gc: &GarbageCollector) -> bool {
        retry(
            &self.turn,
            &self.request,
            self.bytes,
            gc.allocation_headroom(),
        ) || gc.recovery.worker_failed()
    }
}

fn retry(turn: &CallbackTurn, request: &Request, bytes: usize, available: usize) -> bool {
    bytes <= available || request.settled_donor() || !turn.pending()
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
    pub(crate) async fn wait_for_turn(
        &self,
        gc: &GarbageCollector,
        turn: &CallbackTurn,
        request: &Request,
        bytes: usize,
    ) -> Result<()> {
        let mut registration: Option<Registration<'_>> = None;
        let mut interest = None;
        std::future::poll_fn(|cx| {
            // A task can resume in a different execution context. While asleep
            // it needs fitting credit visible to the independent notifier.
            gc.refresh_allocation_interest(&mut interest)?;
            if crate::cell::has_guards()
                || self.event.closed.load(Ordering::Acquire)
                || retry(turn, request, bytes, gc.allocation_headroom())
            {
                return Poll::Ready(Ok(()));
            }
            if gc.recovery.worker_failed() {
                return Poll::Ready(Err(crate::Error::SyncError(
                    "collector worker panicked".into(),
                )));
            }
            if let Some(registered) = &registration {
                let slot = &registered.wait.waker;
                let current = slot.load();
                let Some(old) = current.as_ref() else {
                    return Poll::Ready(Ok(()));
                };
                if !old.will_wake(cx.waker())
                    && slot
                        .compare_and_swap(&*current, Some(Arc::new(cx.waker().clone())))
                        .is_none()
                {
                    return Poll::Ready(Ok(()));
                }
            } else {
                let id = gc.recovery.new_id()?;
                let wait = Arc::new(Wait {
                    turn: turn.clone(),
                    request: request.clone(),
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
                // Registration precedes both the budget handshake and the
                // closed check. Late arrivals to a retired notifier cannot sleep.
                if retry(turn, request, bytes, gc.headroom_after_registration())
                    || self.event.closed.load(Ordering::Acquire)
                {
                    return Poll::Ready(Ok(()));
                }
            }
            Poll::Pending
        })
        .await
    }
}
