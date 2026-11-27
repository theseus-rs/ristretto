//! Coherent cumulative statistics. Readers can retain a version while other
//! threads commit fresh work or complete a collection ticket.
use super::{CollectionKind, Cycle, GarbageCollector, Ordering, Result, Statistics};

thread_local! {
    static SPARE: crate::snapshot::Spare<Statistics> = const { std::cell::Cell::new(None) };
}

/// Accumulate useful work until its reservation or collector step completes.
#[derive(Default)]
pub(super) struct WorkCounters {
    pub(super) marked: usize,
    pub(super) swept: usize,
    pub(super) freed: usize,
    pub(super) logical_freed: usize,
    pub(super) promoted: usize,
    pub(super) references: usize,
}

impl WorkCounters {
    pub(super) fn publish(&mut self, gc: &GarbageCollector) {
        let completed = std::mem::take(self);
        gc.update_statistics(|stats| completed.add_to(stats));
    }

    fn add_to(&self, stats: &mut Statistics) {
        stats.objects_marked += self.marked;
        stats.objects_swept += self.swept;
        stats.bytes_freed += self.logical_freed;
        stats.managed_bytes_freed += self.freed;
        stats.objects_promoted += self.promoted;
        stats.references_examined += self.references;
    }
}

impl GarbageCollector {
    /// Returns live heap accounting and collection statistics.
    ///
    /// # Errors
    /// Reserved for compatibility with callers of the previous collector.
    pub fn statistics(&self) -> Result<Statistics> {
        let mut stats = {
            let snapshot = self.statistics.load();
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            super::phase_tests::after_statistics_snapshot();
            snapshot.as_ref().clone()
        };
        stats.objects_marked += self.leaves_marked_at_birth.load(Ordering::Relaxed);
        stats.bytes_allocated = self.published_debt.load(Ordering::Acquire);
        stats.bytes_live = self.published_bytes.load(Ordering::Acquire);
        stats.bytes_managed = self.managed_bytes();
        stats.allocation_debt = self.allocation_debt.load(Ordering::Acquire);
        stats.live_objects = self.objects.len();
        Ok(stats)
    }

    // The closure can retry against a competing commit. Keep clocks, ticket
    // publication, scheduler notifications and work-counter takes outside it.
    pub(super) fn update_statistics(&self, change: impl Fn(&mut Statistics)) -> usize {
        crate::snapshot::update(&self.statistics, &SPARE, change, |prior| {
            prior.young_collections
        })
    }

    pub(super) fn record_cycle_completion(&self, cycle: &Cycle, counters: &mut WorkCounters) {
        let duration = cycle.started.map(|started| started.elapsed());
        let completed = std::mem::take(counters);
        let previous_young = self.update_statistics(|stats| {
            completed.add_to(stats);
            stats.collections_completed += 1;
            match cycle.kind {
                CollectionKind::Young => stats.young_collections += 1,
                CollectionKind::Old => stats.old_collections += 1,
                CollectionKind::Full => stats.full_collections += 1,
            }
            if let Some(duration) = duration {
                stats.last_collection_duration = Some(duration);
                stats.total_collection_time += duration;
            }
        });
        match cycle.kind {
            CollectionKind::Young => {
                let young = previous_young + 1;
                if young.is_multiple_of(self.configuration.full_collection_interval.max(1)) {
                    self.domains[1].publish_request();
                } else if self.configuration.old_collection_interval != 0
                    && young.is_multiple_of(self.configuration.old_collection_interval)
                {
                    self.old_requested.store(true, Ordering::Release);
                }
            }
            CollectionKind::Full => {
                if let Some(scheduler) = &self.scheduler {
                    scheduler.completed(cycle.reclaimed_bytes, self.managed_budget.used());
                }
            }
            CollectionKind::Old => {}
        }
    }

    pub(super) fn record_step(
        &self,
        started: Option<std::time::Instant>,
        cpu_started: Option<u64>,
        concurrent: bool,
        counters: &mut WorkCounters,
    ) -> Option<std::time::Duration> {
        if let Some(started) = cpu_started
            && let Some(finished) = crate::diagnostics::thread_cpu_ns()
        {
            self.diagnostics
                .worker_cpu_ns
                .fetch_add(finished.saturating_sub(started), Ordering::Relaxed);
        }
        let duration = started.map(|time| time.elapsed());
        let completed = std::mem::take(counters);
        self.update_statistics(|stats| {
            completed.add_to(stats);
            stats.collection_steps += 1;
            if let Some(duration) = duration {
                let micros = u64::try_from(duration.as_micros()).unwrap_or(u64::MAX);
                if concurrent {
                    stats.concurrent_time_us = stats.concurrent_time_us.saturating_add(micros);
                } else {
                    stats.total_pause_time_us = stats.total_pause_time_us.saturating_add(micros);
                }
                stats.max_step_duration = Some(
                    stats
                        .max_step_duration
                        .map_or(duration, |old| old.max(duration)),
                );
            }
        });
        duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recycling_a_snapshot_never_overwrites_a_reader_or_carries_counts_to_another_heap() {
        let gc = GarbageCollector::new();
        let other = GarbageCollector::new();
        let held = gc.statistics.load();
        for _ in 0..8 {
            gc.update_statistics(|s| s.objects_marked += 2);
            other.update_statistics(|s| s.objects_swept += 1);
        }
        assert_eq!((held.objects_marked, held.objects_swept), (0, 0));
        let current = gc.statistics.load();
        let peer = other.statistics.load();
        assert_eq!((current.objects_marked, current.objects_swept), (16, 0));
        assert_eq!((peer.objects_marked, peer.objects_swept), (0, 8));
    }

    #[test]
    fn competing_work_commits_publish_each_exact_delta_once() {
        let gc = GarbageCollector::new();
        let initial = gc.statistics.load_full();
        let other = std::cell::Cell::new(Some(WorkCounters {
            swept: 2,
            freed: 80,
            logical_freed: 48,
            ..WorkCounters::default()
        }));
        let completed = WorkCounters {
            swept: 3,
            freed: 128,
            logical_freed: 64,
            ..WorkCounters::default()
        };
        let previous_young = gc.update_statistics(|snapshot| {
            completed.add_to(snapshot);
            // This commit wins the first publication attempt. The outer update
            // must recalculate its whole delta from the winning snapshot.
            if let Some(mut delta) = other.take() {
                gc.update_statistics(|s| s.young_collections += 1);
                delta.publish(&gc);
            }
        });
        let current = gc.statistics().expect("collection statistics");
        assert_eq!(initial.objects_swept, 0);
        assert_eq!(previous_young, 1);
        assert_eq!(
            (
                current.objects_swept,
                current.managed_bytes_freed,
                current.bytes_freed
            ),
            (5, 208, 112)
        );
    }
}
