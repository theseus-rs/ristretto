//! Forced schedules for counted admission in the complete collector.
#![expect(clippy::panic_in_result_fn)]
use super::*;

#[test]
fn registration_rejects_replay_before_any_effect_and_preserves_the_graph() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let parent = Gc::try_new(&gc, vec![child])?;
            let metadata = gc
                .metadata_for_address(parent.as_ptr() as usize)
                .expect("parent");
            let before = (
                metadata.retention(0, gc.domain_at(0).epoch.load(Ordering::Acquire)),
                metadata.retention(1, gc.domain_at(1).epoch.load(Ordering::Acquire)),
                metadata.old_birth.load(Ordering::Acquire),
                gc.statistics()?,
                gc.managed_bytes(),
            );
            let mut roots = crate::construction::Roots::new(&gc);
            let construction = roots.enter().detach();
            let rejected = gc.register_prepared_checked(&metadata, &construction);
            assert!(rejected.is_err(), "duplicate registration was replayed");
            assert_eq!(
                metadata.retention(0, gc.domain_at(0).epoch.load(Ordering::Acquire)),
                before.0
            );
            assert_eq!(
                metadata.retention(1, gc.domain_at(1).epoch.load(Ordering::Acquire)),
                before.1
            );
            assert_eq!(metadata.old_birth.load(Ordering::Acquire), before.2);
            assert_eq!(gc.statistics()?.bytes_live, before.3.bytes_live);
            assert_eq!(gc.statistics()?.live_objects, before.3.live_objects);
            assert_eq!(gc.managed_bytes(), before.4);
            assert!(gc.publication.try_write().is_some());
            assert!(
                gc.publication_clock
                    .as_ref()
                    .is_none_or(|clock| clock.active() == 0)
            );
            drop(construction);
            gc.collect_full()?;
            assert_eq!(parent.first().map(|value| **value), Some(42));
            assert!(weak.upgrade().is_some());
            drop(parent);
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(not(feature = "experimental-identity-index"))]
#[test]
fn registration_rejects_partial_write_sets_without_repairing_them() -> Result<()> {
    for concurrent in [false, true] {
        for case in 0..3 {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let root = Gc::try_new(&gc, 7_u64)?;
            let address = root.as_ptr() as usize;
            let metadata = gc.metadata_for_address(address).expect("descriptor");
            let id = metadata.id;
            // Controlled corruption models a stopped old multi-index publisher.
            // No collection or payload access occurs until the original complete
            // registry is restored.
            gc.objects.replace_for_test(address, None);
            gc.set_identity_hint(id.to_i64().cast_unsigned(), None);
            gc.objects.set_young_for_test(address, false);
            match case {
                0 => {
                    gc.objects.replace_for_test(address, Some(metadata.clone()));
                }
                1 => {
                    gc.set_identity_hint(
                        id.to_i64().cast_unsigned(),
                        Some(address.wrapping_add(1)),
                    );
                }
                _ => {
                    gc.objects.set_young_for_test(address, true);
                }
            }
            let before_object = gc.objects.lock(address).get(&address).map(|value| value.id);
            let before_identity = gc.identities.get(id.to_i64().cast_unsigned());
            let before_young = gc.objects.contains_young(address);
            let before_marks = [
                metadata.retention(0, gc.domain_at(0).epoch.load(Ordering::Acquire)),
                metadata.retention(1, gc.domain_at(1).epoch.load(Ordering::Acquire)),
            ];
            let before_bytes = gc.statistics()?.bytes_live;
            let mut roots = crate::construction::Roots::new(&gc);
            let construction = roots.enter().detach();
            let rejected = gc.register_prepared_checked(&metadata, &construction);
            assert!(
                rejected.is_err(),
                "partial registration was repaired by replay"
            );
            assert_eq!(
                gc.objects.lock(address).get(&address).map(|value| value.id),
                before_object
            );
            assert_eq!(
                gc.identities.get(id.to_i64().cast_unsigned()),
                before_identity
            );
            assert_eq!(gc.objects.contains_young(address), before_young);
            assert_eq!(
                [
                    metadata.retention(0, gc.domain_at(0).epoch.load(Ordering::Acquire)),
                    metadata.retention(1, gc.domain_at(1).epoch.load(Ordering::Acquire))
                ],
                before_marks
            );
            assert_eq!(gc.statistics()?.bytes_live, before_bytes);
            gc.objects.replace_for_test(address, Some(metadata.clone()));
            gc.set_identity_hint(id.to_i64().cast_unsigned(), Some(address));
            gc.objects.set_young_for_test(address, true);
            drop(construction);
            drop(root);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn exact_address_reservations_do_not_hold_phase_or_live_operation_guards() {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .metadata_shards(1)
                .build(),
        );
        let _guard = gc.objects.lock(8);
        let first = gc.objects.reserve(8).expect("vacant exact slot");
        assert!(gc.objects.reserve(8).is_none());
        let second = gc.objects.reserve(16).expect("other slot on same page");
        second.mark_young();
        assert!(!gc.objects.contains_young(8));
        assert!(gc.objects.contains_young(16));
        assert!(gc.publication.try_write().is_some());
        assert!(
            gc.publication_clock
                .as_ref()
                .is_none_or(|clock| clock.active() == 0)
        );
        drop(second);
        assert!(!gc.objects.contains_young(16));
        assert!(gc.objects.reserve(8).is_none());
        drop(first);
        assert!(gc.objects.reserve(8).is_some());
    }
}

#[test]
fn a_revoked_tree_guard_cannot_remove_a_registration_after_numeric_id_wrap() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .root_storage(crate::RootStorage::Tree)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let original = Gc::try_new(&gc, 7_u64)?;
        let revoked = gc.roots.matching(original.as_ptr().addr())[0];
        let padding = Gc::try_new(&gc, 99_u64)?;
        // SAFETY: no collection runs before the replacement registration below;
        // the stale guard is subsequently dropped without accessing its payload.
        unsafe { gc.remove_root(&**original) };
        // No collection occurs while this allocation is temporarily unrooted.
        // Crossing usize::MAX reproduces the same wrap on native and wasm32.
        gc.next_root_id.store(usize::MAX, Ordering::Relaxed);
        let manual = gc.add_root(&padding);
        assert_eq!(manual, usize::MAX);
        assert_eq!(gc.next_root_id.load(Ordering::Relaxed), 0);
        gc.remove_root_by_id(manual);
        let replacement = gc.create_root_guard(original.clone_gc());
        let reused = gc.roots.matching(original.as_ptr().addr())[0];
        assert_eq!(reused.id, revoked.id);
        assert_ne!(reused.generation, revoked.generation);
        drop(original);
        assert_eq!(
            gc.root_statistics().compatibility_roots,
            2,
            "stale guard removed a replacement registration"
        );
        gc.collect_full()?;
        assert_eq!(**replacement, 7);
        assert_eq!(**padding, 99);
        drop((replacement, padding));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

struct BarrierProbe {
    edge: Option<Gc<u64>>,
    unroots: Arc<AtomicUsize>,
}

// SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
unsafe impl Trace for BarrierProbe {
    fn root(&mut self) {
        self.edge.root();
    }
    fn trace(&self, gc: &GarbageCollector) {
        self.edge.trace(gc);
    }
    fn needs_write_barrier(&self) -> bool {
        self.edge.is_some()
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        self.unroots.fetch_add(1, Ordering::Relaxed);
        if let Some(edge) = &mut self.edge {
            unsafe { Trace::unroot(edge, owner) };
        }
    }
}

#[test]
fn dynamic_leaf_writes_preserve_hooks_and_later_reference_publication() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .promotion_age(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let unroots = Arc::new(AtomicUsize::new(0));
        let root = Gc::try_new(
            &gc,
            crate::cell::RwLock::new(BarrierProbe {
                edge: None,
                unroots: unroots.clone(),
            }),
        )?;
        let address = root.as_ptr() as usize;
        let metadata = gc.metadata_for_address(address).expect("metadata");
        gc.collect_full()?;
        assert_eq!(metadata.age.load(Ordering::Acquire), 1);
        generation_tests::finish_young(&gc)?; // prune the newly promoted empty owner
        let initial = unroots.load(Ordering::Relaxed);
        root.write().edge = None;
        assert_eq!(unroots.load(Ordering::Relaxed), initial + 1);
        assert_eq!(metadata.mutation.load(Ordering::Acquire), 0);
        assert!(!gc.remembered.lock(address).contains(&address));
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        root.write().edge = Some(child);
        assert_eq!(metadata.mutation.load(Ordering::Acquire), 1);
        assert!(gc.remembered.lock(address).contains(&address));
        gc.collect_young();
        for _ in 0..10000 {
            if gc.step() {
                break;
            }
        }
        assert!(!gc.pending());
        assert_eq!(weak.upgrade().as_deref(), Some(&42));
        root.write().edge = None;
        assert_eq!(unroots.load(Ordering::Relaxed), initial + 3);
        assert_eq!(metadata.mutation.load(Ordering::Acquire), 1);
        drop(root);
        gc.collect_full()?;
        gc.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn leaf_write_hooks_finish_while_the_legacy_publication_writer_is_held() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let unroots = Arc::new(AtomicUsize::new(0));
        let dynamic = Gc::try_new(
            &gc,
            crate::cell::RwLock::new(BarrierProbe {
                edge: None,
                unroots: unroots.clone(),
            }),
        )?;
        let scalar = Gc::try_new(&gc, crate::cell::RwLock::new(7_u64))?;
        let initial = unroots.load(Ordering::Relaxed);
        let writer = gc.publication.write();
        let (send, receive) = std::sync::mpsc::channel();
        let worker = {
            let dynamic = dynamic.clone();
            let scalar = scalar.clone();
            thread::spawn(move || {
                dynamic.write().edge = None;
                *scalar.write() = 42;
                send.send(()).expect("completed leaf writes");
            })
        };
        let completed = receive.recv_timeout(std::time::Duration::from_secs(10));
        drop(writer); // release a regression's dependency before joining
        worker
            .join()
            .map_err(|_| crate::Error::SyncError("leaf writer panicked".into()))?;
        completed.map_err(|error| crate::Error::SyncError(error.to_string()))?;
        assert_eq!(*scalar.read(), 42);
        assert_eq!(unroots.load(Ordering::Relaxed), initial + 1);
        drop((dynamic, scalar));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn counted_operations_finish_while_the_legacy_writer_is_held() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .diagnostics(true)
            .root_storage(crate::RootStorage::Contexts)
            .barrier_buffer_size(4)
            .metadata_shards(16)
            .independent_marking(true)
            .work_stealing(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let gate = gc.publication.write();
    let (send, receive) = std::sync::mpsc::channel();
    let worker = {
        let gc = gc.clone();
        thread::spawn(move || {
            let result = gc.register_context().with(|| -> Result<()> {
                let root = Gc::try_new(&gc, 42_u64)?;
                let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
                weak.set_owner(0);
                let weak_copy = weak.clone();
                let copy = Gc::<u64>::from_handle_rooted(&gc, root.as_handle_i64())?;
                gc.write_barrier(&root);
                gc.collect_full()?;
                assert_eq!(weak.upgrade().as_deref(), Some(&42));
                drop((root, copy));
                gc.collect_full()?;
                assert!(weak_copy.upgrade().is_none());
                gc.stop()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
                Ok(())
            });
            send.send(result).expect("test completion");
        })
    };
    let completed = receive.recv_timeout(std::time::Duration::from_secs(30));
    drop(gate); // release a hidden dependency before joining a failing worker
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    completed.map_err(|error| crate::Error::SyncError(error.to_string()))??;
    assert_eq!(gc.diagnostics().publication_read_wait.count, 0);
    assert_eq!(gc.diagnostics().publication_write_wait.count, 0);
    assert_eq!(gc.diagnostics().active_publications, 0);
    assert!(gc.diagnostics().publication_transitions > 0);
    Ok(())
}

#[test]
fn counted_pending_owner_defers_its_slot_without_blocking_collection() -> Result<()> {
    for success in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let target = Gc::try_new(&gc, 42_u64)?;
        let retained = crate::GcWeak::new(&target, crate::ReferenceKind::Final, 0);
        drop(target);
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if lock(&gc.domains[1].cycle).phase == Phase::References {
                break;
            }
        }
        assert_eq!(lock(&gc.domains[1].cycle).phase, Phase::References);
        let owner = if success {
            Some(Gc::try_new(&gc, 7_u64)?)
        } else {
            None
        };
        let mut construction = crate::construction::Roots::new(&gc);
        let scope = construction.enter();
        retained.set_owner(
            owner
                .as_ref()
                .map_or(usize::MAX, |owner| owner.as_ptr() as usize),
        );
        assert!(gc.publication.try_write().is_some());
        assert_eq!(
            gc.publication_clock.as_ref().map(|clock| clock.active()),
            Some(0)
        );
        for _ in 0..3 {
            gc.collect_full()?;
            assert!(!gc.pending());
            assert!(retained.upgrade().is_some());
            assert!(!gc.has_pending_references());
        }
        // Release publication after success or failed construction.
        drop(scope);
        gc.collect_full()?;
        assert!(!gc.pending());
        assert_eq!(retained.upgrade().is_some(), success);
        assert_eq!(
            gc.take_pending_reference::<u64>()?.as_deref(),
            success.then_some(&7)
        );
        assert!(!gc.has_pending_references());
        retained.clear();
        drop(owner);
        gc.stop()?;
        assert_eq!(gc.statistics()?.live_objects, 0);
    }
    Ok(())
}

