//! Same-heap static roots and distinct logical initializer owners; no timing tests.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "finite ownership assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Invocation, Limits, Program};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

type Result<T = ()> = std::result::Result<T, Error>;
const CLASS: &str = "CanonicalSharedStatics";
const BYTES: usize = 1024 * 1024;
const CLEANUP: usize = 1_000_000;
fn program() -> Result<Program> {
    let classes: [&[u8]; 8] = [
        include_bytes!("fixtures/canonical/CanonicalSharedStatics.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Holder.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Slow.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Bad.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Nested.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$CycleA.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$CycleB.class"),
    ];
    Program::from_classfiles(
        classes
            .into_iter()
            .map(ClassFile::from_bytes)
            .collect::<std::result::Result<Vec<_>, _>>()?,
    )
}
fn configuration() -> Configuration {
    Configuration {
        objects: 32,
        roots: 64,
        mutators: 3,
        helpers: 3,
        edges_per_object: 1,
    }
}

// These tests intentionally poll outside any Tokio runtime. yield_now therefore
// wakes the supplied waker immediately; a class-local Notify wait does not. This
// is a bounded deterministic scheduler, not a sleep/time or throughput probe.
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
fn park_first<F: Future>(mut future: Pin<&mut F>) {
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
}
fn park_at_class_wait<F: Future>(mut future: Pin<&mut F>) -> Result<Arc<WakeCount>> {
    let counter = Arc::new(WakeCount::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    for _ in 0..50_000 {
        let previous = counter.0.load(Ordering::Relaxed);
        assert!(future.as_mut().poll(&mut context).is_pending());
        if counter.0.load(Ordering::Relaxed) == previous {
            return Ok(counter);
        }
    }
    Err(Error::Limit("test class wait"))
}
fn aborted(result: &Result<Invocation>) {
    assert!(matches!(
        result,
        Err(Error::Boundary(
            "class initialization was aborted by embedding"
        ))
    ));
}

#[test]
fn same_heap_entries_and_two_context_writes_match_hotspot() -> Result {
    let program = program()?;
    complete(
        program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
            let mut first = heap.attach(Limits::default())?;
            let mut second = heap.attach(Limits::default())?;
            let mut rows = 0;
            let mut total_storage = 0;
            for (index, row) in include_str!("fixtures/canonical/shared-statics-oracle.tsv")
                .lines()
                .enumerate()
            {
                let fields: Vec<_> = row.split('\t').collect();
                let [method, argument, expected] = fields.as_slice() else {
                    return Err(Error::Boundary("oracle columns"));
                };
                let thread = if index.is_multiple_of(2) {
                    &mut first
                } else {
                    &mut second
                };
                let result = thread
                    .invoke(
                        CLASS,
                        method,
                        argument
                            .parse()
                            .map_err(|_| Error::Boundary("oracle argument"))?,
                    )
                    .await;
                if let Ok(expected) = expected.parse::<i32>() {
                    let result = result?;
                    assert_eq!(result.result, expected);
                    total_storage += result.static_storage_allocations;
                    if *method == "read" && index != 0 {
                        assert_eq!(result.initialized_classes, 0);
                    }
                } else {
                    let Err(Error::Initialization { kind, .. }) = result else {
                        return Err(Error::Boundary("oracle error category"));
                    };
                    assert_eq!(kind, *expected);
                }
                rows += 1;
            }
            assert_eq!(rows, 14);
            assert_eq!(total_storage, 5); // Holder, Slow, Nested and the recursive pair.
            assert!(heap.managed_bytes() > 0);
            Ok(())
        }),
    )?;
    Ok(())
}

