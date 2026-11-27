//! Optional, allocation-free collector diagnostics. Durations are not global pauses.

use portable_atomic::AtomicU64;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Aggregate duration of a slow path. Counts include hosts without a clock.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimingStatistics {
    pub count: u64,
    pub total_ns: u64,
    pub max_ns: u64,
}

/// Point-in-time diagnostic counters. Concurrent fields are sampled independently.
#[derive(Clone, Copy, Debug, Default)]
pub struct Diagnostics {
    pub enabled: bool,
    pub clock_available: bool,
    pub publication_read_wait: TimingStatistics,
    /// Retained for report compatibility. Collector writers now try once and
    /// yield on contention; failed attempts contribute to `publication_retries`.
    pub publication_write_wait: TimingStatistics,
    /// Counted publications currently admitted; independently sampled, not a closure decision.
    pub active_publications: usize,
    pub publication_transitions: u64,
    /// Failed legacy writer attempts or counted phase observations/commits.
    pub publication_retries: u64,
    pub payload_read_wait: TimingStatistics,
    /// Held managed read/write guards in cost-aware heaps with diagnostics enabled.
    /// Includes mapped guards and tracing's managed guards. Records after unlock;
    /// relocation's raw exclusive copy guards are excluded. Bare wasm counts only.
    pub payload_guard_holds: TimingStatistics,
    /// Currently held guards; completed durations exclude these unfinished samples.
    pub active_payload_guards: usize,
    pub payload_write_wait: TimingStatistics,
    pub allocation_recovery: TimingStatistics,
    /// Awaited recovery time, including suspension; distinct from inline recovery.
    pub async_allocation_recovery: TimingStatistics,
    pub async_collection_calls: u64,
    pub async_assists: u64,
    /// Allocation waiters notified before their recovery cycle completed.
    pub allocation_early_wakeups: u64,
    pub physical_allocation_early_wakeups: u64,
    pub allocation_waiters: usize,
    pub caller_steps: TimingStatistics,
    pub worker_steps: TimingStatistics,
    /// Sum of worker thread CPU time during collector steps, when supported.
    pub worker_cpu_ns: Option<u64>,
    pub full_collection_calls: u64,
    pub root_registrations: u64,
    /// Inclusive operation timings; nested measurements must not be summed.
    pub descriptor_publication: TimingStatistics,
    pub root_acquire: TimingStatistics,
    pub root_release: TimingStatistics,
    pub barrier_record: TimingStatistics,
    pub barrier_drain: TimingStatistics,

    pub execution_contexts: usize,
    /// Context structures and fixed buffers; excludes Arc/map allocation overhead
    /// and root pages (reported by `root_statistics`).
    pub context_storage_bytes: usize,
    /// Unused context-local physical ranges and centrally reserved logical credit.
    pub cached_allocation_bytes: usize,
    pub cached_allocation_credit: usize,
    /// Per-live-context cache activity, aggregated at observation time.
    pub local_allocation_hits: u64,
    pub local_allocation_refills: u64,
    /// Separate descriptor arena; backing shares the payload arena's physical cap.
    /// Global fallback descriptors and metadata remain outside these arena counters.
    pub descriptor_committed_bytes: usize,
    pub descriptor_occupied_bytes: usize,
    /// Logically retired allocations awaiting an admitted storage borrower or
    /// incremental clearing. These bytes remain charged to the managed limit.
    pub retired_bytes: usize,
    pub retired_objects: usize,
    pub retired_ready: usize,
    pub pending_barrier_records: usize,
    /// Newly reserved records (coalesced events are reported separately).
    pub barrier_buffered: u64,
    pub barrier_coalesced: u64,
    pub barrier_flushed: u64,
    pub barrier_overflows: u64,
    pub coordinator_misses: u64,
    /// Native worker sleeps after failing to claim the coordinator.
    pub worker_backoffs: u64,
    /// Adaptive policy target (zero when disabled), delay and throttled turns.
    /// The target is not a count of threads currently executing instructions.
    pub worker_target: usize,
    pub worker_pacing_us: u64,
    pub worker_throttles: u64,
    /// Bounded groups of tracing callbacks, under either marking policy.
    pub mark_batches: u64,
    /// Marking steps that claimed and published work without the coordinator.
    pub independent_mark_steps: u64,
    pub independent_phase_steps: u64,
    /// Independent claims deferred because a phase transition held the claim writer.
    pub mark_gate_misses: u64,
    /// Claimed tasks returned without execution because a batch yielded.
    pub trace_deferred: u64,
    /// Tasks claimed from another worker queue or the external injector.
    pub mark_stolen: u64,
    /// Queue probes skipped because another thread held the queue lock.
    pub mark_queue_misses: u64,
    /// Young/full queued tasks, capacity in task slots, and active reservations.
    /// Independently sampled; these fields are not a termination decision.
    pub queued_mark_tasks: [usize; 2],
    pub mark_queue_capacity: [usize; 2],
    pub in_flight_work: [usize; 2],
    /// Completed initial-young handoffs and executed old-core callbacks.
    pub old_handoff_objects: u64,
    pub old_trace_tasks: u64,
    pub trace_tasks: u64,
    pub trace_busy: u64,
    pub relocation_tasks: u64,
    pub relocation_busy: u64,
    /// With cost-aware relocation, first failed copy attempt to next non-busy
    /// attempt, including scheduler and selection delay. Only resolved samples
    /// are reported; this is not an exact payload-guard holding duration.
    pub relocation_pin_deferral: TimingStatistics,
    pub relocation_retry_limits: u64,
    pub relocation_cancel_busy: u64,
    pub independent_relocation_steps: u64,
    pub relocation_stolen: u64,
    pub relocation_queue_misses: u64,
    pub queued_relocation_tasks: [usize; 2],
    pub relocation_queue_capacity: [usize; 2],
}

