#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::Program;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    let classes: [&[u8]; 4] = [
        include_bytes!("../fixtures/canonical/CanonicalReferenceArrayWorkload.class"),
        include_bytes!("../fixtures/canonical/CanonicalReferenceArrayWorkload$Node.class"),
        include_bytes!("../fixtures/canonical/CanonicalReferenceArrayWorkload$Other.class"),
        include_bytes!("../fixtures/canonical/CanonicalReferenceArrayWorkload$Holder.class"),
    ];
    Ok(Program::from_classfiles(
        classes
            .map(ClassFile::from_bytes)
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?,
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

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_reference_arrays_match_hotspot_including_2048() -> Result {
    let admitted = program()?;
    let program = admitted.compile()?;
    let mut rows = 0;
    for line in include_str!("../fixtures/canonical/reference-array-oracle.tsv").lines() {
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
            assert!(report.compiled_entries > 0);
            if argument > 0 && matches!(method, "nodes" | "mixed") {
                assert!(report.compiled_backedges > 0);
            }
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
