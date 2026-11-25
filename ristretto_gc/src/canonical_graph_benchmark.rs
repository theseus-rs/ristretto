//! Controlled cyclic-graph cost diagnostic. No default GC policy or pause claim.
use super::graph::{
    GraphAllocation, GraphFootprint, GraphMutator, GraphReadValue, GraphRoot, GraphRuntime,
};
use super::maintenance::{LeafMaintenance, MaintenanceStep};
use crate::canonical_catalog as gc_catalog;
use crate::{Configuration, Error, GarbageCollector, Gc, Result, RootStorage, Trace};
use gc_catalog::bounded::{Budget, Capacities};
use std::hint::black_box;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::{Duration, Instant};

struct Value {
    identity: u64,
    canary: u64,
    drops: Arc<AtomicUsize>,
}
impl Value {
    fn new(identity: u64, drops: &Arc<AtomicUsize>) -> Self {
        Self {
            identity,
            canary: identity ^ 0xb6f2_017a_c933_500d,
            drops: drops.clone(),
        }
    }
}
// SAFETY: these data fields contain no managed edges; graph ownership is entirely
// in the canonical inventory, and destruction only updates an independent counter.
// SAFETY: this fixture has no managed edges and requires no tracing.
unsafe impl Trace for Value {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn error(message: impl Into<String>) -> Error {
    Error::RootError(message.into())
}
fn protocol(e: impl std::fmt::Debug) -> Error {
    error(format!("graph lifecycle: {e:?}"))
}
#[derive(Default)]
struct Report {
    setup: Duration,
    allocation: Duration,
    mutation: Duration,
    read: Duration,
    cleanup: Duration,
    lifecycle: Duration,
    teardown: Duration,
    allocation_polls: u64,
    write_polls: u64,
    read_polls: u64,
    helper_polls: u64,
    objects: u64,
    roots: u64,
    edges: u64,
    checksum: u64,
    catalog_bytes: usize,
    logical_object_bytes: usize,
}
impl Report {
    fn help(&mut self, helper: &mut LeafMaintenance<'_>) -> Result<()> {
        self.helper_polls += 1;
        match helper.poll().map_err(protocol)? {
            MaintenanceStep::ReleasedObjectSlot => self.objects += 1,
            MaintenanceStep::ReleasedRootSlot => self.roots += 1,
            MaintenanceStep::ReleasedEdgeSlot => self.edges += 1,
            _ => {}
        }
        Ok(())
    }
}
fn allocation(
    mut request: GraphAllocation<'_, '_, Value>,
    helper: &mut LeafMaintenance<'_>,
    report: &mut Report,
    limit: usize,
) -> Result<GraphRoot<Value>> {
    for _ in 0..limit {
        report.allocation_polls += 1;
        if let Some(root) = request.poll().map_err(protocol)? {
            return Ok(root);
        }
        report.help(helper)?;
    }
    Err(error("graph allocation poll watchdog"))
}
fn write(
    worker: &mut GraphMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    source: &GraphRoot<Value>,
    target: &GraphRoot<Value>,
    report: &mut Report,
    limit: usize,
) -> Result<()> {
    let mut request = worker.write(source, 0, Some(target)).map_err(protocol)?;
    for _ in 0..limit {
        report.write_polls += 1;
        if request.poll().map_err(protocol)? {
            return Ok(());
        }
        report.help(helper)?;
    }
    Err(error("graph mutation poll watchdog"))
}
fn read(
    worker: &mut GraphMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    source: &GraphRoot<Value>,
    report: &mut Report,
    limit: usize,
) -> Result<GraphRoot<Value>> {
    let mut request = worker.child::<Value, Value>(source, 0).map_err(protocol)?;
    for _ in 0..limit {
        report.read_polls += 1;
        if let Some(value) = request.poll().map_err(protocol)? {
            return match value {
                GraphReadValue::Root(root) => Ok(root),
                GraphReadValue::Null => Err(error("nonnull graph edge returned null")),
            };
        }
        report.help(helper)?;
    }
    Err(error("graph child-read poll watchdog"))
}
fn verify(runtime: &GraphRuntime<'_>, root: &GraphRoot<Value>, expected: u64) -> Result<()> {
    let value = runtime.borrow(root).map_err(protocol)?;
    if black_box(value.identity) != expected || value.canary != (expected ^ 0xb6f2_017a_c933_500d) {
        return Err(error("graph payload identity mismatch"));
    }
    Ok(())
}

pub(in crate::gc::publication_owner) fn run(rounds: u64, capacity: usize) -> Result<String> {
    if !(1..=1_000_000).contains(&rounds) || !(2..=16_384).contains(&capacity) {
        return Err(error(
            "graph lifecycle requires 1..=1000000 rounds and 2..=16384 object slots",
        ));
    }
    if cfg!(all(target_family = "wasm", target_os = "unknown")) {
        return Err(error(
            "graph lifecycle timing requires a native or WASI clock",
        ));
    }
    let limit = (capacity * 2048).max(1_000_000);
    let report = measure(rounds, capacity, limit)?;
    Ok(format!(
        "{{\"version\":1,\"mode\":\"canonical-graph\",\"rounds\":{rounds},\"capacity\":{capacity},\"degree\":2,\"objects\":{},\"payload_bytes\":{},\"logical_object_bytes\":{},\"catalog_bytes\":{},\"poll_limit\":{limit},\"setup_ns\":{},\"allocation_ns\":{},\"mutation_ns\":{},\"read_ns\":{},\"cleanup_ns\":{},\"lifecycle_ns\":{},\"teardown_ns\":{},\"allocation_polls\":{},\"write_polls\":{},\"read_polls\":{},\"helper_polls\":{},\"released_objects\":{},\"released_roots\":{},\"released_edges\":{},\"drops\":{},\"checksum\":{},\"teardown_bytes\":0,\"teardown_catalog_bytes\":0}}",
        rounds * 2,
        size_of::<Value>(),
        report.logical_object_bytes,
        report.catalog_bytes,
        report.setup.as_nanos(),
        report.allocation.as_nanos(),
        report.mutation.as_nanos(),
        report.read.as_nanos(),
        report.cleanup.as_nanos(),
        report.lifecycle.as_nanos(),
        report.teardown.as_nanos(),
        report.allocation_polls,
        report.write_polls,
        report.read_polls,
        report.helper_polls,
        report.objects,
        report.roots,
        report.edges,
        rounds * 2,
        report.checksum,
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "one checked lifecycle boundary and exact teardown"
)]
fn measure(rounds: u64, capacity: usize, limit: usize) -> Result<Report> {
    let setup = Instant::now();
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(64 * 1024 * 1024)
            .build(),
    );
    let capacities = Capacities::derive(capacity, 1, 1)
        .map_err(protocol)?
        .with_roots(capacity * 2)
        .map_err(protocol)?
        .with_edges(capacity * 2)
        .map_err(protocol)?;
    let budget = Budget::new(
        GraphFootprint::new(capacities, 2)
            .map_err(protocol)?
            .total(),
    );
    let runtime = GraphRuntime::new(&gc, capacities, 2, &budget).map_err(protocol)?;
    let mut worker = runtime.mutator().map_err(protocol)?;
    let mut helper = runtime.maintenance().map_err(protocol)?;
    let drops = Arc::new(AtomicUsize::new(0));
    let logical_object_bytes = Gc::allocation_size(&Value::new(0, &drops))?;
    drops.store(0, Ordering::Relaxed);
    let mut report = Report {
        catalog_bytes: budget.used(),
        logical_object_bytes,
        setup: setup.elapsed(),
        ..Report::default()
    };
    let lifecycle = Instant::now();
    for round in 0..rounds {
        let start = Instant::now();
        let a = allocation(
            worker
                .allocate(Value::new(2 * round, &drops))
                .map_err(protocol)?,
            &mut helper,
            &mut report,
            limit,
        )?;
        let b = allocation(
            worker
                .allocate(Value::new(2 * round + 1, &drops))
                .map_err(protocol)?,
            &mut helper,
            &mut report,
            limit,
        )?;
        verify(&runtime, &a, 2 * round)?;
        verify(&runtime, &b, 2 * round + 1)?;
        report.allocation += start.elapsed();
        let start = Instant::now();
        write(&mut worker, &mut helper, &a, &b, &mut report, limit)?;
        write(&mut worker, &mut helper, &b, &a, &mut report, limit)?;
        report.mutation += start.elapsed();
        let start = Instant::now();
        let child = read(&mut worker, &mut helper, &a, &mut report, limit)?;
        verify(&runtime, &child, 2 * round + 1)?;
        report.checksum += 2 * round + 1;
        report.read += start.elapsed();
        let start = Instant::now();
        drop(child);
        drop(a);
        drop(b);
        let expected = round + 1;
        for _ in 0..limit {
            report.help(&mut helper)?;
            if report.objects == 2 * expected
                && report.roots == 3 * expected
                && report.edges == 4 * expected
                && u64::try_from(drops.load(Ordering::Relaxed)).map_err(protocol)? == 2 * expected
            {
                break;
            }
        }
        if report.objects != 2 * expected
            || report.roots != 3 * expected
            || report.edges != 4 * expected
            || u64::try_from(drops.load(Ordering::Relaxed)).map_err(protocol)? != 2 * expected
        {
            return Err(error("graph cleanup watchdog or release-count mismatch"));
        }
        report.cleanup += start.elapsed();
        if gc.managed_bytes() != 0 {
            return Err(error("graph round retained payload credit"));
        }
    }
    report.lifecycle = lifecycle.elapsed();
    if report.checksum != rounds * rounds {
        return Err(error("graph checksum mismatch"));
    }
    let teardown = Instant::now();
    drop(worker);
    drop(helper);
    if runtime.registered() != (0, 0) {
        return Err(error("graph participants retained"));
    }
    drop(runtime);
    if budget.used() != 0 || Arc::strong_count(&drops) != 1 || gc.managed_bytes() != 0 {
        return Err(error("graph teardown ownership or credit leak"));
    }
    drop(budget);
    drop(gc);
    report.teardown = teardown.elapsed();
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cyclic_graph_cost_fixture_reclaims_every_real_value_and_slot() {
        let capacities: &[usize] = if cfg!(miri) { &[2] } else { &[2, 8, 64] };
        for &capacity in capacities {
            let report = measure(3, capacity, 1_000_000).unwrap();
            assert_eq!(
                (report.objects, report.roots, report.edges, report.checksum),
                (6, 9, 12, 9)
            );
        }
    }
    #[test]
    fn graph_cost_fixture_rejects_invalid_inputs_before_setup() {
        for (rounds, capacity) in [(0, 2), (1_000_001, 2), (1, 0), (1, 1), (1, 16_385)] {
            assert!(run(rounds, capacity).is_err());
        }
    }
}
