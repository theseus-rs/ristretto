//! Held metadata indexes must not retain global locks or lose sweep entries.
#![expect(clippy::panic_in_result_fn, clippy::unwrap_in_result)]
use super::*;
use std::sync::mpsc;
use std::time::Duration;

struct Leaf(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
unsafe impl Trace for Leaf {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Leaf {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Release);
    }
}

#[derive(Clone, Copy, Debug)]
enum HeldIndex {
    Objects,
    Remembered,
}

#[test]
#[cfg_attr(miri, ignore = "forced native thread schedule")]
fn reserved_guard_trace_finishes_its_step_while_the_child_shard_is_held() -> Result<()> {
    struct Parent {
        child: GcRootGuard<u64>,
        unrelated_address: usize,
        entered: mpsc::Sender<()>,
        resume: Mutex<Option<mpsc::Receiver<()>>>,
        traced: Arc<AtomicBool>,
    }

    // SAFETY: the child is an independently rooted guard. The numerical
    // address used to challenge the identity check is never dereferenced.
    unsafe impl Trace for Parent {
        fn trace(&self, gc: &GarbageCollector) {
            if TraceRetention::current(gc).is_some_and(|(_, reserved)| reserved)
                && let Some(resume) = lock(&self.resume).take()
            {
                self.entered.send(()).expect("parent entered");
                resume
                    .recv_timeout(Duration::from_secs(10))
                    .expect("resume parent");
                self.child.trace(gc);
                gc.mark_object(SafePtr(self.unrelated_address), self.child.identity());
                self.traced.store(true, Ordering::Release);
            } else {
                self.child.trace(gc);
            }
        }
    }

    let mut outcomes = vec![];
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .threads(0)
                    .concurrent_publication(counted)
                    .independent_marking(independent)
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 41_u64)?;
            let address = child.as_ptr() as usize;
            let witness = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let unrelated = Gc::try_new(&gc, 17_u64)?;
            let unrelated_address = unrelated.as_ptr() as usize;
            let dead = crate::GcWeak::new(&unrelated, crate::ReferenceKind::Weak, 0);
            let (entered, started) = mpsc::channel();
            let (resume, released) = mpsc::channel();
            let traced = Arc::new(AtomicBool::new(false));
            let parent = Gc::try_new(
                &gc,
                Parent {
                    child,
                    unrelated_address,
                    entered,
                    resume: Mutex::new(Some(released)),
                    traced: traced.clone(),
                },
            )?;
            drop(unrelated);
            let ticket = gc.request_full_cycle();
            let (returned, step_done) = mpsc::channel();
            let heap = &gc;
            let advanced = thread::scope(|scope| {
                scope.spawn(move || {
                    for _ in 0..4096 {
                        heap.step();
                        if traced.load(Ordering::Acquire) {
                            returned.send(()).expect("step returned");
                            break;
                        }
                    }
                });
                started
                    .recv_timeout(Duration::from_secs(3))
                    .expect("reserved trace");
                let shard = gc.objects.lock(address);
                resume.send(()).expect("trace child");
                let advanced = step_done.recv_timeout(Duration::from_secs(1)).is_ok();
                drop(shard);
                advanced
            });
            assert!((0..4096).any(|_| {
                gc.step();
                gc.completed_full_ticket() >= ticket
            }));
            assert!(witness.refers_to(&parent.child));
            assert_eq!(**parent.child, 41);
            outcomes.push((counted, independent, advanced, dead.upgrade().is_none()));
        }
    }
    assert!(
        outcomes.iter().all(|row| row.2 && row.3),
        "(counted publication, independent marking, step returned, unrelated cleared): {outcomes:?}"
    );
    Ok(())
}

