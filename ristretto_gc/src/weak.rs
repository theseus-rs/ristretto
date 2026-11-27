//! Conditional heap edges with publication admission and per-slot decisions.
use crate::{GarbageCollector, Gc, Trace};
use portable_atomic::AtomicU64;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

/// Reachability strength of a Java reference's referent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    /// Soft references use an eager clearing policy.
    Soft,
    /// A weak edge does not keep the referent alive.
    Weak,
    /// A final reference retains its referent until the finalizer clears it.
    Final,
    /// Phantom processing follows final-reference tracing.
    Phantom,
    /// Java 8 phantom references retain the referent until explicitly cleared.
    PhantomRetained,
}

#[derive(Debug)]
pub(crate) struct WeakSlot {
    visibility: Option<crate::metadata::BirthVisibility>,
    decision: Mutex<()>,
    #[cfg(test)]
    pub(crate) decision_waiters: AtomicUsize,
    pub(crate) unpublished_owners: AtomicUsize,
    pub(crate) target: AtomicUsize,
    // IDs are never recycled, including across heaps. Conditional edges retain
    // no metadata allocation after their target has been reclaimed.
    pub(crate) target_identity: crate::ObjectId,
    pub(crate) owner: AtomicUsize,
    // Zero denotes an unbound/absent owner, never an address-only match.
    owner_identity: AtomicU64,
    pub(crate) kind: ReferenceKind,
    pub(crate) activated: AtomicBool,
    pub(crate) pending: AtomicBool,
    // A newly created reference cannot participate in a boundary that preceded
    // its creation. Clone preserves the source's decision instead of bypassing it.
    pub(crate) born_after: [AtomicU64; 2],
}

impl WeakSlot {
    pub(crate) fn visible(&self) -> bool {
        self.visibility
            .as_ref()
            .is_none_or(crate::metadata::BirthVisibility::visible)
    }

    /// Never sleep on a slot while excluding a collector phase boundary. Keep
    /// the ordinary admission-before-slot order: after contention, release both
    /// and retry instead of holding the slot while waiting for the legacy gate.
    #[inline]
    fn admitted_decision<'a>(
        &'a self,
        gc: &'a GarbageCollector,
    ) -> (crate::collector::RootPublication<'a>, MutexGuard<'a, ()>) {
        let publication = gc.root_publication_guard();
        if let Some(decision) = self.try_decision() {
            return (publication, decision);
        }
        drop(publication);
        self.wait_admitted_decision(gc)
    }

    #[cold]
    #[inline(never)]
    fn wait_admitted_decision<'a>(
        &'a self,
        gc: &'a GarbageCollector,
    ) -> (crate::collector::RootPublication<'a>, MutexGuard<'a, ()>) {
        loop {
            drop(self.decision());
            let publication = gc.root_publication_guard();
            if let Some(decision) = self.try_decision() {
                return (publication, decision);
            }
            drop(publication);
        }
    }

    pub(crate) fn owner_identity(&self) -> Option<crate::ObjectId> {
        let identity = self.owner_identity.load(Ordering::Acquire);
        (identity != 0).then(|| crate::ObjectId::from_i64(identity.cast_signed()))
    }

    /// Caller holds this slot's decision lock or has not published the slot yet.
    pub(crate) fn bind_owner(&self, identity: Option<crate::ObjectId>) {
        self.owner_identity.store(
            identity.map_or(0, |id| id.to_i64().cast_unsigned()),
            Ordering::Release,
        );
    }

    pub(crate) fn decision(&self) -> MutexGuard<'_, ()> {
        #[cfg(test)]
        self.decision_waiters.fetch_add(1, Ordering::Release);
        let decision = self
            .decision
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        #[cfg(test)]
        self.decision_waiters.fetch_sub(1, Ordering::Release);
        decision
    }

    pub(crate) fn try_decision(&self) -> Option<MutexGuard<'_, ()>> {
        match self.decision.try_lock() {
            Ok(guard) => Some(guard),
            Err(std::sync::TryLockError::Poisoned(error)) => Some(error.into_inner()),
            Err(std::sync::TryLockError::WouldBlock) => None,
        }
    }

    pub(crate) fn birth_stamp(&self, index: usize) -> &AtomicU64 {
        if index == 0 {
            &self.born_after[0]
        } else {
            &self.born_after[1]
        }
    }
}

/// A conditional edge that can be upgraded to a separately rooted handle.
#[derive(Debug)]
pub struct GcWeak<T> {
    pub(crate) collector: Weak<GarbageCollector>,
    pub(crate) slot: Arc<WeakSlot>,
    marker: PhantomData<T>,
}

