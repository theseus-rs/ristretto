//! Admitted Java sessions on an existing collector; ordinary roots remain separate.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration ownership assertions"
)]

use ristretto_classfile::ClassFile;
use ristretto_classloader::Value;
use ristretto_gc::GarbageCollector;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};
use std::future::Future;
use std::task::{Context, Waker};

const CLASS: &str = "CanonicalGraphWorkload";
const CLEANUP: usize = 100_000;

fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])
}

fn configuration() -> Configuration {
    Configuration {
        objects: 8,
        roots: 24,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn existing_ordinary_root_and_parked_second_owner_survive_session_cleanup()
-> Result<(), Box<dyn std::error::Error>> {
    let collector = GarbageCollector::new();
    let ordinary = Value::new_object(&collector, vec![19_i32, 23].into());
    let baseline = collector.managed_bytes();
    let program = program()?;
    let mut parked =
        Box::pin(
            program.with_collector(&collector, configuration(), CLEANUP, async |heap| {
                let mut thread = heap.attach(Limits::default())?;
                thread.invoke(CLASS, "run", 50).await?;
                std::future::pending::<()>().await;
                Ok(())
            }),
        );
    for _ in 0..100 {
        assert!(
            parked
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
    }
    let parked_bytes = collector.managed_bytes();
    assert!(parked_bytes > baseline);
    let completion = program
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            assert_eq!(heap.managed_bytes(), parked_bytes);
            heap.attach(Limits::default())?
                .invoke(CLASS, "run", 3)
                .await
        })
        .await?;
    assert_eq!(
        (completion.value.result, completion.value.allocations),
        (9, 6)
    );
    assert_eq!(
        completion.value.released_objects + completion.released_during_cleanup,
        6
    );
    assert_eq!(collector.managed_bytes(), parked_bytes);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[19, 23]);
    drop(parked);
    assert_eq!(collector.managed_bytes(), baseline);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[19, 23]);
    Ok(())
}

#[tokio::test]
async fn cancelled_and_limited_sessions_refund_only_their_payloads()
-> Result<(), Box<dyn std::error::Error>> {
    let collector = GarbageCollector::new();
    let ordinary = Value::new_object(&collector, vec![31_i32].into());
    let baseline = collector.managed_bytes();
    let program = program()?;
    for prefix in [1, 16, 100] {
        let mut cancelled =
            Box::pin(
                program.with_collector(&collector, configuration(), CLEANUP, async |heap| {
                    heap.attach(Limits::default())?
                        .invoke(CLASS, "run", 50)
                        .await
                }),
            );
        for _ in 0..prefix {
            assert!(
                cancelled
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
        }
        assert!(collector.managed_bytes() > baseline);
        drop(cancelled);
        assert_eq!(collector.managed_bytes(), baseline);
    }
    assert!(matches!(
        program
            .with_collector(&collector, configuration(), 0, async |heap| {
                heap.attach(Limits::default())?
                    .invoke(CLASS, "run", 3)
                    .await
            },)
            .await,
        Err(Error::Limit("heap cleanup poll"))
    ));
    assert_eq!(collector.managed_bytes(), baseline);
    assert!(matches!(
        program
            .with_collector(&collector, configuration(), CLEANUP, async |heap| {
                heap.attach(Limits {
                    instructions: 1,
                    ..Limits::default()
                })?
                .invoke(CLASS, "run", 3)
                .await
            },)
            .await,
        Err(Error::Limit("instruction"))
    ));
    assert_eq!(collector.managed_bytes(), baseline);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[31]);
    Ok(())
}

#[tokio::test]
async fn cancelled_invocation_finishes_through_bounded_session_cleanup()
-> Result<(), Box<dyn std::error::Error>> {
    let collector = GarbageCollector::new();
    let ordinary = Value::new_object(&collector, vec![29_i32].into());
    let baseline = collector.managed_bytes();
    let program = program()?;
    let completion = program
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            let mut invocation = Box::pin(thread.invoke(CLASS, "run", 50));
            for _ in 0..100 {
                assert!(
                    invocation
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(heap.managed_bytes() > baseline);
            drop(invocation);
            Ok(())
        })
        .await?;
    assert!((1..=CLEANUP).contains(&completion.cleanup_polls));
    assert_eq!(collector.managed_bytes(), baseline);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[29]);
    Ok(())
}

