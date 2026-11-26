use super::super::{Configuration, Limits, Report};
use super::*;
use ristretto_classfile::ClassFile;

fn linked_program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../../ristretto_vm/tests/fixtures/canonical/CanonicalLinkedWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../../ristretto_vm/tests/fixtures/canonical/CanonicalLinkedWorkload$Node.class"
        ))?,
    ])
}

fn configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}

fn check(report: &Report, value: i32) {
    assert_eq!(report.result, value.wrapping_mul(5).wrapping_add(76));
    assert_eq!(report.allocations, 2);
    assert_eq!(report.calls, 6);
    assert_eq!(report.released_objects, 2);
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    assert_eq!(report.catalog_bytes_after_drop, 0);
}

#[tokio::test]
async fn linked_methods_preserve_overloads_and_declaring_class() -> Result<(), Error> {
    let program = linked_program()?;
    for value in [i32::MIN, -17, 0, 1, 100, i32::MAX] {
        let report = program
            .run(
                "CanonicalLinkedWorkload",
                "run",
                value,
                configuration(),
                Limits::default(),
            )
            .await?;
        check(&report, value);
    }
    Ok(())
}

#[test]
fn linked_requests_reject_absent_wrong_kind_and_foreign_class_indices() -> Result<(), Error> {
    let program = linked_program()?;
    let mut observed = [0; 3];
    for (class, layout) in program.classes.iter().enumerate() {
        for index in layout.linked.classes.keys() {
            assert!(program.class_constant(class, *index).is_ok());
            assert!(program.field(class, *index).is_err());
            assert!(program.call_target(class, *index).is_err());
            observed[0] += 1;
        }
        for index in layout.linked.fields.keys() {
            assert!(program.field(class, *index).is_ok());
            assert!(program.class_constant(class, *index).is_err());
            assert!(program.call_target(class, *index).is_err());
            observed[1] += 1;
        }
        for index in layout.linked.calls.keys() {
            assert!(program.call_target(class, *index).is_ok());
            assert!(program.class_constant(class, *index).is_err());
            assert!(program.field(class, *index).is_err());
            observed[2] += 1;
        }
        assert!(program.class_constant(class, 0).is_err());
        assert!(program.field(class, 0).is_err());
        assert!(program.call_target(class, 0).is_err());
    }
    assert!(observed.iter().all(|count| *count > 0));
    assert!(program.class_constant(usize::MAX, 1).is_err());
    assert!(program.field(usize::MAX, 1).is_err());
    assert!(program.call_target(usize::MAX, 1).is_err());
    Ok(())
}
