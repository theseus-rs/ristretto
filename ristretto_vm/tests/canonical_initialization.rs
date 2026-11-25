//! Class initialization through real javac methods and shared owning contexts.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};
use std::future::Future;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Context, Wake, Waker};

const CLASS: &str = "CanonicalClassInitialization";
const ENTRY: &str = "CanonicalEntryInitialization";
fn program() -> Result<Program, Error> {
    let bytes: &[&[u8]] = &[
        include_bytes!("fixtures/canonical/CanonicalClassInitialization.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$Slow.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$Bad.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$Nested.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$NestedBad.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$CycleA.class"),
        include_bytes!("fixtures/canonical/CanonicalClassInitialization$CycleB.class"),
        include_bytes!("fixtures/canonical/CanonicalEntryInitialization.class"),
    ];
    Program::from_classfiles(
        bytes
            .iter()
            .map(|bytes| ClassFile::from_bytes(bytes))
            .collect::<Result<Vec<_>, _>>()?,
    )
}
fn config() -> Configuration {
    Configuration {
        objects: 12,
        roots: 32,
        mutators: 3,
        helpers: 3,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn initialization_recursion_and_java_errors_match_hotspot() -> Result<(), Error> {
    program()?.with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
        let mut thread = heap.attach(Limits::default())?;
        for row in include_str!("fixtures/canonical/initialization-oracle.txt").lines() {
            let columns: Vec<_> = row.split_whitespace().collect();
            let [method, argument, expected] = columns.as_slice() else { return Err(Error::Boundary("oracle row")); };
            let argument = argument.parse().map_err(|_| Error::Boundary("oracle argument"))?;
            let outcome = if *method == "entry" { thread.invoke(ENTRY, "run", argument).await }
                else { thread.invoke(CLASS, method, argument).await };
            if let Ok(expected) = expected.parse::<i32>() {
                let outcome = outcome?;
                assert_eq!(outcome.result, expected, "{row}");
                if *method == "slow" { assert_eq!(outcome.class_initializer_calls, usize::from(argument == 4)); }
                if *method == "cycleA" { assert_eq!(outcome.initialized_classes, 2); }
            } else {
                let Err(Error::Initialization { kind, cause }) = outcome else { return Err(Error::Boundary("missing Java initialization fault")); };
                assert_eq!(kind, *expected, "{row}");
                if kind == "ExceptionInInitializerError" {
                    assert!(matches!(cause.as_deref(), Some(Error::Invalid(name)) if name == "NullPointerException"));
                } else { assert!(cause.is_none()); }
            }
        }
        Ok(())
    }).await?;
    Ok(())
}

