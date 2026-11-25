//! Shared owning-root cost and backing audit. Checkpoints allocate nothing in
//! the supplied examples. Callback cost is outside the timed clone/drop loop.
use super::super::allocation::{LeafAllocation, LeafRoot};
use super::super::runtime::{LeafMutator, LeafRuntime, RuntimeFootprint};
use super::super::{
    LeafRecord,
    maintenance::LeafMaintenance,
    root_owner::{PreparedRoot, backing_bytes},
};
use super::{
    GraphAllocation, GraphFootprint, GraphMutator, GraphRead, GraphRoot, GraphRuntime, GraphWrite,
    Worker,
};
use crate::canonical_catalog as gc_catalog;
use crate::{Configuration, Error, Gc, Result, RootStorage};
use crate::{GarbageCollector, Trace};
use gc_catalog::bounded::{Budget, Capacities, RootHandle};
use std::hint::black_box;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const LIMIT: usize = 1_000_000;
const IDENTITY: u64 = 0xa4c9_7015_ee12_853b;
struct Value {
    identity: u64,
    drops: Arc<AtomicUsize>,
}
// SAFETY: the payload holds no managed edges or callbacks; its destructor only
// updates an independent counter. Graph inventory is owned by the catalog.
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
fn protocol(value: impl std::fmt::Debug) -> Error {
    error(format!("root aliases: {value:?}"))
}
fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(error(message))
    }
}
fn clones<T: Clone>(root: &T, copies: u64, checkpoint: &mut impl FnMut(&'static str)) -> Duration {
    checkpoint("clones-before");
    let start = Instant::now();
    for _ in 0..copies {
        let alias = black_box(root).clone();
        black_box(&alias);
        drop(alias);
    }
    let elapsed = start.elapsed();
    checkpoint("clones-after");
    elapsed
}
fn clean(helper: &mut LeafMaintenance<'_>, drops: &AtomicUsize, edges: usize) -> Result<()> {
    let (mut objects, mut roots, mut released_edges) = (0, 0, 0);
    for _ in 0..LIMIT {
        match helper.poll().map_err(protocol)? {
            super::super::maintenance::MaintenanceStep::ReleasedObjectSlot => objects += 1,
            super::super::maintenance::MaintenanceStep::ReleasedRootSlot => roots += 1,
            super::super::maintenance::MaintenanceStep::ReleasedEdgeSlot => released_edges += 1,
            _ => {}
        }
        if objects == 1
            && roots == 1
            && released_edges == edges
            && drops.load(Ordering::Relaxed) == 1
        {
            return Ok(());
        }
    }
    Err(error("root alias cleanup watchdog"))
}

#[expect(
    clippy::too_many_lines,
    reason = "three identical clone boundaries with explicit owning setup and teardown"
)]
pub(in crate::gc::publication_owner) fn run(
    mode: &str,
    copies: u64,
    mut checkpoint: impl FnMut(&'static str),
) -> Result<String> {
    require(
        ["legacy", "canonical-leaf", "canonical-graph"].contains(&mode)
            && (1..=10_000_000).contains(&copies),
        "invalid root alias mode or copies",
    )?;
    require(
        !cfg!(all(target_family = "wasm", target_os = "unknown")),
        "root alias timing requires native or WASI clocks",
    )?;
    let backing = backing_bytes().ok_or_else(|| error("root backing layout overflow"))?;
    checkpoint("prepared-before");
    let prepared = PreparedRoot::new();
    black_box(&prepared);
    checkpoint("prepared-after");
    drop(prepared);
    checkpoint("prepared-dropped");
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(64 * 1024 * 1024)
            .build(),
    );
    let context = gc.register_context();
    let drops = Arc::new(AtomicUsize::new(0));
    let value = Value {
        identity: IDENTITY,
        drops: drops.clone(),
    };
    let mut charged = 0;
    let mut inline_total = 0;
    let mut backing_slots = 0;
    let elapsed = if mode == "legacy" {
        context.with(|| -> Result<Duration> {
            let root = Gc::try_new(&gc, value)?;
            let elapsed = clones(&root, copies, &mut checkpoint);
            let alias = root.clone();
            drop(root);
            require(
                alias.identity == IDENTITY && drops.load(Ordering::Relaxed) == 0,
                "legacy alias lost payload",
            )?;
            drop(alias);
            context.flush();
            gc.collect_full()?;
            Ok(elapsed)
        })?
    } else {
        let mut capacities = Capacities::derive(2, 1, 1)
            .map_err(protocol)?
            .with_roots(1)
            .map_err(protocol)?;
        backing_slots = capacities.root_slots() + capacities.mutators();
        if mode == "canonical-leaf" {
            let footprint = RuntimeFootprint::for_capacities(capacities).map_err(protocol)?;
            let budget = Budget::new(footprint.total());
            let runtime = LeafRuntime::new(&gc, capacities, &budget).map_err(protocol)?;
            charged = budget.used();
            // Independent inline census. The counting example supplies actual
            // allocator-request bytes for the remaining root/request backing.
            inline_total = footprint.catalog().total()
                + size_of::<LeafRuntime<'_>>()
                + size_of::<LeafMutator<'_>>()
                + size_of::<LeafAllocation<'_, ()>>()
                + size_of::<LeafMaintenance<'_>>()
                + size_of::<LeafRoot<()>>()
                - 2 * size_of::<Worker>()
                - size_of::<RootHandle<LeafRecord>>();
            let mut worker = runtime.mutator().map_err(protocol)?;
            let mut helper = runtime.maintenance().map_err(protocol)?;
            let root = {
                let mut request = worker.begin_any(value).map_err(protocol)?;
                let mut result = None;
                for _ in 0..LIMIT {
                    if let Some(root) = request.poll().map_err(protocol)? {
                        result = Some(root);
                        break;
                    }
                    helper.poll().map_err(protocol)?;
                }
                result.ok_or_else(|| error("leaf alias allocation watchdog"))?
            };
            let elapsed = clones(&root, copies, &mut checkpoint);
            let alias = root.clone();
            drop(root);
            require(
                runtime.borrow(&alias).map_err(protocol)?.identity == IDENTITY
                    && drops.load(Ordering::Relaxed) == 0,
                "leaf alias lost payload",
            )?;
            drop(alias);
            clean(&mut helper, &drops, 0)?;
            drop(helper);
            drop(worker);
            drop(runtime);
            require(budget.used() == 0, "leaf alias catalog credit leak")?;
            elapsed
        } else {
            capacities = capacities.with_edges(2).map_err(protocol)?;
            let footprint = GraphFootprint::new(capacities, 1).map_err(protocol)?;
            let budget = Budget::new(footprint.total());
            let runtime = GraphRuntime::new(&gc, capacities, 1, &budget).map_err(protocol)?;
            charged = budget.used();
            inline_total = footprint.catalog.total()
                + size_of::<GraphRuntime<'_>>()
                + size_of::<GraphMutator<'_>>()
                + size_of::<GraphAllocation<'_, '_, ()>>()
                    .max(size_of::<GraphRead<'_, '_, ()>>())
                    .max(size_of::<GraphWrite<'_, '_>>())
                + size_of::<LeafMaintenance<'_>>()
                + size_of::<GraphRoot<()>>()
                - 2 * size_of::<Worker>()
                - size_of::<RootHandle<LeafRecord>>();
            let mut worker = runtime.mutator().map_err(protocol)?;
            let mut helper = runtime.maintenance().map_err(protocol)?;
            let root = {
                let mut request = worker.allocate(value).map_err(protocol)?;
                let mut result = None;
                for _ in 0..LIMIT {
                    if let Some(root) = request.poll().map_err(protocol)? {
                        result = Some(root);
                        break;
                    }
                    helper.poll().map_err(protocol)?;
                }
                result.ok_or_else(|| error("graph alias allocation watchdog"))?
            };
            let elapsed = clones(&root, copies, &mut checkpoint);
            require(
                runtime.catalog.reserved_root_tokens() == 1,
                "graph aliases changed root quota",
            )?;
            let alias = root.clone();
            drop(root);
            require(
                runtime.borrow(&alias).map_err(protocol)?.identity == IDENTITY
                    && drops.load(Ordering::Relaxed) == 0,
                "graph alias lost payload",
            )?;
            drop(alias);
            require(
                runtime.catalog.reserved_root_tokens() == 0,
                "graph last alias leaked root quota",
            )?;
            clean(&mut helper, &drops, 1)?;
            drop(helper);
            drop(worker);
            drop(runtime);
            require(budget.used() == 0, "graph alias catalog credit leak")?;
            elapsed
        }
    };
    require(
        drops.load(Ordering::Relaxed) == 1 && gc.managed_bytes() == 0,
        "root alias payload or credit leak",
    )?;
    Ok(format!(
        "{{\"version\":1,\"mode\":\"{mode}\",\"copies\":{copies},\"clone_drop_ns\":{},\"backing_bytes\":{backing},\"backing_slots\":{backing_slots},\"inline_total_bytes\":{inline_total},\"charged_bytes\":{charged},\"drops\":1,\"checksum\":{IDENTITY},\"teardown_bytes\":0,\"teardown_catalog_bytes\":0}}",
        elapsed.as_nanos()
    ))
}

