//! Controlled cost diagnostic, not a public backend or a latency qualification.
use super::maintenance::MaintenanceStep;
use super::runtime::{LeafRuntime, RuntimeFootprint};
use crate::canonical_catalog as gc_catalog;
use crate::{Configuration, Error, GarbageCollector, Gc, Result, RootStorage, Trace};
use gc_catalog::bounded::{Budget, Capacities};
use std::hint::black_box;
use std::time::{Duration, Instant};

struct Payload([u64; 4]);
// SAFETY: only primitive integers, no managed edges or tracing action.
unsafe impl Trace for Payload {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
fn failure(message: impl Into<String>) -> Error {
    Error::RootError(message.into())
}
fn protocol(error: impl std::fmt::Debug) -> Error {
    failure(format!("leaf benchmark: {error:?}"))
}
fn expected_sum(operations: u64) -> u64 {
    let remainder = operations % 1024;
    operations / 1024 * (1024 * 1025 / 2) + remainder * (remainder + 1) / 2
}
#[derive(Default)]
struct Report {
    operation: Duration,
    cleanup: Duration,
    lifecycle: Duration,
    setup: Duration,
    teardown: Duration,
    checksum: u64,
    allocation_polls: u64,
    helper_polls: u64,
    released_objects: u64,
    released_roots: u64,
    catalog_bytes: usize,
    payload_peak: usize,
}

pub(in crate::gc::publication_owner) fn run(
    mode: &str,
    operations: u64,
    batch: usize,
) -> Result<String> {
    let report = measure(mode, operations, batch)?;
    Ok(format!(
        "{{\"version\":1,\"mode\":\"{mode}\",\"operations\":{operations},\"batch\":{batch},\"payload_bytes\":32,\"logical_object_bytes\":{},\"heap_limit\":67108864,\"operation_ns\":{},\"cleanup_ns\":{},\"lifecycle_ns\":{},\"setup_ns\":{},\"teardown_ns\":{},\"checksum\":{},\"allocation_polls\":{},\"helper_polls\":{},\"released_objects\":{},\"released_roots\":{},\"catalog_bytes\":{},\"payload_peak\":{},\"teardown_bytes\":0,\"teardown_catalog_bytes\":0}}",
        Gc::allocation_size(&Payload([0; 4]))?,
        report.operation.as_nanos(),
        report.cleanup.as_nanos(),
        report.lifecycle.as_nanos(),
        report.setup.as_nanos(),
        report.teardown.as_nanos(),
        report.checksum,
        report.allocation_polls,
        report.helper_polls,
        report.released_objects,
        report.released_roots,
        report.catalog_bytes,
        report.payload_peak,
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "one shared timing boundary for all fixture modes"
)]
fn measure(mode: &str, operations: u64, batch: usize) -> Result<Report> {
    if !["legacy", "canonical-explicit", "canonical-auto"].contains(&mode)
        || !(1..=10_000_000).contains(&operations)
        || !(1..=16384).contains(&batch)
    {
        return Err(failure("invalid leaf lifecycle fixture"));
    }
    if cfg!(all(target_family = "wasm", target_os = "unknown")) {
        return Err(failure(
            "leaf lifecycle timing requires a native or WASI clock",
        ));
    }
    let setup = Instant::now();
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(64 * 1024 * 1024)
            .build(),
    );
    let context = gc.register_context();
    let mut report = Report::default();
    let capacities = Capacities::derive(batch, 1, 1)
        .map_err(protocol)?
        .with_roots(batch)
        .map_err(protocol)?;
    let footprint = RuntimeFootprint::for_capacities(capacities).map_err(protocol)?;
    let budget = Budget::new(footprint.total());
    let runtime = if mode == "legacy" {
        None
    } else {
        Some(LeafRuntime::new(&gc, capacities, &budget).map_err(protocol)?)
    };
    let mut mutator = runtime
        .as_ref()
        .map(LeafRuntime::mutator)
        .transpose()
        .map_err(protocol)?;
    let mut helper = runtime
        .as_ref()
        .map(LeafRuntime::maintenance)
        .transpose()
        .map_err(protocol)?;
    report.catalog_bytes = budget.used();
    report.setup = setup.elapsed();
    let lifecycle = Instant::now();
    context.with(|| -> Result<()> {
        let mut completed = 0;
        while completed < operations {
            let count = (operations - completed).min(batch as u64);
            let start = Instant::now();
            for index in 0..count {
                let value = (completed + index) % 1024 + 1;
                let payload = Payload(black_box([value; 4]));
                if let (Some(runtime), Some(mutator)) = (&runtime, &mut mutator) {
                    let mut request = if mode == "canonical-auto" {
                        mutator.begin_any(payload)
                    } else {
                        mutator.begin(
                            usize::try_from(index).map_err(protocol)?,
                            usize::try_from(index).map_err(protocol)?,
                            payload,
                        )
                    }
                    .map_err(protocol)?;
                    let mut finished = false;
                    for _ in 0..4096 {
                        report.allocation_polls += 1;
                        if let Some(root) = request.poll().map_err(protocol)? {
                            report.checksum +=
                                black_box(runtime.borrow(&root).map_err(protocol)?.0[0]);
                            finished = true;
                            break;
                        }
                    }
                    if !finished {
                        return Err(failure("quiescent allocation did not settle"));
                    }
                } else {
                    let root = Gc::try_new(&gc, payload)?;
                    report.checksum += black_box(root.0[0]);
                }
            }
            report.operation += start.elapsed();
            report.payload_peak = report.payload_peak.max(gc.managed_bytes());
            let start = Instant::now();
            if let Some(helper) = &mut helper {
                let (mut objects, mut roots) = (0, 0);
                // Count actual releases instead of calling diagnostic statistics
                // at every helper poll. One post-batch check confirms real credit.
                for _ in 0..batch * 4096 {
                    report.helper_polls += 1;
                    match helper.poll().map_err(protocol)? {
                        MaintenanceStep::ReleasedObjectSlot => objects += 1,
                        MaintenanceStep::ReleasedRootSlot => roots += 1,
                        _ => {}
                    }
                    if objects == count && roots == count {
                        break;
                    }
                }
                if objects != count || roots != count {
                    return Err(failure("quiescent reclamation did not settle"));
                }
                report.released_objects += objects;
                report.released_roots += roots;
            } else {
                context.flush();
                gc.collect_full()?;
            }
            report.cleanup += start.elapsed();
            if gc.managed_bytes() != 0 {
                return Err(failure("batch retained payload credit"));
            }
            completed += count;
        }
        Ok(())
    })?;
    report.lifecycle = lifecycle.elapsed();
    if report.checksum != expected_sum(operations) {
        return Err(failure("checksum mismatch"));
    }
    let teardown = Instant::now();
    drop(mutator);
    drop(helper);
    drop(runtime);
    if budget.used() != 0 {
        return Err(failure("catalog reservation leaked"));
    }
    drop(budget);
    drop(context);
    gc.stop()?;
    if gc.managed_bytes() != 0 {
        return Err(failure("payload teardown leaked"));
    }
    drop(gc);
    report.teardown = teardown.elapsed();
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "fixture invariants")]
    fn concrete_lifecycle_modes_reclaim_complete_and_partial_batches() -> Result<()> {
        for mode in ["legacy", "canonical-explicit", "canonical-auto"] {
            let report = measure(mode, 7, 4)?;
            assert_eq!(report.checksum, 28);
            if mode == "legacy" {
                assert_eq!(report.catalog_bytes, 0);
                assert_eq!(report.helper_polls, 0);
            } else {
                assert!(report.catalog_bytes > 0);
                assert_eq!((report.released_objects, report.released_roots), (7, 7));
            }
        }
        Ok(())
    }
}
