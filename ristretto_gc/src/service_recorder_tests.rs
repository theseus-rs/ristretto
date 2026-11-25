#![expect(
    clippy::panic_in_result_fn,
    reason = "finite telemetry correctness assertions"
)]
use super::*;
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn event(
    recorder: &ServiceRecorder,
    kind: Kind,
    parent: Option<SpanId>,
    start: u64,
    end: u64,
    outcome: Outcome,
) -> Result<SpanId> {
    let id = recorder
        .begin(kind, Operation::Allocation, parent)
        .ok_or("slot missing")?;
    recorder.finish(id, outcome);
    let slot = recorder.slots.get(id.index).ok_or("slot missing")?;
    slot.start.store(start, Ordering::Relaxed);
    slot.end.store(end, Ordering::Relaxed);
    Ok(id)
}

#[test]
fn pending_gaps_are_separate_from_synchronous_service() -> Result {
    let mut recorder = ServiceRecorder::new(8)?;
    recorder.start = 100;
    event(
        &recorder,
        Kind::Measurement,
        None,
        100,
        1000,
        Outcome::Complete,
    )?;
    let request = event(&recorder, Kind::Request, None, 200, 900, Outcome::Complete)?;
    event(
        &recorder,
        Kind::Preparation,
        Some(request),
        200,
        300,
        Outcome::Complete,
    )?;
    event(
        &recorder,
        Kind::RequestPoll,
        Some(request),
        400,
        500,
        Outcome::Pending,
    )?;
    event(
        &recorder,
        Kind::HelperPoll,
        Some(request),
        550,
        600,
        Outcome::Complete,
    )?;
    event(
        &recorder,
        Kind::RequestPoll,
        Some(request),
        800,
        900,
        Outcome::Complete,
    )?;
    let snapshot = recorder.snapshot();
    assert!(
        snapshot.instrumented_capture_complete,
        "{:?}",
        snapshot.issues
    );
    assert!(!snapshot.global_capture_complete);
    let gaps: Vec<_> = snapshot
        .gaps
        .iter()
        .map(|gap| (gap.start_ns, gap.end_ns, gap.after_pending))
        .collect();
    assert_eq!(
        gaps,
        [(300, 400, false), (500, 550, true), (600, 800, true)]
    );
    assert!(snapshot.missing_sources.contains(&"owner_release"));
    Ok(())
}

#[test]
fn overflow_and_open_events_fail_closed() -> Result {
    let recorder = ServiceRecorder::new(1)?;
    assert!(
        recorder
            .begin(Kind::Request, Operation::Allocation, None)
            .is_some()
    );
    assert!(
        recorder
            .begin(Kind::HelperPoll, Operation::Maintenance, None)
            .is_none()
    );
    let snapshot = recorder.snapshot();
    assert!(snapshot.overflow);
    assert_eq!(snapshot.open_events, 1);
    assert!(!snapshot.instrumented_capture_complete);
    assert!(snapshot.issues.contains(&"missing_measurement_window"));
    Ok(())
}

#[test]
fn foreign_and_duplicate_completions_fail_closed() -> Result {
    let first = ServiceRecorder::new(2)?;
    let second = ServiceRecorder::new(2)?;
    let id = first
        .begin(Kind::Request, Operation::Allocation, None)
        .ok_or("no id")?;
    second.finish(id, Outcome::Complete);
    assert!(
        second
            .begin(Kind::RequestPoll, Operation::Allocation, Some(id))
            .is_none()
    );
    first.finish(id, Outcome::Complete);
    first.finish(id, Outcome::Complete);
    assert!(first.snapshot().issues.contains(&"invalid_completion"));
    assert!(second.snapshot().issues.contains(&"invalid_completion"));
    Ok(())
}

#[test]
fn boundary_crossings_remain_visible_without_clipping() -> Result {
    let mut recorder = ServiceRecorder::new(3)?;
    recorder.start = 100;
    event(
        &recorder,
        Kind::Measurement,
        None,
        200,
        400,
        Outcome::Complete,
    )?;
    event(&recorder, Kind::Request, None, 150, 450, Outcome::Complete)?;
    let snapshot = recorder.snapshot();
    assert!(snapshot.issues.contains(&"measurement_boundary_crossing"));
    let request = snapshot
        .events
        .iter()
        .find(|event| event.kind == Kind::Request)
        .ok_or("request")?;
    assert_eq!((request.start_ns, request.end_ns), (150, Some(450)));
    Ok(())
}

