//! Opt-in Linux publication-gate telemetry, not a complete global-stop recorder.
//!
//! Recording uses fixed slots, atomic operations and `CLOCK_MONOTONIC` calls only.
//! Allocation and JSON export belong outside the captured collector lifetime.
//! Closing requires the caller to stop/join every producer first. A snapshot with
//! unfinished events is incomplete; it never waits for a producer to finish.

use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

/// Identity of this deliberately partial recorder protocol.
pub const RECORDER: &str = "ristretto-publication-gate-v2";

#[derive(Debug, Default)]
struct Slot {
    requested: AtomicU64,
    acquired: AtomicU64,
    released: AtomicU64,
    complete: AtomicBool,
}

/// One successful exclusive publication-gate acquisition. Request time includes
/// acquisition latency; release is sampled after the gate guard has been dropped.
#[derive(Clone, Copy, Debug)]
pub struct PauseEvent {
    pub sequence: usize,
    pub requested_ns: u64,
    pub acquired_ns: u64,
    pub released_ns: Option<u64>,
}

/// Timestamp returned to Java plus the GC-counter observations bracketing it.
/// Collections racing the timestamp remain explicitly bounded, not guessed.
#[derive(Clone, Copy, Debug)]
pub struct MeasurementBoundary {
    pub timestamp_ns: u64,
    pub observer_start_ns: u64,
    pub observer_end_ns: u64,
    pub collections_before: u64,
    pub collections_after: u64,
}

#[derive(Debug, Default)]
struct BoundarySlot {
    timestamp: AtomicU64,
    observer_start: AtomicU64,
    observer_end: AtomicU64,
    before: AtomicU64,
    after: AtomicU64,
}

impl BoundarySlot {
    fn snapshot(&self) -> MeasurementBoundary {
        MeasurementBoundary {
            timestamp_ns: self.timestamp.load(Ordering::Relaxed),
            observer_start_ns: self.observer_start.load(Ordering::Relaxed),
            observer_end_ns: self.observer_end.load(Ordering::Relaxed),
            collections_before: self.before.load(Ordering::Relaxed),
            collections_after: self.after.load(Ordering::Relaxed),
        }
    }
}

/// Closed capture of the instrumented collector lifetime. This covers only the
/// legacy publication writer, and cannot establish zero global GC stops.
#[derive(Debug)]
pub struct PauseSnapshot {
    pub capture_start_ns: u64,
    pub capture_end_ns: u64,
    pub clock_resolution_ns: u64,
    pub publication_attempts: usize,
    pub publication_acquisitions: usize,
    pub publication_failed_attempts: usize,
    pub dropped_events: usize,
    pub open_events: usize,
    pub clock_errors: usize,
    pub measurement_state: &'static str,
    pub measurement_errors: usize,
    pub measurement_begin: Option<MeasurementBoundary>,
    pub measurement_end: Option<MeasurementBoundary>,
    pub events: Vec<PauseEvent>,
}

/// Fixed-capacity, append-only recording. Overflow loses evidence, never blocks
/// the collector, and prevents treating the capture as complete. No global
/// singleton: configuration explicitly binds each collector to its recorder.
#[derive(Debug)]
pub struct PauseRecorder {
    start: u64,
    resolution: u64,
    slots: Box<[Slot]>,
    next: AtomicUsize,
    attempts: AtomicUsize,
    failed_attempts: AtomicUsize,
    clock_errors: AtomicUsize,
    collections: AtomicU64,
    measurement_state: AtomicU8,
    measurement_errors: AtomicUsize,
    measurement_begin: BoundarySlot,
    measurement_end: BoundarySlot,
}

