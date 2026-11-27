//! Sustained Java payload allocation, old-array mutation, pinning, and fragmentation.
//! Usage: `gc_endurance` [seconds=60] [workers=4; 0=cooperative] [allocations/s=10000]
//!                    [heap mebibytes=64] [compact=true] [enforce latency gates=false] [diagnostics=false] [predictive=false] [root slots=false] [mark batch size=1] [independent marking=false] [metadata shards=1] [work stealing=false] [context roots=false] [barrier records=0] [local allocation buffers=false] [adaptive workers=false] [independent relocation=false] [cost aware relocation=false] [relocation reserve bytes=0]
//! The optional 28th argument selects `generic` (default) or `element` owner-array writes.
#![expect(clippy::print_stdout)]
#![recursion_limit = "512"]
#[path = "gc_support/capacity.rs"]
mod capacity;
#[path = "gc_support/region_stats.rs"]
mod region_stats;

use ristretto_classfile::ClassFile;
use ristretto_classloader::{Class, ObjectArray, Reference, ReferenceWriteGuardExt, Value};
use ristretto_gc::{Configuration, GarbageCollector};
#[path = "gc_support/histogram.rs"]
mod gc_support;
use gc_support::Histogram;
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

#[expect(clippy::too_many_lines, reason = "self-contained endurance driver")]
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let seconds = argument(1, 60_u64);
    let workers = argument(2, 4_usize);
    let rate = argument(3, 10_000_u64).max(1);
    let heap_mib = argument(4, 64_usize);
    let compact = argument(5, true);
    let enforce = argument(6, false);
    let diagnostics = argument(7, false);
    let predictive = argument(8, false);
    let root_slots = argument(9, false);
    let mark_batch_size = argument(10, 1_usize);
    let independent_marking = argument(11, false);
    let metadata_shards = argument(12, 1_usize);
    let work_stealing = argument(13, false);
    let context_roots = argument(14, false);
    let barrier_records = argument(15, 0_usize);
    let local_allocation = argument(16, false);
    let adaptive_workers = argument(17, false);
    let independent_relocation = argument(18, false);
    let cost_aware_relocation = argument(19, false);
    let relocation_reserve_bytes = argument(20, 0_usize);
    let concurrent_publication = argument(21, false);
    let overlapping_generations = argument(22, false);
    let region_shards = argument(23, 1_usize);
    let old_collection_interval = argument(27, 0_usize);
    let independent_phases = argument(24, false);
    let telemetry_ms = argument(25, 1000_u64);
    let mode = argument(26, "sync".to_string());
    if mode != "sync" && mode != "async" {
        return Err("endurance mode must be sync or async".into());
    }
    let array_write = argument(28, "generic".to_string());
    let element_write = match array_write.as_str() {
        "generic" => false,
        "element" => true,
        _ => return Err("array write must be generic or element".into()),
    };
    let configuration = Configuration::builder()
        .concurrent_publication(concurrent_publication)
        .overlapping_generations(overlapping_generations)
        .old_collection_interval(old_collection_interval)
        .diagnostics(diagnostics)
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
        .compact_heap(compact)
        .region_layout(1024 * 1024, 4096)
        .max_heap_bytes(heap_mib.saturating_mul(1024 * 1024))
        .allocation_threshold(512 * 1024)
        .full_collection_interval(8)
        .incremental_step_size(64)
        .max_pause_time_us(25)
        .build();
    let gc = GarbageCollector::with_config(configuration.clone());
    let execution_context = gc
        .execution_contexts_enabled()
        .then(|| gc.register_context());
    let mut file = ClassFile::default();
    file.this_class = file.constant_pool.add_class("[Ljava/lang/Object;")?;
    let class = Class::from(None, file)?;
    let old = Value::try_new_object(
        &gc,
        Reference::Array(ObjectArray {
            class,
            elements: vec![Value::Object(None); 2048].into(),
        }),
    )?;
    gc.collect_full()?;
    gc.collect_full()?;
    if workers != 0 {
        gc.start();
    }
    let mut samples = Histogram::default();
    let mut arrivals = Histogram::default();
    let mut steps = Histogram::default();
    let mut random = 0x53a9_07b1_0042_ef15_u64;
    let mut iterations = 0_u64;
    let mut peak_backlog = 0_u64;
    let mut peak_committed = 0;
    let mut peak_live = 0;
    let mut telemetry = capacity::Telemetry::new(telemetry_ms, seconds)?;
    let started = Instant::now();
    let mut next = started;
    let offered = rate.saturating_mul(seconds);
    telemetry.record(&gc, started, 0, rate, offered, true)?;
    let workload = async {
        while started.elapsed() < Duration::from_secs(seconds) {
            random ^= random << 13;
            random ^= random >> 7;
            random ^= random << 17;
            let length = if iterations.is_multiple_of(4096) {
                1024 * 1024
            } else {
                32 << (random % 9)
            };
            let slot = usize::try_from(random % 2048)?;
            let mark = i8::from_ne_bytes([u8::try_from(iterations % 256)?]);
            let operation = Instant::now();
            let child = if mode == "async" {
                let data =
                    ristretto_gc::HeapBuffer::try_with_fn_in_async(&gc, length, |_| Ok(mark))
                        .await?;
                Value::try_new_object_async(&gc, Reference::ByteArray(data)).await?
            } else {
                Value::try_new_object(
                    &gc,
                    Reference::ByteArray(ristretto_gc::HeapBuffer::try_with_fn_in(
                        &gc,
                        length,
                        |_| Ok(mark),
                    )?),
                )?
            };
            let identity = match &child {
                Value::Object(Some(reference)) => reference.identity(),
                _ => return Err("allocation did not return an object".into()),
            };
            {
                let mut owner = old.as_reference_mut()?;
                if element_write {
                    owner.set_array_element(slot, child).map_err(|_| "slot")?;
                } else if let Reference::Array(array) = &mut *owner {
                    *array.elements.get_mut(slot).ok_or("slot")? = child;
                }
            }
            // Pins intentionally span a cooperative step. Relocation must skip this owner.
            if iterations.is_multiple_of(97) {
                let guard = old.as_reference()?;
                let step = Instant::now();
                gc.step();
                steps.record(step.elapsed());
                if let Reference::Array(array) = &*guard {
                    let witness = array.elements.get(slot).ok_or("slot")?;
                    if let Value::Object(Some(reference)) = witness {
                        assert_eq!(reference.identity(), identity);
                        let data = reference.read();
                        if let Reference::ByteArray(bytes) = &*data {
                            assert!(bytes.iter().all(|value| *value == mark));
                        }
                    }
                }
            } else if workers == 0 {
                let step = Instant::now();
                gc.step();
                steps.record(step.elapsed());
            }
            samples.record(operation.elapsed());
            arrivals.record(Instant::now().saturating_duration_since(next));
            let due = (started
                .elapsed()
                .as_nanos()
                .saturating_mul(u128::from(rate))
                / 1_000_000_000
                + 1)
            .min(u128::from(offered));
            peak_backlog = peak_backlog.max(u64::try_from(due)?.saturating_sub(iterations));
            iterations += 1;
            if iterations.is_multiple_of(1024) {
                peak_live = peak_live.max(gc.managed_bytes());
                if let Some(heap) = gc.region_heap() {
                    peak_committed = peak_committed.max(heap.statistics().committed_bytes);
                }
            }
            telemetry.record(&gc, started, iterations, rate, offered, false)?;
            let offset = u128::from(iterations) * 1_000_000_000 / u128::from(rate);
            next = started + Duration::from_nanos(u64::try_from(offset)?);
            if let Some(delay) = next.checked_duration_since(Instant::now()) {
                std::thread::sleep(delay);
            }
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    };
    if let Some(context) = &execution_context {
        context.scope(workload).await?;
    } else {
        workload.await?;
    }
    telemetry.record(&gc, started, iterations, rate, offered, true)?;
    let elapsed = started.elapsed().as_secs_f64();
    let diagnostic = gc.diagnostics();
    if mode == "async" && workers != 0 {
        assert_eq!(
            diagnostic.allocation_recovery.count, 0,
            "ordinary async allocations entered synchronous recovery"
        );
    }
    let scheduling = gc.scheduling_statistics();
    let roots = gc.root_statistics();
    drop(old);
    drop(execution_context);
    gc.stop()?;
    gc.collect_full()?;
    let stats = gc.statistics()?;
    let regions = gc.region_heap().map_or_default(|heap| heap.statistics());
    assert_eq!(
        stats.live_objects, 0,
        "endurance run retained unreachable Java objects"
    );
    assert_eq!(regions.occupied_bytes, 0, "region ranges leaked");
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
    let p999 = samples.percentile_us(999, 1000);
    let p9999 = samples.percentile_us(9999, 10000);
    let timing = |value: ristretto_gc::TimingStatistics| {
        serde_json::json!({
            "count": value.count, "total_ns": value.total_ns, "max_ns": value.max_ns,
        })
    };
    println!(
        "{}",
        serde_json::json!({
            "seconds": elapsed, "workers": workers, "compact": compact,
            "mutators": 1, "seed": "0x53a907b10042ef15",
            "histogram": { "linear_microseconds": 1024, "subdivisions_per_doubling": 64, "bins": 4096 },
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
                "final_mark_queue_capacity": final_work.mark_queue_capacity,
                "final_slot_roots": final_roots.slot_roots,
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
            "configuration": {
                "predictive_collection": configuration.predictive_collection,
                "root_slots": root_slots,
                "diagnostics": configuration.diagnostics,
                "threads": configuration.threads,
                "worker_yield_time_us": configuration.worker_yield_time_us,
                "allocation_threshold": configuration.allocation_threshold,
                "max_pause_time_us": configuration.max_pause_time_us,
                "incremental_step_size": configuration.incremental_step_size,
            "mark_batch_size": configuration.mark_batch_size,
            "independent_marking": configuration.independent_marking,
            "independent_phases": configuration.independent_phases,
            "concurrent_publication": configuration.concurrent_publication,
            "overlapping_generations": configuration.overlapping_generations,
            "old_collection_interval": configuration.old_collection_interval,
            "independent_relocation": configuration.independent_relocation,
            "cost_aware_relocation": configuration.cost_aware_relocation,
            "relocation_reserve_bytes": configuration.relocation_reserve_bytes,
            "metadata_shards": configuration.metadata_shards,
            "region_shards": configuration.region_shards,
            "work_stealing": configuration.work_stealing,
            "context_roots": context_roots, "local_allocation_buffers": local_allocation, "adaptive_workers": adaptive_workers, "barrier_buffer_size": configuration.barrier_buffer_size,
                "parallel_threshold": configuration.parallel_threshold,
                "max_heap_bytes": configuration.max_heap_bytes,
                "promotion_age": configuration.promotion_age,
                "full_collection_interval": configuration.full_collection_interval,
                "compact_heap": configuration.compact_heap,
                "region_size": configuration.region_size,
                "relocation_chunk_bytes": configuration.relocation_chunk_bytes,
            },
            "mode": mode,
            "array_write": array_write,
            "telemetry_interval_ms": telemetry.interval_ms,
            "telemetry_storage_bytes": telemetry.storage_bytes(),
            "capacity_samples": telemetry.samples,
            "offered_per_second": rate, "offered_operations": offered,
            "operations": iterations, "backlog_operations": offered.saturating_sub(iterations),
            "peak_backlog_operations": peak_backlog,
            "completed_per_second": Duration::from_secs(iterations).as_secs_f64() / elapsed,
            "operation_p50_us": samples.percentile_us(50, 100),
            "operation_p99_us": samples.percentile_us(99, 100),
            "operation_p999_us": p999, "operation_p9999_us": p9999,
            "operation_max_ns": samples.max_ns,
            "operation_histogram_us": samples.nonempty_buckets(),
            "arrival_histogram_us": arrivals.nonempty_buckets(),
            "arrival_p50_us": arrivals.percentile_us(50, 100),
            "arrival_p99_us": arrivals.percentile_us(99, 100),
            "arrival_p999_us": arrivals.percentile_us(999, 1000),
            "arrival_p9999_us": arrivals.percentile_us(9999, 10000),
            "arrival_max_ns": arrivals.max_ns,
            "step_p999_us": steps.percentile_us(999, 1000),
            "peak_live_bytes": peak_live, "peak_committed_bytes": peak_committed,
            "young_cycles": stats.young_collections, "full_cycles": stats.full_collections, "old_cycles": stats.old_collections,
            "collection_wall_time_ns": stats.total_collection_time.as_nanos(),
            "caller_assistance_us": stats.total_pause_time_us,
            "bytes_copied": regions.bytes_copied, "bytes_staged": regions.bytes_staged,
            "bytes_recopied": regions.bytes_recopied, "destination_failures": regions.destination_failures,
            "relocations": regions.relocations, "regions_released": regions.regions_released,
            "final_live_objects": stats.live_objects, "final_occupied_bytes": regions.occupied_bytes,
            "regions": region_stats::snapshot(regions),
            "diagnostics": {
                "enabled": diagnostic.enabled, "clock_available": diagnostic.clock_available,
                "publication_read_wait": timing(diagnostic.publication_read_wait),
                "descriptor_publication": timing(diagnostic.descriptor_publication),
                "root_acquire": timing(diagnostic.root_acquire),
                "root_release": timing(diagnostic.root_release),
                "barrier_record": timing(diagnostic.barrier_record),
                "barrier_drain": timing(diagnostic.barrier_drain),
                "publication_write_wait": timing(diagnostic.publication_write_wait),
            "active_publications": diagnostic.active_publications,
            "publication_transitions": diagnostic.publication_transitions,
            "publication_retries": diagnostic.publication_retries,
                "payload_read_wait": timing(diagnostic.payload_read_wait),
                "payload_write_wait": timing(diagnostic.payload_write_wait),
                "allocation_recovery": timing(diagnostic.allocation_recovery),
                "async_allocation_recovery": timing(diagnostic.async_allocation_recovery),
                "async_collection_calls": diagnostic.async_collection_calls,
                "async_assists": diagnostic.async_assists,
                "allocation_early_wakeups": diagnostic.allocation_early_wakeups,
            "physical_allocation_early_wakeups": diagnostic.physical_allocation_early_wakeups,
            "worker_backoffs": diagnostic.worker_backoffs,
                "allocation_waiters": diagnostic.allocation_waiters,
                "caller_steps": timing(diagnostic.caller_steps),
                "worker_steps": timing(diagnostic.worker_steps),
                "worker_cpu_ns": diagnostic.worker_cpu_ns,
                "full_collection_calls": diagnostic.full_collection_calls,
                "root_registrations": diagnostic.root_registrations,
                "coordinator_misses": diagnostic.coordinator_misses,
            "mark_batches": diagnostic.mark_batches,
            "independent_mark_steps": diagnostic.independent_mark_steps,
            "independent_phase_steps": diagnostic.independent_phase_steps, "mark_gate_misses": diagnostic.mark_gate_misses, "trace_deferred": diagnostic.trace_deferred,
                "mark_stolen": diagnostic.mark_stolen, "mark_queue_misses": diagnostic.mark_queue_misses,
            "queued_mark_tasks": diagnostic.queued_mark_tasks, "mark_queue_capacity": diagnostic.mark_queue_capacity,
            "in_flight_work": diagnostic.in_flight_work,
            "execution_contexts": diagnostic.execution_contexts, "context_storage_bytes": diagnostic.context_storage_bytes,
            "pending_barrier_records": diagnostic.pending_barrier_records,
            "cached_allocation_bytes": diagnostic.cached_allocation_bytes,
            "cached_allocation_credit": diagnostic.cached_allocation_credit,
            "local_allocation_hits": diagnostic.local_allocation_hits,
            "local_allocation_refills": diagnostic.local_allocation_refills,
            "descriptor_committed_bytes": diagnostic.descriptor_committed_bytes,
            "descriptor_occupied_bytes": diagnostic.descriptor_occupied_bytes,
            "retired_bytes": diagnostic.retired_bytes,
            "retired_objects": diagnostic.retired_objects,
            "retired_ready": diagnostic.retired_ready,
            "worker_target": diagnostic.worker_target,
            "worker_pacing_us": diagnostic.worker_pacing_us,
            "worker_throttles": diagnostic.worker_throttles,
            "independent_relocation_steps": diagnostic.independent_relocation_steps,
            "relocation_stolen": diagnostic.relocation_stolen,
            "relocation_queue_misses": diagnostic.relocation_queue_misses,
            "queued_relocation_tasks": diagnostic.queued_relocation_tasks,
            "relocation_queue_capacity": diagnostic.relocation_queue_capacity,

            "barrier_buffered": diagnostic.barrier_buffered, "barrier_flushed": diagnostic.barrier_flushed,
            "barrier_overflows": diagnostic.barrier_overflows,
            "barrier_coalesced": diagnostic.barrier_coalesced,
            "old_handoff_objects": diagnostic.old_handoff_objects, "old_trace_tasks": diagnostic.old_trace_tasks,
                "trace_tasks": diagnostic.trace_tasks, "trace_busy": diagnostic.trace_busy,
                "relocation_tasks": diagnostic.relocation_tasks, "relocation_busy": diagnostic.relocation_busy,
            "relocation_pin_deferral": timing(diagnostic.relocation_pin_deferral),
            "payload_guard_holds": timing(diagnostic.payload_guard_holds),
            "active_payload_guards": diagnostic.active_payload_guards,
            "relocation_retry_limits": diagnostic.relocation_retry_limits,
            "relocation_cancel_busy": diagnostic.relocation_cancel_busy,
            }
        })
    );
    if enforce {
        assert!(p999 < 100 && p9999 < 1000, "latency release gate failed");
    }
    Ok(())
}
