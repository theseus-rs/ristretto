//! Advisory native worker activation. Never participates in tracing termination.
use portable_atomic::AtomicU64;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub(crate) struct Demand {
    pub(crate) queued: usize,
    pub(crate) debt: usize,
    pub(crate) live: usize,
    pub(crate) waiters: usize,
}

#[derive(Debug)]
pub(crate) struct Controller {
    primary: AtomicUsize,
    active: AtomicUsize,
    delay: AtomicU64,
    origin: Instant,
    model: Mutex<Model>,
}

impl Controller {
    pub(crate) fn new(workers: usize, cpus: usize, limit: usize, batch: usize, delay: u64) -> Self {
        Self {
            primary: AtomicUsize::new(0),
            active: AtomicUsize::new(1),
            delay: AtomicU64::new(delay.min(1000)),
            origin: Instant::now(),
            model: Mutex::new(Model {
                cap: workers.max(1).min(cpus.max(1)),
                limit,
                batch: batch.clamp(1, 8),
                delay: delay.min(1000),
                active: 1,
                primary_seen: true,
                lower_samples: 0,
                next: Duration::ZERO,
            }),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_sampling_frozen<T>(&self, f: impl FnOnce() -> T) -> T {
        let _guard = self.model.lock().expect("test owns sampler");
        f()
    }

    pub(crate) fn target(&self) -> usize {
        self.active.load(Ordering::Relaxed)
    }
    pub(crate) fn delay(&self) -> u64 {
        self.delay.load(Ordering::Relaxed)
    }

    /// One active seat can move between configured workers, even with one CPU.
    pub(crate) fn admits(&self, index: usize) -> bool {
        let active = self.target();
        let primary = self.primary.load(Ordering::Relaxed);
        index == primary || index < active - usize::from(primary >= active)
    }

    /// Sample at most every 2 ms, deferring on contention. Input acquisition must
    /// also be nonblocking; ordinary mutators never touch this controller.
    pub(crate) fn sample(&self, index: usize, demand: impl FnOnce() -> Option<Demand>) {
        if index != self.primary.load(Ordering::Relaxed) && self.admits(index) {
            return;
        }
        let Ok(mut model) = self.model.try_lock() else {
            return;
        };
        let primary = self.primary.load(Ordering::Relaxed);
        let reporting = index == primary;
        let now = self.origin.elapsed();
        if now < model.next {
            model.primary_seen |= reporting;
            return;
        }
        // A callback can park before the next step even with an empty mark
        // queue. Transfer its seat without increasing the activation target.
        if !model.primary_seen && !reporting && !self.admits(index) {
            self.primary.store(index, Ordering::Relaxed);
        }
        model.primary_seen = index == self.primary.load(Ordering::Relaxed);
        model.next = now.saturating_add(Duration::from_millis(2));
        let Some(demand) = demand() else {
            return;
        };
        let delay = model.update(&demand);
        self.active.store(model.active, Ordering::Relaxed);
        self.delay.store(delay, Ordering::Relaxed);
    }
}

#[derive(Debug)]
struct Model {
    cap: usize,
    limit: usize,
    batch: usize,
    delay: u64,
    active: usize,
    primary_seen: bool,
    lower_samples: usize,
    next: Duration,
}

impl Model {
    fn update(&mut self, demand: &Demand) -> u64 {
        let pressure = demand.waiters != 0
            || (self.limit != usize::MAX
                && (demand.live >= self.limit.saturating_sub(self.limit / 4)
                    || demand.debt >= (self.limit / 8).max(1)));
        // Queued tracing and relocation provide parallel work here. Serialized root,
        // reference and sweep phases retain one progressing worker under pressure.
        let grain = self.batch * if pressure { 1 } else { 4 };
        let target = demand.queued.div_ceil(grain).clamp(1, self.cap);
        match target.cmp(&self.active) {
            std::cmp::Ordering::Greater => {
                self.active = if pressure {
                    target
                } else {
                    (self.active + 1).min(target)
                };
                self.lower_samples = 0;
            }
            std::cmp::Ordering::Less => {
                self.lower_samples += 1;
                if self.lower_samples == 3 {
                    self.active -= 1;
                    self.lower_samples = 0;
                }
            }
            std::cmp::Ordering::Equal => self.lower_samples = 0,
        }
        if pressure {
            0
        } else if demand.queued >= self.batch * self.active {
            self.delay / 2
        } else {
            self.delay
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_activates_only_available_parallel_work_and_cpu_budget() {
        let controller = Controller::new(8, 2, 1024, 8, 50);
        let mut model = controller.model.lock().expect("model");
        let mut demand = Demand {
            queued: 128,
            debt: 0,
            live: 900,
            waiters: 0,
        };
        assert_eq!(model.update(&demand), 0);
        assert_eq!(model.active, 2);
        demand.queued = 0;
        for _ in 0..3 {
            model.update(&demand);
        }
        assert_eq!(model.active, 1);
        demand.waiters = 1;
        demand.live = 0;
        assert_eq!(model.update(&demand), 0);
        assert_eq!(model.active, 1);
    }

    #[test]
    fn activation_increases_gradually_and_low_samples_need_hysteresis() {
        let controller = Controller::new(4, 8, 1024, 1, 50);
        let mut model = controller.model.lock().expect("model");
        let mut demand = Demand {
            queued: 100,
            debt: 0,
            live: 0,
            waiters: 0,
        };
        assert_eq!(model.update(&demand), 25);
        assert_eq!(model.active, 2);
        model.update(&demand);
        assert_eq!(model.active, 3);
        demand.queued = 0;
        model.update(&demand);
        model.update(&demand);
        assert_eq!(model.active, 3);
        demand.queued = 12;
        model.update(&demand); // Reset the decrease streak.
        demand.queued = 0;
        model.update(&demand);
        model.update(&demand);
        assert_eq!(model.active, 3);
        model.update(&demand);
        assert_eq!(model.active, 2);
    }

    #[test]
    fn movable_primary_preserves_the_activation_budget() {
        let controller = Controller::new(6, 4, 1024, 8, 50);
        for active in 1..=4 {
            controller.active.store(active, Ordering::Relaxed);
            for primary in 0..6 {
                controller.primary.store(primary, Ordering::Relaxed);
                assert!(controller.admits(primary));
                assert_eq!((0..6).filter(|i| controller.admits(*i)).count(), active);
            }
        }
    }

    #[test]
    fn contended_samples_defer_and_extreme_inputs_stay_bounded() {
        let controller = Controller::new(0, 0, usize::MAX, usize::MAX, u64::MAX);
        let guard = controller.model.lock().expect("model");
        controller.sample(0, || panic!("must not sample under contention"));
        drop(guard);
        controller.sample(0, || {
            Some(Demand {
                queued: usize::MAX,
                debt: usize::MAX,
                live: usize::MAX,
                waiters: 0,
            })
        });
        assert_eq!(controller.target(), 1);
        assert_eq!(controller.delay(), 500);
    }
}