#[tokio::test]
async fn supplied_collector_payload_limit_is_shared_with_ordinary_allocations()
-> Result<(), Box<dyn std::error::Error>> {
    let reference: ristretto_classloader::Reference = vec![7_i32; 16].into();
    // The public ordinary constructor charges inline storage only. Preserve that
    // scheme, and consume its exact charge before canonical allocation.
    let ordinary_size = size_of::<ristretto_gc::sync::RwLock<ristretto_classloader::Reference>>();
    let collector = GarbageCollector::with_config(ristretto_gc::Configuration {
        max_heap_bytes: ordinary_size,
        ..Default::default()
    });
    let ordinary = Value::new_object(&collector, reference);
    let baseline = collector.managed_bytes();
    assert_eq!(baseline, ordinary_size);
    let result = program()?
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            heap.attach(Limits::default())?
                .invoke(CLASS, "run", 50)
                .await
        })
        .await;
    assert!(matches!(
        result,
        Err(Error::Collector(ristretto_gc::canonical::Error::Heap(_)))
    ));
    assert_eq!(collector.managed_bytes(), baseline);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[7; 16]);
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_sessions_preserve_ordinary_roots_and_cancel_pending_frames()
-> Result<(), Box<dyn std::error::Error>> {
    use ristretto_vm::canonical::CompiledLimits;
    let collector = GarbageCollector::new();
    let ordinary = Value::new_object(&collector, vec![37_i32].into());
    let baseline = collector.managed_bytes();
    let program = program()?;
    let compiled = program.compile()?;
    for prefix in [1, 16, 100] {
        let mut cancelled =
            Box::pin(
                program.with_collector(&collector, configuration(), CLEANUP, async |heap| {
                    compiled
                        .attach(heap, CompiledLimits::default())?
                        .invoke(CLASS, "run", 50)
                        .await
                }),
            );
        // Native entry setup can yield before its first allocation. Reach an
        // actual owned payload before each additional cancellation prefix.
        for _ in 0..1_000 {
            assert!(
                cancelled
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            if collector.managed_bytes() > baseline {
                break;
            }
        }
        assert!(collector.managed_bytes() > baseline);
        for _ in 0..prefix {
            assert!(
                cancelled
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
        }
        assert!(collector.managed_bytes() > baseline);
        drop(cancelled);
        assert_eq!(collector.managed_bytes(), baseline);
    }
    let cancelled = program
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            let mut thread = compiled.attach(heap, CompiledLimits::default())?;
            let mut invocation = Box::pin(thread.invoke(CLASS, "run", 50));
            for _ in 0..100 {
                assert!(
                    invocation
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            }
            assert!(heap.managed_bytes() > baseline);
            drop(invocation);
            Ok(())
        })
        .await?;
    assert!((1..=CLEANUP).contains(&cancelled.cleanup_polls));
    assert_eq!(collector.managed_bytes(), baseline);
    let completion = program
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            compiled
                .attach(heap, CompiledLimits::default())?
                .invoke(CLASS, "run", 3)
                .await
        })
        .await?;
    assert_eq!(
        (completion.value.result, completion.value.allocations),
        (9, 6)
    );
    assert_eq!(
        completion.value.released_objects + completion.released_during_cleanup,
        6
    );
    assert_eq!(collector.managed_bytes(), baseline);
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[37]);
    // Public Invocation has no native census. A zero slice budget must reject
    // this same session path; it cannot silently interpret the method.
    let refused = program
        .with_collector(&collector, configuration(), CLEANUP, async |heap| {
            compiled
                .attach(
                    heap,
                    CompiledLimits {
                        native_slices: 0,
                        ..CompiledLimits::default()
                    },
                )?
                .invoke(CLASS, "run", 3)
                .await
        })
        .await;
    assert!(matches!(refused, Err(Error::Limit("native slice"))));
    assert_eq!(collector.managed_bytes(), baseline);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn existing_vm_entry_uses_its_actual_collector() -> Result<(), Box<dyn std::error::Error>> {
    let vm = ristretto_vm::VM::new(
        ristretto_vm::ConfigurationBuilder::new()
            .batch_compilation(false)
            .build()?,
    )
    .await?;
    let ordinary = Value::new_object(vm.garbage_collector(), vec![41_i32, 43].into());
    let program = program()?;
    let result = program
        .with_vm(&vm, configuration(), CLEANUP, async |heap| {
            assert!(heap.managed_bytes() > 0);
            heap.attach(Limits::default())?
                .invoke(CLASS, "run", 3)
                .await
        })
        .await?;
    assert_eq!(result.value.result, 9);
    #[cfg(feature = "experimental-canonical-jit")]
    {
        let compiled = program.compile()?;
        let result = program
            .with_vm(&vm, configuration(), CLEANUP, async |heap| {
                compiled
                    .attach(heap, ristretto_vm::canonical::CompiledLimits::default())?
                    .invoke(CLASS, "run", 4)
                    .await
            })
            .await?;
        assert_eq!(result.value.result, 16);
    }
    assert_eq!(&*ordinary.as_int_vec_ref()?, &[41, 43]);
    Ok(())
}