#[cfg(not(miri))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "forced index waits across publication/root policies"
)]
fn reserved_address_allows_reclamation_under_held_live_operation_guards() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for storage in [
                crate::RootStorage::Tree,
                crate::RootStorage::Slots,
                crate::RootStorage::Contexts,
            ] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(storage)
                        .metadata_shards(16)
                        .promotion_age(usize::MAX)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .diagnostics(true)
                        .build(),
                );
                assert_eq!(gc.registered_workers(), 0);
                let context = gc.register_context();
                let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
                let drops = Arc::new(AtomicUsize::new(0));
                let mut victims = (0..32)
                    .map(|_| Gc::try_new(&gc, Leaf(drops.clone())))
                    .collect::<Result<Vec<_>>>()?;
                // Other shards must retire garbage while this one remains held.
                victims.clear();
                gc.collect();
                for _ in 0..10000 {
                    if lock(&gc.domains[1].cycle).phase == Phase::Sweep {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Sweep);
                let (prepared, preparation) = mpsc::channel();
                let (resume, resumed) = mpsc::channel();
                *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
                    entered: prepared,
                    resume: resumed,
                });
                let (waiting, observed) = mpsc::channel();
                *lock(&gc.metadata_registration_observer) = Some(waiting);
                let (inside, _entered) = mpsc::channel();
                let (finish, resumed_commit) = mpsc::channel();
                let worker_gc = gc.clone();
                let worker_context = context.clone();
                let constructor = thread::spawn(move || {
                    publication_tests::INDEX_COMMIT_PAUSE.set(Some(ClaimPause {
                        entered: inside,
                        resume: resumed_commit,
                    }));
                    worker_context.with(|| Gc::try_new(&worker_gc, vec![child]))
                });
                preparation
                    .recv_timeout(Duration::from_secs(5))
                    .expect("metadata prepared");
                let all = (0..16)
                    .map(|bucket| gc.objects.lock_bucket(bucket))
                    .collect::<Vec<_>>();
                resume.send(()).expect("resume allocation");
                let (address, identity) = observed
                    .recv_timeout(Duration::from_secs(5))
                    .expect("address reservation despite held guards");
                let bucket = crate::index::address_bucket(address) & 15;
                let held = all
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, guard)| (i == bucket).then_some(guard))
                    .collect::<Vec<_>>();
                let partial_object = gc
                    .objects
                    .try_lock(address)
                    .map(|objects| objects.occupied());
                let partial_identity = gc.identities.get(identity.to_i64().cast_unsigned());
                let admission_free = gc.publication_clock.as_ref().map_or_else(
                    || gc.publication.try_write().is_some(),
                    |clock| clock.active() == 0,
                );
                let collector_gc = gc.clone();
                let observed_drops = drops.clone();
                let (finished, completion) = mpsc::channel();
                let collector = thread::spawn(move || {
                    for _ in 0..10000 {
                        collector_gc.step();
                    }
                    let progressed = observed_drops.load(Ordering::Acquire) != 0;
                    finished.send(progressed).expect("collection progress");
                });
                let progress = completion.recv_timeout(Duration::from_secs(5));
                // Unblock every participant before reporting a failed schedule.
                drop(held);
                finish.send(()).expect("finish installation");
                collector.join().expect("collector worker");
                let parent = constructor.join().expect("constructor worker")?;
                println!(
                    "registration: counted={concurrent} independent={independent} storage={storage:?} progress={progress:?} admission_free={admission_free}"
                );
                assert!(admission_free, "index wait retained global admission");
                assert_eq!(progress, Ok(true));
                assert!(partial_object.is_none_or(|visible| !visible));
                assert!(partial_identity.is_none());
                assert_eq!(*parent[0], 42);
                assert_eq!(
                    gc.address_for_identity(parent.identity()),
                    Some(parent.as_ptr() as usize)
                );
                assert!(gc.objects.contains_young(parent.as_ptr() as usize));
                drop((parent, victims, context));
                gc.stop()?;
                assert_eq!(drops.load(Ordering::Acquire), 32);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn registered_root_scans_trace_the_graph_while_its_object_shard_is_parked() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for storage in [
                crate::RootStorage::Tree,
                crate::RootStorage::Slots,
                crate::RootStorage::Contexts,
            ] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .root_storage(storage)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let root =
                    context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
                let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
                gc.collect();
                // A first-page slot can be read in the very first step. Hold
                // metadata before any collector work, including cycle startup.
                let child = gc
                    .objects
                    .lookup(root[0].as_ptr() as usize)
                    .expect("registered child");
                let objects = gc.objects.lock(root.as_ptr() as usize);
                let worker_gc = gc.clone();
                let (finished, result) = mpsc::channel();
                let worker = thread::spawn(move || {
                    let marked = (0..4096).any(|_| {
                        worker_gc.step();
                        let epoch = worker_gc.domains[1].epoch.load(Ordering::Acquire);
                        epoch != 0 && child.retention(1, epoch) == Some(Retention::Strong)
                    });
                    finished.send(marked).expect("root steps complete");
                });
                let yielded = result.recv_timeout(Duration::from_secs(1));
                let coordinator_free = gc.coordinator.try_lock().is_ok();
                let admission_free = gc.publication_clock.as_ref().map_or_else(
                    || gc.publication.try_write().is_some(),
                    |clock| clock.active() == 0,
                );
                drop(objects);
                worker.join().expect("root worker");
                assert_eq!(yielded, Ok(true), "parked root shard held graph traversal");
                assert!(coordinator_free && admission_free);
                gc.collect_full()?;
                let recovered = weak.upgrade().expect("retried root graph survived");
                assert_eq!(*recovered[0], 42);
                drop((recovered, root, context));
                gc.stop()?;
                assert!(weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(not(miri))]
