//! Process identity is prepared before collectors exist; export follows shutdown.
use ristretto_gc::pause_recorder::{MeasurementBoundary, PauseRecorder, RECORDER};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub struct Capture {
    recorder: Arc<PauseRecorder>,
    identity: Value,
}

impl Capture {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let args: Vec<_> = std::env::args().skip(1).collect();
        let [
            _,
            classes,
            _,
            _,
            policy,
            workload,
            duration,
            warmup,
            rate,
            seed,
        ] = args.as_slice()
        else {
            return Err("pause capture requires the gc_compare argument contract".into());
        };
        let capacity = std::env::var("RISTRETTO_PAUSE_CAPACITY")
            .map_or(Ok(65_536), |value| value.parse::<usize>())?;
        let binary = std::env::current_exe()?;
        let identity = json!({
            "run_id": std::env::var("RISTRETTO_QUALIFICATION_RUN_ID").ok(),
            "policy_sha256": std::env::var("RISTRETTO_QUALIFICATION_POLICY_SHA256").ok(),
            "source_archive_sha256": std::env::var("RISTRETTO_SOURCE_ARCHIVE_SHA256").ok(),
            "binary_sha256": digest(&binary)?,
            "workload_sha256": workload_digest(Path::new(classes))?,
            "process_id": std::process::id(),
            "recorder_capacity": capacity,
            "runtime": format!("ristretto-{policy}"),
            "workload": workload,
            "duration_ms": duration.parse::<u64>()?,
            "warmup_ms": warmup.parse::<u64>()?,
            "rate": rate.parse::<u64>()?,
            "seed": seed.parse::<i32>()?,
        });

        // The fixed buffer is allocated before capture starts. It is charged to
        // the process's actual memory budget; capacity is retained in the trace.
        Ok(Self {
            recorder: Arc::new(PauseRecorder::new(capacity)?),
            identity,
        })
    }

    #[cfg(feature = "experimental-execution-measurement")]
    pub fn identity(&self) -> Value {
        self.identity.clone()
    }

    pub fn recorder(&self) -> Arc<PauseRecorder> {
        Arc::clone(&self.recorder)
    }

    pub fn finish(self, completed: bool) -> Result<(), Box<dyn std::error::Error>> {
        let recorder = Arc::try_unwrap(self.recorder)
            .map_err(|_| "pause capture still has live collector owners after runtime shutdown")?;
        let snapshot = recorder.finish();
        let events: Vec<_> = snapshot
            .events
            .iter()
            .map(|event| {
                json!({
                    "sequence": event.sequence, "source": "publication",
                    "kind": "equivalent_global_stall",
                    "requested_ns": event.requested_ns,
                    "acquired_ns": event.acquired_ns,
                    "released_ns": event.released_ns,
                })
            })
            .collect();
        let pair = snapshot.measurement_begin.zip(snapshot.measurement_end);
        let window = pair.map(|(begin, end)| {
            json!({
                "start_ns": begin.timestamp_ns, "end_ns": end.timestamp_ns,
            })
        });
        let gc_exposure = pair.map(|(begin, end)| json!({
            "scope": "measurement",
            "collections_completed_lower_bound": end.collections_before.checked_sub(begin.collections_after),
            "collections_completed_upper_bound": end.collections_after.checked_sub(begin.collections_before),
        }));
        println!(
            "RISTRETTO_PAUSES {}",
            json!({
                "schema_version": 2, "diagnostic_only": true, "qualification": false,
                "identity": self.identity, "recorder": RECORDER,
                "scope": "instrumented_collector_lifetime",
                "clock": ristretto_gc::clock::monotonic_clock_name(),
                "capture_start_ns": snapshot.capture_start_ns,
                "capture_end_ns": snapshot.capture_end_ns,
                "clock_resolution_ns": snapshot.clock_resolution_ns,
                "capture_complete": false,
                "instrumented_capture_complete": completed && snapshot.dropped_events == 0
                    && snapshot.open_events == 0 && snapshot.clock_errors == 0
                    && snapshot.publication_attempts == snapshot.publication_acquisitions
                        + snapshot.publication_failed_attempts,
                "global_source_coverage_complete": false,
                "global_stop_sources": ["publication"],
                "uncovered_global_stop_sources": ["allocation_recovery", "gc_safepoint"],
                "measurement_window": window,
                "measurement_boundaries": {
                    "state": snapshot.measurement_state, "errors": snapshot.measurement_errors,
                    "begin": snapshot.measurement_begin.map(boundary_json),
                    "end": snapshot.measurement_end.map(boundary_json),
                },
                "gc_exposure": gc_exposure,
                "mutator_admission_waits_recorded": false,
                "publication_attempts": snapshot.publication_attempts,
                "publication_acquisitions": snapshot.publication_acquisitions,
                "publication_failed_attempts": snapshot.publication_failed_attempts,
                "dropped_events": snapshot.dropped_events, "open_events": snapshot.open_events,
                "clock_errors": snapshot.clock_errors, "events": events,
            })
        );
        Ok(())
    }
}

fn boundary_json(boundary: MeasurementBoundary) -> Value {
    json!({
        "timestamp_ns": boundary.timestamp_ns,
        "observer_start_ns": boundary.observer_start_ns,
        "observer_end_ns": boundary.observer_end_ns,
        "collections_before": boundary.collections_before,
        "collections_after": boundary.collections_after,
    })
}

fn digest(path: &Path) -> std::io::Result<String> {
    Ok(hex(&Sha256::digest(std::fs::read(path)?)))
}

/// Hash every recursively discovered class, sorted by UTF-8 relative path.
/// Each record is: path bytes, NUL, lowercase content SHA-256, LF.
fn workload_digest(root: &Path) -> std::io::Result<String> {
    fn visit(root: &Path, relative: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(root.join(relative))? {
            let entry = entry?;
            let path = relative.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                visit(root, &path, paths)?;
            } else if path
                .extension()
                .is_some_and(|extension| extension == "class")
            {
                paths.push(path);
            }
        }
        Ok(())
    }
    let mut paths = Vec::new();
    visit(root, Path::new(""), &mut paths)?;
    if paths.is_empty() {
        return Err(std::io::Error::other("workload class manifest is empty"));
    }
    paths.sort();
    let mut hash = Sha256::new();
    for path in paths {
        let name = path
            .to_str()
            .ok_or_else(|| std::io::Error::other("non-UTF-8 class name"))?;
        hash.update(name.as_bytes());
        hash.update([0]);
        hash.update(digest(&root.join(&path))?.as_bytes());
        hash.update(b"\n");
    }
    Ok(hex(&hash.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(
        String::with_capacity(bytes.len() * 2),
        |mut result, byte| {
            // Writing into String cannot fail.
            let _ = write!(result, "{byte:02x}");
            result
        },
    )
}
