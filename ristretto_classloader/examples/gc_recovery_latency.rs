//! Compare synchronous and asynchronous pressure recovery on a single-thread executor.
//! Usage: `gc_recovery_latency` [seconds=10] [workers=1; 0=cooperative] [heap MiB=16]
//!                            [compact=true] [mode=async|sync] [allocations per yield=8] [predictive=false] [root slots=false] [mark batch size=1] [independent marking=false] [metadata shards=1] [work stealing=false] [context roots=false] [barrier records=0] [local allocation buffers=false] [payload bytes=4096] [adaptive workers=false] [independent relocation=false] [cost aware relocation=false] [relocation reserve bytes=0] [diagnostics=true]
#![expect(clippy::print_stdout)]
#![recursion_limit = "512"]
#[path = "gc_support/histogram.rs"]
mod gc_support;
#[path = "gc_support/region_stats.rs"]
mod region_stats;

use gc_support::Histogram;
use ristretto_classloader::{Reference, Value};
use ristretto_gc::{Configuration, GarbageCollector, HeapBuffer, TimingStatistics};
use std::cell::Cell;
use std::future::Future;
use std::time::{Duration, Instant};

fn argument<T: std::str::FromStr>(index: usize, default: T) -> T {
    std::env::args()
        .nth(index)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run())
}

