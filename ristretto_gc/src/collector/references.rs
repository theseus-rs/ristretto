#[cfg(test)]
use super::lock;
use super::{
    Arc, Context, GarbageCollector, Gc, MetadataRef, ObjectMetadata, Ordering,
    PendingReferenceRoot, Result, Retention, Trace,
};

impl GarbageCollector {
    pub(super) fn register_weak(&self, slot: &Arc<crate::weak::WeakSlot>) {
        let address = slot.target.load(Ordering::Acquire);
        let metadata = self.weak_target_metadata(slot, address);
        let young = metadata.as_ref().is_some_and(|m| self.is_young_metadata(m));
        let id = self.weak_slots.insert(&self.next_weak_id, slot, young);
        // A retaining clone inherits its source's birth exemption and drops its
        // owner. That can create an eligible reference behind an active cursor.
        if matches!(
            slot.kind,
            crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained
        ) {
            for (i, domain) in self.domains.iter().enumerate() {
                if (i != 0 || young)
                    && slot.birth_stamp(i).load(Ordering::Relaxed)
                        != domain.epoch.load(Ordering::Acquire)
                {
                    domain
                        .late_references
                        .offer_if_eligible(domain.weak_phase(), id, slot);
                }
            }
        }
        if self.old_collection_active()
            && let Some(metadata) = metadata
        {
            let _context = Context::enter(self, 1);
            self.shade_known(&metadata);
        }
    }

    pub(super) fn reference_owner(&self, slot: &crate::weak::WeakSlot) -> Option<MetadataRef> {
        let identity = slot.owner_identity()?;
        let address = slot.owner.load(Ordering::Acquire);
        self.objects
            .lookup(address)
            .filter(|metadata| metadata.id == identity)
    }

