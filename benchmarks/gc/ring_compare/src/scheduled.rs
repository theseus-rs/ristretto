//! Fixed arrivals on one persistent heap. Bounded histograms, no per-call trace.
#![allow(
    clippy::result_large_err,
    reason = "Propagate canonical with_heap callback errors"
)]
use super::{
    Admission, Inputs,
    shared::{Context, cycle_batch},
};
use ristretto_classfile::ClassFile;
use ristretto_gc::clock::{monotonic_clock_name, monotonic_resolution_ns, monotonic_time_ns};
use ristretto_vm_research::canonical::{Error, Program};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;
type Result<T> = std::result::Result<T, Error>;
const SECOND: u64 = 1_000_000_000;
const BINS: usize = 4096;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    schema_version: u32,
    run_id: String,
    mode: String,
    warmup_invocations: usize,
    duration_seconds: u64,
    arrival_rate: u64,
    max_completions: u64,
}
impl Plan {
    fn validate(&self) -> Result<()> {
        if self.schema_version != 2
            || self.run_id.is_empty()
            || !matches!(self.mode.as_str(), "scheduled-check" | "scheduled-measure")
            || self.warmup_invocations > 128
            || !(1..=86400).contains(&self.duration_seconds)
            || !(1..=SECOND).contains(&self.arrival_rate)
            || !(1..=SECOND).contains(&self.max_completions)
            || (self.mode == "scheduled-check"
                && (self.max_completions != 3 || self.warmup_invocations != 0))
        {
            return Err(Error::Boundary("invalid fixed-arrival plan"));
        }
        Ok(())
    }
    fn measured(&self) -> bool {
        self.mode == "scheduled-measure"
    }
}
fn clock() -> Result<u64> {
    monotonic_time_ns().ok_or(Error::Boundary("scheduled clock unavailable"))
}
fn clock_if(enabled: bool) -> Result<Option<u64>> {
    enabled.then(clock).transpose()
}
fn elapsed(begin: Option<u64>) -> Result<Option<u64>> {
    begin
        .map(|start| {
            clock()?
                .checked_sub(start)
                .ok_or(Error::Boundary("clock reversed"))
        })
        .transpose()
}
fn due(index: u64, rate: u64) -> u64 {
    index / rate * SECOND + index % rate * SECOND / rate
}
/// Number of floor(index * SECOND / rate) release times at or before offset.
/// The +1 nanosecond handles non-divisor rates exactly, without consumer input.
fn arrived(offset: u64, rate: u64, offered: u64) -> u64 {
    let next = offset + 1;
    (next / SECOND * rate + (next % SECOND * rate).div_ceil(SECOND)).min(offered)
}
fn wait_until(deadline: u64) -> Result<u64> {
    loop {
        let now = clock()?;
        if now >= deadline {
            return Ok(now);
        }
        let remaining = deadline - now;
        if remaining >= 1_000_000 {
            std::thread::sleep(std::time::Duration::from_millis(remaining / 1_000_000));
        } else {
            std::thread::yield_now();
        }
    }
}
struct Histogram {
    counts: Box<[u64]>,
    max_ns: u64,
}
impl Histogram {
    fn new() -> Self {
        Self {
            counts: vec![0; BINS].into_boxed_slice(),
            max_ns: 0,
        }
    }
    fn record(&mut self, ns: u64) -> Result<()> {
        let us = ns.div_ceil(1000);
        let index = if us <= 1024 {
            us
        } else {
            let exponent = us.ilog2();
            let width = 1_u64 << (exponent - 6);
            1024 + u64::from(exponent - 10) * 64 + (us - (1_u64 << exponent)).div_ceil(width)
        };
        let slot = self
            .counts
            .get_mut(usize::try_from(index).map_err(|_| Error::Boundary("histogram index"))?)
            .ok_or(Error::Boundary("histogram range exceeded"))?;
        *slot += 1;
        self.max_ns = self.max_ns.max(ns);
        Ok(())
    }
    fn packet(&self) -> Value {
        let rows = self
            .counts
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .map(|(i, count)| {
                let upper = if i <= 1024 {
                    i as u64
                } else {
                    let relative = i - 1024;
                    let exponent = 10 + relative / 64;
                    (1_u64 << exponent) + (relative % 64) as u64 * (1_u64 << (exponent - 6))
                };
                [upper, *count]
            })
            .collect::<Vec<_>>();
        json!({"max_ns":self.max_ns,"buckets_us":rows})
    }
}
#[derive(Default, Serialize)]
struct Counts {
    allocations: usize,
    released_objects: usize,
    helper_polls: usize,
    native_slices: usize,
}
impl Counts {
    fn add(&mut self, value: &ristretto_vm_research::canonical::Invocation) -> Result<()> {
        for (total, value) in [
            (&mut self.allocations, value.allocations),
            (&mut self.released_objects, value.released_objects),
            (&mut self.helper_polls, value.helper_polls),
            (&mut self.native_slices, value.native_slices),
        ] {
            *total = total
                .checked_add(value)
                .ok_or(Error::Boundary("scheduled counter overflow"))?;
        }
        Ok(())
    }
}
struct Data {
    packet: Value,
    warmup: Counts,
    sample: Counts,
    setup_ns: Option<u64>,
    cleanup_start: Option<u64>,
    payload: usize,
}
#[allow(
    clippy::too_many_lines,
    reason = "Keep release, deadline and completion accounting adjacent"
)]
async fn calls(
    context: &mut impl Context,
    admission: &Admission,
    plan: &Plan,
    profiles: impl Fn() -> Result<Value>,
    mut service: Histogram,
    mut response: Histogram,
    mut timeline: Vec<[u64; 3]>,
) -> Result<Data> {
    let measured = plan.measured();
    let before_warmup = profiles()?;
    let mut warmup = Counts::default();
    for _ in 0..plan.warmup_invocations {
        let value = context.invoke(admission).await?;
        if value.result != admission.expected_result {
            return Err(Error::Boundary("warmup checksum"));
        }
        warmup.add(&value)?;
    }
    let before_samples = profiles()?;
    let a = context.cycles()?;
    let start = clock_if(measured)?;
    let b = context.cycles()?;
    let duration = plan.duration_seconds * SECOND;
    let offered = plan.arrival_rate * plan.duration_seconds;
    let deadline = start
        .map(|v| {
            v.checked_add(duration)
                .ok_or(Error::Boundary("deadline overflow"))
        })
        .transpose()?;
    let mut completed = 0;
    let mut on_time = 0;
    let mut peak_backlog = 0;
    let mut sample = Counts::default();
    let mut checksum = 0_i64;
    let mut cap_reached = false;
    loop {
        if !measured && completed == 3 {
            break;
        }
        let mut begin = None;
        let mut release = None;
        if let (Some(start), Some(deadline)) = (start, deadline) {
            let now = clock()?;
            if now >= deadline || completed == offered {
                break;
            }
            if completed == plan.max_completions {
                cap_reached = true;
                break;
            }
            let due = start
                .checked_add(due(completed, plan.arrival_rate))
                .ok_or(Error::Boundary("release overflow"))?;
            let now = wait_until(due)?;
            if now >= deadline {
                break;
            }
            peak_backlog = peak_backlog.max(
                arrived(now - start, plan.arrival_rate, offered).saturating_sub(completed + 1),
            );
            begin = Some(now);
            release = Some(due);
        }
        let value = context.invoke(admission).await?;
        let finish = clock_if(measured)?;
        if value.result != admission.expected_result {
            return Err(Error::Boundary("scheduled checksum"));
        }
        sample.add(&value)?;
        completed += 1;
        checksum += i64::from(value.result);
        if let (Some(start), Some(begin), Some(release), Some(finish), Some(deadline)) =
            (start, begin, release, finish, deadline)
        {
            service.record(
                finish
                    .checked_sub(begin)
                    .ok_or(Error::Boundary("service clock reversed"))?,
            )?;
            response.record(
                finish
                    .checked_sub(release)
                    .ok_or(Error::Boundary("response clock reversed"))?,
            )?;
            if finish <= deadline {
                on_time += 1;
            }
            let backlog =
                arrived(finish - start, plan.arrival_rate, offered).saturating_sub(completed);
            peak_backlog = peak_backlog.max(backlog);
            if timeline.len() < 128 && finish - start >= duration / 128 * timeline.len() as u64 {
                timeline.push([finish - start, completed, backlog]);
            }
        }
    }
    if measured {
        // Keep a fixed offer horizon, including the idle gap after the final due request.
        // A cap is an invalid observation: still account every predeclared arrival.
        wait_until(deadline.ok_or(Error::Boundary("missing deadline"))?)?;
        peak_backlog = peak_backlog.max(offered - completed);
    }
    let c = context.cycles()?;
    let end = clock_if(measured)?;
    let d = context.cycles()?;
    let batch = cycle_batch([a, b, c, d], start, end)?;
    let batch = serde_json::to_value(batch).map_err(|_| Error::Boundary("cycle serialization"))?;
    if batch["completed_primary_cycles"]["lower"]
        .as_u64()
        .unwrap_or(0)
        < 1
    {
        return Err(Error::Boundary("no primary-cycle exposure"));
    }
    let after_samples = profiles()?;
    let stats = if measured {
        json!({"duration_ns":duration,"start_clock_ns":start,"end_clock_ns":end,
        "deadline_clock_ns":deadline,"offered":offered,"completed":completed,"deadline_completed":on_time,
        "deadline_backlog":offered-on_time,"after_deadline_completions":completed-on_time,
        "unfinished":offered-completed,"peak_backlog":peak_backlog,"completion_cap_reached":cap_reached,
        "status":if cap_reached {"completion_cap"} else if completed < offered {"unfinished"} else {"complete"},
        "service":service.packet(),"response":response.packet(),"timeline":timeline})
    } else {
        Value::Null
    };
    Ok(Data {
        packet: json!({"completed":completed,"checksum":checksum,"schedule":stats,
        "method_profiles_before_warmup":before_warmup,"method_profiles_before_samples":before_samples,
        "method_profiles_after_samples":after_samples,"primary_cycle_batch":batch}),
        warmup,
        sample,
        setup_ns: None,
        cleanup_start: None,
        payload: 0,
    })
}

