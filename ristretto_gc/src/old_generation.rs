//! Conservative old-only tracing with bounded young-to-old handoff.
//!
//! Full collections remain the precise collector for conditional references and
//! cross-generation cycles. An old pass treats its initial young cohort and old
//! conditional referents as snapshot roots. It never processes reference slots or
//! completes a full-collection ticket. New young allocations remain collectible.
use super::{
    Context, Cycle, Excluded, GarbageCollector, MetadataRef, ObjectMetadata, Ordering, Phase,
    Retention, TraceProgress, TraceRetention, Unbounded, lock,
};

impl GarbageCollector {
    pub(super) fn old_collection_active(&self) -> bool {
        self.configuration.old_collection_interval != 0
            && self.old_active.load(Ordering::Acquire)
            && self.domains[1].active()
    }

    pub(super) fn old_excludes_young(&self, index: usize, metadata: &ObjectMetadata) -> bool {
        index == 1
            && self.old_collection_active()
            && metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
    }

    /// Request an experimental old-only pass. Disabled configurations request a
    /// precise full collection instead. This is asynchronous even on cooperative
    /// hosts; call `step` at safe boundaries to drive it. Full requests take priority.
    pub fn collect_old(&self) {
        if self.configuration.old_collection_interval == 0 {
            self.collect();
            return;
        }
        self.old_requested.store(true, Ordering::Release);
        self.wake();
    }

    /// Called after the age store while the promoting object shard and admission
    /// are held. Thus every old-core callback observes an old allocation, and old
    /// closure cannot miss the promotion's queued traversal.
    pub(super) fn publish_old_promotion(&self, metadata: &MetadataRef) {
        if self.domain_index() == 0
            && self.old_collection_active()
            && self.domains[1].weak_phase() == 0
        {
            self.domains[1]
                .mark_queue
                .push(0, super::MarkItem::fresh(metadata.clone()));
        }
    }

    /// Caller owns publication admission. An old callback
    /// can still hold a young graph after its owner is mutated. Preserve that graph
    /// for subsequent young snapshots, without putting young objects on the old
    /// tracing queue. Owning-root changes alone do not retain post-snapshot births.
    pub(super) fn retain_old_callback_edge(&self, address: usize) {
        if !self.old_collection_active() || self.domains[1].weak_phase() != 0 {
            return;
        }
        if let Some(metadata) = self.objects.lock(address).get(&address)
            && metadata.age.load(Ordering::Acquire) < self.configuration.promotion_age.max(1)
            && !self.logically_retired(metadata)
        {
            metadata.mark(
                1,
                self.domains[1].epoch.load(Ordering::Acquire),
                Retention::Strong,
            );
        }
    }

    /// One cursor/callback unit. Its cycle lock excludes old closure, while each
    /// edge publication carries short admission. Young closure never waits for
    /// the whole callback. Metadata plus the old snapshot protects its graph;
    /// a storage lease alone would not suffice.
    pub(super) fn old_handoff_unit(&self, cycle: &mut Cycle, work: &mut usize) -> bool {
        if cycle.handoff.is_none() {
            let address = match self.objects.scan_young(&mut cycle.young_cursor) {
                crate::index::Scan::Entry(address, ()) => address,
                crate::index::Scan::Advance => return true,
                crate::index::Scan::Busy => return false,
                crate::index::Scan::Done => {
                    cycle.phase = Phase::OldReferences;
                    cycle.cursor = 0;
                    return true;
                }
            };
            let _admission = self.allocation_guard();
            let Some(objects) = self.objects.try_lock(address) else {
                cycle.young_cursor.retry_entry();
                return false;
            };
            let Some(metadata) = objects.get(&address) else {
                return true;
            };
            // Births are black but not old snapshot roots. The separate birth
            // epoch remains stable even if a heap deletion later retains one.
            if metadata.old_birth.load(Ordering::Acquire) == cycle.epoch
                || self.logically_retired(metadata)
                || metadata.age.load(Ordering::Acquire) >= self.configuration.promotion_age.max(1)
            {
                return true;
            }
            metadata.mark(1, cycle.epoch, Retention::Strong);
            let metadata = metadata.clone();
            drop(objects);
            {
                let _young = Context::enter(self, 0);
                self.shade_known(&metadata);
            }
            cycle.handoff = Some((metadata, 0));
        }
        let Some((metadata, offset)) = cycle.handoff.take() else {
            return true;
        };
        let budget = (*work + 1).min(32).min(metadata.trace.budget.max(1));
        *work = work.saturating_sub(budget.saturating_sub(1));
        let _retention = TraceRetention::enter(self, Retention::Strong);
        let progress = if metadata.trace.step_fn.is_none() {
            TraceProgress::Complete
        } else if let Some(_lease) = ObjectMetadata::lease(&metadata) {
            if let Ok(_scan) = metadata.scan.try_lock() {
                metadata
                    .trace
                    .step_fn
                    .map_or(TraceProgress::Complete, |step| {
                        step(metadata.trace.ptr, self, offset, budget)
                    })
            } else {
                TraceProgress::Busy
            }
        } else {
            TraceProgress::Complete
        };
        match progress {
            TraceProgress::Complete => {
                if self.diagnostics_enabled() {
                    self.diagnostics
                        .old_handoff_objects
                        .fetch_add(1, Ordering::Relaxed);
                }
            }
            TraceProgress::Continue(offset) => cycle.handoff = Some((metadata, offset)),
            TraceProgress::Busy => {
                cycle.handoff = Some((metadata, offset));
                return false;
            }
        }
        true
    }

    /// Conditional referents are conservative roots of an old pass. Only a precise
    /// full collection makes global weak/final/phantom decisions; young collections
    /// retain their existing ordered decisions. Late slot registration publishes
    /// the same old root under admission, so enumeration may safely finish.
    pub(super) fn old_reference_root_unit(&self, cycle: &mut Cycle) {
        let _admission = self.allocation_guard();
        let next = lock(&self.weak_slots)
            .range((Excluded(cycle.cursor), Unbounded))
            .next()
            .map(|(id, slot)| (*id, slot.upgrade()));
        let Some((id, slot)) = next else {
            cycle.cursor = 0;
            cycle.phase = if self.configuration.root_storage == crate::RootStorage::Contexts
                || self.configuration.local_allocation_buffers
            {
                Phase::ContextRoots
            } else if self.scanned_slots().is_some() {
                Phase::RootSlots
            } else {
                Phase::Roots
            };
            return;
        };
        if let Some(slot) = slot {
            let Some(_decision) = slot.try_decision() else {
                return;
            };
            let address = slot.target.load(Ordering::Acquire);
            if self.weak_target_is_registered(&slot, address) {
                let _retention = TraceRetention::enter(self, Retention::Strong);
                self.shade(address);
            }
        }
        cycle.cursor = id;
    }
}
