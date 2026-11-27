use super::{
    CollectionKind, Context, GarbageCollector, MatureCoverage, Ordering, Phase, Unit, WorkCounters,
    lock, now, phase_work,
};

impl GarbageCollector {
    #[expect(clippy::too_many_lines, reason = "resumable phases share one budget")]
    pub(super) fn coordinated_step(&self, concurrent: bool) -> Option<bool> {
        let started = now();
        // Declared before the guards so the detached epoch outlives publication,
        // both cursor locks and the domain context, including on a failed start.
        // Publish its remaining disposal work after leaving those scopes.
        let mut detached_epoch = None;
        let Ok(coordinator) = self.coordinator.try_lock() else {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .coordinator_misses
                    .fetch_add(1, Ordering::Relaxed);
            }
            return None;
        };
        let coordinator = Some(coordinator);
        let _measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure(self.diagnostics_enabled());
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let preferred = self.next_domain.fetch_add(1, Ordering::Relaxed) % 2;
        let terminal =
            self.domains.iter().enumerate().position(|(index, domain)| {
                self.terminal_priority(index) && domain.weak_phase() != 0
            });
        let index = terminal.unwrap_or_else(|| {
            if self.domain_at(preferred).active()
                || self.domain_at(preferred).requested.load(Ordering::Acquire)
                || (preferred == 1 && self.old_requested.load(Ordering::Acquire))
            {
                preferred
            } else {
                1 - preferred
            }
        });
        let context = Context::enter(self, index);
        let mut cycle = self.domain().cycle.try_lock().ok()?;
        if cycle.phase == Phase::Idle {
            // An in-flight callback or another domain can keep the heap busy.
            // Only a real request needs to exclude publishers in this domain.
            if !(self.domain().requested.load(Ordering::Acquire)
                || index == 1 && self.old_requested.load(Ordering::Acquire))
            {
                return (!self.pending()).then_some(true);
            }
            // Drain an earlier young snapshot before starting a requested full
            // pass. Otherwise its floating roots can be imported into successive
            // full passes after every mutator has already dropped them. New young
            // work yields to that pending start, without suspending mutators.
            // Old-only handoffs additionally exclude young starts throughout
            // precise full tracing because their young retention is conservative.
            if index == 0
                && ((self.domains[1].requested.load(Ordering::Acquire)
                    && !self.old_collection_active())
                    || (self.configuration.old_collection_interval != 0
                        && self.domains[1].active()
                        && !self.old_collection_active()
                        && self.domains[1].weak_phase() < 5))
                || index == 1
                    && self.domain().requested.load(Ordering::Acquire)
                    && self.domains[0].active()
            {
                return Some(false);
            }
            if self
                .domain()
                .epoch
                .load(Ordering::Acquire)
                .checked_add(crate::metadata::EPOCH_STRIDE)
                .is_none()
            {
                // Keep the request pending while resetting one entry per unit.
                // Neither domain shades an inactive domain, so no old epoch
                // can alias epoch one when the reset finishes.
                self.contexts
                    .completed_disabled
                    .store(true, Ordering::Release);
                cycle.phase = Phase::ResetMarks;
                cycle.object_cursor = crate::index::Cursor::default();
            }
        }
        if cycle.phase == Phase::Idle {
            let Some(publication) = self.try_publication_write() else {
                return Some(false);
            };
            let Some(observation) = self.observe_phase() else {
                return Some(false);
            };
            let full_request = self.domain().requested.swap(false, Ordering::AcqRel);
            let old_request = index == 1 && self.old_requested.swap(false, Ordering::AcqRel);
            if !full_request && !old_request {
                // An in-flight destructor keeps pending() true, even after its
                // cycle ends. This worker did not claim that task or a request.
                return (!self.pending()).then_some(true);
            }
            let full = index == 1;
            #[cfg(not(target_family = "wasm"))]
            if full && full_request {
                self.domain().capture_full_cleanup(&mut cycle);
            }
            // Capture before publishing any part of the snapshot. A request
            // arriving after this cut leaves the flag set for the next start.
            let ticket = self.domain().request_ticket.load(Ordering::Acquire);
            cycle.kind = if full && !full_request {
                CollectionKind::Old
            } else if full {
                CollectionKind::Full
            } else {
                CollectionKind::Young
            };
            if full {
                self.old_active
                    .store(cycle.kind == CollectionKind::Old, Ordering::Release);
            }
            cycle.epoch =
                self.domain().epoch.load(Ordering::Acquire) + crate::metadata::EPOCH_STRIDE;
            self.domain().epoch.store(cycle.epoch, Ordering::Release);
            self.domain().mark_queue.open_graph_inbox(
                cycle.epoch,
                cycle.kind == CollectionKind::Old,
                self.this.clone(),
            );
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::phase_tests::before_snapshot_commit();
            if !self.commit_phase(observation, true, true, 0) {
                detached_epoch = self.domain().mark_queue.close_graph_inbox();
                if cycle.kind == CollectionKind::Old {
                    self.old_requested.store(true, Ordering::Release);
                } else {
                    self.domain().publish_request();
                }
                drop(publication);
                drop(cycle);
                drop(coordinator);
                drop(context);
                drop(detached_epoch);
                return Some(false);
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::phase_tests::after_snapshot_activation();
            if index == 1 {
                self.mature_census.strong.store(0, Ordering::Release);
                self.domain().mark_queue.reset_mature_strong();
                cycle.mature_coverage = MatureCoverage::Unchecked;
                cycle.packet_cleanup = None;
            }
            cycle.phase = if cycle.kind == CollectionKind::Old {
                Phase::OldHandoff
            } else if self.configuration.root_storage == crate::RootStorage::Contexts
                || self.configuration.local_allocation_buffers
            {
                Phase::ContextRoots
            } else if self.scanned_slots().is_some() {
                Phase::RootSlots
            } else {
                Phase::Roots
            };
            cycle.root_cursor = crate::roots::Cursor::default();
            cycle.context_cursor = crate::context::RootCursor::default();
            cycle.cursor = 0;
            cycle.reference_stage = 0;
            cycle.ticket = ticket;
            cycle.relocations.clear();
            cycle.relocation_scan_done = false;
            debug_assert!(cycle.phantom_retained.is_empty());
            cycle.young_cursor = crate::index::YoungCursor::default();
            cycle.remembered_cursor = crate::index::RememberedCursor::default();
            cycle.remembered_overflow = if cycle.kind == CollectionKind::Young {
                self.remembered.dirty.cursor()
            } else {
                crate::index::RememberedOverflow::default()
            };
            cycle.sweep_young_cursor = crate::index::YoungCursor::default();
            cycle.object_cursor = crate::index::Cursor::default();
            cycle.sweep_object_cursor = crate::index::Cursor::default();
            cycle.young_retirement = crate::index::YoungRetirement::default();
            #[cfg(feature = "experimental-identity-index")]
            {
                cycle.identity_removals =
                    std::array::from_fn(|_| crate::identity_index::Removal::default());
                cycle.identity_cleanup = crate::identity_index::Cleanup::default();
            }
            cycle.started = started;
            cycle.reclaimed_bytes = 0;
            cycle.payload_trim = crate::region::TrimCursor::default();
            cycle.descriptor_trim = crate::region::TrimCursor::default();
            let scheduler_start = self
                .scheduler
                .as_ref()
                .filter(|_| cycle.kind == CollectionKind::Full)
                .map(|scheduler| (scheduler, self.managed_budget.used()));
            self.domain().active.store(true, Ordering::Release);
            self.domain().marking.store(true, Ordering::Release);
            self.allocation_debt.store(0, Ordering::Release);
            self.published_debt.store(0, Ordering::Release);
            // The phase flags and both debts are visible to newly admitted
            // mutators. Keep the cycle serialized through its bookkeeping.
            drop(publication);
            if let Some((scheduler, live)) = scheduler_start {
                scheduler.started(live);
            }
            self.update_statistics(|stats| {
                stats.collections_started += 1;
                stats.last_collection_start = started;
            });
        }
        let budget = self.configuration.incremental_step_size.max(1);
        let mut counters = WorkCounters::default();
        let mut work = budget;
        'units: {
            let mut cycle = cycle;
            let mut coordinator = coordinator;
            while work != 0 {
                if self.configuration.independent_phases {
                    drop(coordinator.take());
                }
                work -= 1;
                // A soft deadline may defer subsequent work, but cannot prevent the
                // first unit after a scheduling delay or expensive phase setup.
                if work + 1 < budget
                    && let Some(started) = started
                    && self.configuration.max_pause_time_us != 0
                    && started.elapsed().as_micros()
                        >= u128::from(self.configuration.max_pause_time_us)
                {
                    break;
                }
                if matches!(
                    cycle.phase,
                    Phase::References | Phase::Sweep | Phase::Compact
                ) && self.contexts.pending.load(Ordering::Acquire) != 0
                {
                    break;
                }
                match cycle.phase {
                    Phase::Idle => break,
                    Phase::OldHandoff => {
                        if !self.old_handoff_unit(&mut cycle, &mut work) {
                            break;
                        }
                    }
                    Phase::OldReferences => {
                        self.old_reference_root_unit(&mut cycle);
                    }
                    Phase::ContextRoots => match self.context_root_unit(&mut cycle) {
                        phase_work::ContextUnit::Progress => {}
                        phase_work::ContextUnit::Cleanup(cleanup) => {
                            drop(cycle);
                            drop(coordinator);
                            cleanup.run(self);
                            break 'units;
                        }
                    },
                    Phase::ResetMarks
                    | Phase::ResetReferences
                    | Phase::RootSlots
                    | Phase::Roots
                    | Phase::CrossRoots
                    | Phase::Remembered => {
                        if !self.root_unit(&mut cycle) {
                            break;
                        }
                    }
                    Phase::Mark => {
                        let claim_admission = self
                            .configuration
                            .independent_phases
                            .then(|| self.allocation_guard());
                        let mut tasks = crate::work::Claim::default();
                        let mut remaining = work + 1;
                        let count = self.claim_mark_batch(&mut tasks, &mut remaining);
                        if count != 0 {
                            work = remaining;
                            // Reserve the whole batch before releasing the coordinator. Neither
                            // generation may finish marking while even an unexecuted task is local.
                            self.domain().in_flight.reserve(count);
                            let epoch = cycle.epoch;
                            drop(claim_admission);
                            drop(cycle);
                            drop(coordinator);
                            let stalled = self.trace_mark_batch(&mut tasks, started, &mut counters);
                            // Another worker can close the epoch as soon as the last
                            // reservation is released. Publish completed mark work first.
                            if counters.marked != 0 {
                                counters.publish(self);
                            }
                            self.domain().in_flight.release(count);
                            if stalled || work == 0 {
                                break 'units;
                            }
                            // A reservation protects publication of this batch. Re-enter
                            // only to claim further work in the same mark phase.
                            coordinator = if self.configuration.independent_phases {
                                None
                            } else if let Ok(guard) = self.coordinator.try_lock() {
                                Some(guard)
                            } else {
                                break 'units;
                            };
                            let Ok(next) = self.domain().cycle.try_lock() else {
                                break 'units;
                            };
                            cycle = next;
                            if cycle.epoch != epoch || cycle.phase != Phase::Mark {
                                break;
                            }
                        } else {
                            drop(claim_admission);
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            let Some(observation) = self.observe_phase() else {
                                break;
                            };
                            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                                break;
                            }
                            // Never wait for an unreserved claim while retaining
                            // the coordinator or the publication writer. The
                            // claim still excludes closure; this caller yields.
                            let _claims = if let Some(gate) = &self.mark_claims {
                                match gate.try_write() {
                                    Ok(guard) => Some(guard),
                                    Err(std::sync::TryLockError::Poisoned(error)) => {
                                        Some(error.into_inner())
                                    }
                                    Err(std::sync::TryLockError::WouldBlock) => break,
                                }
                            } else {
                                None
                            };
                            if self.closure_in_flight() != 0
                                || (index == 1 && self.domains[0].active())
                            {
                                break;
                            }
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            if empty {
                                // A clone can still enter the previous reference stage while
                                // its graph work is draining. Process those exact slots before
                                // advancing the logical retention floor.
                                let previous_stage = self.domain().weak_phase();
                                let revisit =
                                    self.domain().late_references.has_work(previous_stage);
                                let stage = if cycle.kind == CollectionKind::Old {
                                    5
                                } else if revisit {
                                    previous_stage
                                } else {
                                    cycle.reference_stage.clamp(1, 5)
                                };
                                if cycle.kind == CollectionKind::Old
                                    && previous_stage == 0
                                    && let Some(epoch) = self.domain().mark_queue.birth_epoch()
                                    && !epoch.mature_deletions.seal_if_empty()
                                {
                                    break;
                                }
                                if !self.commit_phase(observation, true, false, stage) {
                                    break;
                                }
                                self.domain().mark_queue.finish_strong_births();
                                self.domain().trace_ready.store(false, Ordering::Release);
                                self.domain().marking.store(false, Ordering::Release);
                                if stage >= 5 {
                                    self.domain().weak_phase.store(5, Ordering::Release);
                                    cycle.phase = Phase::Sweep;
                                } else {
                                    cycle.reference_stage = stage;
                                    self.domain().weak_phase.store(stage, Ordering::Release);
                                    cycle.cursor = if revisit { usize::MAX } else { 0 };
                                    cycle.phase = Phase::References;
                                }
                            }
                        }
                    }
                    Phase::References => match self.reference_unit(&mut cycle, &mut counters) {
                        Unit::Progress => {}
                        Unit::Busy => break,
                        Unit::External(work) => {
                            let epoch = cycle.epoch;
                            drop(cycle);
                            drop(coordinator);
                            work.run(self, index, epoch, &mut counters);
                            break 'units;
                        }
                        Unit::Boundary => {
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            let Some(observation) = self.observe_phase() else {
                                break;
                            };
                            // Sample after observing the clock. A registration between
                            // the empty scan and this observation may already have finished.
                            if self
                                .domain()
                                .late_references
                                .has_work(cycle.reference_stage)
                            {
                                continue;
                            }
                            if self.contexts.pending.load(Ordering::Acquire) != 0
                                || self.closure_in_flight() != 0
                            {
                                break;
                            }
                            let next_stage = cycle.reference_stage + 1;
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            let marking = next_stage == 3 || !empty;
                            let stage = if marking {
                                self.domain().weak_phase()
                            } else {
                                next_stage
                            };
                            if !self.commit_phase(observation, true, marking, stage) {
                                break;
                            }
                            cycle.cursor = 0;
                            cycle.reference_stage = next_stage;
                            self.domain().weak_phase.store(stage, Ordering::Release);
                            if marking {
                                self.domain().marking.store(true, Ordering::Release);
                                cycle.phase = Phase::Mark;
                                self.domain().trace_ready.store(true, Ordering::Release);
                            } else if cycle.reference_stage >= 5 {
                                self.domain().weak_phase.store(5, Ordering::Release);
                                cycle.phase = Phase::Sweep;
                            }
                        }
                    },
                    Phase::SelectRegions => {
                        if matches!(self.select_regions_unit(&mut cycle), Unit::Busy) {
                            break;
                        }
                    }
                    Phase::Compact => {
                        // Stage five can reopen marking, but it cannot revisit
                        // phantom discovery. Charge this cycle for its own set
                        // before taking the final publication writer.
                        if cycle.phantom_retained.pop_first().is_some() {
                            continue;
                        }
                        if let Some(heap) = &self.regions {
                            if let Some(queues) = &self.domain().relocations {
                                let window = self.configuration.threads.clamp(1, 16) * 16;
                                if !cycle.relocation_scan_done && queues.len() < window {
                                    match self.next_relocation(&mut cycle) {
                                        crate::index::Scan::Entry(_, metadata) => {
                                            queues.push(
                                                self.local_queue(),
                                                crate::relocation::Task {
                                                    metadata,
                                                    attempts: 0,
                                                },
                                            );
                                            continue;
                                        }
                                        crate::index::Scan::Advance => continue,
                                        crate::index::Scan::Busy => break,
                                        crate::index::Scan::Done => {
                                            cycle.relocation_scan_done = true;
                                        }
                                    }
                                }
                                if !cycle.relocation_scan_done {
                                    break;
                                }
                                // Claims cannot disappear between dequeue and reservation.
                                // Independent callbacks publish continuations before retiring flight.
                                let _claims = if let Some(gate) = &self.relocation_claims {
                                    match gate.try_write() {
                                        Ok(guard) => Some(guard),
                                        Err(std::sync::TryLockError::Poisoned(error)) => {
                                            Some(error.into_inner())
                                        }
                                        Err(std::sync::TryLockError::WouldBlock) => break,
                                    }
                                } else {
                                    None
                                };
                                if queues.len() != 0 || self.closure_in_flight() != 0 {
                                    break;
                                }
                                self.domain()
                                    .relocation_ready
                                    .store(false, Ordering::Release);
                            } else {
                                let claim_admission = self
                                    .configuration
                                    .independent_phases
                                    .then(|| self.allocation_guard());
                                let next = if let Some(task) = cycle.relocations.pop_front() {
                                    Some(task)
                                } else {
                                    match self.next_relocation(&mut cycle) {
                                        crate::index::Scan::Entry(_, metadata) => {
                                            Some((metadata, 0))
                                        }
                                        crate::index::Scan::Advance => continue,
                                        crate::index::Scan::Busy => break,
                                        crate::index::Scan::Done => None,
                                    }
                                };
                                if let Some((metadata, attempts)) = next {
                                    self.domain().in_flight.reserve(1);
                                    drop(claim_admission);
                                    drop(cycle);
                                    drop(coordinator);
                                    let repeat = self.relocate(&metadata, heap, attempts);
                                    coordinator = if self.configuration.independent_phases {
                                        None
                                    } else {
                                        Some(lock(&self.coordinator))
                                    };
                                    cycle = lock(&self.domain().cycle);
                                    if repeat {
                                        cycle
                                            .relocations
                                            .push_back((metadata, attempts.saturating_add(1)));
                                    }
                                    self.domain().in_flight.release(1);
                                    continue;
                                }
                                drop(claim_admission);
                                if self.closure_in_flight() != 0 {
                                    break;
                                }
                            }
                            if !cycle.payload_trim.trim_accessible(heap, 32) {
                                break;
                            }
                        }
                        if let Some(heap) = &self.descriptor_regions
                            && !cycle.descriptor_trim.trim_accessible(heap, 32)
                        {
                            break;
                        }
                        // Close publication before retiring this snapshot: no stale gray
                        // pointer may survive into a generation that can reclaim its target.
                        if coordinator.is_none() {
                            let Ok(guard) = self.coordinator.try_lock() else {
                                break;
                            };
                            coordinator = Some(guard);
                        }
                        let Some(publication) = self.try_publication_write() else {
                            break;
                        };
                        let Some(observation) = self.observe_phase() else {
                            break;
                        };
                        if self.contexts.pending.load(Ordering::Acquire) != 0 {
                            break;
                        }
                        let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                            break;
                        };
                        if !empty {
                            if !self.commit_phase(observation, true, true, 5) {
                                break;
                            }
                            if let Some(heap) = &self.regions
                                && (!self.configuration.overlapping_generations || index == 1)
                            {
                                heap.end_evacuation();
                            }
                            self.domain().marking.store(true, Ordering::Release);
                            cycle.phase = Phase::Mark;
                            self.domain().trace_ready.store(true, Ordering::Release);
                            continue;
                        }
                        if !self.commit_phase(observation, false, false, 0) {
                            break;
                        }
                        if let Some(heap) = &self.regions
                            && (!self.configuration.overlapping_generations || index == 1)
                        {
                            heap.end_evacuation();
                        }
                        #[cfg(test)]
                        super::publication_tests::complete_late_offers(&self.domain().mark_queue);
                        detached_epoch = self.domain().mark_queue.close_graph_inbox();
                        self.domain().active.store(false, Ordering::Release);
                        self.domain().marking.store(false, Ordering::Release);
                        self.domain().weak_phase.store(0, Ordering::Release);
                        #[cfg(not(feature = "experimental-identity-index"))]
                        {
                            cycle.identity_cursor = crate::identity_pages::RetireCursor::default();
                        }
                        cycle.phase = Phase::Idle;
                        #[cfg(all(
                            feature = "experimental-pause-recorder",
                            target_os = "linux",
                            target_has_atomic = "64"
                        ))]
                        if let Some(recorder) = &self.configuration.pause_recorder {
                            recorder.collection_completed();
                        }
                        // Roots and barriers may enter the inactive domain.
                        // The coordinator still orders metrics and ticket completion.
                        drop(publication);
                        self.record_cycle_completion(&cycle, &mut counters);
                        #[cfg(not(target_family = "wasm"))]
                        if cycle.kind == CollectionKind::Full {
                            cycle.active_full_cleanup = None;
                        }
                        if cycle.kind != CollectionKind::Old {
                            self.domain()
                                .completed_ticket
                                .store(cycle.ticket, Ordering::Release);
                        }
                        if index == 0 {
                            self.next_domain.store(1, Ordering::Relaxed);
                        }
                        break;
                    }
                    Phase::Sweep => match self.sweep_unit(&mut cycle, &mut work, &mut counters) {
                        Unit::Progress => {}
                        Unit::Busy => break,
                        Unit::External(external) => {
                            let epoch = cycle.epoch;
                            drop(cycle);
                            drop(coordinator);
                            external.run(self, index, epoch, &mut counters);
                            coordinator = if self.configuration.independent_phases {
                                None
                            } else {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break 'units;
                                };
                                Some(guard)
                            };
                            let Ok(next) = self.domain().cycle.try_lock() else {
                                break 'units;
                            };
                            cycle = next;
                            if cycle.epoch != epoch || cycle.phase != Phase::Sweep {
                                break 'units;
                            }
                        }
                        Unit::Boundary => {
                            if coordinator.is_none() {
                                let Ok(guard) = self.coordinator.try_lock() else {
                                    break;
                                };
                                coordinator = Some(guard);
                            }
                            let Some(_publication) = self.try_publication_write() else {
                                break;
                            };
                            if self.contexts.pending.load(Ordering::Acquire) != 0 {
                                break;
                            }
                            let Some(empty) = self.domain().mark_queue.try_is_empty() else {
                                break;
                            };
                            if !empty {
                                let Some(observation) = self.observe_phase() else {
                                    break;
                                };
                                if !self.commit_phase(observation, true, true, 5) {
                                    break;
                                }
                                self.domain().marking.store(true, Ordering::Release);
                                cycle.phase = Phase::Mark;
                                self.domain().trace_ready.store(true, Ordering::Release);
                                continue;
                            }
                            cycle.phase = Phase::Compact;
                            // A full cycle enumerates every payload in a selected region.
                            // Young-only relocation retains the existing packing policy.
                            if cycle.kind != CollectionKind::Young
                                && self
                                    .regions
                                    .as_ref()
                                    .is_some_and(|heap| heap.begin_evacuation())
                            {
                                cycle.phase = Phase::SelectRegions;
                            }
                            cycle.relocation_scan_done = false;
                            self.domain().relocation_ready.store(
                                cycle.phase == Phase::Compact
                                    && self.regions.is_some()
                                    && self.configuration.independent_relocation,
                                Ordering::Release,
                            );
                            cycle.cursor = 0;
                            cycle.object_cursor = crate::index::Cursor::default();
                            cycle.young_cursor = crate::index::YoungCursor::default();
                            break;
                        }
                    },
                }
            }
        }
        drop(context);
        drop(detached_epoch);
        self.record_step(started, cpu_started, concurrent, &mut counters);
        let idle = !self.pending();
        self.notify_progress();
        Some(idle)
    }
}
