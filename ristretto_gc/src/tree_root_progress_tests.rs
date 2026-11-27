use super::*;

thread_local! {
    static EMPTY_ROOT_PAGE: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const {
        std::cell::RefCell::new(None)
    };
}

pub(crate) fn probe_empty_root_page() {
    if let Some(probe) = EMPTY_ROOT_PAGE.take() {
        probe();
    }
}

#[test]
fn an_older_empty_tree_page_allows_a_full_collection_during_unlink() {
    use crate::{GcWeak, ReferenceKind};
    let mut completed = Vec::new();
    for counted in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .allocation_threshold(usize::MAX)
                .concurrent_publication(counted)
                .build(),
        );
        let removed = Gc::try_new(&gc, 11_u64).unwrap();
        let live = Gc::try_new(&gc, 22_u64).unwrap();
        let witness = GcWeak::new(&removed, ReferenceKind::Weak, 0);
        gc.next_root_id.store(32, Ordering::Relaxed);
        let older = gc.add_root(&removed);
        gc.next_root_id.store(64, Ordering::Relaxed);
        let recent = gc.add_root(&live);
        drop(removed);
        let observed = std::rc::Rc::new(std::cell::Cell::new(None));
        let result = observed.clone();
        let replacement = std::rc::Rc::new(std::cell::RefCell::new(None));
        let published = replacement.clone();
        let peer = gc.clone();
        EMPTY_ROOT_PAGE.set(Some(Box::new(move || {
            let ticket = peer.request_full_cycle();
            for _ in 0..2048 {
                if peer.completed_full_ticket() >= ticket {
                    break;
                }
                peer.step();
            }
            let finished = peer.completed_full_ticket() >= ticket;
            result.set(Some((finished, witness.is_cleared())));
            if finished {
                // Reinstall the same numeric page while the unlinker still holds
                // its old directory view.
                let child = Gc::try_new(&peer, 33_u64).unwrap();
                let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
                peer.next_root_id.store(older, Ordering::Relaxed);
                assert_eq!(peer.add_root(&child), older);
                published.replace(Some(weak));
            }
        })));
        gc.remove_root_by_id(older);
        assert!(
            EMPTY_ROOT_PAGE.take().is_none(),
            "older page was never unlinked"
        );
        completed.push((counted, observed.get()));
        if let Some(weak) = replacement.take() {
            gc.collect_full().unwrap();
            assert_eq!(*weak.upgrade().unwrap(), 33);
            gc.remove_root_by_id(older);
            gc.collect_full().unwrap();
            assert!(weak.is_cleared());
        }
        assert_eq!(**live, 22);
        gc.remove_root_by_id(recent);
        drop(live);
        gc.collect_full().unwrap();
        assert_eq!(gc.managed_bytes(), 0);
    }
    assert_eq!(
        completed,
        [(false, Some((true, true))), (true, Some((true, true)))]
    );
}

