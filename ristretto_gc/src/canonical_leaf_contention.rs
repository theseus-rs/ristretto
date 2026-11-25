//! Controlled finite-heap stress diagnostic. Its payload has no managed edges,
//! and its Drop is known bounded bookkeeping. This is not general GC qualification.
use super::allocation::LeafRoot;
use super::maintenance::{LeafMaintenance, MaintenanceStep};
use super::runtime::{LeafMutator, LeafRuntime, RuntimeFootprint};
use crate::canonical_catalog as gc_catalog;
use crate::{Configuration, Error, GarbageCollector, Gc, Result, RootStorage, Trace};
use gc_catalog::bounded::{Budget, Capacities};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(target_family = "wasm"))]
#[path = "canonical_leaf_contention_native.rs"]
mod native;

const REQUEST_POLLS: usize = 1_000_000;

struct Item {
    actor: u64,
    sequence: u64,
    canary: u64,
    drops: Arc<AtomicUsize>,
}
impl Item {
    fn new(actor: usize, sequence: usize, drops: &Arc<AtomicUsize>) -> Self {
        let (actor, sequence) = (actor as u64, sequence as u64);
        Self {
            actor,
            sequence,
            canary: actor ^ sequence ^ 0x9e37_79b9,
            drops: drops.clone(),
        }
    }
}
// SAFETY: only primitive words and an independent atomic counter, no GC edges.
unsafe impl Trace for Item {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Item {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn failure(message: impl Into<String>) -> Error {
    Error::RootError(message.into())
}
fn protocol(error: impl std::fmt::Debug) -> Error {
    failure(format!("leaf contention: {error:?}"))
}

#[derive(Clone, Copy)]
struct Config {
    actors: usize,
    helpers: usize,
    operations: usize,
    capacity: usize,
    retained: usize,
}
impl Config {
    fn validate(self, mode: &str) -> Result<()> {
        if !["native", "cooperative"].contains(&mode)
            || !(1..=16).contains(&self.actors)
            || !(1..=8).contains(&self.helpers)
            || !(1..=10_000_000).contains(&self.operations)
            || !(1..=16384).contains(&self.capacity)
            || self.retained > 64
            || self.capacity < self.actors * (self.retained + 1)
        {
            return Err(failure("invalid leaf contention fixture"));
        }
        if cfg!(target_family = "wasm") && mode == "native" {
            return Err(failure("native contention requires threads"));
        }
        Ok(())
    }
    fn objects(self) -> usize {
        self.actors * self.operations
    }
    fn request_limit(self) -> usize {
        // Empty discovery slots and complete retirement sweeps are real work.
        // A fixed limit gave large catalogs less than one adequate cleanup
        // window. This is a declared diagnostic allowance, not a latency bound.
        REQUEST_POLLS.max(self.capacity * 1024)
    }
    fn helper_limit(self) -> u64 {
        // This is a finite stress watchdog, not a protocol progress bound.
        (self.objects() as u64 * 8192).max(5_000_000)
    }
}

#[derive(Default)]
struct ActorStats {
    completed: usize,
    polls: u64,
    max_polls: usize,
    reads: u64,
    checksum: u64,
}
struct Actor {
    id: usize,
    retained: usize,
    held: VecDeque<(usize, LeafRoot<Item>)>,
    stats: ActorStats,
}
impl Actor {
    fn new(id: usize, retained: usize) -> Self {
        Self {
            id,
            retained,
            held: VecDeque::with_capacity(retained),
            stats: ActorStats::default(),
        }
    }
    fn evict(&mut self) {
        if self.held.len() == self.retained {
            drop(self.held.pop_front());
        }
    }
    fn check(
        &mut self,
        runtime: &LeafRuntime<'_>,
        sequence: usize,
        root: &LeafRoot<Item>,
    ) -> Result<u64> {
        let value = runtime.borrow(root).map_err(protocol)?;
        if value.actor != self.id as u64
            || value.sequence != sequence as u64
            || value.canary != self.id as u64 ^ sequence as u64 ^ 0x9e37_79b9
        {
            return Err(failure(format!(
                "wrong live identity: actor {} sequence {sequence}",
                self.id
            )));
        }
        self.stats.reads += 1;
        Ok(value.actor + value.sequence)
    }
    fn accept(
        &mut self,
        runtime: &LeafRuntime<'_>,
        root: LeafRoot<Item>,
        polls: usize,
    ) -> Result<()> {
        let sequence = self.stats.completed;
        self.stats.checksum += self.check(runtime, sequence, &root)?;
        self.stats.completed += 1;
        self.stats.max_polls = self.stats.max_polls.max(polls);
        if self.retained != 0 {
            self.held.push_back((sequence, root));
            // Read the oldest retained values after each completed allocation;
            // helper work runs between these checks and may reclaim other slots.
            let held = std::mem::take(&mut self.held);
            for (sequence, root) in &held {
                self.check(runtime, *sequence, root)?;
            }
            self.held = held;
        }
        Ok(())
    }
}

#[derive(Default)]
struct HelperStats {
    polls: u64,
    objects: usize,
    roots: usize,
}
impl HelperStats {
    fn poll(&mut self, helper: &mut LeafMaintenance<'_>) -> Result<MaintenanceStep> {
        self.polls += 1;
        let step = helper.poll().map_err(protocol)?;
        match step {
            MaintenanceStep::ReleasedObjectSlot => self.objects += 1,
            MaintenanceStep::ReleasedRootSlot => self.roots += 1,
            _ => {}
        }
        Ok(step)
    }
}
type Progress = (Vec<ActorStats>, Vec<HelperStats>);

fn cooperative(
    runtime: &LeafRuntime<'_>,
    mut mutators: Vec<LeafMutator<'_>>,
    mut helpers: Vec<LeafMaintenance<'_>>,
    drops: &Arc<AtomicUsize>,
    config: Config,
) -> Result<Progress> {
    let mut actors = (0..config.actors)
        .map(|id| Actor::new(id, config.retained))
        .collect::<Vec<_>>();
    let mut stats = (0..config.helpers)
        .map(|_| HelperStats::default())
        .collect::<Vec<_>>();
    for sequence in 0..config.operations {
        for actor in &mut actors {
            actor.evict();
        }
        let mut requests = mutators
            .iter_mut()
            .enumerate()
            .map(|(id, worker)| {
                worker
                    .begin_any(Item::new(id, sequence, drops))
                    .map(Some)
                    .map_err(protocol)
            })
            .collect::<Result<Vec<_>>>()?;
        for polls in 1..=config.request_limit() {
            for (actor, request) in actors.iter_mut().zip(&mut requests) {
                if let Some(pending) = request {
                    actor.stats.polls += 1;
                    if let Some(root) = pending.poll().map_err(protocol)? {
                        actor.accept(runtime, root, polls)?;
                        *request = None;
                    }
                }
                for (helper, counts) in helpers.iter_mut().zip(&mut stats) {
                    counts.poll(helper)?;
                }
            }
            if requests.iter().all(Option::is_none) {
                break;
            }
        }
        if requests.iter().any(Option::is_some) {
            return Err(failure(format!(
                "cooperative allocation stalled at sequence {sequence}"
            )));
        }
    }
    let actor_stats = actors.into_iter().map(|actor| actor.stats).collect();
    for _ in 0..config.helper_limit() {
        if stats.iter().map(|s| s.objects).sum::<usize>() == config.objects()
            && stats.iter().map(|s| s.roots).sum::<usize>() == config.objects()
            && drops.load(Ordering::Relaxed) == config.objects()
        {
            return Ok((actor_stats, stats));
        }
        for (helper, counts) in helpers.iter_mut().zip(&mut stats) {
            counts.poll(helper)?;
        }
    }
    Err(failure(format!(
        "cooperative reclamation stalled: objects={} roots={} drops={}",
        stats.iter().map(|s| s.objects).sum::<usize>(),
        stats.iter().map(|s| s.roots).sum::<usize>(),
        drops.load(Ordering::Relaxed)
    )))
}

pub(in crate::gc::publication_owner) fn run(
    mode: &str,
    actors: usize,
    helpers: usize,
    operations: usize,
    capacity: usize,
    retained: usize,
) -> Result<String> {
    let config = Config {
        actors,
        helpers,
        operations,
        capacity,
        retained,
    };
    config.validate(mode)?;
    let request_poll_limit = config.request_limit();
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(64 * 1024 * 1024)
            .build(),
    );
    let capacities = Capacities::derive(capacity, actors, helpers)
        .map_err(protocol)?
        .with_roots(capacity)
        .map_err(protocol)?;
    let budget = Budget::new(
        RuntimeFootprint::for_capacities(capacities)
            .map_err(protocol)?
            .total(),
    );
    let runtime = LeafRuntime::new(&gc, capacities, &budget).map_err(protocol)?;
    let catalog_bytes = runtime.charged_bytes();
    let mutators = (0..actors)
        .map(|_| runtime.mutator().map_err(protocol))
        .collect::<Result<Vec<_>>>()?;
    let helpers = (0..helpers)
        .map(|_| runtime.maintenance().map_err(protocol))
        .collect::<Result<Vec<_>>>()?;
    let drops = Arc::new(AtomicUsize::new(0));
    let logical_object_bytes = Gc::allocation_size(&Item::new(0, 0, &drops))?;
    // The size probe above has already dropped; begin the workload at zero.
    drops.store(0, Ordering::Relaxed);
    let (actors, helpers) = match mode {
        #[cfg(not(target_family = "wasm"))]
        "native" => native::run(&runtime, mutators, helpers, &drops, config)?,
        _ => cooperative(&runtime, mutators, helpers, &drops, config)?,
    };
    let destructions = drops.load(Ordering::Relaxed);
    let payload_bytes = gc.managed_bytes();
    let registered = runtime.registered();
    drop(runtime);
    let catalog_teardown = budget.used();
    let counter_owners = Arc::strong_count(&drops);
    if destructions != config.objects()
        || payload_bytes != 0
        || registered != (0, 0)
        || catalog_teardown != 0
        || counter_owners != 1
    {
        return Err(failure(format!(
            "contention teardown: destroyed={destructions} payload={payload_bytes} participants={registered:?} catalog={catalog_teardown} counter_owners={counter_owners}"
        )));
    }
    for (id, stats) in actors.iter().enumerate() {
        let operations = config.operations as u64;
        let expected = id as u64 * operations + operations * (operations - 1) / 2;
        if stats.completed != config.operations || stats.checksum != expected {
            return Err(failure(format!(
                "incomplete actor {id}: completed={} checksum={}",
                stats.completed, stats.checksum
            )));
        }
    }
    if helpers.iter().map(|s| s.objects).sum::<usize>() != config.objects()
        || helpers.iter().map(|s| s.roots).sum::<usize>() != config.objects()
    {
        return Err(failure("incomplete physical slot release"));
    }
    let actors = actors.iter().enumerate().map(|(id, s)| format!(
        "{{\"actor\":{id},\"completed\":{},\"allocation_polls\":{},\"max_allocation_polls\":{},\"reads\":{},\"checksum\":{}}}",
        s.completed, s.polls, s.max_polls, s.reads, s.checksum)).collect::<Vec<_>>().join(",");
    let helpers = helpers
        .iter()
        .enumerate()
        .map(|(id, s)| {
            format!(
                "{{\"helper\":{id},\"polls\":{},\"released_objects\":{},\"released_roots\":{}}}",
                s.polls, s.objects, s.roots
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!(
        "{{\"version\":2,\"request_poll_limit\":{request_poll_limit},\"mode\":\"{mode}\",\"actors\":{},\"helpers\":{},\"operations_per_actor\":{},\"capacity\":{},\"retained\":{},\"objects\":{},\"catalog_bytes\":{catalog_bytes},\"logical_object_bytes\":{logical_object_bytes},\"destructions\":{destructions},\"payload_teardown_bytes\":{payload_bytes},\"catalog_teardown_bytes\":{catalog_teardown},\"counter_owners\":{counter_owners},\"actor_progress\":[{actors}],\"helper_progress\":[{helpers}]}}",
        config.actors,
        config.helpers,
        config.operations,
        config.capacity,
        config.retained,
        config.objects()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contention_driver_cooperative_reclaims_at_minimum_capacity() {
        for retained in [0, 2] {
            let report = run("cooperative", 3, 2, 6, 3 * (retained + 1), retained).unwrap();
            assert!(report.contains("\"destructions\":18,"));
            assert!(report.contains("\"counter_owners\":1,"));
        }
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn contention_driver_native_reclaims_at_minimum_capacity() {
        for retained in [0, 2] {
            let report = run("native", 3, 2, 6, 3 * (retained + 1), retained).unwrap();
            assert!(report.contains("\"destructions\":18,"));
            assert!(report.contains("\"counter_owners\":1,"));
        }
    }

    #[test]
    fn contention_driver_rejects_unserviceable_configuration_before_setup() {
        for (mode, actors, helpers, operations, capacity, retained) in [
            ("invalid", 1, 1, 1, 1, 0),
            ("cooperative", 0, 1, 1, 1, 0),
            ("cooperative", 1, 0, 1, 1, 0),
            ("cooperative", 1, 1, 0, 1, 0),
            ("cooperative", 1, 1, 1, 0, 0),
            ("cooperative", 1, 1, 1, 1, 1),
            ("cooperative", usize::MAX, 1, 1, usize::MAX, usize::MAX),
        ] {
            assert!(run(mode, actors, helpers, operations, capacity, retained).is_err());
        }
        #[cfg(target_family = "wasm")]
        assert!(run("native", 1, 1, 1, 1, 0).is_err());
    }
}
