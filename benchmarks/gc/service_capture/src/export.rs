//! Export after the body/cleanup. Events serialize one at a time; no second
//! full JSON-event array is retained alongside the recorder snapshot.
use ristretto_gc::service_recorder::{Event, Gap, Kind, Operation, Outcome, SpanId};
use ristretto_vm::canonical::Report;
use ristretto_vm::canonical_telemetry::{Capture, CycleCapture, CycleWindow};
use serde::Serialize;
use serde_json::{Value, json};
use std::io::Write;

#[derive(Serialize)]
struct EventView {
    sequence: usize,
    parent: Option<usize>,
    kind: &'static str,
    operation: &'static str,
    start_ns: u64,
    end_ns: Option<u64>,
    outcome: Option<&'static str>,
}
impl From<&Event> for EventView {
    fn from(event: &Event) -> Self {
        Self {
            sequence: event.id.sequence(),
            parent: event.parent.map(SpanId::sequence),
            kind: match event.kind {
                Kind::Request => "request",
                Kind::Preparation => "preparation",
                Kind::RequestPoll => "request_poll",
                Kind::HelperPoll => "helper_poll",
                Kind::Measurement => "measurement",
            },
            operation: match event.operation {
                Operation::Allocation => "allocation",
                Operation::ChildRead => "child_read",
                Operation::EdgeWrite => "edge_write",
                Operation::Maintenance => "maintenance",
                Operation::Measurement => "measurement",
                Operation::PrimitiveArrayPreparation => "primitive_array_preparation",
            },
            start_ns: event.start_ns,
            end_ns: event.end_ns,
            outcome: event.outcome.map(|value| match value {
                Outcome::Complete => "complete",
                Outcome::Pending => "pending",
                Outcome::Failed => "failed",
                Outcome::Abandoned => "abandoned",
            }),
        }
    }
}

#[derive(Serialize)]
struct GapView {
    request: usize,
    start_ns: u64,
    end_ns: u64,
    after_pending: bool,
}
impl From<&Gap> for GapView {
    fn from(gap: &Gap) -> Self {
        Self {
            request: gap.request.sequence(),
            start_ns: gap.start_ns,
            end_ns: gap.end_ns,
            after_pending: gap.after_pending,
        }
    }
}

fn report_json(report: &Report) -> Value {
    json!({"result":report.result,"instructions":report.instructions,
        "instruction_count_available":report.instruction_count_available,"allocations":report.allocations,
        "array_allocations":report.array_allocations,"reference_reads":report.reference_reads,"reference_writes":report.reference_writes,
        "calls":report.calls,"helper_polls":report.helper_polls,"released_objects":report.released_objects,
        "payload_bytes_after_cleanup":report.payload_bytes_after_cleanup,"catalog_bytes_after_drop":report.catalog_bytes_after_drop,
        "compiled_entries":report.compiled_entries,"compiled_resumes":report.compiled_resumes,
        "compiled_backedges":report.compiled_backedges,"native_slices":report.native_slices})
}

fn cycle_json(capture: CycleCapture) -> Value {
    let (status, window) = match capture {
        CycleCapture::Unobserved => ("unobserved", None),
        CycleCapture::Incomplete => ("incomplete", None),
        CycleCapture::MultipleWindows => ("multiple_windows", None),
        CycleCapture::Complete(window) => ("complete", Some(window)),
    };
    let bounds = |value: ristretto_gc::canonical::PrimaryCycles| json!({"lower": value.lower, "upper": value.upper});
    let observations = window.map_or([None; 4], |window| {
        [
            window.before_start,
            window.after_start,
            window.before_end,
            window.after_end,
        ]
    });
    json!({"status":status,"scope":"canonical_primary_full_epochs",
        "measurement_sequence":window.and_then(|window|window.measurement).map(SpanId::sequence),
        "observations":observations.map(|value|value.map(bounds)),
        "bounds":window.and_then(CycleWindow::bounds).map(bounds)})
}

pub fn write(
    capture: Capture,
    identity: &Value,
    report: Option<&Report>,
    error: Option<&str>,
) -> std::io::Result<()> {
    let snapshot = capture.snapshot;
    let binding = capture.binding;
    let header = json!({"schema_version":2,"protocol":snapshot.protocol,"diagnostic_only":true,"qualification":false,
        "cycle_observation":cycle_json(capture.cycles),
        "scope":"canonical_single_actor_services","window_kind":"entry_body_v1","identity":identity,
        "binding":{"process_id":binding.process_id,"run_id":binding.run_id,"source_sha256":binding.source_sha256,
            "binary_sha256":binding.binary_sha256,"workload_sha256":binding.workload_sha256,
            "configuration_sha256":binding.configuration_sha256},
        "clock":snapshot.clock,"clock_resolution_ns":snapshot.clock_resolution_ns,
        "capture_start_ns":snapshot.capture_start_ns,"capture_end_ns":snapshot.capture_end_ns,"capacity":snapshot.capacity,
        "overflow":snapshot.overflow,"open_events":snapshot.open_events,"producer_issues":snapshot.issues,
        "instrumented_capture_complete":snapshot.instrumented_capture_complete,"global_capture_complete":snapshot.global_capture_complete,
        "missing_sources":snapshot.missing_sources,"measurement_sequence":snapshot.measurement.map(SpanId::sequence),
        "recorder_id":snapshot.events.first().map(|event|event.id.recorder_id()),
        "execution":{"status":if error.is_none(){"ok"}else{"error"},"error":error,"report":report.map(report_json)}});
    let object = header
        .as_object()
        .ok_or_else(|| std::io::Error::other("capture header is not an object"))?;
    let stdout = std::io::stdout();
    let mut output = std::io::BufWriter::new(stdout.lock());
    write!(output, "RISTRETTO_CANONICAL_SERVICE {{")?;
    for (key, value) in object {
        serde_json::to_writer(&mut output, key)?;
        write!(output, ":")?;
        serde_json::to_writer(&mut output, value)?;
        write!(output, ",")?;
    }
    write!(output, "\"events\":[")?;
    for (index, event) in snapshot.events.iter().enumerate() {
        if index != 0 {
            write!(output, ",")?;
        }
        serde_json::to_writer(&mut output, &EventView::from(event))?;
    }
    write!(output, "],\"gaps\":[")?;
    for (index, gap) in snapshot.gaps.iter().enumerate() {
        if index != 0 {
            write!(output, ",")?;
        }
        serde_json::to_writer(&mut output, &GapView::from(gap))?;
    }
    writeln!(output, "]}}")?;
    output.flush()
}
