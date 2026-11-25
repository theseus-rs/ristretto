//! One cancellable registration per pending Java Reference Handler wait.
use super::{Context, GarbageCollector, Poll, Recovery, lock};

// The address of this non-zero-sized, mutably borrowed token identifies the wait.
// The async frame keeps the token alive across registration, polls and removal.
struct Registration<'a> {
    recovery: &'a Recovery,
    token: &'a mut u8,
}

impl Registration<'_> {
    fn key(&self) -> usize {
        std::ptr::from_ref(&*self.token).addr()
    }

    fn register(&self, context: &Context<'_>) {
        let key = self.key();
        // A custom RawWaker may run arbitrary code on clone and drop.
        let waker = context.waker().clone();
        let old = {
            let mut waiters = lock(&self.recovery.waiters);
            let queued = waiters.ready_references.remove(&key);
            let previous = waiters.references.insert(key, waker);
            self.recovery.update_counts(&waiters);
            (queued, previous)
        };
        drop(old);
    }
}

impl Drop for Registration<'_> {
    fn drop(&mut self) {
        let key = self.key();
        let old = {
            let mut waiters = lock(&self.recovery.waiters);
            let queued = waiters.ready_references.remove(&key);
            let previous = waiters.references.remove(&key);
            self.recovery.update_counts(&waiters);
            (queued, previous)
        };
        drop(old);
    }
}

impl Recovery {
    pub(crate) async fn wait_for_references(&self, gc: &GarbageCollector) {
        if gc.has_pending_references() {
            return;
        }
        let mut token = 0;
        let registration = Registration {
            recovery: self,
            token: &mut token,
        };
        std::future::poll_fn(|cx| {
            registration.register(cx);
            // Registration before this check closes the race with publication.
            if gc.has_pending_references() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }

    pub(crate) fn references_ready(&self) {
        let mut waiters = lock(&self.waiters);
        if waiters.references.is_empty() {
            return;
        }
        let mut waiting = std::mem::take(&mut waiters.references);
        waiters.ready_references.append(&mut waiting);
        self.update_counts(&waiters);
    }
}