#[allow(
    clippy::too_many_lines,
    reason = "Keep persistent heap construction and drain boundaries explicit"
)]
pub(super) async fn execute(directory: &Path, bytes: &[u8]) -> super::Result {
    let plan: Plan = serde_json::from_slice(bytes)?;
    plan.validate()?;
    let Inputs {
        admission,
        class_bytes,
        mut identity,
    } = Inputs::read(directory)?;
    if admission.schema_version != 3 || identity["run_id"] != plan.run_id {
        return Err("scheduled admission/plan identity mismatch".into());
    }
    identity["resource_receipt_sha256"] =
        json!(super::inputs::digest(&directory.join("resources.json"))?);
    identity["sample_plan_sha256"] =
        json!(super::inputs::digest(&directory.join("sample-plan.json"))?);
    let measured = plan.measured();
    let begin = clock_if(measured)?;
    let classes = class_bytes
        .iter()
        .map(|v| ClassFile::from_bytes(v))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let program = Program::from_classfiles(classes)?;
    drop(class_bytes);
    let admission_ns = elapsed(begin)?;
    let begin = clock_if(measured)?;
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let compiled = if admission.tier == "compiled" {
        Some(program.compile()?)
    } else {
        None
    };
    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
    if admission.tier != "interpreted" {
        return Err("compiled tier unavailable".into());
    }
    let compilation_ns = if admission.tier == "compiled" {
        elapsed(begin)?
    } else {
        None
    };
    // Constant observer storage is allocated before heap setup and the offer clock.
    let service = Histogram::new();
    let response = Histogram::new();
    let timeline = Vec::with_capacity(128);
    let configuration = admission.configuration.value();
    let catalog = configuration.charged_bytes()?;
    let ceiling = admission
        .max_payload_bytes
        .ok_or("payload ceiling missing")?;
    let cleanup = admission
        .heap_cleanup_polls
        .ok_or("cleanup limit missing")?;
    let setup_start = clock_if(measured)?;
    let finished = program
        .with_heap(configuration, ceiling, cleanup, async |heap| {
            let profiles = || {
                #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
                {
                    super::profiles(compiled.as_ref())
                        .map_err(|_| Error::Boundary("method census unavailable"))
                }
                #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
                {
                    Ok(Value::Null)
                }
            };
            let data = if admission.tier == "interpreted" {
                let mut context = heap.attach(admission.limits.interpreted())?;
                let setup = elapsed(setup_start)?;
                let mut data = calls(
                    &mut context,
                    &admission,
                    &plan,
                    profiles,
                    service,
                    response,
                    timeline,
                )
                .await?;
                data.setup_ns = setup;
                data.payload = heap.managed_bytes();
                data.cleanup_start = clock_if(measured)?;
                drop(context);
                data
            } else {
                #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
                {
                    let mut context = compiled
                        .as_ref()
                        .ok_or(Error::Boundary("missing compiled program"))?
                        .attach(heap, admission.limits.compiled())?;
                    let setup = elapsed(setup_start)?;
                    let mut data = calls(
                        &mut context,
                        &admission,
                        &plan,
                        profiles,
                        service,
                        response,
                        timeline,
                    )
                    .await?;
                    data.setup_ns = setup;
                    data.payload = heap.managed_bytes();
                    data.cleanup_start = clock_if(measured)?;
                    drop(context);
                    data
                }
                #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
                {
                    return Err(Error::Boundary("compiled tier unavailable"));
                }
            };
            Ok(data)
        })
        .await?;
    let cleanup_ns = elapsed(finished.value.cleanup_start)?;
    let data = finished.value;
    if data.warmup.allocations.checked_add(data.sample.allocations)
        != data
            .warmup
            .released_objects
            .checked_add(data.sample.released_objects)
            .and_then(|v| v.checked_add(finished.released_during_cleanup))
    {
        return Err("whole workload release balance".into());
    }
    let begin = clock_if(measured)?;
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    drop(compiled);
    drop(program);
    let teardown_ns = elapsed(begin)?;
    println!(
        "RISTRETTO_RING_SCHEDULED {}",
        json!({"schema_version":1,"qualification":false,
        "identity":identity,"sample_mode":plan.mode,"scope":"shared_heap_scheduled_v1",
        "collector_lifecycle":"one_heap_and_context_across_warmup_and_schedule",
        "arrival_policy":"fixed_index_release_logical_fifo_v1","warmup_invocations":plan.warmup_invocations,
        "clock":monotonic_clock_name(),"clock_resolution_ns":monotonic_resolution_ns(),
        "expected_result":admission.expected_result,"checksum_verified":true,"heap_cleanup_verified":true,
        "data":data.packet,"workload_counts":{"warmup":data.warmup,"sample":data.sample,
            "cleanup_polls":finished.cleanup_polls,"released_during_cleanup":finished.released_during_cleanup},
        "heap":{"max_payload_bytes":ceiling,"reserved_catalog_bytes":catalog,"cleanup_limit":cleanup,"payload_bytes_before_cleanup":data.payload},
        "preparation":{"admission_ns":admission_ns,"compilation_ns":compilation_ns,"heap_setup_and_attach_ns":data.setup_ns},
        "heap_detach_cleanup_and_drop_ns":cleanup_ns,"program_teardown_ns":teardown_ns,
        "global_capture_complete":false,"sustainable_throughput_available":false})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn releases_count_without_consumer_progress() {
        for rate in [1, 3, 7, 999, 1001, 1_000_000_000] {
            for index in [0, 1, 2, 4, 10, 99] {
                let at = due(index, rate);
                assert_eq!(arrived(at, rate, 1_000), index + 1);
                if at > 0 {
                    assert_eq!(arrived(at - 1, rate, 1_000), index);
                }
            }
        }
        assert_eq!(arrived(100 * SECOND, 7, 21), 21);
    }
    #[test]
    fn bounded_histogram_covers_bin_edges() -> Result<()> {
        let mut h = Histogram::new();
        let ns = [
            0,
            1,
            1_024_000,
            1_024_001,
            2_048_000,
            2_048_001,
            86400 * SECOND,
        ];
        for value in ns {
            h.record(value)?;
        }
        let packet = h.packet();
        let rows = packet["buckets_us"]
            .as_array()
            .ok_or(Error::Boundary("missing bins"))?;
        assert_eq!(
            rows.iter().filter_map(|r| r[1].as_u64()).sum::<u64>(),
            ns.len() as u64
        );
        assert_eq!(packet["max_ns"], 86400 * SECOND);
        Ok(())
    }
}
