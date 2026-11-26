#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "integration test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Program, Report};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload$Holder.class"
        ))?,
    ])?)
}
fn numeric_program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalNumericEdgeWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalNumericEdgeWorkload$WideHolder.class"
        ))?,
    ])?)
}
fn cases() -> Result<[(Program, &'static str, &'static str, usize); 3]> {
    Ok([
        (
            program()?,
            "CanonicalPrimitiveArrayWorkload",
            include_str!(
                "../../../ristretto_vm/tests/fixtures/canonical/primitive-array-oracle.tsv"
            ),
            147,
        ),
        (
            Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
                "../../../ristretto_vm/tests/fixtures/canonical_bytecode/CanonicalNarrowReturnWorkload.class"
            ))?])?,
            "CanonicalNarrowReturnWorkload",
            include_str!(
                "../../../ristretto_vm/tests/fixtures/canonical_bytecode/narrow-return-oracle.tsv"
            ),
            40,
        ),
        (
            numeric_program()?,
            "CanonicalNumericEdgeWorkload",
            include_str!("../../../ristretto_vm/tests/fixtures/canonical/numeric-edge-oracle.tsv"),
            114,
        ),
    ])
}
fn configuration() -> Configuration {
    Configuration {
        objects: 16,
        roots: 48,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}
fn check(result: std::result::Result<Report, Error>, expected: &str) -> Result {
    if let Ok(value) = expected.parse::<i32>() {
        let report = result?;
        assert_eq!(report.result, value);
        assert_eq!(report.allocations, report.released_objects);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    } else {
        let error = result.err().ok_or("expected Java array fault")?;
        let class = expected.rsplit('.').next().ok_or("missing fault class")?;
        assert!(error.to_string().contains(class), "{error} != {expected}");
    }
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_primitive_arrays_match_hotspot_oracle() -> Result {
    use ristretto_vm_research::canonical::CompiledLimits;
    for (program, class, oracle, expected_count) in cases()? {
        let compiled = program.compile()?;
        let mut count = 0;
        for line in oracle.lines().filter(|line| !line.starts_with('#')) {
            let mut columns = line.split('\t');
            let method = columns.next().ok_or("missing method")?;
            let argument = columns.next().ok_or("missing argument")?.parse()?;
            let expected = columns.next().ok_or("missing result")?;
            let result = compiled
                .run(
                    class,
                    method,
                    argument,
                    configuration(),
                    CompiledLimits::default(),
                )
                .await;
            println!("compiled {method}({argument}) -> {result:?}");
            check(result, expected)?;
            count += 1;
        }
        assert_eq!(count, expected_count);
    }
    Ok(())
}
