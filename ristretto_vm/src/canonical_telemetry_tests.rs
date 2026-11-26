#![expect(
    clippy::panic_in_result_fn,
    reason = "finite telemetry ownership assertions"
)]
use super::*;
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn binding() -> Binding {
    Binding {
        process_id: current_process_id(),
        run_id: "finite-fixture".into(),
        source_sha256: "a".repeat(64),
        binary_sha256: "b".repeat(64),
        workload_sha256: "c".repeat(64),
        configuration_sha256: "d".repeat(64),
    }
}

#[test]
fn live_guard_prevents_export_and_abandonment_is_not_success() -> Result {
    let telemetry = Telemetry::new(8, binding())?;
    let window = telemetry.measurement();
    let request = telemetry.request(Operation::Allocation);
    let telemetry = telemetry
        .finish()
        .err()
        .ok_or("export accepted live producer")?;
    drop(request);
    window.finish(Outcome::Complete);
    let capture = telemetry.finish().map_err(|_| "still owned")?;
    assert!(capture.snapshot.issues.contains(&"abandoned_event"));
    assert!(!capture.snapshot.global_capture_complete);
    Ok(())
}

#[test]
fn binding_checks_pid_and_digest_shape() -> Result {
    let mut invalid = binding();
    invalid.process_id = Some(0);
    assert!(Telemetry::new(8, invalid).is_err());
    let mut invalid = binding();
    invalid.source_sha256 = "not-a-digest".into();
    assert!(Telemetry::new(8, invalid).is_err());
    let telemetry = Telemetry::new(8, binding())?;
    let capture = telemetry.finish().map_err(|_| "owned")?;
    assert!(
        capture
            .snapshot
            .issues
            .contains(&"missing_measurement_window")
    );
    Ok(())
}