#[test]
fn construction_cannot_reverse_a_closed_reference_or_retirement_decision() -> Result<()> {
    for concurrent in [false, true] {
        for kind in [
            crate::ReferenceKind::Soft,
            crate::ReferenceKind::Weak,
            crate::ReferenceKind::Final,
            crate::ReferenceKind::Phantom,
            crate::ReferenceKind::PhantomRetained,
        ] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let target = Gc::try_new(&gc, 42_u64)?;
            // An absent owner does not retain Final/PhantomRetained graphs.
            let link = crate::GcWeak::new(&target, kind, usize::MAX);
            drop(target);
            let stage = match kind {
                crate::ReferenceKind::Soft | crate::ReferenceKind::Weak => 1,
                crate::ReferenceKind::Phantom => 3,
                crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained => 5,
            };
            gc.collect();
            let at_decision = || {
                let cycle = lock(&gc.domains[1].cycle);
                if stage == 5 {
                    cycle.phase == Phase::Sweep
                } else {
                    cycle.phase == Phase::References && cycle.reference_stage == stage
                }
            };
            for _ in 0..1000 {
                gc.step();
                if at_decision() {
                    break;
                }
            }
            assert!(at_decision());
            assert!(link.upgrade().is_none());
            let mut construction = crate::construction::Roots::new(&gc);
            let scope = construction.enter();
            link.set_owner(usize::MAX);
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
            gc.collect_full()?;
            assert_eq!(gc.statistics()?.live_objects, 0);
            assert!(link.upgrade().is_none());
            drop(scope);
            gc.stop()?;
            assert!(!gc.has_pending_references());
        }
    }
    Ok(())
}

#[test]
fn a_private_parent_can_install_its_root_while_a_phase_can_close() -> Result<()> {
    fn probe(gc: &GarbageCollector) {
        assert!(
            gc.publication.try_write().is_some(),
            "private root holds phase admission"
        );
        if let Some(clock) = &gc.publication_clock {
            assert!(
                clock.observe().is_some(),
                "private root prevents phase observation"
            );
        }
    }
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .build(),
        );
        *lock(&gc.tree_root_insert_probe) = Some(probe);
        let parent = Gc::try_new(&gc, 17_u64)?;
        *lock(&gc.tree_root_insert_probe) = None;
        gc.collect_full()?;
        assert_eq!(**parent, 17);
        drop(parent);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

type ObservedBirthProbe = fn(&GarbageCollector, &ObjectMetadata);
type PendingBirthProbe = fn(&GarbageCollector, &MetadataRef);

thread_local! {
    static LINKED_GRAPH_PROBE: std::cell::Cell<Option<fn(&GarbageCollector)>> = const { std::cell::Cell::new(None) };
    static OBSERVED_ACTIVATION_PROBE: std::cell::Cell<Option<ObservedBirthProbe>> = const { std::cell::Cell::new(None) };
    static PENDING_BIRTH_PROBE: std::cell::Cell<Option<PendingBirthProbe>> = const { std::cell::Cell::new(None) };
}

pub(super) fn inspect_linked_graph(gc: &GarbageCollector) {
    if let Some(probe) = LINKED_GRAPH_PROBE.take() {
        probe(gc);
    }
}

pub(super) fn inspect_before_activation(gc: &GarbageCollector, metadata: &ObjectMetadata) {
    if let Some(probe) = OBSERVED_ACTIVATION_PROBE.take() {
        probe(gc, metadata);
    }
}

pub(super) fn inspect_pending_birth(gc: &GarbageCollector, metadata: &MetadataRef) {
    if let Some(probe) = PENDING_BIRTH_PROBE.take() {
        probe(gc, metadata);
    }
}