#[expect(clippy::too_many_lines, reason = "self-contained comparison driver")]
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let seconds = argument(1, 10_u64);
    let workers = argument(2, 1_usize);
    let heap_mib = argument(3, 16_usize);
    let compact = argument(4, true);
    let mode = argument(5, "async".to_owned());
    let batch_size = argument(6, 8_u64).max(1);
    let predictive = argument(7, false);
    let root_slots = argument(8, false);
    let mark_batch_size = argument(9, 1_usize);
    let independent_marking = argument(10, false);
    let metadata_shards = argument(11, 1_usize);
    let work_stealing = argument(12, false);
    if !matches!(mode.as_str(), "sync" | "async") {
        return Err("mode must be sync or async".into());
    }
    let context_roots = argument(13, false);
    let barrier_records = argument(14, 0_usize);
    let local_allocation = argument(15, false);
    let adaptive_workers = argument(17, false);
    let independent_relocation = argument(18, false);
    let cost_aware_relocation = argument(19, false);
    let relocation_reserve_bytes = argument(20, 0_usize);
    let diagnostics_enabled = argument(21, true);
    let payload_bytes = argument(16, 4096_usize).max(1);
    let concurrent_publication = argument(22, false);
    let overlapping_generations = argument(23, false);
    let region_shards = argument(24, 1_usize);
    let old_collection_interval = argument(26, 0_usize);
    let independent_phases = argument(25, false);
    let configuration = Configuration::builder()
        .concurrent_publication(concurrent_publication)
        .overlapping_generations(overlapping_generations)
        .old_collection_interval(old_collection_interval)
        .diagnostics(diagnostics_enabled)
        .predictive_collection(predictive)
        .mark_batch_size(mark_batch_size)
        .independent_marking(independent_marking)
        .independent_phases(independent_phases)
        .independent_relocation(independent_relocation)
        .cost_aware_relocation(cost_aware_relocation)
        .relocation_reserve_bytes(relocation_reserve_bytes)
        .metadata_shards(metadata_shards)
        .region_shards(region_shards)
        .work_stealing(work_stealing)
        .barrier_buffer_size(barrier_records)
        .local_allocation_buffers(local_allocation)
        .adaptive_workers(adaptive_workers)
        .root_storage(if context_roots {
            ristretto_gc::RootStorage::Contexts
        } else if root_slots {
            ristretto_gc::RootStorage::Slots
        } else {
            ristretto_gc::RootStorage::Tree
        })
        .threads(workers)
        .max_heap_bytes(heap_mib.saturating_mul(1024 * 1024))
        .compact_heap(compact)
        .allocation_threshold(usize::MAX)
        .incremental_step_size(64)
        .max_pause_time_us(25)
        .region_layout(1024 * 1024, 4096)
        .build();
    let gc = GarbageCollector::with_config(configuration.clone());
    let execution_context = gc
        .execution_contexts_enabled()
        .then(|| gc.register_context());
    if workers != 0 {
        gc.start();
    }
    let mut operations = Histogram::default();
    let mut polls = Histogram::default();
    let mut timer_delay = Histogram::default();
    let end = Cell::new(None);
    let started = Instant::now();
    let mut count = 0_u64;
    // The only long-lived roots are this fixed ring. Both modes yield every batch;
    // debt collection is disabled to compare recovery with optional prediction.
    {
        let workload = async {
            let result: Result<(), Box<dyn std::error::Error>> = async {
                let mut roots = vec![Value::Object(None); 512];
                while started.elapsed() < Duration::from_secs(seconds) {
                    let begin = Instant::now();
                    let value = if mode == "async" {
                        let data =
                            HeapBuffer::try_with_fn_in_async(&gc, payload_bytes, |_| Ok(7_i8))
                                .await?;
                        Value::try_new_object_async(&gc, Reference::ByteArray(data)).await?
                    } else {
                        let data = HeapBuffer::try_with_fn_in(&gc, payload_bytes, |_| Ok(7_i8))?;
                        Value::try_new_object(&gc, Reference::ByteArray(data))?
                    };
                    let slot = usize::try_from(count % 512)?;
                    if let Some(root) = roots.get_mut(slot) {
                        *root = value;
                    }
                    operations.record(begin.elapsed());
                    count += 1;
                    if count.is_multiple_of(batch_size) {
                        tokio::task::yield_now().await;
                    }
                }
                Ok(())
            }
            .await;
            end.set(Some(tokio::time::Instant::now()));
            result
        };
        let mut workload = std::pin::pin!(workload);
        let measured = std::future::poll_fn(|context| {
            let begin = Instant::now();
            let result = if let Some(execution) = &execution_context {
                execution.with(|| workload.as_mut().poll(context))
            } else {
                workload.as_mut().poll(context)
            };
            polls.record(begin.elapsed());
            result
        });
        let observer = async {
            let mut interval = tokio::time::interval(Duration::from_millis(1));
            // Burst delivery preserves missed deadlines instead of hiding executor stalls.
            loop {
                let deadline = interval.tick().await;
                if end.get().is_some_and(|end| deadline > end) {
                    break;
                }
                timer_delay.record(deadline.elapsed());
            }
        };
        let (result, ()) = tokio::join!(measured, observer);
        result?;
    }
    let elapsed = started.elapsed().as_secs_f64();
    let diagnostics = gc.diagnostics();
    let during = gc.statistics()?;
    let scheduling = gc.scheduling_statistics();
    let roots = gc.root_statistics();
    assert_eq!(diagnostics.allocation_waiters, 0);
    if mode == "async" {
        assert_eq!(diagnostics.allocation_recovery.count, 0);
    }
    drop(execution_context);
    gc.stop()?;
    gc.collect_full()?;
    let statistics = gc.statistics()?;
    let regions = gc
        .region_heap()
        .map(|heap| heap.statistics())
        .unwrap_or_default();
    assert_eq!(statistics.live_objects, 0);
    assert_eq!(regions.shared_committed_bytes, 0);
    let final_roots = gc.root_statistics();
    assert_eq!(final_roots.slot_roots, 0);
    assert_eq!(final_roots.compatibility_roots, 0);
    assert_eq!(final_roots.pages, 0);
    assert_eq!(final_roots.reserved_slots, 0);
    assert_eq!(final_roots.resident_pages, 0);
    let final_work = gc.diagnostics();
    assert_eq!(final_work.pending_barrier_records, 0);
    assert_eq!(final_work.execution_contexts, 0);
    assert_eq!(final_work.cached_allocation_bytes, 0);
    assert_eq!(final_work.cached_allocation_credit, 0);
    assert_eq!(final_work.descriptor_committed_bytes, 0);
    assert_eq!(final_work.descriptor_occupied_bytes, 0);
    assert_eq!(final_work.retired_bytes, 0);
    assert_eq!(final_work.retired_objects, 0);
    assert_eq!(final_work.retired_ready, 0);
    assert_eq!(final_work.active_payload_guards, 0);
    assert_eq!(final_work.active_publications, 0);
    assert_eq!(final_work.queued_relocation_tasks, [0, 0]);
    assert_eq!(final_work.relocation_queue_capacity, [0, 0]);
    assert_eq!(final_work.queued_mark_tasks, [0, 0]);
    assert_eq!(final_work.in_flight_work, [0, 0]);
    assert_eq!(final_work.mark_queue_capacity, [0, 0]);
    assert_eq!(regions.occupied_bytes, 0);
    let timing = |value: TimingStatistics| serde_json::json!({"count": value.count, "total_ns": value.total_ns, "max_ns": value.max_ns});
    let histogram = |value: &Histogram| {
        serde_json::json!({
            "count": value.count, "p999_us": value.percentile_us(999, 1000),
            "p9999_us": value.percentile_us(9999, 10000), "max_ns": value.max_ns,
            "histogram_us": value.nonempty_buckets(),
        })
    };
    println!(
        "{}",
        serde_json::json!({
            "predictive": predictive, "root_slots": root_slots,
            "roots": {
                "pages_created": roots.pages_created, "pages_released": roots.pages_released,
                "slot_reuses": roots.slot_reuses, "page_bytes_before_shutdown": roots.page_bytes,
                "final_pages": final_roots.pages,
                "reserved_slots_before_shutdown": roots.reserved_slots,
                "resident_page_bytes_before_shutdown": roots.resident_page_bytes,
                "final_reserved_slots": final_roots.reserved_slots,
                "final_resident_pages": final_roots.resident_pages,
                "final_queued_mark_tasks": final_work.queued_mark_tasks,
                "final_in_flight_work": final_work.in_flight_work,
            "final_execution_contexts": final_work.execution_contexts,
            "final_cached_allocation_bytes": final_work.cached_allocation_bytes,
            "final_cached_allocation_credit": final_work.cached_allocation_credit,
            "final_descriptor_committed_bytes": final_work.descriptor_committed_bytes,
            "final_descriptor_occupied_bytes": final_work.descriptor_occupied_bytes,
            "final_retired_bytes": final_work.retired_bytes,
            "final_retired_objects": final_work.retired_objects,
            "final_retired_ready": final_work.retired_ready,
            "final_active_payload_guards": final_work.active_payload_guards,
            "final_active_publications": final_work.active_publications,
            "final_queued_relocation_tasks": final_work.queued_relocation_tasks,
            "final_relocation_queue_capacity": final_work.relocation_queue_capacity,

            "final_pending_barrier_records": final_work.pending_barrier_records,
                "final_mark_queue_capacity": final_work.mark_queue_capacity, "final_slot_roots": final_roots.slot_roots,
                "final_compatibility_roots": final_roots.compatibility_roots,
            },
            "scheduling": {
                "samples": scheduling.samples, "requests": scheduling.requests,
                "allocation_bytes_per_second": scheduling.allocation_bytes_per_second,
                "reclamation_latency_ns": scheduling.reclamation_latency_ns,
                "headroom_bytes": scheduling.headroom_bytes, "trigger_bytes": scheduling.trigger_bytes,
                "survival_ppm": scheduling.survival_ppm, "live_bytes": scheduling.live_bytes,
                "fragmented_free_bytes": scheduling.fragmented_free_bytes,
            },
            "mode": mode, "seconds": elapsed, "workers": workers, "compact": compact,
            "operations": count, "completed_per_second": Duration::from_secs(count).as_secs_f64() / elapsed,
            "batch_size": batch_size, "payload_bytes": payload_bytes, "retained_roots": 512,
            "max_heap_bytes": configuration.max_heap_bytes,
            "allocation_threshold": configuration.allocation_threshold,
            "incremental_step_size": configuration.incremental_step_size,
            "mark_batch_size": configuration.mark_batch_size,
            "independent_marking": configuration.independent_marking,
            "independent_phases": configuration.independent_phases,
            "concurrent_publication": configuration.concurrent_publication,
            "overlapping_generations": configuration.overlapping_generations,
            "old_collection_interval": configuration.old_collection_interval,
            "independent_relocation": configuration.independent_relocation,
            "cost_aware_relocation": configuration.cost_aware_relocation,
            "diagnostics_enabled": diagnostics_enabled,
            "relocation_reserve_bytes": configuration.relocation_reserve_bytes,
            "metadata_shards": configuration.metadata_shards,
            "region_shards": configuration.region_shards,
            "work_stealing": configuration.work_stealing,
            "context_roots": context_roots, "local_allocation_buffers": local_allocation, "adaptive_workers": adaptive_workers, "barrier_buffer_size": configuration.barrier_buffer_size,
            "max_pause_time_us": configuration.max_pause_time_us,
            "worker_yield_time_us": configuration.worker_yield_time_us,
            "region_size": configuration.region_size, "relocation_chunk_bytes": configuration.relocation_chunk_bytes,
            "promotion_age": configuration.promotion_age, "full_collection_interval": configuration.full_collection_interval,
            "operation": histogram(&operations), "executor_poll": histogram(&polls), "timer_delay": histogram(&timer_delay),
            "inline_recovery": timing(diagnostics.allocation_recovery),
            "async_recovery": timing(diagnostics.async_allocation_recovery),
            "async_assists": diagnostics.async_assists,
            "allocation_early_wakeups": diagnostics.allocation_early_wakeups,
            "physical_allocation_early_wakeups": diagnostics.physical_allocation_early_wakeups,
            "worker_backoffs": diagnostics.worker_backoffs, "allocation_waiters": diagnostics.allocation_waiters,
            "caller_steps": timing(diagnostics.caller_steps), "worker_steps": timing(diagnostics.worker_steps),
            "worker_cpu_ns": diagnostics.worker_cpu_ns,
            "publication_read_wait": timing(diagnostics.publication_read_wait),
                "descriptor_publication": timing(diagnostics.descriptor_publication),
                "root_acquire": timing(diagnostics.root_acquire),
                "root_release": timing(diagnostics.root_release),
                "barrier_record": timing(diagnostics.barrier_record),
                "barrier_drain": timing(diagnostics.barrier_drain),
            "publication_write_wait": timing(diagnostics.publication_write_wait),
            "active_publications": diagnostics.active_publications,
            "publication_transitions": diagnostics.publication_transitions,
            "publication_retries": diagnostics.publication_retries,
            "coordinator_misses": diagnostics.coordinator_misses,
            "mark_batches": diagnostics.mark_batches,
            "independent_mark_steps": diagnostics.independent_mark_steps, "mark_gate_misses": diagnostics.mark_gate_misses, "trace_deferred": diagnostics.trace_deferred,
            "mark_stolen": diagnostics.mark_stolen, "mark_queue_misses": diagnostics.mark_queue_misses,
            "queued_mark_tasks": diagnostics.queued_mark_tasks, "mark_queue_capacity": diagnostics.mark_queue_capacity,
            "in_flight_work": diagnostics.in_flight_work,
            "execution_contexts": diagnostics.execution_contexts, "context_storage_bytes": diagnostics.context_storage_bytes,
            "pending_barrier_records": diagnostics.pending_barrier_records,
            "cached_allocation_bytes": diagnostics.cached_allocation_bytes,
            "cached_allocation_credit": diagnostics.cached_allocation_credit,
            "local_allocation_hits": diagnostics.local_allocation_hits,
            "local_allocation_refills": diagnostics.local_allocation_refills,
            "descriptor_committed_bytes": diagnostics.descriptor_committed_bytes,
            "descriptor_occupied_bytes": diagnostics.descriptor_occupied_bytes,
            "retired_bytes": diagnostics.retired_bytes,
            "retired_objects": diagnostics.retired_objects,
            "retired_ready": diagnostics.retired_ready,
            "worker_target": diagnostics.worker_target,
            "worker_pacing_us": diagnostics.worker_pacing_us,
            "worker_throttles": diagnostics.worker_throttles,
            "relocation_tasks": diagnostics.relocation_tasks,
            "relocation_busy": diagnostics.relocation_busy,
            "relocation_pin_deferral": timing(diagnostics.relocation_pin_deferral),
            "payload_guard_holds": timing(diagnostics.payload_guard_holds),
            "active_payload_guards": diagnostics.active_payload_guards,
            "relocation_retry_limits": diagnostics.relocation_retry_limits,
            "relocation_cancel_busy": diagnostics.relocation_cancel_busy,
            "independent_relocation_steps": diagnostics.independent_relocation_steps,
            "relocation_stolen": diagnostics.relocation_stolen,
            "relocation_queue_misses": diagnostics.relocation_queue_misses,
            "queued_relocation_tasks": diagnostics.queued_relocation_tasks,
            "relocation_queue_capacity": diagnostics.relocation_queue_capacity,

            "barrier_buffered": diagnostics.barrier_buffered, "barrier_flushed": diagnostics.barrier_flushed,
            "barrier_overflows": diagnostics.barrier_overflows,
            "barrier_coalesced": diagnostics.barrier_coalesced,
            "old_handoff_objects": diagnostics.old_handoff_objects, "old_trace_tasks": diagnostics.old_trace_tasks,
            "trace_tasks": diagnostics.trace_tasks, "trace_busy": diagnostics.trace_busy,
            "old_collections": during.old_collections, "full_collections": during.full_collections, "bytes_freed": during.managed_bytes_freed,
            "final_live_objects": statistics.live_objects, "final_occupied_bytes": regions.occupied_bytes,
            "regions": region_stats::snapshot(regions),
        })
    );
    Ok(())
}
