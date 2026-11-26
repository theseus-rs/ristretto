#![cfg(feature = "experimental-canonical-telemetry")]
#![expect(clippy::panic_in_result_fn, reason = "finite correctness assertions")]
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Limits, Program, Report};
use ristretto_vm_research::canonical_telemetry::{
    Binding, Capture, CycleCapture, Kind, Operation, Outcome, Telemetry, current_process_id,
};
use std::future::Future;
use std::task::{Context, Waker};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn program() -> Result<Program> {
    Ok(Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload$Holder.class"
        ))?,
    ])?)
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
fn telemetry(capacity: usize) -> Result<Telemetry> {
    // Deliberately declared unit-test bindings, not a source-authenticated timing
    // artifact. The separate export harness supplies and verifies actual digests.
    Ok(Telemetry::new(
        capacity,
        Binding {
            process_id: current_process_id(),
            run_id: "canonical-service-unit-fixture".to_owned(),
            source_sha256: "0".repeat(64),
            binary_sha256: "0".repeat(64),
            workload_sha256: "0".repeat(64),
            configuration_sha256: "0".repeat(64),
        },
    )?)
}
fn finish(telemetry: Telemetry) -> Result<Capture> {
    telemetry
        .finish()
        .map_err(|_| "recorder owner escaped invocation".into())
}
fn verify(report: &Report, capture: &Capture) -> Result {
    let snapshot = &capture.snapshot;
    assert!(
        snapshot.instrumented_capture_complete,
        "{:?}",
        snapshot.issues
    );
    assert!(!snapshot.global_capture_complete);
    let CycleCapture::Complete(cycles) = capture.cycles else {
        return Err("missing completed cycle window".into());
    };
    assert_eq!(cycles.measurement, snapshot.measurement);
    let bounds = cycles.bounds().ok_or("unavailable cycle observations")?;
    assert!(bounds.lower <= bounds.upper);
    assert!(snapshot.missing_sources.contains(&"global_actor_state"));
    assert_eq!(snapshot.open_events, 0);
    let events = &snapshot.events;
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind == Kind::HelperPoll)
            .count(),
        report.helper_polls
    );
    for (operation, expected) in [
        (Operation::Allocation, report.allocations),
        (Operation::ChildRead, report.reference_reads),
        (Operation::EdgeWrite, report.reference_writes),
    ] {
        assert_eq!(
            events
                .iter()
                .filter(|event| event.kind == Kind::Request
                    && event.operation == operation
                    && event.outcome == Some(Outcome::Complete))
                .count(),
            expected
        );
    }
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind == Kind::Preparation
                && event.operation == Operation::PrimitiveArrayPreparation)
            .count(),
        report.array_allocations
    );
    assert!(events.iter().any(|event| event.kind == Kind::RequestPoll && event.outcome == Some(Outcome::Pending)));
    assert!(snapshot.gaps.iter().any(|gap| gap.after_pending));
    let window = events
        .iter()
        .find(|event| event.kind == Kind::Measurement)
        .ok_or("missing body window")?;
    let end = window.end_ns.ok_or("open body window")?;
    assert!(
        events
            .iter()
            .any(|event| event.kind == Kind::HelperPoll && event.start_ns >= end)
    );
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    assert_eq!(report.catalog_bytes_after_drop, 0);
    Ok(())
}

#[tokio::test]
async fn interpreted_body_records_preparation_pending_service_and_cleanup() -> Result {
    let program = program()?;
    let ordinary = program
        .run(
            "CanonicalPrimitiveArrayWorkload",
            "run",
            1,
            configuration(),
            Limits::default(),
        )
        .await?;
    let recording = telemetry(16_384)?;
    let report = program
        .run_recorded(
            "CanonicalPrimitiveArrayWorkload",
            "run",
            1,
            configuration(),
            Limits::default(),
            &recording,
        )
        .await?;
    assert_eq!(report.result, ordinary.result);
    assert_eq!(report.allocations, ordinary.allocations);
    verify(&report, &finish(recording)?)
}

#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[tokio::test]
async fn compiled_body_records_the_same_canonical_service_boundary() -> Result {
    use ristretto_vm_research::canonical::CompiledLimits;
    let program = program()?;
    let compiled = program.compile()?;
    let recording = telemetry(16_384)?;
    let report = compiled
        .run_recorded(
            "CanonicalPrimitiveArrayWorkload",
            "run",
            1,
            configuration(),
            CompiledLimits::default(),
            &recording,
        )
        .await?;
    verify(&report, &finish(recording)?)
}

#[tokio::test]
async fn preparation_failure_and_overflow_are_incomplete() -> Result {
    let program = program()?;
    let recording = telemetry(128)?;
    assert!(
        program
            .run_recorded(
                "CanonicalPrimitiveArrayWorkload",
                "failNegative",
                -1,
                configuration(),
                Limits::default(),
                &recording
            )
            .await
            .is_err()
    );
    let capture = finish(recording)?;
    assert!(!capture.snapshot.instrumented_capture_complete);
    assert!(capture.snapshot.events.iter().any(|event| event.operation
        == Operation::PrimitiveArrayPreparation
        && event.outcome == Some(Outcome::Failed)));
    assert!(
        capture
            .snapshot
            .events
            .iter()
            .any(|event| event.kind == Kind::Request && event.outcome == Some(Outcome::Failed))
    );
    let recording = telemetry(1)?;
    let report = program
        .run_recorded(
            "CanonicalPrimitiveArrayWorkload",
            "run",
            1,
            configuration(),
            Limits::default(),
            &recording,
        )
        .await?;
    assert_eq!(report.payload_bytes_after_cleanup, 0);
    let capture = finish(recording)?;
    assert!(capture.snapshot.overflow);
    assert!(!capture.snapshot.instrumented_capture_complete);
    assert!(!capture.snapshot.global_capture_complete);
    Ok(())
}

