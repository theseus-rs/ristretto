//! Persistent statics on the shared canonical heap, compared with real Java.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::{ClassFile, FieldAccessFlags};
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Limits, Program};

const CLASS: &str = "CanonicalStaticFields";
const FILE: &[u8] =
    include_bytes!("../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticFields.class");
const NODE: &[u8] =
    include_bytes!("../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticFields$Node.class");
fn program() -> Result<Program, Error> {
    Program::from_classfiles([ClassFile::from_bytes(FILE)?, ClassFile::from_bytes(NODE)?])
}
fn config() -> Configuration {
    Configuration {
        objects: 5,
        roots: 12,
        mutators: 2,
        helpers: 2,
        edges_per_object: 1,
    }
}

#[tokio::test]
async fn persistent_static_entries_match_the_hotspot_sequence()
-> Result<(), Box<dyn std::error::Error>> {
    let result = program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            let mut allocated = 0;
            let mut released = 0;
            for row in
                include_str!("../../ristretto_vm/tests/fixtures/canonical/static-fields-oracle.txt")
                    .lines()
            {
                let columns: Vec<_> = row.split_whitespace().collect();
                let [method, argument, expected] = columns.as_slice() else {
                    return Err(Error::Boundary("oracle row"));
                };
                let parse = |text: &str| {
                    text.parse::<i32>()
                        .map_err(|e| Error::Invalid(e.to_string()))
                };
                let invocation = thread.invoke(CLASS, method, parse(argument)?).await?;
                assert_eq!(invocation.result, parse(expected)?, "{row}");
                allocated += invocation.allocations;
                released += invocation.released_objects;
            }
            assert_eq!(allocated, 20);
            Ok((allocated, released))
        })
        .await?;
    assert_eq!(
        result.value.0,
        result.value.1 + result.released_during_cleanup
    );
    Ok(())
}

#[tokio::test]
async fn two_contexts_share_statics_while_old_cycles_are_reclaimed() -> Result<(), Error> {
    program()?
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut writer = heap.attach(Limits::default())?;
            let mut reader = heap.attach(Limits::default())?;
            let first = reader.invoke(CLASS, "read", 0).await?;
            assert_eq!(
                (
                    first.result,
                    first.allocations,
                    first.static_storage_allocations
                ),
                (7, 1, 1)
            );
            assert_eq!(first.entry_initialization.allocations, 1);
            let mut released = 0;
            for value in 1..=16 {
                let write = writer.invoke(CLASS, "write", value).await?;
                assert_eq!(write.result, 2 * value + 7);
                assert_eq!(write.static_storage_allocations, 0);
                released += write.released_objects;
                assert_eq!(reader.invoke(CLASS, "read", 0).await?.result, 2 * value + 7);
                let churn = reader.invoke(CLASS, "churn", 8).await?;
                assert_eq!(churn.result, 28);
                released += churn.released_objects;
            }
            assert!(released >= 16 * 9 - config().objects);
            assert_eq!(writer.invoke(CLASS, "returned", 0).await?.result, 16);
            assert_eq!(reader.invoke(CLASS, "read", 0).await?.result, 23);
            Ok(())
        })
        .await?;
    Ok(())
}

#[tokio::test]
async fn distinct_heaps_have_distinct_static_storage() -> Result<(), Error> {
    let program = program()?;
    for value in [11, -13] {
        program
            .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
                let mut thread = heap.attach(Limits::default())?;
                assert_eq!(thread.invoke(CLASS, "read", 0).await?.result, 7);
                assert_eq!(
                    thread.invoke(CLASS, "write", value).await?.result,
                    2 * value + 7
                );
                Ok(())
            })
            .await?;
    }
    let report = program
        .run(
            CLASS,
            "write",
            11,
            Configuration {
                mutators: 1,
                helpers: 1,
                ..config()
            },
            Limits::default(),
        )
        .await?;
    assert_eq!(
        (
            report.result,
            report.allocations,
            report.internal_allocations
        ),
        (29, 2, 1)
    );
    assert_eq!(
        (
            report.payload_bytes_after_cleanup,
            report.catalog_bytes_after_drop
        ),
        (0, 0)
    );
    Ok(())
}