impl<T: Trace + 'static> GcWeak<T> {
    /// Makes a weak edge. `owner` is the managed Reference object, or zero for a host weak handle.
    #[must_use]
    pub fn new(target: &Gc<T>, kind: ReferenceKind, owner: usize) -> Self {
        let collector = target.collector();
        let gc = collector.upgrade();
        let visibility = gc
            .as_ref()
            .is_some_and(|gc| gc.can_publish_host_weak(kind, owner))
            .then(crate::metadata::BirthVisibility::default);
        let slot = Arc::new(WeakSlot {
            visibility,
            decision: Mutex::new(()),
            #[cfg(test)]
            decision_waiters: AtomicUsize::new(0),
            unpublished_owners: AtomicUsize::new(0),
            target: AtomicUsize::new(target.as_ptr() as usize),
            target_identity: target.identity(),
            owner: AtomicUsize::new(owner),
            owner_identity: AtomicU64::new(0),
            kind,
            activated: AtomicBool::new(false),
            pending: AtomicBool::new(false),
            born_after: std::array::from_fn(|_| AtomicU64::new(0)),
        });
        if let Some(gc) = &gc {
            if let Some(visibility) = &slot.visibility {
                gc.publish_host_weak(&slot, visibility, None);
            } else {
                let mut reservation = if owner == 0 {
                    None
                } else {
                    crate::construction::prepare_reference_owner(gc, &slot)
                };
                let publication = gc.root_publication_guard();
                for (stamp, epoch) in slot.born_after.iter().zip(gc.weak_birth_epochs()) {
                    stamp.store(epoch, Ordering::Relaxed);
                }
                slot.bind_owner(gc.identity_for_address(owner));
                crate::construction::protect_reference_owner(gc, &slot, reservation.as_mut());
                publication.register_weak(&slot);
            }
        }
        Self {
            collector,
            slot,
            marker: PhantomData,
        }
    }

    /// Tests the allocation identity without rooting or tracing the referent.
    #[must_use]
    pub fn refers_to(&self, target: &Gc<T>) -> bool {
        self.slot.target_identity == target.identity() && !self.is_cleared()
    }

    /// Tests whether this edge is explicitly or logically cleared, without upgrading it.
    #[must_use]
    pub fn is_cleared(&self) -> bool {
        let Some(gc) = self.collector.upgrade() else {
            return true;
        };
        if self.slot.target.load(Ordering::Acquire) == 0 {
            return true;
        }
        // Observe the same phase and slot decision as an upgrade. No root,
        // publication record, or tracing work is created by this observation.
        let (_publication, _decision) = self.slot.admitted_decision(&gc);
        let address = self.slot.target.load(Ordering::Acquire);
        gc.weak_is_cleared(&self.slot, address)
    }

    /// Acquires a strong root if the referent has not been cleared.
    #[must_use]
    pub fn upgrade(&self) -> Option<Gc<T>> {
        let gc = self.collector.upgrade()?;
        let hint = self.slot.target.load(Ordering::Acquire);
        if hint == 0 {
            return None;
        }
        let mut reservation = gc.prepare_root_registration(hint);
        let (publication, decision) = self.slot.admitted_decision(&gc);
        let address = self.slot.target.load(Ordering::Acquire);
        if address == 0 || gc.weak_is_cleared(&self.slot, address) {
            return None;
        }
        let retaining = matches!(
            self.slot.kind,
            ReferenceKind::Final | ReferenceKind::PhantomRetained
        );
        let prepared = Gc::prepare_conditional_root(
            &gc,
            i64::try_from(address).ok()?,
            retaining,
            Some(self.slot.target_identity),
            &publication,
            &mut reservation,
        )
        .ok()?;
        drop(decision);
        drop(publication);
        Some(prepared.finish())
    }

    /// Clears this edge without enqueueing it.
    pub fn clear(&self) {
        self.slot.target.store(0, Ordering::Release);
    }

    /// Associates this edge with its containing Reference allocation.
    pub fn set_owner(&self, owner: usize) {
        if let Some(gc) = self.collector.upgrade() {
            let mut reservation = crate::construction::prepare_reference_owner(&gc, &self.slot);
            let (_publication, _decision) = self.slot.admitted_decision(&gc);
            crate::construction::protect_reference_owner(&gc, &self.slot, reservation.as_mut());
            self.slot.owner.store(owner, Ordering::Release);
            self.slot.bind_owner(gc.identity_for_address(owner));
            return;
        }
        let _decision = self.slot.decision();
        self.slot.owner.store(owner, Ordering::Release);
        self.slot.bind_owner(None);
    }
}

