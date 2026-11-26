//! Execute a real Java classfile through the restricted canonical VM tier.
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Limits, Program};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    let compiled = match arguments.as_slice() {
        [] => false,
        [mode] if mode == "--compiled" => true,
        _ => return Err("usage: canonical_java [--compiled]".into()),
    };
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    };
    let report = if compiled {
        #[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
        {
            program
                .compile()?
                .run(
                    "CanonicalGraphWorkload",
                    "run",
                    8,
                    configuration,
                    ristretto_vm_research::canonical::CompiledLimits::default(),
                )
                .await?
        }
        #[cfg(not(all(feature = "experimental-canonical-jit", not(target_family = "wasm"))))]
        {
            return Err("--compiled requires native experimental-canonical-jit".into());
        }
    } else {
        program
            .run(
                "CanonicalGraphWorkload",
                "run",
                8,
                configuration,
                Limits::default(),
            )
            .await?
    };
    if report.result != 64
        || report.allocations != 16
        || report.payload_bytes_after_cleanup != 0
        || report.catalog_bytes_after_drop != 0
        || (compiled && (report.compiled_entries != 25 || report.compiled_backedges != 8))
    {
        return Err("canonical Java fixture invariant".into());
    }
    println!("{report:?}");
    Ok(())
}
