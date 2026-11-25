//! Forced schedules for the experimental conservative old-only pass.
#![expect(clippy::panic_in_result_fn)]
#![cfg_attr(not(target_family = "wasm"), expect(clippy::unwrap_in_result))]
use super::*;

fn config(concurrent: bool) -> Configuration {
    Configuration::builder()
        .old_collection_interval(usize::MAX)
        .concurrent_publication(concurrent)
        .independent_marking(true)
        .work_stealing(true)
        .metadata_shards(16)
        .region_shards(16)
        .promotion_age(1)
        .incremental_step_size(1)
        .max_pause_time_us(0)
        .allocation_threshold(usize::MAX)
        .full_collection_interval(usize::MAX)
        .build()
}

#[test]
fn old_pass_reclaims_old_garbage_and_does_not_complete_full_tickets() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(config(concurrent));
        // The counted old path must not depend on the compatibility writer.
        let _legacy_writer = concurrent.then(|| gc.publication.write());
        let value = Gc::try_new(&gc, 42_u64)?;
        gc.collect_full()?;
        let old_ticket = gc.completed_full_ticket();
        drop(value);
        gc.collect_old();
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.old_collections, 1);
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.completed_full_ticket(), old_ticket);
        gc.stop()?;
    }
    Ok(())
}

