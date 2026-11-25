//! Opt-in pressure prediction. Allocation sampling never waits for the model or region registry.
use crate::region::{Pressure, RegionHeap};
use portable_atomic::AtomicU64;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

/// Advisory full-collection estimates, independent of optional stall diagnostics.
/// Values describe the latest sample, not allocation guarantees or reserved storage.
#[derive(Clone, Copy, Debug, Default)]
pub struct SchedulingStatistics {
    pub enabled: bool,
    pub clock_available: bool,
    pub samples: u64,
    pub requests: u64,
    pub allocation_bytes_per_second: Option<u64>,
    /// Request/start to the first reclaimed bytes, or completion when nothing was freed.
    pub reclamation_latency_ns: Option<u64>,
    pub headroom_bytes: usize,
    pub trigger_bytes: usize,
    /// Estimated retained fraction of the full cycle's starting bytes, in millionths.
    pub survival_ppm: u64,
    pub live_bytes: usize,
    /// Free region bytes outside the largest extent. These bytes can still serve smaller requests.
    pub fragmented_free_bytes: usize,
}

#[derive(Debug)]
pub(crate) struct Scheduler {
    origin: Option<Instant>,
    allocated: AtomicU64,
    next_sample: AtomicU64,
    quantum: u64,
    model: Mutex<Model>,
}

impl Scheduler {
    pub(crate) fn new(limit: usize) -> Self {
        let origin = crate::diagnostics::now();
        let quantum = (limit / 64).clamp(4096, 1024 * 1024) as u64;
        Self {
            origin,
            allocated: AtomicU64::new(0),
            next_sample: AtomicU64::new(quantum.min(limit.saturating_sub(limit / 4) as u64).max(1)),
            quantum,
            model: Mutex::new(Model::new(limit, origin.is_some())),
        }
    }

    fn time(&self) -> Option<Duration> {
        self.origin.map(|origin| origin.elapsed())
    }

    pub(crate) fn allocated(&self, bytes: usize, live: usize, heap: Option<&RegionHeap>) -> bool {
        let total = self
            .allocated
            .fetch_add(bytes as u64, Ordering::Relaxed)
            .saturating_add(bytes as u64);
        if total < self.next_sample.load(Ordering::Relaxed) {
            return false;
        }
        // A collector phase hook may own the model. Never block publication on it.
        let Ok(mut model) = self.model.try_lock() else {
            return false;
        };
        let total = self.allocated.load(Ordering::Relaxed);
        if total < self.next_sample.load(Ordering::Relaxed) {
            return false;
        }
        let request = model.sample(
            self.time(),
            total,
            live,
            bytes,
            heap.and_then(RegionHeap::pressure),
        );
        // Sample sooner near the occupancy threshold, including heaps smaller than a quantum.
        let remaining = (model.stats.trigger_bytes.saturating_sub(live) as u64)
            .max(
                (model.minimum_growth as u64)
                    .saturating_sub(total.saturating_sub(model.completed_total)),
            )
            .max(1);
        let interval = if model.active || model.pending {
            self.quantum
        } else {
            self.quantum.min(remaining)
        };
        self.next_sample
            .store(total.saturating_add(interval), Ordering::Relaxed);
        request
    }

    pub(crate) fn started(&self, live: usize) {
        let mut model = self
            .model
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        model.started(self.time(), live);
    }

    pub(crate) fn reclaimed(&self) {
        let mut model = self
            .model
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        model.reclaimed(self.time());
    }

    pub(crate) fn completed(&self, freed: usize, live: usize) {
        let mut model = self
            .model
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let total = self.allocated.load(Ordering::Relaxed);
        model.completed(self.time(), total, freed, live);
        self.next_sample.store(
            total.saturating_add(self.quantum.min(model.minimum_growth as u64).max(1)),
            Ordering::Relaxed,
        );
    }

    pub(crate) fn statistics(&self) -> SchedulingStatistics {
        self.model
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .stats
    }
}

#[derive(Debug)]
struct Model {
    limit: usize,
    stats: SchedulingStatistics,
    last_sample: Option<Duration>,
    sampled_total: u64,
    pending: bool,
    active: bool,
    requested_at: Option<Duration>,
    started_at: Option<Duration>,
    start_live: usize,
    observed_reclamation: bool,
    completed_total: u64,
    minimum_growth: usize,
    burst: usize,
}

