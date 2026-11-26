#[cfg(test)]
use super::lock;
use super::{Context, GarbageCollector, Ordering, RetirementStep, WorkCounters, now};

impl GarbageCollector {
    /// Performs at most one configured batch. Returns true when no collection is pending.
    ///
    /// The work budget bounds visits, not arbitrary user `Trace` or destructor code. Implement
    /// `Trace::trace_step` for large objects and avoid blocking in tracing/destruction.
    pub fn step(&self) -> bool {
        self.step_inner(false).unwrap_or(false)
    }

    pub(super) fn claim_mark_batch(
        &self,
        claim: &mut crate::work::Claim,
        remaining: &mut usize,
    ) -> usize {
        self.domain().mark_queue.claim_into(
            self.local_queue(),
            self.configuration.mark_batch_size,
            remaining,
            claim,
        );
        #[cfg(test)]
        if claim.count() != 0 {
            let pause = lock(&self.mark_claim_pause).take();
            if let Some(pause) = pause {
                pause.entered.send(()).expect("claim entered");
                pause
                    .resume
                    .recv_timeout(std::time::Duration::from_secs(10))
                    .expect("resume claim");
            }
        }
        if self.diagnostics_enabled() {
            if claim.stolen != 0 {
                self.diagnostics
                    .mark_stolen
                    .fetch_add(claim.stolen as u64, Ordering::Relaxed);
            }
            if claim.misses != 0 {
                self.diagnostics
                    .mark_queue_misses
                    .fetch_add(claim.misses as u64, Ordering::Relaxed);
            }
        }
        claim.count()
    }

    /// A successful independent step never reacquires the coordinator. Reservations
    /// remain outstanding until all continuations and statistics have been published.
    #[cfg(test)]
    pub(super) fn independent_mark_step(&self, concurrent: bool) -> Option<bool> {
        self.claim_independent_mark(concurrent, &mut false)
    }

