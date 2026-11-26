//! Interpreter counterpart for the native putstatic wait/return fixture, including WASI.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "finite ownership assertions")]
#[path = "support/canonical_static_handoff.rs"]
mod handoff;
use ristretto_vm::canonical::{Error, Limits};
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};
type Result<T = ()> = std::result::Result<T, Error>;
#[derive(Default)]
struct WakeCount(AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
fn complete<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..1_000_000 {
        if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
            return result;
        }
    }
    Err(Error::Limit("test executor poll"))
}
#[test]
fn interpreter_putstatic_wait_and_cleared_return_match_hotspot() -> Result {
    let program = handoff::program()?;
    complete(program.with_heap(
        handoff::configuration(),
        1024 * 1024,
        1_000_000,
        async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut waiter = heap.attach(Limits::default())?;
            let mut other = heap.attach(Limits::default())?;
            let mut initializing = Box::pin(owner.invoke(handoff::CLASS, "prepare", 0));
            assert!(
                initializing
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            let mut waiting = Box::pin(waiter.invoke(handoff::CLASS, "store", 31));
            let count = Arc::new(WakeCount::default());
            let waker = Waker::from(count.clone());
            let mut context = Context::from_waker(&waker);
            let mut found = false;
            for _ in 0..50_000 {
                let before = count.0.load(Ordering::Relaxed);
                assert!(waiting.as_mut().poll(&mut context).is_pending());
                if count.0.load(Ordering::Relaxed) == before {
                    found = true;
                    break;
                }
            }
            assert!(found);
            let before = count.0.load(Ordering::Relaxed);
            let cycles = other.primary_cycles()?.upper;
            for _ in 0..8 {
                assert_eq!(other.invoke(handoff::CLASS, "churn", 17).await?.result, 136);
            }
            assert!(other.primary_cycles()?.lower > cycles);
            assert_eq!(count.0.load(Ordering::Relaxed), before);
            assert_eq!(initializing.await?.result, 32640);
            assert!(count.0.load(Ordering::Relaxed) > before);
            let result = waiting.await?;
            assert_eq!(
                (
                    result.result,
                    result.class_initializer_calls,
                    result.class_initialization_waits
                ),
                (32671, 0, 1)
            );
            assert_eq!(waiter.invoke(handoff::CLASS, "empty", 0).await?.result, 1);
            let mut rows = 0;
            for row in handoff::ORACLE.lines() {
                let fields: Vec<_> = row.split('\t').collect();
                let [method, argument, expected] = fields.as_slice() else {
                    return Err(Error::Boundary("handoff oracle columns"));
                };
                assert_eq!(
                    waiter
                        .invoke(
                            handoff::CLASS,
                            method,
                            argument
                                .parse()
                                .map_err(|_| Error::Boundary("handoff oracle argument"))?
                        )
                        .await?
                        .result,
                    expected
                        .parse::<i32>()
                        .map_err(|_| Error::Boundary("handoff oracle value"))?
                );
                rows += 1;
            }
            assert_eq!(rows, 5);
            Ok(())
        },
    ))?;
    Ok(())
}
