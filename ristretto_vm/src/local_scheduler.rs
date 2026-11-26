//! Keeps cooperative Java tasks alive across WASI host calls.
use std::future::Future;

#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalScheduler {
    #[cfg(target_os = "wasi")]
    id: u64,
}

#[cfg(target_os = "wasi")]
mod wasi {
    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use tokio::task::LocalSet;

    std::thread_local! {
        pub(super) static SETS: RefCell<BTreeMap<u64, Rc<LocalSet>>> = const { RefCell::new(BTreeMap::new()) };
        pub(super) static ACTIVE: Cell<Option<u64>> = const { Cell::new(None) };
    }
    pub(super) struct Active(pub(super) Option<u64>);
    impl Drop for Active {
        fn drop(&mut self) {
            ACTIVE.with(|active| active.set(self.0));
        }
    }
}

impl LocalScheduler {
    pub(crate) fn new() -> Self {
        #[cfg(target_os = "wasi")]
        {
            static NEXT: portable_atomic::AtomicU64 = portable_atomic::AtomicU64::new(1);
            Self {
                id: NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            }
        }
        #[cfg(not(target_os = "wasi"))]
        {
            Self {}
        }
    }

    #[cfg(not(target_os = "wasi"))]
    pub(crate) async fn run<F: Future>(&self, future: F) -> F::Output {
        future.await
    }

    #[cfg(target_os = "wasi")]
    pub(crate) fn run<F: Future>(&self, future: F) -> impl Future<Output = F::Output> + use<F> {
        let id = self.id;
        // Box before constructing the wrapper future: Java bootstrap futures are large,
        // and WASI has a smaller stack than native runtimes.
        let future = Box::pin(future);
        async move {
            if wasi::ACTIVE.with(|active| active.get() == Some(id)) {
                return future.await;
            }
            let local = wasi::SETS.with(|sets| {
                sets.borrow_mut()
                    .entry(id)
                    .or_insert_with(|| std::rc::Rc::new(tokio::task::LocalSet::new()))
                    .clone()
            });
            let mut run = std::pin::pin!(local.run_until(async {
                // Even an immediately ready host invocation must give queued reference
                // handlers and finalizers an opportunity to run.
                tokio::task::yield_now().await;
                future.await
            }));
            std::future::poll_fn(|context| {
                let _active = wasi::Active(wasi::ACTIVE.with(|active| active.replace(Some(id))));
                run.as_mut().poll(context)
            })
            .await
        }
    }

    #[cfg_attr(
        not(target_os = "wasi"),
        expect(
            clippy::unused_self,
            reason = "uniform scheduler lifecycle across platforms"
        )
    )]
    pub(crate) fn close(self) {
        #[cfg(target_os = "wasi")]
        {
            let removed = wasi::SETS.with(|sets| sets.borrow_mut().remove(&self.id));
            // Dropping the LocalSet cancels its remaining futures and releases GC roots.
            drop(removed);
        }
    }
}

#[cfg(all(test, target_os = "wasi"))]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[tokio::test]
    async fn daemon_futures_survive_host_calls_and_drop_at_shutdown() {
        struct Cleanup(Rc<Cell<bool>>);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        let scheduler = LocalScheduler::new();
        let completed = Rc::new(Cell::new(false));
        let dropped = Rc::new(Cell::new(false));
        let (send, receive) = tokio::sync::oneshot::channel();
        let task_completed = completed.clone();
        let cleanup = Cleanup(dropped.clone());
        scheduler
            .run(async move {
                tokio::task::spawn_local(async move {
                    let _cleanup = cleanup;
                    let _ = receive.await;
                    task_completed.set(true);
                    std::future::pending::<()>().await;
                });
            })
            .await;
        assert!(!dropped.get());
        send.send(()).expect("task survives host return");
        scheduler.run(async {}).await;
        assert!(completed.get());
        assert!(!dropped.get());
        scheduler.close();
        assert!(dropped.get());
    }
}