impl PauseRecorder {
    /// Preallocate before creating collectors or starting measured work.
    ///
    /// # Errors
    /// Returns an error for empty/excessive capacity or an unavailable clock.
    pub fn new(capacity: usize) -> std::io::Result<Self> {
        if capacity == 0 || capacity > u32::MAX as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid pause capacity",
            ));
        }
        let resolution = crate::clock::monotonic_resolution_ns()
            .ok_or_else(|| std::io::Error::other("monotonic clock resolution unavailable"))?;
        let slots = (0..capacity).map(|_| Slot::default()).collect();
        Ok(Self {
            start: crate::clock::monotonic_time_ns()
                .ok_or_else(|| std::io::Error::other("monotonic clock unavailable"))?,
            resolution,
            slots,
            next: AtomicUsize::new(0),
            attempts: AtomicUsize::new(0),
            failed_attempts: AtomicUsize::new(0),
            clock_errors: AtomicUsize::new(0),
            collections: AtomicU64::new(0),
            measurement_state: AtomicU8::new(0),
            measurement_errors: AtomicUsize::new(0),
            measurement_begin: BoundarySlot::default(),
            measurement_end: BoundarySlot::default(),
        })
    }

    /// Establish the measurement start exactly at the returned monotonic stamp.
    /// May be called once. A duplicate or overlapping boundary returns `None`,
    /// records a protocol error, and never waits for the earlier caller.
    #[must_use]
    pub fn begin_measurement(&self) -> Option<u64> {
        self.boundary(0, &self.measurement_begin)
    }

    /// Establish the end exactly at the returned stamp, after one successful
    /// begin. Counter samples and observer costs remain visible in the snapshot.
    #[must_use]
    pub fn end_measurement(&self) -> Option<u64> {
        self.boundary(2, &self.measurement_end)
    }

    fn boundary(&self, expected: u8, slot: &BoundarySlot) -> Option<u64> {
        if self
            .measurement_state
            .compare_exchange(expected, expected + 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            self.measurement_errors.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        let observer_start = self.timestamp();
        let before = self.collections.load(Ordering::Acquire);
        let timestamp = self.timestamp();
        let after = self.collections.load(Ordering::Acquire);
        slot.observer_start.store(observer_start, Ordering::Relaxed);
        slot.before.store(before, Ordering::Relaxed);
        slot.timestamp.store(timestamp, Ordering::Relaxed);
        slot.after.store(after, Ordering::Relaxed);
        slot.observer_end.store(self.timestamp(), Ordering::Relaxed);
        self.measurement_state
            .store(expected + 2, Ordering::Release);
        Some(timestamp)
    }

    pub(crate) fn collection_completed(&self) {
        self.collections.fetch_add(1, Ordering::Release);
    }

    pub(crate) fn request(&self) -> u64 {
        self.attempts.fetch_add(1, Ordering::Relaxed);
        self.timestamp()
    }

    pub(crate) fn failed_attempt(&self) {
        self.failed_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn timestamp(&self) -> u64 {
        if let Some(value) = crate::clock::monotonic_time_ns() {
            value
        } else {
            self.clock_errors.fetch_add(1, Ordering::Relaxed);
            0
        }
    }

    /// Called only after successful `try_write`; unsuccessful attempts did not
    /// stop publication and consume no event slots. The timestamp was taken
    /// before `try_write`, so acquisition/rendezvous cost remains included.
    pub(crate) fn acquired(&self, requested: u64) -> Option<PauseGuard<'_>> {
        let acquired = self.timestamp();
        let sequence = self.next.fetch_add(1, Ordering::Relaxed);
        let slot = self.slots.get(sequence)?;
        slot.requested.store(requested, Ordering::Relaxed);
        slot.acquired.store(acquired, Ordering::Release);
        Some(PauseGuard {
            recorder: self,
            slot,
        })
    }

    /// Export after stopping/joining producers. This never waits on open events.
    /// A live-producer snapshot must not be called a complete capture. Consuming
    /// the recorder prevents reuse after exporting; sharing owners must first
    /// be released (for example by `Arc::try_unwrap` after collector shutdown).
    #[must_use]
    pub fn finish(self) -> PauseSnapshot {
        let end = self.timestamp();
        let count = self.next.load(Ordering::Acquire);
        let events: Vec<_> = self
            .slots
            .iter()
            .take(count)
            .enumerate()
            .map(|(sequence, slot)| {
                let complete = slot.complete.load(Ordering::Acquire);
                PauseEvent {
                    sequence,
                    requested_ns: slot.requested.load(Ordering::Relaxed),
                    acquired_ns: slot.acquired.load(Ordering::Acquire),
                    released_ns: complete.then(|| slot.released.load(Ordering::Relaxed)),
                }
            })
            .collect();
        let measurement_state = self.measurement_state.load(Ordering::Acquire);
        PauseSnapshot {
            measurement_state: match measurement_state {
                0 => "missing",
                1 => "begin_incomplete",
                2 => "open",
                3 => "end_incomplete",
                4 => "closed",
                _ => "invalid",
            },
            measurement_errors: self.measurement_errors.load(Ordering::Relaxed),
            measurement_begin: (measurement_state >= 2).then(|| self.measurement_begin.snapshot()),
            measurement_end: (measurement_state == 4).then(|| self.measurement_end.snapshot()),
            capture_start_ns: self.start,
            capture_end_ns: end,
            clock_resolution_ns: self.resolution,
            publication_attempts: self.attempts.load(Ordering::Relaxed),
            publication_acquisitions: count,
            publication_failed_attempts: self.failed_attempts.load(Ordering::Relaxed),
            dropped_events: count.saturating_sub(self.slots.len()),
            open_events: events
                .iter()
                .filter(|event| event.released_ns.is_none())
                .count(),
            clock_errors: self.clock_errors.load(Ordering::Relaxed),
            events,
        }
    }
}