#[test]
fn capped_allocation_finishes_while_a_nursery_reader_is_parked() -> Result<()> {
    use std::future::Future;
    use std::task::{Context as TaskContext, Poll, Waker};
    let bytes = Gc::allocation_size(&41_u64)?;
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(1)
                .concurrent_publication(counted)
                .max_heap_bytes(2 * bytes)
                .allocation_threshold(usize::MAX)
                .promotion_age(1)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let survivor = Gc::try_new(&gc, 41_u64)?;
        let survivor_metadata = gc
            .metadata_for_address(survivor.as_ptr() as usize)
            .expect("survivor");
        let garbage = Gc::try_new(&gc, 7_u64)?;
        let weak = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
        drop(garbage);
        let (entered, parked) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let reader_gc = gc.clone();
        let reader = thread::spawn(move || {
            let _hint = reader_gc.objects.scan_young_with(
                &mut crate::index::YoungCursor::default(),
                || {
                    entered.send(()).expect("reader parked on a real directory");
                    resumed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("resume reader");
                },
            );
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("nursery view acquired");
        let mut allocation = Box::pin(Gc::try_new_async(&gc, 23_u64));
        let mut cx = TaskContext::from_waker(Waker::noop());
        let outcome = (0..4096).find_map(|_| match allocation.as_mut().poll(&mut cx) {
            Poll::Ready(result) => Some(result),
            Poll::Pending => None,
        });
        drop(allocation);
        let finished = (0..4096).any(|_| gc.step());
        let freed = gc.statistics.load().bytes_freed;
        let promoted = survivor_metadata.age.load(Ordering::Acquire) == 1;
        resume.send(()).expect("resume nursery reader");
        reader.join().expect("reader");
        let peer = outcome.expect("capped allocation waited for nursery reader")?;
        assert!(freed >= bytes);
        assert!(finished && promoted);
        assert!(weak.upgrade().is_none());
        assert_eq!((**survivor, **peer), (41, 23));
        drop((survivor, peer));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn promotion_waits_only_for_the_object_shard_and_preserves_generation_membership() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for remembered in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .independent_phases(independent)
                        .metadata_shards(16)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .allocation_threshold(usize::MAX)
                        .full_collection_interval(usize::MAX)
                        .promotion_age(1)
                        .build(),
                );
                let root = Gc::try_new(&gc, 42_u64)?;
                let address = root.as_ptr() as usize;
                let metadata = gc.metadata_for_address(address).expect("root metadata");
                gc.collect_young();
                for _ in 0..10000 {
                    if lock(&gc.domains[0].cycle).phase == Phase::Sweep {
                        break;
                    }
                    gc.step();
                }
                assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Sweep);
                let (entered, observed) = mpsc::channel();
                let (release, released) = mpsc::channel();
                let holder_gc = gc.clone();
                let holder = thread::spawn(move || {
                    let _objects = (!remembered).then(|| holder_gc.objects.lock(address));
                    let _remembered = remembered.then(|| holder_gc.remembered.lock(address));
                    entered.send(()).expect("membership held");
                    released
                        .recv_timeout(Duration::from_secs(10))
                        .expect("membership release");
                });
                observed
                    .recv_timeout(Duration::from_secs(10))
                    .expect("held membership");
                let worker_gc = gc.clone();
                let (finished, completed) = mpsc::channel();
                let worker = thread::spawn(move || {
                    for _ in 0..64 {
                        worker_gc.step();
                    }
                    finished.send(()).expect("steps complete");
                });
                let yielded = completed.recv_timeout(Duration::from_secs(1));
                let age = metadata.age.load(Ordering::Acquire);
                let coordinator_free = gc.coordinator.try_lock().is_ok();
                let publication_free = gc.publication.try_write().is_some();
                release.send(()).expect("release membership");
                holder.join().expect("membership holder");
                worker.join().expect("sweeper");
                yielded.expect("promotion blocked on membership");
                assert!(coordinator_free && publication_free);
                assert_eq!(age, usize::from(remembered));
                if !remembered {
                    assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Sweep);
                }
                assert_eq!(gc.objects.contains_young(address), !remembered);
                assert_eq!(gc.remembered.lock(address).contains(address), remembered);
                assert_eq!(gc.address_for_identity(root.identity()), Some(address));
                for _ in 0..10000 {
                    if gc.step() {
                        break;
                    }
                }
                assert!(!gc.pending());
                assert_eq!(metadata.age.load(Ordering::Acquire), 1);
                assert_eq!(gc.statistics()?.objects_promoted, 1);
                assert!(!gc.objects.contains_young(address));
                assert!(gc.remembered.lock(address).contains(address));
                assert_eq!(**root, 42);
                drop(root);
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn sweep_reclaims_other_shards_while_an_index_is_held() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for young in [false, true] {
                for held in [HeldIndex::Objects, HeldIndex::Remembered] {
                    held_sweep_index(concurrent, independent, young, held, false)?;
                    if concurrent {
                        held_sweep_index(concurrent, independent, young, held, true)?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::fn_params_excessive_bools,
    reason = "forced policy and admission axes"
)]
#[expect(
    clippy::too_many_lines,
    reason = "bounded schedule with cleanup before assertions"
)]
#[expect(clippy::expect_used, reason = "forced schedule test assertions")]
fn held_sweep_index(
    concurrent: bool,
    independent: bool,
    young: bool,
    held_index: HeldIndex,
    hold_admission: bool,
) -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(concurrent)
            .independent_phases(independent)
            .metadata_shards(16)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .promotion_age(100)
            .diagnostics(true)
            .build(),
    );
    assert_eq!(gc.registered_workers(), 0);
    let mut roots = Vec::new();
    let mut blocked = None;
    let mut available = None;
    for _ in 0..4096 {
        let drops = Arc::new(AtomicUsize::new(0));
        let root = Gc::try_new(&gc, Leaf(drops.clone()))?;
        let bucket = crate::index::address_bucket(root.as_ptr() as usize) & 15;
        if bucket == 0 && blocked.is_none() {
            blocked = Some((root.identity(), drops, roots.len()));
        } else if bucket != 0 && available.is_none() {
            available = Some((drops, roots.len()));
        }
        roots.push(Some(root));
        if blocked.is_some() && available.is_some() {
            break;
        }
    }
    let (id, blocked_drops, blocked_root) = blocked.expect("first object shard populated");
    let (available_drops, available_root) = available.expect("another object shard populated");
    let blocked_address = roots[blocked_root].as_ref().expect("blocked root").as_ptr() as usize;
    drop(roots[blocked_root].take());
    drop(roots[available_root].take());
    if young {
        gc.collect_young();
    } else {
        gc.collect();
    }
    let domain = usize::from(!young);
    for _ in 0..10000 {
        if lock(&gc.domains[domain].cycle).phase == Phase::Sweep {
            break;
        }
        gc.step();
    }
    assert_eq!(lock(&gc.domains[domain].cycle).phase, Phase::Sweep);
    assert_eq!(blocked_drops.load(Ordering::Acquire), 0);
    assert_eq!(available_drops.load(Ordering::Acquire), 0);
    let (entered, observed) = mpsc::channel();
    let (release, released) = mpsc::channel();
    let holder_gc = gc.clone();
    let holder = thread::spawn(move || {
        let _publication = hold_admission.then(|| holder_gc.allocation_guard());
        let pause = || {
            entered.send(()).expect("index locked");
            released
                .recv_timeout(Duration::from_secs(10))
                .expect("index release");
        };
        match held_index {
            HeldIndex::Objects => {
                let _guard = holder_gc.objects.lock_bucket(0);
                pause();
            }
            HeldIndex::Remembered => {
                let _guard = holder_gc.remembered.lock_bucket(0);
                pause();
            }
        }
    });
    observed
        .recv_timeout(Duration::from_secs(10))
        .expect("held index observed");
    let worker_gc = gc.clone();
    let freed = available_drops.clone();
    let other = blocked_drops.clone();
    let remembered = matches!(held_index, HeldIndex::Remembered);
    let (finished, completion) = mpsc::channel();
    let worker = thread::spawn(move || {
        for _ in 0..10000 {
            worker_gc.step();
            if freed.load(Ordering::Acquire) != 0
                && (!remembered || other.load(Ordering::Acquire) != 0)
            {
                break;
            }
        }
        finished
            .send(freed.load(Ordering::Acquire))
            .expect("sweep observer");
    });
    let progress = completion.recv_timeout(Duration::from_secs(1));
    let coordinator_free = gc.coordinator.try_lock().is_ok();
    let publication_free = gc.publication.try_write().is_some();
    let before_release = blocked_drops.load(Ordering::Acquire);
    let identity_preserved =
        gc.identities.get(id.to_i64().cast_unsigned()) == Some(blocked_address);
    let active = gc
        .publication_clock
        .as_ref()
        .map_or(0, |clock| clock.active());
    println!(
        "held sweep index: counted={concurrent} independent={independent} young={young} index={held_index:?} admitted={hold_admission}; progress={progress:?} coordinator_free={coordinator_free} publication_free={publication_free} active={active} blocked_drops={before_release} identity_preserved={identity_preserved}"
    );
    release.send(()).expect("release index holder");
    holder.join().expect("index holder");
    worker.join().expect("sweep worker");
    assert_eq!(progress, Ok(1), "unrelated sweep did not progress");
    assert!(coordinator_free && publication_free);
    assert_eq!(active, usize::from(hold_admission));
    assert_eq!(before_release, usize::from(remembered));
    assert_eq!(identity_preserved, !remembered);
    assert_eq!(
        gc.address_for_identity(id),
        (!remembered).then_some(blocked_address)
    );
    gc.collect_full()?;
    assert_eq!(blocked_drops.load(Ordering::Acquire), 1);
    assert_eq!(available_drops.load(Ordering::Acquire), 1);
    drop(roots);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
