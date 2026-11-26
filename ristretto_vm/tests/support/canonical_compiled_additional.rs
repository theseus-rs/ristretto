#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "integration test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{CompiledLimits, Error, Limits, Program};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?)
}
fn configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn actual_compiled_graph_uses_canonical_roots_and_reclaims_cycles() -> Result {
    let program = program()?;
    let compiled = program.compile()?;
    for rounds in [0, 1, 3, 8] {
        let report = compiled
            .run(
                "CanonicalGraphWorkload",
                "run",
                rounds,
                configuration(),
                CompiledLimits::default(),
            )
            .await?;
        assert_eq!(report.result, rounds * rounds);
        let rounds = usize::try_from(rounds)?;
        assert_eq!(report.allocations, 2 * rounds);
        assert_eq!(report.reference_reads, 3 * rounds);
        assert_eq!(report.reference_writes, 4 * rounds);
        assert_eq!(report.compiled_entries, 1 + 3 * rounds);
        assert_eq!(report.compiled_backedges, u64::try_from(rounds)?);
        assert_eq!(report.released_objects, 2 * rounds);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
        assert!(!report.instruction_count_available);
        assert_eq!(report.instructions, 0);
        println!("rounds={rounds} {report:?}");
    }
    Ok(())
}

#[tokio::test]
async fn compiled_full_root_quota_reclaims_frame_and_request_roots() -> Result {
    let program = program()?;
    let compiled = program.compile()?;
    let mut config = configuration();
    config.roots = 2;
    let limits = CompiledLimits {
        request_polls: 2_000,
        ..CompiledLimits::default()
    };
    assert!(matches!(
        compiled
            .run("CanonicalGraphWorkload", "run", 1, config, limits)
            .await,
        Err(Error::Limit("request poll"))
    ));
    Ok(())
}

#[tokio::test]
async fn native_slice_and_frame_limits_reclaim_all_owners() -> Result {
    let program = program()?;
    let compiled = program.compile()?;
    let limits = CompiledLimits {
        native_slices: 1,
        ..CompiledLimits::default()
    };
    assert!(matches!(
        compiled
            .run("CanonicalGraphWorkload", "run", 1, configuration(), limits)
            .await,
        Err(Error::Limit("native slice"))
    ));
    let limits = CompiledLimits {
        frames: 1,
        ..CompiledLimits::default()
    };
    assert!(matches!(
        compiled
            .run("CanonicalGraphWorkload", "run", 1, configuration(), limits)
            .await,
        Err(Error::Limit("frame"))
    ));
    Ok(())
}

#[tokio::test]
async fn reference_return_handoff_preserves_identity_after_unlink() -> Result {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalReturnWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../fixtures/canonical/CanonicalReturnWorkload$Node.class"
        ))?,
    ])?;
    let compiled = program.compile()?;
    for value in [7, -13] {
        let interpreted = program
            .run(
                "CanonicalReturnWorkload",
                "run",
                value,
                configuration(),
                Limits::default(),
            )
            .await?;
        let report = compiled
            .run(
                "CanonicalReturnWorkload",
                "run",
                value,
                configuration(),
                CompiledLimits::default(),
            )
            .await?;
        assert_eq!(report.result, value);
        assert_eq!(report.result, interpreted.result);
        assert_eq!(report.compiled_entries, 3);
        assert_eq!(report.allocations, 1);
        assert_eq!(report.released_objects, 1);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}
