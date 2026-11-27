//! Explicit diagnostic phase boundaries; ordinary Java methods run on peer VMs.
use crate::Thread;
use ristretto_classloader::Value;
use ristretto_types::{IntrinsicMetadata, IntrinsicMethod, IntrinsicRegistry, Parameters, Result};
use std::sync::Arc;

pub(crate) fn register(registry: &mut IntrinsicRegistry<Thread>) -> Result<()> {
    static BEGIN: IntrinsicMetadata = IntrinsicMetadata {
        signature: "GcTelemetry.beginMeasurement()J",
        implementation: "ristretto_vm::measurement::begin",
        source: "ristretto_vm/src/measurement.rs",
        versions: 31,
    };
    static END: IntrinsicMetadata = IntrinsicMetadata {
        signature: "GcTelemetry.endMeasurement()J",
        implementation: "ristretto_vm::measurement::end",
        source: "ristretto_vm/src/measurement.rs",
        versions: 31,
    };
    registry.register(&BEGIN, IntrinsicMethod::Sync(begin))?;
    registry.register(&END, IntrinsicMethod::Sync(end))
}

fn boundary(thread: &Thread, begin: bool) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let stamp = || -> Result<u64> {
        if let Some(recorder) = collector.pause_recorder() {
            if begin {
                recorder.begin_measurement()
            } else {
                recorder.end_measurement()
            }
        } else {
            ristretto_gc::clock::monotonic_time_ns()
        }
        .ok_or_else(|| {
            ristretto_types::Error::InternalError(
                "invalid measurement boundary or unavailable monotonic clock".to_owned(),
            )
        })
    };
    #[cfg(feature = "experimental-execution-measurement")]
    let timestamp = vm
        .execution_recorder
        .measurement_boundary(thread.id(), begin, stamp)?;
    #[cfg(not(feature = "experimental-execution-measurement"))]
    let timestamp = stamp()?;
    Ok(Some(Value::Long(timestamp.cast_signed())))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "registered synchronous intrinsic ABI owns the thread Arc"
)]
fn begin(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    boundary(&thread, true)
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "registered synchronous intrinsic ABI owns the thread Arc"
)]
fn end(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    boundary(&thread, false)
}
