//! Cross-tier scalar calls checked against Java 17 classfiles and `HotSpot` results.
#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{CompiledLimits, Limits, Program};

#[tokio::test]
async fn scalar_calls_match_hotspot_through_compiled_return_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    let narrow = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical_bytecode/CanonicalNarrowReturnWorkload.class"
    ))?])?;
    let wide = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalWideCalls.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalWideScalar.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 1,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    for (program, class, rows, expected_count) in [
        (
            narrow,
            "CanonicalNarrowReturnWorkload",
            include_str!("fixtures/canonical_bytecode/narrow-return-oracle.tsv"),
            40,
        ),
        (
            wide,
            "CanonicalWideCalls",
            include_str!("fixtures/canonical/wide-calls-oracle.tsv"),
            10,
        ),
    ] {
        let compiled = program.compile()?;
        let mut checked = 0;
        for row in rows.lines().filter(|row| !row.starts_with('#')) {
            let mut columns = row.split('\t');
            let method = columns.next().ok_or("missing method")?;
            let argument = columns.next().ok_or("missing argument")?.parse()?;
            let expected: i32 = columns.next().ok_or("missing result")?.parse()?;
            let interpreted = program
                .run(class, method, argument, configuration, Limits::default())
                .await?;
            let native = compiled
                .run(
                    class,
                    method,
                    argument,
                    configuration,
                    CompiledLimits::default(),
                )
                .await?;
            assert_eq!(interpreted.result, expected, "{class}.{method}({argument})");
            assert_eq!(native.result, expected, "{class}.{method}({argument})");
            assert!(native.compiled_entries > 1);
            assert_eq!(native.payload_bytes_after_cleanup, 0);
            assert_eq!(native.catalog_bytes_after_drop, 0);
            checked += 1;
        }
        assert_eq!(checked, expected_count);
    }
    Ok(())
}