    pub(super) fn logically_retired(&self, metadata: &ObjectMetadata) -> bool {
        self.domains.iter().enumerate().any(|(index, domain)| {
            if domain.weak_phase() != 5 || self.old_excludes_young(index, metadata) {
                return false;
            }
            let exempt = index == 0
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1);
            !exempt && !metadata.marked(index, domain.epoch.load(Ordering::Acquire))
        })
    }

    pub(super) fn reference_owner_live(&self, slot: &crate::weak::WeakSlot) -> bool {
        self.reference_owner(slot).is_some_and(|metadata| {
            if self.configuration.overlapping_generations && self.logically_retired(&metadata) {
                return false;
            }
            self.visible_mark(
                &metadata,
                self.domain_index(),
                self.domain().epoch.load(Ordering::Acquire),
            ) || ((self.domain_index() == 0)
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1))
        })
    }

    pub(super) fn reference_metadata_live(
        &self,
        metadata: &ObjectMetadata,
        kind: crate::ReferenceKind,
    ) -> bool {
        self.reference_metadata_live_at(
            metadata,
            kind,
            self.domain_index(),
            self.domain().epoch.load(Ordering::Acquire),
        )
    }

    pub(super) fn reference_metadata_live_at(
        &self,
        metadata: &ObjectMetadata,
        kind: crate::ReferenceKind,
        index: usize,
        epoch: u64,
    ) -> bool {
        let final_retention = matches!(
            kind,
            crate::ReferenceKind::Phantom | crate::ReferenceKind::PhantomRetained
        );
        self.old_excludes_young(index, metadata)
            || metadata.marked_before(index, epoch, final_retention)
            || metadata
                .direct_marks
                .get(index, epoch, &self.domain_at(index).mark_queue)
                .is_some_and(|strength| {
                    strength
                        <= if final_retention {
                            Retention::Final
                        } else {
                            Retention::Strong
                        }
                })
            || (index == 0
                && metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1))
    }

    pub(crate) fn weak_is_cleared(&self, slot: &crate::weak::WeakSlot, address: usize) -> bool {
        self.uncleared_weak_target(slot, address).is_none()
    }

    /// An exact visible descriptor whose conditional edge survived both domains.
    /// Publication admission protects the phase decision. The descriptor grants
    /// no payload access and terminal retirement is a separate test.
    pub(super) fn uncleared_weak_target(
        &self,
        slot: &crate::weak::WeakSlot,
        address: usize,
    ) -> Option<MetadataRef> {
        let metadata = self.weak_target_metadata(slot, address)?;
        let cleared = (0..2).any(|index| {
            let _context = Context::enter(self, index);
            if slot.birth_stamp(index).load(Ordering::Acquire)
                == self.domain().epoch.load(Ordering::Acquire)
            {
                return false;
            }
            let phase = self.domain().weak_phase();
            let decision = match slot.kind {
                crate::ReferenceKind::Soft | crate::ReferenceKind::Weak => phase >= 1,
                crate::ReferenceKind::Phantom => phase >= 3,
                crate::ReferenceKind::PhantomRetained | crate::ReferenceKind::Final => false,
            };
            decision && !self.reference_metadata_live(&metadata, slot.kind)
        });
        (!cleared).then_some(metadata)
    }

    pub(crate) fn weak_birth_epochs(&self) -> [u64; 2] {
        std::array::from_fn(|index| {
            let domain = self.domain_at(index);
            if domain.weak_phase() != 0 {
                domain.epoch.load(Ordering::Acquire)
            } else {
                0
            }
        })
    }

    /// Returns whether Java reference processing has published work for the Reference Handler.
    #[must_use]
    pub fn has_pending_references(&self) -> bool {
        self.pending_references.get().is_some_and(|queue| {
            // A private append increments the reservation count before linking.
            queue.pending() != 0 && !queue.is_empty()
        })
    }

    /// Waits without polling for reference processing to publish a pending reference.
    pub async fn wait_for_pending_references(&self) {
        self.recovery.wait_for_references(self).await;
    }

    /// Transfers a pending reference's root to the caller.
    ///
    /// # Errors
    /// Returns an error if the caller requests the wrong allocation type.
    pub fn take_pending_reference<T: Trace + 'static>(self: &Arc<Self>) -> Result<Option<Gc<T>>> {
        let Some(queue) = self.pending_references.get() else {
            return Ok(None);
        };
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        let next = queue.test_pop_after_head(super::reference_tests::pause_after_pending_head);
        #[cfg(not(all(test, not(target_family = "wasm"), not(miri))))]
        let next = queue.pop();
        let Some((identity, id)) = next else {
            return Ok(None);
        };
        // The queue's root remains registered after dequeue. Waiting on the queue
        // or allocating the caller's owning record needs no outer admission;
        // Identity validation rejects a replacement at a revoked owner's former
        // address. Root generations also prevent stale queue cleanup from
        // removing another registration after the public numeric ID wraps.
        let queued_root = PendingReferenceRoot {
            collector: self,
            id,
        };
        #[cfg(test)]
        {
            let pause = lock(&self.pending_transfer_pause).take();
            if let Some(pause) = pause {
                pause
                    .entered
                    .send(())
                    .map_err(|error| crate::Error::SyncError(error.to_string()))?;
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(if cfg!(miri) {
                        120
                    } else {
                        10
                    }))
                    .map_err(|error| crate::Error::SyncError(error.to_string()))?;
            }
        }
        let result = Gc::from_handle_rooted(self, identity.to_i64());
        drop(queued_root);
        result.map(Some)
    }

    pub(super) fn enqueue_reference(&self, slot: &crate::weak::WeakSlot) {
        if let Some(metadata) = self.reference_owner(slot) {
            let id = self.insert_root(metadata.ptr().0);
            let queue = self
                .pending_references
                .get_or_init(|| Box::new(crate::publication_queue::Queue::new()));
            #[cfg(all(test, not(target_family = "wasm")))]
            queue.test_push_after_link(
                (metadata.id, id),
                super::reference_tests::pause_after_pending_notice,
            );
            #[cfg(not(all(test, not(target_family = "wasm"))))]
            queue.push((metadata.id, id));
        }
    }
}