#[test]
fn parked_initializer_waiter_roots_and_unrelated_classes_make_progress() -> Result {
    let program = program()?;
    complete(
        program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut waiter = heap.attach(Limits::default())?;
            let mut other = heap.attach(Limits::default())?;
            let mut initializing = Box::pin(owner.invoke(CLASS, "slow", 0));
            park_first(initializing.as_mut());
            let mut waiting = Box::pin(waiter.invoke(CLASS, "waitWithRoot", 31));
            let notified = park_at_class_wait(waiting.as_mut())?;
            let before_wake = notified.0.load(Ordering::Relaxed);
            let before_cycles = other.primary_cycles()?.upper;
            for value in 0..32 {
                assert_eq!(
                    other.invoke(CLASS, "write", value).await?.result,
                    value * 2 + 7
                );
            }
            assert!(other.primary_cycles()?.lower > before_cycles);
            assert_eq!(notified.0.load(Ordering::Relaxed), before_wake);
            let initialized = initializing.await?;
            assert_eq!(initialized.result, 32657);
            assert_eq!(initialized.class_initializer_calls, 1);
            assert!(notified.0.load(Ordering::Relaxed) > before_wake);
            let outcome = waiting.await?;
            assert_eq!(outcome.result, 32688);
            assert_eq!(outcome.class_initializer_calls, 0);
            assert_eq!(outcome.static_storage_allocations, 0);
            assert_eq!(outcome.class_initialization_waits, 1);
            Ok(())
        }),
    )?;
    Ok(())
}

#[test]
fn owner_cancellation_or_failure_wakes_waiters_without_replaying_initializer() -> Result {
    let program = program()?;
    for cancel in [false, true] {
        complete(
            program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
                let mut owner = heap.attach(Limits::default())?;
                let mut waiter = heap.attach(Limits::default())?;
                let method = if cancel { "slow" } else { "bad" };
                let mut initializing = Box::pin(owner.invoke(CLASS, method, 0));
                park_first(initializing.as_mut());
                let mut waiting = Box::pin(waiter.invoke(CLASS, method, 0));
                let notified = park_at_class_wait(waiting.as_mut())?;
                let before = notified.0.load(Ordering::Relaxed);
                if cancel {
                    drop(initializing);
                } else {
                    assert!(matches!(
                        initializing.await,
                        Err(Error::Initialization {
                            kind: "ExceptionInInitializerError",
                            ..
                        })
                    ));
                }
                assert!(notified.0.load(Ordering::Relaxed) > before);
                if cancel {
                    aborted(&waiting.await);
                    aborted(&owner.invoke(CLASS, method, 0).await);
                } else {
                    assert!(matches!(
                        waiting.await,
                        Err(Error::Initialization {
                            kind: "NoClassDefFoundError",
                            ..
                        })
                    ));
                    assert!(matches!(
                        owner.invoke(CLASS, method, 0).await,
                        Err(Error::Initialization {
                            kind: "NoClassDefFoundError",
                            ..
                        })
                    ));
                }
                assert_eq!(owner.invoke(CLASS, "write", 4).await?.result, 15);
                assert_eq!(waiter.invoke(CLASS, "read", 0).await?.result, 15);
                Ok(())
            }),
        )?;
    }
    Ok(())
}