#[tokio::test]
async fn entry_initialization_is_counted_once_and_shares_the_instruction_limit() -> Result<(), Error>
{
    let program = program()?;
    assert!(matches!(
        program.run_scalar(ENTRY, "identity", 4),
        Err(Error::Unsupported(_))
    ));
    let single = Configuration {
        mutators: 1,
        helpers: 1,
        ..config()
    };
    let report = program
        .run(ENTRY, "identity", 4, single, Limits::default())
        .await?;
    assert_eq!(
        (report.result, report.calls, report.class_initializer_calls),
        (4, 3, 1)
    );
    assert_eq!(
        (
            report.entry_initialization.calls,
            report.entry_initialization.allocations
        ),
        (2, 1)
    );
    assert_eq!(
        report.instructions,
        report.entry_initialization.instructions + 2
    );
    assert!(matches!(
        program
            .run(
                ENTRY,
                "identity",
                4,
                single,
                Limits {
                    instructions: report.entry_initialization.instructions,
                    ..Limits::default()
                }
            )
            .await,
        Err(Error::Limit("instructions"))
    ));
    program
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            assert_eq!(
                thread
                    .invoke(ENTRY, "run", 4)
                    .await?
                    .entry_initialization
                    .calls,
                2
            );
            let second = thread.invoke(ENTRY, "run", -13).await?;
            assert_eq!(second.result, 6);
            assert_eq!(second.entry_initialization.calls, 0);
            assert_eq!(second.class_initializer_calls, 0);
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn parked_initializer_keeps_a_waiting_static_value_alive_during_other_gc_work()
-> Result<(), Error> {
    program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut waiter = heap.attach(Limits::default())?;
            let mut other = heap.attach(Limits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let before = heap.managed_bytes();
            assert!(before > 0);
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(heap.managed_bytes() > before);
            let start = other.primary_cycles()?;
            let mut released = 0;
            for _ in 0..16 {
                let churn = other.invoke(CLASS, "churn", 8).await?;
                assert_eq!(churn.result, 28);
                released += churn.released_objects;
            }
            assert!(released >= 128 - config().objects);
            assert!(other.primary_cycles()?.lower > start.upper);
            let initialized = initialization.await?;
            assert_eq!(
                (initialized.result, initialized.class_initializer_calls),
                (2_096_149, 1)
            );
            let stored = waiting.await?;
            assert_eq!((stored.result, stored.class_initializer_calls), (31, 0));
            assert!(stored.class_initialization_waits > 0);
            assert_eq!(other.invoke(CLASS, "returned", 0).await?.result, 31);
            Ok(())
        })
        .await?;
    Ok(())
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

#[tokio::test]
async fn cancelled_initializer_wakes_waiter_and_only_poisons_its_own_class() -> Result<(), Error> {
    program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut waiter = heap.attach(Limits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let wake = Arc::new(WakeCount::default());
            let waker = Waker::from(wake.clone());
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(&waker))
                        .is_pending()
                );
            }
            let before = wake.0.load(Ordering::Relaxed);
            drop(initialization);
            assert!(wake.0.load(Ordering::Relaxed) > before);
            assert!(matches!(
                waiting.await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            assert!(matches!(
                owner.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(_))
            ));
            assert_eq!(waiter.invoke(CLASS, "cycleA", 0).await?.result, 3);
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn dropping_a_waiter_does_not_poison_the_initializer_owner() -> Result<(), Error> {
    program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut waiter = heap.attach(Limits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            drop(waiting);
            assert_eq!(initialization.await?.result, 2_096_149);
            let result = waiter.invoke(CLASS, "storeSlow", 42).await?;
            assert_eq!((result.result, result.class_initializer_calls), (42, 0));
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn separate_native_executors_wait_for_one_logical_initializer_owner() -> Result<(), Error> {
    use std::sync::mpsc::channel;
    use std::time::Duration;
    program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            std::thread::scope(|scope| {
                let (parked, observe) = channel();
                let (waiting, resume) = channel();
                let first = scope.spawn(move || {
                    let mut thread = heap.attach(Limits::default())?;
                    let mut invocation = Box::pin(thread.invoke(CLASS, "slow", 4));
                    for _ in 0..120 {
                        assert!(
                            invocation
                                .as_mut()
                                .poll(&mut Context::from_waker(Waker::noop()))
                                .is_pending()
                        );
                    }
                    parked.send(()).map_err(|e| Error::Invalid(e.to_string()))?;
                    resume
                        .recv_timeout(Duration::from_secs(30))
                        .map_err(|e| Error::Invalid(e.to_string()))?;
                    tokio::runtime::Builder::new_current_thread()
                        .build()
                        .map_err(|e| Error::Invalid(e.to_string()))?
                        .block_on(invocation)
                });
                let second = scope.spawn(move || {
                    observe
                        .recv_timeout(Duration::from_secs(30))
                        .map_err(|e| Error::Invalid(e.to_string()))?;
                    let mut thread = heap.attach(Limits::default())?;
                    let mut invocation = Box::pin(thread.invoke(CLASS, "storeSlow", 31));
                    for _ in 0..300 {
                        assert!(
                            invocation
                                .as_mut()
                                .poll(&mut Context::from_waker(Waker::noop()))
                                .is_pending()
                        );
                    }
                    waiting
                        .send(())
                        .map_err(|e| Error::Invalid(e.to_string()))?;
                    tokio::runtime::Builder::new_current_thread()
                        .build()
                        .map_err(|e| Error::Invalid(e.to_string()))?
                        .block_on(invocation)
                });
                let first = first
                    .join()
                    .map_err(|_| Error::Boundary("owner panicked"))??;
                let second = second
                    .join()
                    .map_err(|_| Error::Boundary("waiter panicked"))??;
                assert_eq!(
                    (first.result, first.class_initializer_calls),
                    (2_096_149, 1)
                );
                assert_eq!((second.result, second.class_initializer_calls), (31, 0));
                assert!(second.class_initialization_waits > 0);
                Ok(())
            })
        })
        .await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_initialization_recursion_and_java_errors_match_hotspot() -> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program.with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
        let mut thread = compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
        for row in include_str!("fixtures/canonical/initialization-oracle.txt").lines() {
            let columns: Vec<_> = row.split_whitespace().collect();
            let [method, argument, expected] = columns.as_slice() else { return Err(Error::Boundary("oracle row")); };
            let argument = argument.parse().map_err(|_| Error::Boundary("oracle argument"))?;
            let outcome = if *method == "entry" { thread.invoke(ENTRY, "run", argument).await }
                else { thread.invoke(CLASS, method, argument).await };
            if let Ok(expected) = expected.parse::<i32>() {
                let outcome = outcome?;
                assert_eq!(outcome.result, expected, "{row}");
                if *method == "slow" { assert_eq!(outcome.class_initializer_calls, usize::from(argument == 4)); }
                if *method == "cycleA" { assert_eq!(outcome.initialized_classes, 2); }
            } else {
                let Err(Error::Initialization { kind, cause }) = outcome else { return Err(Error::Boundary("missing Java initialization fault")); };
                assert_eq!(kind, *expected, "{row}");
                if kind == "ExceptionInInitializerError" {
                    assert!(matches!(cause.as_deref(), Some(Error::Invalid(name)) if name == "NullPointerException"));
                } else { assert!(cause.is_none()); }
            }
        }
        Ok(())
    }).await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_parked_initializer_keeps_a_waiting_static_value_alive_during_other_gc_work()
-> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut waiter =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut other =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let before = heap.managed_bytes();
            assert!(before > 0);
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(heap.managed_bytes() > before);
            let start = other.primary_cycles()?;
            let mut released = 0;
            for _ in 0..16 {
                let churn = other.invoke(CLASS, "churn", 8).await?;
                assert_eq!(churn.result, 28);
                released += churn.released_objects;
            }
            assert!(released >= 128 - config().objects);
            assert!(other.primary_cycles()?.lower > start.upper);
            let initialized = initialization.await?;
            assert_eq!(
                (initialized.result, initialized.class_initializer_calls),
                (2_096_149, 1)
            );
            let stored = waiting.await?;
            assert_eq!((stored.result, stored.class_initializer_calls), (31, 0));
            assert!(stored.class_initialization_waits > 0);
            assert_eq!(other.invoke(CLASS, "returned", 0).await?.result, 31);
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_cancelled_initializer_wakes_waiter_and_only_poisons_its_own_class()
-> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut waiter =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let wake = Arc::new(WakeCount::default());
            let waker = Waker::from(wake.clone());
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(&waker))
                        .is_pending()
                );
            }
            let before = wake.0.load(Ordering::Relaxed);
            drop(initialization);
            assert!(wake.0.load(Ordering::Relaxed) > before);
            assert!(matches!(
                waiting.await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            assert!(matches!(
                owner.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(_))
            ));
            assert_eq!(waiter.invoke(CLASS, "cycleA", 0).await?.result, 3);
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_dropping_a_waiter_does_not_poison_the_initializer_owner() -> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut owner =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut waiter =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut initialization = Box::pin(owner.invoke(CLASS, "slow", 4));
            for _ in 0..120 {
                assert!(
                    initialization
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            let mut waiting = Box::pin(waiter.invoke(CLASS, "storeSlow", 31));
            for _ in 0..300 {
                assert!(
                    waiting
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            drop(waiting);
            assert_eq!(initialization.await?.result, 2_096_149);
            let result = waiter.invoke(CLASS, "storeSlow", 42).await?;
            assert_eq!((result.result, result.class_initializer_calls), (42, 0));
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_entry_initialization_counts_actual_entries_and_shares_the_slice_limit()
-> Result<(), Error> {
    use ristretto_vm::canonical::CompiledLimits;
    let program = program()?;
    let compiled = program.compile()?;
    let single = Configuration {
        mutators: 1,
        helpers: 1,
        ..config()
    };
    let report = compiled
        .run(ENTRY, "identity", 4, single, CompiledLimits::default())
        .await?;
    assert_eq!(
        (
            report.result,
            report.calls,
            report.compiled_entries,
            report.class_initializer_calls
        ),
        (4, 3, 3, 1)
    );
    assert_eq!(
        (
            report.entry_initialization.calls,
            report.entry_initialization.allocations
        ),
        (2, 1)
    );
    assert!(!report.instruction_count_available);
    assert!(matches!(
        compiled
            .run(
                ENTRY,
                "identity",
                4,
                single,
                CompiledLimits {
                    native_slices: 1,
                    ..CompiledLimits::default()
                }
            )
            .await,
        Err(Error::Limit("native slice"))
    ));
    assert!(matches!(
        compiled
            .run(
                ENTRY,
                "identity",
                4,
                single,
                CompiledLimits {
                    frames: 1,
                    ..CompiledLimits::default()
                }
            )
            .await,
        Err(Error::Limit("frame"))
    ));
    Ok(())
}
