//! One canonical heap and attached Java context across warmup and sample calls.
#![allow(
    clippy::result_large_err,
    reason = "Propagate the canonical API error type through its with_heap callback"
)]
use super::{Admission, Plan};
use ristretto_gc::canonical::PrimaryCycles;
use ristretto_gc::clock::{monotonic_clock_name, monotonic_resolution_ns, monotonic_time_ns};
use ristretto_vm_research::canonical::{Error, Invocation, JavaThread, Program};
type Result<T> = std::result::Result<T, Error>;
use serde::Serialize;
use serde_json::{Value, json};

pub(super) trait Context {
    async fn invoke(&mut self, admission: &Admission) -> Result<Invocation>;
    fn cycles(&mut self) -> Result<PrimaryCycles>;
}
impl Context for JavaThread<'_, '_, '_> {
    async fn invoke(&mut self, admission: &Admission) -> Result<Invocation> {
        self.invoke(&admission.class, &admission.method, admission.argument)
            .await
    }
    fn cycles(&mut self) -> Result<PrimaryCycles> {
        self.primary_cycles()
    }
}
#[cfg(all(feature = "compiled", not(target_family = "wasm")))]
impl Context for ristretto_vm_research::canonical::CompiledThread<'_, '_, '_, '_> {
    async fn invoke(&mut self, admission: &Admission) -> Result<Invocation> {
        self.invoke(&admission.class, &admission.method, admission.argument)
            .await
    }
    fn cycles(&mut self) -> Result<PrimaryCycles> {
        self.primary_cycles()
    }
}
fn clock(enabled: bool) -> Result<Option<u64>> {
    if enabled {
        Ok(Some(
            monotonic_time_ns().ok_or(Error::Boundary("benchmark clock unavailable"))?,
        ))
    } else {
        Ok(None)
    }
}
fn elapsed(start: Option<u64>) -> Result<Option<u64>> {
    start
        .map(|start| {
            monotonic_time_ns()
                .and_then(|end| end.checked_sub(start))
                .ok_or(Error::Boundary("benchmark clock unavailable/reversed"))
        })
        .transpose()
}
fn add(total: &mut usize, value: usize) -> Result<()> {
    *total = total
        .checked_add(value)
        .ok_or(Error::Boundary("benchmark count overflow"))?;
    Ok(())
}
#[derive(Serialize)]
struct Sample {
    elapsed_ns: Option<u64>,
    start_clock_ns: Option<u64>,
    end_clock_ns: Option<u64>,
    result: i32,
    allocations: usize,
    released_objects: usize,
    helper_polls: usize,
    native_slices: usize,
}
#[derive(Serialize)]
struct Cycles {
    lower: u64,
    upper: u64,
}
impl From<PrimaryCycles> for Cycles {
    fn from(value: PrimaryCycles) -> Self {
        Self {
            lower: value.lower,
            upper: value.upper,
        }
    }
}
#[derive(Serialize)]
pub(super) struct CycleBatch {
    scope: &'static str,
    before_start: Cycles,
    after_start: Cycles,
    before_end: Cycles,
    after_end: Cycles,
    start_clock_ns: Option<u64>,
    end_clock_ns: Option<u64>,
    elapsed_ns: Option<u64>,
    completed_primary_cycles: Cycles,
}
pub(super) fn cycle_batch(
    values: [PrimaryCycles; 4],
    start: Option<u64>,
    end: Option<u64>,
) -> Result<CycleBatch> {
    let mut minimum = 0;
    for value in values {
        minimum = minimum.max(value.lower);
        if value.lower > value.upper || minimum > value.upper {
            return Err(Error::Boundary("inconsistent primary-cycle observations"));
        }
    }
    let duration = match (start, end) {
        (Some(start), Some(end)) => Some(
            end.checked_sub(start)
                .ok_or(Error::Boundary("batch clock reversed"))?,
        ),
        (None, None) => None,
        _ => return Err(Error::Boundary("partial batch clock")),
    };
    Ok(CycleBatch {
        scope: "sample_batch_including_driver_gaps_and_observer",
        before_start: values[0].into(),
        after_start: values[1].into(),
        before_end: values[2].into(),
        after_end: values[3].into(),
        start_clock_ns: start,
        end_clock_ns: end,
        elapsed_ns: duration,
        completed_primary_cycles: Cycles {
            lower: values[2].lower.saturating_sub(values[1].upper),
            upper: values[3]
                .upper
                .checked_sub(values[0].lower)
                .ok_or(Error::Boundary("cycle upper reversed"))?,
        },
    })
}
struct LoopResult {
    samples: Vec<Sample>,
    before_warmup: Value,
    before_samples: Value,
    after_samples: Value,
    batch: Option<CycleBatch>,
    allocations: usize,
    released: usize,
    warmup_released: usize,
    warmup_allocations: usize,
    warmup_helper_polls: usize,
    warmup_native_slices: usize,
    setup_ns: Option<u64>,
    cleanup_start: Option<u64>,
    payload_before_cleanup: usize,
}
async fn calls(
    context: &mut impl Context,
    admission: &Admission,
    plan: &Plan,
    mut samples: Vec<Sample>,
    profiles: impl Fn() -> Result<Value>,
) -> Result<LoopResult> {
    let measured = plan.mode == "measure";
    let observed = plan.mode != "warmup-check";
    let mut result = LoopResult {
        samples: Vec::new(),
        before_warmup: Value::Null,
        before_samples: Value::Null,
        after_samples: Value::Null,
        batch: None,
        allocations: 0,
        released: 0,
        warmup_released: 0,
        warmup_allocations: 0,
        warmup_helper_polls: 0,
        warmup_native_slices: 0,
        setup_ns: None,
        cleanup_start: None,
        payload_before_cleanup: 0,
    };
    if observed {
        result.before_warmup = profiles()?;
    }
    for _ in 0..plan.warmup_invocations {
        let call = context.invoke(admission).await?;
        if call.result != admission.expected_result {
            return Err(Error::Boundary("warmup checksum"));
        }
        add(&mut result.warmup_allocations, call.allocations)?;
        add(&mut result.warmup_released, call.released_objects)?;
        add(&mut result.warmup_helper_polls, call.helper_polls)?;
        add(&mut result.warmup_native_slices, call.native_slices)?;
    }
    if observed {
        result.before_samples = profiles()?;
    }
    // Four passive observations bound the batch. The two inner observations and
    // driver gaps belong to this batch clock, never to an invocation timer.
    let before_start = observed.then(|| context.cycles()).transpose()?;
    let start = clock(measured)?;
    let after_start = observed.then(|| context.cycles()).transpose()?;
    for _ in 0..plan.samples {
        let begin = clock(measured)?;
        let call = context.invoke(admission).await?;
        let finish = clock(measured)?;
        let duration = match (begin, finish) {
            (Some(start), Some(end)) => Some(
                end.checked_sub(start)
                    .ok_or(Error::Boundary("invocation clock reversed"))?,
            ),
            (None, None) => None,
            _ => return Err(Error::Boundary("partial invocation clock")),
        };
        if call.result != admission.expected_result {
            return Err(Error::Boundary("sample checksum"));
        }
        add(&mut result.allocations, call.allocations)?;
        add(&mut result.released, call.released_objects)?;
        if observed {
            samples.push(Sample {
                elapsed_ns: duration,
                start_clock_ns: begin,
                end_clock_ns: finish,
                result: call.result,
                allocations: call.allocations,
                released_objects: call.released_objects,
                helper_polls: call.helper_polls,
                native_slices: call.native_slices,
            });
        }
    }
    let before_end = observed.then(|| context.cycles()).transpose()?;
    let end = clock(measured)?;
    let after_end = observed.then(|| context.cycles()).transpose()?;
    if let (Some(a), Some(b), Some(c), Some(d)) = (before_start, after_start, before_end, after_end)
    {
        let batch = cycle_batch([a, b, c, d], start, end)?;
        if batch.completed_primary_cycles.lower
            < admission
                .minimum_primary_cycles
                .ok_or(Error::Boundary("missing primary-cycle requirement"))?
        {
            return Err(Error::Boundary(
                "sample batch primary-cycle exposure absent",
            ));
        }
        result.batch = Some(batch);
    }
    if observed {
        result.after_samples = profiles()?;
    }
    result.samples = samples;
    Ok(result)
}

