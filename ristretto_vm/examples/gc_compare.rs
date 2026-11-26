//! Execute the shared GC comparison workload with an explicit collector policy.
//! Arguments: JDK home, class directory, heap MiB, workers, baseline|candidate[-jit],
//! followed by `GcBench` arguments. Native and WASI use the same entry point.
#![forbid(unsafe_code)]
#![expect(clippy::print_stdout, reason = "benchmark machine-readable output")]

use ristretto_gc::{Configuration, GarbageCollector, RootStorage};
use ristretto_vm::{ClassPath, ConfigurationBuilder, VM};
use std::path::PathBuf;

#[cfg(all(
    feature = "experimental-pause-recorder",
    target_os = "linux",
    target_has_atomic = "64"
))]
#[path = "support/pause_capture.rs"]
mod pause_capture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    let capture = pause_capture::Capture::new()?;
    #[cfg(not(target_family = "wasm"))]
    let mut runtime = if std::env::args()
        .nth(5)
        .is_some_and(|policy| policy.ends_with("-jit"))
    {
        // The synchronous compiled ABI needs block_in_place for async helpers.
        // Keep executor concurrency explicit; all threads share the process quota.
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.worker_threads(1);
        builder
    } else {
        tokio::runtime::Builder::new_current_thread()
    };
    #[cfg(target_family = "wasm")]
    let mut runtime = tokio::runtime::Builder::new_current_thread();
    let runtime = runtime.enable_all().build()?;
    let result = runtime.block_on(run(
        #[cfg(all(
            feature = "experimental-pause-recorder",
            target_os = "linux",
            target_has_atomic = "64"
        ))]
        capture.recorder(),
        #[cfg(all(
            feature = "experimental-execution-measurement",
            target_os = "linux",
            target_has_atomic = "64"
        ))]
        capture.identity(),
    ));
    drop(runtime);
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    capture.finish(result.is_ok())?;
    result
}

#[expect(
    clippy::too_many_lines,
    reason = "explicit benchmark runtime setup and evidence export"
)]
async fn run(
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    recorder: std::sync::Arc<ristretto_gc::pause_recorder::PauseRecorder>,
    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    execution_identity: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [
        home,
        classes,
        heap,
        workers,
        policy,
        workload,
        duration,
        warmup,
        rate,
        seed,
    ] = args.as_slice()
    else {
        return Err("expected: JDK classes heap-MiB workers baseline|candidate[-jit] workload duration-ms warmup-ms rate seed".into());
    };
    let heap = heap
        .parse::<usize>()?
        .checked_mul(1024 * 1024)
        .ok_or("heap overflow")?;
    let workers = workers.parse::<usize>()?;
    if cfg!(target_family = "wasm") && workers != 0 {
        return Err("wasm requires zero native workers".into());
    }
    let (policy, jit_enabled) = policy
        .strip_suffix("-jit")
        .map_or((policy.as_str(), false), |policy| (policy, true));
    if jit_enabled && cfg!(target_family = "wasm") {
        return Err("JIT comparison is native only".into());
    }
    let candidate = match policy {
        "baseline" => false,
        "candidate" => true,
        _ => return Err("unknown collector policy".into()),
    };
    let gc_configuration = collector_configuration(heap, workers, candidate);
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    let gc_configuration = Configuration {
        pause_recorder: Some(recorder),
        ..gc_configuration
    };
    let gc = GarbageCollector::with_config(gc_configuration);
    let configuration = ConfigurationBuilder::new()
        .java_home(PathBuf::from(home))
        .class_path(ClassPath::from(&[PathBuf::from(classes)]))
        .main_class("GcBench")
        .interpreted(!jit_enabled)
        .garbage_collector(gc.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    if vm.is_jit_enabled() != jit_enabled {
        return Err(
            "requested execution mode is unavailable; refusing a mislabeled comparison".into(),
        );
    }
    if workers == 0 {
        gc.stop()?;
    }
    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    vm.arm_execution_measurement(ristretto_vm::execution_measurement::Limits {
        methods: std::env::var("RISTRETTO_EXECUTION_METHOD_LIMIT")
            .map_or(Ok(4096), |value| value.parse::<usize>())?,
        ..Default::default()
    })?;
    let context = gc.register_context();
    let parameters = [
        workload.as_str(),
        duration.as_str(),
        warmup.as_str(),
        rate.as_str(),
        seed.as_str(),
    ];
    #[cfg(feature = "execution-profile")]
    let execution_checkpoint = vm.execution_profile_checkpoint(
        "main invocation, including main-class loading, warmup and reporting",
    );
    let result = Box::pin(context.scope(vm.invoke_main(&parameters))).await;
    #[cfg(all(
        feature = "experimental-execution-measurement",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    println!(
        "RISTRETTO_EXECUTION_MEASUREMENT {}",
        serde_json::json!({
            "schema_version":1, "identity":execution_identity, "capture":vm.take_execution_measurement()
        })
    );
    result?;
    #[cfg(feature = "execution-profile")]
    println!(
        "RISTRETTO_EXECUTION_INTERVAL {}",
        serde_json::to_string(&vm.execution_profile_since(execution_checkpoint)?)?
    );
    #[cfg(feature = "execution-profile")]
    println!(
        "RISTRETTO_EXECUTION {}",
        serde_json::to_string(&vm.execution_profile())?
    );
    let statistics = gc.statistics()?;
    println!(
        "RISTRETTO_GC {{\"live_objects\":{},\"managed_bytes\":{},\"workers\":{},\"candidate\":{},\"jit_enabled\":{},\"collections_completed\":{}}}",
        statistics.live_objects,
        statistics.bytes_managed,
        workers,
        candidate,
        jit_enabled,
        statistics.collections_completed
    );
    vm.wait_for_non_daemon_threads().await?;
    #[cfg(all(
        feature = "experimental-pause-recorder",
        target_os = "linux",
        target_has_atomic = "64"
    ))]
    gc.stop()?;
    Ok(())
}

fn collector_configuration(heap: usize, workers: usize, candidate: bool) -> Configuration {
    // Threadless Wasm drives collection cooperatively. Keep its publication and
    // memory policies, without worker queues, claim gates or distributed shards.
    let distributed = candidate && !cfg!(target_family = "wasm");
    Configuration::builder()
        .max_heap_bytes(heap)
        .threads(workers)
        .compact_heap(candidate)
        .root_storage(if candidate {
            RootStorage::Contexts
        } else {
            RootStorage::Tree
        })
        .barrier_buffer_size(if candidate { 32 } else { 0 })
        .local_allocation_buffers(candidate)
        .predictive_collection(candidate)
        .adaptive_workers(distributed)
        .concurrent_publication(candidate)
        .independent_marking(distributed)
        .independent_phases(distributed)
        .independent_relocation(distributed)
        .work_stealing(distributed)
        .metadata_shards(if distributed { 16 } else { 1 })
        .region_shards(if distributed { 16 } else { 1 })
        .mark_batch_size(if candidate { 8 } else { 1 })
        .overlapping_generations(candidate)
        .old_collection_interval(if candidate { 4 } else { 0 })
        .cost_aware_relocation(candidate)
        .relocation_reserve_bytes(if candidate { heap / 64 } else { 0 })
        .build()
}
