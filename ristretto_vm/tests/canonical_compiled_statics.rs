//! Native initializer execution and shared registry handoff; finite correctness only.
#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "finite ownership assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{
    CompiledLimits, CompiledProgram, CompiledThread, Error, Invocation, JavaHeap, JavaThread,
    Limits, Program,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};

type Result<T = ()> = std::result::Result<T, Error>;
const CLASS: &str = "CanonicalSharedStatics";
const BYTES: usize = 1024 * 1024;
const CLEANUP: usize = 5_000_000;

fn classes(bytes: &[&[u8]]) -> Result<Program> {
    Program::from_classfiles(
        bytes
            .iter()
            .map(|bytes| ClassFile::from_bytes(bytes))
            .collect::<std::result::Result<Vec<_>, _>>()?,
    )
}
fn statics() -> Result<Program> {
    classes(&[
        include_bytes!("fixtures/canonical/CanonicalStaticRootsSmall.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticRootsSmall$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticRootsFull.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticRootsFull$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$Journal.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$A.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$B.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$Lazy.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$Fieldless.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$Bad.class"),
        include_bytes!("fixtures/canonical/CanonicalStaticProtocol$NestedBad.class"),
    ])
}
fn shared() -> Result<Program> {
    classes(&[
        include_bytes!("fixtures/canonical/CanonicalSharedStatics.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Holder.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Slow.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Bad.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$Nested.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$CycleA.class"),
        include_bytes!("fixtures/canonical/CanonicalSharedStatics$CycleB.class"),
    ])
}
fn configuration() -> Configuration {
    Configuration {
        objects: 32,
        roots: 64,
        mutators: 3,
        helpers: 3,
        edges_per_object: 8,
    }
}
fn native_limits() -> CompiledLimits {
    CompiledLimits {
        cleanup_polls: CLEANUP,
        native_slices: 1_000_000,
        ..CompiledLimits::default()
    }
}

