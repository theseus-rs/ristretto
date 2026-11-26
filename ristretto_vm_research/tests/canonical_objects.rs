#![cfg(feature = "experimental-canonical-runtime")]
#![expect(
    clippy::panic_in_result_fn,
    reason = "test assertions report mismatches"
)]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Limits, Program};

#[tokio::test]
async fn construct_read_fault_and_refund() -> Result<(), Box<dyn std::error::Error>> {
    let file = ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalIntObject.class"
    ))?;
    let program = Program::from_classfiles([file])?;
    let config = Configuration {
        objects: 2,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let report = program
        .run("CanonicalIntObject", "read", 42, config, Limits::default())
        .await?;
    assert_eq!((report.result, report.allocations), (42, 1));
    assert_eq!(
        (
            report.payload_bytes_after_cleanup,
            report.catalog_bytes_after_drop
        ),
        (0, 0)
    );
    assert!(matches!(
        program
            .run("CanonicalIntObject", "fault", 0, config, Limits::default())
            .await,
        Err(Error::Invalid(_))
    ));
    assert!(matches!(
        program
            .run(
                "CanonicalIntObject",
                "read",
                42,
                config,
                Limits {
                    instructions: 5,
                    ..Limits::default()
                }
            )
            .await,
        Err(Error::Limit("instruction"))
    ));
    Ok(())
}

#[tokio::test]
async fn field_load_keeps_child_alive_across_unlink() -> Result<(), Box<dyn std::error::Error>> {
    let file = ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalObjectLink.class"
    ))?;
    let program = Program::from_classfiles([file])?;
    let config = Configuration {
        objects: 3,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let report = program
        .run(
            "CanonicalObjectLink",
            "read",
            -37,
            config,
            Limits::default(),
        )
        .await?;
    assert_eq!(report.result, -37);
    assert_eq!(
        (
            report.allocations,
            report.reference_reads,
            report.reference_writes
        ),
        (2, 1, 2)
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