#[derive(Debug, Default)]
pub(crate) struct Timing {
    count: AtomicU64,
    total: AtomicU64,
    max: AtomicU64,
}

impl Timing {
    pub(crate) fn record(&self, started: Option<Instant>) {
        self.record_duration(started.map(|time| time.elapsed()));
    }

    pub(crate) fn record_duration(&self, duration: Option<std::time::Duration>) {
        self.count.fetch_add(1, Ordering::Relaxed);
        if let Some(duration) = duration {
            let ns = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
            self.total.fetch_add(ns, Ordering::Relaxed);
            self.max.fetch_max(ns, Ordering::Relaxed);
        }
    }

    fn snapshot(&self) -> TimingStatistics {
        TimingStatistics {
            count: self.count.load(Ordering::Relaxed),
            total_ns: self.total.load(Ordering::Relaxed),
            max_ns: self.max.load(Ordering::Relaxed),
        }
    }

    pub(crate) fn measure(&self, enabled: bool) -> Measurement<'_> {
        self.measure_from(enabled, enabled.then(now).flatten())
    }

    /// Count a successful operation while retaining the time spent claiming its work.
    pub(crate) fn measure_from(&self, enabled: bool, started: Option<Instant>) -> Measurement<'_> {
        Measurement {
            timing: enabled.then_some(self),
            started,
        }
    }
}

pub(crate) struct Measurement<'a> {
    timing: Option<&'a Timing>,
    started: Option<Instant>,
}

impl Measurement<'_> {
    /// The diagnostic and collection step end at the same work boundary.
    /// Keep the normal drop measurement if the operation unwinds earlier.
    pub(crate) fn finish_with(mut self, duration: Option<std::time::Duration>) {
        if let Some(timing) = self.timing.take() {
            timing.record_duration(duration);
        }
    }
}

impl Drop for Measurement<'_> {
    fn drop(&mut self) {
        if let Some(timing) = self.timing {
            timing.record(self.started);
        }
    }
}

/// Bare wasm has no host clock. Do not invoke Instant there, even with diagnostics enabled.
#[cfg_attr(
    not(all(target_family = "wasm", target_os = "unknown")),
    expect(clippy::unnecessary_wraps)
)]
pub(crate) fn now() -> Option<Instant> {
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    {
        None
    }
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    {
        Some(Instant::now())
    }
}

