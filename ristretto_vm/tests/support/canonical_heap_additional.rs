//! Repeated entries and independently parked local Java contexts on one heap.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};
use std::future::Future;
use std::task::{Context, Waker};

const CLASS: &str = "CanonicalRingWorkload";
const PAYLOAD_LIMIT: usize = 1024 * 1024;
const DRAIN_LIMIT: usize = 1_000_000;

fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalRingWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalRingWorkload$Node.class"
        ))?,
    ])
}

fn configuration() -> Configuration {
    Configuration {
        objects: 48,
        roots: 192,
        mutators: 2,
        helpers: 2,
        edges_per_object: 3,
    }
}

#[tokio::test]
async fn parked_java_thread_retains_its_graph_while_another_reuses_slots() -> Result<(), Error> {
    let completion = program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut first = heap.attach(Limits::default())?;
            let mut second = heap.attach(Limits::default())?;
            assert!(heap.attach(Limits::default()).is_err());
            let mut parked = Box::pin(first.invoke(CLASS, "small", 17));
            for _ in 0..120 {
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
                let outcome = second.invoke(CLASS, "small", 1).await?;
                assert_eq!((outcome.result, outcome.allocations), (619_975_554, 18));
                released += outcome.released_objects;
            }
            assert!(released >= 8 * 18 - configuration().objects);
            assert!(second.primary_cycles()?.lower > start.upper);
            let outcome = parked.await?;
            assert_eq!((outcome.result, outcome.allocations), (652_228_386, 50));
            assert!(heap.managed_bytes() > 0);
            Ok(released + outcome.released_objects)
        })
        .await?;
    assert_eq!(completion.value + completion.released_during_cleanup, 194);
    assert!(completion.cleanup_polls > 0);
    Ok(())
}

#[tokio::test]
async fn cancelled_or_faulting_invocations_release_frames_and_the_thread_can_run_again()
-> Result<(), Error> {
    program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            for prefix in [1, 16, 120] {
                let mut cancelled = Box::pin(thread.invoke(CLASS, "small", 17));
                for _ in 0..prefix {
                    assert!(
                        cancelled
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .is_pending()
                    );
                }
                drop(cancelled);
                let result = thread.invoke(CLASS, "small", 1).await?;
                assert_eq!((result.result, result.allocations), (619_975_554, 18));
            }
            let mut limited = heap.attach(Limits {
                instructions: 3,
                ..Limits::default()
            })?;
            for _ in 0..2 {
                assert!(matches!(
                    limited.invoke(CLASS, "small", 17).await,
                    Err(Error::Limit("instruction"))
                ));
            }
            drop(limited);
            let _reused_quota = heap.attach(Limits::default())?;
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn payload_and_final_cleanup_limits_apply_to_the_live_heap() -> Result<(), Error> {
    let program = program()?;
    let run = program
        .with_heap(configuration(), 1, DRAIN_LIMIT, async |heap| {
            heap.attach(Limits::default())?
                .invoke(CLASS, "small", 1)
                .await
        })
        .await;
    assert!(matches!(
        run,
        Err(Error::Collector(ristretto_gc::canonical::Error::Heap(_)))
    ));
    let run = program
        .with_heap(configuration(), PAYLOAD_LIMIT, 0, async |heap| {
            heap.attach(Limits::default())?
                .invoke(CLASS, "small", 1)
                .await
        })
        .await;
    assert!(matches!(run, Err(Error::Limit("heap cleanup poll"))));
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn reusable_compiled_context_runs_alongside_interpreted_java_and_recovers_from_cancel()
-> Result<(), Error> {
    use ristretto_vm::canonical::CompiledLimits;
    let program_a = program()?;
    let compiled = program_a.compile()?;
    let other_program = program()?;
    let foreign = other_program.compile()?;
    program_a
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            assert!(matches!(
                foreign.attach(heap, CompiledLimits::default()),
                Err(Error::Boundary(_))
            ));
            let mut native = compiled.attach(heap, CompiledLimits::default())?;
            let mut interpreted = heap.attach(Limits::default())?;
            let mut parked = Box::pin(native.invoke(CLASS, "small", 17));
            for _ in 0..120 {
                assert!(
                    parked
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(compiled.method_profiles().is_err());
            for _ in 0..4 {
                assert_eq!(
                    interpreted.invoke(CLASS, "small", 7).await?.result,
                    -493_097_750
                );
            }
            drop(parked);
            for _ in 0..3 {
                let outcome = native.invoke(CLASS, "small", 1).await?;
                assert_eq!((outcome.result, outcome.allocations), (619_975_554, 18));
                assert!(outcome.native_slices > 0);
            }
            Ok(())
        })
        .await?;
    assert!(
        compiled
            .method_profiles()?
            .iter()
            .any(|method| method.name == "small" && method.entries >= 4)
    );
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn native_threads_make_progress_with_independent_executors_on_one_catalog()
-> Result<(), Error> {
    use ristretto_vm::canonical::JavaThread;
    use std::sync::mpsc::channel;

    fn work(mut thread: JavaThread<'_, '_, '_>) -> Result<usize, Error> {
        let executor = tokio::runtime::Builder::new_current_thread()
            .build()
            .map_err(|_| Error::Boundary("local executor"))?;
        executor.block_on(async {
            let mut released = 0;
            for _ in 0..8 {
                let result = thread.invoke(CLASS, "small", 1).await?;
                assert_eq!((result.result, result.allocations), (619_975_554, 18));
                released += result.released_objects;
            }
            Ok(released)
        })
    }

    let result = program()?
        .with_heap(configuration(), PAYLOAD_LIMIT, DRAIN_LIMIT, async |heap| {
            std::thread::scope(|scope| {
                // Register in order, then run both workloads on their own executors.
                // Disconnect wakes the waiting task if a peer fails before startup.
                let (first_ready, started) = channel();
                let (second_ready, begin) = channel();
                let a = scope.spawn(move || {
                    let thread = heap.attach(Limits::default())?;
                    first_ready
                        .send(())
                        .map_err(|_| Error::Boundary("second task ended"))?;
                    begin
                        .recv()
                        .map_err(|_| Error::Boundary("second task ended"))?;
                    work(thread)
                });
                let b = scope.spawn(move || {
                    started
                        .recv()
                        .map_err(|_| Error::Boundary("first task ended"))?;
                    let thread = heap.attach(Limits::default())?;
                    second_ready
                        .send(())
                        .map_err(|_| Error::Boundary("first task ended"))?;
                    work(thread)
                });
                let first = a
                    .join()
                    .map_err(|_| Error::Boundary("first task panicked"))??;
                let second = b
                    .join()
                    .map_err(|_| Error::Boundary("second task panicked"))??;
                Ok(first + second)
            })
        })
        .await?;
    assert_eq!(result.value + result.released_during_cleanup, 16 * 18);
    Ok(())
}
