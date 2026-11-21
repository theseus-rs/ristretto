//! Native request-only execution on the same admitted Java objects tier.
#![cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#![expect(clippy::panic_in_result_fn, reason = "test assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{CompiledLimits, Error, Limits, Program};
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
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
async fn javac_calls_backedges_and_wide_locals_match_hotspot_and_interpreter() -> Result {
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
async fn frame_and_native_slice_limits_do_not_leave_program_running() -> Result {
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
async fn cancelling_parked_call_releases_exclusive_run_lease() -> Result {
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
async fn native_java_depth_uses_bounded_heap_frames_instead_of_recursive_host_polls() -> Result {
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

#[tokio::test]
async fn primitive_array_cast_mismatch_returns_a_java_fault() -> Result {
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical_rejected/CanonicalPrimitiveArrayTypeFailure.class"
    ))?])?;
    let compiled = program.compile()?;
    assert!(matches!(
        compiled
            .run(
                "CanonicalPrimitiveArrayTypeFailure",
                "run",
                0,
                configuration(),
                CompiledLimits::default()
            )
            .await,
        Err(Error::TypeFault("ClassCastException"))
    ));
    Ok(())
}

fn graph_program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?)
}
fn graph_configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}
#[tokio::test]
async fn native_graph_allocations_calls_fields_cycles_and_cleanup_match_interpreter() -> Result {
    let program = graph_program()?;
    let compiled = program.compile()?;
    for rounds in [0, 1, 8, 64] {
        let interpreted = program
            .run(
                "CanonicalGraphWorkload",
                "run",
                rounds,
                graph_configuration(),
                Limits::default(),
            )
            .await?;
        let report = compiled
            .run(
                "CanonicalGraphWorkload",
                "run",
                rounds,
                graph_configuration(),
                CompiledLimits::default(),
            )
            .await?;
        assert_eq!(report.result, rounds * rounds);
        assert_eq!(report.result, interpreted.result);
        assert_eq!(report.allocations, usize::try_from(rounds)? * 2);
        assert_eq!(report.allocations, interpreted.allocations);
        assert_eq!(report.reference_reads, interpreted.reference_reads);
        assert_eq!(report.reference_writes, interpreted.reference_writes);
        assert_eq!(report.calls, interpreted.calls);
        assert_eq!(report.compiled_entries, report.calls);
        assert_eq!(report.compiled_backedges, u64::try_from(rounds)?);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}
#[tokio::test]
async fn native_root_pressure_errors_release_the_invocation() -> Result {
    let program = graph_program()?;
    let compiled = program.compile()?;
    let config = Configuration {
        roots: 2,
        ..graph_configuration()
    };
    let limits = CompiledLimits {
        request_polls: 2000,
        ..CompiledLimits::default()
    };
    assert!(matches!(
        compiled
            .run("CanonicalGraphWorkload", "run", 8, config, limits)
            .await,
        Err(Error::Limit("request poll"))
    ));
    assert_eq!(
        compiled
            .run(
                "CanonicalGraphWorkload",
                "run",
                8,
                graph_configuration(),
                CompiledLimits::default()
            )
            .await?
            .result,
        64
    );
    Ok(())
}

#[tokio::test]
async fn native_entry_drives_fieldless_scalar_class_initialization() -> Result {
    let file = ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical/CanonicalCompiledInitialization.class"
    ))?;
    assert_eq!(file.fields, [] as [ristretto_classfile::Field; 0]);
    assert!(file.methods.iter().any(|method| {
        file.constant_pool
            .try_get_utf8(method.name_index)
            .is_ok_and(|name| name == "<clinit>")
    }));
    let program = Program::from_classfiles([file])?;
    assert_eq!(
        program
            .run(
                "CanonicalCompiledInitialization",
                "run",
                17,
                configuration(),
                Limits::default()
            )
            .await?
            .result,
        17
    );
    let report = program
        .compile()?
        .run(
            "CanonicalCompiledInitialization",
            "run",
            17,
            configuration(),
            CompiledLimits::default(),
        )
        .await?;
    assert_eq!(report.result, 17);
    assert_eq!(report.class_initializer_calls, 1);
    assert_eq!(report.compiled_entries, 2);
    assert_eq!(report.entry_initialization.calls, 1);
    assert_eq!(report.static_storage_allocations, 0);
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    assert_eq!(report.catalog_bytes_after_drop, 0);
    Ok(())
}

#[tokio::test]
async fn shared_native_context_rejects_foreign_program_and_survives_limit_and_cancel() -> Result {
    use std::future::Future;
    use std::task::{Context, Waker};
    let program = program()?;
    let foreign = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical/CanonicalCompiledCalls.class"
    ))?])?;
    let compiled = program.compile()?;
    let foreign_compiled = foreign.compile()?;
    program
        .with_heap(configuration(), 1024 * 1024, 100_000, async |heap| {
            assert!(matches!(
                foreign_compiled.attach(heap, CompiledLimits::default()),
                Err(Error::Boundary(
                    "compiled program belongs to a different heap program"
                ))
            ));
            let mut limited = compiled.attach(
                heap,
                CompiledLimits {
                    native_slices: 1,
                    ..CompiledLimits::default()
                },
            )?;
            assert!(matches!(
                limited.invoke("CanonicalCompiledCalls", "run", 9).await,
                Err(Error::Limit("native slice"))
            ));
            assert_eq!(
                limited
                    .invoke("CanonicalCompiledCalls", "depth", 0)
                    .await?
                    .result,
                0
            );
            drop(limited);
            let mut thread = compiled.attach(heap, CompiledLimits::default())?;
            let mut invocation = Box::pin(thread.invoke("CanonicalCompiledCalls", "run", 64));
            assert!(
                invocation
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            drop(invocation);
            assert_eq!(
                thread
                    .invoke("CanonicalCompiledCalls", "run", 9)
                    .await?
                    .result,
                82
            );
            Ok(())
        })
        .await?;
    Ok(())
}
