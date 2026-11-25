//! Bounded per-domain work; only global phase transitions need the coordinator.
use super::{
    CollectionKind, Context, Cycle, Excluded, GarbageCollector, MetadataRef, Ordering, Phase,
    ReclaimTask, TraceRetention, Unbounded, WorkCounters, lock, now,
};

pub(super) const SWEEP_BATCH: usize = 16;

pub(super) enum Unit {
    Progress,
    Busy,
    Boundary,
    Sweep(SweepWork),
}

pub(super) enum SweepWork {
    Reclaim(ReclaimTask, usize),
    Unlink(SealedPages),
}

impl SweepWork {
    pub(super) fn run(
        self,
        gc: &GarbageCollector,
        index: usize,
        epoch: u64,
        counters: &mut WorkCounters,
    ) {
        match self {
            Self::Reclaim(task, budget) => gc.reclaim_swept(task, index, epoch, budget, counters),
            Self::Unlink(pages) => pages.unlink(gc),
        }
    }
}

#[derive(Default)]
pub(super) struct SealedPages {
    address: Option<crate::index::Sealed>,
    #[cfg(not(feature = "experimental-identity-index"))]
    identity: Option<crate::identity_pages::Sealed>,
}
impl SealedPages {
    fn into_unit(self) -> Unit {
        #[cfg(not(feature = "experimental-identity-index"))]
        if self.identity.is_some() {
            return Unit::Sweep(SweepWork::Unlink(self));
        }
        if self.address.is_some() {
            Unit::Sweep(SweepWork::Unlink(self))
        } else {
            Unit::Progress
        }
    }

    fn unlink(self, gc: &GarbageCollector) {
        if let Some(page) = self.address {
            gc.objects.unlink(&page);
        }
        #[cfg(not(feature = "experimental-identity-index"))]
        if let Some(page) = self.identity {
            gc.identities.unlink(page);
        }
    }
}

pub(super) enum ContextUnit {
    Progress,
    Busy,
    Cleanup(crate::context::ContextCleanup),
}