#[test]
fn cancelled_waiter_does_not_abort_owner_and_cross_owner_cycles_cancel_cleanly() -> Result {
    let program = program()?;
    complete(
        program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
            let mut first = heap.attach(Limits::default())?;
            let mut second = heap.attach(Limits::default())?;
            let mut initializing = Box::pin(first.invoke(CLASS, "slow", 0));
            park_first(initializing.as_mut());
            let mut waiting = Box::pin(second.invoke(CLASS, "slow", 0));
            let _wake = park_at_class_wait(waiting.as_mut())?;
            drop(waiting);
            assert_eq!(initializing.await?.result, 32657);
            assert_eq!(second.invoke(CLASS, "slow", 1).await?.result, 32658);
            let mut a = Box::pin(first.invoke(CLASS, "cycleA", 0));
            park_first(a.as_mut());
            let mut b = Box::pin(second.invoke(CLASS, "cycleB", 0));
            park_first(b.as_mut());
            let _a_wake = park_at_class_wait(a.as_mut())?;
            let b_wake = park_at_class_wait(b.as_mut())?;
            let before = b_wake.0.load(Ordering::Relaxed);
            drop(a);
            assert!(b_wake.0.load(Ordering::Relaxed) > before);
            aborted(&b.await);
            aborted(&first.invoke(CLASS, "cycleA", 0).await);
            aborted(&second.invoke(CLASS, "cycleB", 0).await);
            Ok(())
        }),
    )?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn native_threads_wait_and_wake_without_blocking_other_class_or_gc_progress() -> Result {
    use std::sync::mpsc::channel;
    use std::time::Duration;
    let program = program()?;
    for cancel in [false, true] {
        complete(
            program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
                std::thread::scope(|scope| {
                    let (started, owner_started) = channel();
                    let (parked, waiter_parked) = channel();
                    let (resume, owner_resume) = channel();
                    let owner = scope.spawn(move || -> Result {
                        let mut thread = heap.attach(Limits::default())?;
                        let mut initializing = Box::pin(thread.invoke(CLASS, "slow", 0));
                        park_first(initializing.as_mut());
                        started
                            .send(())
                            .map_err(|_| Error::Boundary("waiter ended"))?;
                        owner_resume
                            .recv_timeout(Duration::from_secs(20))
                            .map_err(|_| Error::Boundary("independent progress did not finish"))?;
                        if cancel {
                            drop(initializing);
                        } else {
                            let result = complete(initializing)?;
                            assert_eq!(result.result, 32657);
                            assert_eq!(result.class_initializer_calls, 1);
                        }
                        Ok(())
                    });
                    let waiter = scope.spawn(move || -> Result {
                        owner_started
                            .recv_timeout(Duration::from_secs(20))
                            .map_err(|_| Error::Boundary("owner did not start"))?;
                        let mut thread = heap.attach(Limits::default())?;
                        let mut waiting = Box::pin(thread.invoke(CLASS, "waitWithRoot", 19));
                        let _waker = park_at_class_wait(waiting.as_mut())?;
                        parked
                            .send(())
                            .map_err(|_| Error::Boundary("independent task ended"))?;
                        let executor = tokio::runtime::Builder::new_current_thread()
                            .enable_time()
                            .build()
                            .map_err(|_| Error::Boundary("native executor"))?;
                        let result = executor.block_on(async {
                            tokio::time::timeout(Duration::from_secs(20), waiting)
                                .await
                                .map_err(|_| Error::Boundary("class waiter not woken"))
                        })?;
                        if cancel {
                            aborted(&result);
                        } else {
                            let result = result?;
                            assert_eq!(result.result, 32676);
                            assert_eq!(result.class_initializer_calls, 0);
                            assert_eq!(result.class_initialization_waits, 1);
                        }
                        Ok(())
                    });
                    let independent = scope.spawn(move || -> Result {
                        waiter_parked
                            .recv_timeout(Duration::from_secs(20))
                            .map_err(|_| Error::Boundary("waiter did not park"))?;
                        let mut thread = heap.attach(Limits::default())?;
                        let before = thread.primary_cycles()?.upper;
                        for value in 0..32 {
                            assert_eq!(
                                complete(thread.invoke(CLASS, "write", value))?.result,
                                value * 2 + 7
                            );
                        }
                        assert!(thread.primary_cycles()?.lower > before);
                        resume
                            .send(())
                            .map_err(|_| Error::Boundary("owner ended"))?;
                        Ok(())
                    });
                    owner
                        .join()
                        .map_err(|_| Error::Boundary("owner panicked"))??;
                    waiter
                        .join()
                        .map_err(|_| Error::Boundary("waiter panicked"))??;
                    independent
                        .join()
                        .map_err(|_| Error::Boundary("independent task panicked"))??;
                    Ok(())
                })
            }),
        )?;
    }
    Ok(())
}