    pub(super) fn claim_independent_mark(
        &self,
        concurrent: bool,
        reserved_miss: &mut bool,
    ) -> Option<bool> {
        // A new publication can race this hint; the coordinated fallback and
        // the next step still inspect the authoritative phase and work queues.
        if self
            .domains
            .iter()
            .all(|domain| !domain.trace_ready.load(Ordering::Acquire))
        {
            return None;
        }
        let gate = self.mark_claims.as_ref()?;
        let started = now();
        let Ok(claim) = gate.try_read() else {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .mark_gate_misses
                    .fetch_add(1, Ordering::Relaxed);
            }
            return None;
        };
        // Phase enumeration and newly requested generations need coordinator turns,
        // even when another generation always has a resumable tracing task ready.
        if self.old_requested.load(Ordering::Acquire) && !self.domains[1].active()
            || self.domains.iter().any(|domain| {
                let active = domain.active();
                (!active && domain.requested.load(Ordering::Acquire))
                    || (active
                        && !domain.trace_ready.load(Ordering::Acquire)
                        && domain.weak_phase() == 0)
            })
        {
            return None;
        }
        // A failed fast claim must not advance this hint: the fallback needs to
        // close the empty generation, not alternate back to the busy one again.
        let preferred = self.next_domain.load(Ordering::Relaxed) % 2;
        let terminal =
            self.domains.iter().enumerate().position(|(index, domain)| {
                self.terminal_priority(index) && domain.weak_phase() != 0
            });
        let index = terminal.unwrap_or_else(|| {
            if self
                .domain_at(preferred)
                .trace_ready
                .load(Ordering::Acquire)
            {
                preferred
            } else {
                1 - preferred
            }
        });
        if !self.domain_at(index).trace_ready.load(Ordering::Acquire) {
            return None;
        }
        let context = Context::enter(self, index);
        let mut remaining = self.configuration.incremental_step_size.max(1);
        let mut tasks = crate::work::Claim::default();
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let count = self.claim_mark_batch(&mut tasks, &mut remaining);
        if count == 0 {
            // There is already a live reservation in precisely the generation
            // we just probed. A coordinator cannot close it. A new request or
            // published continuation is reconsidered by the next helper call.
            *reserved_miss =
                self.domain().in_flight.count() != 0 && !self.domain_has_work(1 - index);
            return None;
        }
        self.next_domain.fetch_add(1, Ordering::Relaxed);
        self.domain().in_flight.reserve(count);
        let mut reserved = count;
        drop(claim);
        let _measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        let mut counters = WorkCounters::default();
        loop {
            let stalled = self.trace_mark_batch(&mut tasks, started, &mut counters);
            if stalled || remaining == 0 {
                break;
            }
            if let Some(started) = started
                && self.configuration.max_pause_time_us != 0
                && started.elapsed().as_micros() >= u128::from(self.configuration.max_pause_time_us)
            {
                break;
            }
            tasks.stolen = 0;
            tasks.misses = 0;
            // A reservation from this step is still outstanding. The interlock means neither generation can
            // close marking, so further claims need no phase lock or epoch recheck.
            let count = self.claim_mark_batch(&mut tasks, &mut remaining);
            if count == 0 {
                break;
            }
            self.domain().in_flight.reserve(count);
            reserved += count;
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .independent_mark_steps
                .fetch_add(1, Ordering::Relaxed);
        }
        self.record_step(started, cpu_started, concurrent, &mut counters);
        // Release last: completion tickets must never precede this step's counters.
        self.domain().in_flight.release(reserved);
        drop(context);
        self.notify_progress();
        Some(!self.pending())
    }

    pub(super) fn step_inner(&self, concurrent: bool) -> Option<bool> {
        if self.remembered.dirty.cleanup_pending() {
            self.remembered.dirty.cleanup_step(|id, address| {
                self.objects
                    .lookup(address)
                    .is_some_and(|metadata| metadata.id.to_i64().cast_unsigned() == id)
            });
        }
        let global_repair = self
            .scanned_slots()
            .is_some_and(crate::roots::RootSlots::repair_step);
        let context_repair = self.contexts.repairs.step();
        if (global_repair || context_repair)
            && self.retired_ready.load(Ordering::Acquire) == 0
            && !self.released_donors.has_jobs()
            && !self.domain_has_work(0)
            && !self.domain_has_work(1)
            && self.contexts.pending.load(Ordering::Acquire) == 0
        {
            return Some(!self.pending());
        }
        // A delayed borrower cannot hold a collection ticket open. Once released,
        // give its bounded clearing work alternating turns with active cycles.
        if ((self.retired_objects.load(Ordering::Acquire) != 0
            && self.retired_ready.load(Ordering::Acquire) != 0)
            || self.released_donors.has_jobs())
            && (!self.retirement_turn.fetch_xor(true, Ordering::Relaxed)
                || !self
                    .domains
                    .iter()
                    .any(|domain| domain.active() || domain.requested.load(Ordering::Acquire)))
            && let Some(step) = self.reap_retired(concurrent, None)
        {
            return match step {
                RetirementStep::Progress(done) => Some(done),
                RetirementStep::Contended => None,
            };
        }
        // Alternate drains with tracing/phase work so a replenished buffer cannot
        // starve marking. Busy claims rotate without dropping their reservation.
        if self.contexts.pending.load(Ordering::Acquire) != 0
            && !self.barrier_turn.fetch_xor(true, Ordering::Relaxed)
            && let Some(progress) = self.drain_barrier_step(concurrent)
        {
            return Some(progress);
        }
        let mut waiting = false;
        let result = if let Some(progress) = self.claim_independent_mark(concurrent, &mut waiting) {
            Some(progress)
        } else if waiting {
            None
        } else {
            self.independent_relocation_step(concurrent).or_else(|| {
                match self.independent_phase_step(concurrent) {
                    Ok(Some(progress)) => Some(progress),
                    Err(index) if !self.domain_has_work(1 - index) => None,
                    _ => self.coordinated_step(concurrent),
                }
            })
        };
        // A producer may park after linking while still holding the coordinator.
        // Another ordinary worker or caller can deliver that visible notification.
        if result.is_none() && self.has_pending_references() {
            self.notify_progress();
        }
        result
    }
}
