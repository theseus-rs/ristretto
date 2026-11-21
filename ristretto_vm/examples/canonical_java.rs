//! Execute a real Java classfile through the restricted canonical VM tier.
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Limits, Program};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    };
    let report = program
        .run(
            "CanonicalGraphWorkload",
            "run",
            8,
            configuration,
            Limits::default(),
        )
        .await?;
    if report.result != 64
        || report.allocations != 16
        || report.payload_bytes_after_cleanup != 0
        || report.catalog_bytes_after_drop != 0
    {
        return Err("canonical Java fixture invariant".into());
    }
    println!("{report:?}");
    Ok(())
}
