//! Forced overlap of young cycles with full-cycle reclamation.
#![expect(clippy::panic_in_result_fn)]
use super::*;

pub(super) fn finish_young(gc: &GarbageCollector) -> Result<()> {
    let completed = gc.statistics()?.young_collections;
    gc.collect_young();
    for _ in 0..10000 {
        gc.step();
        if gc.statistics()?.young_collections > completed {
            return Ok(());
        }
    }
    panic!("young cycle blocked by full reclamation");
}

#[test]
fn a_requested_full_start_drains_the_previous_young_snapshot() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let root = Gc::try_new(&gc, 42_u64)?;
        gc.collect_young();
        for _ in 0..100 {
            gc.step();
            if lock(&gc.domains[0].cycle).phase == Phase::Mark {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Mark);
        // The young snapshot still retains this now-unreachable object.
        drop(root);
        gc.collect();
        gc.next_domain.store(1, Ordering::Release);
        gc.coordinated_step(false);
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Idle);
        for _ in 0..1000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.full_collections, 1);
        assert_eq!(gc.statistics()?.live_objects, 0);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn full_sweeping_does_not_promote_allocations_born_after_tracing() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .overlapping_generations(true)
                .concurrent_publication(concurrent)
                .promotion_age(1)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .build(),
        );
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if gc.domains[1].weak_phase() == 5 {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Sweep);
        let value = Gc::try_new(&gc, 42_u64)?;
        let weak = crate::GcWeak::new(&value, crate::ReferenceKind::Weak, 0);
        while !gc.step() {}
        assert!(gc.is_young_allocation(value.as_ptr() as usize));
        drop(value);
        finish_young(&gc)?;
        assert!(weak.upgrade().is_none());
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn independent_copy_work_gives_young_cycles_coordinator_turns() -> Result<()> {
    struct Copying(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Copying {
        fn trace(&self, _: &GarbageCollector) {}
        fn external_size(&self) -> usize {
            1024 * 1024
        }
        fn compact_guarded(&self, _: &crate::region::RegionHeap, bytes: usize) -> TraceProgress {
            self.0.fetch_add(1, Ordering::Relaxed);
            if bytes == 0 {
                TraceProgress::Complete
            } else {
                TraceProgress::Continue(0)
            }
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .overlapping_generations(true)
            .concurrent_publication(true)
            .independent_marking(true)
            .independent_relocation(true)
            .work_stealing(true)
            .threads(4)
            .compact_heap(true)
            .promotion_age(1)
            .allocation_threshold(usize::MAX)
            .full_collection_interval(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let copies = Arc::new(AtomicUsize::new(0));
    let mut roots = Vec::new();
    for _ in 0..64 {
        roots.push(Gc::try_new(&gc, Copying(copies.clone()))?);
    }
    gc.collect();
    for _ in 0..10000 {
        gc.coordinated_step(false);
        if gc.domains[1].relocations.as_ref().expect("queues").len() == 64 {
            break;
        }
    }
    assert_eq!(
        gc.domains[1].relocations.as_ref().expect("queues").len(),
        64
    );
    let young = Gc::try_new(&gc, 42_u64)?;
    let weak = crate::GcWeak::new(&young, crate::ReferenceKind::Weak, 0);
    drop(young);
    finish_young(&gc)?;
    assert!(weak.upgrade().is_none());
    assert!(gc.domains[1].active());
    assert!(
        copies.load(Ordering::Relaxed) < 1000,
        "young progress waited for the full copy queue"
    );
    drop(roots);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn young_reference_processing_rejects_a_logically_retired_old_owner() -> Result<()> {
    struct Owner(Arc<AtomicBool>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Owner {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            if self.0.load(Ordering::Acquire) {
                TraceProgress::Busy
            } else {
                TraceProgress::Complete
            }
        }
    }
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .overlapping_generations(true)
                .concurrent_publication(concurrent)
                .independent_marking(true)
                .promotion_age(1)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let first = Gc::try_new(&gc, Owner(Arc::new(AtomicBool::new(false))))?;
        let second = Gc::try_new(&gc, Owner(Arc::new(AtomicBool::new(false))))?;
        gc.collect_full()?;
        // The single-shard sweep visits lower addresses first. Hold clearing of
        // that allocation, leaving the other old owner white but still registered.
        let (hold, owner) = if first.as_ptr() < second.as_ptr() {
            (first, second)
        } else {
            (second, first)
        };
        let release = hold.0.clone();
        release.store(true, Ordering::Release);
        let address = owner.as_ptr() as usize;
        drop((hold, owner));
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if !lock(&gc.domains[1].cycle).reclaim.is_empty() {
                break;
            }
        }
        assert!(!lock(&gc.domains[1].cycle).reclaim.is_empty());
        let metadata = gc
            .objects
            .lock(address)
            .get(&address)
            .cloned()
            .expect("unswept owner");
        assert!(!metadata.marked(1, gc.domains[1].epoch.load(Ordering::Acquire)));
        // Hold only this domain's sweep cursor. Busy clearing now rotates into
        // retirement and would otherwise let the old owner disappear entirely.
        let full_cycle = lock(&gc.domains[1].cycle);
        for kind in [
            crate::ReferenceKind::Weak,
            crate::ReferenceKind::Final,
            crate::ReferenceKind::PhantomRetained,
        ] {
            let target = Gc::try_new(&gc, 42_u64)?;
            let link = crate::GcWeak::new(&target, kind, address);
            drop(target);
            finish_young(&gc)?;
            assert!(
                !gc.has_pending_references(),
                "young processing resurrected a retired old owner"
            );
            assert!(
                link.upgrade().is_none(),
                "retired owner retained a new referent"
            );
            assert!(gc.objects.lock(address).occupied());
        }
        drop(full_cycle);
        release.store(false, Ordering::Release);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn young_garbage_is_reclaimed_while_an_old_clear_remains_busy() -> Result<()> {
    struct Held {
        hold: Arc<AtomicBool>,
        drops: Arc<AtomicUsize>,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Held {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            if self.hold.load(Ordering::Acquire) {
                TraceProgress::Busy
            } else {
                TraceProgress::Complete
            }
        }
    }
    impl Drop for Held {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Release);
        }
    }
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .overlapping_generations(true)
                    .concurrent_publication(concurrent)
                    .independent_marking(independent)
                    .metadata_shards(16)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let hold = Arc::new(AtomicBool::new(true));
            let drops = Arc::new(AtomicUsize::new(0));
            drop(Gc::try_new(
                &gc,
                Held {
                    hold: hold.clone(),
                    drops: drops.clone(),
                },
            )?);
            gc.collect();
            for _ in 0..1000 {
                gc.step();
                if !lock(&gc.domains[1].cycle).reclaim.is_empty() {
                    break;
                }
            }
            assert!(!lock(&gc.domains[1].cycle).reclaim.is_empty());
            assert_eq!(gc.domains[1].weak_phase(), 5);
            for round in 0..3 {
                let value = Gc::try_new(&gc, round)?;
                let weak = crate::GcWeak::new(&value, crate::ReferenceKind::Weak, 0);
                drop(value);
                finish_young(&gc)?;
                assert!(
                    weak.upgrade().is_none(),
                    "new young garbage retained by full snapshot"
                );
                // The busy allocation remains charged even after full sweep ends.
                assert!(gc.managed_bytes() > 0);
                assert_eq!(drops.load(Ordering::Acquire), 0);
            }
            hold.store(false, Ordering::Release);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Acquire), 1);
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "forced callback/worker assertions")]
#[expect(
    clippy::too_many_lines,
    reason = "one forced leased-copy/young-retirement schedule"
)]
fn young_cycles_complete_while_a_full_relocation_callback_is_in_flight() -> Result<()> {
    struct Held {
        entered: std::sync::mpsc::Sender<()>,
        resume: Mutex<std::sync::mpsc::Receiver<()>>,
        seen: Arc<AtomicBool>,
        drops: Arc<AtomicUsize>,
        child: Gc<u64>,
    }
    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    unsafe impl Trace for Held {
        fn root(&mut self) {
            self.child.root();
        }
        unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.child, owner) };
        }
        fn trace(&self, gc: &GarbageCollector) {
            self.child.trace(gc);
        }
        fn compact_guarded(&self, _: &crate::region::RegionHeap, _: usize) -> TraceProgress {
            if !self.seen.swap(true, Ordering::AcqRel) {
                self.entered.send(()).expect("entered relocation");
                lock(&self.resume)
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume relocation");
                // The callback may inspect outgoing objects. Its entire traced
                // snapshot graph must survive, not just this descriptor's lease.
                assert_eq!(*self.child, 42);
            }
            TraceProgress::Complete
        }
    }
    impl Drop for Held {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Release);
        }
    }
    struct Born(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Born {
        fn trace(&self, _: &GarbageCollector) {}
        fn compact_guarded(&self, _: &crate::region::RegionHeap, _: usize) -> TraceProgress {
            self.0.fetch_add(1, Ordering::Relaxed);
            TraceProgress::Complete
        }
    }
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .overlapping_generations(true)
                    .concurrent_publication(concurrent)
                    .independent_relocation(independent)
                    .independent_marking(true)
                    .work_stealing(true)
                    .threads(4)
                    .compact_heap(true)
                    .diagnostics(true)
                    .cost_aware_relocation(true)
                    .promotion_age(100)
                    .allocation_threshold(usize::MAX)
                    .full_collection_interval(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let (entered, observed) = std::sync::mpsc::channel();
            let (release, resume) = std::sync::mpsc::channel();
            let seen = Arc::new(AtomicBool::new(false));
            let drops = Arc::new(AtomicUsize::new(0));
            let child = Gc::try_new(&gc, 42_u64)?;
            let root = Gc::try_new(
                &gc,
                Held {
                    entered,
                    resume: Mutex::new(resume),
                    seen: seen.clone(),
                    drops: drops.clone(),
                    child: child.clone_gc(),
                },
            )?;
            drop(child);
            let weak_root = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
            gc.collect();
            for _ in 0..10000 {
                gc.coordinated_step(false);
                if lock(&gc.domains[1].cycle).phase == Phase::Compact {
                    break;
                }
            }
            assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::Compact);
            let born_copies = Arc::new(AtomicUsize::new(0));
            let born = Gc::try_new(&gc, Born(born_copies.clone()))?;
            let weak_born = crate::GcWeak::new(&born, crate::ReferenceKind::Weak, 0);
            drop(born);
            let worker = {
                let gc = gc.clone();
                thread::spawn(move || {
                    while !seen.load(Ordering::Acquire) {
                        gc.step();
                    }
                })
            };
            observed
                .recv_timeout(std::time::Duration::from_secs(10))
                .expect("callback entered");
            assert_eq!(gc.domains[1].in_flight.load(Ordering::Acquire), 1);
            assert!(gc.region_heap().expect("heap").evacuation_active());
            drop(root);
            for round in 0..3 {
                let value = Gc::try_new(&gc, round)?;
                let weak = crate::GcWeak::new(&value, crate::ReferenceKind::Weak, 0);
                drop(value);
                finish_young(&gc)?;
                assert!(weak.upgrade().is_none());
                assert!(weak_born.upgrade().is_none());
                assert!(weak_root.upgrade().is_some());
                assert_eq!(gc.domains[1].in_flight.load(Ordering::Acquire), 1);
                assert_eq!(drops.load(Ordering::Acquire), 0);
                assert_eq!(born_copies.load(Ordering::Relaxed), 0);
                assert!(
                    gc.region_heap().expect("heap").evacuation_active(),
                    "young completion ended the full evacuation selection"
                );
            }
            release.send(()).expect("release callback");
            worker.join().expect("worker");
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Acquire), 1);
            assert!(!gc.region_heap().expect("heap").evacuation_active());
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn retired_bytes_cover_every_unregistered_allocation_until_storage_is_released() -> Result<()> {
    struct Chunk([u8; 16]);
    // SAFETY: the fixture has no managed edges or external storage.
    unsafe impl Trace for Chunk {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, offset: usize, _: usize) -> TraceProgress {
            if offset < self.0.len() {
                self.0[offset] = 0;
                TraceProgress::Continue(offset + 1)
            } else {
                TraceProgress::Complete
            }
        }
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .diagnostics(true)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let bytes = Gc::allocation_size(&Chunk([0; 16]))?;
    let mut roots = Vec::new();
    for value in 0..16 {
        roots.push(Gc::try_new(&gc, Chunk([value; 16]))?);
    }
    let addresses: Vec<_> = roots.iter().map(|root| root.as_ptr().addr()).collect();
    drop(roots);
    let ticket = gc.request_full_cycle();
    let mut observed_local_debt = false;
    for _ in 0..8192 {
        gc.step();
        let registered = addresses
            .iter()
            .filter(|a| gc.objects.lookup(**a).is_some())
            .count();
        let unregistered_bytes = gc.managed_bytes() - registered * bytes;
        assert_eq!(gc.diagnostics().retired_bytes, unregistered_bytes);
        observed_local_debt |= !lock(&gc.domains[1].cycle).reclaim.is_empty();
        if gc.completed_full_ticket() >= ticket && gc.managed_bytes() == 0 {
            break;
        }
    }
    assert!(observed_local_debt);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.diagnostics().retired_bytes, 0);
    Ok(())
}