impl GarbageCollector {
    #[expect(
        clippy::too_many_lines,
        reason = "one budget for independent domain units"
    )]
    pub(super) fn independent_phase_step(&self, concurrent: bool) -> Option<bool> {
        if !self.configuration.independent_phases {
            return None;
        }
        let preferred = self.next_domain.load(Ordering::Relaxed) % 2;
        let terminal =
            self.domains.iter().enumerate().position(|(index, domain)| {
                self.terminal_priority(index) && domain.weak_phase() != 0
            });
        let index = terminal.unwrap_or_else(|| {
            if self.domain_at(preferred).active() {
                preferred
            } else {
                1 - preferred
            }
        });
        let context = Context::enter(self, index);
        let cycle = self.domain().cycle.try_lock().ok()?;
        let started = now();
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let budget = self.configuration.incremental_step_size.max(1);
        let mut remaining = budget;
        let mut counters = WorkCounters::default();
        let progressed = 'units: {
            let mut cycle = cycle;
            let mut progressed = false;
            while remaining != 0 {
                if progressed
                    && self.configuration.max_pause_time_us != 0
                    && started.is_some_and(|started| {
                        started.elapsed().as_micros()
                            >= u128::from(self.configuration.max_pause_time_us)
                    })
                {
                    break;
                }
                remaining -= 1;
                if cycle.phase == Phase::ContextRoots {
                    match self.context_root_unit(&mut cycle) {
                        ContextUnit::Progress => {
                            progressed = true;
                            continue;
                        }
                        ContextUnit::Busy => {
                            progressed = true;
                            break;
                        }
                        ContextUnit::Cleanup(cleanup) => {
                            drop(cycle);
                            cleanup.run(self);
                            break 'units true;
                        }
                    }
                }
                let unit = match cycle.phase {
                    Phase::OldHandoff => {
                        if self.old_handoff_unit(&mut cycle, &mut remaining) {
                            Unit::Progress
                        } else {
                            Unit::Busy
                        }
                    }
                    Phase::OldReferences => {
                        self.old_reference_root_unit(&mut cycle);
                        Unit::Progress
                    }
                    Phase::ResetMarks
                    | Phase::ResetReferences
                    | Phase::RootSlots
                    | Phase::Roots
                    | Phase::CrossRoots
                    | Phase::Remembered => {
                        if self.root_unit(&mut cycle) {
                            Unit::Progress
                        } else {
                            Unit::Busy
                        }
                    }
                    Phase::References => self.reference_unit(&mut cycle, &mut counters),
                    Phase::Sweep => self.sweep_unit(&mut cycle, &mut remaining, &mut counters),
                    Phase::SelectRegions => {
                        if self
                            .regions
                            .as_ref()
                            .is_none_or(|heap| heap.select_evacuation())
                        {
                            cycle.phase = Phase::Compact;
                            self.domain().relocation_ready.store(
                                self.regions.is_some() && self.configuration.independent_relocation,
                                Ordering::Release,
                            );
                        }
                        Unit::Progress
                    }
                    _ => Unit::Boundary,
                };
                match unit {
                    Unit::Progress => progressed = true,
                    Unit::Busy => {
                        progressed = true;
                        break;
                    }
                    Unit::Boundary => break,
                    Unit::Sweep(work) => {
                        progressed = true;
                        let epoch = cycle.epoch;
                        drop(cycle);
                        work.run(self, index, epoch, &mut counters);
                        let Ok(next) = self.domain().cycle.try_lock() else {
                            break 'units true;
                        };
                        cycle = next;
                        if cycle.epoch != epoch || cycle.phase != Phase::Sweep {
                            break 'units true;
                        }
                    }
                }
            }
            progressed
        };
        if !progressed {
            return None;
        }
        let measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        self.next_domain.fetch_add(1, Ordering::Relaxed);
        if self.diagnostics_enabled() {
            self.diagnostics
                .independent_phase_steps
                .fetch_add(1, Ordering::Relaxed);
        }
        drop(context);
        self.record_step(started, cpu_started, concurrent, &mut counters);
        drop(measurement);
        self.notify_progress();
        Some(!self.pending())
    }

    pub(super) fn context_root_unit(&self, cycle: &mut Cycle) -> ContextUnit {
        let _admission = self
            .configuration
            .independent_phases
            .then(|| self.allocation_guard());
        match self.contexts.scan(&mut cycle.context_cursor) {
            crate::context::RootScan::Root(address) => {
                if self.try_shade(address).is_none() {
                    cycle.context_cursor.retry_root();
                    return ContextUnit::Busy;
                }
            }
            crate::context::RootScan::Advance => {}
            crate::context::RootScan::Done => cycle.phase = Phase::RootSlots,
            crate::context::RootScan::Cleanup(cleanup) => {
                return ContextUnit::Cleanup(cleanup);
            }
        }
        ContextUnit::Progress
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one bounded unit for each root enumeration phase"
    )]
    pub(super) fn root_unit(&self, cycle: &mut Cycle) -> bool {
        // An independent root read and its cross-domain publication are one
        // admission. Neither domain may close in the read-to-shade gap.
        let _admission = self
            .configuration
            .independent_phases
            .then(|| self.allocation_guard());
        let index = self.domain_index();
        match cycle.phase {
            Phase::ResetMarks => match self
                .objects
                .scan(&mut cycle.object_cursor, MetadataRef::clone)
            {
                crate::index::Scan::Entry(_, metadata) => {
                    metadata.reset_mark(index);
                    if index == 1 {
                        metadata.old_birth.store(0, Ordering::Release);
                    }
                }
                crate::index::Scan::Advance => {}
                crate::index::Scan::Busy => return false,
                crate::index::Scan::Done => {
                    cycle.phase = Phase::ResetReferences;
                    cycle.cursor = 0;
                }
            },
            Phase::ResetReferences => {
                let next = lock(&self.weak_slots)
                    .range((Excluded(cycle.cursor), Unbounded))
                    .next()
                    .map(|(id, slot)| (*id, slot.upgrade()));
                if let Some((id, slot)) = next {
                    cycle.cursor = id;
                    if let Some(slot) = slot {
                        slot.birth_stamp(index).store(0, Ordering::Release);
                    }
                } else {
                    self.domain().epoch.store(0, Ordering::Release);
                    cycle.phase = Phase::Idle;
                }
            }
            Phase::RootSlots => {
                match self
                    .scanned_slots()
                    .map(|roots| roots.scan(&mut cycle.root_cursor))
                {
                    Some(crate::roots::Scan::Root(address)) => {
                        if self.try_shade(address).is_none() {
                            cycle.root_cursor.retry_root();
                            return false;
                        }
                    }
                    Some(crate::roots::Scan::Advance) => {}
                    Some(crate::roots::Scan::Done) | None => cycle.phase = Phase::Roots,
                }
            }
            Phase::Roots => {
                let Some(roots) = super::try_lock(&self.roots) else {
                    return false;
                };
                let next = roots.next(cycle.cursor);
                drop(roots);
                if let Some((id, address)) = next {
                    if self.try_shade(address).is_none() {
                        return false;
                    }
                    cycle.cursor = id;
                } else {
                    cycle.cursor = 0;
                    cycle.phase = Phase::CrossRoots;
                }
            }
            Phase::CrossRoots => {
                let other = self.domain_at(1 - index);
                let next = if other.active() && cycle.kind != CollectionKind::Old {
                    if index == 0 {
                        match self.objects.scan_young(&mut cycle.young_cursor) {
                            crate::index::Scan::Entry(address, ()) => Some(address),
                            crate::index::Scan::Advance => return true,
                            crate::index::Scan::Busy => return false,
                            crate::index::Scan::Done => None,
                        }
                    } else {
                        match self.objects.scan(&mut cycle.object_cursor, |_| ()) {
                            crate::index::Scan::Entry(address, ()) => Some(address),
                            crate::index::Scan::Advance => return true,
                            crate::index::Scan::Busy => return false,
                            crate::index::Scan::Done => None,
                        }
                    }
                } else {
                    None
                };
                if let Some(address) = next {
                    let Some(objects) = self.objects.try_lock(address) else {
                        if index == 0 {
                            cycle.young_cursor.retry_entry();
                        } else {
                            cycle.object_cursor.retry_entry();
                        }
                        return false;
                    };
                    let metadata = objects
                        .get(&address)
                        .filter(|metadata| index != 0 || self.is_young_metadata(metadata))
                        .cloned();
                    drop(objects);
                    if let Some(metadata) = metadata
                        && let Some(retention) =
                            metadata.retention(1 - index, other.epoch.load(Ordering::Acquire))
                    {
                        let _retention = TraceRetention::enter(self, retention);
                        self.shade_known(&metadata);
                    }
                    cycle.cursor = address;
                } else {
                    cycle.cursor = 0;
                    cycle.phase = Phase::Remembered;
                }
            }
            Phase::Remembered => {
                // Promotions are remembered so younger children of survivors remain live.
                let next = if cycle.kind == CollectionKind::Young {
                    match self.remembered.scan(&mut cycle.remembered_cursor) {
                        crate::index::Scan::Entry(address, ()) => Some(address),
                        crate::index::Scan::Advance => return true,
                        crate::index::Scan::Busy => return false,
                        crate::index::Scan::Done => None,
                    }
                } else {
                    None
                };
                if let Some(address) = next {
                    let Some(objects) = self.objects.try_lock(address) else {
                        cycle.remembered_cursor.retry_entry();
                        return false;
                    };
                    let metadata = objects.get(&address).cloned();
                    drop(objects);
                    if let Some(metadata) = metadata {
                        self.shade_metadata(&metadata);
                    }
                    cycle.cursor = address;
                } else {
                    cycle.phase = Phase::Mark;
                    self.domain().trace_ready.store(true, Ordering::Release);
                }
            }
            _ => {}
        }
        true
    }
    #[expect(
        clippy::too_many_lines,
        reason = "keep one conditional-slot decision and its lock scope together"
    )]
    pub(super) fn reference_unit(&self, cycle: &mut Cycle, counters: &mut WorkCounters) -> Unit {
        let Some(_publication) = self.try_publication_write() else {
            return Unit::Busy;
        };
        let _admission = (self.configuration.independent_phases
            && self.publication_clock.is_some())
        .then(|| self.allocation_guard());
        if self.contexts.pending.load(Ordering::Acquire) != 0 {
            return Unit::Busy;
        }
        let index = self.domain_index();
        let slots = if cycle.kind == CollectionKind::Young {
            &self.young_weak_slots
        } else {
            &self.weak_slots
        };
        let next = lock(slots)
            .range((Excluded(cycle.cursor), Unbounded))
            .next()
            .map(|(id, slot)| (*id, slot.upgrade()));
        let Some((id, slot)) = next else {
            return Unit::Boundary;
        };
        let Some(slot) = slot else {
            cycle.cursor = id;
            lock(&self.weak_slots).remove(&id);
            lock(&self.young_weak_slots).remove(&id);
            return Unit::Progress;
        };
        if slot.unpublished_owners.load(Ordering::Acquire) != 0 {
            // Eligible graphs already have construction roots. This check must
            // precede the slot lock: owner binding itself can be waiting for it.
            // If construction finishes now, normal root deletion barriers keep
            // this snapshot safe; the reference remains indexed for a later pass.
            cycle.cursor = id;
            return Unit::Progress;
        }
        let Some(_decision) = slot.try_decision() else {
            return Unit::Busy;
        };
        if slot.unpublished_owners.load(Ordering::Acquire) != 0 {
            // Construction roots retain eligible referents and their graphs.
            // Leave the slot indexed for a later cycle, without holding this
            // cycle open for a hook that may be suspended indefinitely.
            cycle.cursor = id;
            return Unit::Progress;
        }
        cycle.cursor = id;
        counters.references += 1;
        let address = slot.target.load(Ordering::Acquire);
        let pending = slot.pending.load(Ordering::Acquire);
        if !pending && !self.is_young_allocation(address) {
            lock(&self.young_weak_slots).remove(&id);
        }
        if address == 0 && !pending {
            lock(&self.weak_slots).remove(&id);
            return Unit::Progress;
        }
        let owner = slot.owner.load(Ordering::Acquire);
        if cycle.reference_stage == 4 {
            // Final-reference tracing has now established reachability of
            // Reference subclasses as well. A dead Reference must never be
            // resurrected solely to deliver a queue notification.
            if slot.pending.swap(false, Ordering::AcqRel)
                && owner != 0
                && self.reference_owner_live(&slot)
            {
                self.enqueue_reference(&slot);
            }
            return Unit::Progress;
        }
        // Clearing applies even to Reference objects that will only become
        // reachable through finalization. Queue eligibility is decided after
        // that closure, independently of clearing the conditional edge.
        // Retaining edges, however, must not keep dead owners' graphs alive.
        if matches!(
            slot.kind,
            crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained
        ) && owner != 0
            && !self.reference_owner_live(&slot)
        {
            return Unit::Progress;
        }
        let relevant = match slot.kind {
            crate::ReferenceKind::Soft | crate::ReferenceKind::Weak => cycle.reference_stage == 1,
            crate::ReferenceKind::Final => cycle.reference_stage == 2,
            crate::ReferenceKind::Phantom => cycle.reference_stage == 3,
            crate::ReferenceKind::PhantomRetained => {
                cycle.reference_stage
                    == if slot.activated.load(Ordering::Acquire) {
                        2
                    } else {
                        3
                    }
            }
        };
        if relevant
            && slot.birth_stamp(index).load(Ordering::Acquire) != cycle.epoch
            && (!self.weak_target_is_registered(&slot, address)
                || !self.reference_live_in_cycle(address, slot.kind)
                || cycle.phantom_retained.contains(&address))
        {
            if matches!(
                slot.kind,
                crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained
            ) && self.weak_target_is_registered(&slot, address)
            {
                if slot.kind == crate::ReferenceKind::PhantomRetained && cycle.reference_stage == 3
                {
                    cycle.phantom_retained.insert(address);
                }
                self.shade(address);
            } else {
                slot.target.store(0, Ordering::Release);
            }
            if !slot.activated.swap(true, Ordering::AcqRel) && owner != 0 {
                if slot.kind == crate::ReferenceKind::Final {
                    self.enqueue_reference(&slot);
                } else {
                    slot.pending.store(true, Ordering::Release);
                }
            }
        }
        Unit::Progress
    }
    pub(super) fn sweep_unit(
        &self,
        cycle: &mut Cycle,
        work: &mut usize,
        counters: &mut WorkCounters,
    ) -> Unit {
        let scan_budget = (*work + 1).min(32);
        if let Some(task) = cycle.reclaim.pop_front() {
            let scan_budget = scan_budget.min(task.metadata.trace.budget);
            *work = work.saturating_sub(scan_budget.saturating_sub(1));
            self.retired_bytes
                .fetch_add(task.metadata.size(), Ordering::AcqRel);
            self.retired_ready.fetch_add(1, Ordering::AcqRel);
            return Unit::Sweep(SweepWork::Reclaim(task, scan_budget));
        }
        let Some(_publication) = self.try_publication_write() else {
            return Unit::Busy;
        };
        let _admission = (self.configuration.independent_phases
            && self.publication_clock.is_some())
        .then(|| self.allocation_guard());
        let limit = if self.configuration.max_pause_time_us == 0 {
            SWEEP_BATCH
        } else {
            1
        };
        for visit in 0..limit {
            if visit != 0 {
                if *work == 0 {
                    return Unit::Progress;
                }
                *work -= 1;
            }
            match self.sweep_one(cycle, work, counters) {
                Unit::Progress => {}
                Unit::Boundary if !cycle.reclaim.is_empty() => return Unit::Progress,
                next => return next,
            }
        }
        Unit::Progress
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep admission and transactional index removal in one scope"
    )]
    fn sweep_one(&self, cycle: &mut Cycle, work: &mut usize, counters: &mut WorkCounters) -> Unit {
        let index = self.domain_index();
        let scan_budget = (*work + 1).min(32);
        let mut sealed = SealedPages::default();
        let removed = {
            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                return Unit::Busy;
            }
            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                return Unit::Busy;
            };
            if !empty {
                return Unit::Boundary;
            }
            let retry = |cycle: &mut Cycle| {
                if cycle.kind == CollectionKind::Young {
                    cycle.sweep_young_cursor.retry_entry();
                } else {
                    cycle.sweep_object_cursor.retry_entry();
                }
                Unit::Busy
            };
            let selected = if cycle.kind == CollectionKind::Young {
                match self.objects.scan_young(&mut cycle.sweep_young_cursor) {
                    crate::index::Scan::Entry(address, ()) => {
                        let Some(member) = self.objects.try_lock(address) else {
                            return retry(cycle);
                        };
                        Some((address, member))
                    }
                    crate::index::Scan::Advance => return Unit::Progress,
                    crate::index::Scan::Busy => return Unit::Busy,
                    crate::index::Scan::Done => None,
                }
            } else {
                // Keep the deadline check between individual entries when a
                // time limit is active. Otherwise share one admission and
                // shard lookup across at most the remaining sweep budget.
                let limit = if self.configuration.max_pause_time_us == 0 {
                    scan_budget
                } else {
                    1
                };
                match self.objects.scan_sweep(
                    &mut cycle.sweep_object_cursor,
                    index,
                    cycle.epoch,
                    self.configuration.promotion_age.max(1),
                    limit,
                ) {
                    crate::index::Scan::Entry(
                        address,
                        crate::index::SweepEntry::Object(member),
                    ) => Some((address, member)),
                    crate::index::Scan::Entry(
                        _,
                        crate::index::SweepEntry::Survivors(survivors),
                    ) => {
                        *work -= survivors - 1;
                        return Unit::Progress;
                    }
                    crate::index::Scan::Advance => return Unit::Progress,
                    crate::index::Scan::Busy => return Unit::Busy,
                    crate::index::Scan::Done => None,
                }
            };
            let Some((address, objects)) = selected else {
                #[cfg(feature = "experimental-identity-index")]
                return if self.identities.cleanup_step(&mut cycle.identity_cleanup) {
                    Unit::Boundary
                } else {
                    Unit::Progress
                };
                #[cfg(not(feature = "experimental-identity-index"))]
                return Unit::Boundary;
            };
            if let Some(metadata) = objects.get(&address) {
                let previous = metadata.age.load(Ordering::Acquire);
                let promotion_age = self.configuration.promotion_age.max(1);
                // A reader may have selected an address just before promotion
                // or reuse. Revalidate the current live descriptor under its guard.
                if (cycle.kind == CollectionKind::Young && previous >= promotion_age)
                    || self.old_excludes_young(index, metadata)
                {
                    return Unit::Progress;
                }
                if metadata.marked(self.domain_index(), cycle.epoch) {
                    if self.configuration.overlapping_generations
                        && index == 1
                        && metadata.retention(index, cycle.epoch).is_none()
                    {
                        // This allocation did not survive the traced
                        // snapshot. Leave aging/promotion to young GC.
                        return Unit::Progress;
                    }
                    let age = previous.saturating_add(1).min(promotion_age);
                    if previous < age && age == promotion_age {
                        // Acquire every membership lock before changing age or
                        // cross-generation marks. A retry repeats no side effect.
                        let Some(mut remembered) = self.remembered.try_lock(address) else {
                            return retry(cycle);
                        };
                        self.promotion_snapshot(metadata);
                        metadata.age.store(age, Ordering::Release);
                        sealed.address = objects.promote(&mut cycle.young_retirement);
                        remembered.insert(address);
                        counters.promoted += 1;
                        drop(remembered);
                        self.publish_old_promotion(metadata);
                    } else {
                        metadata.age.store(age, Ordering::Release);
                    }
                    None
                } else {
                    // Registry removal commits only after every shard is held.
                    // In particular, an identity must remain resolvable when a
                    // contended remembered shard defers this retirement.
                    let Some(mut remembered) = self.remembered.try_lock(address) else {
                        return retry(cycle);
                    };
                    #[cfg(feature = "experimental-identity-index")]
                    {
                        // One list visit per unit. The sweep cursor preserves the
                        // selected address in each shard while this owning cursor
                        // advances; retries retain no index or publication guards.
                        let shard = crate::index::address_bucket(address)
                            & (crate::index::shard_count(self.configuration.metadata_shards) - 1);
                        let Some(cursor) = cycle.identity_removals.get_mut(shard) else {
                            return retry(cycle);
                        };
                        if self
                            .identities
                            .remove_step(metadata.id.to_i64().cast_unsigned(), address, cursor)
                            .is_none()
                        {
                            return retry(cycle);
                        }
                    }
                    #[cfg(not(feature = "experimental-identity-index"))]
                    {
                        sealed.identity = self.identities.retire(
                            metadata.id.to_i64().cast_unsigned(),
                            address,
                            &mut cycle.identity_cursor,
                        );
                    }
                    remembered.remove(address);
                    let (removed, address_page) = objects.retire(&mut cycle.young_retirement);
                    sealed.address = address_page;
                    removed
                }
            } else {
                None
            }
        };
        if let Some(metadata) = removed {
            if metadata.retire() {
                cycle.reclaim.push_back(ReclaimTask {
                    metadata,
                    offset: 0,
                    finalized: false,
                });
            } else {
                self.defer_retirement(metadata);
            }
        }
        sealed.into_unit()
    }
}
