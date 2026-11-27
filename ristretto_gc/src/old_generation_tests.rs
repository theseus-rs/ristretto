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
fn late_weak_registration_marks_a_white_mature_target_in_the_old_pass() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(config(counted));
        let original = Gc::try_new(&gc, 47_u64)?;
        gc.collect_full()?;
        let mut edge = original.clone_gc();
        // SAFETY: the owner protects demotion. Only the inert identity is used
        // below, and the collector is driven by this test alone.
        unsafe { edge.unroot() };
        let metadata = gc.objects.lookup(original.as_ptr().addr()).unwrap();
        drop(original);
        gc.collect_old();
        assert!((0..10_000).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::Mark
        }));
        let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
        assert!(!metadata.marked(1, epoch));
        let weak = crate::GcWeak::new(&edge, crate::ReferenceKind::Weak, 0);
        assert!(metadata.marked(1, epoch));
        assert!((0..10_000).any(|_| gc.step()));
        assert_eq!(*weak.upgrade().unwrap(), 47);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[test]
fn numeric_deletions_publish_initial_young_retention_before_the_old_handoff_scan() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .old_collection_interval(4)
                .overlapping_generations(true)
                .concurrent_publication(counted)
                .barrier_buffer_size(4)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let mut children = Vec::new();
        for value in 0..8u64 {
            children.push(Gc::try_new(&gc, value)?.into_rooted());
        }
        let owner = Gc::try_new(&gc, children)?;
        gc.collect_old();
        assert!((0..2048).any(|_| {
            gc.coordinated_step(false);
            lock(&gc.domains[1].cycle).phase == Phase::OldHandoff
        }));
        let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
        // Activation can visit its first handoff entry in the same step.
        let (edge, initial) = owner
            .iter()
            .find_map(|edge| {
                let target = gc
                    .objects
                    .lookup_identity(edge.as_ptr().addr(), edge.identity())?;
                (!target.marked(1, epoch)).then_some((edge, target))
            })
            .expect("young target ahead of the old cursor");
        let late = Gc::try_new(&gc, 23u64)?;
        let after = gc
            .objects
            .lookup_identity(late.as_ptr().addr(), late.identity())
            .expect("post-snapshot birth");
        assert_eq!(after.old_birth.load(Ordering::Acquire), epoch);
        assert!(after.marked(1, epoch));
        assert_eq!(after.retention(1, epoch), None);
        // SAFETY: late keeps this exact post-snapshot allocation live.
        let raw = unsafe { Gc::from_raw(late.as_ptr())? };
        let context = gc.register_context();
        context.with(|| {
            gc.write_barrier(edge);
            raw.write_barrier(&gc);
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
            assert_eq!(initial.retention(1, epoch), Some(Retention::Strong));
            assert_eq!(after.retention(1, epoch), None);
        });
        drop((raw, late, owner));
        let overlapping_owner = Gc::try_new(&gc, vec![Gc::try_new(&gc, 31u64)?.into_rooted()])?;
        let overlapping = overlapping_owner.first().expect("overlapping child");
        let both = gc
            .objects
            .lookup_identity(overlapping.as_ptr().addr(), overlapping.identity())
            .expect("overlapping birth");
        // SAFETY: overlapping_owner protects this child until the barrier returns.
        let raw_both = unsafe { Gc::from_raw(overlapping.as_ptr())? };
        // A later young snapshot can read the initial child's old retention
        // while the old cursor has yet to reach this child.
        let old = lock(&gc.domains[1].cycle);
        let young_before = gc.statistics()?.young_collections;
        gc.collect_young();
        assert!((0..2048).any(|_| {
            gc.coordinated_step(false);
            gc.domains[0].active()
        }));
        let young_epoch = gc.domains[0].epoch.load(Ordering::Acquire);
        assert!(!both.marked(0, young_epoch));
        context.with(|| raw_both.write_barrier(&gc));
        assert_eq!(both.retention(0, young_epoch), Some(Retention::Strong));
        assert_eq!(both.retention(1, epoch), None);
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        drop((raw_both, overlapping_owner));
        assert!((0..10000).any(|_| {
            gc.step();
            gc.statistics()
                .is_ok_and(|stats| stats.young_collections > young_before)
        }));
        assert!(both.birth.is_live());
        assert!(initial.birth.is_live());
        assert!(!after.birth.is_live());
        drop(old);
        gc.collect_full()?;
        assert!(!initial.birth.is_live());
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
        assert!(gc.objects.lock(old_address).occupied());
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

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn old_reference_shade_completes_with_target_shard_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(config(counted));
        let target = Gc::try_new(&gc, 47_u64)?;
        let metadata = gc.objects.lookup(target.as_ptr().addr()).unwrap();
        let weak = crate::GcWeak::new(&target, crate::ReferenceKind::Weak, 0);
        gc.collect_full()?;
        drop(target);
        gc.collect_old();
        assert!((0..4096).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::OldReferences
        }));
        let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
        assert!(!metadata.marked(1, epoch));
        let held = gc.objects.lock(metadata.ptr().0);
        let (done, finished) = mpsc::channel();
        let progress = thread::scope(|scope| {
            let heap = &gc;
            scope.spawn(move || {
                heap.step();
                done.send(()).unwrap();
            });
            let result = finished.recv_timeout(Duration::from_secs(2));
            drop(held);
            result
        });
        assert!(progress.is_ok(), "old-only shade waited on target shard");
        assert!(metadata.marked(1, epoch));
        assert!((0..4096).any(|_| gc.step()));
        assert_eq!(*weak.upgrade().unwrap(), 47);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one forced trace, handoff and promotion schedule"
)]
fn old_callback_retains_a_young_edge_while_its_object_shard_is_parked() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    struct Child(Arc<AtomicUsize>);
    // SAFETY: this fixture has no outgoing managed references.
    unsafe impl Trace for Child {
        fn trace(&self, gc: &GarbageCollector) {
            if gc.domain_index() == 1 && gc.old_collection_active() {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    struct Parent {
        edge: Mutex<Option<Gc<Child>>>,
        ready: mpsc::Sender<()>,
        resume: Mutex<Option<mpsc::Receiver<()>>>,
        traced: mpsc::Sender<()>,
        edge_done: Arc<AtomicBool>,
    }

    // SAFETY: the only outgoing edge is traced under its mutation lock.
    unsafe impl Trace for Parent {
        fn trace(&self, gc: &GarbageCollector) {
            if gc.domain_index() == 1
                && gc.old_collection_active()
                && let Some(resume) = lock(&self.resume).take()
            {
                self.ready.send(()).expect("old callback");
                resume.recv().expect("allow edge publication");
                lock(&self.edge).trace(gc);
                self.edge_done.store(true, Ordering::Release);
                self.traced.send(()).expect("edge published");
            } else {
                lock(&self.edge).trace(gc);
            }
        }
    }

    for counted in [false, true] {
        let gc = GarbageCollector::with_config(config(counted));
        let (ready, started) = mpsc::channel();
        let (go, resume) = mpsc::channel();
        let (traced, published) = mpsc::channel();
        let edge_done = Arc::new(AtomicBool::new(false));
        let parent = Gc::try_new(
            &gc,
            Parent {
                edge: Mutex::new(None),
                ready,
                resume: Mutex::new(Some(resume)),
                traced,
                edge_done: edge_done.clone(),
            },
        )?;
        gc.collect_full()?;
        generation_tests::finish_young(&gc)?;
        gc.collect_old();
        assert!((0..4096).any(|_| {
            gc.coordinated_step(false);
            lock(&gc.domains[1].cycle).phase == Phase::Mark
        }));
        let scans = Arc::new(AtomicUsize::new(0));
        let mut child = Gc::try_new(&gc, Child(scans.clone()))?.into_rooted();
        let address = child.as_ptr() as usize;
        let metadata = gc.objects.lookup(address).expect("post-snapshot child");
        // SAFETY: the fixture holds the old cycle at Mark while installing and
        // remembering its edge. The owning parent remains live throughout.
        unsafe { child.unroot() };
        *lock(&parent.edge) = Some(child);
        gc.remember(&parent);
        let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
        assert_eq!(metadata.old_birth.load(Ordering::Acquire), epoch);
        assert_eq!(metadata.retention(1, epoch), None);

        {
            let _context = Context::enter(&gc, 1);
            gc.mark_object(SafePtr(address), parent.identity());
            assert_eq!(
                metadata.retention(1, epoch),
                None,
                "mismatched identity shaded a birth"
            );
        }
        let stop = edge_done;
        let heap = gc.clone();
        let worker = thread::spawn(move || {
            for _ in 0..4096 {
                heap.step();
                if stop.load(Ordering::Acquire) {
                    break;
                }
            }
        });
        started
            .recv_timeout(Duration::from_secs(5))
            .expect("old trace entered");
        let shard = gc.objects.lock(address);
        go.send(()).expect("resume trace");
        let progressed = published.recv_timeout(Duration::from_secs(3)).is_ok();
        let strength = metadata.retention(1, epoch);
        drop(shard);
        worker.join().expect("join old marker");
        assert!(progressed, "old trace waited for its target's object shard");
        assert_eq!(strength, Some(Retention::Strong));

        assert_eq!(scans.load(Ordering::Relaxed), 0);
        let old_cycle = lock(&gc.domains[1].cycle);
        generation_tests::finish_young(&gc)?;
        assert!(!gc.is_young_allocation(address));
        drop(old_cycle);
        assert!((0..4096).any(|_| gc.step()));
        assert!(
            scans.load(Ordering::Relaxed) > 0,
            "promotion failed to publish old traversal"
        );
        drop(parent);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
