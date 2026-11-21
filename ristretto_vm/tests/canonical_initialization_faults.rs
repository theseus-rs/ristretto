//! Java initializer faults retain Java failure semantics after array/cast support.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "oracle and ownership assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};
use std::future::Future;
use std::task::{Context, Waker};

const CLASS: &str = "CanonicalInitializerFaults";
fn program() -> Result<Program, Error> {
    let bytes: &[&[u8]] = &[
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Bounds.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$BoundsStore.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Cast.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Negative.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$NegativeReference.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Nested.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$NestedNegative.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Node.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$NullArray.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Other.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Slow.class"
        ),
        include_bytes!(
            "fixtures/canonical_initializer_faults/CanonicalInitializerFaults$Store.class"
        ),
        include_bytes!("fixtures/canonical_initializer_faults/CanonicalInitializerFaults.class"),
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
        objects: 64,
        roots: 96,
        mutators: 2,
        helpers: 2,
        edges_per_object: 8,
    }
}
#[tokio::test]
async fn java_array_and_cast_initializer_failures_match_hotspot() -> Result<(), Error> {
    program()?
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut subsequent = heap.attach(Limits::default())?;
            for row in include_str!("fixtures/canonical_initializer_faults/oracle.txt").lines() {
                let columns: Vec<_> = row.split_whitespace().collect();
                let [method, attempt, expected, expected_cause] = columns.as_slice() else {
                    return Err(Error::Boundary("initializer oracle row"));
                };
                let thread = if *attempt == "0" {
                    &mut owner
                } else {
                    &mut subsequent
                };
                let Err(Error::Initialization { kind, cause }) =
                    thread.invoke(CLASS, method, 0).await
                else {
                    return Err(Error::Boundary("expected Java initializer failure"));
                };
                assert_eq!(kind, *expected, "{row}");
                let actual_cause = match cause.as_deref() {
                    Some(Error::ArrayFault(name) | Error::TypeFault(name)) => *name,
                    Some(Error::Invalid(name)) => name.as_str(),
                    None => "-",
                    _ => {
                        return Err(Error::Boundary(
                            "initializer cause was wrapped more than once",
                        ));
                    }
                };
                assert_eq!(actual_cause, *expected_cause, "{row}");
            }
            Ok(())
        })
        .await?;
    Ok(())
}
#[tokio::test]
async fn initializer_engine_limit_remains_an_embedding_failure() -> Result<(), Error> {
    program()?
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits {
                instructions: 8,
                ..Limits::default()
            })?;
            let mut other = heap.attach(Limits::default())?;
            assert!(matches!(
                owner.invoke(CLASS, "slow", 0).await,
                Err(Error::Limit(_))
            ));
            assert!(matches!(
                other.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            Ok(())
        })
        .await?;
    Ok(())
}
#[tokio::test]
async fn cancelled_initializer_remains_an_embedding_failure() -> Result<(), Error> {
    program()?
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = heap.attach(Limits::default())?;
            let mut other = heap.attach(Limits::default())?;
            let mut initializing = Box::pin(owner.invoke(CLASS, "slow", 0));
            assert!(
                initializing
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            assert!(heap.managed_bytes() > 0);
            drop(initializing);
            assert!(matches!(
                other.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            Ok(())
        })
        .await?;
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_java_array_and_cast_initializer_failures_match_hotspot() -> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut subsequent =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            for row in include_str!("fixtures/canonical_initializer_faults/oracle.txt").lines() {
                let columns: Vec<_> = row.split_whitespace().collect();
                let [method, attempt, expected, expected_cause] = columns.as_slice() else {
                    return Err(Error::Boundary("initializer oracle row"));
                };
                let thread = if *attempt == "0" {
                    &mut owner
                } else {
                    &mut subsequent
                };
                let Err(Error::Initialization { kind, cause }) =
                    thread.invoke(CLASS, method, 0).await
                else {
                    return Err(Error::Boundary("expected Java initializer failure"));
                };
                assert_eq!(kind, *expected, "{row}");
                let actual_cause = match cause.as_deref() {
                    Some(Error::ArrayFault(name) | Error::TypeFault(name)) => *name,
                    Some(Error::Invalid(name)) => name.as_str(),
                    None => "-",
                    _ => {
                        return Err(Error::Boundary(
                            "initializer cause was wrapped more than once",
                        ));
                    }
                };
                assert_eq!(actual_cause, *expected_cause, "{row}");
            }
            Ok(())
        })
        .await?;
    Ok(())
}
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_initializer_engine_limit_remains_an_embedding_failure() -> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner = compiled.attach(
                heap,
                ristretto_vm::canonical::CompiledLimits {
                    native_slices: 8,
                    ..ristretto_vm::canonical::CompiledLimits::default()
                },
            )?;
            let mut other =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            assert!(matches!(
                owner.invoke(CLASS, "slow", 0).await,
                Err(Error::Limit(_))
            ));
            assert!(matches!(
                other.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            Ok(())
        })
        .await?;
    Ok(())
}
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_cancelled_initializer_remains_an_embedding_failure() -> Result<(), Error> {
    let program = program()?;
    let compiled = program.compile()?;
    program
        .with_heap(config(), 4 * 1024 * 1024, 1_000_000, async |heap| {
            let mut owner =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut other =
                compiled.attach(heap, ristretto_vm::canonical::CompiledLimits::default())?;
            let mut initializing = Box::pin(owner.invoke(CLASS, "slow", 0));
            for _ in 0..64 {
                assert!(
                    initializing
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                if heap.managed_bytes() > 0 {
                    break;
                }
            }
            assert!(heap.managed_bytes() > 0);
            drop(initializing);
            assert!(matches!(
                other.invoke(CLASS, "slow", 0).await,
                Err(Error::Boundary(
                    "class initialization was aborted by embedding"
                ))
            ));
            Ok(())
        })
        .await?;
    Ok(())
}