#[test]
fn cooperative_full_gc_can_pass_an_indexed_private_graph() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        let child = Gc::try_new(&gc, 73_u64)?;
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        PENDING_BIRTH_PROBE.set(Some(|gc, metadata| {
            let address = metadata.ptr().0;
            assert_eq!(gc.address_for_identity(metadata.id), Some(address));
            assert!(gc.objects.contains_young(address));
            assert!(gc.metadata_for_address(address).is_none());
            gc.collect_full()
                .expect("cooperative cycle passes private birth");
            gc.collect();
            assert!((0..4096).any(|_| {
                gc.step();
                lock(&gc.domains[1].cycle).phase == Phase::Sweep
            }));
            assert!(!metadata.birth.is_live());
        }));
        let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
        let metadata = gc
            .metadata_for_address(parent.as_ptr() as usize)
            .expect("activated parent");
        assert!(metadata.marked(1, gc.domains[1].epoch.load(Ordering::Acquire)));
        while !gc.step() {}
        assert_eq!(parent.first().map(|edge| **edge), Some(73));
        assert_eq!(weak.upgrade().map(|root| *root), Some(73));
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static INDEXED_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static IDLE_BIRTH_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static PENDING_BIRTH_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static OBSERVED_ACTIVATION_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static GRAPH_SLOT_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static BEFORE_GRAPH_LINK_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static BIRTH_DECISION_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static YOUNG_SLOT_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    pub(super) static INDEX_COMMIT_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static TYPED_ACTIVATION_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static TYPED_LOOKUP_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static COVERED_REMOVAL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static DROP_COVERAGE_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static EXTERNAL_TRACE_EPOCH: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
    static TYPED_PROPOSALS_PAUSE: std::cell::Cell<Option<ClaimPause>> = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn record_covered_removal() {
    COVERED_REMOVAL.set(true);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_drop_coverage() {
    wait_for_constructor_resume(DROP_COVERAGE_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_external_trace_epoch() {
    wait_for_constructor_resume(EXTERNAL_TRACE_EPOCH.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_typed_proposals() {
    wait_for_constructor_resume(TYPED_PROPOSALS_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_typed_lookup() {
    wait_for_constructor_resume(TYPED_LOOKUP_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_typed_activation() {
    wait_for_constructor_resume(TYPED_ACTIVATION_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "forced root lookup schedule")]
fn a_stopped_typed_identity_lookup_allows_unrelated_capped_recovery() -> Result<()> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Waker};
    use std::time::Duration;

    let bytes = Gc::allocation_size(&11_u64)?;
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for (activated, keep_original) in [(false, false), (false, true), (true, false)] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .max_heap_bytes(2 * bytes)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .metadata_shards(1)
                        .max_pause_time_us(0)
                        .root_storage(storage)
                        .concurrent_publication(counted)
                        .build(),
                );
                let context = gc.register_context();
                drop(Gc::try_new(&gc, 11_u64)?);
                let mut survivor = Some(Gc::try_new(&gc, 17_u64)?);
                let identity = survivor.as_ref().expect("root").as_handle_i64();
                assert!(Gc::<i64>::from_handle_rooted(&gc, identity).is_err());
                let (entered, parked) = mpsc::channel();
                let (resume, receiver) = mpsc::channel();
                let resolver_gc = gc.clone();
                let resolver = thread::spawn(move || {
                    let hook = if activated {
                        &TYPED_ACTIVATION_PAUSE
                    } else {
                        &TYPED_LOOKUP_PAUSE
                    };
                    hook.set(Some(ClaimPause {
                        entered,
                        resume: receiver,
                    }));
                    context.with(|| Gc::<u64>::from_handle_rooted(&resolver_gc, identity))
                });
                parked
                    .recv_timeout(Duration::from_secs(5))
                    .expect("typed root handoff");
                if !keep_original {
                    drop(survivor.take());
                }
                let peer_gc = gc.clone();
                let (done, result) = mpsc::channel();
                let peer = thread::spawn(move || {
                    let mut allocation = Box::pin(Gc::try_new_async(&peer_gc, 23_u64));
                    let mut context = TaskContext::from_waker(Waker::noop());
                    for _ in 0..1024 {
                        if let Poll::Ready(root) = allocation.as_mut().poll(&mut context) {
                            // A recovery ticket may free its first victim before
                            // reaching the target. Finish the full cycle while parked.
                            let root = root.and_then(|root| {
                                peer_gc.collect_full()?;
                                Ok(root)
                            });
                            done.send(root).expect("publish capped result");
                            return;
                        }
                    }
                });
                let outcome = result.recv_timeout(Duration::from_secs(3));
                resume.send(()).expect("resume typed lookup");
                let acquisition = resolver.join().expect("typed lookup");
                peer.join().expect("capped peer");
                assert_eq!(**outcome.expect("capped peer completed while parked")?, 23);
                if activated || keep_original {
                    let root = acquisition?;
                    assert_eq!(root.as_handle_i64(), identity);
                    assert_eq!(*root, 17);
                } else {
                    assert!(
                        acquisition.is_err(),
                        "scan passed and retired a private proposal"
                    );
                }
                drop(survivor);
                gc.stop()?;
                assert_eq!(gc.root_statistics().reserved_slots, 0);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[derive(Clone, Copy)]
enum RootSource {
    Identity,
    Edge,
    Address,
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[expect(
    clippy::unwrap_in_result,
    reason = "deterministic bounded peer schedule"
)]
fn run_parked_root_schedule(
    counted: bool,
    storage: crate::RootStorage,
    committed: bool,
    keep_parent: bool,
    source: RootSource,
) -> Result<()> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Waker};
    use std::time::Duration;
    let scalar = Gc::allocation_size(&23_u64)?;
    let node = Gc::allocation_size(&Vec::<Gc<u64>>::with_capacity(1))?;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(2 * scalar + 2 * node)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .metadata_shards(1)
            .root_storage(storage)
            .concurrent_publication(counted)
            .build(),
    );
    let context = gc.register_context();
    drop(Gc::try_new(&gc, 11_u64)?);
    let grandchild = Gc::try_new(&gc, 17_u64)?;
    let target = Gc::try_new(&gc, vec![grandchild.into_rooted()])?;
    let handle = target.as_handle_i64();
    let addr = target.as_ptr() as usize;
    let mut edge = target.clone_gc();
    // SAFETY: the original target still owns the value. The worker
    // uses only the identity edge until a checked clone is acquired.
    unsafe { edge.unroot() };
    let mut parent = Some(Gc::try_new(&gc, vec![target.into_rooted()])?);
    gc.phase_clock.disable_certification();
    gc.collect();
    assert!((0..1024).any(|_| {
        gc.step();
        lock(&gc.domains[1].cycle).phase == Phase::Mark
    }));
    assert!(
        !gc.objects
            .lookup(addr)
            .unwrap()
            .marked(1, gc.domains[1].epoch.load(Ordering::Acquire))
    );
    let (entered, parked) = mpsc::channel();
    let (resume, receiver) = mpsc::channel();
    let resolver_gc = gc.clone();
    let lookup = thread::spawn(move || {
        let hook = if committed {
            &TYPED_ACTIVATION_PAUSE
        } else {
            &TYPED_PROPOSALS_PAUSE
        };
        hook.set(Some(ClaimPause {
            entered,
            resume: receiver,
        }));
        context.with(|| match source {
            RootSource::Edge => Ok(edge.rooted_clone()),
            RootSource::Address => {
                Gc::<Vec<Gc<u64>>>::from_raw_rooted(&resolver_gc, i64::try_from(addr).unwrap())
            }
            RootSource::Identity => Gc::<Vec<Gc<u64>>>::from_handle_rooted(&resolver_gc, handle),
        })
    });
    parked
        .recv_timeout(Duration::from_secs(5))
        .expect("typed proposal boundary");
    if !keep_parent {
        drop(parent.take());
    }
    let mut request = Box::pin(Gc::try_new_async(&gc, 23_u64));
    let mut cx = TaskContext::from_waker(Waker::noop());
    let outcome = (0..1024).find_map(|_| match request.as_mut().poll(&mut cx) {
        Poll::Pending => None,
        Poll::Ready(value) => Some(value),
    });
    if outcome.is_some() {
        // Bound each pass independently so a lost claim cannot hang the test.
        for _ in 0..2 {
            let ticket = gc.request_full_cycle();
            assert!((0..8192).any(|_| {
                gc.step();
                gc.completed_full_ticket() >= ticket
            }));
        }
    }
    resume.send(()).expect("resume resolver");
    let root = lookup.join().expect("resolver");
    assert_eq!(
        **outcome.expect("capped peer exhausted its poll budget")?,
        23
    );
    if committed || keep_parent {
        let root = root?;
        assert_eq!(root.as_handle_i64(), handle);
        assert_eq!(root.first().map(|child| **child), Some(17));
    } else {
        assert!(root.is_err());
    }
    drop(parent);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn a_parked_root_of_an_unvisited_child_never_holds_capped_recovery() -> Result<()> {
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for (committed, keep_parent, source) in [
                (false, false, RootSource::Identity),
                (false, true, RootSource::Identity),
                (true, false, RootSource::Identity),
                (false, true, RootSource::Edge),
                (true, false, RootSource::Edge),
                (false, false, RootSource::Address),
                (false, true, RootSource::Address),
                (true, false, RootSource::Address),
            ] {
                run_parked_root_schedule(counted, storage, committed, keep_parent, source)?;
            }
        }
    }
    Ok(())
}

fn drop_test_registration(
    gc: &GarbageCollector,
    context: &crate::ExecutionContext,
    address: usize,
) -> crate::roots::Registration {
    match gc.configuration.root_storage {
        crate::RootStorage::Tree => {
            let roots = gc
                .typed_slots
                .get_or_init(|| Box::new(crate::roots::RootSlots::default()));
            let record = roots.prepare_typed_attempt(address);
            assert!(record.commit_typed());
            crate::roots::Registration::Overflow(record)
        }
        crate::RootStorage::Slots => gc
            .scanned_slots()
            .unwrap()
            .prepare(address)
            .commit(address, None),
        crate::RootStorage::Contexts => context
            .0
            .roots
            .prepare(address)
            .commit(address, Some(context.0.clone())),
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "bounded inter-thread test schedule"
)]
fn typed_root_drop_orders_both_winners_against_a_new_full_cycle() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for storage in [
        crate::RootStorage::Tree,
        crate::RootStorage::Slots,
        crate::RootStorage::Contexts,
    ] {
        for counted in [false, true] {
            for scan_first in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .concurrent_publication(counted)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .build(),
                );
                let context = gc.register_context();
                let mut removers = Vec::new();
                // One startup step can already visit a root. Park three independent
                // removers so the remaining sources definitely precede their scan.
                for value in 17..20_u64 {
                    let child = Gc::try_new(&gc, value)?;
                    let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
                    let addr = parent.as_ptr().addr();
                    let handle = parent.as_handle_i64();
                    let registration = drop_test_registration(&gc, &context, addr);
                    drop(parent);
                    let (entered, paused) = mpsc::channel();
                    let (resume, wait) = mpsc::channel();
                    let peer = gc.clone();
                    let stale = registration.clone();
                    let remover = thread::spawn(move || {
                        DROP_COVERAGE_PAUSE.set(Some(ClaimPause {
                            entered,
                            resume: wait,
                        }));
                        peer.remove_detached_root(&registration);
                        COVERED_REMOVAL.take()
                    });
                    paused
                        .recv_timeout(Duration::from_secs(5))
                        .expect("removal certified");
                    removers.push((resume, remover, handle, value, stale));
                }
                let ticket = gc.request_full_cycle();
                gc.step();
                assert_ne!(lock(&gc.domains[1].cycle).phase, Phase::Idle);
                if scan_first {
                    assert!((0..8192).any(|_| {
                        gc.step();
                        gc.completed_full_ticket() >= ticket
                    }));
                }
                let mut outcomes = Vec::new();
                for (resume, remover, handle, value, stale) in removers {
                    resume.send(()).expect("resume removal");
                    outcomes.push((remover.join().expect("remover"), handle, value, stale));
                }
                assert_eq!(outcomes.iter().any(|(won, ..)| *won), !scan_first);
                assert!((0..8192).any(|_| {
                    gc.step();
                    gc.completed_full_ticket() >= ticket
                }));
                for (removed_first, handle, value, stale) in outcomes {
                    let result = Gc::<Vec<Gc<u64>>>::from_handle_rooted(&gc, handle);
                    if removed_first {
                        assert!(
                            result.is_err(),
                            "a root deleted before its scan retained garbage"
                        );
                    } else {
                        assert_eq!(result?.first().map(|child| **child), Some(value));
                    }
                    gc.remove_owned_root(&stale);
                }
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[test]
fn unvisited_deletion_links_independent_young_and_full_traversals() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .root_storage(crate::RootStorage::Slots)
                .concurrent_publication(counted)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        let context = gc.register_context();
        let mut sources = Vec::new();
        for _ in 0..4 {
            let child = Gc::try_new(&gc, 41_u64)?;
            let handle = child.as_handle_i64();
            let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
            let address = parent.as_ptr().addr();
            sources.push((
                gc.objects.lookup(address).unwrap(),
                drop_test_registration(&gc, &context, address),
                handle,
            ));
            drop(parent);
        }
        gc.collect();
        gc.collect_young();
        for index in [1, 0] {
            assert!((0..1024).any(|_| {
                gc.next_domain.store(index, Ordering::Release);
                gc.coordinated_step(false);
                gc.domains[index].active()
            }));
        }
        let epochs = gc
            .domains
            .each_ref()
            .map(|d| d.epoch.load(Ordering::Acquire));
        let index = sources
            .iter()
            .position(|(m, _, _)| (0..2).all(|i| !m.marked(i, epochs[i])))
            .expect("unvisited in both domains");
        let (metadata, source, handle) = sources.swap_remove(index);
        gc.remove_detached_root(&source);
        // Absent admission, both queues own the strong deletion traversal.
        // Neither domain has marked the source synchronously.
        for (i, epoch) in epochs.into_iter().enumerate() {
            assert!(!metadata.marked(i, epoch));
            assert!(
                gc.domains[i]
                    .mark_queue
                    .queued_sample()
                    .is_some_and(|n| n > 0)
            );
        }
        assert!((0..16384).any(|_| gc.step()));
        assert_eq!(*Gc::<u64>::from_handle_rooted(&gc, handle)?, 41);
        gc.collect_full()?;
        assert!(Gc::<u64>::from_handle_rooted(&gc, handle).is_err());
        for (_, source, _) in sources {
            gc.remove_detached_root(&source);
        }
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "deterministic inter-thread schedule"
)]
fn unvisited_root_deletion_orders_traversal_before_removal_or_cancels() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for storage in [
        crate::RootStorage::Tree,
        crate::RootStorage::Slots,
        crate::RootStorage::Contexts,
    ] {
        for (counted, bind_epochs) in [(false, false), (true, false), (false, true), (true, true)] {
            for visit_first in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .concurrent_publication(counted)
                        .incremental_step_size(1)
                        .allocation_threshold(usize::MAX)
                        .max_pause_time_us(0)
                        .build(),
                );
                let context = gc.register_context();
                // The first one-unit step can already visit one root. Choose an
                // unvisited source after the step, independent of slot ordering.
                let mut sources = Vec::new();
                for _ in 0..3 {
                    let child = Gc::try_new(&gc, 53_u64)?;
                    let child_handle = child.as_handle_i64();
                    let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
                    let address = parent.as_ptr().addr();
                    let metadata = gc.objects.lookup(address).unwrap();
                    let source = drop_test_registration(&gc, &context, address);
                    sources.push((metadata, source, child_handle));
                    drop(parent);
                }
                if bind_epochs {
                    gc.phase_clock.disable_certification();
                }
                let ticket = gc.request_full_cycle();
                assert!((0..4096).any(|_| {
                    gc.step();
                    gc.domains[1].active()
                }));
                let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
                let index = sources
                    .iter()
                    .position(|(m, _, _)| !m.marked(1, epoch))
                    .expect("unvisited parent");
                let (_, source, child_handle) = sources.swap_remove(index);
                let (entered, paused) = mpsc::channel();
                let (resume, wait) = mpsc::channel();
                let peer = gc.clone();
                let remover = thread::spawn(move || {
                    TYPED_PROPOSALS_PAUSE.set(Some(ClaimPause {
                        entered,
                        resume: wait,
                    }));
                    peer.remove_detached_root(&source);
                    COVERED_REMOVAL.take()
                });
                paused
                    .recv_timeout(Duration::from_secs(5))
                    .expect("linked deletion debt");
                let complete = || {
                    (0..8192).any(|_| {
                        gc.step();
                        gc.completed_full_ticket() >= ticket
                    })
                };
                if visit_first {
                    assert!(complete(), "a parked remover must leave the root scannable");
                }
                resume.send(()).unwrap();
                assert_eq!(remover.join().unwrap(), !visit_first);
                assert!(complete());
                let child = Gc::<u64>::from_handle_rooted(&gc, child_handle)?;
                assert_eq!(*child, 53);
                drop(child);
                gc.collect_full()?;
                assert!(Gc::<u64>::from_handle_rooted(&gc, child_handle).is_err());
                for (_, registration, _) in sources {
                    gc.remove_detached_root(&registration);
                }
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn covered_active_typed_drop_preserves_outstanding_child_traversal() -> Result<()> {
    for storage in [
        crate::RootStorage::Tree,
        crate::RootStorage::Slots,
        crate::RootStorage::Contexts,
    ] {
        for counted in [false, true] {
            for bind_epochs in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .root_storage(storage)
                        .concurrent_publication(counted)
                        .incremental_step_size(1)
                        .allocation_threshold(usize::MAX)
                        .max_pause_time_us(0)
                        .build(),
                );
                let child = Gc::try_new(&gc, 17_u64)?;
                let child_handle = child.as_handle_i64();
                let child_metadata = gc.objects.lookup(child.as_ptr().addr()).unwrap();
                let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
                let address = parent.as_ptr().addr();
                let meta = gc.objects.lookup(address).unwrap();
                let context = gc.register_context();
                let registration = drop_test_registration(&gc, &context, address);
                drop(parent);
                if bind_epochs {
                    gc.phase_clock.disable_certification();
                }
                let ticket = gc.request_full_cycle();
                assert!((0..4096).any(|_| {
                    gc.step();
                    let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
                    meta.retention(1, epoch) == Some(Retention::Strong)
                        && !child_metadata.marked(1, epoch)
                }));
                COVERED_REMOVAL.set(false);
                gc.remove_detached_root(&registration);
                assert!(
                    COVERED_REMOVAL.take(),
                    "active strong coverage should bypass admission"
                );
                assert!((0..8192).any(|_| {
                    gc.step();
                    gc.completed_full_ticket() >= ticket
                }));
                let child = Gc::<u64>::from_handle_rooted(&gc, child_handle)?;
                assert_eq!(*child, 17);
                drop(child);
                gc.collect_full()?;
                assert!(Gc::<u64>::from_handle_rooted(&gc, child_handle).is_err());
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "bounded inter-thread retirement schedule"
)]
fn a_root_born_during_retirement_can_drop_before_or_after_the_next_scan() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for scan_first in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .root_storage(crate::RootStorage::Contexts)
                    .overlapping_generations(true)
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .allocation_threshold(usize::MAX)
                    .max_pause_time_us(0)
                    .build(),
            );
            let context = gc.register_context();
            let anchor = Gc::try_new(&gc, 17_u64)?;
            let previous = gc.request_full_cycle();
            assert!((0..4096).any(|_| {
                gc.step();
                gc.domains[1].weak_phase() == 5
            }));
            let parent = Gc::try_new(&gc, vec![anchor.into_rooted()])?;
            let handle = parent.as_handle_i64();
            let address = parent.as_ptr().addr();
            let registration = drop_test_registration(&gc, &context, address);
            let metadata = gc.objects.lookup(address).unwrap();
            let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
            assert_ne!(metadata.retention(1, epoch), Some(Retention::Strong));
            drop(parent);
            let (entered, paused) = mpsc::channel();
            let (resume, wait) = mpsc::channel();
            let peer = gc.clone();
            let remover = thread::spawn(move || {
                DROP_COVERAGE_PAUSE.set(Some(ClaimPause {
                    entered,
                    resume: wait,
                }));
                peer.remove_detached_root(&registration);
                COVERED_REMOVAL.take()
            });
            paused
                .recv_timeout(Duration::from_secs(5))
                .expect("retirement sampled");
            assert!((0..8192).any(|_| {
                gc.step();
                gc.completed_full_ticket() >= previous
            }));
            let next = gc.request_full_cycle();
            if scan_first {
                assert!((0..8192).any(|_| {
                    gc.step();
                    gc.completed_full_ticket() >= next
                }));
            }
            resume.send(()).unwrap();
            assert_eq!(remover.join().unwrap(), !scan_first);
            assert!((0..8192).any(|_| {
                gc.step();
                gc.completed_full_ticket() >= next
            }));
            let recovered = Gc::<Vec<Gc<u64>>>::from_handle_rooted(&gc, handle);
            if scan_first {
                assert_eq!(*recovered?[0], 17);
            } else {
                assert!(recovered.is_err());
            }
            gc.collect_full()?;
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn both_epochs_claim_released_root_debt_after_strong_closure() -> Result<()> {
    let gc = GarbageCollector::new();
    let root = Gc::try_new(&gc, 17_u64)?;
    let metadata = gc.objects.lookup(root.as_ptr() as usize).unwrap();
    let slots = crate::roots::RootSlots::default();
    let record = slots.prepare_typed_attempt(root.as_ptr() as usize);
    let queues = [0, 1].map(|index| {
        let queue = MarkQueues::new(0, 1);
        queue.open_graph_inbox(4, false);
        queue.finish_strong_births();
        let epoch = queue.birth_epoch().unwrap();
        // An existing weaker mark must be upgraded even if the handle drops
        // before the next claim. The other domain has an independent epoch.
        metadata.mark(index, 4, Retention::Phantom);
        queue.propose_root(
            &epoch,
            crate::work::RootProposal {
                metadata: metadata.clone(),
                record: record.clone(),
                index,
                strength: Retention::Final,
                promotion_age: 2,
            },
        );
        assert_eq!(queue.try_is_empty(), Some(false));
        queue
    });
    assert!(record.commit_typed());
    assert_eq!(record.take_address(), Some(root.as_ptr() as usize));
    slots.finish_overflow(&record);
    for (index, queue) in queues.into_iter().enumerate() {
        let claimed = queue.claim(0, 1, &mut 1);
        assert!(
            claimed
                .items
                .into_iter()
                .flatten()
                .any(|(item, _)| MetadataRef::ptr_eq(&item.metadata, &metadata))
        );
        assert_eq!(metadata.retention(index, 4), Some(Retention::Final));
        assert_eq!(queue.try_is_empty(), Some(true));
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "bounded parked activation schedule"
)]
fn active_epoch_identity_protects_a_typed_graph_with_numeric_certification_disabled() -> Result<()>
{
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for phase in [Phase::Mark, Phase::Sweep] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 17_u64)?.into_rooted();
            let root = Gc::try_new(&gc, vec![child])?;
            let identity = root.as_handle_i64();
            gc.phase_clock.disable_certification();
            gc.collect();
            assert!((0..1024).any(|_| {
                gc.step();
                lock(&gc.domains[1].cycle).phase == phase
            }));
            let (entered, parked) = mpsc::channel();
            let (resume, receiver) = mpsc::channel();
            let other = gc.clone();
            let lookup = thread::spawn(move || {
                TYPED_ACTIVATION_PAUSE.set(Some(ClaimPause {
                    entered,
                    resume: receiver,
                }));
                Gc::<Vec<Gc<u64>>>::from_handle_rooted(&other, identity)
            });
            parked
                .recv_timeout(Duration::from_secs(3))
                .expect("guard-free activation in active epoch");
            drop(root);
            let other = gc.clone();
            let (finished, result) = mpsc::channel();
            let peer = thread::spawn(move || {
                let outcome = (|| -> Result<()> {
                    for _ in 0..2 {
                        other.collect_full()?;
                    }
                    Ok(())
                })();
                finished.send(outcome).expect("collection observer");
            });
            let outcome = result.recv_timeout(Duration::from_secs(3));
            resume.send(()).expect("resume active lookup");
            let protected = lookup.join().expect("lookup")?;
            peer.join().expect("collection");
            outcome.expect("cycles finished before resuming lookup")?;
            assert_eq!(protected.first().map(|child| **child), Some(17));
            drop(protected);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_registration_indexes() {
    wait_for_constructor_resume(INDEXED_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_young_slot() {
    wait_for_constructor_resume(YOUNG_SLOT_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_inside_registration_commit() {
    wait_for_constructor_resume(INDEX_COMMIT_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(crate) fn pause_while_graph_slot_owned() {
    wait_for_constructor_resume(GRAPH_SLOT_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_before_graph_link() {
    wait_for_constructor_resume(BEFORE_GRAPH_LINK_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_after_birth_decision() {
    wait_for_constructor_resume(BIRTH_DECISION_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_before_observed_activation() {
    wait_for_constructor_resume(OBSERVED_ACTIVATION_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_pending_birth() {
    wait_for_constructor_resume(PENDING_BIRTH_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_at_idle_birth() {
    wait_for_constructor_resume(IDLE_BIRTH_PAUSE.take());
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_while_an_indexed_birth_is_pending() -> Result<()> {
    for phase in [None, Some(Phase::Mark), Some(Phase::Sweep)] {
        check_parked_constructor_recovery(phase, |_, pause| PENDING_BIRTH_PAUSE.set(Some(pause)))?;
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn wait_for_constructor_resume(pause: Option<ClaimPause>) {
    if let Some(pause) = pause {
        pause.entered.send(()).expect("constructor parked");
        pause
            .resume
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("resume constructor");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_while_a_private_parent_is_parked_at_root_insertion() -> Result<()> {
    check_parked_constructor_recovery(None, |gc, pause| {
        *lock(&gc.metadata_prepare_pause) = Some(pause);
        *lock(&gc.tree_root_insert_probe) = Some(GarbageCollector::pause_metadata_preparation);
    })
}

#[cfg(all(
    not(feature = "experimental-identity-index"),
    not(target_family = "wasm"),
    not(miri)
))]
#[test]
fn capped_allocation_reclaims_with_a_parked_identity_page_preparer() -> Result<()> {
    check_parked_constructor_recovery(None, |gc, pause| {
        *lock(&gc.metadata_prepare_pause) = Some(pause);
    })
}

#[test]
fn an_idle_indexed_birth_preserves_a_pending_start_observation() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let clock = gc.publication_clock.as_ref().expect("counted clock");
    for rebased in [false, true] {
        if rebased {
            clock.disable_certification();
        }
        let observed = clock.observe().expect("idle domains");
        let scalar = Gc::try_new(&gc, 42_u64)?;
        let graph = Gc::try_new(&gc, Vec::<Gc<u64>>::new())?;
        // Simulate the start CAS without changing domain state. Later real
        // collections verify both private roots survived the unstamped births.
        assert!(clock.transition(observed, 0, 0));
        gc.collect_full()?;
        assert_eq!(**scalar, 42);
        assert!(graph.is_empty());
        drop((scalar, graph));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "channel and join failures invalidate the forced schedule"
)]
fn counted_cycles_start_on_either_side_of_an_indexed_birth() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    for kind in [
        CollectionKind::Young,
        CollectionKind::Full,
        CollectionKind::Old,
    ] {
        for after_decision in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(true)
                    .old_collection_interval(usize::MAX)
                    .promotion_age(1)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 41_u64)?;
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            if kind == CollectionKind::Old {
                gc.collect_full()?;
            }
            let (entered, parked) = mpsc::channel();
            let (resume, receiver) = mpsc::channel();
            let heap = gc.clone();
            let constructor = thread::spawn(move || {
                let hook = if after_decision {
                    &BIRTH_DECISION_PAUSE
                } else {
                    &INDEX_COMMIT_PAUSE
                };
                hook.set(Some(ClaimPause {
                    entered,
                    resume: receiver,
                }));
                Gc::try_new(&heap, vec![child.into_rooted()])
            });
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("constructor reached birth boundary");
            match kind {
                CollectionKind::Young => gc.collect_young(),
                CollectionKind::Full => gc.collect(),
                CollectionKind::Old => gc.collect_old(),
            }
            let index = usize::from(kind != CollectionKind::Young);
            let started = (0..64).any(|_| {
                gc.step();
                gc.domains[index].active()
            });
            let epoch = gc.domains[index].epoch.load(Ordering::Acquire);
            resume.send(()).expect("resume birth");
            let graph = constructor.join().expect("constructor")?;
            assert!(
                started,
                "{kind:?} start waited for the publisher, after_decision={after_decision}"
            );
            if kind == CollectionKind::Old {
                let descriptor = gc
                    .metadata_for_address(graph.as_ptr() as usize)
                    .expect("graph");
                assert_eq!(
                    descriptor.old_birth.load(Ordering::Acquire) == epoch,
                    !after_decision
                );
            }
            while !gc.step() {}
            assert_eq!(weak.upgrade().map(|root| *root), Some(41));
            assert_eq!(graph.first().map(|edge| **edge), Some(41));
            drop(graph);
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "forced constructor schedule")]
fn a_passed_idle_birth_joins_the_current_epoch_after_two_collections() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for storage in [
        crate::RootStorage::Tree,
        crate::RootStorage::Slots,
        crate::RootStorage::Contexts,
    ] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .root_storage(storage)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let child = Gc::try_new(&gc, 71_u64)?;
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let (entered, parked) = mpsc::channel();
        let (resume, receiver) = mpsc::channel();
        let heap = gc.clone();
        let constructor = thread::spawn(move || {
            IDLE_BIRTH_PAUSE.set(Some(ClaimPause {
                entered,
                resume: receiver,
            }));
            Gc::try_new(&heap, vec![child.into_rooted()])
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("idle clock decision");
        gc.collect_full()?;
        gc.collect_full()?;
        gc.collect();
        assert!((0..4096).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::Sweep
        }));
        let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
        resume.send(()).expect("resume idle publisher");
        let parent = constructor.join().expect("constructor")?;
        let metadata = gc
            .metadata_for_address(parent.as_ptr() as usize)
            .expect("activated graph");
        assert!(
            metadata.marked(1, epoch),
            "a passed idle birth must join the current sweep"
        );
        while !gc.step() {}
        assert_eq!(weak.upgrade().map(|child| *child), Some(71));
        assert_eq!(parent.first().map(|edge| **edge), Some(71));
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "forced constructor unwind")]
fn unwinding_partial_and_passed_births_releases_the_exact_indexes_and_charge() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for (counted, installed) in [(false, false), (false, true), (true, false), (true, true)] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 11_u64)?;
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let (entered, parked) = mpsc::channel();
        let (resume, receiver) = mpsc::channel();
        let (reserved, reservation) = mpsc::channel();
        *lock(&gc.metadata_registration_observer) = Some(reserved);
        let heap = gc.clone();
        let constructor = thread::spawn(move || {
            let hook = if installed {
                &PENDING_BIRTH_PAUSE
            } else {
                &YOUNG_SLOT_PAUSE
            };
            hook.set(Some(ClaimPause {
                entered,
                resume: receiver,
            }));
            Gc::try_new(&heap, vec![child.into_rooted()])
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("physical insertion");
        let (address, id) = reservation
            .recv_timeout(Duration::from_secs(5))
            .expect("exact reservation");
        assert_eq!(gc.address_for_identity(id), installed.then_some(address));
        assert_eq!(gc.objects.lock(address).occupied(), installed);
        assert!(gc.objects.contains_young(address));
        assert!(gc.metadata_for_address(address).is_none());
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        drop(resume); // Inject an unwind inside the pre-activation hook.
        assert!(constructor.join().is_err());
        assert_eq!(gc.address_for_identity(id), None);
        assert!(!gc.objects.lock(address).occupied());
        assert!(!gc.objects.contains_young(address));
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.statistics()?.live_objects, 0);
        assert_eq!(gc.statistics()?.bytes_live, 0);
        assert_eq!(gc.managed_bytes(), 0);
        let next = Gc::try_new(&gc, 23_u64)?;
        assert_eq!(**next, 23);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "channel and join failures invalidate the forced schedule"
)]
fn capped_reservation_reclaims_other_shards_during_counted_registration() -> Result<()> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Wake, Waker};
    use std::time::Duration;

    struct Notify(AtomicBool);
    impl Wake for Notify {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }
    const VICTIMS: usize = 64;
    let bytes = Gc::allocation_size(&0_u64)?;
    for independent in [false, true] {
        let cap = (VICTIMS + 1) * bytes;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .metadata_shards(16)
                .concurrent_publication(true)
                .independent_phases(independent)
                .max_heap_bytes(cap)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let mut buckets = BTreeSet::new();
        for _ in 0..VICTIMS {
            let victim = Gc::try_new(&gc, 1_u64)?;
            buckets.insert(crate::index::address_bucket(victim.as_ptr() as usize) & 15);
        }
        assert!(buckets.len() > 1, "need garbage outside the parked shard");
        gc.collect();
        assert!((0..1024).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::Mark
        }));
        let (entered, parked) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let heap = gc.clone();
        let constructor = thread::spawn(move || {
            INDEX_COMMIT_PAUSE.set(Some(ClaimPause {
                entered,
                resume: resumed,
            }));
            Gc::try_new(&heap, 17_u64)
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("constructor holds write set");
        assert_eq!(gc.managed_bytes(), cap);
        let notify = Arc::new(Notify(AtomicBool::new(false)));
        let waker = Waker::from(notify.clone());
        let mut cx = TaskContext::from_waker(&waker);
        let mut peer = Box::pin(gc.reserve_bytes_async(bytes));
        assert!(peer.as_mut().poll(&mut cx).is_pending());
        // This is the exact reservation awaited by Gc::try_new_async. Drive only
        // on real wakes, and keep the credit charged through the progress check.
        let credit = (0..4096).find_map(|_| {
            if !notify.0.swap(false, Ordering::AcqRel) {
                return None;
            }
            match peer.as_mut().poll(&mut cx) {
                Poll::Ready(result) => Some(result),
                Poll::Pending => None,
            }
        });
        let freed = lock(&gc.statistics).bytes_freed;
        resume.send(()).expect("resume constructor");
        let parent = constructor.join().expect("constructor")?;
        let reserved = credit.expect("unrelated reclaim must fund the capped reservation")?;
        assert_eq!(reserved.size, bytes);
        assert!(freed >= bytes);
        assert_eq!(**parent, 17);
        drop((reserved, peer, parent));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_after_registration_indexes_are_released() -> Result<()> {
    for phase in [None, Some(Phase::Mark), Some(Phase::Sweep)] {
        check_parked_constructor_recovery(phase, |_, pause| INDEXED_PAUSE.set(Some(pause)))?;
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_inside_registration_commit() -> Result<()> {
    for phase in [None, Some(Phase::Mark), Some(Phase::Sweep)] {
        check_parked_constructor_recovery(phase, |_, pause| INDEX_COMMIT_PAUSE.set(Some(pause)))?;
        check_parked_constructor_recovery(phase, |_, pause| YOUNG_SLOT_PAUSE.set(Some(pause)))?;
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn a_parked_leaf_birth_allows_capped_allocation_during_mark_and_sweep() -> Result<()> {
    for phase in [Phase::Mark, Phase::Sweep] {
        check_parked_constructor_recovery(Some(phase), |_, pause| {
            BIRTH_DECISION_PAUSE.set(Some(pause));
        })?;
        check_parked_constructor_recovery(Some(phase), |_, pause| {
            OBSERVED_ACTIVATION_PAUSE.set(Some(pause));
        })?;
        check_parked_constructor_recovery(Some(phase), |_, pause| {
            PENDING_BIRTH_PROBE.set(Some(|gc, metadata| {
                assert!(gc.metadata_for_address(metadata.ptr().0).is_none());
            }));
            BIRTH_DECISION_PAUSE.set(Some(pause));
        })?;
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced publication schedule assertions"
)]
fn check_parked_constructor_recovery(
    phase: Option<Phase>,
    install_pause: fn(&GarbageCollector, ClaimPause),
) -> Result<()> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Waker};
    use std::time::Duration;
    let bytes = Gc::allocation_size(&17_u64)?;
    let mut stalled = Vec::new();
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(2 * bytes)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .concurrent_publication(counted)
                .build(),
        );
        drop(Gc::try_new(&gc, 11_u64)?);
        if let Some(phase) = phase {
            gc.collect();
            assert!((0..1024).any(|_| {
                gc.step();
                lock(&gc.domains[1].cycle).phase == phase
            }));
        }
        let (entered, parked) = mpsc::channel();
        let (resume, receiver) = mpsc::channel();
        let constructor_gc = gc.clone();
        let constructor = thread::spawn(move || {
            install_pause(
                &constructor_gc,
                ClaimPause {
                    entered,
                    resume: receiver,
                },
            );
            Gc::try_new(&constructor_gc, 17_u64)
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("constructor reached the requested publication boundary");
        assert_eq!(gc.managed_bytes(), 2 * bytes);
        let mut peer = Box::pin(Gc::try_new_async(&gc, 23_u64));
        let mut context = TaskContext::from_waker(Waker::noop());
        let recovered = (0..1024).find_map(|_| match peer.as_mut().poll(&mut context) {
            Poll::Ready(result) => Some(result),
            Poll::Pending => None,
        });
        // The public statistics snapshot visits the parked object's shard.
        let freed_while_parked = lock(&gc.statistics).bytes_freed;
        drop(peer);
        resume.send(()).expect("resume constructor");
        let parent = constructor.join().expect("constructor")?;
        let Some(other) = recovered else {
            stalled.push(if counted { "counted" } else { "legacy" });
            drop(parent);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
            continue;
        };
        let other = other?;
        assert!(freed_while_parked >= bytes);
        assert_eq!((**parent, **other), (17, 23));
        assert_eq!(gc.managed_bytes(), 2 * bytes);
        drop((parent, other));
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    assert!(
        stalled.is_empty(),
        "capped recovery waited for publisher resumption under {stalled:?}"
    );
    Ok(())
}

struct Scans(Arc<AtomicUsize>);
// SAFETY: the counter is unmanaged. Its trace hook records each visit.
unsafe impl Trace for Scans {
    fn trace(&self, _: &GarbageCollector) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_peer_traces_an_indexed_graph_while_its_publisher_is_parked() -> Result<()> {
    check_parked_graph_recovery(|pause| INDEXED_PAUSE.set(Some(pause)), true)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_after_a_strong_graph_publishes_traversal() -> Result<()> {
    check_parked_graph_recovery(|pause| BIRTH_DECISION_PAUSE.set(Some(pause)), true)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_before_a_strong_graph_publishes_traversal() -> Result<()> {
    check_parked_graph_recovery(|pause| BEFORE_GRAPH_LINK_PAUSE.set(Some(pause)), false)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_while_a_graph_holds_an_unpublished_slot() -> Result<()> {
    check_parked_graph_recovery(|pause| GRAPH_SLOT_PAUSE.set(Some(pause)), false)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn check_parked_graph_recovery(
    install_pause: fn(ClaimPause),
    traversal_visible: bool,
) -> Result<()> {
    let scalar = Gc::allocation_size(&41_u64)?;
    let graph = crate::gc::Allocation::<(Gc<u64>, Scans)>::layout().size();
    let cap = 2 * scalar + graph;
    let mut stalled = Vec::new();
    for counted in [false, true] {
        for hold_injector in [false, true] {
            for numeric in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .max_heap_bytes(cap)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .build(),
                );
                if !numeric {
                    gc.phase_clock.disable_certification();
                }
                if !park_graph_and_allocate(
                    &gc,
                    cap,
                    hold_injector,
                    install_pause,
                    traversal_visible,
                )? {
                    stalled.push((counted, hold_injector, numeric));
                }
            }
        }
    }
    assert!(
        stalled.is_empty(),
        "stalled (counted, held injector, numeric): {stalled:?}"
    );
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn park_graph_and_allocate(
    gc: &Arc<GarbageCollector>,
    cap: usize,
    hold_injector: bool,
    install_pause: fn(ClaimPause),
    traversal_visible: bool,
) -> Result<bool> {
    use std::future::Future;
    use std::sync::mpsc;
    use std::task::{Context as TaskContext, Poll, Waker};
    use std::time::Duration;
    drop(Gc::try_new(gc, 13_u64)?);
    let child = Gc::try_new(gc, 41_u64)?;
    let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
    gc.collect();
    assert!((0..1024).any(|_| {
        gc.step();
        lock(&gc.domains[1].cycle).phase == Phase::Mark
            && gc.domains[1].mark_queue.queued_sample() == Some(0)
    }));
    let exercise = || -> Result<bool> {
        let before = gc.statistics()?;
        let (entered, parked) = mpsc::channel();
        let (resume, receiver) = mpsc::channel();
        let heap = gc.clone();
        let scans = Arc::new(AtomicUsize::new(0));
        let marker = Scans(scans.clone());
        let constructor = thread::spawn(move || {
            install_pause(ClaimPause {
                entered,
                resume: receiver,
            });
            Gc::try_new(&heap, (child.into_rooted(), marker))
        });
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("indexed graph");
        let charged = gc.managed_bytes();
        let queued = gc.domains[1].mark_queue.queued_sample();
        let scans_before_peer = scans.load(Ordering::Relaxed);
        let mut peer = Box::pin(Gc::try_new_async(gc, 23_u64));
        let mut cx = TaskContext::from_waker(Waker::noop());
        let recovered = (0..1024).find_map(|_| match peer.as_mut().poll(&mut cx) {
            Poll::Ready(value) => Some(value),
            Poll::Pending => None,
        });
        drop(peer);
        let finished = (0..1024).any(|_| gc.step());
        let during = gc.statistics()?;
        let scans_while_parked = scans.load(Ordering::Relaxed) - scans_before_peer;
        resume.send(()).expect("resume graph publisher");
        let parent = constructor.join().expect("graph publisher")?;
        let Some(other) = recovered else {
            drop(parent);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
            return Ok(false);
        };
        let other = other?;
        assert_eq!(charged, cap);
        if traversal_visible {
            assert_eq!(queued, Some(1));
            assert!(scans_while_parked > 0, "peer never traced the graph");
            assert!(during.objects_marked > before.objects_marked);
        }
        assert!(finished && during.full_collections > before.full_collections);
        assert!(during.bytes_freed >= before.bytes_freed + size_of::<u64>());
        assert_eq!(**other, 23);
        assert_eq!(weak.upgrade().map(|value| *value), Some(41));
        assert_eq!(*parent.0, 41);
        drop((parent, other));
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
        Ok(true)
    };
    if hold_injector {
        gc.domains[1].mark_queue.with_held_injector_result(exercise)
    } else {
        exercise()
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn capped_allocation_reclaims_while_a_graph_waits_between_certification_and_activation()
-> Result<()> {
    check_parked_graph_recovery(|pause| OBSERVED_ACTIVATION_PAUSE.set(Some(pause)), false)
}

#[test]
fn a_skipped_graph_ticket_cannot_duplicate_the_next_attempts_traversal() -> Result<()> {
    fn leave_cancelled_ticket(gc: &GarbageCollector, _: &ObjectMetadata) {
        for d in &gc.domains {
            assert_eq!(d.mark_queue.queued_sample(), Some(1));
        }
        // Young claims its ticket while the graph is private. Full retains
        // the cancelled ticket even after the next attempt goes live.
        let claim = gc.domains[0].mark_queue.claim(0, 1, &mut 1);
        assert!(claim.items.iter().all(Option::is_none));
    }
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        for i in [1, 0] {
            if i == 1 {
                gc.collect();
            } else {
                gc.collect_young();
            }
            assert!((0..1024).any(|_| {
                gc.next_domain.store(i, Ordering::Release);
                gc.coordinated_step(false);
                lock(&gc.domains[i].cycle).phase == Phase::Mark
            }));
        }
        OBSERVED_ACTIVATION_PROBE.set(Some(|gc, _| {
            // Consume the first attempt in both domains, then leave a cancelled
            // retry ticket in the full queue while the third attempt activates.
            for d in &gc.domains {
                let claim = d.mark_queue.claim(0, 1, &mut 1);
                assert!(claim.items.iter().all(Option::is_none));
            }
            OBSERVED_ACTIVATION_PROBE.set(Some(leave_cancelled_ticket));
        }));
        let scans = Arc::new(AtomicUsize::new(0));
        let parent = Gc::try_new(&gc, Scans(scans.clone()))?;
        assert!(OBSERVED_ACTIVATION_PROBE.take().is_none());
        assert_eq!(gc.domains[1].mark_queue.queued_sample(), Some(2));
        while !gc.step() {}
        assert_eq!(scans.load(Ordering::Relaxed), 2);
        drop(parent);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn a_graph_relinks_if_the_same_strong_flags_now_name_a_different_epoch() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    gc.collect();
    assert!((0..1024).any(|_| {
        gc.step();
        lock(&gc.domains[1].cycle).phase == Phase::Mark
    }));
    LINKED_GRAPH_PROBE.set(Some(|gc| {
        // Isolate epoch identity from the independent registry-skip defence.
        // Replace the actual owning inbox while preserving the same clock flags.
        let queue = &gc.domains[1].mark_queue;
        assert_eq!(queue.queued_sample(), Some(1));
        queue.close_graph_inbox();
        queue.open_graph_inbox(4, false);
        assert_eq!(queue.queued_sample(), Some(0));
    }));
    let scans = Arc::new(AtomicUsize::new(0));
    let root = Gc::try_new(&gc, Scans(scans.clone()))?;
    assert!(LINKED_GRAPH_PROBE.take().is_none());
    assert_eq!(gc.domains[1].mark_queue.queued_sample(), Some(1));
    while !gc.step() {}
    assert_eq!(scans.load(Ordering::Relaxed), 1);
    drop(root);
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn unwinding_a_linked_graph_refunds_the_private_payload_before_the_queue_drains() {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        gc.collect();
        assert!((0..1024).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::Mark
        }));
        OBSERVED_ACTIVATION_PROBE.set(Some(|_, _| panic!("unwind with linked graph ticket")));
        let scans = Arc::new(AtomicUsize::new(0));
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Gc::try_new(&gc, Scans(scans.clone()))
        }));
        assert!(failed.is_err());
        assert!(OBSERVED_ACTIVATION_PROBE.take().is_none());
        assert_eq!(gc.domains[1].mark_queue.queued_sample(), Some(1));
        assert_eq!(gc.managed_bytes(), 0);
        while !gc.step() {}
        assert_eq!(scans.load(Ordering::Relaxed), 0);
        assert_eq!(gc.domains[1].mark_queue.queued_sample(), Some(0));
    }
}

#[test]
fn strong_birth_completes_leaves_and_queues_graphs_once_per_active_domain() -> Result<()> {
    for counted in [false, true] {
        for active in [[true, false], [false, true], [true, true]] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            // Start full first so it never needs to drain an older young cycle.
            // Select each coordinator turn until the requested domain reaches Mark.
            for index in [1, 0].into_iter().filter(|&index| active[index]) {
                if index == 1 {
                    gc.collect();
                } else {
                    gc.collect_young();
                }
                assert!((0..1024).any(|_| {
                    gc.next_domain.store(index, Ordering::Release);
                    gc.coordinated_step(false);
                    lock(&gc.domains[index].cycle).phase == Phase::Mark
                }));
            }
            let queue_depths = || gc.domains.each_ref().map(|d| d.mark_queue.queued_sample());
            let expected_depths = active.map(|tracing| Some(usize::from(tracing)));
            let count = usize::from(active[0]) + usize::from(active[1]);
            let initial = gc.statistics()?.objects_marked;
            assert_eq!(queue_depths(), [Some(0); 2]);
            let leaf = Gc::try_new(&gc, 41_u64)?;
            assert_eq!(gc.statistics()?.objects_marked, initial + count);
            assert_eq!(queue_depths(), [Some(0); 2]);
            let weak = crate::GcWeak::new(&leaf, crate::ReferenceKind::Weak, 0);
            let graph = Gc::try_new(&gc, vec![leaf.into_rooted()])?;
            assert_eq!(gc.statistics()?.objects_marked, initial + count);
            assert_eq!(queue_depths(), expected_depths);
            // Rooting the already-strong graph must not claim a second traversal.
            let alias = graph.clone();
            assert_eq!(queue_depths(), expected_depths);
            while !gc.step() {}
            assert_eq!(gc.statistics()?.objects_marked, initial + 2 * count);
            assert_eq!(weak.upgrade().map(|value| *value), Some(41));
            drop((graph, alias));
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn stale_old_birth_marks_are_removed_before_retrying_in_idle_domains() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .old_collection_interval(usize::MAX)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        gc.collect_old();
        assert!((0..4096).any(|_| {
            gc.step();
            gc.old_collection_active()
        }));
        OBSERVED_ACTIVATION_PROBE.set(Some(|gc, metadata| {
            let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
            assert_ne!(epoch, 0);
            assert_eq!(metadata.old_birth.load(Ordering::Acquire), epoch);
            assert!(metadata.marked(1, epoch));
            // Complete old tracing, then pass both the private parent root and
            // its descriptor in a separate full pass before resuming the birth.
            gc.collect_full()
                .expect("collection can pass private mark preparation");
            while !gc.step() {}
            assert!(!gc.domains.iter().any(Domain::active));
        }));
        let root = Gc::try_new(&gc, 97_u64)?;
        assert!(OBSERVED_ACTIVATION_PROBE.take().is_none());
        let metadata = gc
            .metadata_for_address(root.as_ptr() as usize)
            .expect("live leaf");
        assert_eq!(metadata.old_birth.load(Ordering::Acquire), 0);
        for i in 0..2 {
            assert!(!metadata.marked(i, crate::metadata::EPOCH_STRIDE));
        }
        gc.collect_full()?;
        assert_eq!(**root, 97);
        drop(root);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn graph_observed_in_sweep_acquires_traversal_in_a_new_strong_epoch() -> Result<()> {
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(counted)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let child = Gc::try_new(&gc, 79_u64)?;
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        gc.collect();
        assert!((0..4096).any(|_| {
            gc.step();
            lock(&gc.domains[1].cycle).phase == Phase::Sweep
        }));
        OBSERVED_ACTIVATION_PROBE.set(Some(|gc, metadata| {
            assert!(metadata.marked(1, gc.domains[1].epoch.load(Ordering::Acquire)));
            while !gc.step() {}
            gc.collect();
            assert!((0..4096).any(|_| {
                gc.step();
                lock(&gc.domains[1].cycle).phase == Phase::Mark
            }));
        }));
        let scans = Arc::new(AtomicUsize::new(0));
        let parent = Gc::try_new(&gc, (child.into_rooted(), Scans(scans.clone())))?;
        assert!(OBSERVED_ACTIVATION_PROBE.take().is_none());
        assert!(
            gc.metadata_for_address(parent.as_ptr() as usize)
                .expect("live graph")
                .marked(1, gc.domains[1].epoch.load(Ordering::Acquire))
        );
        assert_eq!(scans.load(Ordering::Relaxed), 0);
        while !gc.step() {}
        assert!(scans.load(Ordering::Relaxed) > 0);
        assert_eq!(weak.upgrade().map(|edge| *edge), Some(79));
        assert_eq!(*parent.0, 79);
        drop(parent);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn descriptor_birth_protects_a_root_installed_after_its_scan() -> Result<()> {
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for kind in [
                CollectionKind::Young,
                CollectionKind::Full,
                CollectionKind::Old,
            ] {
                let domain = usize::from(kind != CollectionKind::Young);
                for phase in [Phase::Mark, Phase::Sweep] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .root_storage(storage)
                            .concurrent_publication(counted)
                            .old_collection_interval(usize::MAX)
                            .promotion_age(1)
                            .incremental_step_size(1)
                            .max_pause_time_us(0)
                            .allocation_threshold(usize::MAX)
                            .build(),
                    );
                    let context = gc.register_context();
                    let child = context.with(|| Gc::try_new(&gc, 41_u64))?;
                    let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
                    match kind {
                        CollectionKind::Young => gc.collect_young(),
                        CollectionKind::Full => gc.collect(),
                        CollectionKind::Old => {
                            // This new young parent will own an already promoted child.
                            gc.collect_full()?;
                            gc.collect_old();
                        }
                    }
                    let reached = (0..1024).any(|_| {
                        gc.step();
                        lock(&gc.domains[domain].cycle).phase == phase
                    });
                    assert!(reached, "{kind:?} cycle did not reach {phase:?}");
                    assert_eq!(gc.old_collection_active(), kind == CollectionKind::Old);
                    let parent = context.with(|| Gc::try_new(&gc, vec![child.into_rooted()]))?;
                    let epoch = gc.domains[domain].epoch.load(Ordering::Acquire);
                    assert!(
                        gc.metadata_for_address(parent.as_ptr() as usize)
                            .is_some_and(|metadata| metadata.marked(domain, epoch))
                    );
                    gc.collect_full()?;
                    assert_eq!(weak.upgrade().map(|value| *value), Some(41));
                    assert_eq!(parent.first().map(|value| **value), Some(41));
                    drop(parent);
                    gc.collect_full()?;
                    assert!(weak.upgrade().is_none());
                    assert_eq!(gc.managed_bytes(), 0);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn idle_address_and_identity_lookups_use_slots_and_weak_upgrade_keeps_its_decision() -> Result<()> {
    thread_local! {
        static CHECKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }
    fn probe(gc: &GarbageCollector) {
        // Observe the actual insertion, without depending on a type query that
        // root entries no longer need. Validation's original admission survives.
        assert_eq!(gc.diagnostics().active_publications, 1);
        CHECKS.set(CHECKS.get() + 1);
    }
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .diagnostics(true)
            .build(),
    );
    let root = Gc::try_new(&gc, 42_u64)?;
    let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
    CHECKS.set(0);
    *lock(&gc.tree_root_insert_probe) = Some(probe);
    let typed = Gc::<u64>::from_handle_rooted(&gc, root.as_handle_i64())?;
    assert_eq!(*typed, 42);
    assert_eq!(gc.root_statistics().slot_roots, 1);
    assert_eq!(CHECKS.get(), 0);
    drop(typed);
    let by_address = Gc::<u64>::from_raw_rooted(&gc, root.as_ptr_i64())?;
    assert_eq!(*by_address, 42);
    assert_eq!(gc.root_statistics().slot_roots, 1);
    assert_eq!(CHECKS.get(), 0);
    drop(by_address);
    assert_eq!(weak.upgrade().map(|root| *root), Some(42));
    assert_eq!(CHECKS.get(), 1);
    assert_eq!(gc.diagnostics().active_publications, 0);
    *lock(&gc.tree_root_insert_probe) = None;
    drop(root);
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.statistics()?.live_objects, 0);
    Ok(())
}

#[test]
fn tree_root_changes_do_not_call_type_capability_hooks_under_admission() -> Result<()> {
    thread_local! {
        static OBSERVED: std::cell::RefCell<Weak<GarbageCollector>> = const { std::cell::RefCell::new(Weak::new()) };
        static QUERIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }
    struct Probe(u64);
    // SAFETY: this fixture contains no outgoing GC references.
    unsafe impl Trace for Probe {
        fn trace(&self, _: &GarbageCollector) {}
        fn may_contain_gc_references() -> bool {
            OBSERVED.with(|observed| {
                if let Some(gc) = observed.borrow().upgrade() {
                    assert!(
                        gc.publication.try_write().is_some(),
                        "type query held legacy admission"
                    );
                    if let Some(clock) = &gc.publication_clock {
                        assert_eq!(clock.active(), 0, "type query held counted admission");
                    }
                    QUERIES.set(QUERIES.get() + 1);
                }
            });
            false
        }
    }
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .root_storage(crate::RootStorage::Tree)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        OBSERVED.set(Arc::downgrade(&gc));
        let root = Gc::try_new(&gc, Probe(42))?;
        let before = QUERIES.get();
        assert!(before != 0, "type query probe never executed");
        let edge: &Gc<Probe> = &root;
        let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
        let owned = edge.rooted_clone();
        let resolved = Gc::<Probe>::from_handle_rooted(&gc, root.as_handle_i64())?;
        let upgraded = weak.upgrade().expect("live weak target");
        let numeric = gc.add_root(&root);
        gc.remove_root_by_id(numeric);
        assert_eq!(owned.0 + resolved.0 + upgraded.0, 126);
        drop((owned, resolved, upgraded));
        assert_eq!(
            QUERIES.get(),
            before,
            "root membership queried its type again"
        );
        OBSERVED.set(Weak::new());
        drop(root);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn admitted_root_rejects_a_foreign_collectors_token() -> Result<()> {
    let gc = GarbageCollector::new();
    let other = GarbageCollector::new();
    let root = Gc::try_new(&gc, 42_u64)?;
    let mut reservation = gc.reserve_owned_root(root.as_ptr() as usize);
    let publication = other.root_publication_guard();
    assert!(
        Gc::<u64>::prepare_conditional_root(
            &gc,
            root.as_ptr_i64(),
            false,
            Some(root.identity()),
            &publication,
            &mut reservation
        )
        .is_err()
    );
    gc.collect_full()?;
    assert_eq!(**root, 42);
    Ok(())
}

#[test]
fn prepared_root_protects_its_graph_after_admission_and_cancels_cleanly() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for finish in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(concurrent)
                        .root_storage(storage)
                        .diagnostics(true)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let context = gc.register_context();
                let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
                let parent = context.with(|| Gc::try_new(&gc, vec![child]))?;
                let weak = crate::GcWeak::new(&parent, crate::ReferenceKind::Weak, 0);
                let prepared = context.with(|| {
                    let mut reservation = gc.reserve_owned_root(parent.as_ptr() as usize);
                    let publication = gc.root_publication_guard();
                    Gc::<Vec<Gc<u64>>>::prepare_conditional_root(
                        &gc,
                        parent.as_ptr_i64(),
                        false,
                        Some(parent.identity()),
                        &publication,
                        &mut reservation,
                    )
                })?;
                drop(parent);
                assert_eq!(gc.diagnostics().active_publications, 0);
                for _ in 0..3 {
                    drop(Gc::try_new(&gc, 7_u64)?);
                    gc.collect_full()?;
                    assert_eq!(gc.statistics()?.live_objects, 2);
                }
                if finish {
                    let parent = prepared.finish();
                    assert_eq!(parent.first().map(|child| **child), Some(42));
                    drop(parent);
                } else {
                    drop(prepared);
                }
                gc.collect_full()?;
                assert!(weak.upgrade().is_none());
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced publication schedule assertions"
)]
fn paused_metadata_preparation_keeps_construction_edges_without_global_admission() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let (entered, observed) = std::sync::mpsc::channel();
        let (resume, resumed) = std::sync::mpsc::channel();
        *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
            entered,
            resume: resumed,
        });
        let worker_gc = gc.clone();
        let worker = thread::spawn(move || Gc::try_new(&worker_gc, vec![child]));
        observed
            .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                120
            } else {
                10
            }))
            .expect("paused metadata preparation");
        assert_eq!(gc.diagnostics().active_publications, 0);
        for _ in 0..3 {
            drop(Gc::try_new(&gc, 7_u64)?);
            gc.collect_full()?;
            assert_eq!(gc.statistics()?.live_objects, 1); // Only the construction child is registered.
        }
        resume.send(()).expect("resume registration");
        let parent = worker.join().expect("constructor worker")?;
        assert_eq!(parent.first().map(|child| **child), Some(42));
        drop(parent);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced publication schedule assertions"
)]
fn waiting_reference_owner_binding_keeps_its_graph_without_holding_full_cycles() -> Result<()> {
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    struct Owner(Arc<crate::GcWeak<Vec<Gc<u64>>>>);
    // SAFETY: the only GC link is conditional; its slot handles tracing and owner binding.
    unsafe impl Trace for Owner {
        fn trace(&self, _: &GarbageCollector) {}
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            let address = owner.address();
            self.0.set_owner(address);
        }
    }

    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .diagnostics(true)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
        let target = Gc::try_new(&gc, vec![child])?;
        let weak = Arc::new(crate::GcWeak::new(
            &target,
            crate::ReferenceKind::Final,
            usize::MAX,
        ));
        let (entered, observed) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
            entered,
            resume: resumed,
        });
        let worker_gc = gc.clone();
        let link = weak.clone();
        let constructor = thread::spawn(move || Gc::try_new(&worker_gc, Owner(link)));
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("attached owner");
        assert_eq!(weak.slot.unpublished_owners.load(Ordering::Acquire), 1);
        let decision = weak.slot.decision();
        drop(target); // only the construction root now protects this conditional graph
        resume.send(()).expect("resume owner publication");
        let deadline = Instant::now() + Duration::from_secs(3);
        while weak.slot.decision_waiters.load(Ordering::Acquire) == 0 && Instant::now() < deadline {
            thread::yield_now();
        }
        let waiting = weak.slot.decision_waiters.load(Ordering::Acquire) != 0;
        let admissions = gc.diagnostics().active_publications;
        let gate_available = gc.publication.try_write().is_some();
        let collector_gc = gc.clone();
        let (send, receive) = mpsc::channel();
        let collector = thread::spawn(move || -> Result<()> {
            for _ in 0..3 {
                drop(Gc::try_new(&collector_gc, 99_u64)?);
                collector_gc.collect_full()?;
                assert_eq!(collector_gc.statistics()?.live_objects, 3);
            }
            send.send(()).expect("full cycle observer");
            Ok(())
        });
        let progressed = receive.recv_timeout(Duration::from_secs(3)).is_ok();
        drop(decision); // unblock both workers even if the progress assertion fails
        let owner = constructor.join().expect("owner constructor")?;
        collector.join().expect("full collector")?;
        assert!(waiting, "constructor did not reach owner binding");
        assert_eq!(admissions, 0, "owner binding retained counted admission");
        assert!(
            gate_available,
            "owner binding retained the publication reader"
        );
        assert!(
            progressed,
            "owner binding stopped unrelated full reclamation"
        );
        assert_eq!(weak.slot.unpublished_owners.load(Ordering::Acquire), 0);
        assert_eq!(weak.slot.owner_identity(), Some(owner.identity()));
        let referent = weak.upgrade().expect("retained conditional graph");
        assert_eq!(referent.first().map(|child| **child), Some(42));
        weak.clear();
        drop((referent, owner, weak));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced identity-resolution schedule"
)]
fn stalled_identity_lookup_allows_collection_and_revalidates_the_address() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    for concurrent in [false, true] {
        for keep_alive in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .diagnostics(true)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let mut root = Some(Gc::try_new(&gc, vec![child])?);
            let handle = root.as_ref().expect("original root").as_handle_i64();
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            *lock(&gc.identity_resolution_pause) = Some(ClaimPause {
                entered,
                resume: resumed,
            });
            let worker_gc = gc.clone();
            let lookup_worker =
                thread::spawn(move || Gc::<Vec<Gc<u64>>>::from_handle_rooted(&worker_gc, handle));
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("resolved address hint");
            let admissions = gc.diagnostics().active_publications;
            let gate_available = gc.publication.try_write().is_some();
            if !keep_alive {
                drop(root.take());
            }
            let worker_gc = gc.clone();
            let (send, receive) = mpsc::channel();
            let collector = thread::spawn(move || -> Result<()> {
                for _ in 0..3 {
                    drop(Gc::try_new(&worker_gc, 99_u64)?);
                    worker_gc.collect_full()?;
                }
                send.send(()).expect("collection observer");
                Ok(())
            });
            let progressed = receive.recv_timeout(Duration::from_secs(3)).is_ok();
            // After retirement, even an allocator reusing the address must not
            // satisfy the saved original identity or graph type.
            let replacement = if progressed {
                Some(Gc::try_new(&gc, 7_u64)?)
            } else {
                None // release a regressed lookup before attempting more publication
            };
            resume.send(()).expect("resume root validation");
            let resolved = lookup_worker.join().expect("identity resolver");
            collector.join().expect("full collector")?;
            assert_eq!(admissions, 0, "identity lookup retained admission");
            assert!(
                gate_available,
                "identity lookup retained the publication reader"
            );
            assert!(progressed, "identity lookup held full cycles open");
            if keep_alive {
                let resolved = resolved?;
                assert_eq!(resolved.as_handle_i64(), handle);
                assert_eq!(resolved.first().map(|child| **child), Some(42));
            } else {
                assert!(
                    resolved.is_err(),
                    "expired address hint resurrected its graph"
                );
            }
            drop((root, replacement));
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn completed_edge_barriers_only_invalidate_observations_when_they_publish() -> Result<()> {
    for buffer_size in [0, 4] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .barrier_buffer_size(buffer_size)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        gc.register_context().with(|| -> Result<()> {
            // The registered parent protects this edge, but the first root unit
            // only queues its traversal; the child's mark is still white.
            let root = Gc::try_new(&gc, 42_u64)?;
            let parent = Gc::try_new(&gc, vec![root.clone_gc()])?.into_rooted();
            let mut edge = root.clone_gc();
            // SAFETY: The owning handle protects demotion; the stale identity is never dereferenced.
            unsafe { edge.unroot() };
            drop(root);
            let root = edge;
            gc.collect();
            gc.step();
            let clock = gc.publication_clock.as_ref().expect("counted policy");
            assert!(gc.domains[1].active());
            let flags = clock.flags(1);
            let before = clock.observe().expect("no publishers");
            gc.write_barrier(&root);
            assert!(!clock.transition(before, 1, flags));
            for _ in 0..16 {
                let before = clock.observe().expect("no publishers");
                gc.write_barrier(&root);
                gc.snapshot_edge(root.as_ptr() as usize);
                // Same flags: tests revision invalidation, not a phase closure.
                // A coalesced buffered record still has its pending reservation.
                assert!(clock.transition(before, 1, flags));
            }
            if buffer_size != 0 {
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
            }
            while !gc.step() {}
            assert_eq!(*root, 42);
            drop((root, parent));
            gc.collect_full()?;
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
            Ok(())
        })?;
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn reclamation_completes_under_repeated_edge_barriers() -> Result<()> {
    repeated_edge_reclamation(false)
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn young_reclamation_completes_under_repeated_old_edge_barriers() -> Result<()> {
    repeated_edge_reclamation(true)
}

#[cfg(not(target_family = "wasm"))]
#[expect(
    clippy::too_many_lines,
    reason = "keep producer acknowledgement, collection deadline and joined cleanup together"
)]
fn repeated_edge_reclamation(young: bool) -> Result<()> {
    use std::time::{Duration, Instant};
    for buffer_size in [0, 4] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(true)
                .independent_phases(true)
                .barrier_buffer_size(buffer_size)
                .metadata_shards(16)
                .promotion_age(1)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = Gc::try_new(&gc, 42_u64)?.into_rooted();
        if young {
            gc.collect_full()?;
        }
        let stop = Arc::new(AtomicBool::new(false));
        let start = Arc::new(std::sync::Barrier::new(5));
        let round = Arc::new(AtomicUsize::new(0));
        let observed: Arc<[AtomicUsize; 4]> =
            Arc::new(std::array::from_fn(|_| AtomicUsize::new(0)));
        let workers: Vec<_> = (0..4)
            .map(|index| {
                let (gc, root, stop, start) =
                    (gc.clone(), root.clone(), stop.clone(), start.clone());
                let (round, observed) = (round.clone(), observed.clone());
                thread::spawn(move || {
                    gc.register_context().with(|| {
                        start.wait();
                        let mut operations = 0;
                        let progress = observed.get(index).expect("producer index");
                        while !stop.load(Ordering::Acquire) {
                            let current = round.load(Ordering::Acquire);
                            gc.write_barrier(&root);
                            progress.store(current, Ordering::Release);
                            operations += 1;
                            if operations % 64 == 0 {
                                thread::yield_now();
                            }
                        }
                        operations
                    })
                })
            })
            .collect();
        // Each full cycle must complete while all producers are still running.
        // Stop/join even on a deadline failure, so a failing test cannot strand them.
        start.wait();
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut completed = 0;
        for cycle in 1..=8 {
            drop(Gc::try_new(&gc, 99_u64)?);
            let ticket = if young {
                gc.collect_young();
                0
            } else {
                gc.request_full_cycle()
            };
            let pending = || {
                if young {
                    gc.pending()
                } else {
                    gc.completed_full_ticket() < ticket
                }
            };
            // Start the snapshot, then require a barrier from every producer
            // before advancing it. A start rendezvous alone does not guarantee
            // each producer runs before a fast collector finishes all cycles.
            let active = || gc.domains[usize::from(!young)].active();
            while !active() && pending() && Instant::now() < deadline {
                gc.step();
                thread::yield_now();
            }
            if !active() {
                break;
            }
            round.store(cycle, Ordering::Release);
            let acknowledged = || observed.iter().all(|n| n.load(Ordering::Acquire) >= cycle);
            while !acknowledged() && Instant::now() < deadline {
                thread::yield_now();
            }
            if !acknowledged() {
                break;
            }
            while pending() && Instant::now() < deadline {
                gc.step();
                thread::yield_now();
            }
            if pending() {
                break;
            }
            completed += 1;
        }
        stop.store(true, Ordering::Release);
        for worker in workers {
            assert!(
                worker
                    .join()
                    .unwrap_or_else(|error| std::panic::resume_unwind(error))
                    > 0
            );
        }
        assert_eq!(
            completed,
            8,
            "reclamation stalled under edge barriers: buffer={buffer_size}, young={young}, phase={:?}, pending={}",
            lock(&gc.domains[1].cycle).phase,
            gc.contexts.pending.load(Ordering::Acquire)
        );
        assert_eq!(*root, 42);
        assert_eq!(gc.statistics()?.live_objects, 1);
        drop(root);
        gc.collect_full()?;
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn completed_edge_cache_preserves_dirty_notifications_and_new_epochs() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .barrier_buffer_size(4)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    // Exercise the buffered fallback even when read-only certification would
    // already prove that this edge needs no work.
    gc.publication_clock
        .as_ref()
        .expect("counted policy")
        .disable_certification();
    let context = gc.register_context();
    context.with(|| -> Result<()> {
        let root = Gc::try_new(&gc, 42_u64)?.into_rooted();
        for _ in 0..2 {
            gc.collect();
            gc.step();
            gc.write_barrier(&root);
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
            context.flush();
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
            for _ in 0..16 {
                gc.write_barrier(&root);
            }
            assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
            for _ in 0..2 {
                gc.remember(&root);
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
                context.flush();
            }
            while !gc.step() {}
            assert_eq!(*root, 42);
        }
        // After promotion, this edge is outside the young snapshot. Completion
        // caching must also suppress those irrelevant old-target notifications.
        gc.collect_young();
        gc.step();
        gc.write_barrier(&root);
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
        context.flush();
        for _ in 0..16 {
            gc.write_barrier(&root);
        }
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
        while !gc.step() {}
        assert_eq!(*root, 42);
        // Simulate imminent epoch exhaustion. The optional completed cache must
        // be disabled before marks are reset and a numeric epoch is reused.
        gc.domains[1].epoch.store(u64::MAX - 3, Ordering::Release);
        gc.collect();
        for _ in 0..1000 {
            gc.step();
            if gc.domains[1].active() {
                break;
            }
        }
        assert!(gc.domains[1].active());
        assert!(gc.contexts.completed_disabled.load(Ordering::Acquire));
        gc.write_barrier(&root);
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
        context.flush();
        gc.write_barrier(&root);
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 1);
        while !gc.step() {}
        drop(root);
        gc.collect_full()?;
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    })
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_paused_read_only_barrier_check_does_not_hold_admission_or_a_shard() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let child = Gc::try_new(&gc, 42_u64)?.into_rooted();
    let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
    let root = Gc::try_new(&gc, vec![child])?;
    let edge = root.clone_gc();
    gc.collect();
    gc.step();
    assert!(gc.domains[1].active());
    let (entered, receive) = std::sync::mpsc::channel();
    let (resume, release) = std::sync::mpsc::channel();
    *lock(&gc.edge_certificate_pause) = Some(ClaimPause {
        entered,
        resume: release,
    });
    let worker = {
        let gc = gc.clone();
        thread::spawn(move || {
            gc.write_barrier(&edge);
        })
    };
    receive
        .recv_timeout(std::time::Duration::from_secs(10))
        .map_err(|error| crate::Error::SyncError(error.to_string()))?;
    assert_eq!(
        gc.publication_clock.as_ref().map(|clock| clock.active()),
        Some(0)
    );
    let collections = (|| -> Result<()> {
        for _ in 0..3 {
            drop(Gc::try_new(&gc, 99_u64)?);
            gc.collect_full()?;
            assert_eq!(weak.upgrade().as_deref(), Some(&42));
        }
        Ok(())
    })();
    resume
        .send(())
        .map_err(|error| crate::Error::SyncError(error.to_string()))?;
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    collections?;
    drop(root);
    gc.collect_full()?;
    gc.stop()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_certified_context_cache_hit_does_not_wait_for_the_object_shard() -> Result<()> {
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .barrier_buffer_size(4)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let context = gc.register_context();
    let root = Gc::try_new(&gc, 42_u64)?.into_rooted();
    gc.collect();
    gc.step();
    context.with(|| gc.write_barrier(&root)); // certify and cache the Strong target
    assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
    let shard = gc.objects.lock(root.as_ptr() as usize);
    let (send, receive) = std::sync::mpsc::channel();
    let worker = {
        let (gc, context, root) = (gc.clone(), context.clone(), root.clone());
        thread::spawn(move || {
            context.with(|| gc.write_barrier(&root));
            send.send(()).expect("cache hit finished");
        })
    };
    let finished = receive.recv_timeout(std::time::Duration::from_secs(10));
    drop(shard);
    worker
        .join()
        .unwrap_or_else(|error| std::panic::resume_unwind(error));
    finished.map_err(|error| crate::Error::SyncError(error.to_string()))?;
    drop(root);
    gc.collect_full()?;
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(clippy::unwrap_in_result, reason = "forced preparation schedule")]
fn a_stalled_root_writer_allows_full_cycles_before_validated_commit() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .diagnostics(true)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let root =
                context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
            drop(Gc::try_new(&gc, 99_u64)?);
            let handle = root.as_handle_i64();
            let state = context
                .with(|| crate::context::current(&gc))
                .expect("context state");
            let roots = if storage == crate::RootStorage::Contexts {
                &state.roots
            } else {
                gc.root_slots.as_deref().expect("global slots")
            };
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            *lock(&roots.prepare_pause) = Some((entered, resumed));
            let worker_gc = gc.clone();
            let other_context = context.clone();
            let constructor = thread::spawn(move || {
                context.with(|| Gc::<Vec<Gc<u64>>>::from_handle_rooted(&worker_gc, handle))
            });
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("writer stalled");
            let admissions = gc
                .publication_clock
                .as_ref()
                .map_or(0, |clock| clock.active());
            let gate_available = gc.publication.try_write().is_some();
            let collector_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let scanner = thread::spawn(move || -> Result<()> {
                // Resolve the very same handle in the same slot registry. The
                // preferred writer is still parked inside its root reservation.
                let other = other_context
                    .with(|| Gc::<Vec<Gc<u64>>>::from_handle_rooted(&collector_gc, handle))?;
                for _ in 0..3 {
                    collector_gc.collect_full()?;
                }
                assert_eq!(other.first().map(|child| **child), Some(42));
                finished
                    .send(collector_gc.statistics()?.live_objects)
                    .expect("cycle observer");
                Ok(())
            });
            let result = completion.recv_timeout(Duration::from_secs(3));
            resume.send(()).expect("resume root writer");
            let resolved = constructor.join().expect("root constructor")?;
            scanner.join().expect("collector")?;
            assert_eq!(admissions, 0);
            assert!(gate_available);
            assert_eq!(result.expect("full progress with root writer stalled"), 2);
            assert_eq!(resolved.first().map(|child| **child), Some(42));
            drop((root, resolved, state));
            gc.stop()?;
            assert_eq!(gc.root_statistics().reserved_slots, 0);
            assert_eq!(gc.root_statistics().resident_pages, 0);
            assert_eq!(gc.statistics()?.live_objects, 0);
        }
    }
    Ok(())
}

#[test]
fn invisible_root_reservations_do_not_resurrect_expired_graphs_and_unwind_cleanly() -> Result<()> {
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            context.with(|| -> Result<()> {
                let root = Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()])?;
                let address = root.as_ptr_i64();
                let identity = root.identity();
                let mut reservation = gc.reserve_owned_root(root.as_ptr() as usize);
                drop(root);
                gc.collect_full()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
                let publication = gc.root_publication_guard();
                assert!(
                    Gc::<Vec<Gc<u64>>>::prepare_conditional_root(
                        &gc,
                        address,
                        false,
                        Some(identity),
                        &publication,
                        &mut reservation
                    )
                    .is_err()
                );
                drop(publication);
                drop(reservation);
                assert_eq!(gc.root_statistics().reserved_slots, 0);
                #[cfg(panic = "unwind")]
                {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let _reservation = gc.reserve_owned_root(0x1000);
                        panic!("cancel before commit");
                    }));
                    assert!(result.is_err());
                }
                assert_eq!(gc.root_statistics().reserved_slots, 0);
                Ok(())
            })?;
            drop(context);
            gc.stop()?;
            assert_eq!(gc.root_statistics().resident_pages, 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced weak root-preparation schedule"
)]
fn weak_upgrade_revalidates_after_collection_during_slot_preparation() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let root = Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()])?;
            let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
            let tree_id = gc.add_root(&root);
            drop(root);
            let state = context
                .with(|| crate::context::current(&gc))
                .expect("context state");
            let roots = if storage == crate::RootStorage::Contexts {
                &state.roots
            } else {
                gc.root_slots.as_deref().expect("global slots")
            };
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            *lock(&roots.prepare_pause) = Some((entered, resumed));
            let upgrader = thread::spawn(move || context.with(|| weak.upgrade()));
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("weak preparation");
            let scanner_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let scanner = thread::spawn(move || -> Result<()> {
                scanner_gc.remove_root_by_id(tree_id);
                for _ in 0..3 {
                    scanner_gc.collect_full()?;
                }
                finished
                    .send(scanner_gc.statistics()?.live_objects)
                    .expect("cycle observer");
                Ok(())
            });
            let progress = completion.recv_timeout(Duration::from_secs(3));
            resume.send(()).expect("resume weak preparation");
            let resolved = upgrader.join().expect("weak upgrader");
            scanner.join().expect("collector")?;
            assert_eq!(progress.expect("collection during weak preparation"), 0);
            assert!(resolved.is_none());
            drop(state);
            gc.stop()?;
            assert_eq!(gc.root_statistics().reserved_slots, 0);
            assert_eq!(gc.root_statistics().resident_pages, 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn root_deletion_and_full_reclamation_finish_while_a_slot_writer_is_stalled() -> Result<()> {
    check_root_deletion_progress(false)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn compatibility_root_deletion_and_reclamation_finish_while_a_slot_writer_is_stalled() -> Result<()>
{
    check_root_deletion_progress(true)
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[expect(clippy::unwrap_in_result, reason = "forced root deletion schedule")]
fn check_root_deletion_progress(compatibility: bool) -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let root =
                context.with(|| Gc::try_new(&gc, vec![Gc::new(&gc, 42_u64).into_rooted()]))?;
            let handle = root.as_handle_i64();
            let state = context
                .with(|| crate::context::current(&gc))
                .expect("context state");
            let roots = if storage == crate::RootStorage::Contexts {
                &state.roots
            } else {
                gc.root_slots.as_deref().expect("global slots")
            };
            let (entered, observed) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            *lock(&roots.prepare_pause) = Some((entered, resumed));
            let publisher_gc = gc.clone();
            let publisher = thread::spawn(move || {
                context.with(|| Gc::<Vec<Gc<u64>>>::from_handle_rooted(&publisher_gc, handle))
            });
            observed
                .recv_timeout(Duration::from_secs(10))
                .expect("root writer paused");
            let (deleted, deletion) = mpsc::channel();
            let remover_gc = gc.clone();
            let remover = thread::spawn(move || {
                if compatibility {
                    // SAFETY: this is the final payload access through the root;
                    // the stopped publisher validates identity before use.
                    unsafe { remover_gc.remove_root(&**root) };
                }
                drop(root);
                deleted.send(()).expect("deletion observer");
            });
            let completed = deletion.recv_timeout(Duration::from_secs(3)).is_ok();
            let scanner_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let scanner = thread::spawn(move || -> Result<()> {
                for _ in 0..3 {
                    scanner_gc.collect_full()?;
                }
                finished
                    .send(scanner_gc.statistics()?.live_objects)
                    .expect("collection observer");
                Ok(())
            });
            let progress = completion.recv_timeout(Duration::from_secs(3));
            resume.send(()).expect("release stalled writer");
            let reconstructed = publisher.join().expect("publisher");
            remover.join().expect("root remover");
            scanner.join().expect("scanner")?;
            assert!(completed, "root removal waited for a slot writer");
            assert_eq!(progress.expect("collection during stalled writer"), 0);
            assert!(reconstructed.is_err(), "expired identity was resurrected");
            drop(state);
            gc.stop()?;
            assert_eq!(gc.root_statistics().slot_roots, 0);
            assert_eq!(gc.root_statistics().reserved_slots, 0);
            assert_eq!(gc.root_statistics().resident_pages, 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    reason = "forced schedule channel failures are test failures"
)]
#[cfg(feature = "experimental-identity-index")]
fn a_stopped_identity_reader_does_not_hold_payloads_or_later_history() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let value = Gc::try_new(&gc, 42_u64)?;
        let identity = value.identity().to_i64().cast_unsigned();
        let (entered, observed) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let reader_gc = gc.clone();
        let reader = thread::spawn(move || {
            let _snapshot = reader_gc.identities.test_snapshot(identity);
            entered.send(()).expect("reader snapshot");
            resume
                .recv_timeout(Duration::from_secs(10))
                .expect("reader release");
        });
        observed
            .recv_timeout(Duration::from_secs(10))
            .expect("snapshot observed");
        drop(value);
        gc.collect_full()?;
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(gc.identities.retained_nodes(), 1);
        for _ in 0..32 {
            drop(Gc::try_new(&gc, 7_u64)?);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
            assert_eq!(gc.identities.retained_nodes(), 1);
        }
        release.send(()).expect("release reader");
        reader.join().expect("reader completed");
        assert_eq!(gc.identities.retained_nodes(), 0);
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn handles_and_collection_finish_while_all_slot_writers_are_parked() {
    use std::sync::mpsc;
    use std::time::Duration;
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let root = context.with(|| Gc::new(&gc, 42_u64));
            let handle = root.as_handle_i64();
            drop(Gc::new(&gc, 99_u64));
            let state = context.with(|| crate::context::current(&gc)).unwrap();
            let roots = if storage == crate::RootStorage::Contexts {
                &state.roots
            } else {
                gc.root_slots.as_deref().unwrap()
            };
            let worker_gc = gc.clone();
            let (finished, completion) = mpsc::channel();
            let (worker, result) = roots.with_all_writers_held(|| {
                let worker = thread::spawn(move || {
                    let other = context
                        .with(|| Gc::<u64>::from_handle_rooted(&worker_gc, handle))
                        .unwrap();
                    drop(root);
                    for _ in 0..3 {
                        worker_gc.collect_full().unwrap();
                    }
                    assert_eq!(worker_gc.statistics().unwrap().live_objects, 1);
                    assert_eq!(*other, 42);
                    drop(other);
                    worker_gc.collect_full().unwrap();
                    finished
                        .send(worker_gc.statistics().unwrap().live_objects)
                        .unwrap();
                });
                let result = completion.recv_timeout(Duration::from_secs(3));
                (worker, result)
            });
            worker.join().unwrap();
            assert_eq!(
                result.expect("handle acquisition or reclamation waited for all slot writers"),
                0
            );
            drop(state);
            gc.stop().unwrap();
        }
    }
}

