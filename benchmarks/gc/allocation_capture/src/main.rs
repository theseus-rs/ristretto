//! Allocation diagnostics are a separate binary, never a timing comparator.
mod counting;
mod inputs;
use counting::{Counts, Snapshot};
use inputs::{Admission, Inputs};
use ristretto_classfile::ClassFile;
use ristretto_vm_research::canonical::{Program, Report};
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize)]
struct Stage {
    phase: &'static str,
    sample: Option<usize>,
    counts: Option<Counts>,
    result: Option<i32>,
    payload_allocations: Option<usize>,
    array_allocations: Option<usize>,
    released_objects: Option<usize>,
    payload_bytes_after_cleanup: Option<usize>,
    catalog_bytes_after_drop: Option<usize>,
    compiled_entries: Option<usize>,
    compiled_backedges: Option<u64>,
    java_calls: Option<usize>,
    helper_polls: Option<usize>,
}

fn finish(
    phase: &'static str,
    sample: Option<usize>,
    before: Option<Snapshot>,
    report: Option<&Report>,
) -> Result<Stage> {
    // Sample before report validation/serialization; all report fields are scalars.
    let counts = before.map(counting::end).transpose()?;
    Ok(Stage {
        phase,
        sample,
        counts,
        result: report.map(|value| value.result),
        payload_allocations: report.map(|value| value.allocations),
        array_allocations: report.map(|value| value.array_allocations),
        released_objects: report.map(|value| value.released_objects),
        payload_bytes_after_cleanup: report.map(|value| value.payload_bytes_after_cleanup),
        catalog_bytes_after_drop: report.map(|value| value.catalog_bytes_after_drop),
        compiled_entries: report.map(|value| value.compiled_entries),
        compiled_backedges: report.map(|value| value.compiled_backedges),
        java_calls: report.map(|value| value.calls),
        helper_polls: report.map(|value| value.helper_polls),
    })
}

fn validate(report: &Report, admission: &Admission) -> Result<()> {
    if report.result != admission.expected_result
        || report.payload_bytes_after_cleanup != 0
        || report.catalog_bytes_after_drop != 0
        || report.released_objects != report.allocations
        || (admission.tier == "compiled" && report.compiled_entries == 0)
        || (admission.tier == "compiled"
            && admission.require_compiled_backedges
            && report.compiled_backedges == 0)
    {
        return Err("checksum, execution, or complete cleanup validation failed".into());
    }
    Ok(())
}

fn arguments() -> Result<(PathBuf, usize, bool)> {
    let mut arguments = std::env::args_os().skip(1);
    let directory = arguments
        .next()
        .ok_or("expected input directory [--check] [--samples N]")?;
    let mut check = false;
    let mut samples = 1;
    let mut supplied_samples = false;
    while let Some(argument) = arguments.next() {
        if argument == "--check" && !check {
            check = true;
        } else if argument == "--samples" && !supplied_samples {
            samples = arguments
                .next()
                .ok_or("missing sample count")?
                .to_str()
                .ok_or("non-UTF8 sample count")?
                .parse()?;
            supplied_samples = true;
        } else {
            return Err("unknown or repeated option".into());
        }
    }
    if !(1..=64).contains(&samples) {
        return Err("sample count must be in 1..=64".into());
    }
    Ok((PathBuf::from(directory), samples, check))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let (directory, samples, check) = arguments()?;
    if check {
        counting::disable();
    }
    let inputs = Inputs::read(&directory)?;
    let admission = &inputs.admission;
    // File reads, input hashing, executor startup and output capacity are outside
    // counted phases. The exact bytes below are the bytes hashed by Inputs.
    let mut stages = Vec::with_capacity(samples + 3);
    let begin = || (!check).then(counting::begin);
    let before = begin();
    let classes = inputs
        .class_bytes
        .iter()
        .map(|bytes| ClassFile::from_bytes(bytes))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let program = Program::from_classfiles(classes)?;
    stages.push(finish("admission", None, before, None)?);

    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let compiled = if admission.tier == "compiled" {
        let before = begin();
        let compiled = program.compile()?;
        stages.push(finish("compilation", None, before, None)?);
        Some(compiled)
    } else {
        None
    };

    for sample in 0..samples {
        let before = begin();
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
                return Err("native compiled tier unavailable".into());
            }
        };
        let stage = finish("full_invocation", Some(sample), before, Some(&report))?;
        validate(&report, admission)?;
        stages.push(stage);
    }
    let before = begin();
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    drop(compiled);
    drop(program);
    stages.push(finish("program_teardown", None, before, None)?);

    // First and only output, after all measured spans and cleanup checks.
    let packet = json!({
        "schema_version": 1,
        "diagnostic_kind": "canonical_process_global_allocator",
        "check_only": check,
        "measurement": !check,
        "qualification": false,
        "timing_comparable": false,
        "target": if cfg!(target_family = "wasm") { "wasi" } else { "native" },
        "admission": admission,
        "admission_sha256": inputs.admission_sha256,
        "classes": inputs.classes,
        "class_manifest_sha256": inputs.class_manifest_sha256,
        "samples": samples,
        "invocation_window_kind": "full_invocation_v1",
        "warmup_invocations": 0,
        "stages": stages,
        "scope": "Rust global allocator across this process; requested sizes, not RSS or payload-only accounting",
        "excluded": ["input file I/O and hashing", "executor startup", "JSON/output", "direct native allocator calls", "JIT executable mappings", "allocator metadata and usable-size rounding", "WASI host allocations"],
        "snapshot_contract": "sequential stage boundaries in a current-thread driver; background allocator traffic is included, and counter snapshots are not a cross-thread atomic transaction",
        "realloc_contract": "one request; on success one old-size release and new-size allocation, even in place; hidden physical overlap is excluded",
        "invocation_scope": "Program::run or CompiledProgram::run including heap setup, frame setup, entry execution, cleanup, runtime drop and profile/report construction",
        "artifact_identity": "external build and launch receipts required; this packet binds only consumed admission/class inputs"
    });
    println!("RISTRETTO_CANONICAL_ALLOCATIONS {packet}");
    Ok(())
}