#[tokio::test]
async fn cancelled_pending_request_releases_recorder_guards() -> Result {
    let program = program()?;
    let recording = telemetry(1024)?;
    let mut future = Box::pin(program.run_recorded(
        "CanonicalPrimitiveArrayWorkload",
        "run",
        1,
        configuration(),
        Limits::default(),
        &recording,
    ));
    let mut context = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut context).is_pending());
    drop(future);
    let capture = finish(recording)?;
    assert_eq!(capture.cycles, CycleCapture::Incomplete);
    assert_eq!(capture.snapshot.open_events, 0);
    assert!(!capture.snapshot.instrumented_capture_complete);
    assert!(
        capture
            .snapshot
            .events
            .iter()
            .any(|event| event.kind == Kind::Request && event.outcome == Some(Outcome::Abandoned))
    );
    assert!(
        capture
            .snapshot
            .events
            .iter()
            .any(|event| event.kind == Kind::Measurement
                && event.outcome == Some(Outcome::Abandoned))
    );
    assert!(!capture.snapshot.issues.contains(&"invalid_request_parent"));
    assert!(!capture.snapshot.issues.contains(&"clock_order"));
    #[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
    {
        use ristretto_vm_research::canonical::CompiledLimits;
        let compiled = program.compile()?;
        let recording = telemetry(1024)?;
        let mut future = Box::pin(compiled.run_recorded(
            "CanonicalPrimitiveArrayWorkload",
            "run",
            1,
            configuration(),
            CompiledLimits::default(),
            &recording,
        ));
        // The native entry first emits the call to booleans. Its next slice
        // reaches the actual canonical array-publication request.
        for _ in 0..2 {
            assert!(future.as_mut().poll(&mut context).is_pending());
        }
        drop(future);
        let capture = finish(recording)?;
        assert_eq!(capture.cycles, CycleCapture::Incomplete);
        assert_eq!(capture.snapshot.open_events, 0);
        assert!(!capture.snapshot.instrumented_capture_complete);
        assert!(
            capture
                .snapshot
                .events
                .iter()
                .any(|event| event.kind == Kind::Request
                    && event.outcome == Some(Outcome::Abandoned))
        );
        assert!(!capture.snapshot.issues.contains(&"invalid_request_parent"));
        assert!(!capture.snapshot.issues.contains(&"clock_order"));
    }
    Ok(())
}

#[tokio::test]
async fn separate_invocations_cannot_merge_dedicated_heap_cycle_counters() -> Result {
    let program = program()?;
    let recording = telemetry(4096)?;
    for _ in 0..2 {
        program
            .run_recorded(
                "CanonicalPrimitiveArrayWorkload",
                "ints",
                1,
                configuration(),
                Limits::default(),
                &recording,
            )
            .await?;
    }
    let capture = finish(recording)?;
    assert_eq!(capture.cycles, CycleCapture::MultipleWindows);
    assert!(!capture.snapshot.instrumented_capture_complete);
    Ok(())
}

#[tokio::test]
async fn cancelled_window_cannot_be_reused_as_a_fresh_cycle_origin() -> Result {
    let program = program()?;
    let recording = telemetry(4096)?;
    let mut context = Context::from_waker(Waker::noop());
    let mut first = Box::pin(program.run_recorded(
        "CanonicalPrimitiveArrayWorkload",
        "run",
        1,
        configuration(),
        Limits::default(),
        &recording,
    ));
    assert!(first.as_mut().poll(&mut context).is_pending());
    drop(first);
    program
        .run_recorded(
            "CanonicalPrimitiveArrayWorkload",
            "ints",
            1,
            configuration(),
            Limits::default(),
            &recording,
        )
        .await?;
    let capture = finish(recording)?;
    assert_eq!(capture.cycles, CycleCapture::MultipleWindows);
    assert!(!capture.snapshot.instrumented_capture_complete);
    Ok(())
}

#[tokio::test]
async fn overlapping_windows_cannot_merge_independent_cycle_counters() -> Result {
    let program = program()?;
    let recording = telemetry(4096)?;
    let mut context = Context::from_waker(Waker::noop());
    let mut first = Box::pin(program.run_recorded(
        "CanonicalPrimitiveArrayWorkload",
        "run",
        1,
        configuration(),
        Limits::default(),
        &recording,
    ));
    let mut second = Box::pin(program.run_recorded(
        "CanonicalPrimitiveArrayWorkload",
        "run",
        1,
        configuration(),
        Limits::default(),
        &recording,
    ));
    assert!(first.as_mut().poll(&mut context).is_pending());
    assert!(second.as_mut().poll(&mut context).is_pending());
    drop(first);
    drop(second);
    let capture = finish(recording)?;
    assert_eq!(capture.cycles, CycleCapture::MultipleWindows);
    assert_eq!(capture.snapshot.open_events, 0);
    assert!(!capture.snapshot.instrumented_capture_complete);
    Ok(())
}
