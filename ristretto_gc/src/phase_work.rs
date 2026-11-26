//! Bounded per-domain work; only global phase transitions need the coordinator.
use super::{
    CollectionKind, Context, Cycle, GarbageCollector, MatureCoverage, MetadataRef, Ordering, Phase,
    ReclaimTask, TraceRetention, WorkCounters, now,
};

type WhiteRun<'a> = (
    crate::index::Membership<'a>,
    crate::index::RememberedGuard<'a>,
);

pub(super) const SWEEP_BATCH: usize = 16;

pub(super) enum Unit {
    Progress,
    Busy,
    Boundary,
    Sweep(SweepWork),
}

pub(super) enum SweepWork {
    ReleasePacket(crate::work::Packet),
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
            Self::ReleasePacket(packet) => drop(packet),
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
    Cleanup(crate::context::ContextCleanup),
}

impl GarbageCollector {
    #[expect(
        clippy::too_many_lines,
        reason = "one budget for independent domain units"
    )]
    pub(super) fn independent_phase_step(&self, concurrent: bool) -> Result<Option<bool>, usize> {
        if !self.configuration.independent_phases {
            return Ok(None);
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
        let cycle = self.domain().cycle.try_lock().map_err(|_| index)?;
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
            return Ok(None);
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
        Ok(Some(!self.pending()))
    }

    pub(super) fn context_root_unit(&self, cycle: &mut Cycle) -> ContextUnit {
        let _admission = self
            .configuration
            .independent_phases
            .then(|| self.allocation_guard());
        match self.contexts.scan(&mut cycle.context_cursor) {
            crate::context::RootScan::Root(address) => {
                self.shade_registered_root(address);
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
                if let Some((id, entry)) = self.weak_slots.next(false, cycle.cursor) {
                    cycle.cursor = id;
                    if let Some(slot) = entry.upgrade()
                        && slot.visible()
                    {
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
                        self.shade_registered_root(address);
                    }
                    Some(crate::roots::Scan::Advance) => {}
                    Some(crate::roots::Scan::Done) | None => cycle.phase = Phase::Roots,
                }
            }
            Phase::Roots => match self.roots.scan(cycle.cursor) {
                crate::roots::TreeScan::Root(id, address) => {
                    self.shade_registered_root(address);
                    cycle.cursor = id;
                }
                crate::roots::TreeScan::Retry => return false,
                crate::roots::TreeScan::Advance(id) => cycle.cursor = id,
                crate::roots::TreeScan::Done => {
                    cycle.cursor = 0;
                    cycle.phase = Phase::CrossRoots;
                }
            },
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
                        && let Some(retention) = self.visible_retention(
                            &metadata,
                            1 - index,
                            other.epoch.load(Ordering::Acquire),
                        )
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
                if let Some((id, address)) =
                    self.remembered.dirty.next(&mut cycle.remembered_overflow)
                {
                    if let Some(metadata) = self.objects.lookup(address)
                        && metadata.id.to_i64().cast_unsigned() == id
                    {
                        self.shade_metadata(&metadata);
                    }
                    return true;
                }
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
        let late = &self.domain().late_references;
        let young = cycle.kind == CollectionKind::Young;
        // Alternate so repeated late registrations cannot starve the range scan.
        cycle.late_reference_turn = !cycle.late_reference_turn;
        let mut take_late = || late.next(cycle.reference_stage, &mut cycle.retry_reference);
        let next = if cycle.late_reference_turn {
            take_late().map(|e| (e, true))
        } else {
            None
        }
        .or_else(|| {
            self.weak_slots
                .next(young, cycle.cursor)
                .map(|e| (e, false))
        })
        .or_else(|| take_late().map(|e| (e, true)));
        let Some(((id, entry), extra)) = next else {
            return Unit::Boundary;
        };
        let result = self.decide_reference(cycle, counters, id, &entry);
        if matches!(result, Unit::Busy) {
            if extra {
                cycle.retry_reference = Some((id, entry));
            }
        } else if !extra {
            cycle.cursor = id;
        }
        result
    }

    fn decide_reference(
        &self,
        cycle: &mut Cycle,
        counters: &mut WorkCounters,
        id: usize,
        entry: &std::sync::Weak<crate::weak::WeakSlot>,
    ) -> Unit {
        let index = self.domain_index();
        let Some(slot) = entry.upgrade() else {
            self.weak_slots.prune(id, entry, false);
            return Unit::Progress;
        };
        // The private publisher will sample both domains again if this scan
        // passes it. Keep both entries and leave its birth stamps untouched.
        if !slot.visible() {
            return Unit::Progress;
        }
        if slot.unpublished_owners.load(Ordering::Acquire) != 0 {
            // Eligible graphs already have construction roots. This check must
            // precede the slot lock: owner binding itself can be waiting for it.
            // If construction finishes now, normal root deletion barriers keep
            // this snapshot safe; the reference remains indexed for a later pass.
            return Unit::Progress;
        }
        let Some(_decision) = slot.try_decision() else {
            return Unit::Busy;
        };
        if slot.unpublished_owners.load(Ordering::Acquire) != 0 {
            // Construction roots retain eligible referents and their graphs.
            // Leave the slot indexed for a later cycle, without holding this
            // cycle open for a hook that may be suspended indefinitely.
            return Unit::Progress;
        }
        counters.references += 1;
        let address = slot.target.load(Ordering::Acquire);
        let pending = slot.pending.load(Ordering::Acquire);
        if address == 0 && !pending {
            self.weak_slots.prune(id, entry, false);
            return Unit::Progress;
        }
        // Reading an owning descriptor grants no access to its payload. A
        // parked object-shard holder cannot stall the weak/soft decision while
        // the coordinator owns publication. An address replacement is a miss.
        let target = self.weak_target_metadata(&slot, address);
        if !pending && target.as_ref().is_none_or(|m| !self.is_young_metadata(m)) {
            self.weak_slots.prune(id, entry, true);
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
            && (target
                .as_ref()
                .is_none_or(|m| !self.reference_metadata_live(m, slot.kind))
                || cycle.phantom_retained.contains(&address))
        {
            if matches!(
                slot.kind,
                crate::ReferenceKind::Final | crate::ReferenceKind::PhantomRetained
            ) && let Some(metadata) = &target
            {
                if slot.kind == crate::ReferenceKind::PhantomRetained && cycle.reference_stage == 3
                {
                    cycle.phantom_retained.insert(address);
                }
                self.shade_known(metadata);
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
            self.retired_ready.fetch_add(1, Ordering::AcqRel);
            return Unit::Sweep(SweepWork::Reclaim(task, scan_budget));
        }
        let Some(_publication) = self.try_publication_write() else {
            return Unit::Busy;
        };
        let _admission = (self.configuration.independent_phases
            && self.publication_clock.is_some())
        .then(|| self.allocation_guard());
        if matches!(cycle.mature_coverage, MatureCoverage::Unchecked) {
            cycle.mature_coverage = if cycle.kind == CollectionKind::Full
                && !self.configuration.overlapping_generations
                && self.configuration.old_collection_interval == 0
            {
                self.mature_census.coverage()
            } else {
                MatureCoverage::Incomplete
            };
        }
        let limit = if self.configuration.max_pause_time_us == 0 {
            SWEEP_BATCH
        } else {
            1
        };
        let mut run = None;
        let mut visits = limit;
        while visits != 0 {
            if visits != limit {
                if *work == 0 {
                    return Unit::Progress;
                }
                *work -= 1;
            }
            visits -= 1;
            match self.sweep_one(cycle, work, counters, &mut run, &mut visits) {
                Unit::Progress => {}
                Unit::Boundary if !cycle.reclaim.is_empty() => return Unit::Progress,
                next => return next,
            }
        }
        Unit::Progress
    }

    fn check_sweep_mark_work(&self) -> Option<Unit> {
        match self.domain().mark_queue.try_is_empty() {
            Some(true) => None,
            Some(false) => Some(Unit::Boundary),
            None => Some(Unit::Busy),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep admission and transactional index removal in one scope"
    )]
    fn sweep_one<'gc>(
        &'gc self,
        cycle: &mut Cycle,
        work: &mut usize,
        counters: &mut WorkCounters,
        run: &mut Option<WhiteRun<'gc>>,
        visits: &mut usize,
    ) -> Unit {
        let index = self.domain_index();
        let mut sealed = SealedPages::default();
        let removed = {
            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                return Unit::Busy;
            }
            let full_scan =
                cycle.kind == CollectionKind::Full && !self.configuration.overlapping_generations;
            if !full_scan && let Some(unit) = self.check_sweep_mark_work() {
                return unit;
            }
            let retry = |cycle: &mut Cycle| {
                if cycle.kind == CollectionKind::Young {
                    cycle.sweep_young_cursor.retry_entry();
                } else {
                    cycle.sweep_object_cursor.retry_entry();
                }
                Unit::Busy
            };
            #[cfg(not(feature = "experimental-identity-index"))]
            if let Some((member, remembered)) = run.as_mut() {
                let mut retired: [Option<MetadataRef>; SWEEP_BATCH] = std::array::from_fn(|_| None);
                let limit = (*work).min(*visits) + 1;
                let mut deferred = None;
                let (count, page) = self.objects.take_white_word(
                    member,
                    &mut cycle.sweep_object_cursor,
                    cycle.epoch,
                    &mut retired[..limit],
                    |address, metadata| {
                        if sealed.identity.is_some() {
                            return false;
                        }
                        if self.contexts.pending.load(Ordering::Acquire) != 0 {
                            deferred = Some(Unit::Busy);
                            return false;
                        }
                        deferred = self.check_sweep_mark_work();
                        if deferred.is_some() {
                            return false;
                        }
                        sealed.identity = self.identities.retire(
                            metadata.id.to_i64().cast_unsigned(),
                            address,
                            &mut cycle.identity_cursor,
                        );
                        remembered.remove(address);
                        true
                    },
                );
                if count != 0 {
                    *work -= count - 1;
                    *visits -= count - 1;
                    sealed.address = page;
                    if sealed.address.is_some() || deferred.is_some() {
                        drop(run.take());
                    }
                    for metadata in retired.into_iter().flatten() {
                        self.enqueue_swept(metadata, cycle, run);
                    }
                    // A deferred candidate has no effects and remains next. A
                    // structural cleanup from this prefix must be handed out.
                    return if sealed.address.is_some() || sealed.identity.is_some() {
                        sealed.into_unit()
                    } else {
                        deferred.unwrap_or(Unit::Progress)
                    };
                }
                if let Some(unit) = deferred {
                    return unit;
                }
            }
            let mut retained = None;
            let consecutive = run.take().and_then(|(mut member, remembered)| {
                let address = self.objects.next_white(
                    &mut member,
                    &mut cycle.sweep_object_cursor,
                    index,
                    cycle.epoch,
                )?;
                retained = Some(remembered);
                Some((address, member))
            });
            let selected = if consecutive.is_some() {
                consecutive
            } else if cycle.kind == CollectionKind::Young {
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
                    (*work + 1).min(64)
                } else {
                    1
                };
                match self.objects.scan_sweep(
                    &mut cycle.sweep_object_cursor,
                    index,
                    cycle.epoch,
                    self.configuration.promotion_age.max(1),
                    limit,
                    || self.mature_census.current(cycle.mature_coverage),
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
            if full_scan && let Some(unit) = self.check_sweep_mark_work() {
                if selected.is_some() {
                    cycle.sweep_object_cursor.retry_entry();
                }
                return unit;
            }
            let Some((address, mut objects)) = selected else {
                if let Some(cursor) = &mut cycle.packet_cleanup {
                    match self.domain_at(1).mark_queue.take_cache_entry(cursor) {
                        crate::index::Scan::Entry(_, packet) => {
                            return Unit::Sweep(SweepWork::ReleasePacket(packet));
                        }
                        crate::index::Scan::Advance => return Unit::Progress,
                        crate::index::Scan::Done => cycle.packet_cleanup = None,
                        crate::index::Scan::Busy => return Unit::Busy,
                    }
                }
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
                        // Overcount while promotion is in progress. A scanner
                        // can never certify an uncounted newly mature object.
                        self.mature_census.promotions.fetch_add(1, Ordering::AcqRel);
                        metadata.age.store(age, Ordering::Release);
                        let promoted = metadata.clone();
                        sealed.address = objects.promote(&mut cycle.young_retirement);
                        remembered.insert(address);
                        counters.promoted += 1;
                        drop(remembered);
                        self.publish_old_promotion(&promoted);
                    } else {
                        metadata.age.store(age, Ordering::Release);
                    }
                    None
                } else {
                    // Registry removal commits only after every shard is held.
                    // In particular, an identity must remain resolvable when a
                    // contended remembered shard defers this retirement.
                    let Some(mut remembered) =
                        retained.or_else(|| self.remembered.try_lock(address))
                    else {
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
                    let was_mature = previous == promotion_age;
                    let (removed, address_page) = objects.retire(&mut cycle.young_retirement);
                    if was_mature && removed.is_some() {
                        cycle.packet_cleanup = Some(0);
                        self.mature_census
                            .retirements
                            .fetch_add(1, Ordering::AcqRel);
                    }
                    sealed.address = address_page;
                    if full_scan && sealed.address.is_none() {
                        *run = Some((objects, remembered));
                    }
                    removed
                }
            } else {
                None
            }
        };
        if let Some(metadata) = removed {
            self.enqueue_swept(metadata, cycle, run);
        }
        sealed.into_unit()
    }

    fn enqueue_swept(
        &self,
        metadata: MetadataRef,
        cycle: &mut Cycle,
        run: &mut Option<WhiteRun<'_>>,
    ) {
        self.remembered
            .dirty
            .retire(metadata.id.to_i64().cast_unsigned());
        // The byte debt belongs to the retired descriptor, independent of
        // which queue or callback currently owns its clearing progress.
        self.retired_bytes
            .fetch_add(metadata.size(), Ordering::AcqRel);
        if metadata.retire() {
            cycle.reclaim.push_back(ReclaimTask {
                metadata,
                offset: 0,
                finalized: false,
            });
        } else {
            // Returning the retiring borrow may publish external clearing
            // work. Do not retain page or remembered guards for that handoff.
            drop(run.take());
            self.defer_retirement(metadata);
        }
    }
}
