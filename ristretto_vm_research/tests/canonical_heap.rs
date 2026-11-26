//! Real Java entries sharing one private canonical catalog.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Limits, Program};
use std::future::Future;
use std::task::{Context, Waker};

const CLASS: &str = "CanonicalGraphWorkload";
const PAYLOAD_LIMIT: usize = 1024 * 1024;
const DRAIN_LIMIT: usize = 1_000_000;

fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])
}

fn configuration() -> Configuration {
    Configuration {
        objects: 8,
        roots: 24,
        mutators: 2,
        helpers: 2,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn parked_java_frame_survives_other_entries_and_slot_reuse() -> Result<(), Error> {
    let completion = program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut first = heap.attach(Limits::default())?;
            let mut second = heap.attach(Limits::default())?;
            assert!(heap.attach(Limits::default()).is_err());
            let mut parked = Box::pin(first.invoke(CLASS, "run", 50));
            for _ in 0..100 {
                assert!(
                    parked
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(heap.managed_bytes() > 0);
            let start = second.primary_cycles()?;
            let mut released = 0;
            for _ in 0..8 {
                let result = second.invoke(CLASS, "run", 3).await?;
                assert_eq!((result.result, result.allocations), (9, 6));
                released += result.released_objects;
            }
            assert!(released >= 48 - configuration().objects);
            assert!(second.primary_cycles()?.lower > start.upper);
            let result = parked.await?;
            assert_eq!((result.result, result.allocations), (2500, 100));
            Ok(released + result.released_objects)
        })
        .await?;
    assert_eq!(completion.value + completion.released_during_cleanup, 148);
    Ok(())
}

#[tokio::test]
async fn cancelled_and_limited_calls_release_frames_and_contexts_are_reusable() -> Result<(), Error>
{
    program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            for prefix in [1, 16, 100] {
                let mut cancelled = Box::pin(thread.invoke(CLASS, "run", 50));
                for _ in 0..prefix {
                    assert!(
                        cancelled
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .is_pending()
                    );
                }
                drop(cancelled);
                let result = thread.invoke(CLASS, "run", 3).await?;
                assert_eq!((result.result, result.allocations), (9, 6));
            }
            let mut limited = heap.attach(Limits {
                instructions: 3,
                ..Limits::default()
            })?;
            for _ in 0..2 {
                assert!(matches!(
                    limited.invoke(CLASS, "run", 50).await,
                    Err(Error::Limit("instruction"))
                ));
            }
            drop(limited);
            let _reused = heap.attach(Limits::default())?;
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn payload_and_cleanup_limits_apply_to_the_shared_heap() -> Result<(), Error> {
    let program = program()?;
    let result = program
        .with_heap(configuration(), 1, DRAIN_LIMIT, async |heap| {
            heap.attach(Limits::default())?
                .invoke(CLASS, "run", 1)
                .await
        })
        .await;
    assert!(matches!(
        result,
        Err(Error::Collector(ristretto_gc::canonical::Error::Heap(_)))
    ));
    let result = program
        .with_heap(configuration(), PAYLOAD_LIMIT, 0, async |heap| {
            heap.attach(Limits::default())?
                .invoke(CLASS, "run", 1)
                .await
        })
        .await;
    assert!(matches!(result, Err(Error::Limit("heap cleanup poll"))));
    Ok(())
}

#[tokio::test]
async fn partial_registration_failure_refunds_the_mutator() -> Result<(), Error> {
    let configuration = Configuration {
        helpers: 1,
        ..configuration()
    };
    program()?
        .with_heap(configuration, PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let first = heap.attach(Limits::default())?;
            // A mutator is available but the helper claim fails. Its temporary
            // mutator must be returned, so both repeated failures and later attach work.
            assert!(heap.attach(Limits::default()).is_err());
            assert!(heap.attach(Limits::default()).is_err());
            drop(first);
            let mut next = heap.attach(Limits::default())?;
            assert_eq!(next.invoke(CLASS, "run", 1).await?.result, 1);
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn independent_os_threads_execute_on_the_same_heap() -> Result<(), Error> {
    use ristretto_vm_research::canonical::JavaThread;
    use std::sync::mpsc::channel;
    use std::time::Duration;
    fn work(mut thread: JavaThread<'_, '_, '_>) -> Result<usize, Error> {
        let executor = tokio::runtime::Builder::new_current_thread()
            .build()
            .map_err(|error| Error::Invalid(error.to_string()))?;
        executor.block_on(async {
            let mut released = 0;
            for _ in 0..8 {
                let result = thread.invoke(CLASS, "run", 3).await?;
                assert_eq!((result.result, result.allocations), (9, 6));
                released += result.released_objects;
            }
            Ok(released)
        })
    }
    let result = program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            std::thread::scope(|scope| {
                let (ready, started) = channel();
                let (go, begin) = channel();
                let first = scope.spawn(move || {
                    let thread = heap.attach(Limits::default())?;
                    ready.send(()).map_err(|e| Error::Invalid(e.to_string()))?;
                    begin
                        .recv_timeout(Duration::from_secs(30))
                        .map_err(|e| Error::Invalid(e.to_string()))?;
                    work(thread)
                });
                let second = scope.spawn(move || {
                    started
                        .recv_timeout(Duration::from_secs(30))
                        .map_err(|e| Error::Invalid(e.to_string()))?;
                    let thread = heap.attach(Limits::default())?;
                    go.send(()).map_err(|e| Error::Invalid(e.to_string()))?;
                    work(thread)
                });
                let a = first
                    .join()
                    .map_err(|_| Error::Invalid("first task panicked".into()))??;
                let b = second
                    .join()
                    .map_err(|_| Error::Invalid("second task panicked".into()))??;
                Ok(a + b)
            })
        })
        .await?;
    assert_eq!(result.value + result.released_during_cleanup, 96);
    Ok(())
}

#[tokio::test]
async fn java_null_fault_leaves_the_context_reusable() -> Result<(), Error> {
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalIntObject.class"
    ))?])?;
    program
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            assert!(
                matches!(thread.invoke("CanonicalIntObject", "fault", 0).await,
            Err(Error::Invalid(name)) if name == "NullPointerException")
            );
            assert_eq!(
                thread
                    .invoke("CanonicalIntObject", "read", -37)
                    .await?
                    .result,
                -37
            );
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn scalar_entries_keep_their_primitive_dispatch_without_allocations() -> Result<(), Error> {
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalScalarEntry.class"
    ))?])?;
    let completion = program
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            let answer = thread.invoke("CanonicalScalarEntry", "answer", 0).await?;
            assert_eq!((answer.result, answer.allocations), (64, 0));
            assert_eq!(
                thread
                    .invoke("CanonicalScalarEntry", "identity", i32::MIN)
                    .await?
                    .result,
                i32::MIN
            );
            assert_eq!(heap.managed_bytes(), 0);
            Ok(())
        })
        .await?;
    assert_eq!(
        (completion.cleanup_polls, completion.released_during_cleanup),
        (0, 0)
    );
    Ok(())
}

