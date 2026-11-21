#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program, Report};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalPrimitiveArrayWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalPrimitiveArrayWorkload$Holder.class"
        ))?,
    ])?)
}
fn numeric_program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalNumericEdgeWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalNumericEdgeWorkload$WideHolder.class"
        ))?,
    ])?)
}
fn cases() -> Result<[(Program, &'static str, &'static str, usize); 3]> {
    Ok([
        (
            program()?,
            "CanonicalPrimitiveArrayWorkload",
            include_str!("fixtures/canonical/primitive-array-oracle.tsv"),
            147,
        ),
        (
            Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
                "fixtures/canonical_bytecode/CanonicalNarrowReturnWorkload.class"
            ))?])?,
            "CanonicalNarrowReturnWorkload",
            include_str!("fixtures/canonical_bytecode/narrow-return-oracle.tsv"),
            40,
        ),
        (
            numeric_program()?,
            "CanonicalNumericEdgeWorkload",
            include_str!("fixtures/canonical/numeric-edge-oracle.tsv"),
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

#[tokio::test]
async fn interpreted_primitive_arrays_match_hotspot_oracle() -> Result {
    for (program, class, oracle, expected_count) in cases()? {
        let mut count = 0;
        for line in oracle.lines().filter(|line| !line.starts_with('#')) {
            let mut columns = line.split('\t');
            let method = columns.next().ok_or("missing method")?;
            let argument = columns.next().ok_or("missing argument")?.parse()?;
            let expected = columns.next().ok_or("missing result")?;
            let result = program
                .run(class, method, argument, configuration(), Limits::default())
                .await;
            println!("interpreted {method}({argument}) -> {result:?}");
            check(result, expected)?;
            count += 1;
        }
        assert_eq!(count, expected_count);
    }
    Ok(())
}

#[tokio::test]
async fn array_backing_limit_rejects_before_allocation() -> Result {
    let program = program()?;
    let limits = Limits {
        array_bytes: 7,
        ..Limits::default()
    };
    assert!(matches!(
        program
            .run(
                "CanonicalPrimitiveArrayWorkload",
                "longs",
                0,
                configuration(),
                limits
            )
            .await,
        Err(Error::Limit("array backing bytes"))
    ));
    Ok(())
}

#[test]
fn unsupported_wide_stack_operations_reject_before_execution() -> Result {
    for bytes in [
        include_bytes!("fixtures/canonical_rejected/CanonicalUnsupportedPop2.class").as_slice(),
        include_bytes!("fixtures/canonical_rejected/CanonicalUnsupportedDup2.class").as_slice(),
    ] {
        let class = ClassFile::from_bytes(bytes)?;
        assert!(matches!(
            Program::from_classfiles([class]),
            Err(Error::Unsupported(_))
        ));
    }
    Ok(())
}