#[test]
fn old_pass_retains_conditional_referents_until_precise_full_collection() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(config(concurrent));
        let value = Gc::try_new(&gc, 42_u64)?;
        let weak = crate::GcWeak::new(&value, crate::ReferenceKind::Weak, 0);
        gc.collect_full()?;
        drop(value);
        gc.collect_old();
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.old_collections, 1);
        assert!(weak.upgrade().is_some());
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        gc.stop()?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn young_cycles_finish_while_an_unrelated_old_trace_is_held() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    struct Held {
        entered: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
        enabled: AtomicBool,
    }
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Held {
        fn trace(&self, gc: &GarbageCollector) {
            if gc.old_collection_active()
                && gc.domain_index() == 1
                && self.enabled.swap(false, Ordering::AcqRel)
            {
                self.entered.send(()).expect("entered");
                lock(&self.release)
                    .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 10 }))
                    .expect("release");
            }
        }
    }
    for concurrent in [false, true] {
        for independent in [false, true] {
            let mut configuration = config(concurrent);
            configuration.independent_marking = independent;
            let gc = GarbageCollector::with_config(configuration);
            let (entered_tx, entered_rx) = mpsc::channel();
            let (release_tx, release_rx) = mpsc::channel();
            let root = Gc::try_new(
                &gc,
                Held {
                    entered: entered_tx,
                    release: Mutex::new(release_rx),
                    enabled: AtomicBool::new(true),
                },
            )?;
            gc.collect_full()?;
            generation_tests::finish_young(&gc)?; // Prune the clean old owner.
            assert!(gc.remembered.lock(root.as_ptr() as usize).is_empty());
            gc.collect_old();
            for _ in 0..1000 {
                if lock(&gc.domains[1].cycle).phase == Phase::Mark {
                    break;
                }
                gc.step();
            }
            assert_eq!(lock(&gc.domains[1].cycle).kind, CollectionKind::Old);
            let worker_gc = gc.clone();
            let worker = thread::spawn(move || worker_gc.step());
            entered_rx
                .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 10 }))
                .expect("held old trace");
            let prior_full = gc.completed_full_ticket();
            let requested_full = gc.request_full_cycle();
            for _ in 0..3 {
                let garbage = Gc::try_new(&gc, 17_u64)?;
                let weak = crate::GcWeak::new(&garbage, crate::ReferenceKind::Weak, 0);
                drop(garbage);
                generation_tests::finish_young(&gc)?;
                assert!(
                    weak.upgrade().is_none(),
                    "old tracing retained a post-snapshot birth"
                );
                assert!(gc.old_collection_active());
                assert_eq!(gc.completed_full_ticket(), prior_full);
                assert!(gc.coordinator.try_lock().is_ok());
            }
            release_tx.send(()).expect("release old trace");
            worker.join().expect("worker");
            for _ in 0..10000 {
                if gc.step() {
                    break;
                }
            }
            assert_eq!(gc.statistics()?.old_collections, 1);
            assert!(gc.completed_full_ticket() >= requested_full);
            drop(root);
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn young_handoff_and_promotion_publish_old_edges() -> Result<()> {
    struct Edge(Gc<u64>);
    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    unsafe impl Trace for Edge {
        fn root(&mut self) {
            self.0.root();
        }
        fn trace(&self, gc: &GarbageCollector) {
            self.0.trace(gc);
        }
        unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
            unsafe { Trace::unroot(&mut self.0, owner) };
        }
    }
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(config(concurrent));
        let old = Gc::try_new(&gc, 42_u64)?;
        gc.collect_full()?;
        let old_address = old.as_ptr() as usize;
        let young = Gc::try_new(&gc, Edge(old.into_rooted()))?;
        gc.collect_old();
        // Start old handoff, then promote the young owner before its cursor passes.
        gc.step();
        generation_tests::finish_young(&gc)?;
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(gc.statistics()?.old_collections, 1);
        assert!(gc.objects.lock(old_address).contains_key(&old_address));
        assert_eq!(*young.0, 42);
        drop(young);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn old_sweep_ignores_young_and_blackens_late_promotions() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(config(concurrent));
        gc.collect_old();
        for _ in 0..1000 {
            gc.step();
            if gc.domains[1].weak_phase() == 5 {
                break;
            }
        }
        assert!(gc.old_collection_active());
        let young = Gc::try_new(&gc, 42_u64)?;
        let weak = crate::GcWeak::new(&young, crate::ReferenceKind::Weak, 0);
        assert!(weak.upgrade().is_some());
        // Stop old sweep at its own cycle lock while young promotes this birth.
        let old_cycle = lock(&gc.domains[1].cycle);
        generation_tests::finish_young(&gc)?;
        assert!(!gc.is_young_allocation(young.as_ptr() as usize));
        drop(old_cycle);
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert_eq!(**young, 42);
        assert!(weak.upgrade().is_some());
        drop(young);
        gc.stop()?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one forced graph borrow/mutation schedule"
)]
fn old_callbacks_interlock_with_their_own_remembered_graph() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    struct Child(Arc<AtomicUsize>);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Child {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Child {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Release);
        }
    }
    struct Parent {
        child: Mutex<Option<Gc<Child>>>,
        copying: bool,
        enabled: AtomicBool,
        entered: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    impl Parent {
        fn hold(&self) {
            if self.enabled.swap(false, Ordering::AcqRel) {
                let child = lock(&self.child).clone().expect("child installed");
                self.entered.send(()).expect("entered");
                lock(&self.release)
                    .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 10 }))
                    .expect("release");
                assert_eq!(child.0.load(Ordering::Acquire), 0);
            }
        }
    }
    // SAFETY: outgoing fields remain independently rooted and are included in tracing.
    unsafe impl Trace for Parent {
        fn trace(&self, gc: &GarbageCollector) {
            if !self.copying && gc.old_collection_active() && gc.domain_index() == 1 {
                self.hold();
            }
            lock(&self.child).trace(gc);
        }
        fn compact_guarded(&self, _: &crate::region::RegionHeap, _: usize) -> TraceProgress {
            if self.copying {
                self.hold();
            }
            TraceProgress::Complete
        }
    }
    for concurrent in [false, true] {
        for copying in [false, true] {
            let mut configuration = config(concurrent);
            configuration.compact_heap = true;
            configuration.independent_relocation = true;
            let gc = GarbageCollector::with_config(configuration);
            let (entered_tx, entered_rx) = mpsc::channel();
            let (release_tx, release_rx) = mpsc::channel();
            let parent = Gc::try_new(
                &gc,
                Parent {
                    child: Mutex::new(None),
                    copying,
                    enabled: AtomicBool::new(false),
                    entered: entered_tx,
                    release: Mutex::new(release_rx),
                },
            )?;
            gc.collect_full()?;
            generation_tests::finish_young(&gc)?;
            gc.collect_old();
            let phase = if copying { Phase::Compact } else { Phase::Mark };
            for _ in 0..1000 {
                if lock(&gc.domains[1].cycle).phase == phase {
                    break;
                }
                // Do not execute an independent copy claim before installing the edge.
                gc.coordinated_step(false);
            }
            assert_eq!(lock(&gc.domains[1].cycle).phase, phase);
            let drops = Arc::new(AtomicUsize::new(0));
            let mut child = Gc::try_new(&gc, Child(drops.clone()))?.into_rooted();
            // SAFETY: This serialized fixture installs and remembers the edge before starting collector work.
            unsafe { child.unroot() };
            *lock(&parent.child) = Some(child);
            gc.remember(&parent);
            parent.enabled.store(true, Ordering::Release);
            let worker_gc = gc.clone();
            let worker = thread::spawn(move || {
                for _ in 0..1000 {
                    worker_gc.step();
                }
            });
            entered_rx
                .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 10 }))
                .expect("held callback");
            // Remove the edge after the callback copied it into its local borrow.
            lock(&parent.child).take();
            gc.remember(&parent);
            let completed = gc.statistics()?.young_collections;
            gc.collect_young();
            for _ in 0..100 {
                gc.step();
            }
            assert_eq!(gc.statistics()?.young_collections, completed);
            assert_eq!(drops.load(Ordering::Acquire), 0);
            release_tx.send(()).expect("release callback");
            worker.join().expect("worker");
            for _ in 0..10000 {
                if gc.step() {
                    break;
                }
            }
            assert!(gc.statistics()?.young_collections > completed);
            drop(parent);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Acquire), 1);
        }
    }
    Ok(())
}