/// Final alias owns the only remaining catalog lease. The supplied collector,
/// budget and payload counter outlive both checkpoints, so their own backing
/// cannot obscure the final runtime-owner free/refund boundary.
#[expect(
    clippy::too_many_lines,
    reason = "paired leaf/graph audit with explicit final-owner teardown"
)]
#[cfg(feature = "experimental-canonical-audit")]
pub(in crate::gc::publication_owner) fn refund_order(
    mode: &str,
    mut checkpoint: impl FnMut(&'static str, usize),
) -> Result<()> {
    require(
        ["leaf", "graph"].contains(&mode),
        "invalid root refund mode",
    )?;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let drops = Arc::new(AtomicUsize::new(0));
    let value = Value {
        identity: IDENTITY,
        drops: drops.clone(),
    };
    let capacities = Capacities::derive(2, 1, 1)
        .map_err(protocol)?
        .with_roots(1)
        .map_err(protocol)?;
    if mode == "leaf" {
        let budget = Budget::new(
            RuntimeFootprint::for_capacities(capacities)
                .map_err(protocol)?
                .total(),
        );
        let runtime = LeafRuntime::new(&gc, capacities, &budget).map_err(protocol)?;
        let mut worker = runtime.mutator().map_err(protocol)?;
        let mut helper = runtime.maintenance().map_err(protocol)?;
        let root = {
            let mut request = worker.begin_any(value).map_err(protocol)?;
            let mut result = None;
            for _ in 0..LIMIT {
                if let Some(root) = request.poll().map_err(protocol)? {
                    result = Some(root);
                    break;
                }
                helper.poll().map_err(protocol)?;
            }
            result.ok_or_else(|| error("refund leaf allocation watchdog"))?
        };
        let alias = root.clone();
        require(
            runtime.borrow(&alias).map_err(protocol)?.identity == IDENTITY,
            "refund leaf identity mismatch",
        )?;
        let charge = budget.used();
        drop(helper);
        drop(worker);
        drop(runtime);
        drop(root);
        require(
            charge != 0 && budget.used() == charge && drops.load(Ordering::Relaxed) == 0,
            "leaf alias lost runtime charge or payload",
        )?;
        checkpoint("before", budget.used());
        drop(alias);
        checkpoint("after", budget.used());
        require(budget.used() == 0, "leaf alias retained catalog credit")?;
    } else {
        let capacities = capacities.with_edges(2).map_err(protocol)?;
        let budget = Budget::new(
            GraphFootprint::new(capacities, 1)
                .map_err(protocol)?
                .total(),
        );
        let runtime = GraphRuntime::new(&gc, capacities, 1, &budget).map_err(protocol)?;
        let mut worker = runtime.mutator().map_err(protocol)?;
        let mut helper = runtime.maintenance().map_err(protocol)?;
        let root = {
            let mut request = worker.allocate(value).map_err(protocol)?;
            let mut result = None;
            for _ in 0..LIMIT {
                if let Some(root) = request.poll().map_err(protocol)? {
                    result = Some(root);
                    break;
                }
                helper.poll().map_err(protocol)?;
            }
            result.ok_or_else(|| error("refund graph allocation watchdog"))?
        };
        let alias = root.clone();
        require(
            runtime.borrow(&alias).map_err(protocol)?.identity == IDENTITY,
            "refund graph identity mismatch",
        )?;
        let charge = budget.used();
        drop(helper);
        drop(worker);
        drop(runtime);
        drop(root);
        require(
            charge != 0 && budget.used() == charge && drops.load(Ordering::Relaxed) == 0,
            "graph alias lost runtime charge or payload",
        )?;
        checkpoint("before", budget.used());
        drop(alias);
        checkpoint("after", budget.used());
        require(budget.used() == 0, "graph alias retained catalog credit")?;
    }
    require(
        drops.load(Ordering::Relaxed) == 1 && gc.managed_bytes() == 0,
        "refund audit payload or heap credit leak",
    )
}
