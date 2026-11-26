//! Ordinary canonical invocation samples. Recorder diagnostics use another binary.
mod inputs;
mod scheduled;
mod shared;
use inputs::{Admission, Inputs};
use ristretto_classfile::ClassFile;
use ristretto_gc::clock::{monotonic_clock_name, monotonic_resolution_ns, monotonic_time_ns};
use ristretto_vm::canonical::{Program, Report};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    schema_version: u32,
    run_id: String,
    mode: String,
    warmup_invocations: usize,
    samples: usize,
}

#[derive(Serialize)]
struct Sample {
    elapsed_ns: Option<u64>,
    result: i32,
    allocations: usize,
    array_allocations: usize,
    released_objects: usize,
    payload_bytes_after_cleanup: usize,
    catalog_bytes_after_drop: usize,
    java_calls: usize,
    compiled_entries: usize,
    compiled_resumes: usize,
    compiled_backedges: u64,
    native_slices: usize,
    helper_polls: usize,
}

fn clock(enabled: bool) -> Result<Option<u64>> {
    if enabled {
        Ok(Some(
            monotonic_time_ns().ok_or("monotonic clock unavailable")?,
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
                .ok_or_else(|| "monotonic clock unavailable/reversed".into())
        })
        .transpose()
}

fn sample(report: &Report, admission: &Admission, elapsed_ns: Option<u64>) -> Result<Sample> {
    if report.result != admission.expected_result
        || report.payload_bytes_after_cleanup != 0
        || report.catalog_bytes_after_drop != 0
        || report.released_objects != report.allocations
        || (admission.tier == "compiled" && report.compiled_entries == 0)
        || (admission.tier == "compiled"
            && admission.require_compiled_backedges
            && report.compiled_backedges == 0)
    {
        return Err("ring checksum, cleanup or execution-tier evidence failed".into());
    }
    Ok(Sample {
        elapsed_ns,
        result: report.result,
        allocations: report.allocations,
        array_allocations: report.array_allocations,
        released_objects: report.released_objects,
        payload_bytes_after_cleanup: report.payload_bytes_after_cleanup,
        catalog_bytes_after_drop: report.catalog_bytes_after_drop,
        java_calls: report.calls,
        compiled_entries: report.compiled_entries,
        compiled_resumes: report.compiled_resumes,
        compiled_backedges: report.compiled_backedges,
        native_slices: report.native_slices,
        helper_polls: report.helper_polls,
    })
}

#[cfg(all(feature = "compiled", not(target_family = "wasm")))]
fn profiles(compiled: Option<&ristretto_vm::canonical::CompiledProgram<'_>>) -> Result<Value> {
    let Some(compiled) = compiled else {
        return Ok(Value::Null);
    };
    let result = compiled
        .method_profiles()?
        .iter()
        .map(|p| {
            json!({"class":p.class,"name":p.name,"descriptor":p.descriptor,
            "entries":p.entries,"resumes":p.resumes,"backedges":p.backedges})
        })
        .collect::<Vec<_>>();
    Ok(json!(result))
}

#[allow(
    clippy::too_many_lines,
    reason = "Keep preparation, invocation and teardown clock boundaries visible together"
)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    let [directory] = arguments.as_slice() else {
        return Err("expected capture directory".into());
    };
    let directory = PathBuf::from(directory);
    let plan_bytes = std::fs::read(directory.join("sample-plan.json"))?;
    if serde_json::from_slice::<Value>(&plan_bytes)?["schema_version"] == 2 {
        return scheduled::execute(&directory, &plan_bytes).await;
    }
    let plan: Plan = serde_json::from_slice(&plan_bytes)?;
    let measured = plan.mode == "measure";
    let warmup_check = plan.mode == "warmup-check";
    if plan.schema_version != 1
        || !matches!(plan.mode.as_str(), "check" | "warmup-check" | "measure")
        || !(1..=128).contains(&plan.samples)
        || plan.warmup_invocations > 128
        || (plan.mode == "check" && (plan.samples != 1 || plan.warmup_invocations != 0))
    {
        return Err("unsupported sample plan; checks have no timing or warmup".into());
    }
    let Inputs {
        admission,
        class_bytes,
        mut identity,
    } = Inputs::read(&directory)?;
    if admission.schema_version == 3
        || identity["run_id"] != plan.run_id
        || admission.class != "CanonicalRingWorkload"
        || !matches!(admission.method.as_str(), "small" | "medium" | "large")
        || admission.argument < 0
    {
        return Err("ring run/admission identity mismatch".into());
    }
    identity["resource_receipt_sha256"] = json!(inputs::digest(&directory.join("resources.json"))?);
    identity["sample_plan_sha256"] = json!(inputs::digest(&directory.join("sample-plan.json"))?);
    // Input hashing and executor startup precede preparation. No output inside
    // admission, compilation, warmup or any invocation sample.
    let begin = clock(measured)?;
    let classes = class_bytes
        .iter()
        .map(|bytes| ClassFile::from_bytes(bytes))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let program = Program::from_classfiles(classes)?;
    drop(class_bytes);
    let admission_ns = elapsed(begin)?;
    let begin = clock(measured)?;
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
    if admission.schema_version == 2 {
        let outcome = shared::run(
            &program,
            &admission,
            &plan,
            #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
            compiled.as_ref(),
        )
        .await?;
        let begin = clock(measured)?;
        #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
        drop(compiled);
        drop(program);
        let teardown_ns = elapsed(begin)?;
        println!(
            "RISTRETTO_CANONICAL_RING {}",
            outcome.packet(
                &identity,
                &admission,
                &plan,
                admission_ns,
                compilation_ns,
                teardown_ns
            )
        );
        return Ok(());
    }
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let before_warmup = if warmup_check {
        Value::Null
    } else {
        profiles(compiled.as_ref())?
    };
    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
    let before_warmup = Value::Null;
    let mut samples = Vec::with_capacity(if warmup_check { 0 } else { plan.samples });
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let mut before_samples = Value::Null;
    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
    let before_samples = Value::Null;
    for index in 0..plan.warmup_invocations + plan.samples {
        if index == plan.warmup_invocations && !warmup_check {
            #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
            {
                before_samples = profiles(compiled.as_ref())?;
            }
        }
        let begin = clock(measured && index >= plan.warmup_invocations)?;
        let report = if admission.tier == "interpreted" {
            program
                .run(
                    &admission.class,
                    &admission.method,
                    admission.argument,
                    admission.configuration.value(),
                    admission.limits.interpreted(),
                )
                .await?
        } else {
            #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
            {
                compiled
                    .as_ref()
                    .ok_or("compiled program missing")?
                    .run(
                        &admission.class,
                        &admission.method,
                        admission.argument,
                        admission.configuration.value(),
                        admission.limits.compiled(),
                    )
                    .await?
            }
            #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
            {
                return Err("compiled tier unavailable".into());
            }
        };
        let duration = elapsed(begin)?;
        if warmup_check {
            if report.result != admission.expected_result
                || report.payload_bytes_after_cleanup != 0
                || report.catalog_bytes_after_drop != 0
            {
                return Err("untimed warmup checksum/cleanup failed".into());
            }
            continue;
        }
        let result = sample(&report, &admission, duration)?;
        if index >= plan.warmup_invocations {
            samples.push(result);
        }
    }
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let after_samples = if warmup_check {
        Value::Null
    } else {
        profiles(compiled.as_ref())?
    };
    #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
    let after_samples = Value::Null;
    let begin = clock(measured)?;
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    drop(compiled);
    drop(program);
    let teardown_ns = elapsed(begin)?;
    if warmup_check {
        println!(
            "RISTRETTO_RING_WARMUP_CHECK {}",
            json!({
                "schema_version":1,"qualification":false,"identity":identity,
                "scope":"warmup_correctness_only","completed_warmups":plan.warmup_invocations,
                "completed_checks":plan.samples,"expected_result":admission.expected_result,
                "checksum_verified":true,"cleanup_verified":true,"timings":null,
                "allocation_counters":null,"execution_counters":null,"gc_counters":null,
                "method_profiles":null
            })
        );
        return Ok(());
    }
    println!(
        "RISTRETTO_CANONICAL_RING {}",
        json!({
            "schema_version":1,"qualification":false,"diagnostic_only":true,
            "sample_mode":plan.mode,"identity":identity,"scope":"full_invocation_v1",
            "collector_lifecycle":"dedicated_heap_setup_body_cleanup_per_invocation",
            "service_recording_enabled":false,"warmup_invocations":plan.warmup_invocations,
            "samples":samples,"clock":monotonic_clock_name(),"clock_resolution_ns":monotonic_resolution_ns(),
            "preparation":{"admission_ns":admission_ns,"compilation_ns":compilation_ns},
            "program_teardown_ns":teardown_ns,"method_profiles_before_warmup":before_warmup,
            "method_profiles_before_samples":before_samples,"method_profiles_after_samples":after_samples,
            "gc_cycles_completed":null,"global_capture_complete":false,
            "scheduled_latency_available":false,"sustainable_throughput_available":false
        })
    );
    Ok(())
}