pub(crate) fn thread_cpu_ns() -> Option<u64> {
    #[cfg(unix)]
    {
        let mut value = std::mem::MaybeUninit::<libc::timespec>::uninit();
        // SAFETY: clock_gettime initializes the pointed-to timespec on success.
        if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, value.as_mut_ptr()) } != 0 {
            return None;
        }
        // SAFETY: the successful call above initialized both fields.
        let value = unsafe { value.assume_init() };
        u64::try_from(value.tv_sec)
            .ok()?
            .checked_mul(1_000_000_000)?
            .checked_add(u64::try_from(value.tv_nsec).ok()?)
    }
    #[cfg(not(unix))]
    {
        None
    }
}

#[derive(Debug, Default)]
pub(crate) struct Counters {
    pub payload_guard_holds: Timing,
    pub active_payload_guards: AtomicUsize,
    pub publication_read_wait: Timing,
    pub publication_write_wait: Timing,
    pub publication_transitions: AtomicU64,
    pub publication_retries: AtomicU64,
    pub payload_read_wait: Timing,
    pub payload_write_wait: Timing,
    pub allocation_recovery: Timing,
    pub async_allocation_recovery: Timing,
    pub async_collection_calls: AtomicU64,
    pub async_assists: AtomicU64,
    pub allocation_early_wakeups: AtomicU64,
    pub physical_allocation_early_wakeups: AtomicU64,
    pub caller_steps: Timing,
    pub worker_steps: Timing,
    pub worker_cpu_ns: AtomicU64,
    pub full_collection_calls: AtomicU64,
    pub root_registrations: AtomicU64,
    pub descriptor_publication: Timing,
    pub root_acquire: Timing,
    pub root_release: Timing,
    pub barrier_record: Timing,
    pub barrier_drain: Timing,

    pub barrier_buffered: AtomicU64,
    pub barrier_coalesced: AtomicU64,
    pub barrier_flushed: AtomicU64,
    pub barrier_overflows: AtomicU64,
    pub coordinator_misses: AtomicU64,
    pub worker_backoffs: AtomicU64,
    pub worker_throttles: AtomicU64,
    pub mark_batches: AtomicU64,
    pub independent_mark_steps: AtomicU64,
    pub independent_phase_steps: AtomicU64,
    pub mark_gate_misses: AtomicU64,
    pub trace_deferred: AtomicU64,
    pub mark_stolen: AtomicU64,
    pub mark_queue_misses: AtomicU64,
    pub old_handoff_objects: AtomicU64,
    pub old_trace_tasks: AtomicU64,
    pub trace_tasks: AtomicU64,
    pub trace_busy: AtomicU64,
    pub relocation_tasks: AtomicU64,
    pub relocation_busy: AtomicU64,
    pub relocation_pin_deferral: Timing,
    pub relocation_retry_limits: AtomicU64,
    pub relocation_cancel_busy: AtomicU64,
    pub independent_relocation_steps: AtomicU64,
    pub relocation_stolen: AtomicU64,
    pub relocation_queue_misses: AtomicU64,
}

