//! Composition checks use the sealed typed API, real values, and actual cleanup.
use super::*;
use crate::gc::publication_owner::canonical_leaf::allocation::LeafRoot;
use crate::gc::publication_owner::canonical_leaf::maintenance::{LeafMaintenance, MaintenanceStep};
use crate::gc::publication_owner::canonical_leaf::runtime::{LeafRuntime, RuntimeFootprint};
use std::collections::VecDeque;

struct Item {
    actor: u64,
    sequence: u64,
    canary: u64,
    drops: Arc<AtomicUsize>,
}
impl Item {
    fn new(actor: usize, sequence: usize, drops: &Arc<AtomicUsize>) -> Self {
        let actor = actor as u64;
        let sequence = sequence as u64;
        Self {
            actor,
            sequence,
            canary: actor ^ sequence ^ 0x9e37_79b9,
            drops: drops.clone(),
        }
    }
}
// SAFETY: primitive identity words and an independent atomic counter, no GC edges.
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
fn check(runtime: &LeafRuntime<'_>, root: &LeafRoot<Item>, actor: usize, sequence: usize) {
    let value = runtime.borrow(root).expect("retained root must stay live");
    assert_eq!(
        (value.actor, value.sequence),
        (actor as u64, sequence as u64),
        "wrong live identity"
    );
    assert_eq!(value.canary, actor as u64 ^ sequence as u64 ^ 0x9e37_79b9);
}
fn tick(helper: &mut LeafMaintenance<'_>, releases: &mut [usize; 2]) {
    match helper.poll().expect("maintenance progress") {
        MaintenanceStep::ReleasedObjectSlot => releases[0] += 1,
        MaintenanceStep::ReleasedRootSlot => releases[1] += 1,
        _ => {}
    }
}
fn finish(
    helpers: &mut [LeafMaintenance<'_>],
    drops: &AtomicUsize,
    expected: usize,
    releases: &mut [usize; 2],
) {
    for _ in 0..100_000 {
        if *releases == [expected; 2] && drops.load(Ordering::Relaxed) == expected {
            return;
        }
        for helper in &mut *helpers {
            tick(helper, releases);
        }
    }
    assert_eq!(*releases, [expected; 2], "incomplete physical slot release");
    assert_eq!(
        drops.load(Ordering::Relaxed),
        expected,
        "incomplete payload destruction"
    );
}

#[test]
fn contention_cooperative_actors_preserve_retained_identities_and_reclaim_every_round() {
    let (actors, helper_count, keep, rounds) = (3, 2, 2, 12);
    let gc = collector(true, RootStorage::Contexts);
    let capacities = Capacities::derive(actors * (keep + 2), actors, helper_count)
        .unwrap()
        .with_roots(actors * (keep + 2))
        .unwrap();
    let budget = Budget::new(
        RuntimeFootprint::for_capacities(capacities)
            .unwrap()
            .total(),
    );
    let runtime = LeafRuntime::new(&gc, capacities, &budget).unwrap();
    let mut mutators = (0..actors)
        .map(|_| runtime.mutator().unwrap())
        .collect::<Vec<_>>();
    let mut helpers = (0..helper_count)
        .map(|_| runtime.maintenance().unwrap())
        .collect::<Vec<_>>();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut held = (0..actors).map(|_| VecDeque::new()).collect::<Vec<_>>();
    let mut completed = vec![0; actors];
    let mut releases = [0; 2];
    for sequence in 0..rounds {
        for ring in &mut held {
            if ring.len() == keep {
                drop(ring.pop_front());
            }
        }
        {
            let mut requests = mutators
                .iter_mut()
                .enumerate()
                .map(|(actor, worker)| {
                    Some(
                        worker
                            .begin_any(Item::new(actor, sequence, &drops))
                            .unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            for _ in 0..100_000 {
                for (actor, request) in requests.iter_mut().enumerate() {
                    if let Some(root) = request.as_mut().and_then(|pending| pending.poll().unwrap())
                    {
                        check(&runtime, &root, actor, sequence);
                        held[actor].push_back((sequence, root));
                        completed[actor] += 1;
                        *request = None;
                    }
                    for helper in &mut helpers {
                        tick(helper, &mut releases);
                    }
                }
                for (actor, ring) in held.iter().enumerate() {
                    for (identity, root) in ring {
                        check(&runtime, root, actor, *identity);
                    }
                }
                if requests.iter().all(Option::is_none) {
                    break;
                }
            }
            assert!(
                requests.iter().all(Option::is_none),
                "cooperative actors stalled"
            );
        }
        assert!(completed.iter().all(|count| *count == sequence + 1));
    }
    drop(held);
    finish(&mut helpers, &drops, actors * rounds, &mut releases);
    assert_eq!(gc.managed_bytes(), 0);
    drop(helpers);
    drop(mutators);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(drops.load(Ordering::Relaxed), actors * rounds);
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one scoped concurrency fixture and teardown"
)]
fn contention_native_actors_share_a_bounded_heap_without_losing_roots_or_reclamation() {
    use std::sync::Barrier;
    use std::sync::atomic::AtomicBool;
    struct Exit<'a> {
        completed: bool,
        done: &'a AtomicUsize,
        failed: &'a AtomicBool,
    }
    impl Drop for Exit<'_> {
        fn drop(&mut self) {
            if !self.completed {
                self.failed.store(true, Ordering::Release);
            }
            self.done.fetch_add(1, Ordering::Release);
        }
    }
    let (actors, helper_count, keep, rounds) = (3, 2, 2, if cfg!(miri) { 6 } else { 64 });
    let gc = collector(true, RootStorage::Contexts);
    let capacities = Capacities::derive(actors * (keep + 2), actors, helper_count)
        .unwrap()
        .with_roots(actors * (keep + 2))
        .unwrap();
    let budget = Budget::new(
        RuntimeFootprint::for_capacities(capacities)
            .unwrap()
            .total(),
    );
    let runtime = LeafRuntime::new(&gc, capacities, &budget).unwrap();
    let mutators = (0..actors)
        .map(|_| runtime.mutator().unwrap())
        .collect::<Vec<_>>();
    let helpers = (0..helper_count)
        .map(|_| runtime.maintenance().unwrap())
        .collect::<Vec<_>>();
    let drops = Arc::new(AtomicUsize::new(0));
    let barrier = Barrier::new(actors + helper_count);
    let done = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    let released_objects = AtomicUsize::new(0);
    let released_roots = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        let mut readers = Vec::new();
        for (actor, mut worker) in mutators.into_iter().enumerate() {
            let (runtime, drops, barrier, done, failed) =
                (&runtime, &drops, &barrier, &done, &failed);
            readers.push(scope.spawn(move || {
                let mut exit = Exit {
                    completed: false,
                    done,
                    failed,
                };
                let mut held = VecDeque::new();
                barrier.wait();
                for sequence in 0..rounds {
                    if held.len() == keep {
                        drop(held.pop_front());
                    }
                    let mut request = worker.begin_any(Item::new(actor, sequence, drops)).unwrap();
                    let mut received = None;
                    for poll in 0..1_000_000 {
                        assert!(!failed.load(Ordering::Acquire), "another actor failed");
                        if let Some(root) = request.poll().unwrap() {
                            received = Some(root);
                            break;
                        }
                        if poll % 32 == 0 {
                            std::thread::yield_now();
                        }
                    }
                    let root = received.expect("native allocation stalled");
                    check(runtime, &root, actor, sequence);
                    held.push_back((sequence, root));
                    for (identity, root) in &held {
                        check(runtime, root, actor, *identity);
                    }
                }
                drop(held);
                exit.completed = true;
                rounds
            }));
        }
        let mut collectors = Vec::new();
        for mut helper in helpers {
            let (barrier, done, failed, objects, roots) =
                (&barrier, &done, &failed, &released_objects, &released_roots);
            collectors.push(scope.spawn(move || {
                barrier.wait();
                let mut local = [0; 2];
                for poll in 0..5_000_000 {
                    if failed.load(Ordering::Acquire) {
                        return local;
                    }
                    match helper.poll().expect("concurrent maintenance") {
                        MaintenanceStep::ReleasedObjectSlot => {
                            local[0] += 1;
                            objects.fetch_add(1, Ordering::Relaxed);
                        }
                        MaintenanceStep::ReleasedRootSlot => {
                            local[1] += 1;
                            roots.fetch_add(1, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                    if done.load(Ordering::Acquire) == actors
                        && objects.load(Ordering::Relaxed) == actors * rounds
                        && roots.load(Ordering::Relaxed) == actors * rounds
                    {
                        return local;
                    }
                    if poll % 32 == 0 {
                        std::thread::yield_now();
                    }
                }
                failed.store(true, Ordering::Release);
                panic!("concurrent reclamation stalled");
            }));
        }
        for actor in readers {
            assert_eq!(actor.join().unwrap(), rounds);
        }
        let mut total = [0; 2];
        for helper in collectors {
            let counts = helper.join().unwrap();
            total[0] += counts[0];
            total[1] += counts[1];
        }
        assert_eq!(total, [actors * rounds; 2]);
    });
    assert!(!failed.load(Ordering::Acquire));
    assert_eq!(drops.load(Ordering::Relaxed), actors * rounds);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(runtime.registered(), (0, 0));
    drop(runtime);
    assert_eq!(budget.used(), 0);
}