#[test]
fn volatile_fields_and_final_writes_outside_initializer_reject() -> Result<(), Error> {
    for flags in [FieldAccessFlags::VOLATILE, FieldAccessFlags::FINAL] {
        let mut file = ClassFile::from_bytes(FILE)?;
        let number = file
            .fields
            .iter_mut()
            .find(|field| {
                file.constant_pool
                    .try_get_utf8(field.name_index)
                    .is_ok_and(|name| name == "number")
            })
            .ok_or(Error::Boundary("fixture field"))?;
        number.access_flags |= flags;
        assert!(matches!(
            Program::from_classfiles([file, ClassFile::from_bytes(NODE)?]),
            Err(Error::Unsupported(_))
        ));
    }
    Ok(())
}

#[tokio::test]
async fn static_arrays_retain_their_actual_element_type_across_entries() -> Result<(), Error> {
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticArrays.class"
    ))?])?;
    let configuration = Configuration {
        objects: 16,
        roots: 32,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    };
    let heap = program
        .with_heap(configuration, 8 * 1024 * 1024, 100_000, async |heap| {
            let mut thread = heap.attach(Limits::default())?;
            for value in [37, -11] {
                assert_eq!(
                    thread
                        .invoke("CanonicalStaticArrays", "store", value)
                        .await?
                        .result,
                    value
                );
                assert_eq!(
                    thread
                        .invoke("CanonicalStaticArrays", "unlink", 0)
                        .await?
                        .result,
                    value
                );
            }
            Ok(())
        })
        .await?;
    assert!(heap.cleanup_polls > 0);
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_persistent_static_entries_match_the_hotspot_sequence()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    let result = program
        .with_heap(config(), 1024 * 1024, 1_000_000, async |heap| {
            let mut thread = compiled.attach(
                heap,
                ristretto_vm_research::canonical::CompiledLimits::default(),
            )?;
            let mut allocated = 0;
            let mut released = 0;
            for row in
                include_str!("../../ristretto_vm/tests/fixtures/canonical/static-fields-oracle.txt")
                    .lines()
            {
                let columns: Vec<_> = row.split_whitespace().collect();
                let [method, argument, expected] = columns.as_slice() else {
                    return Err(Error::Boundary("oracle row"));
                };
                let parse = |text: &str| {
                    text.parse::<i32>()
                        .map_err(|e| Error::Invalid(e.to_string()))
                };
                let invocation = thread.invoke(CLASS, method, parse(argument)?).await?;
                assert_eq!(invocation.result, parse(expected)?, "{row}");
                allocated += invocation.allocations;
                released += invocation.released_objects;
            }
            assert_eq!(allocated, 20);
            Ok((allocated, released))
        })
        .await?;
    assert_eq!(
        result.value.0,
        result.value.1 + result.released_during_cleanup
    );
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_static_arrays_retain_their_actual_element_type_across_entries() -> Result<(), Error>
{
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticArrays.class"
    ))?])?;
    let configuration = Configuration {
        objects: 16,
        roots: 32,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    };
    let compiled = program.compile()?;
    let heap = program
        .with_heap(configuration, 8 * 1024 * 1024, 100_000, async |heap| {
            let mut thread = compiled.attach(
                heap,
                ristretto_vm_research::canonical::CompiledLimits::default(),
            )?;
            for value in [37, -11] {
                assert_eq!(
                    thread
                        .invoke("CanonicalStaticArrays", "store", value)
                        .await?
                        .result,
                    value
                );
                assert_eq!(
                    thread
                        .invoke("CanonicalStaticArrays", "unlink", 0)
                        .await?
                        .result,
                    value
                );
            }
            Ok(())
        })
        .await?;
    assert!(heap.cleanup_polls > 0);
    Ok(())
}

#[path = "support/canonical_statics_additional.rs"]
mod additional_regressions;
