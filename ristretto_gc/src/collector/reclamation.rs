use super::{
    Arc, CleanupScope, CollectionKind, GarbageCollector, MetadataRef, ObjectMetadata, Ordering,
    Phase, ReclaimStage, ReclaimTask, RetirementStep, TraceProgress, WorkCounters, lock, now,
    phase_work,
};
#[cfg(not(target_family = "wasm"))]
use super::{WORKER, WorkerSignals, native_callbacks};

impl GarbageCollector {
    pub(super) fn defer_retirement(&self, metadata: MetadataRef) {
        self.retired_objects.fetch_add(1, Ordering::AcqRel);
        if metadata.arm_retirement_notification() {
            // The borrower left before arming. This sweep step already drives
            // worker notification after releasing its coordination guards.
            self.queue_released_storage(metadata);
        }
    }

    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
    pub(crate) fn with_parked_descriptor_registry(&self, work: impl FnOnce()) {
        if let Some(heap) = &self.descriptor_regions {
            heap.with_parked_registry(work);
        }
    }

    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
    pub(crate) fn has_released_donor_jobs(&self) -> bool {
        self.released_donors.has_jobs()
    }

    pub(super) fn queue_released_storage(&self, metadata: MetadataRef) {
        self.retired_ready.fetch_add(1, Ordering::AcqRel);
        if metadata.can_transfer_retired_charge() {
            self.released_donors.publish(metadata);
        } else {
            self.publish_retirement(ReclaimTask::new(metadata));
        }
    }

    /// Publish a continuation before releasing the current callback's ready count.
    pub(super) fn queue_clearing(&self, task: ReclaimTask, progress: TraceProgress) {
        self.retired_ready.fetch_add(1, Ordering::AcqRel);
        self.requeue_retirement(task, progress);
    }

    pub(super) fn requeue_retirement(&self, mut task: ReclaimTask, progress: TraceProgress) {
        task.advance(progress);
        // A Busy callback or regional return still owns all its storage. Once
        // its retry is public the requesting thread may itself release the blocker.
        #[cfg(not(target_family = "wasm"))]
        let _deferred = if progress == TraceProgress::Busy {
            task.full_cleanup.take()
        } else {
            None
        };
        self.retired_objects.fetch_add(1, Ordering::AcqRel);
        self.publish_retirement(task);
    }

    pub(super) fn publish_retirement(&self, task: ReclaimTask) {
        if let Some(queue) = self.retired.load().as_ref() {
            queue.push(task);
            return;
        }
        // Allocate only on the first deferred handoff. A preempted initializer
        // owns private storage and cannot prevent another producer installing a queue.
        let queue = Arc::new(crate::publication_queue::Queue::new());
        let previous = self.retired.compare_and_swap(
            std::ptr::null::<crate::publication_queue::Queue<ReclaimTask>>(),
            Some(queue.clone()),
        );
        previous.as_ref().unwrap_or(&queue).push(task);
    }

