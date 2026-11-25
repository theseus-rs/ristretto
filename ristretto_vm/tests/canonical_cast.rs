#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program, Report};

fn program() -> Result<Program, Box<dyn std::error::Error>> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalCastWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalCastWorkload$Node.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalCastWorkload$Other.class"
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
fn check(result: Result<Report, Error>, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(expected) = expected.parse::<i32>() {
        let report = result?;
        assert_eq!(report.result, expected);
        assert_eq!(report.allocations, report.released_objects);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
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
#[tokio::test]
async fn interpreted_casts_match_hotspot_and_preserve_array_components()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let mut rows = 0;
    for line in include_str!("fixtures/canonical/cast-oracle.tsv").lines() {
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
                Limits::default(),
            )
            .await;
        println!("interpreted {method}({argument}) -> {result:?}");
        check(result, expected)?;
        rows += 1;
    }
    assert_eq!(rows, 85);
    Ok(())
}
#[test]
fn unsupported_cast_targets_and_instanceof_reject_before_execution()
-> Result<(), Box<dyn std::error::Error>> {
    for bytes in [
        include_bytes!("fixtures/canonical_rejected/CanonicalCastUnsupportedDimensions.class")
            .as_slice(),
        include_bytes!("fixtures/canonical_rejected/CanonicalCastUnsupportedInterface.class")
            .as_slice(),
        include_bytes!("fixtures/canonical_rejected/CanonicalCastUnsupportedMissing.class")
            .as_slice(),
        include_bytes!("fixtures/canonical_rejected/CanonicalCastUnsupportedInstanceOf.class")
            .as_slice(),
    ] {
        assert!(matches!(
            Program::from_classfiles([ClassFile::from_bytes(bytes)?]),
            Err(Error::Unsupported(_))
        ));
    }
    Ok(())
}

fn access_program(
    public: bool,
    same_package: bool,
    method: &str,
) -> Result<Result<Program, Error>, Box<dyn std::error::Error>> {
    let mut target = ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical_access/casttarget/CanonicalCastAccessTarget.class"
    ))?;
    if !public {
        target
            .access_flags
            .remove(ristretto_classfile::ClassAccessFlags::PUBLIC);
    }
    let mut caller = ClassFile::from_bytes(if same_package {
        include_bytes!("fixtures/canonical_access/casttarget/CanonicalCastSamePackage.class")
            .as_slice()
    } else {
        include_bytes!("fixtures/canonical_access/castoutside/CanonicalCastOtherPackage.class")
            .as_slice()
    })?;
    let mut retained = Vec::new();
    for definition in std::mem::take(&mut caller.methods) {
        let name = caller.constant_pool.try_get_utf8(definition.name_index)?;
        if name == "<init>" || name == method {
            retained.push(definition);
        }
    }
    caller.methods = retained;
    Ok(Program::from_classfiles([caller, target]))
}

#[tokio::test]
async fn class_constant_access_is_checked_for_casts_and_allocations()
-> Result<(), Box<dyn std::error::Error>> {
    for method in ["classCast", "arrayCast", "arrayNew", "objectNew"] {
        assert!(
            matches!(access_program(false, false, method)?, Err(Error::Unsupported(message)) if message.contains("inaccessible"))
        );
        for (public, same_package) in [(true, false), (false, true)] {
            let program = access_program(public, same_package, method)??;
            let class = if same_package {
                "casttarget/CanonicalCastSamePackage"
            } else {
                "castoutside/CanonicalCastOtherPackage"
            };
            let expected = if !same_package && matches!(method, "classCast" | "arrayCast") {
                "java.lang.ClassCastException"
            } else {
                "3"
            };
            check(
                program
                    .run(class, method, 3, configuration(), Limits::default())
                    .await,
                expected,
            )?;
        }
    }
    Ok(())
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn native_casts_match_hotspot_and_preserve_reference_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    let mut rows = 0;
    for line in include_str!("fixtures/canonical/cast-oracle.tsv").lines() {
        let mut fields = line.split('\t');
        let method = fields.next().ok_or("method")?;
        let argument = fields.next().ok_or("argument")?.parse()?;
        let expected = fields.next().ok_or("expected")?;
        let result = compiled
            .run(
                "CanonicalCastWorkload",
                method,
                argument,
                configuration(),
                ristretto_vm::canonical::CompiledLimits::default(),
            )
            .await;
        println!("native {method}({argument}) -> {result:?}");
        if let Ok(report) = &result {
            assert_eq!(report.compiled_entries, report.calls);
            assert!(report.compiled_entries > 0);
        }
        check(result, expected)?;
        rows += 1;
    }
    assert_eq!(rows, 85);
    Ok(())
}
