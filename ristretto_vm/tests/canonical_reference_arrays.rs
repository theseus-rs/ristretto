#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};

fn program() -> Result<Program, Box<dyn std::error::Error>> {
    let classes: [&[u8]; 4] = [
        include_bytes!("fixtures/canonical/CanonicalReferenceArrayWorkload.class"),
        include_bytes!("fixtures/canonical/CanonicalReferenceArrayWorkload$Node.class"),
        include_bytes!("fixtures/canonical/CanonicalReferenceArrayWorkload$Other.class"),
        include_bytes!("fixtures/canonical/CanonicalReferenceArrayWorkload$Holder.class"),
    ];
    Ok(Program::from_classfiles(
        classes
            .map(ClassFile::from_bytes)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?,
    )?)
}
fn configuration(length: i32, nodes: bool) -> Configuration {
    Configuration {
        objects: if length == 2048 && nodes {
            4096
        } else if length == 2048 {
            512
        } else {
            128
        },
        roots: 64,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}

#[tokio::test]
async fn interpreted_reference_arrays_match_hotspot_including_2048()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let mut rows = 0;
    for line in include_str!("fixtures/canonical/reference-array-oracle.tsv").lines() {
        let mut fields = line.split('\t');
        let method = fields.next().ok_or("method")?;
        let argument = fields.next().ok_or("argument")?.parse::<i32>()?;
        let expected = fields.next().ok_or("expected")?;
        let result = program
            .run(
                "CanonicalReferenceArrayWorkload",
                method,
                argument,
                configuration(argument, method == "nodes"),
                Limits {
                    cleanup_polls: 5_000_000,
                    ..Limits::default()
                },
            )
            .await;
        println!("{method}({argument}) -> {result:?}");
        if let Ok(expected) = expected.parse::<i32>() {
            let report = result?;
            assert_eq!(report.result, expected);
            assert_eq!(report.allocations, report.released_objects);
            assert_eq!(report.payload_bytes_after_cleanup, 0);
            assert_eq!(report.catalog_bytes_after_drop, 0);
            if argument == 2048 {
                assert_eq!(report.internal_allocations, 292);
            }
            assert!(report.reference_array_allocations >= 1);
        } else {
            let error = result.err().ok_or("missing Java fault")?;
            assert!(
                error
                    .to_string()
                    .contains(expected.rsplit('.').next().ok_or("fault name")?),
                "{error} != {expected}"
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 34);
    Ok(())
}

#[tokio::test]
async fn reference_array_resource_rejections_are_explicit() -> Result<(), Box<dyn std::error::Error>>
{
    let program = program()?;
    for (objects, roots, edges, bytes, argument) in [
        (292, 64, 8, 8_000_000, 2048),
        (512, 4, 8, 8_000_000, 2048),
        (512, 64, 4, 8_000_000, 2048),
        (512, 64, 8, 0, 2048),
        (512, 64, 8, usize::MAX, i32::MAX),
    ] {
        let result = program
            .run(
                "CanonicalReferenceArrayWorkload",
                "failNegative",
                argument,
                Configuration {
                    objects,
                    roots,
                    mutators: 1,
                    helpers: 1,
                    edges_per_object: edges,
                },
                Limits {
                    array_bytes: bytes,
                    ..Limits::default()
                },
            )
            .await;
        assert!(matches!(result, Err(Error::Limit(_))), "{result:?}");
    }
    Ok(())
}

#[test]
fn multidimensional_reference_layouts_still_reject_before_execution()
-> Result<(), Box<dyn std::error::Error>> {
    for bytes in [
        include_bytes!("fixtures/canonical_rejected/CanonicalRejectedReferenceComponent.class")
            .as_slice(),
        include_bytes!("fixtures/canonical_rejected/CanonicalRejectedReferenceDimensions.class")
            .as_slice(),
    ] {
        assert!(matches!(
            Program::from_classfiles([ClassFile::from_bytes(bytes)?]),
            Err(Error::Unsupported(_))
        ));
    }
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_reference_arrays_match_hotspot_including_2048()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    let mut rows = 0;
    for line in include_str!("fixtures/canonical/reference-array-oracle.tsv").lines() {
        let mut fields = line.split('\t');
        let method = fields.next().ok_or("method")?;
        let argument = fields.next().ok_or("argument")?.parse::<i32>()?;
        let expected = fields.next().ok_or("expected")?;
        let result = compiled
            .run(
                "CanonicalReferenceArrayWorkload",
                method,
                argument,
                configuration(argument, method == "nodes"),
                ristretto_vm::canonical::CompiledLimits {
                    cleanup_polls: 5_000_000,
                    ..ristretto_vm::canonical::CompiledLimits::default()
                },
            )
            .await;
        println!("{method}({argument}) -> {result:?}");
        if let Ok(expected) = expected.parse::<i32>() {
            let report = result?;
            assert_eq!(report.result, expected);
            assert_eq!(report.compiled_entries, report.calls);
            assert!(report.compiled_entries > 0);
            assert!(!report.instruction_count_available);
            assert_eq!(report.allocations, report.released_objects);
            assert_eq!(report.payload_bytes_after_cleanup, 0);
            assert_eq!(report.catalog_bytes_after_drop, 0);
            if argument == 2048 {
                assert_eq!(report.internal_allocations, 292);
            }
            assert!(report.reference_array_allocations >= 1);
        } else {
            let error = result.err().ok_or("missing Java fault")?;
            assert!(
                error
                    .to_string()
                    .contains(expected.rsplit('.').next().ok_or("fault name")?),
                "{error} != {expected}"
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 34);
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_reference_array_quota_failure_does_not_pin_frame_owners()
-> Result<(), Box<dyn std::error::Error>> {
    use ristretto_vm::canonical::CompiledLimits;
    let program = program()?;
    let compiled = program.compile()?;
    assert!(matches!(
        compiled
            .run(
                "CanonicalReferenceArrayWorkload",
                "failNegative",
                2048,
                Configuration {
                    objects: 292,
                    ..configuration(2048, false)
                },
                CompiledLimits::default()
            )
            .await,
        Err(Error::Limit(_))
    ));
    let result = compiled
        .run(
            "CanonicalReferenceArrayWorkload",
            "returned",
            9,
            configuration(9, false),
            CompiledLimits::default(),
        )
        .await?;
    assert_eq!(result.payload_bytes_after_cleanup, 0);
    assert_eq!(result.catalog_bytes_after_drop, 0);
    Ok(())
}

#[path = "support/canonical_reference_arrays_additional.rs"]
mod additional_regressions;