// Outside Tokio, yield_now wakes this exact waker; Notify waiting does not. The
// bounded executor distinguishes an actual class wait without clocks or sleeps.
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
// These fixture entries first request a static field on the target class.
// The explicit native stack yields once at that request and once while the
// initializer's storage publication is pending; the interpreter reaches the
// same owning lease in its first poll. Park at the lease, not at request emission.
fn park_initializer<F: Future>(mut future: Pin<&mut F>, native: bool) {
    for _ in 0..=usize::from(native) {
        park_first(future.as_mut());
    }
}
fn class_wait<F: Future>(mut future: Pin<&mut F>) -> Result<Arc<WakeCount>> {
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(count.clone());
    let mut context = Context::from_waker(&waker);
    for _ in 0..50_000 {
        let before = count.0.load(Ordering::Relaxed);
        assert!(future.as_mut().poll(&mut context).is_pending());
        if count.0.load(Ordering::Relaxed) == before {
            return Ok(count);
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

enum Thread<'c, 'p, 'h, 'gc> {
    Native(CompiledThread<'c, 'p, 'h, 'gc>),
    Interpreted(JavaThread<'p, 'h, 'gc>),
}
impl<'c, 'p, 'h, 'gc> Thread<'c, 'p, 'h, 'gc> {
    fn attach(
        native: bool,
        compiled: &'c CompiledProgram<'p>,
        heap: &'h JavaHeap<'p, 'gc>,
    ) -> Result<Self> {
        Ok(if native {
            Self::Native(compiled.attach(heap, native_limits())?)
        } else {
            Self::Interpreted(heap.attach(Limits::default())?)
        })
    }
    async fn invoke(&mut self, method: &str, value: i32) -> Result<Invocation> {
        self.invoke_at(CLASS, method, value).await
    }
    async fn invoke_at(&mut self, class: &str, method: &str, value: i32) -> Result<Invocation> {
        match self {
            Self::Native(thread) => thread.invoke(class, method, value).await,
            Self::Interpreted(thread) => thread.invoke(class, method, value).await,
        }
    }
}

#[path = "support/canonical_static_handoff.rs"]
mod handoff;

#[test]
fn outgoing_putstatic_root_survives_another_owner_wait_and_cleared_static_return() -> Result {
    let program = handoff::program()?;
    let compiled_a = program.compile()?;
    let compiled_b = program.compile()?;
    for (a, b) in [(true, true), (true, false), (false, true)] {
        complete(
            program.with_heap(handoff::configuration(), BYTES, CLEANUP, async |heap| {
                let mut owner = Thread::attach(a, &compiled_a, heap)?;
                let mut waiter = Thread::attach(b, &compiled_b, heap)?;
                let mut other = heap.attach(Limits::default())?;
                let mut initializing = Box::pin(owner.invoke_at(handoff::CLASS, "prepare", 0));
                park_initializer(initializing.as_mut(), a);
                let mut waiting = Box::pin(waiter.invoke_at(handoff::CLASS, "store", 31));
                let wake = class_wait(waiting.as_mut())?;
                let before = wake.0.load(Ordering::Relaxed);
                let cycles = other.primary_cycles()?.upper;
                for _ in 0..8 {
                    assert_eq!(other.invoke(handoff::CLASS, "churn", 17).await?.result, 136);
                }
                assert!(other.primary_cycles()?.lower > cycles);
                assert_eq!(wake.0.load(Ordering::Relaxed), before);
                assert_eq!(initializing.await?.result, 32640);
                assert!(wake.0.load(Ordering::Relaxed) > before);
                let result = waiting.await?;
                assert_eq!(
                    (
                        result.result,
                        result.class_initializer_calls,
                        result.class_initialization_waits
                    ),
                    (32671, 0, 1)
                );
                assert_eq!(
                    waiter.invoke_at(handoff::CLASS, "empty", 0).await?.result,
                    1
                );
                for row in handoff::ORACLE.lines() {
                    let fields: Vec<_> = row.split('\t').collect();
                    let [method, argument, expected] = fields.as_slice() else {
                        return Err(Error::Boundary("handoff oracle columns"));
                    };
                    assert_eq!(
                        waiter
                            .invoke_at(
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
                }
                Ok(())
            }),
        )?;
    }
    Ok(())
}

#[test]
fn actual_static_initializers_execute_natively_and_match_hotspot() -> Result {
    let program = statics()?;
    let compiled = program.compile()?;
    let mut rows = 0;
    for row in include_str!("fixtures/canonical/static-oracle.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let [class, method, argument, expected] = fields.as_slice() else {
            return Err(Error::Boundary("oracle columns"));
        };
        let before: u64 = compiled
            .method_profiles()?
            .iter()
            .filter(|p| p.name == "<clinit>")
            .map(|p| p.entries)
            .sum();
        let result = complete(
            compiled.run(
                class,
                method,
                argument
                    .parse()
                    .map_err(|_| Error::Boundary("oracle argument"))?,
                Configuration {
                    objects: if class.ends_with("Full") { 4096 } else { 64 },
                    mutators: 1,
                    helpers: 1,
                    ..configuration()
                },
                native_limits(),
            ),
        );
        if let Ok(expected) = expected.parse::<i32>() {
            let report = result?;
            assert_eq!(report.result, expected);
            assert_eq!(report.allocations, report.released_objects);
            assert_eq!(report.payload_bytes_after_cleanup, 0);
            assert_eq!(report.catalog_bytes_after_drop, 0);
            assert!(!report.instruction_count_available);
            assert_eq!(report.instructions, 0);
            assert_eq!(report.calls, report.compiled_entries);
            let after: u64 = compiled
                .method_profiles()?
                .iter()
                .filter(|p| p.name == "<clinit>")
                .map(|p| p.entries)
                .sum();
            assert_eq!(after - before, report.class_initializer_calls as u64);
            if class.starts_with("CanonicalStaticRoots") {
                assert_eq!(report.class_initializer_calls, 1);
                assert_eq!(
                    report.entry_initialization.allocations,
                    if class.ends_with("Full") { 295 } else { 5 }
                );
                assert_eq!(report.entry_initialization.calls, 1);
                assert!(report.entry_initialization.native_slices > 0);
                assert!(!report.entry_initialization.instruction_count_available);
                assert_eq!(report.entry_initialization.instructions, 0);
            }
        } else {
            let Err(Error::Initialization { kind, cause }) = result else {
                return Err(Error::Boundary("expected Java initialization fault"));
            };
            assert_eq!(kind, "ExceptionInInitializerError");
            assert!(matches!(
                cause.as_deref(),
                Some(Error::ArrayFault("NegativeArraySizeException"))
            ));
        }
        rows += 1;
    }
    assert_eq!(rows, 16);
    Ok(())
}

#[test]
fn native_and_interpreted_entries_share_persistent_static_state() -> Result {
    let program = shared()?;
    let compiled_a = program.compile()?;
    let compiled_b = program.compile()?;
    for (a, b) in [(true, true), (true, false), (false, true)] {
        complete(
            program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
                let mut first = Thread::attach(a, &compiled_a, heap)?;
                let mut second = Thread::attach(b, &compiled_b, heap)?;
                let mut rows = 0;
                let mut storage = 0;
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
                            method,
                            argument
                                .parse()
                                .map_err(|_| Error::Boundary("oracle argument"))?,
                        )
                        .await;
                    if let Ok(expected) = expected.parse::<i32>() {
                        let result = result?;
                        assert_eq!(result.result, expected);
                        storage += result.static_storage_allocations;
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
                assert_eq!((rows, storage), (14, 5));
                assert!(heap.managed_bytes() > 0);
                Ok(())
            }),
        )?;
    }
    Ok(())
}

#[test]
fn native_class_wait_retains_roots_while_other_classes_and_gc_progress() -> Result {
    let program = shared()?;
    let compiled_a = program.compile()?;
    let compiled_b = program.compile()?;
    for (a, b) in [(true, true), (true, false), (false, true)] {
        complete(
            program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
                let mut owner = Thread::attach(a, &compiled_a, heap)?;
                let mut waiter = Thread::attach(b, &compiled_b, heap)?;
                let mut other = heap.attach(Limits::default())?;
                let mut initializing = Box::pin(owner.invoke("slow", 0));
                park_initializer(initializing.as_mut(), a);
                let mut waiting = Box::pin(waiter.invoke("waitWithRoot", 31));
                let wake = class_wait(waiting.as_mut())?;
                let before = wake.0.load(Ordering::Relaxed);
                let cycles = other.primary_cycles()?.upper;
                for value in 0..32 {
                    assert_eq!(
                        other.invoke(CLASS, "write", value).await?.result,
                        value * 2 + 7
                    );
                }
                assert!(other.primary_cycles()?.lower > cycles);
                assert_eq!(wake.0.load(Ordering::Relaxed), before);
                let result = initializing.await?;
                assert_eq!((result.result, result.class_initializer_calls), (32657, 1));
                assert!(wake.0.load(Ordering::Relaxed) > before);
                let result = waiting.await?;
                assert_eq!(
                    (
                        result.result,
                        result.class_initializer_calls,
                        result.static_storage_allocations,
                        result.class_initialization_waits
                    ),
                    (32688, 0, 0, 1)
                );
                Ok(())
            }),
        )?;
    }
    Ok(())
}

#[test]
fn native_owner_failure_or_cancellation_wakes_waiters_and_never_replays() -> Result {
    let program = shared()?;
    let compiled_a = program.compile()?;
    let compiled_b = program.compile()?;
    for (a, b) in [(true, true), (true, false), (false, true)] {
        for cancel in [false, true] {
            complete(
                program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
                    let mut owner = Thread::attach(a, &compiled_a, heap)?;
                    let mut waiter = Thread::attach(b, &compiled_b, heap)?;
                    let method = if cancel { "slow" } else { "bad" };
                    let mut initializing = Box::pin(owner.invoke(method, 0));
                    park_initializer(initializing.as_mut(), a);
                    let mut waiting = Box::pin(waiter.invoke(method, 0));
                    let wake = class_wait(waiting.as_mut())?;
                    let before = wake.0.load(Ordering::Relaxed);
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
                    assert!(wake.0.load(Ordering::Relaxed) > before);
                    if cancel {
                        aborted(&waiting.await);
                        aborted(&owner.invoke(method, 0).await);
                    } else {
                        assert!(matches!(
                            waiting.await,
                            Err(Error::Initialization {
                                kind: "NoClassDefFoundError",
                                ..
                            })
                        ));
                        assert!(matches!(
                            owner.invoke(method, 0).await,
                            Err(Error::Initialization {
                                kind: "NoClassDefFoundError",
                                ..
                            })
                        ));
                    }
                    assert_eq!(owner.invoke("write", 4).await?.result, 15);
                    assert_eq!(waiter.invoke("read", 0).await?.result, 15);
                    Ok(())
                }),
            )?;
        }
    }
    Ok(())
}

