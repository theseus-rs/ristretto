//! Time series for distinguishing sustainable load from a growing backlog or heap.
use ristretto_gc::{GarbageCollector, Result};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Serialize)]
pub struct Sample {
    seconds: f64,
    completed_operations: u64,
    offered_operations: u64,
    backlog_operations: u64,
    logical_bytes: usize,
    retired_bytes: Option<usize>,
    shared_committed_bytes: usize,
    descriptor_committed_bytes: Option<usize>,
    cached_allocation_bytes: Option<usize>,
    cached_allocation_credit: Option<usize>,
    context_storage_bytes: Option<usize>,
    young_cycles: usize,
    full_cycles: usize,
    old_cycles: usize,
}

pub struct Telemetry {
    pub interval_ms: u64,
    pub samples: Vec<Sample>,
    next: Duration,
}

impl Telemetry {
    pub fn new(
        requested_ms: u64,
        seconds: u64,
    ) -> std::result::Result<Self, std::collections::TryReserveError> {
        // Bound the recorder's own memory even for a very long run. Report the
        // effective interval and reserved storage rather than silently omitting it.
        let interval_ms = if requested_ms == 0 {
            0
        } else {
            requested_ms
                .max(seconds.saturating_mul(1000).div_ceil(100_000))
                .max(1)
        };
        let mut samples = Vec::new();
        if let Some(count) = seconds.saturating_mul(1000).checked_div(interval_ms) {
            samples
                .try_reserve_exact(usize::try_from(count).unwrap_or(100_000).min(100_000) + 2)?;
        }
        Ok(Self {
            interval_ms,
            samples,
            next: Duration::ZERO,
        })
    }

    pub fn storage_bytes(&self) -> usize {
        self.samples.capacity() * size_of::<Sample>()
    }

    pub fn record(
        &mut self,
        gc: &GarbageCollector,
        started: Instant,
        completed: u64,
        rate: u64,
        offered: u64,
        force: bool,
    ) -> Result<()> {
        if self.interval_ms == 0 {
            return Ok(());
        }
        let elapsed = started.elapsed();
        if !force && elapsed < self.next {
            return Ok(());
        }
        let statistics = gc.statistics()?;
        let diagnostic = gc.diagnostics();
        let regions = gc.region_heap().map_or_default(|heap| heap.statistics());
        let due = (elapsed.as_nanos().saturating_mul(u128::from(rate)) / 1_000_000_000 + 1)
            .min(u128::from(offered));
        let due = u64::try_from(due).unwrap_or(offered);
        self.samples.push(Sample {
            seconds: elapsed.as_secs_f64(),
            completed_operations: completed,
            offered_operations: due,
            backlog_operations: due.saturating_sub(completed),
            logical_bytes: statistics.bytes_managed,
            retired_bytes: diagnostic.enabled.then_some(diagnostic.retired_bytes),
            shared_committed_bytes: regions.shared_committed_bytes,
            descriptor_committed_bytes: diagnostic
                .enabled
                .then_some(diagnostic.descriptor_committed_bytes),
            cached_allocation_bytes: diagnostic
                .enabled
                .then_some(diagnostic.cached_allocation_bytes),
            cached_allocation_credit: diagnostic
                .enabled
                .then_some(diagnostic.cached_allocation_credit),
            context_storage_bytes: diagnostic
                .enabled
                .then_some(diagnostic.context_storage_bytes),
            young_cycles: statistics.young_collections,
            full_cycles: statistics.full_collections,
            old_cycles: statistics.old_collections,
        });
        self.next = elapsed.saturating_add(Duration::from_millis(self.interval_ms));
        Ok(())
    }
}