#[test]
fn a_parked_tree_read_view_allows_unrelated_payload_reclamation() {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(concurrent)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .build(),
        );
        let live = Gc::try_new(&gc, 11_u64).unwrap();
        let live_bytes = gc.managed_bytes();
        drop(Gc::try_new(&gc, 22_u64).unwrap());
        assert!(gc.managed_bytes() > live_bytes);
        let view = gc.roots.read_view();
        gc.collect();
        for _ in 0..4096 {
            gc.step();
            if gc.managed_bytes() == live_bytes {
                break;
            }
        }
        let retained = gc.managed_bytes();
        drop(view);
        assert_eq!(
            retained, live_bytes,
            "parked root view prevented unrelated reclamation"
        );
        assert_eq!(**live, 11);
        drop(live);
        gc.collect_full().unwrap();
        gc.stop().unwrap();
        assert_eq!(gc.managed_bytes(), 0);
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "assert the progress and identity contract"
)]
fn explicit_tree_root_preparation_allows_a_fresh_full_collection() -> Result<()> {
    fn collect_during_reservation(gc: &GarbageCollector) {
        lock(&gc.tree_root_insert_probe).take();
        let ticket = gc.request_full_cycle();
        for _ in 0..2048 {
            if gc.completed_full_ticket() >= ticket {
                return;
            }
            gc.step();
        }
        assert!(
            gc.completed_full_ticket() >= ticket,
            "full ticket stalled during explicit root preparation"
        );
    }

    for counted in [false, true] {
        for manual in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let target = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let weak = crate::GcWeak::new(&target, crate::ReferenceKind::Weak, 0);
            let original = target.identity();
            *lock(&gc.tree_root_insert_probe) = Some(collect_during_reservation);
            if manual {
                let registration = gc.add_root(&target);
                drop(target);
                gc.collect_full()?;
                assert_eq!(
                    weak.upgrade().map(|object| (object.identity(), *object)),
                    Some((original, 42))
                );
                gc.remove_root_by_id(registration);
            } else {
                let guard = target.as_root(&gc)?;
                drop(target);
                gc.collect_full()?;
                assert_eq!((guard.identity(), **guard), (original, 42));
                assert!(weak.refers_to(&guard));
                drop(guard);
            }
            assert!(lock(&gc.tree_root_insert_probe).is_none());
            gc.collect_full()?;
            assert!(weak.is_cleared());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "assert each forced schedule after releasing the producer"
)]
fn parked_manual_root_revalidates_the_borrowed_identity_after_a_peer_collection() -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    let mut observations = Vec::new();
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            for retain in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let root = Gc::try_new(&gc, 17_u64)?.into_rooted();
                let expected = root.identity();
                let weak = crate::GcWeak::new(&root, crate::ReferenceKind::Weak, 0);
                let mut borrowed = root.clone();
                // SAFETY: the owning root protects this conversion. Only checked
                // identity registration is attempted after it may be released.
                unsafe { borrowed.unroot() };
                let mut root = Some(root);
                let (entered, parked) = mpsc::channel();
                let (resume, release) = mpsc::channel();
                *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
                    entered,
                    resume: release,
                });
                *lock(&gc.tree_root_insert_probe) =
                    Some(GarbageCollector::pause_metadata_preparation);
                let (finished, registration) = thread::scope(|scope| {
                    let request = scope.spawn(|| {
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            gc.add_root(&borrowed)
                        }))
                    });
                    parked
                        .recv_timeout(Duration::from_secs(4))
                        .expect("manual root backing preparation");
                    if !retain {
                        drop(root.take());
                    }
                    let ticket = gc.request_full_cycle();
                    for _ in 0..2048 {
                        if gc.completed_full_ticket() >= ticket {
                            break;
                        }
                        gc.step();
                    }
                    let finished = gc.completed_full_ticket() >= ticket;
                    resume.send(()).unwrap();
                    (finished, request.join().unwrap().ok())
                });
                observations.push((counted, storage, retain, finished, registration.is_some()));
                drop(root);
                gc.collect_full()?;
                if let Some(id) = registration {
                    if retain {
                        assert_eq!(
                            weak.upgrade().map(|r| (r.identity(), *r)),
                            Some((expected, 17))
                        );
                    }
                    gc.remove_root_by_id(id);
                }
                gc.collect_full()?;
                assert!(weak.is_cleared());
                assert_eq!(gc.root_statistics().compatibility_roots, 0);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
    assert!(
        observations
            .iter()
            .all(|(_, _, retain, finished, registered)| *finished && retain == registered),
        "(counted, storage, retained, ticket complete, registered): {observations:?}"
    );
    Ok(())
}

#[test]
fn conditional_owner_reservation_permits_a_fresh_full_ticket() {
    use crate::{GcWeak, ReferenceKind};

    fn collect(gc: &GarbageCollector) {
        lock(&gc.tree_root_insert_probe).take();
        let ticket = gc.request_full_cycle();
        for _ in 0..2048 {
            if gc.completed_full_ticket() >= ticket {
                return;
            }
            gc.step();
        }
        assert!(
            gc.completed_full_ticket() >= ticket,
            "full ticket stalled while preparing the referent root"
        );
    }

    struct Owner {
        donor: Option<Gc<u64>>,
        slot: Option<GcWeak<u64>>,
    }
    // SAFETY: any donor independently owns its root until attach consumes it.
    // The registered object then contains only the conditional slot.
    unsafe impl Trace for Owner {
        fn trace(&self, _: &GarbageCollector) {}
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            let gc = owner.collector().upgrade().unwrap();
            *lock(&gc.tree_root_insert_probe) = Some(collect);
            if let Some(donor) = self.donor.take() {
                self.slot = Some(GcWeak::new(&donor, ReferenceKind::Weak, owner.address()));
            } else {
                self.slot.as_ref().unwrap().set_owner(owner.address());
            }
            assert!(lock(&gc.tree_root_insert_probe).is_none());
        }
    }
    for counted in [false, true] {
        for storage in [
            crate::RootStorage::Tree,
            crate::RootStorage::Slots,
            crate::RootStorage::Contexts,
        ] {
            // New private slots have a donor. A pre-existing slot may lose its
            // last donor before the full collection inside reservation.
            for (new, retain) in [(true, true), (false, true), (false, false)] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .concurrent_publication(counted)
                        .root_storage(storage)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let value = Gc::try_new(&gc, 73_u64).unwrap().into_rooted();
                let original = value.identity();
                let payload = Owner {
                    donor: new.then(|| value.clone()),
                    slot: (!new).then(|| GcWeak::new(&value, ReferenceKind::Weak, 0)),
                };
                let donor = retain.then_some(value);
                let owner = Gc::try_new(&gc, payload).unwrap();
                let slot = owner.slot.as_ref().unwrap();
                assert_eq!(
                    slot.upgrade().map(|v| (v.identity(), *v)),
                    retain.then_some((original, 73))
                );
                drop(donor);
                gc.collect_full().unwrap();
                assert!(slot.is_cleared());
                assert_eq!(
                    gc.take_pending_reference::<Owner>()
                        .unwrap()
                        .map(|v| v.identity()),
                    retain.then_some(owner.identity())
                );
                assert!(gc.take_pending_reference::<Owner>().unwrap().is_none());
                drop(owner);
                gc.collect_full().unwrap();
                assert_eq!(gc.root_statistics().compatibility_roots, 0);
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
    }
}

