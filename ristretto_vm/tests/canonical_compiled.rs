//! Native request-only execution on the same admitted Java objects tier.
#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{CompiledLimits, Error, Limits, Program};
fn program() -> Result<Program, Box<dyn std::error::Error>> {
    Ok(Program::from_classfiles([ClassFile::from_bytes(
        include_bytes!("fixtures/canonical/CanonicalCompiledCalls.class"),
    )?])?)
}
fn configuration() -> Configuration {
    Configuration {
        objects: 2,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}
#[tokio::test]
async fn javac_calls_backedges_and_wide_locals_match_hotspot_and_interpreter()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    for line in include_str!("fixtures/canonical/compiled-calls-oracle.tsv").lines() {
        let mut fields = line.split('\t');
        let method = fields.next().ok_or("method")?;
        let argument = fields.next().ok_or("argument")?.parse::<i32>()?;
        let expected = fields.next().ok_or("result")?.parse::<i32>()?;
        assert!(fields.next().is_none());
        let interpreted = program
            .run(
                "CanonicalCompiledCalls",
                method,
                argument,
                configuration(),
                Limits {
                    frames: 128,
                    ..Limits::default()
                },
            )
            .await?;
        assert_eq!(interpreted.result, expected);
        let report = compiled
            .run(
                "CanonicalCompiledCalls",
                method,
                argument,
                configuration(),
                CompiledLimits {
                    frames: 128,
                    ..CompiledLimits::default()
                },
            )
            .await?;
        assert_eq!(report.result, expected);
        assert!(!report.instruction_count_available);
        assert_eq!(report.instructions, 0);
        assert_eq!(report.compiled_entries, report.calls);
        assert!(report.compiled_entries > 0);
        assert_eq!(
            report.native_slices,
            report.compiled_entries + report.compiled_resumes
        );
        if method == "run" && argument > 0 {
            assert_eq!(report.compiled_backedges, u64::try_from(argument)?);
        }
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}
#[tokio::test]
async fn frame_and_native_slice_limits_do_not_leave_program_running()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    for (method, limits, expected) in [
        (
            "depth",
            CompiledLimits {
                frames: 2,
                ..CompiledLimits::default()
            },
            "frame",
        ),
        (
            "run",
            CompiledLimits {
                native_slices: 1,
                ..CompiledLimits::default()
            },
            "native slice",
        ),
    ] {
        assert!(
            matches!(compiled.run("CanonicalCompiledCalls",method,9,configuration(),limits).await,Err(Error::Limit(kind)) if kind==expected)
        );
        assert_eq!(
            compiled
                .run(
                    "CanonicalCompiledCalls",
                    "run",
                    1,
                    configuration(),
                    CompiledLimits::default()
                )
                .await?
                .result,
            2
        );
    }
    Ok(())
}
#[tokio::test]
async fn cancelling_parked_call_releases_exclusive_run_lease()
-> Result<(), Box<dyn std::error::Error>> {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let program = program()?;
    let compiled = program.compile()?;
    let mut invocation = Box::pin(compiled.run(
        "CanonicalCompiledCalls",
        "run",
        64,
        configuration(),
        CompiledLimits::default(),
    ));
    let mut context = Context::from_waker(Waker::noop());
    assert!(matches!(
        invocation.as_mut().poll(&mut context),
        Poll::Pending
    ));
    assert!(matches!(
        compiled
            .run(
                "CanonicalCompiledCalls",
                "run",
                0,
                configuration(),
                CompiledLimits::default()
            )
            .await,
        Err(Error::Boundary("compiled program already running"))
    ));
    drop(invocation);
    assert_eq!(
        compiled
            .run(
                "CanonicalCompiledCalls",
                "run",
                1,
                configuration(),
                CompiledLimits::default()
            )
            .await?
            .result,
        2
    );
    Ok(())
}

#[tokio::test]
async fn native_java_depth_uses_bounded_heap_frames_instead_of_recursive_host_polls()
-> Result<(), Box<dyn std::error::Error>> {
    let program = program()?;
    let compiled = program.compile()?;
    let limits = CompiledLimits {
        frames: 1025,
        native_slices: 3000,
        ..CompiledLimits::default()
    };
    let report = compiled
        .run(
            "CanonicalCompiledCalls",
            "depth",
            1024,
            configuration(),
            limits,
        )
        .await?;
    assert_eq!(report.result, 1024);
    assert_eq!(report.compiled_entries, 1025);
    assert_eq!(report.compiled_resumes, 1024);
    assert_eq!(report.compiled_backedges, 0);
    assert!(matches!(
        compiled
            .run(
                "CanonicalCompiledCalls",
                "depth",
                64,
                configuration(),
                CompiledLimits::default()
            )
            .await,
        Err(Error::Limit("frame"))
    ));
    Ok(())
}

#[test]
fn compiler_intersection_rejects_heap_operations_before_any_native_entry()
-> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    assert!(matches!(
        program.compile(),
        Err(Error::Jit(ristretto_jit::Error::UnsupportedMethod(_)))
    ));
    Ok(())
}