    /// The caller owns one ready reservation and the retired byte charge. Once a
    /// callback unwinds, its partially cleared payload can only be destroyed.
    pub(super) fn clear_retired(
        &self,
        task: &mut ReclaimTask,
        budget: usize,
        counters: &mut WorkCounters,
        selection: Option<crate::retirement_grants::Selection>,
    ) -> TraceProgress {
        use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
        let metadata = &mut task.metadata;
        let callback_panic = if let ReclaimStage::Clear { offset, finalized } = task.stage {
            debug_assert!(metadata.storage_ready());
            match catch_unwind(AssertUnwindSafe(|| {
                if !finalized {
                    metadata.run_finalizer();
                }
                metadata.trace.clear_step(offset, budget)
            })) {
                Ok(TraceProgress::Complete) => None,
                Ok(progress) => return progress,
                Err(panic) => Some(panic),
            }
        } else {
            None
        };
        let destruction = catch_unwind(AssertUnwindSafe(|| match &mut task.stage {
            ReclaimStage::Clear { .. } => {
                if let Some(region) = ObjectMetadata::destroy_retired(metadata, self, selection) {
                    task.stage = ReclaimStage::Return(region);
                    false
                } else {
                    true
                }
            }
            ReclaimStage::Return(region) => region.finish(self),
        }));
        if matches!(destruction, Ok(false)) {
            if let Some(panic) = callback_panic {
                // Preserve sole ownership of deferred backing before propagating
                // the callback panic. The queued job accounts for its own refund.
                let mut next = ReclaimTask::new(metadata.clone());
                std::mem::swap(&mut next.stage, &mut task.stage);
                #[cfg(not(target_family = "wasm"))]
                {
                    next.full_cleanup = task.full_cleanup.take();
                }
                self.queue_clearing(next, TraceProgress::Busy);
                self.retired_ready.fetch_sub(1, Ordering::AcqRel);
                self.wake();
                resume_unwind(panic);
            }
            return TraceProgress::Busy;
        }
        self.retired_bytes
            .fetch_sub(metadata.size(), Ordering::AcqRel);
        counters.swept += 1;
        counters.freed += metadata.size();
        counters.logical_freed += metadata.logical_size();
        #[cfg(not(target_family = "wasm"))]
        drop(task.full_cleanup.take());
        if let Some(panic) = callback_panic.or(destruction.err()) {
            self.retired_ready.fetch_sub(1, Ordering::AcqRel);
            counters.flush(&mut lock(&self.statistics));
            self.wake();
            resume_unwind(panic);
        }
        TraceProgress::Complete
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn cleanup_pool(&self, metadata: &ObjectMetadata) -> Option<Arc<WorkerSignals>> {
        if !metadata.has_cleanup()
            || WORKER
                .get()
                .is_none_or(|(heap, _)| heap != std::ptr::from_ref(self).addr())
        {
            return None;
        }
        native_callbacks::pool().filter(|pool| !pool.closed.load(Ordering::Acquire))
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn offload_cleanup(
        &self,
        task: ReclaimTask,
        budget: usize,
        origin: Option<(usize, u64)>,
    ) -> Option<ReclaimTask> {
        let Some(pool) = self.cleanup_pool(&task.metadata) else {
            return Some(task);
        };
        let batch = native_callbacks::Batch {
            first: (task, budget),
            rest: Vec::new(),
        };
        pool.callbacks
            .send(self, batch, origin)
            .map(|batch| batch.first.0)
    }

    /// Execute a sweep callback with exclusive descriptor ownership and no cycle,
    /// coordinator, or publication lock. A continuation can return to its original
    /// sweep while that exact epoch has space in its bounded continuation queue.
    pub(super) fn reclaim_swept(
        &self,
        mut task: ReclaimTask,
        index: usize,
        epoch: u64,
        budget: usize,
        counters: &mut WorkCounters,
        selection: Option<crate::retirement_grants::Selection>,
    ) {
        #[cfg(not(target_family = "wasm"))]
        if selection.is_none() {
            let Some(owned) = self.offload_cleanup(task, budget, Some((index, epoch))) else {
                return;
            };
            task = owned;
        }
        let _cleanup = task
            .metadata
            .has_cleanup()
            .then(|| CleanupScope::enter(self));
        let progress = self.clear_retired(&mut task, budget, counters, selection);
        if progress == TraceProgress::Complete {
            self.record_sweep_reclaimed(index, epoch, task.metadata.size());
        } else if progress != TraceProgress::Busy
            && let Ok(mut cycle) = self.domain_at(index).cycle.try_lock()
            && cycle.epoch == epoch
            && cycle.phase == Phase::Sweep
            && cycle.reclaim.len() < phase_work::SWEEP_BATCH
        {
            task.advance(progress);
            cycle.reclaim.push_back(task);
        } else {
            self.queue_clearing(task, progress);
        }
        self.retired_ready.fetch_sub(1, Ordering::AcqRel);
    }

    pub(super) fn record_sweep_reclaimed(&self, index: usize, epoch: u64, bytes: usize) {
        if let Some(scheduler) = &self.scheduler
            && let Ok(mut cycle) = self.domain_at(index).cycle.try_lock()
            && cycle.epoch == epoch
            && cycle.kind == CollectionKind::Full
            && cycle.phase != Phase::Idle
        {
            if cycle.reclaimed_bytes == 0 {
                scheduler.reclaimed();
            }
            cycle.reclaimed_bytes = cycle.reclaimed_bytes.saturating_add(bytes);
        }
    }

    pub(crate) fn storage_ready(&self, metadata: MetadataRef) {
        self.queue_released_storage(metadata);
        self.wake();
    }

    /// One bounded clearing unit, excluding unrelated leased allocations. Ready
    /// entries stay charged through callbacks and continuation publication.
    pub(super) fn reap_retired(
        &self,
        concurrent: bool,
        check: Option<&mut crate::retirement_grants::ReleasedCheck>,
    ) -> Option<RetirementStep> {
        // Popping transfers exclusive descriptor ownership to this call. It has
        // already been removed from every generation's registry, so neither the
        // coordinator nor a generation cursor protects this callback's storage.
        let started = now();
        let cpu_started = (concurrent && self.diagnostics_enabled())
            .then(crate::diagnostics::thread_cpu_ns)
            .flatten();
        let timing = if concurrent {
            &self.diagnostics.worker_steps
        } else {
            &self.diagnostics.caller_steps
        };
        let budget = self.configuration.incremental_step_size.clamp(1, 32);
        let queued = || self.retired.load().as_ref()?.pop().map(|task| (task, None));
        let prefer_queue = check.is_none() && self.released_turn.fetch_xor(true, Ordering::Relaxed);
        let mut task = if prefer_queue { queued() } else { None };
        let mut counters = WorkCounters::default();
        if task.is_none() {
            let released = match check {
                Some(check) => {
                    Some(check.step(&self.released_donors, &self.grant_registry, budget)?)
                }
                None => self.released_donors.help(&self.grant_registry, budget),
            };
            match released {
                Some(crate::retirement_grants::ReleasedWork::Progress) => {
                    self.record_step(started, cpu_started, concurrent, &mut counters);
                    drop(timing.measure_from(self.diagnostics_enabled(), started));
                    self.notify_progress();
                    return Some(RetirementStep::Progress(false));
                }
                Some(crate::retirement_grants::ReleasedWork::Claimed(metadata, selection)) => {
                    task = Some((ReclaimTask::new(metadata), Some(selection)));
                }
                None => task = queued(),
            }
        }
        let (mut task, selection) = task?;
        let measurement = timing.measure_from(self.diagnostics_enabled(), started);
        self.retired_objects.fetch_sub(1, Ordering::AcqRel);
        let budget = budget.min(task.metadata.trace.budget);
        #[cfg(not(target_family = "wasm"))]
        if selection.is_none() {
            let Some(owned) = self.offload_cleanup(task, budget, None) else {
                return Some(RetirementStep::Progress(false));
            };
            task = owned;
        }
        let _cleanup = task
            .metadata
            .has_cleanup()
            .then(|| CleanupScope::enter(self));
        let progress = self.clear_retired(&mut task, budget, &mut counters, selection);
        let contended = progress == TraceProgress::Busy;
        if progress == TraceProgress::Complete {
            self.retired_ready.fetch_sub(1, Ordering::AcqRel);
        } else {
            self.requeue_retirement(task, progress);
        }
        self.record_step(started, cpu_started, concurrent, &mut counters);
        drop(measurement);
        self.notify_progress();
        Some(if contended {
            RetirementStep::Contended
        } else {
            RetirementStep::Progress(!self.pending())
        })
    }
}