impl<T> Clone for GcWeak<T> {
    fn clone(&self) -> Self {
        // Reserve independent slot storage before admission or the source slot's
        // decision lock. No target/birth decision is copied until both are held.
        let gc = self.collector.upgrade();
        let visibility = gc
            .as_ref()
            .is_some_and(|gc| {
                gc.can_publish_host_weak(self.slot.kind, self.slot.owner.load(Ordering::Acquire))
            })
            .then(crate::metadata::BirthVisibility::default);
        let slot = Arc::new(WeakSlot {
            decision: Mutex::new(()),
            #[cfg(test)]
            decision_waiters: AtomicUsize::new(0),
            unpublished_owners: AtomicUsize::new(0),
            target: AtomicUsize::new(if visibility.is_some() {
                self.slot.target.load(Ordering::Acquire)
            } else {
                0
            }),
            visibility,
            target_identity: self.slot.target_identity,
            owner: AtomicUsize::new(0),
            owner_identity: AtomicU64::new(0),
            kind: self.slot.kind,
            activated: AtomicBool::new(false),
            pending: AtomicBool::new(false),
            born_after: std::array::from_fn(|_| AtomicU64::new(0)),
        });
        if let Some(gc) = gc.as_ref()
            && let Some(visibility) = &slot.visibility
        {
            gc.publish_host_weak(&slot, visibility, Some(&self.slot));
        } else if let Some(gc) = gc.as_ref() {
            let (publication, _decision) = self.slot.admitted_decision(gc);
            let address = self.slot.target.load(Ordering::Acquire);
            slot.target.store(
                if gc.weak_is_cleared(&self.slot, address) {
                    0
                } else {
                    address
                },
                Ordering::Relaxed,
            );
            for ((destination, source), epoch) in slot
                .born_after
                .iter()
                .zip(&self.slot.born_after)
                .zip(gc.weak_birth_epochs())
            {
                if source.load(Ordering::Acquire) == epoch {
                    destination.store(epoch, Ordering::Relaxed);
                }
            }
            publication.register_weak(&slot);
        }
        Self {
            collector: self.collector.clone(),
            slot,
            marker: PhantomData,
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "tests assert collector invariants"
)]
mod tests {
    use super::*;

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn a_waiter_on_an_old_weak_slot_does_not_stop_young_reclamation() -> crate::Result<()> {
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        for concurrent in [false, true] {
            for operation in 0..3 {
                let gc = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(concurrent)
                        .diagnostics(true)
                        .promotion_age(1)
                        .full_collection_interval(usize::MAX)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .build(),
                );
                let target = Gc::try_new(&gc, 42_u64)?;
                gc.collect_full()?;
                // This slot is outside the young reference index. Only an
                // unnecessary global admission could make its waiter stop young GC.
                let weak = GcWeak::new(&target, ReferenceKind::Weak, target.as_ptr().addr());
                drop(Gc::try_new(&gc, 99_u64)?);
                let before = gc.statistics()?.young_collections;
                let decision = weak.slot.decision();
                let (waiting, admissions, completed) = std::thread::scope(|scope| {
                    let waiter = scope.spawn(|| match operation {
                        0 => assert_eq!(weak.upgrade().as_deref(), Some(&42)),
                        1 => assert_eq!(weak.clone().upgrade().as_deref(), Some(&42)),
                        _ => weak.set_owner(0),
                    });
                    let deadline = Instant::now() + Duration::from_secs(3);
                    while weak.slot.decision_waiters.load(Ordering::Acquire) == 0
                        && Instant::now() < deadline
                    {
                        std::thread::yield_now();
                    }
                    let waiting = weak.slot.decision_waiters.load(Ordering::Acquire) != 0;
                    let admissions = gc.diagnostics().active_publications;
                    let (send, receive) = mpsc::channel();
                    let collector_gc = &gc;
                    let collector = scope.spawn(move || {
                        let gc = collector_gc;
                        gc.collect_young();
                        for _ in 0..10_000 {
                            gc.step();
                            if gc.statistics().expect("statistics").young_collections > before {
                                send.send(true).expect("progress observer");
                                return;
                            }
                        }
                        send.send(false).expect("progress observer");
                    });
                    let completed = receive
                        .recv_timeout(Duration::from_secs(3))
                        .unwrap_or(false);
                    // Always release before joining, including a regression where
                    // the collector is blocked on the legacy publication writer.
                    drop(decision);
                    waiter.join().expect("weak waiter");
                    collector.join().expect("young collector");
                    (waiting, admissions, completed)
                });
                assert!(waiting, "weak operation never reached the held slot");
                assert_eq!(admissions, 0, "slot wait retained publication admission");
                assert!(
                    completed,
                    "held old slot blocked unrelated young collection"
                );
                assert_eq!(gc.statistics()?.live_objects, 1);
                assert_eq!(weak.upgrade().as_deref(), Some(&42));
                drop((weak, target));
                gc.stop()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
            }
        }
        Ok(())
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn a_weak_waiter_rechecks_a_boundary_crossed_while_waiting() -> crate::Result<()> {
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        for concurrent in [false, true] {
            for clone in [false, true] {
                let gc = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(concurrent)
                        .allocation_threshold(usize::MAX)
                        .incremental_step_size(1)
                        .max_pause_time_us(0)
                        .build(),
                );
                let target = Gc::try_new(&gc, 42_u64)?;
                let weak = GcWeak::new(&target, ReferenceKind::Weak, target.as_ptr().addr());
                drop(target);
                let decision = weak.slot.decision();
                let (waiting, crossed, cleared) = std::thread::scope(|scope| {
                    let waiter = scope.spawn(|| {
                        if clone {
                            weak.clone().upgrade().is_none()
                        } else {
                            weak.upgrade().is_none()
                        }
                    });
                    let deadline = Instant::now() + Duration::from_secs(3);
                    while weak.slot.decision_waiters.load(Ordering::Acquire) == 0
                        && Instant::now() < deadline
                    {
                        std::thread::yield_now();
                    }
                    let waiting = weak.slot.decision_waiters.load(Ordering::Acquire) != 0;
                    let (send, receive) = mpsc::channel();
                    let collector_gc = &gc;
                    let collector = scope.spawn(move || {
                        collector_gc.collect();
                        for _ in 0..10_000 {
                            collector_gc.step();
                            if collector_gc.weak_birth_epochs() != [0; 2] {
                                send.send(true).expect("boundary observer");
                                return;
                            }
                        }
                        send.send(false).expect("boundary observer");
                    });
                    let crossed = receive
                        .recv_timeout(Duration::from_secs(3))
                        .unwrap_or(false);
                    drop(decision);
                    let cleared = waiter.join().expect("weak waiter");
                    collector.join().expect("boundary collector");
                    (waiting, crossed, cleared)
                });
                assert!(waiting, "weak operation never reached the held slot");
                assert!(crossed, "weak waiter excluded the reference boundary");
                assert!(cleared, "weak operation revived a logically cleared target");
                drop(weak);
                gc.stop()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
            }
        }
        Ok(())
    }

