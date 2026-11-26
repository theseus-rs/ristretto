#[cfg(test)]
use super::lock;
use super::{
    Arc, GarbageCollector, Gc, GcRootGuard, MetadataRef, Ordering, RootReservation, Trace,
    publication_state,
};

impl GarbageCollector {
    pub(crate) fn scanned_slots(&self) -> Option<&crate::roots::RootSlots> {
        self.root_slots
            .as_deref()
            .or_else(|| self.typed_slots.get())
    }

    pub(crate) fn reserve_owned_root(&self, address: usize) -> RootReservation<'_> {
        let context = (self.root_slots.is_some()
            && self.configuration.root_storage == crate::RootStorage::Contexts)
            .then(|| crate::context::current(self))
            .flatten();
        let slot = self.root_slots.as_ref().map(|roots| {
            context.as_ref().map_or_else(
                || roots.prepare(address),
                |context| context.roots.prepare(address),
            )
        });
        RootReservation {
            collector: self,
            address,
            context,
            slot,
        }
    }

    /// A managed edge already carries its original Rust type. Resolve its
    /// exact identity in this collector and certify liveness as for a VM handle.
    pub(crate) fn clone_edge_root<T>(&self, root: &Gc<T>) -> Option<crate::roots::Registration> {
        self.acquire_checked_root::<T>(root.ptr.as_ptr().addr(), Some(root.identity()), |_| true)
            .ok()
            .map(|(_, _, registration)| registration)
    }

    pub(crate) fn add_owned_root<T>(&self, root: &Gc<T>) -> crate::roots::Registration {
        let _measurement = self
            .diagnostics
            .root_acquire
            .measure(self.diagnostics_enabled());
        let mut reservation = self.reserve_owned_root(root.ptr.as_ptr() as usize);
        let _publication = self.allocation_guard();
        self.commit_owned_root(root, &mut reservation)
    }

    /// Install the initial root of an exclusively owned, unregistered allocation.
    /// Its address cannot resolve to a live object yet. Descriptor registration
    /// performs the birth/traversal barrier in each active domain before exposing
    /// the handle. Construction roots protect the outgoing edges until then.
    pub(crate) fn add_private_root(&self, metadata: &MetadataRef) -> crate::roots::Registration {
        let _measurement = self
            .diagnostics
            .root_acquire
            .measure(self.diagnostics_enabled());
        self.install_root_registration(&mut self.reserve_owned_root(metadata.ptr().0))
    }

    /// Caller retains this collector's publication admission through commit/shading.
    pub(super) fn commit_owned_root<T>(
        &self,
        root: &Gc<T>,
        reservation: &mut RootReservation<'_>,
    ) -> crate::roots::Registration {
        let address = root.ptr.as_ptr() as usize;
        assert!(
            reservation.protects(self, address),
            "foreign root reservation"
        );
        let registration = self.install_root_registration(reservation);
        self.shade_all(address);
        registration
    }

    pub(super) fn install_root_registration(
        &self,
        reservation: &mut RootReservation<'_>,
    ) -> crate::roots::Registration {
        let address = reservation.address;
        let Some(slot) = reservation.slot.take() else {
            return crate::roots::Registration::Tree(self.insert_root(address));
        };
        let registration = slot.commit(address, reservation.context.take());
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        registration
    }

    pub(crate) fn remove_owned_root(&self, id: &crate::roots::Registration) {
        self.remove_owned_root_with_construction::<true>(id);
    }

    /// Release a transferred registration without consulting a helper's ambient
    /// construction scope. The caller must already own this registration and its
    /// graph protection; normal deletion admission, barriers and reuse still apply.
    pub(crate) fn remove_detached_root(&self, id: &crate::roots::Registration) {
        self.remove_owned_root_with_construction::<false>(id);
    }

    pub(super) fn remove_owned_root_with_construction<const ADOPT: bool>(
        &self,
        id: &crate::roots::Registration,
    ) {
        let _measurement = self
            .diagnostics
            .root_release
            .measure(self.diagnostics_enabled());
        match id {
            crate::roots::Registration::Tree(id) => {
                self.remove_tree_registration::<ADOPT>(id.id, Some(id.generation));
            }
            crate::roots::Registration::Overflow(record) => {
                if let Some(roots) = self.scanned_slots() {
                    self.remove_overflow_root::<ADOPT>(roots, record);
                }
            }
            crate::roots::Registration::ContextOverflow(context, record) => {
                self.remove_overflow_root::<ADOPT>(&context.roots, record);
            }
            crate::roots::Registration::Slot(root) => {
                if self.remove_paged_root::<ADOPT>(root)
                    && let Some(roots) = self.scanned_slots()
                {
                    roots.finish_removal(root);
                    self.wake_root_repair(roots);
                }
            }
            crate::roots::Registration::Context(context, root) => {
                if self.remove_paged_root::<ADOPT>(root) && context.roots.finish_removal(root) {
                    self.wake_context_root_repair(context);
                }
            }
        }
    }

    pub(super) fn remove_paged_root<const ADOPT: bool>(
        &self,
        root: &crate::roots::SlotRoot,
    ) -> bool {
        if !ADOPT
            && let Some(address) = root.begin_removal()
            && self.deletion_covered(address, || root.removal_pending())
            && root.finish_removal()
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::publication_tests::record_covered_removal();
            return root.release();
        }
        let _publication = self.allocation_guard();
        if let Some(address) = root.take_address() {
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
            root.release()
        } else {
            false
        }
    }

    /// The exact removal intent must already be visible to every root scanner.
    pub(super) fn deletion_covered(
        &self,
        address: usize,
        removal_pending: impl FnOnce() -> bool,
    ) -> bool {
        let observation = self.phase_clock.indexed_root();
        // Retirement ignores all deletion shades. If a new domain starts
        // after this sample, its root scan still races the exact removal intent.
        let covered = observation.deletion_closed()
            || self.objects.lookup(address).is_some_and(|metadata| {
                if self.typed_root_covered(observation, &metadata) {
                    return true;
                }
                // Attached contexts already batch deletion barriers in fixed-size
                // buffers. Preserve that path and its allocation behavior.
                if self.configuration.barrier_buffer_size != 0
                    && crate::context::current(self).is_some()
                {
                    return false;
                }
                let phases = self.phase_clock.indexed_root();
                match &phases {
                    publication_state::Birth::Idle => return true,
                    publication_state::Birth::Inline(s) | publication_state::Birth::Epochs(s)
                        if (0..2).any(|i| s.stage(i).is_some_and(|stage| stage != 0)) =>
                    {
                        return false;
                    }
                    _ => {}
                }
                // Each attempt has its own decision. Collectors can cancel a parked
                // publisher when they visit a proposal. Validate the source after
                // linking, while a live matching generation still owns the address.
                let debt = Arc::new(crate::roots::Record::typed(address));
                let certified = self.certify_root_attempt(phases, &metadata, &debt) == Some(true);
                #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
                super::publication_tests::pause_after_typed_proposals();
                certified && removal_pending() && debt.commit_typed()
            });
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        super::publication_tests::pause_after_drop_coverage();
        covered
    }

    pub(super) fn remove_overflow_root<const ADOPT: bool>(
        &self,
        roots: &crate::roots::RootSlots,
        record: &Arc<crate::roots::Record>,
    ) {
        if !ADOPT
            && let Some(address) = record.begin_typed_removal()
            && self.deletion_covered(address, || record.typed_removal_pending())
            && record.finish_typed_removal()
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::publication_tests::record_covered_removal();
            roots.finish_overflow(record);
            return;
        }
        let publication = self.allocation_guard();
        if let Some(address) = record.take_address() {
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
            drop(publication);
            roots.finish_overflow(record);
        }
    }

    pub(super) fn insert_root(&self, address: usize) -> crate::roots::TreeRoot {
        self.insert_tree_root(address, true)
    }

    pub(super) fn insert_tree_root(&self, address: usize, owned: bool) -> crate::roots::TreeRoot {
        #[cfg(test)]
        {
            let probe = *lock(&self.tree_root_insert_probe);
            if let Some(probe) = probe {
                probe(self);
            }
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        if owned {
            self.roots.insert(&self.next_root_id, address)
        } else {
            self.roots.insert_manual(&self.next_root_id, address)
        }
    }

    /// Called under a publication reader before the removed root can cease to
    /// protect its target. Unpublished descriptors have no registry entry yet.
    pub(crate) fn retain_construction_root(
        &self,
        address: usize,
    ) -> Option<crate::roots::TreeRoot> {
        let address = self.objects.lock(address).get(&address)?.ptr().0;
        Some(self.insert_root(address))
    }

    /// A conditional edge is not proof of liveness. Check its allocation identity
    /// and closed weak/retirement decisions before creating a construction root.
    /// Called under short publication admission and the slot's decision lock.
    pub(crate) fn retain_construction_referent(
        &self,
        slot: &crate::weak::WeakSlot,
    ) -> Option<crate::roots::TreeRoot> {
        let address = slot.target.load(Ordering::Acquire);
        if address == 0 || self.weak_is_cleared(slot, address) {
            return None;
        }
        let metadata = self.objects.lock(address).get(&address).cloned()?;
        if metadata.id != slot.target_identity || self.logically_retired(&metadata) {
            return None;
        }
        let id = self.insert_root(metadata.ptr().0);
        // Enumeration may already have passed this root. Publish its graph to
        // each active mark queue before admission ends, respecting phase floors.
        self.shade_all(address);
        Some(id)
    }

    /// Registers a manual root, including while a collection is in progress.
    /// Release the returned ID with `remove_root_by_id`; owning handles and
    /// internal collector registrations are managed independently.
    pub fn add_root<T: Trace>(&self, root: &Gc<T>) -> usize {
        let _publication = self.allocation_guard();
        let id = self.insert_tree_root(root.ptr.as_ptr() as usize, false);
        self.shade_all(root.ptr.as_ptr() as usize);
        id.id
    }

    /// Creates an owning root registration in this allocation's collector.
    ///
    /// # Panics
    /// Panics before registration if the handle belongs to another collector or
    /// is unassociated. Use `Gc::as_root` to receive a mismatch as an error.
    pub fn create_root_guard<T: Trace>(self: &Arc<Self>, root: Gc<T>) -> GcRootGuard<T> {
        assert!(
            root.belongs_to(self),
            "root guard collector does not own this allocation"
        );
        let id = self.add_owned_root(&root);
        GcRootGuard::new(self.clone(), id, root)
    }

    /// Removes a manual root returned by `add_root`, preserving the snapshot
    /// during an active marking cycle. Other IDs cannot revoke owning handles or
    /// internal collector registrations.
    /// Numeric IDs are invalid after removal and can be reused after usize wrap.
    pub fn remove_root_by_id(&self, id: usize) {
        self.remove_tree_registration::<true>(id, None);
    }

    pub(super) fn remove_tree_root(&self, root: crate::roots::TreeRoot) {
        self.remove_tree_registration::<true>(root.id, Some(root.generation));
    }

    pub(super) fn remove_tree_registration<const ADOPT: bool>(
        &self,
        id: usize,
        generation: Option<std::num::NonZeroU64>,
    ) {
        if !ADOPT
            && let Some(generation) = generation
            && (self.configuration.barrier_buffer_size == 0
                || crate::context::current(self).is_none())
            && self.roots.remove_covered(
                crate::roots::TreeRoot { id, generation },
                |address, pending| self.deletion_covered(address, pending),
            )
        {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::publication_tests::record_covered_removal();
            return;
        }
        let _publication = self.allocation_guard();
        let removed = self.roots.remove(id, generation);
        if let Some(address) = removed {
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::pause_after_tree_detach();
            if ADOPT {
                crate::construction::retain(self, address);
            }
            self.barrier(address, None);
        }
    }

    /// Revokes every registration for an allocation, including owning handles.
    ///
    /// Prefer dropping owning handles or removing an `add_root` registration
    /// with `remove_root_by_id`.
    ///
    /// # Safety
    /// The `root` argument must remain valid for this entire call, independently
    /// of the registrations being revoked (for example, while reclamation is
    /// externally excluded).
    /// The caller must prevent access through every affected handle after the
    /// allocation can be reclaimed, including concurrent readers and references
    /// borrowed from those handles. Dropping revoked handles without accessing
    /// their payload is allowed. If handles remain usable, the caller must keep
    /// the allocation and its traced graph independently reachable until a new
    /// registration is established. Revocation does not transfer ownership.
    ///
    /// ```compile_fail,E0133
    /// use ristretto_gc::{GarbageCollector, Gc};
    /// let gc = GarbageCollector::new();
    /// let root = Gc::new(&gc, 42_u64);
    /// gc.remove_root(&**root);
    /// ```
    pub unsafe fn remove_root<T: Trace>(&self, root: &T) {
        // The caller independently protects the payload. Each deletion validates
        // its exact registration and publishes its own barrier; enumeration
        // needs no heap-wide admission across unrelated slots and contexts.
        let ptr = std::ptr::from_ref(root).cast::<()>();
        if let Some(roots) = self.scanned_slots() {
            for record in roots.matching_overflow(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Overflow(record));
            }
            for id in roots.matching(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Slot(id));
            }
        }
        self.contexts.for_each(|context| {
            for record in context.roots.matching_overflow(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::ContextOverflow(
                    context.clone(),
                    record,
                ));
            }
            for id in context.roots.matching(ptr as usize) {
                self.remove_owned_root(&crate::roots::Registration::Context(context.clone(), id));
            }
        });
        let ids = self.roots.matching(ptr as usize);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        super::publication_tests::pause_after_revocation_scan();
        for id in ids {
            self.remove_tree_root(id);
        }
    }
}