#[test]
fn young_remembered_scans_do_not_borrow_logically_dead_old_owners() -> Result<()> {
    struct Owner {
        child: Mutex<Option<Gc<u64>>>,
        scans: Arc<AtomicUsize>,
    }
    // SAFETY: outgoing fields remain independently rooted and are included in tracing.
    unsafe impl Trace for Owner {
        fn trace(&self, gc: &GarbageCollector) {
            self.scans.fetch_add(1, Ordering::Relaxed);
            lock(&self.child).trace(gc);
        }
    }
    for concurrent in [false, true] {
        for old_only in [false, true] {
            let gc = GarbageCollector::with_config(config(concurrent));
            let scans = Arc::new(AtomicUsize::new(0));
            let owner = Gc::try_new(
                &gc,
                Owner {
                    child: Mutex::new(None),
                    scans: scans.clone(),
                },
            )?;
            gc.collect_full()?;
            let mut child = Gc::try_new(&gc, 42_u64)?.into_rooted();
            // SAFETY: This serialized fixture installs and remembers the edge before starting collector work.
            unsafe { child.unroot() };
            *lock(&owner.child) = Some(child);
            gc.remember(&owner);
            drop(owner);
            if old_only {
                gc.collect_old();
            } else {
                gc.collect();
            }
            for _ in 0..1000 {
                gc.step();
                if gc.domains[1].weak_phase() == 5 {
                    break;
                }
            }
            assert_eq!(gc.domains[1].weak_phase(), 5);
            let before = scans.load(Ordering::Relaxed);
            let held = lock(&gc.domains[1].cycle);
            generation_tests::finish_young(&gc)?;
            assert_eq!(scans.load(Ordering::Relaxed), before);
            drop(held);
            gc.stop()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn precise_full_handoff_does_not_inherit_the_conservative_young_snapshot() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(config(concurrent));
        drop(Gc::try_new(&gc, vec![7_u8; 1024])?);
        gc.collect_old();
        for _ in 0..10000 {
            gc.step();
            if gc.domains[1].weak_phase() == 5 {
                break;
            }
        }
        assert_eq!(gc.domains[1].weak_phase(), 5);
        // Hold old retirement while young takes a snapshot of its retained cohort.
        let old = lock(&gc.domains[1].cycle);
        gc.collect_young();
        for _ in 0..10000 {
            gc.step();
            if lock(&gc.domains[0].cycle).phase == Phase::Mark {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[0].cycle).phase, Phase::Mark);
        drop(old);
        let young = lock(&gc.domains[0].cycle);
        for _ in 0..10000 {
            gc.next_domain.store(1, Ordering::Relaxed);
            gc.coordinated_step(false);
            if !gc.domains[1].active() {
                break;
            }
        }
        assert!(!gc.domains[1].active());
        let full = gc.request_full_cycle();
        for _ in 0..10 {
            gc.step();
        }
        assert!(
            !gc.domains[1].active(),
            "precise full started with conservative young roots"
        );
        drop(young);
        for _ in 0..10000 {
            gc.step();
            if gc.completed_full_ticket() >= full {
                break;
            }
        }
        assert!(gc.completed_full_ticket() >= full);
        assert_eq!(gc.managed_bytes(), 0);
        gc.stop()?;
    }
    Ok(())
}