    #[test]
    fn an_expired_reference_owner_cannot_enqueue_a_replacement() -> crate::Result<()> {
        for concurrent in [false, true] {
            for kind in [
                ReferenceKind::Soft,
                ReferenceKind::Weak,
                ReferenceKind::Final,
                ReferenceKind::Phantom,
                ReferenceKind::PhantomRetained,
            ] {
                let gc = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(concurrent)
                        .metadata_shards(16)
                        .build(),
                );
                let target = Gc::try_new(&gc, 42_u64)?;
                let owner = Gc::try_new(&gc, 7_u64)?;
                let weak = GcWeak::new(&target, kind, owner.as_ptr() as usize);
                drop(owner);
                gc.collect_full()?;
                assert!(!gc.has_pending_references());
                let replacement = Gc::try_new(&gc, 99_u64)?;
                // Model reuse while preserving the original owner's identity.
                weak.slot
                    .owner
                    .store(replacement.as_ptr() as usize, Ordering::Release);
                drop(target);
                gc.collect_full()?;
                assert!(!gc.has_pending_references(), "enqueued a replacement owner");
                assert!(weak.upgrade().is_none(), "dead owner retained its referent");
                drop(replacement);
                gc.stop()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
            }
        }
        Ok(())
    }

    #[test]
    #[expect(
        clippy::unwrap_in_result,
        reason = "assert expected constructor/queue outcomes"
    )]
    fn attachment_binds_only_successfully_published_reference_owners() -> crate::Result<()> {
        struct Owner {
            link: Arc<GcWeak<u64>>,
            fail: bool,
        }
        // SAFETY: the only GC link is conditional; its slot handles tracing and owner binding.
        unsafe impl Trace for Owner {
            fn trace(&self, _: &GarbageCollector) {}
            unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
                let address = owner.address();
                self.link.set_owner(address);
                assert!(!self.fail, "attachment failed");
            }
        }
        // WASI uses aborting panics; its fixture exercises successful attachment.
        let outcomes: &[bool] = if cfg!(target_family = "wasm") {
            &[false]
        } else {
            &[false, true]
        };
        for concurrent in [false, true] {
            for legacy in [false, true] {
                for &fail in outcomes {
                    let gc = GarbageCollector::with_config(
                        crate::Configuration::builder()
                            .concurrent_publication(concurrent)
                            .build(),
                    );
                    let target = Gc::try_new(&gc, 42_u64)?;
                    let link = Arc::new(GcWeak::new(&target, ReferenceKind::Final, 0));
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let owner = Owner {
                            link: link.clone(),
                            fail,
                        };
                        if legacy {
                            Ok(Gc::new(&gc, owner))
                        } else {
                            Gc::try_new(&gc, owner)
                        }
                    }));
                    assert_eq!(link.slot.unpublished_owners.load(Ordering::Acquire), 0);
                    if fail {
                        assert!(result.is_err());
                        assert!(link.slot.owner_identity().is_none());
                        let replacement = Gc::try_new(&gc, 99_u64)?;
                        link.slot
                            .owner
                            .store(replacement.as_ptr() as usize, Ordering::Release);
                        drop(target);
                        gc.collect_full()?;
                        assert!(!gc.has_pending_references());
                        assert!(link.upgrade().is_none());
                        drop(replacement);
                    } else {
                        let owner = result.expect("successful attachment")?;
                        assert_eq!(link.slot.owner_identity(), Some(owner.identity()));
                        drop(target);
                        gc.collect_full()?;
                        assert!(link.upgrade().is_some());
                        let pending = gc.take_pending_reference::<Owner>()?.expect("notification");
                        assert_eq!(pending.identity(), owner.identity());
                        assert!(!gc.has_pending_references());
                        link.clear();
                        drop((pending, owner));
                    }
                    gc.stop()?;
                    assert_eq!(gc.statistics()?.live_objects, 0);
                    assert_eq!(gc.managed_bytes(), 0);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn creating_a_weak_slot_from_a_stale_edge_cannot_bind_a_reused_address() -> crate::Result<()> {
        for concurrent in [false, true] {
            for kind in [
                ReferenceKind::Soft,
                ReferenceKind::Weak,
                ReferenceKind::Final,
                ReferenceKind::Phantom,
                ReferenceKind::PhantomRetained,
            ] {
                let gc = GarbageCollector::with_config(
                    crate::Configuration::builder()
                        .concurrent_publication(concurrent)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let first = Gc::try_new(&gc, 1_u64)?;
                let mut stale = first.clone_gc();
                // SAFETY: The owning handle protects demotion; the stale identity is never dereferenced.
                unsafe { stale.unroot() };
                let original = stale.identity();
                drop(first);
                gc.collect_full()?;
                let replacement = Gc::try_new(&gc, 2_u64)?;
                // Simulate address reuse without dereferencing freed storage.
                // The stale edge retains its original immutable object identity.
                stale.ptr = replacement.ptr;
                assert_ne!(original, replacement.identity());
                assert!(gc.clone_edge_root(&stale).is_none());
                let weak = GcWeak::new(&stale, kind, 0);
                assert!(weak.upgrade().is_none());
                assert!(weak.clone().upgrade().is_none());
                assert_eq!(**replacement, 2);
                drop((weak, stale, replacement));
                gc.stop()?;
                assert_eq!(gc.statistics()?.live_objects, 0);
            }
        }
        Ok(())
    }

    #[test]
    fn an_expired_slot_cannot_upgrade_a_different_allocation_at_its_address() -> crate::Result<()> {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder().metadata_shards(16).build(),
        );
        let first = Gc::new(&gc, 1_u64);
        let weak = GcWeak::new(&first, ReferenceKind::Weak, 0);
        drop(first);
        gc.collect_full()?;
        let replacement = Gc::new(&gc, 2_u64);
        // Model allocator address reuse deterministically, keeping the slot's original
        // allocation identity while presenting an address occupied by a new allocation.
        weak.slot
            .target
            .store(replacement.as_ptr() as usize, Ordering::Release);
        assert!(weak.upgrade().is_none());
        assert!(weak.clone().upgrade().is_none());
        Ok(())
    }
}
