#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Program, Report};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalCastWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalCastWorkload$Node.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalCastWorkload$Other.class"
        ))?,
    ])?)
}
fn configuration() -> Configuration {
    Configuration {
        objects: 16,
        roots: 32,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}
fn check(result: std::result::Result<Report, Error>, expected: &str, compiled: bool) -> Result {
    if let Ok(expected) = expected.parse::<i32>() {
        let report = result?;
        assert_eq!(report.result, expected);
        assert_eq!(report.allocations, report.released_objects);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
        if compiled {
            assert!(report.compiled_entries > 0);
        }
    } else {
        let error = result.err().ok_or("missing Java fault")?;
        assert!(
            error
                .to_string()
                .contains(expected.rsplit('.').next().ok_or("fault name")?),
            "{error} != {expected}"
        );
    }
    Ok(())
}
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_casts_match_hotspot_and_preserve_array_components() -> Result {
    let admitted = program()?;
    let program = admitted.compile()?;
    let mut rows = 0;
    for line in include_str!("../fixtures/canonical/cast-oracle.tsv").lines() {
        let mut fields = line.split('\t');
        let method = fields.next().ok_or("method")?;
        let argument = fields.next().ok_or("argument")?.parse()?;
        let expected = fields.next().ok_or("expected")?;
        let result = program
            .run(
                "CanonicalCastWorkload",
                method,
                argument,
                configuration(),
                ristretto_vm::canonical::CompiledLimits::default(),
            )
            .await;
        println!("compiled {method}({argument}) -> {result:?}");
        check(result, expected, true)?;
        rows += 1;
    }
    assert_eq!(rows, 85);
    Ok(())
}
