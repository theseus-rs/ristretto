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

    pub(crate) fn prepare_root_registration(
        &self,
        address: usize,
    ) -> super::PreparedRootRegistration<'_> {
        let owned = self.reserve_owned_root(address);
        let tree = owned.slot.is_none().then(|| self.reserve_tree_root());
        super::PreparedRootRegistration { owned, tree }
    }

    pub(crate) fn reserve_tree_root(&self) -> crate::roots::VacantRoot<'_> {
        #[cfg(test)]
        self.probe_tree_root_insertion();
        self.roots.reserve(&self.next_root_id)
    }

    pub(super) fn publish_reserved_tree_root(
        &self,
        tree: &mut crate::roots::VacantRoot<'_>,
        address: usize,
    ) -> crate::roots::TreeRoot {
        tree.publish(address);
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        tree.key()
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
        // The existing owner keeps the graph live while this independent root is prepared.
        let address = root.ptr.as_ptr().addr();
        let mut reservation = self.prepare_root_registration(address);
        let _publication = self.allocation_guard();
        let registration = reservation.publish();
        self.shade_all(address);
        registration
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
        self.detach_root::<ADOPT>(|| root.take_address()) && root.release()
    }

    /// Keep source removal, any replacement publication, and the deletion
    /// barrier in the same admission. The source remains scannable throughout
    /// preparation. Only the exact winning removal supplies an address.
    fn detach_root<const ADOPT: bool>(&self, take: impl FnOnce() -> Option<usize>) -> bool {
        let detach = |replacement: Option<&mut crate::roots::VacantRoot<'_>>| {
            let _publication = self.allocation_guard();
            let Some(address) = take() else { return false };
            if let Some(root) = replacement
                && self.objects.lookup(address).is_some()
            {
                // Skip the initial root of an unpublished allocation on unwind.
                self.publish_reserved_tree_root(root, address);
            }
            self.barrier(address, None);
            true
        };
        if ADOPT {
            crate::construction::with_removal_root(self, detach)
        } else {
            detach(None)
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
        if self.detach_root::<ADOPT>(|| record.take_address()) {
            roots.finish_overflow(record);
        }
    }

    #[cfg(test)]
    pub(super) fn probe_tree_root_insertion(&self) {
        let probe = *lock(&self.tree_root_insert_probe);
        if let Some(probe) = probe {
            probe(self);
        }
    }

    pub(super) fn insert_root(&self, address: usize) -> crate::roots::TreeRoot {
        #[cfg(test)]
        self.probe_tree_root_insertion();
        if self.diagnostics_enabled() {
            self.diagnostics
                .root_registrations
                .fetch_add(1, Ordering::Relaxed);
        }
        self.roots.insert(&self.next_root_id, address)
    }

    /// A conditional edge is not proof of liveness. Check its allocation identity
    /// and closed weak/retirement decisions before creating a construction root.
    /// Called under short publication admission and the slot's decision lock.
    pub(crate) fn retain_construction_referent(
        &self,
        slot: &crate::weak::WeakSlot,
        tree: &mut crate::roots::VacantRoot<'_>,
    ) -> Option<crate::roots::TreeRoot> {
        let address = slot.target.load(Ordering::Acquire);
        let metadata = self.uncleared_weak_target(slot, address)?;
        if self.logically_retired(&metadata) {
            return None;
        }
        let id = self.publish_reserved_tree_root(tree, metadata.ptr().0);
        // Enumeration may already have passed this root. Publish its graph to
        // each active mark queue before admission ends, respecting phase floors.
        self.shade_live_domains(&metadata);
        Some(id)
    }

    /// Registers a manual root, including while a collection is in progress.
    /// Release the returned ID with `remove_root_by_id`; owning handles and
    /// internal collector registrations are managed independently.
    ///
    /// # Panics
    /// Panics if the handle is foreign, unassociated or no longer live.
    #[expect(
        clippy::expect_used,
        reason = "the manual root API requires a live handle"
    )]
    pub fn add_root<T: Trace>(&self, root: &Gc<T>) -> usize {
        assert!(
            root.belongs_to(self),
            "manual root collector does not own this allocation"
        );
        #[cfg(test)]
        self.probe_tree_root_insertion();
        let mut tree = self.roots.reserve_manual(&self.next_root_id);
        let address = root.ptr.as_ptr().addr();
        let _publication = self.allocation_guard();
        // The argument can be a borrowed edge. Match its non-reused identity and
        // check both domains after preparation, before making the manual ID visible.
        let metadata = self
            .objects
            .lookup_identity(address, root.identity())
            .filter(|metadata| self.can_root(metadata, false))
            .expect("manual root target is no longer live");
        let id = self.publish_reserved_tree_root(&mut tree, address);
        self.shade_live_domains(&metadata);
        id.id
    }

    /// Creates an owning root registration in this allocation's collector.
    ///
    /// # Panics
    /// Panics if the handle belongs to another collector, is unassociated, or has
    /// retired. Use `Gc::as_root` for a fallible conversion.
    #[expect(
        clippy::expect_used,
        reason = "the owning guard API requires a live handle"
    )]
    pub fn create_root_guard<T: Trace>(self: &Arc<Self>, root: Gc<T>) -> GcRootGuard<T> {
        let id = root
            .guard_registration(self)
            .expect("cannot register root guard");
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
        self.detach_root::<ADOPT>(|| {
            let address = self.roots.remove(id, generation)?;
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::pause_after_tree_detach();
            Some(address)
        });
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
