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

#[cfg(not(miri))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "forced index waits across publication/root policies"
)]
fn allocation_index_waits_release_admission_and_leave_registration_uncommitted() -> Result<()> {
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
                let worker_gc = gc.clone();
                let worker_context = context.clone();
                let constructor = thread::spawn(move || {
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
                    .expect("index wait");
                let bucket = crate::index::address_bucket(address) & 15;
                let held = all
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, guard)| (i == bucket).then_some(guard))
                    .collect::<Vec<_>>();
                let partial_object = gc
                    .objects
                    .try_lock(address)
                    .map(|objects| objects.contains_key(&address));
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
fn slot_and_context_root_reads_retry_held_metadata_without_retaining_admission() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
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
                let phase = if storage == crate::RootStorage::Contexts {
                    Phase::ContextRoots
                } else {
                    Phase::RootSlots
                };
                let objects = gc.objects.lock(root.as_ptr() as usize);
                let worker_gc = gc.clone();
                let (finished, result) = mpsc::channel();
                let worker = thread::spawn(move || {
                    for _ in 0..64 {
                        worker_gc.step();
                    }
                    finished.send(()).expect("root steps complete");
                });
                let yielded = result.recv_timeout(Duration::from_secs(1));
                let coordinator_free = gc.coordinator.try_lock().is_ok();
                let admission_free = gc.publication_clock.as_ref().map_or_else(
                    || gc.publication.try_write().is_some(),
                    |clock| clock.active() == 0,
                );
                println!(
                    "held root metadata: counted={concurrent} independent={independent} storage={storage:?}; yielded={yielded:?} coordinator_free={coordinator_free} admission_free={admission_free}"
                );
                drop(objects);
                worker.join().expect("root worker");
                assert_eq!(yielded, Ok(()));
                assert!(coordinator_free && admission_free);
                assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
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
        let freed = lock(&gc.statistics).bytes_freed;
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
fn promotion_retries_before_changing_age_or_generation_membership() -> Result<()> {
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
                assert_eq!(age, 0, "partial promotion escaped a failed lock attempt");
                assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Sweep);
                assert!(gc.objects.contains_young(address));
                assert!(!gc.remembered.lock(address).contains(&address));
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
                assert!(gc.remembered.lock(address).contains(&address));
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
    let (finished, completion) = mpsc::channel();
    let worker = thread::spawn(move || {
        for _ in 0..10000 {
            worker_gc.step();
            if freed.load(Ordering::Acquire) != 0 {
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
    assert_eq!(before_release, 0, "held retirement was skipped");
    assert!(
        identity_preserved,
        "deferred retirement removed its identity"
    );
    assert_eq!(gc.address_for_identity(id), Some(blocked_address));
    gc.collect_full()?;
    assert_eq!(blocked_drops.load(Ordering::Acquire), 1);
    assert_eq!(available_drops.load(Ordering::Acquire), 1);
    drop(roots);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