impl Model {
    fn new(limit: usize, clock: bool) -> Self {
        Self {
            limit,
            stats: SchedulingStatistics {
                enabled: true,
                clock_available: clock,
                headroom_bytes: limit / 4,
                trigger_bytes: limit.saturating_sub(limit / 4),
                ..SchedulingStatistics::default()
            },
            last_sample: clock.then_some(Duration::ZERO),
            sampled_total: 0,
            pending: false,
            active: false,
            requested_at: None,
            started_at: None,
            start_live: 0,
            observed_reclamation: false,
            completed_total: 0,
            minimum_growth: (limit / 64).max(1),
            burst: 0,
        }
    }

    fn sample(
        &mut self,
        now: Option<Duration>,
        total: u64,
        live: usize,
        allocation: usize,
        pressure: Option<Pressure>,
    ) -> bool {
        self.stats.samples = self.stats.samples.saturating_add(1);
        self.rate(now, total);
        self.burst = allocation
            .max(self.burst.saturating_sub(self.burst / 8))
            .min(self.limit / 8);
        self.headroom(pressure);
        if self.active
            || self.pending
            || live < self.stats.trigger_bytes
            || total.saturating_sub(self.completed_total) < self.minimum_growth as u64
        {
            return false;
        }
        self.pending = true;
        self.requested_at = now;
        self.stats.requests = self.stats.requests.saturating_add(1);
        true
    }

    fn rate(&mut self, now: Option<Duration>, total: u64) {
        if let (Some(now), Some(previous)) = (now, self.last_sample) {
            let nanos = now.saturating_sub(previous).as_nanos();
            if nanos == 0 {
                return;
            }
            let rate = u128::from(total.saturating_sub(self.sampled_total)) * 1_000_000_000 / nanos;
            let rate = u64::try_from(rate).unwrap_or(u64::MAX);
            self.stats.allocation_bytes_per_second =
                Some(peak(self.stats.allocation_bytes_per_second, rate));
        }
        self.last_sample = now;
        self.sampled_total = total;
    }

    fn headroom(&mut self, pressure: Option<Pressure>) {
        // Startup/clock-free policy keeps a quarter of the heap as advisory headroom.
        let predicted = match (
            self.stats.allocation_bytes_per_second,
            self.stats.reclamation_latency_ns,
        ) {
            (Some(rate), Some(latency)) => {
                usize::try_from(u128::from(rate) * u128::from(latency) / 1_000_000_000)
                    .unwrap_or(usize::MAX)
            }
            _ => self.limit / 4,
        };
        let mut fragmentation = 0;
        if let Some(pressure) = pressure {
            self.stats.fragmented_free_bytes = pressure.free.saturating_sub(pressure.largest_free);
            // Dispersed free space matters here only when backing headroom is low
            // and a recent allocation would not fit the largest extent.
            if pressure.committed >= self.limit.saturating_sub(self.limit / 8)
                && pressure.largest_free < self.burst
            {
                fragmentation = self.stats.fragmented_free_bytes.min(self.limit / 8);
            }
        }
        // Survival provides a small relocation allowance; these are not reserved bytes.
        let relocation = self.stats.live_bytes / 32;
        self.stats.headroom_bytes = predicted
            .saturating_add(self.burst)
            .saturating_add(relocation)
            .saturating_add(fragmentation)
            .clamp((self.limit / 32).max(1), (self.limit / 2).max(1));
        self.stats.trigger_bytes = self.limit.saturating_sub(self.stats.headroom_bytes);
    }

    fn started(&mut self, now: Option<Duration>, live: usize) {
        self.active = true;
        self.pending = false;
        self.started_at = self.requested_at.take().or(now);
        self.start_live = live;
        self.observed_reclamation = false;
    }

    fn reclaimed(&mut self, now: Option<Duration>) {
        if let (Some(now), Some(start)) = (now, self.started_at) {
            let nanos = u64::try_from(now.saturating_sub(start).as_nanos()).unwrap_or(u64::MAX);
            self.stats.reclamation_latency_ns =
                Some(peak(self.stats.reclamation_latency_ns, nanos));
        }
        self.observed_reclamation = true;
    }

    fn completed(&mut self, now: Option<Duration>, total: u64, freed: usize, live: usize) {
        if !self.observed_reclamation {
            self.reclaimed(now);
        }
        self.rate(now, total);
        let retained = self.start_live.saturating_sub(freed);
        self.stats.survival_ppm =
            u64::try_from((retained as u128) * 1_000_000 / self.start_live.max(1) as u128)
                .unwrap_or(1_000_000);
        self.stats.live_bytes = live;
        // A nearly all-live heap must grow before prediction can request another cycle.
        // Demand-driven recovery is still allowed to override this cooldown.
        self.minimum_growth = (self.limit / 64).max(retained / 16).max(1);
        self.completed_total = total;
        self.active = false;
        self.headroom(None);
    }
}