pub(super) struct Outcome {
    data: LoopResult,
    cleanup_polls: usize,
    released_during_cleanup: usize,
    cleanup_ns: Option<u64>,
    reserved_catalog_bytes: usize,
}
#[allow(
    clippy::too_many_lines,
    reason = "Keep lifetime and clock boundaries visible"
)]
#[cfg_attr(
    not(all(feature = "compiled", not(target_family = "wasm"))),
    allow(
        clippy::needless_lifetimes,
        reason = "Native compiled context shares this program lifetime"
    )
)]
pub(super) async fn run<'p>(
    program: &'p Program,
    admission: &Admission,
    plan: &Plan,
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))] compiled: Option<
        &ristretto_vm_research::canonical::CompiledProgram<'p>,
    >,
) -> Result<Outcome> {
    let measured = plan.mode == "measure";
    let storage = Vec::with_capacity(if plan.mode == "warmup-check" {
        0
    } else {
        plan.samples
    });
    let configuration = admission.configuration.value();
    let reserved_catalog_bytes = configuration.charged_bytes()?;
    let max_payload_bytes = admission
        .max_payload_bytes
        .ok_or(Error::Boundary("missing payload ceiling"))?;
    let cleanup_limit = admission
        .heap_cleanup_polls
        .ok_or(Error::Boundary("missing cleanup limit"))?;
    let setup_start = clock(measured)?;
    let completed = program
        .with_heap(
            configuration,
            max_payload_bytes,
            cleanup_limit,
            async |heap| {
                let profile = || {
                    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
                    {
                        super::profiles(compiled)
                            .map_err(|_| Error::Boundary("compiled census unavailable"))
                    }
                    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
                    {
                        Ok(Value::Null)
                    }
                };
                let data = if admission.tier == "interpreted" {
                    let mut context = heap.attach(admission.limits.interpreted())?;
                    let setup_ns = elapsed(setup_start)?;
                    let mut data = calls(&mut context, admission, plan, storage, profile).await?;
                    data.setup_ns = setup_ns;
                    data.payload_before_cleanup = heap.managed_bytes();
                    data.cleanup_start = clock(measured)?;
                    drop(context);
                    data
                } else {
                    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
                    {
                        let mut context = compiled
                            .ok_or(Error::Boundary("missing compiled program"))?
                            .attach(heap, admission.limits.compiled())?;
                        let setup_ns = elapsed(setup_start)?;
                        let mut data =
                            calls(&mut context, admission, plan, storage, profile).await?;
                        data.setup_ns = setup_ns;
                        data.payload_before_cleanup = heap.managed_bytes();
                        data.cleanup_start = clock(measured)?;
                        drop(context);
                        data
                    }
                    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
                    {
                        return Err(Error::Boundary("compiled tier unavailable"));
                    }
                };
                Ok(data)
            },
        )
        .await?;
    let cleanup_ns = elapsed(completed.value.cleanup_start)?;
    let allocations = completed
        .value
        .allocations
        .checked_add(completed.value.warmup_allocations);
    let released = completed
        .value
        .released
        .checked_add(completed.value.warmup_released)
        .and_then(|value| value.checked_add(completed.released_during_cleanup));
    if allocations.is_none() || allocations != released {
        return Err(Error::Boundary("whole-heap release balance"));
    }
    Ok(Outcome {
        data: completed.value,
        cleanup_polls: completed.cleanup_polls,
        released_during_cleanup: completed.released_during_cleanup,
        cleanup_ns,
        reserved_catalog_bytes,
    })
}
impl Outcome {
    pub(super) fn packet(
        self,
        identity: &Value,
        admission: &Admission,
        plan: &Plan,
        admission_ns: Option<u64>,
        compilation_ns: Option<u64>,
        teardown_ns: Option<u64>,
    ) -> Value {
        let observed = plan.mode != "warmup-check";
        json!({"schema_version":2,"qualification":false,"diagnostic_only":true,
            "sample_mode":plan.mode,"identity":identity,"scope":"shared_heap_invocation_v1",
            "collector_lifecycle":"one_heap_and_context_across_warmup_and_samples",
            "service_recording_enabled":false,"warmup_invocations":plan.warmup_invocations,
            "completed_samples":plan.samples,"expected_result":admission.expected_result,
            "checksum_verified":true,"heap_cleanup_verified":true,
            "samples":if observed { Some(self.data.samples) } else { None },
            "clock":monotonic_clock_name(),"clock_resolution_ns":monotonic_resolution_ns(),
            "preparation":{"admission_ns":admission_ns,"compilation_ns":compilation_ns,
                "heap_setup_and_attach_ns":self.data.setup_ns},
            "heap_detach_cleanup_and_drop_ns":self.cleanup_ns,"program_teardown_ns":teardown_ns,
            "method_profiles_before_warmup":self.data.before_warmup,
            "method_profiles_before_samples":self.data.before_samples,"method_profiles_after_samples":self.data.after_samples,
            "primary_cycle_batch":self.data.batch,
            "heap":{"max_payload_bytes":admission.max_payload_bytes,"reserved_catalog_bytes":self.reserved_catalog_bytes,
                "cleanup_limit":admission.heap_cleanup_polls,
                "payload_bytes_before_cleanup":if observed {Some(self.data.payload_before_cleanup)} else {None}},
            "workload_counts":if observed { Some(json!({
                "warmup_allocations":self.data.warmup_allocations,"warmup_released_objects":self.data.warmup_released,
                "warmup_helper_polls":self.data.warmup_helper_polls,"warmup_native_slices":self.data.warmup_native_slices,
                "sample_allocations":self.data.allocations,"sample_released_objects":self.data.released,
                "cleanup_polls":self.cleanup_polls,"released_during_cleanup":self.released_during_cleanup
            })) } else {None},
            "in_invocation_gc_exposure_proven":false,"global_capture_complete":false,
            "scheduled_latency_available":false,"sustainable_throughput_available":false})
    }
}