#[test]
fn cancelled_native_waiter_preserves_owner_and_native_cycles_cancel_cleanly() -> Result {
    let program = shared()?;
    let compiled_a = program.compile()?;
    let compiled_b = program.compile()?;
    complete(
        program.with_heap(configuration(), BYTES, CLEANUP, async |heap| {
            let mut first = compiled_a.attach(heap, native_limits())?;
            let mut second = compiled_b.attach(heap, native_limits())?;
            let mut initializing = Box::pin(first.invoke(CLASS, "slow", 0));
            park_initializer(initializing.as_mut(), true);
            let mut waiting = Box::pin(second.invoke(CLASS, "waitWithRoot", 31));
            let _wake = class_wait(waiting.as_mut())?;
            drop(waiting);
            assert_eq!(initializing.await?.result, 32657);
            assert_eq!(second.invoke(CLASS, "slow", 1).await?.result, 32658);
            let mut a = Box::pin(first.invoke(CLASS, "cycleA", 0));
            park_initializer(a.as_mut(), true);
            let mut b = Box::pin(second.invoke(CLASS, "cycleB", 0));
            park_initializer(b.as_mut(), true);
            let _a_wake = class_wait(a.as_mut())?;
            let b_wake = class_wait(b.as_mut())?;
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

#[cfg(feature = "experimental-canonical-telemetry")]
fn telemetry()
-> std::result::Result<ristretto_vm::canonical_telemetry::Telemetry, Box<dyn std::error::Error>> {
    use ristretto_vm::canonical_telemetry::{Binding, Telemetry, current_process_id};
    Ok(Telemetry::new(
        65_536,
        Binding {
            process_id: current_process_id(),
            run_id: "native-static-correctness".to_owned(),
            source_sha256: "0".repeat(64),
            binary_sha256: "0".repeat(64),
            workload_sha256: "0".repeat(64),
            configuration_sha256: "0".repeat(64),
        },
    )?)
}

#[cfg(feature = "experimental-canonical-telemetry")]
#[test]
fn native_initialization_is_outside_entry_body_and_cancel_or_failure_never_opens_it()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    use ristretto_vm::canonical_telemetry::{CycleCapture, Kind, Operation, Outcome};
    let program = statics()?;
    let compiled = program.compile()?;
    let config = Configuration {
        objects: 64,
        mutators: 1,
        helpers: 1,
        ..configuration()
    };
    let recorder = telemetry()?;
    let report = complete(compiled.run_recorded(
        "CanonicalStaticRootsSmall",
        "prepared",
        0,
        config,
        native_limits(),
        &recorder,
    ))?;
    let capture = recorder.finish().map_err(|_| "recorder escaped")?;
    assert!(
        capture.snapshot.instrumented_capture_complete,
        "{:?}",
        capture.snapshot.issues
    );
    assert!(!capture.snapshot.global_capture_complete);
    assert!(matches!(capture.cycles, CycleCapture::Complete(_)));
    let window = capture
        .snapshot
        .events
        .iter()
        .find(|event| event.kind == Kind::Measurement)
        .ok_or("missing body")?;
    let before_allocations = capture
        .snapshot
        .events
        .iter()
        .filter(|event| {
            event.kind == Kind::Request
                && event.operation == Operation::Allocation
                && event.outcome == Some(Outcome::Complete)
                && event.end_ns.is_some_and(|end| end <= window.start_ns)
        })
        .count();
    assert_eq!(before_allocations, report.entry_initialization.allocations);
    assert_eq!(before_allocations, 5);
    assert_eq!(report.allocations, 5);
    assert!(report.entry_initialization.native_slices > 0);
    assert!(!report.entry_initialization.instruction_count_available);
    assert_eq!(report.entry_initialization.instructions, 0);
    for cancel in [false, true] {
        let recorder = telemetry()?;
        if cancel {
            let mut future = Box::pin(compiled.run_recorded(
                "CanonicalStaticRootsSmall",
                "prepared",
                0,
                config,
                native_limits(),
                &recorder,
            ));
            park_first(future.as_mut());
            drop(future);
        } else {
            assert!(matches!(
                complete(compiled.run_recorded(
                    "CanonicalStaticProtocol$Bad",
                    "run",
                    0,
                    config,
                    native_limits(),
                    &recorder
                )),
                Err(Error::Initialization {
                    kind: "ExceptionInInitializerError",
                    ..
                })
            ));
        }
        let capture = recorder.finish().map_err(|_| "recorder escaped")?;
        assert!(!capture.snapshot.instrumented_capture_complete);
        assert_eq!(capture.cycles, CycleCapture::Unobserved);
        assert!(
            !capture
                .snapshot
                .events
                .iter()
                .any(|event| event.kind == Kind::Measurement)
        );
        assert_eq!(capture.snapshot.open_events, 0);
        assert!(compiled.method_profiles().is_ok());
    }
    Ok(())
}