/// Respond immediately to increased demand; decay old peaks over several samples/cycles.
fn peak(previous: Option<u64>, value: u64) -> u64 {
    previous.map_or(value, |previous| value.max(previous - previous / 8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(
        clippy::unnecessary_wraps,
        reason = "tests exercise optional clock samples"
    )]
    fn time(millis: u64) -> Option<Duration> {
        Some(Duration::from_millis(millis))
    }

    #[test]
    fn predicts_pressure_and_includes_request_queue_time() {
        let mut model = Model::new(1_000_000, true);
        assert!(!model.sample(time(100), 100_000, 100_000, 4096, None));
        assert!(model.sample(time(800), 800_000, 800_000, 4096, None));
        model.started(time(820), 800_000);
        model.reclaimed(time(900));
        model.completed(time(950), 850_000, 700_000, 150_000);
        assert_eq!(model.stats.reclamation_latency_ns, Some(100_000_000));
        assert_eq!(model.stats.survival_ppm, 125_000);
        let before = model.stats.headroom_bytes;
        model.sample(time(960), 1_000_000, 300_000, 4096, None);
        assert!(model.stats.headroom_bytes > before);
        assert_eq!(model.stats.headroom_bytes, 500_000);
    }

    #[test]
    fn high_survival_coalesces_requests_and_requires_growth() {
        let mut model = Model::new(1_000_000, true);
        assert!(model.sample(time(100), 900_000, 900_000, 4096, None));
        assert!(!model.sample(time(101), 901_000, 901_000, 4096, None));
        model.started(time(102), 901_000);
        assert!(!model.sample(time(103), 902_000, 902_000, 4096, None));
        model.completed(time(200), 902_000, 0, 902_000);
        assert_eq!(model.stats.survival_ppm, 1_000_000);
        assert_eq!(model.stats.reclamation_latency_ns, Some(100_000_000));
        assert!(!model.sample(time(300), 903_000, 903_000, 4096, None));
        assert!(model.sample(time(400), 970_000, 970_000, 4096, None));
    }

    #[test]
    fn clock_free_policy_is_deterministic_and_never_invents_timing() {
        let mut model = Model::new(1024, false);
        assert!(!model.sample(None, 500, 500, 1, None));
        assert!(model.sample(None, 800, 800, 1, None));
        model.started(None, 800);
        model.reclaimed(None);
        model.completed(None, 800, 700, 100);
        assert_eq!(model.stats.allocation_bytes_per_second, None);
        assert_eq!(model.stats.reclamation_latency_ns, None);
        assert!(!model.stats.clock_available);
        assert_eq!(model.stats.headroom_bytes, 260);
    }

    #[test]
    fn fragmented_extents_only_add_pressure_when_backing_is_tight() {
        let mut model = Model::new(1_000_000, false);
        let pressure = Pressure {
            committed: 500_000,
            free: 200_000,
            largest_free: 1000,
        };
        model.sample(None, 100_000, 100_000, 4096, Some(pressure));
        let normal = model.stats.headroom_bytes;
        model.sample(
            None,
            200_000,
            200_000,
            4096,
            Some(Pressure {
                committed: 950_000,
                ..pressure
            }),
        );
        assert_eq!(model.stats.fragmented_free_bytes, 199_000);
        assert_eq!(model.stats.headroom_bytes, normal + 125_000);
    }

    #[test]
    fn tiny_heaps_and_extreme_rates_keep_estimates_bounded() {
        for limit in [1, 2, 4096, usize::MAX - 1] {
            let mut model = Model::new(limit, true);
            model.stats.allocation_bytes_per_second = Some(u64::MAX);
            model.stats.reclamation_latency_ns = Some(u64::MAX);
            model.burst = usize::MAX;
            model.headroom(None);
            assert!(model.stats.headroom_bytes <= limit);
            assert_eq!(
                model.stats.headroom_bytes + model.stats.trigger_bytes,
                limit
            );
        }
    }

    #[test]
    fn contention_defers_sample_without_losing_allocated_bytes() {
        let scheduler = Scheduler::new(8192);
        let guard = scheduler.model.lock().expect("model");
        assert!(!scheduler.allocated(7000, 7000, None));
        drop(guard);
        assert!(scheduler.allocated(1, 7001, None));
        assert_eq!(scheduler.statistics().samples, 1);
        assert_eq!(scheduler.allocated.load(Ordering::Relaxed), 7001);
    }
}
