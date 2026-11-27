//! One explicit Java measurement interval. This is diagnostic counter evidence,
//! not an atomic all-actor cut, CPU attribution, or pause/performance qualification.
use crate::execution_profile::{ExecutionCheckpoint, ExecutionInterval, Recorder};
use ristretto_gc::clock::monotonic_time_ns;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// Caps apply to retained diagnostic records, copied identity/error strings and
/// compiled-code bytes. Referenced VM method objects and allocator overhead are
/// not an exact byte census; they remain part of the process resource budget.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Limits {
    pub methods: usize,
    pub functions_per_method: usize,
    pub identity_bytes_per_method: usize,
    pub retained_code_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            methods: 4096,
            functions_per_method: 4,
            identity_bytes_per_method: 4096,
            retained_code_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Boundary {
    pub timestamp_ns: u64,
    pub observer_start_ns: u64,
    pub snapshot_start_ns: u64,
    pub snapshot_end_ns: u64,
    pub observer_end_ns: u64,
}
/// Export once after Java returns, including failed/open protocols.
#[derive(Debug, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "explicit independent evidence limitations in the serialized protocol"
)]
pub struct Capture {
    pub schema_version: u32,
    pub scope: &'static str,
    pub diagnostic_only: bool,
    pub qualification: bool,
    pub atomic_global_cut: bool,
    pub all_actor_coverage: bool,
    pub state: &'static str,
    pub boundary_thread_id: Option<u64>,
    pub limits: Option<Limits>,
    pub evidence_losses: usize,
    pub protocol_errors: usize,
    pub retained_code_bytes: usize,
    pub begin: Option<Boundary>,
    pub end: Option<Boundary>,
    pub interval: Option<ExecutionInterval>,
}
#[derive(Debug)]
struct Window {
    state: &'static str,
    actor: Option<u64>,
    begin: Option<Boundary>,
    end: Option<Boundary>,
    checkpoint: Option<ExecutionCheckpoint>,
    interval: Option<ExecutionInterval>,
}
impl Default for Window {
    fn default() -> Self {
        Self {
            state: "missing",
            actor: None,
            begin: None,
            end: None,
            checkpoint: None,
            interval: None,
        }
    }
}
#[derive(Debug, Default)]
pub(crate) struct State {
    limits: OnceLock<Limits>,
    losses: AtomicUsize,
    errors: AtomicUsize,
    code: AtomicUsize,
    window: Mutex<Window>,
}
fn error(message: &str) -> crate::Error {
    crate::Error::InternalError(message.to_owned())
}
fn clock() -> crate::Result<u64> {
    monotonic_time_ns().ok_or_else(|| error("execution measurement clock unavailable"))
}
impl State {
    pub(crate) fn arm(&self, limits: Limits) -> crate::Result<()> {
        if !(1..=65536).contains(&limits.methods)
            || !(1..=16).contains(&limits.functions_per_method)
            || !(1..=65536).contains(&limits.identity_bytes_per_method)
            || !(1..=1024 * 1024 * 1024).contains(&limits.retained_code_bytes)
        {
            return Err(error("invalid execution measurement limits"));
        }
        self.limits
            .set(limits)
            .map_err(|_| error("execution measurement already armed"))
    }
    pub(crate) fn lose(&self) {
        self.losses.fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn losses(&self) -> usize {
        self.losses.load(Ordering::Acquire)
    }
    pub(crate) fn admit_method(
        &self,
        count: usize,
        present: bool,
        class: &str,
        name: &str,
        descriptor: &str,
    ) -> bool {
        let Some(limits) = self.limits.get() else {
            return true;
        };
        if (!present && count >= limits.methods)
            || class
                .len()
                .saturating_add(name.len())
                .saturating_add(descriptor.len())
                > limits.identity_bytes_per_method
        {
            self.lose();
            return false;
        }
        true
    }
    pub(crate) fn admit_error(&self, bytes: usize) -> bool {
        if self
            .limits
            .get()
            .is_some_and(|limit| bytes > limit.identity_bytes_per_method)
        {
            self.lose();
            return false;
        }
        true
    }
    pub(crate) fn retain_code(&self, bytes: usize) -> bool {
        let Some(limit) = self.limits.get() else {
            return true;
        };
        // Called under the recorder's existing mutex; no retry loop is added.
        let before = self.code.load(Ordering::Relaxed);
        let Some(after) = before
            .checked_add(bytes)
            .filter(|after| *after <= limit.retained_code_bytes)
        else {
            self.lose();
            return false;
        };
        self.code.store(after, Ordering::Release);
        true
    }
    pub(crate) fn admit_function(&self, count: usize, bytes: usize) -> bool {
        if self
            .limits
            .get()
            .is_some_and(|limit| count >= limit.functions_per_method)
        {
            self.lose();
            return false;
        }
        self.retain_code(bytes)
    }
    pub(crate) fn take(&self) -> Capture {
        let mut window = self.window.try_lock().map_or_else(
            |_| {
                self.errors.fetch_add(1, Ordering::Relaxed);
                Window {
                    state: "contended",
                    ..Window::default()
                }
            },
            |mut guard| {
                std::mem::replace(
                    &mut *guard,
                    Window {
                        state: "taken",
                        ..Window::default()
                    },
                )
            },
        );
        if window.state == "taken" {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
        Capture {
            schema_version: 1,
            scope: "explicit_java_measurement",
            diagnostic_only: true,
            qualification: false,
            atomic_global_cut: false,
            all_actor_coverage: false,
            state: window.state,
            boundary_thread_id: window.actor,
            limits: self.limits.get().copied(),
            evidence_losses: self.losses(),
            protocol_errors: self.errors.load(Ordering::Acquire),
            retained_code_bytes: self.code.load(Ordering::Acquire),
            begin: window.begin.take(),
            end: window.end.take(),
            interval: window.interval.take(),
        }
    }
}
impl Recorder {
    pub(crate) fn measurement_boundary(
        self: &Arc<Self>,
        actor: u64,
        begin: bool,
        timestamp: impl FnOnce() -> crate::Result<u64>,
    ) -> crate::Result<u64> {
        if self.measurement.limits.get().is_none() {
            return timestamp();
        }
        let result = self.measurement_boundary_inner(actor, begin, timestamp);
        if result.is_err() {
            self.measurement.errors.fetch_add(1, Ordering::Relaxed);
        }
        result
    }
    fn measurement_boundary_inner(
        self: &Arc<Self>,
        actor: u64,
        begin: bool,
        timestamp: impl FnOnce() -> crate::Result<u64>,
    ) -> crate::Result<u64> {
        // Competing/recursive boundaries reject immediately rather than adding a wait.
        let mut window = self
            .measurement
            .window
            .try_lock()
            .map_err(|_| error("concurrent execution boundary"))?;
        if (begin && window.state != "missing")
            || (!begin && (window.state != "open" || window.actor != Some(actor)))
        {
            return Err(error(
                "duplicate, reversed, or foreign-thread execution boundary",
            ));
        }
        window.state = "failed";
        let observer_start_ns = clock()?;
        if begin {
            let snapshot_start_ns = clock()?;
            let checkpoint = self.checkpoint("GcTelemetry explicit measurement".to_owned());
            let snapshot_end_ns = clock()?;
            let timestamp_ns = timestamp()?;
            let observer_end_ns = clock()?;
            if !(observer_start_ns <= snapshot_start_ns
                && snapshot_start_ns <= snapshot_end_ns
                && snapshot_end_ns <= timestamp_ns
                && timestamp_ns <= observer_end_ns)
            {
                return Err(error("reversed execution begin clock"));
            }
            window.actor = Some(actor);
            window.checkpoint = Some(checkpoint);
            window.begin = Some(Boundary {
                timestamp_ns,
                observer_start_ns,
                snapshot_start_ns,
                snapshot_end_ns,
                observer_end_ns,
            });
            window.state = "open";
            Ok(timestamp_ns)
        } else {
            let timestamp_ns = timestamp()?;
            let snapshot_start_ns = clock()?;
            let checkpoint = window
                .checkpoint
                .take()
                .ok_or_else(|| error("execution checkpoint missing"))?;
            let interval = self.since(checkpoint)?;
            let snapshot_end_ns = clock()?;
            let observer_end_ns = clock()?;
            let start = window
                .begin
                .as_ref()
                .ok_or_else(|| error("execution begin missing"))?;
            if !(start.observer_end_ns <= observer_start_ns
                && observer_start_ns <= timestamp_ns
                && timestamp_ns <= snapshot_start_ns
                && snapshot_start_ns <= snapshot_end_ns
                && snapshot_end_ns <= observer_end_ns
                && start.timestamp_ns < timestamp_ns)
            {
                return Err(error("reversed execution end clock"));
            }
            window.end = Some(Boundary {
                timestamp_ns,
                observer_start_ns,
                snapshot_start_ns,
                snapshot_end_ns,
                observer_end_ns,
            });
            window.interval = Some(interval);
            window.state = "closed";
            Ok(timestamp_ns)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_storage_limits_reject_instead_of_silently_omitting() -> crate::Result<()> {
        let state = State::default();
        state.arm(Limits {
            methods: 1,
            functions_per_method: 1,
            identity_bytes_per_method: 8,
            retained_code_bytes: 10,
        })?;
        assert!(state.admit_method(0, false, "C", "m", "()V"));
        assert!(!state.admit_method(1, false, "C", "n", "()V"));
        assert!(!state.admit_method(1, true, "LongClassName", "m", "()V"));
        assert!(!state.admit_error(9));
        assert!(state.admit_function(0, 10));
        assert!(!state.admit_function(1, 0));
        assert!(!state.retain_code(1));
        let capture = state.take();
        assert_eq!(capture.retained_code_bytes, 10);
        assert_eq!(capture.evidence_losses, 5);
        assert!(state.arm(Limits::default()).is_err());
        Ok(())
    }

    #[test]
    fn contended_boundary_and_export_fail_without_waiting() -> crate::Result<()> {
        let recorder = Arc::new(Recorder::default());
        recorder.arm_measurement(Limits::default())?;
        let _held = recorder
            .measurement
            .window
            .lock()
            .map_err(|_| error("poisoned fixture lock"))?;
        assert!(recorder.measurement_boundary(1, true, clock).is_err());
        let capture = recorder.measurement.take();
        assert_eq!(capture.state, "contended");
        assert_eq!(capture.protocol_errors, 2);
        Ok(())
    }
}
