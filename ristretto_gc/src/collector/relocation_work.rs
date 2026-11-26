use super::{
    Context, Cycle, GarbageCollector, MetadataRef, ObjectMetadata, Ordering, TraceProgress,
    WorkCounters, now,
};

impl GarbageCollector {
    pub(super) fn next_relocation(&self, cycle: &mut Cycle) -> crate::index::Scan<MetadataRef> {
        if self.domain_index() != 0 {
            return match self
                .objects
                .scan(&mut cycle.object_cursor, MetadataRef::clone)
            {
                crate::index::Scan::Entry(_, metadata)
                    if self.old_excludes_young(1, &metadata)
                        || self.configuration.overlapping_generations
                            && metadata.retention(1, cycle.epoch).is_none() =>
                {
                    crate::index::Scan::Advance
                }
                next => next,
            };
        }
        let address = match self.objects.scan_young(&mut cycle.young_cursor) {
            crate::index::Scan::Entry(address, ()) => address,
            crate::index::Scan::Advance => return crate::index::Scan::Advance,
            crate::index::Scan::Busy => return crate::index::Scan::Busy,
            crate::index::Scan::Done => return crate::index::Scan::Done,
        };
        let Some(objects) = self.objects.try_lock(address) else {
            cycle.young_cursor.retry_entry();
            return crate::index::Scan::Busy;
        };
        objects
            .get(&address)
            .filter(|metadata| self.is_young_metadata(metadata))
            .cloned()
            .map_or(crate::index::Scan::Advance, |metadata| {
                crate::index::Scan::Entry(address, metadata)
            })
    }

    pub(super) fn relocate(
        &self,
        metadata: &MetadataRef,
        heap: &crate::region::RegionHeap,
        attempts: usize,
    ) -> bool {
        if self.old_excludes_young(self.domain_index(), metadata)
            || self.configuration.overlapping_generations
                && (self.logically_retired(metadata)
                    || (self.domain_index() == 1
                        && metadata
                            .retention(1, self.domains[1].epoch.load(Ordering::Acquire))
                            .is_none()))
        {
            return false;
        }
        // Old-only copying excludes remembered scans on this descriptor.
        let Ok(_lease) = ObjectMetadata::borrow_callback(metadata, self.old_collection_active())
        else {
            return false;
        };
        let bytes = self.configuration.relocation_chunk_bytes;
        let limit = metadata
            .size()
            .div_ceil(bytes.max(1))
            .saturating_mul(4)
            .max(4);
        let progress = metadata.trace.compact_guarded(heap, bytes);
        if let Some(history) = &metadata.relocation {
            history.observe(
                progress,
                metadata.trace.ptr,
                heap,
                self.diagnostics_enabled()
                    .then_some(&self.diagnostics.relocation_pin_deferral),
            );
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .relocation_tasks
                .fetch_add(1, Ordering::Relaxed);
            if progress == TraceProgress::Busy {
                self.diagnostics
                    .relocation_busy
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
        if attempts.saturating_add(1) >= limit && progress != TraceProgress::Complete {
            if self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_retry_limits
                    .fetch_add(1, Ordering::Relaxed);
            }
            let cancelled = metadata.trace.compact_guarded(heap, 0);
            if cancelled == TraceProgress::Busy && self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_cancel_busy
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
        matches!(progress, TraceProgress::Continue(_)) && attempts.saturating_add(1) < limit
    }

    pub(super) fn relocation_domain(&self) -> Option<usize> {
        if self.configuration.overlapping_generations {
            let preferred = self.next_domain.load(Ordering::Relaxed) % 2;
            let young = &self.domains[0];
            if preferred == 0
                && (young.active() || young.requested.load(Ordering::Acquire))
                && !young.relocation_ready.load(Ordering::Acquire)
            {
                // Roots, mark closure and reference/sweep units need coordinator
                // turns even while the full generation has a persistent copy queue.
                return None;
            }
            if self
                .domain_at(preferred)
                .relocation_ready
                .load(Ordering::Acquire)
            {
                Some(preferred)
            } else if self
                .domain_at(1 - preferred)
                .relocation_ready
                .load(Ordering::Acquire)
            {
                Some(1 - preferred)
            } else {
                None
            }
        } else {
            self.domains
                .iter()
                .position(|domain| domain.relocation_ready.load(Ordering::Acquire))
        }
    }

    pub(super) fn independent_relocation_step(&self, concurrent: bool) -> Option<bool> {
        let heap = self.regions.as_ref()?;
        let gate = self.relocation_claims.as_ref()?;
        let started = now();
        let claim = gate.try_read().ok()?;
        let index = self.relocation_domain()?;
        let context = Context::enter(self, index);
        let queues = self.domain().relocations.as_ref()?;
        let (task, stolen, misses) = queues.claim(self.local_queue());
        if self.diagnostics_enabled() {
            self.diagnostics
                .relocation_stolen
                .fetch_add(stolen as u64, Ordering::Relaxed);
            self.diagnostics
                .relocation_queue_misses
                .fetch_add(misses as u64, Ordering::Relaxed);
        }
        let mut task = task?;
        if self.configuration.overlapping_generations {
            self.next_domain.fetch_add(1, Ordering::Relaxed);
        }
        self.domain().in_flight.reserve(1);
        drop(claim);
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let measurement = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        }
        .measure_from(self.diagnostics_enabled(), started);
        // Keep the reservation through the whole batch, including subsequent
        // dequeue/publish gaps. Closure cannot retire this generation while
        // the batch owns work, even after we release the claim reader.
        for work in 0..self.configuration.incremental_step_size.max(1) {
            if self.relocate(&task.metadata, heap, task.attempts) {
                queues.push(
                    self.local_queue(),
                    crate::relocation::Task {
                        metadata: task.metadata,
                        attempts: task.attempts.saturating_add(1),
                    },
                );
            }
            if work + 1 == self.configuration.incremental_step_size.max(1)
                || (self.configuration.max_pause_time_us != 0
                    && started.is_some_and(|started| {
                        started.elapsed().as_micros()
                            >= u128::from(self.configuration.max_pause_time_us)
                    }))
            {
                break;
            }
            let (next, stolen, misses) = queues.claim(self.local_queue());
            if self.diagnostics_enabled() {
                self.diagnostics
                    .relocation_stolen
                    .fetch_add(stolen as u64, Ordering::Relaxed);
                self.diagnostics
                    .relocation_queue_misses
                    .fetch_add(misses as u64, Ordering::Relaxed);
            }
            let Some(next) = next else { break };
            task = next;
        }
        if self.diagnostics_enabled() {
            self.diagnostics
                .independent_relocation_steps
                .fetch_add(1, Ordering::Relaxed);
        }
        self.record_step(
            started,
            cpu_started,
            concurrent,
            &mut WorkCounters::default(),
        );
        self.domain().in_flight.release(1);
        drop(measurement);
        drop(context);
        self.notify_progress();
        Some(!self.pending())
    }
}