#[path = "support/canonical_heap_additional.rs"]
mod additional_regressions;

#[tokio::test]
async fn long_scalar_entries_yield_to_a_second_mutator_and_preserve_the_budget() -> Result<(), Error>
{
    let class = "CanonicalScalarLoop";
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalScalarLoop.class"
    ))?])?;
    let rounds = 3_000;
    let expected = program.run_scalar(class, "calculate", rounds)?;
    program
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut first = heap.attach(Limits::default())?;
            let mut second = heap.attach(Limits::default())?;
            let mut call = Box::pin(first.invoke(class, "calculate", rounds));
            assert!(
                call.as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert_eq!(
                second.invoke(class, "calculate", 2).await?.result,
                program.run_scalar(class, "calculate", 2)?
            );
            assert_eq!(call.await?.result, expected);
            drop(second);
            let mut limited = heap.attach(Limits {
                instructions: 128,
                ..Limits::default()
            })?;
            assert!(matches!(
                limited.invoke(class, "calculate", rounds).await,
                Err(Error::Limit("instructions"))
            ));
            assert_eq!(heap.managed_bytes(), 0);
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn scalar_returns_at_the_exact_budget_across_a_scheduler_yield() -> Result<(), Error> {
    let class = "CanonicalScalarLoop";
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalScalarLoop.class"
    ))?])?;
    // javap: four prologue opcodes, eleven per iteration, three exit-test
    // opcodes and two return opcodes. Five rounds place ireturn at bytecode 64.
    for rounds in [0_i32, 5, 6, 69] {
        let instructions = 9 + 11 * usize::try_from(rounds).expect("nonnegative rounds");
        let expected = (0..rounds).fold(1_i32, |acc, i| acc.wrapping_mul(31).wrapping_sub(i));
        for limit in [instructions - 1, instructions, instructions + 1] {
            let synchronous = program.run_scalar_with_limit(class, "calculate", rounds, limit);
            let result = program
                .run(
                    class,
                    "calculate",
                    rounds,
                    Configuration {
                        mutators: 1,
                        helpers: 1,
                        ..configuration()
                    },
                    Limits {
                        instructions: limit,
                        ..Limits::default()
                    },
                )
                .await;
            if limit < instructions {
                assert!(matches!(synchronous, Err(Error::Limit("instructions"))));
                assert!(matches!(result, Err(Error::Limit("instructions"))));
            } else {
                assert_eq!(synchronous?, expected);
                let report = result?;
                assert_eq!(
                    (report.result, report.instructions, report.allocations),
                    (expected, instructions, 0)
                );
            }
        }
    }
    Ok(())
}