impl Counters {
    pub(crate) fn snapshot(&self, enabled: bool) -> Diagnostics {
        Diagnostics {
            payload_guard_holds: self.payload_guard_holds.snapshot(),
            active_payload_guards: self.active_payload_guards.load(Ordering::Relaxed),
            enabled,
            clock_available: !cfg!(all(target_family = "wasm", target_os = "unknown")),
            publication_read_wait: self.publication_read_wait.snapshot(),
            publication_write_wait: self.publication_write_wait.snapshot(),
            active_publications: 0,
            publication_transitions: self.publication_transitions.load(Ordering::Relaxed),
            publication_retries: self.publication_retries.load(Ordering::Relaxed),
            payload_read_wait: self.payload_read_wait.snapshot(),
            payload_write_wait: self.payload_write_wait.snapshot(),
            allocation_recovery: self.allocation_recovery.snapshot(),
            async_allocation_recovery: self.async_allocation_recovery.snapshot(),
            async_collection_calls: self.async_collection_calls.load(Ordering::Relaxed),
            async_assists: self.async_assists.load(Ordering::Relaxed),
            allocation_early_wakeups: self.allocation_early_wakeups.load(Ordering::Relaxed),
            physical_allocation_early_wakeups: self
                .physical_allocation_early_wakeups
                .load(Ordering::Relaxed),
            allocation_waiters: 0,
            caller_steps: self.caller_steps.snapshot(),
            worker_steps: self.worker_steps.snapshot(),
            worker_cpu_ns: thread_cpu_ns().map(|_| self.worker_cpu_ns.load(Ordering::Relaxed)),
            full_collection_calls: self.full_collection_calls.load(Ordering::Relaxed),
            root_registrations: self.root_registrations.load(Ordering::Relaxed),
            descriptor_publication: self.descriptor_publication.snapshot(),
            root_acquire: self.root_acquire.snapshot(),
            root_release: self.root_release.snapshot(),
            barrier_record: self.barrier_record.snapshot(),
            barrier_drain: self.barrier_drain.snapshot(),

            execution_contexts: 0,
            context_storage_bytes: 0,
            cached_allocation_bytes: 0,
            cached_allocation_credit: 0,
            local_allocation_hits: 0,
            local_allocation_refills: 0,
            descriptor_committed_bytes: 0,
            descriptor_occupied_bytes: 0,
            retired_bytes: 0,
            retired_objects: 0,
            retired_ready: 0,
            pending_barrier_records: 0,
            barrier_buffered: self.barrier_buffered.load(Ordering::Relaxed),
            barrier_coalesced: self.barrier_coalesced.load(Ordering::Relaxed),
            barrier_flushed: self.barrier_flushed.load(Ordering::Relaxed),
            barrier_overflows: self.barrier_overflows.load(Ordering::Relaxed),
            coordinator_misses: self.coordinator_misses.load(Ordering::Relaxed),
            worker_backoffs: self.worker_backoffs.load(Ordering::Relaxed),
            worker_target: 0,
            worker_pacing_us: 0,
            worker_throttles: self.worker_throttles.load(Ordering::Relaxed),
            mark_batches: self.mark_batches.load(Ordering::Relaxed),
            independent_mark_steps: self.independent_mark_steps.load(Ordering::Relaxed),
            independent_phase_steps: self.independent_phase_steps.load(Ordering::Relaxed),
            mark_gate_misses: self.mark_gate_misses.load(Ordering::Relaxed),
            trace_deferred: self.trace_deferred.load(Ordering::Relaxed),
            mark_stolen: self.mark_stolen.load(Ordering::Relaxed),
            mark_queue_misses: self.mark_queue_misses.load(Ordering::Relaxed),
            queued_mark_tasks: [0; 2],
            mark_queue_capacity: [0; 2],
            in_flight_work: [0; 2],
            old_handoff_objects: self.old_handoff_objects.load(Ordering::Relaxed),
            old_trace_tasks: self.old_trace_tasks.load(Ordering::Relaxed),
            trace_tasks: self.trace_tasks.load(Ordering::Relaxed),
            trace_busy: self.trace_busy.load(Ordering::Relaxed),
            relocation_tasks: self.relocation_tasks.load(Ordering::Relaxed),
            relocation_busy: self.relocation_busy.load(Ordering::Relaxed),
            relocation_pin_deferral: self.relocation_pin_deferral.snapshot(),
            relocation_retry_limits: self.relocation_retry_limits.load(Ordering::Relaxed),
            relocation_cancel_busy: self.relocation_cancel_busy.load(Ordering::Relaxed),
            independent_relocation_steps: self.independent_relocation_steps.load(Ordering::Relaxed),
            relocation_stolen: self.relocation_stolen.load(Ordering::Relaxed),
            relocation_queue_misses: self.relocation_queue_misses.load(Ordering::Relaxed),
            queued_relocation_tasks: [0; 2],
            relocation_queue_capacity: [0; 2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn shared_step_duration_preserves_counts_totals_and_maximum() {
        let timing = Timing::default();
        for (enabled, nanos) in [
            (true, Some(11)),
            (true, None),
            (false, Some(100)),
            (true, Some(5)),
        ] {
            timing
                .measure_from(enabled, None)
                .finish_with(nanos.map(Duration::from_nanos));
        }
        // An unfinished guard still records its count on drop, including on a clockless host.
        drop(timing.measure_from(true, None));
        assert_eq!(
            timing.snapshot(),
            TimingStatistics {
                count: 4,
                total_ns: 16,
                max_ns: 11
            }
        );
    }
}
