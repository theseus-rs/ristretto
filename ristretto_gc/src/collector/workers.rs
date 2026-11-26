use super::{
    Arc, BTreeMap, CleanupScope, GarbageCollector, HeapOwner, Ordering, Result, Weak, lock,
};
#[cfg(not(target_family = "wasm"))]
use super::{WorkerContext, WorkerSignals, native_callbacks, thread};

impl GarbageCollector {
    /// Returns the number of registered native workers, including sleeping workers.
    /// This snapshot is available without diagnostic instrumentation. It is not a
    /// count of threads currently using CPU; concurrent start/stop can change it.
    /// Cooperative hosts report zero. Native configuration `threads = 0` selects
    /// one worker when `start` is called; `stop` joins workers before returning.
    #[must_use]
    pub fn registered_workers(&self) -> usize {
        self.recovery.registered_workers()
    }

    /// Only a logical allocation that would sleep behind native collection needs
    /// independent wake delivery. Reuse it until this set of workers stops.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn ensure_refund_notifier(&self, settlement_only: bool) -> Result<()> {
        if self.refund_notifier.load().is_some() {
            return Ok(());
        }
        let _workers = lock(&self.collector_thread);
        // stop() may already have taken the handles while a worker is still
        // parked. Keep independent delivery available until the workers exit.
        if (!settlement_only && self.recovery.registered_workers() == 0)
            || self.refund_notifier.load().is_some()
        {
            return Ok(());
        }
        let notifier = crate::recovery::RefundNotifier::spawn(self)?;
        self.refund_notifier.store(Some(Arc::new(notifier)));
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn close_refund_notifier(&self) {
        if let Some(notifier) = self.refund_notifier.swap(None) {
            notifier.close();
        }
    }