#[test]
fn malformed_windows_parents_and_clock_order_fail_closed() -> Result {
    let mut recorder = ServiceRecorder::new(5)?;
    recorder.start = 100;
    let window = event(
        &recorder,
        Kind::Measurement,
        None,
        300,
        200,
        Outcome::Complete,
    )?;
    event(
        &recorder,
        Kind::Measurement,
        None,
        100,
        500,
        Outcome::Complete,
    )?;
    event(
        &recorder,
        Kind::RequestPoll,
        Some(window),
        350,
        360,
        Outcome::Complete,
    )?;
    event(
        &recorder,
        Kind::Preparation,
        None,
        400,
        450,
        Outcome::Abandoned,
    )?;
    let snapshot = recorder.snapshot();
    for issue in [
        "clock_order",
        "multiple_measurement_windows",
        "invalid_request_parent",
        "missing_request_parent",
        "abandoned_event",
    ] {
        assert!(snapshot.issues.contains(&issue), "missing {issue}");
    }
    Ok(())
}

#[test]
#[cfg(feature = "experimental-canonical-runtime")]
fn real_canonical_prepare_poll_reclaim_produces_partial_service_evidence() -> Result {
    use crate::GarbageCollector;
    use crate::canonical::{Budget, Configuration, Runtime};
    use std::task::Poll;
    let gc = GarbageCollector::new();
    let config = Configuration {
        objects: 2,
        roots: 3,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&gc, config, &budget)?;
    let mut mutator = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let recorder = ServiceRecorder::new(4096)?;
    let window = recorder
        .begin(Kind::Measurement, Operation::Measurement, None)
        .ok_or("window")?;
    let request = recorder
        .begin(Kind::Request, Operation::Allocation, None)
        .ok_or("request")?;
    let prep = recorder
        .begin(Kind::Preparation, Operation::Allocation, Some(request))
        .ok_or("prep")?;
    let mut allocation = mutator.allocate(42_u64)?;
    recorder.finish(prep, Outcome::Complete);
    let mut root = None;
    for _ in 0..256 {
        let poll = recorder
            .begin(Kind::RequestPoll, Operation::Allocation, Some(request))
            .ok_or("poll")?;
        match allocation.poll()? {
            Poll::Ready(value) => {
                recorder.finish(poll, Outcome::Complete);
                root = Some(value);
                break;
            }
            Poll::Pending => recorder.finish(poll, Outcome::Pending),
        }
        let assist = recorder
            .begin(Kind::HelperPoll, Operation::Maintenance, Some(request))
            .ok_or("helper")?;
        helper.poll()?;
        recorder.finish(assist, Outcome::Complete);
    }
    drop(allocation);
    let root = root.ok_or("bounded fixture did not finish")?;
    assert_eq!(*runtime.borrow(&root)?, 42);
    recorder.finish(request, Outcome::Complete);
    assert!(gc.managed_bytes() > 0);
    drop(root); // deliberately missing release producer remains declared.
    for _ in 0..1024 {
        let assist = recorder
            .begin(Kind::HelperPoll, Operation::Maintenance, None)
            .ok_or("cleanup")?;
        helper.poll()?;
        recorder.finish(assist, Outcome::Complete);
        if gc.managed_bytes() == 0 {
            break;
        }
    }
    assert_eq!(gc.managed_bytes(), 0);
    recorder.finish(window, Outcome::Complete);
    let snapshot = recorder.snapshot();
    assert!(
        snapshot.instrumented_capture_complete,
        "{:?}",
        snapshot.issues
    );
    assert!(!snapshot.global_capture_complete);
    assert!(
        snapshot
            .events
            .iter()
            .any(|event| event.outcome == Some(Outcome::Pending))
    );
    drop((mutator, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}
