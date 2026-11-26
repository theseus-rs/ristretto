//! Opt-in canonical service diagnostics. Binding/setup/export happen outside the
//! measured body. Cloned handles and span guards only change local Rc ownership.
//! This single-actor producer never claims complete global pause coverage.

use ristretto_gc::service_recorder::ServiceRecorder;
pub use ristretto_gc::service_recorder::{Kind, Operation, Outcome, Snapshot, SpanId};
use std::cell::Cell;
use std::rc::Rc;

#[path = "canonical_cycle_windows.rs"]
mod cycle_windows;
pub(crate) use cycle_windows::CycleMeasurement;
pub use cycle_windows::{CycleCapture, CycleWindow};

/// Declared inputs, to be checked against the harness's independent receipts.
/// The constructor checks shape and the actual PID, not the supplied digests'
/// file contents. Those hashes are not self-authenticating evidence.
#[derive(Debug)]
pub struct Binding {
    pub process_id: Option<u32>,
    pub run_id: String,
    pub source_sha256: String,
    pub binary_sha256: String,
    pub workload_sha256: String,
    pub configuration_sha256: String,
}

/// WASI/bare Wasm do not provide a process-ID contract; never call their
/// unsupported std process API or substitute a fabricated native PID.
#[must_use]
pub fn current_process_id() -> Option<u32> {
    #[cfg(target_family = "wasm")]
    {
        None
    }
    #[cfg(not(target_family = "wasm"))]
    {
        Some(std::process::id())
    }
}

#[derive(Debug)]
struct Inner {
    recorder: ServiceRecorder,
    binding: Binding,
    cycles: Cell<CycleCapture>,
}

/// One explicitly supplied, non-Send run handle; there is no process singleton.
#[derive(Clone, Debug)]
pub struct Telemetry(Rc<Inner>);

#[derive(Debug)]
pub struct Capture {
    pub binding: Binding,
    pub snapshot: Snapshot,
    pub cycles: CycleCapture,
}

#[derive(Debug)]
pub struct Span {
    inner: Rc<Inner>,
    id: Option<SpanId>,
}

impl Telemetry {
    /// Preallocate before execution; requires actual process identity and declared
    /// source, executable, bytecode/workload and admission/configuration digests.
    ///
    /// # Errors
    /// Invalid binding, capacity or clock. Digest file validation is a harness duty.
    pub fn new(capacity: usize, binding: Binding) -> std::io::Result<Self> {
        let digest =
            |value: &str| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
        if binding.process_id != current_process_id()
            || binding.run_id.is_empty()
            || ![
                &binding.source_sha256,
                &binding.binary_sha256,
                &binding.workload_sha256,
                &binding.configuration_sha256,
            ]
            .into_iter()
            .all(|value| digest(value))
        {
            return Err(std::io::Error::other("invalid canonical telemetry binding"));
        }
        Ok(Self(Rc::new(Inner {
            recorder: ServiceRecorder::new(capacity)?,
            binding,
            cycles: Cell::new(CycleCapture::Unobserved),
        })))
    }

    #[must_use]
    pub fn request(&self, operation: Operation) -> Span {
        self.service(Kind::Request, operation, None)
    }

    /// Synchronous service only; hold request spans separately across await/yield.
    #[must_use]
    pub fn service(&self, kind: Kind, operation: Operation, parent: Option<SpanId>) -> Span {
        Span {
            inner: self.0.clone(),
            id: self.0.recorder.begin(kind, operation, parent),
        }
    }

    /// The caller supplies the true body boundary. Do not surround full `run`
    /// (setup/cleanup), infer this from stdout, or treat an absent window as zero.
    #[must_use]
    pub fn measurement(&self) -> Span {
        self.service(Kind::Measurement, Operation::Measurement, None)
    }

    /// Export only after execution and all span guards/cloned handles are gone.
    ///
    /// # Errors
    /// Returns this handle unchanged when another producer/guard still owns it.
    pub fn finish(self) -> Result<Capture, Self> {
        Rc::try_unwrap(self.0)
            .map(|inner| Capture {
                binding: inner.binding,
                snapshot: inner.recorder.snapshot(),
                cycles: inner.cycles.get(),
            })
            .map_err(Self)
    }
}

impl Span {
    #[must_use]
    pub fn id(&self) -> Option<SpanId> {
        self.id
    }

    pub fn finish(mut self, outcome: Outcome) {
        if let Some(id) = self.id.take() {
            self.inner.recorder.finish(id, outcome);
        }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.inner.recorder.finish(id, Outcome::Abandoned);
        }
    }
}

#[cfg(test)]
#[path = "canonical_telemetry_tests.rs"]
mod tests;