pub(crate) struct PauseGuard<'a> {
    recorder: &'a PauseRecorder,
    slot: &'a Slot,
}

impl Drop for PauseGuard<'_> {
    fn drop(&mut self) {
        self.slot
            .released
            .store(self.recorder.timestamp(), Ordering::Relaxed);
        self.slot.complete.store(true, Ordering::Release);
    }
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "correctness assertions with fallible clock setup"
)]
mod tests {
    use super::*;

    #[test]
    fn completed_events_include_request_and_release() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(2)?;
        let requested = recorder.request();
        drop(recorder.acquired(requested));
        let snapshot = recorder.finish();
        assert_eq!(snapshot.events.len(), 1);
        let event = snapshot.events[0];
        assert!(snapshot.capture_start_ns <= event.requested_ns);
        assert!(event.requested_ns <= event.acquired_ns);
        assert!(Some(event.acquired_ns) <= event.released_ns);
        assert!(event.released_ns <= Some(snapshot.capture_end_ns));
        assert_eq!(
            (
                snapshot.dropped_events,
                snapshot.open_events,
                snapshot.clock_errors
            ),
            (0, 0, 0)
        );
        Ok(())
    }

    #[test]
    fn overflow_is_counted_without_reusing_evidence() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        drop(recorder.acquired(recorder.request()));
        assert!(recorder.acquired(recorder.request()).is_none());
        let snapshot = recorder.finish();
        assert_eq!(snapshot.events.len(), 1);
        assert_eq!(snapshot.dropped_events, 1);
        Ok(())
    }

    #[test]
    fn forgotten_guard_is_reported_as_open() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        std::mem::forget(recorder.acquired(recorder.request()));
        let snapshot = recorder.finish();
        assert_eq!(snapshot.open_events, 1);
        assert!(snapshot.events[0].released_ns.is_none());
        Ok(())
    }

    #[test]
    fn measurement_counts_exclude_startup_and_reporting() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        recorder.collection_completed();
        let begin = recorder.begin_measurement();
        recorder.collection_completed();
        recorder.collection_completed();
        let end = recorder.end_measurement();
        recorder.collection_completed();
        let snapshot = recorder.finish();
        assert_eq!(snapshot.measurement_state, "closed");
        assert_eq!(snapshot.measurement_errors, 0);
        let first = snapshot
            .measurement_begin
            .ok_or_else(|| std::io::Error::other("begin absent"))?;
        let last = snapshot
            .measurement_end
            .ok_or_else(|| std::io::Error::other("end absent"))?;
        assert_eq!(Some(first.timestamp_ns), begin);
        assert_eq!(Some(last.timestamp_ns), end);
        assert_eq!((first.collections_before, first.collections_after), (1, 1));
        assert_eq!((last.collections_before, last.collections_after), (3, 3));
        assert!(first.observer_start_ns <= first.timestamp_ns);
        assert!(first.timestamp_ns <= first.observer_end_ns);
        assert!(first.observer_end_ns <= last.observer_start_ns);
        Ok(())
    }

    #[test]
    fn invalid_boundary_protocol_stays_visible() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        assert!(recorder.end_measurement().is_none());
        assert!(recorder.begin_measurement().is_some());
        assert!(recorder.begin_measurement().is_none());
        assert!(recorder.end_measurement().is_some());
        assert!(recorder.end_measurement().is_none());
        let snapshot = recorder.finish();
        assert_eq!(snapshot.measurement_errors, 3);
        assert_eq!(snapshot.measurement_state, "closed");
        Ok(())
    }

    #[test]
    fn open_measurement_cannot_fabricate_an_end() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        assert!(recorder.begin_measurement().is_some());
        let snapshot = recorder.finish();
        assert_eq!(snapshot.measurement_state, "open");
        assert!(snapshot.measurement_begin.is_some());
        assert!(snapshot.measurement_end.is_none());
        Ok(())
    }

    #[test]
    fn simultaneous_measurement_starts_never_wait_or_overwrite() -> std::io::Result<()> {
        let recorder = PauseRecorder::new(1)?;
        let successes = std::thread::scope(|scope| {
            let other = scope.spawn(|| recorder.begin_measurement().is_some());
            let current = recorder.begin_measurement().is_some();
            other
                .join()
                .map(|other| usize::from(other) + usize::from(current))
                .map_err(|_| std::io::Error::other("boundary thread panicked"))
        })?;
        assert_eq!(successes, 1);
        assert!(recorder.end_measurement().is_some());
        assert_eq!(recorder.finish().measurement_errors, 1);
        Ok(())
    }

    #[test]
    fn zero_capacity_is_rejected() {
        assert!(PauseRecorder::new(0).is_err());
    }
}