    /// Starts the native worker. Cooperative hosts need no worker.
    #[cfg(not(target_family = "wasm"))]
    pub fn start(&self) {
        let mut worker = lock(&self.collector_thread);
        if !worker.is_empty() {
            return;
        }
        let count = self.configuration.threads.max(1);
        let signals = Arc::new(WorkerSignals::new(count));
        self.worker_signals.store(Some(signals.clone()));
        for index in 0..count {
            let weak = self.this.clone();
            let generation = signals.clone();
            let registration = crate::recovery::WorkerRegistration::new(self);
            worker.push(thread::spawn(move || {
                let _registration = registration;
                native_callbacks::register(&generation);
                let mut contention_delay = 1_u64;
                #[expect(
                    clippy::indexing_slicing,
                    reason = "each worker gets its ordinal in this fixed pool"
                )]
                let signal = &generation.workers[index];
                signal.register();
                let mut retry = true;
                loop {
                    if !retry {
                        signal.wait();
                    }
                    if generation.closed.load(Ordering::Acquire) {
                        break;
                    }
                    let Some(gc) = weak.upgrade() else { break };
                    let _worker = WorkerContext::enter(&gc, index);
                    while gc.worker_pending() {
                        if generation.closed.load(Ordering::Acquire) {
                            break;
                        }
                        if !gc.worker_admitted(index) {
                            thread::sleep(std::time::Duration::from_millis(1));
                            continue;
                        }
                        let claimed = gc.step_inner(true).is_some();
                        if !claimed && gc.configuration.worker_yield_time_us == 0 {
                            // A competing helper owns the available claim. Spread
                            // retries without sleeping or delaying a successful step.
                            for _ in 0..contention_delay {
                                std::hint::spin_loop();
                            }
                            contention_delay = (contention_delay * 2).min(16384);
                        }
                        let pacing = gc.pacing.as_ref().map_or(
                            gc.configuration.worker_yield_time_us,
                            crate::pacing::Controller::delay,
                        );
                        if !claimed && gc.configuration.worker_yield_time_us != 0 {
                            // Failed claims cannot help an allocator. Back off even
                            // under pressure, then decay the delay after useful claims.
                            gc.back_off_worker(&mut contention_delay);
                            continue;
                        }
                        if claimed {
                            // A single successful unit does not imply that contention
                            // with the helper holding the next packet has subsided.
                            // Only failed attempts wait. Continuous useful work runs
                            // immediately and gradually restores prompt retries.
                            contention_delay = if gc.configuration.worker_yield_time_us == 0 {
                                contention_delay.saturating_sub(1).max(1)
                            } else {
                                (contention_delay / 2).max(1)
                            };
                        }
                        let pressure = gc.managed_budget.used()
                            >= gc
                                .configuration
                                .max_heap_bytes
                                .saturating_sub(gc.configuration.max_heap_bytes / 4);
                        // Do not pace a full recovery cycle after sweep has freed
                        // some bytes but an allocation still awaits its completion.
                        if pacing == 0 || pressure || gc.recovery.waiter_count() != 0 {
                            thread::yield_now();
                        } else {
                            thread::sleep(std::time::Duration::from_micros(pacing.min(1000)));
                        }
                    }
                    // A request can look idle while another worker transfers it to
                    // active state. Never block the idle handoff on that worker.
                    retry = if let Ok(_coordinator) = gc.coordinator.try_lock() {
                        gc.worker_pending()
                    } else {
                        gc.back_off_worker(&mut contention_delay);
                        true
                    };
                }
            }));
        }
        // A request may predate worker registration. Poll the authoritative
        // demand once before sleeping, including when start races a request.
        signals.wake();
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn worker_admitted(&self, index: usize) -> bool {
        let Some(pacing) = &self.pacing else {
            return true;
        };
        pacing.sample(index, || {
            Some(crate::pacing::Demand {
                queued: self.domains.iter().try_fold(0, |count, domain| {
                    domain.mark_queue.queued_sample().map(|queued| {
                        count
                            + queued
                            + domain
                                .relocations
                                .as_ref()
                                .map_or(0, crate::relocation::Queues::len)
                    })
                })?,
                debt: self.allocation_debt.load(Ordering::Relaxed),
                live: self.managed_budget.used(),
                waiters: self.recovery.waiter_count(),
            })
        });
        let admitted = pacing.admits(index);
        if !admitted && self.diagnostics_enabled() {
            self.diagnostics
                .worker_throttles
                .fetch_add(1, Ordering::Relaxed);
        }
        admitted
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn back_off_worker(&self, contention_delay: &mut u64) {
        let pacing = self.configuration.worker_yield_time_us.min(1000);
        if pacing == 0 {
            thread::yield_now();
            return;
        }
        let delay = (*contention_delay).min(pacing);
        if self.diagnostics_enabled() {
            self.diagnostics
                .worker_backoffs
                .fetch_add(1, Ordering::Relaxed);
        }
        thread::sleep(std::time::Duration::from_micros(delay));
        *contention_delay = delay.saturating_mul(2).min(pacing);
    }

    /// Cooperative hosts execute collection through `step` or `collect_full`.
    #[cfg(target_family = "wasm")]
    pub fn start(&self) {}

    /// Stops the worker and releases unrooted shutdown-owned allocations.
    ///
    /// # Errors
    /// Returns an error if a native worker panics.
    pub fn stop(&self) -> Result<()> {
        if CleanupScope::contains(self) {
            return Err(crate::Error::CollectionPhaseError(
                "cleanup cannot wait for its own collector to stop".into(),
            ));
        }
        let _shutdown = CleanupScope::enter(self);
        #[cfg(not(target_family = "wasm"))]
        let cleanup_failed = {
            let (workers, generation) = {
                let mut workers = lock(&self.collector_thread);
                let generation = self.worker_signals.swap(None);
                if let Some(previous) = &generation {
                    previous.close();
                }
                (std::mem::take(&mut *workers), generation)
            };
            for worker in workers {
                if worker.thread().id() == thread::current().id() {
                    continue;
                }
                worker
                    .join()
                    .map_err(|_| crate::Error::SyncError("collector worker panicked".into()))?;
            }
            let cleanup_failed = if let Some(generation) = generation {
                generation.callbacks.join()?
            } else {
                false
            };
            let notifier = {
                let _workers = lock(&self.collector_thread);
                (self.recovery.registered_workers() == 0)
                    .then(|| self.refund_notifier.swap(None))
                    .flatten()
            };
            if let Some(notifier) = notifier {
                notifier.close();
                notifier.join();
            }
            cleanup_failed
        };
        self.collect_full()?;
        while self.retired_ready.load(Ordering::Acquire) != 0 || self.released_donors.has_jobs() {
            self.step();
        }
        for domain in &self.domains {
            domain.mark_queue.trim_empty();
            if let Some(queues) = &domain.relocations {
                queues.trim_empty();
            }
        }
        if let Some(roots) = self.scanned_slots() {
            roots.trim_empty();
        }
        self.contexts.for_each(|context| {
            context.roots.trim_empty();
            context.release_allocations(self);
        });
        if let Some(heap) = &self.descriptor_regions {
            while !heap.trim(64) {}
        }
        #[cfg(not(target_family = "wasm"))]
        if cleanup_failed {
            return Err(crate::Error::SyncError(
                "collector callback panicked".into(),
            ));
        }
        Ok(())
    }

    /// Registers VM metadata whose roots exist independently of allocated instances.
    pub fn register_heap_owner<T: HeapOwner + 'static>(&self, owner: &Arc<T>) {
        let weak = Arc::downgrade(owner);
        let weak: Weak<dyn HeapOwner> = weak;
        lock(&self.heap_owners).insert(Arc::as_ptr(owner).cast::<()>() as usize, weak);
    }

    /// Releases class mirrors and static roots when the VM that owns this heap shuts down.
    /// Call only after requesting cancellation of its Java tasks.
    pub fn release_vm_roots(&self) {
        let mut owners = BTreeMap::new();
        for (address, owner) in lock(&self.heap_owners).iter() {
            if let Some(owner) = owner.upgrade() {
                owners.insert(*address, owner);
            }
        }
        self.objects.for_each(|metadata| {
            if metadata.birth.is_live()
                && let Some(owner) = metadata.owner.as_ref().and_then(Weak::upgrade)
            {
                owners.insert(Arc::as_ptr(&owner).cast::<()>() as usize, owner);
            }
        });
        for owner in owners.into_values() {
            owner.release_heap_roots();
        }
    }

    pub(super) fn cleanup_remaining_objects(&self) {
        self.contexts
            .for_each(|context| context.release_allocations(self));
        #[cfg(feature = "experimental-identity-index")]
        self.identities.clear();
        for domain in &self.domains {
            // An if-let scrutinee would retain its guard through the callback
            // body. Transfer the task first, then run all teardown outside it.
            let tasks = std::mem::take(&mut lock(&domain.cycle).reclaim);
            for task in tasks {
                task.metadata.run_finalizer();
                task.metadata.run_drop_fn();
            }
            domain.mark_queue.clear();
            if let Some(queues) = &domain.relocations {
                queues.clear();
            }
        }
        let objects = self.objects.drain();
        #[cfg(not(feature = "experimental-identity-index"))]
        for metadata in &objects {
            self.identities
                .remove(metadata.id.to_i64().cast_unsigned(), metadata.ptr().0);
        }
        for metadata in &objects {
            metadata.retire();
            if metadata.storage_ready() {
                metadata.run_finalizer();
            }
        }
        for metadata in &objects {
            metadata.run_drop_fn();
        }
        self.released_donors.cleanup();
        if let Some(queue) = self.retired.load_full() {
            while let Some(task) = queue.pop() {
                task.metadata.run_finalizer();
                task.metadata.run_drop_fn();
            }
        }
        // An outstanding lease owns its descriptor and storage after heap teardown.
        self.retired_objects.store(0, Ordering::Release);
        self.retired_bytes.store(0, Ordering::Release);
        self.retired_ready.store(0, Ordering::Release);
        self.remembered.clear();
        // Private reservations may outlive the ordinary heap. Each owner refunds its own charge.
    }

    pub(super) fn wake_context_root_repair(&self, context: &crate::context::State) {
        if self.contexts.repairs.publish(context) {
            self.wake();
        }
    }

    pub(super) fn wake_root_repair(&self, roots: &crate::roots::RootSlots) {
        if roots.repair_pending() {
            self.wake();
        }
    }

    #[cfg_attr(
        target_family = "wasm",
        expect(
            clippy::unused_self,
            reason = "cooperative hosts have no native workers"
        )
    )]
    pub(super) fn wake(&self) {
        #[cfg(not(target_family = "wasm"))]
        if let Some(signals) = self.worker_signals.load().as_ref() {
            signals.wake();
        }
    }
}