#[test]
fn all_slot_writers_parked_cooperatively() {
    for concurrent in [false, true] {
        for storage in [crate::RootStorage::Slots, crate::RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let context = gc.register_context();
            let original = context.with(|| Gc::new(&gc, 42_u64));
            let handle = original.as_handle_i64();
            let expired = Gc::new(&gc, 31_u64);
            let expired_handle = expired.as_handle_i64();
            drop(expired);
            gc.collect_full().unwrap();
            let state = context.with(|| crate::context::current(&gc)).unwrap();
            let roots = if storage == crate::RootStorage::Contexts {
                &state.roots
            } else {
                gc.root_slots.as_deref().unwrap()
            };
            let (live, duplicate) = roots.with_all_writers_held(|| {
                let resolve = |id| context.with(|| Gc::<u64>::from_handle_rooted(&gc, id));
                assert!(resolve(expired_handle).is_err());
                let live = resolve(handle).unwrap();
                let duplicate = resolve(handle).unwrap();
                drop(original);
                gc.collect_full().unwrap();
                assert_eq!(*live, 42);
                (live, duplicate)
            });
            let occupancy = roots.statistics();
            assert_eq!(
                (
                    occupancy.slot_roots,
                    occupancy.reserved_slots,
                    occupancy.overflow_records
                ),
                (2, 0, 2)
            );
            roots.with_all_writers_held(|| {
                // SAFETY: this is the last payload access through either revoked root.
                unsafe { gc.remove_root(&*live) };
                gc.collect_full().unwrap();
                assert_eq!(gc.statistics().unwrap().live_objects, 0);
                drop((live, duplicate));
            });
            let occupancy = roots.statistics();
            assert_eq!(
                (
                    occupancy.slot_roots,
                    occupancy.reserved_slots,
                    occupancy.overflow_records
                ),
                (0, 0, 0)
            );
            drop(state);
            gc.stop().unwrap();
        }
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_external_trace_cannot_rewind_a_later_full_epoch() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;
    for counted in [false, true] {
        for rooted in [false, true] {
            for (conservative, same_stage) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .root_storage(crate::RootStorage::Slots)
                        .concurrent_publication(counted)
                        .build(),
                );
                let leaf = Gc::try_new(&gc, 17_u64)?;
                let leaf_meta = gc.objects.lookup(leaf.as_ptr().addr()).unwrap();
                let child = Gc::try_new(&gc, vec![leaf.into_rooted()])?;
                let child_meta = gc.objects.lookup(child.as_ptr().addr()).unwrap();
                let parent = Gc::try_new(&gc, vec![child.into_rooted()])?;
                let extra = rooted.then(|| parent[0].rooted_clone());
                if conservative {
                    gc.phase_clock.disable_certification();
                }
                let first = gc.request_full_cycle();
                assert!((0..1024).any(|_| {
                    gc.step();
                    gc.domains[1].active()
                        && !child_meta.marked(1, gc.domains[1].epoch.load(Ordering::Acquire))
                }));
                let (entered, parked) = mpsc::channel();
                let (resume, receiver) = mpsc::channel();
                thread::scope(|scope| {
                    let edge = extra.as_ref().unwrap_or(&parent[0]);
                    let heap = &gc;
                    let worker = scope.spawn(move || {
                        EXTERNAL_TRACE_EPOCH.set(Some(ClaimPause {
                            entered,
                            resume: receiver,
                        }));
                        edge.trace(heap);
                    });
                    parked
                        .recv_timeout(Duration::from_secs(5))
                        .expect("direct trace sampled E1");
                    assert!((0..8192).any(|_| {
                        gc.step();
                        gc.domains[1].completed_ticket.load(Ordering::Acquire) >= first
                    }));
                    gc.collect();
                    assert!((0..8192).any(|_| {
                        gc.step();
                        if same_stage {
                            gc.domains[1].active() && gc.domains[1].weak_phase() == 0
                        } else {
                            lock(&gc.domains[1].cycle).phase == Phase::Sweep
                        }
                    }));
                    let epoch = gc.domains[1].epoch.load(Ordering::Acquire);
                    if same_stage {
                        assert!(
                            !child_meta.marked(1, epoch),
                            "normal E2 root work is pending"
                        );
                    } else {
                        assert_eq!(child_meta.retention(1, epoch), Some(Retention::Strong));
                    }
                    resume.send(()).unwrap();
                    worker.join().unwrap();
                    if same_stage {
                        assert_eq!(
                            child_meta
                                .direct_marks
                                .get(1, epoch, &gc.domains[1].mark_queue),
                            Some(Retention::Strong),
                            "matching flags must publish to the actual new epoch"
                        );
                        assert!((0..8192).any(|_| gc.step()));
                    }
                    assert_eq!(child_meta.retention(1, epoch), Some(Retention::Strong));
                    assert_eq!(leaf_meta.retention(1, epoch), Some(Retention::Strong));
                    assert_eq!(*parent[0][0], 17);
                });
                drop((extra, parent));
                gc.collect_full()?;
                gc.collect_full()?;
                assert_eq!(gc.statistics()?.bytes_managed, 0);
            }
        }
    }
    Ok(())
}