#[test]
fn removed_construction_root_protects_the_exact_graph_through_handoff() {
    use crate::{GcWeak, ReferenceKind, RootStorage};

    fn require_ticket(gc: &GarbageCollector) {
        lock(&gc.tree_root_insert_probe).take();
        let ticket = gc.request_full_cycle();
        for _ in 0..2048 {
            if gc.completed_full_ticket() >= ticket {
                return;
            }
            gc.step();
        }
        assert!(
            gc.completed_full_ticket() >= ticket,
            "full ticket stalled during removal preparation"
        );
    }
    struct Owner {
        manual: usize,
        graph: GcWeak<Vec<Gc<u64>>>,
        original: crate::ObjectId,
    }
    // SAFETY: the slot is a host weak witness. The numeric ID grants no payload access.
    unsafe impl Trace for Owner {
        fn trace(&self, _: &GarbageCollector) {}
        unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
            let gc = owner.collector().upgrade().unwrap();
            *lock(&gc.tree_root_insert_probe) = Some(require_ticket);
            gc.remove_root_by_id(self.manual);
            assert!(lock(&gc.tree_root_insert_probe).is_none());
            gc.collect_full().unwrap();
            let graph = self.graph.upgrade().unwrap();
            assert_eq!(graph.identity(), self.original);
            assert_eq!(*graph[0], 41);
        }
    }
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .root_storage(storage)
                    .concurrent_publication(counted)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 41_u64).unwrap();
            let weak_child = GcWeak::new(&child, ReferenceKind::Weak, 0);
            let graph = Gc::try_new(&gc, vec![child.into_rooted()]).unwrap();
            let owner = Owner {
                manual: gc.add_root(&graph),
                graph: GcWeak::new(&graph, ReferenceKind::Weak, 0),
                original: graph.identity(),
            };
            drop(graph);
            drop(Gc::try_new(&gc, owner).unwrap());
            gc.collect_full().unwrap();
            assert!(weak_child.is_cleared());
            assert_eq!(gc.root_statistics().compatibility_roots, 0);
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn parked_construction_removal_cannot_capture_a_reused_generation() {
    use crate::{GcWeak, ReferenceKind, RootStorage};
    use std::sync::mpsc;
    use std::time::Duration;

    // The collector probes without borrowing the TLS capture or holding admission.
    fn park(gc: &GarbageCollector) {
        lock(&gc.tree_root_insert_probe).take();
        gc.pause_metadata_preparation();
    }
    fn full_ticket(gc: &GarbageCollector) -> bool {
        let ticket = gc.request_full_cycle();
        for _ in 0..2048 {
            if gc.completed_full_ticket() >= ticket {
                return true;
            }
            gc.step();
        }
        gc.completed_full_ticket() >= ticket
    }
    for counted in [false, true] {
        for storage in [RootStorage::Tree, RootStorage::Slots, RootStorage::Contexts] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .root_storage(storage)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let child = Gc::try_new(&gc, 41_u64).unwrap();
            let original = GcWeak::new(&child, ReferenceKind::Weak, 0);
            let graph = Gc::try_new(&gc, vec![child.into_rooted()]).unwrap();
            let manual = gc.add_root(&graph);
            let exact = gc
                .roots
                .matching(graph.as_ptr().addr())
                .into_iter()
                .find(|key| key.id == manual)
                .unwrap();
            let replacement = Gc::try_new(&gc, 73_u64).unwrap();
            let expected = replacement.identity();
            let witness = GcWeak::new(&replacement, ReferenceKind::Weak, 0);
            drop(graph);
            let (entered, parked) = mpsc::channel();
            let (resume, release) = mpsc::channel();
            *lock(&gc.metadata_prepare_pause) = Some(ClaimPause {
                entered,
                resume: release,
            });
            *lock(&gc.tree_root_insert_probe) = Some(park);
            thread::scope(|s| {
                let worker = s.spawn(|| {
                    let mut scope = crate::construction::Roots::new(&gc);
                    let _active = scope.enter();
                    gc.remove_tree_root(exact);
                });
                parked.recv_timeout(Duration::from_secs(3)).unwrap();
                let progressed = full_ticket(&gc);
                if progressed {
                    assert!(!original.is_cleared());
                    gc.remove_root_by_id(manual);
                    // Force the real allocator's wrap/reuse case while the exact token is parked.
                    gc.next_root_id.store(manual, Ordering::Relaxed);
                    assert_eq!(gc.add_root(&replacement), manual);
                    drop(replacement);
                    assert!(full_ticket(&gc));
                    assert!(original.is_cleared());
                }
                resume.send(()).unwrap();
                worker.join().unwrap();
                assert!(
                    progressed,
                    "first full ticket stalled during retention preparation"
                );
            });
            gc.collect_full().unwrap();
            assert_eq!(
                witness.upgrade().map(|v| (v.identity(), *v)),
                Some((expected, 73))
            );
            gc.remove_root_by_id(manual);
            gc.collect_full().unwrap();
            assert!(witness.is_cleared());
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
}
