//! Fixed-capacity canonical service telemetry. This is not global-pause coverage.
//! Setup/export allocate; begin/finish only access fixed slots, atomics and the
//! warmed monotonic clock. Recording never waits for a producer or retries a CAS.
//! Consume the recorder only after all producers and guards have been released.

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};

pub const PROTOCOL: &str = "ristretto-canonical-service-v1";
pub const MISSING_SOURCES: &[&str] = &[
    "owner_release",
    "native_execution_and_frame_publication",
    "cancellation_and_teardown",
    "global_actor_state",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Kind {
    Request = 1,
    Preparation = 2,
    RequestPoll = 3,
    HelperPoll = 4,
    Measurement = 5,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Operation {
    Allocation = 1,
    ChildRead = 2,
    EdgeWrite = 3,
    Maintenance = 4,
    Measurement = 5,
    PrimitiveArrayPreparation = 6,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Outcome {
    Complete = 1,
    Pending = 2,
    Failed = 3,
    Abandoned = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpanId {
    recorder: u64,
    index: usize,
}

impl SpanId {
    #[must_use]
    pub fn recorder_id(self) -> u64 {
        self.recorder
    }
    #[must_use]
    pub fn sequence(self) -> usize {
        self.index
    }
}

static NEXT_RECORDER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Default)]
struct Slot {
    // 0 unused, 1 open, 2 finishing, 3 completed. Only one finisher succeeds.
    state: AtomicU8,
    kind: AtomicU8,
    operation: AtomicU8,
    parent: AtomicUsize,
    start: AtomicU64,
    end: AtomicU64,
    outcome: AtomicU8,
}

#[derive(Debug)]
pub struct Event {
    pub id: SpanId,
    pub parent: Option<SpanId>,
    pub kind: Kind,
    pub operation: Operation,
    pub start_ns: u64,
    pub end_ns: Option<u64>,
    pub outcome: Option<Outcome>,
}

/// Time inside a request but outside its recorded synchronous service spans.
/// This can contain scheduler delay or uninstrumented runtime work, not just GC.
#[derive(Debug)]
pub struct Gap {
    pub request: SpanId,
    pub start_ns: u64,
    pub end_ns: u64,
    pub after_pending: bool,
}

#[derive(Debug)]
pub struct Snapshot {
    pub protocol: &'static str,
    pub clock: &'static str,
    pub clock_resolution_ns: Option<u64>,
    pub capture_start_ns: u64,
    pub capture_end_ns: Option<u64>,
    pub capacity: usize,
    pub overflow: bool,
    pub open_events: usize,
    pub issues: Vec<&'static str>,
    pub events: Vec<Event>,
    pub gaps: Vec<Gap>,
    pub measurement: Option<SpanId>,
    pub instrumented_capture_complete: bool,
    pub global_capture_complete: bool,
    pub missing_sources: &'static [&'static str],
}

#[derive(Debug)]
pub struct ServiceRecorder {
    identity: u64,
    slots: Box<[Slot]>,
    next: AtomicUsize,
    overflow: AtomicBool,
    invalid: AtomicBool,
    clock_error: AtomicBool,
    start: u64,
    resolution: Option<u64>,
}

impl ServiceRecorder {
    /// Preallocate and initialize the clock before running measured work.
    ///
    /// # Errors
    /// Rejects zero/excessive capacity or an unavailable clock.
    pub fn new(capacity: usize) -> std::io::Result<Self> {
        if capacity == 0 || capacity > u32::MAX as usize {
            return Err(std::io::Error::other("invalid service recorder capacity"));
        }
        let start = crate::clock::monotonic_time_ns()
            .ok_or_else(|| std::io::Error::other("monotonic clock unavailable"))?;
        let identity = NEXT_RECORDER
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map_err(|_| std::io::Error::other("recorder identity exhausted"))?;
        Ok(Self {
            identity,
            slots: (0..capacity).map(|_| Slot::default()).collect(),
            next: AtomicUsize::new(0),
            overflow: AtomicBool::new(false),
            invalid: AtomicBool::new(false),
            clock_error: AtomicBool::new(false),
            start,
            resolution: crate::clock::monotonic_resolution_ns(),
        })
    }

    /// Starts one event without allocation or retry. Exhaustion poisons capture;
    /// it never changes or waits for runtime work. Parent must be a request ID.
    #[must_use]
    pub fn begin(
        &self,
        kind: Kind,
        operation: Operation,
        parent: Option<SpanId>,
    ) -> Option<SpanId> {
        if parent.is_some_and(|id| id.recorder != self.identity) {
            self.invalid.store(true, Ordering::Relaxed);
            return None;
        }
        if self.overflow.load(Ordering::Relaxed) {
            return None;
        }
        let index = self.next.fetch_add(1, Ordering::Relaxed);
        let Some(slot) = self.slots.get(index) else {
            self.overflow.store(true, Ordering::Relaxed);
            return None;
        };
        let start = self.time();
        slot.kind.store(kind as u8, Ordering::Relaxed);
        slot.operation.store(operation as u8, Ordering::Relaxed);
        slot.parent
            .store(parent.map_or(0, |id| id.index + 1), Ordering::Relaxed);
        slot.start.store(start.unwrap_or(0), Ordering::Relaxed);
        slot.state.store(1, Ordering::Release);
        Some(SpanId {
            recorder: self.identity,
            index,
        })
    }

    /// Ends one event; duplicate/invalid completion poisons capture. An abandoned
    /// guard is diagnostic evidence of missing cancellation coverage, not success.
    pub fn finish(&self, id: SpanId, outcome: Outcome) {
        if id.recorder != self.identity {
            self.invalid.store(true, Ordering::Relaxed);
            return;
        }
        let Some(slot) = self.slots.get(id.index) else {
            self.invalid.store(true, Ordering::Relaxed);
            return;
        };
        if slot
            .state
            .compare_exchange(1, 2, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            self.invalid.store(true, Ordering::Relaxed);
            return;
        }
        slot.end.store(self.time().unwrap_or(0), Ordering::Relaxed);
        slot.outcome.store(outcome as u8, Ordering::Relaxed);
        slot.state.store(3, Ordering::Release);
    }

    fn time(&self) -> Option<u64> {
        let value = crate::clock::monotonic_time_ns();
        if value.is_none() {
            self.clock_error.store(true, Ordering::Relaxed);
        }
        value
    }

    /// Export after producers stop. This may allocate and sort. It never claims
    /// global completeness, including when no service event was recorded.
    #[must_use]
    pub fn snapshot(self) -> Snapshot {
        let end = self.time();
        let mut snapshot = Snapshot {
            protocol: PROTOCOL,
            clock: crate::clock::monotonic_clock_name(),
            clock_resolution_ns: self.resolution,
            capture_start_ns: self.start,
            capture_end_ns: end,
            capacity: self.slots.len(),
            overflow: self.overflow.load(Ordering::Relaxed),
            open_events: 0,
            issues: Vec::new(),
            events: Vec::new(),
            gaps: Vec::new(),
            measurement: None,
            instrumented_capture_complete: false,
            global_capture_complete: false,
            missing_sources: MISSING_SOURCES,
        };
        if snapshot.overflow {
            snapshot.issues.push("overflow");
        }
        if self.invalid.load(Ordering::Relaxed) {
            snapshot.issues.push("invalid_completion");
        }
        if self.clock_error.load(Ordering::Relaxed) {
            snapshot.issues.push("clock_failure");
        }
        for (index, slot) in self
            .slots
            .iter()
            .enumerate()
            .take(self.next.load(Ordering::Relaxed))
        {
            let state = slot.state.load(Ordering::Acquire);
            let kind = match slot.kind.load(Ordering::Relaxed) {
                1 => Kind::Request,
                2 => Kind::Preparation,
                3 => Kind::RequestPoll,
                4 => Kind::HelperPoll,
                5 => Kind::Measurement,
                _ => {
                    snapshot.issues.push("uninitialized_slot");
                    continue;
                }
            };
            let operation = match slot.operation.load(Ordering::Relaxed) {
                1 => Operation::Allocation,
                2 => Operation::ChildRead,
                3 => Operation::EdgeWrite,
                4 => Operation::Maintenance,
                5 => Operation::Measurement,
                6 => Operation::PrimitiveArrayPreparation,
                _ => {
                    snapshot.issues.push("invalid_operation");
                    continue;
                }
            };
            let outcome = match slot.outcome.load(Ordering::Relaxed) {
                1 => Some(Outcome::Complete),
                2 => Some(Outcome::Pending),
                3 => Some(Outcome::Failed),
                4 => Some(Outcome::Abandoned),
                _ => None,
            };
            if state != 3 {
                snapshot.open_events += 1;
            }
            snapshot.events.push(Event {
                id: SpanId {
                    recorder: self.identity,
                    index,
                },
                parent: slot
                    .parent
                    .load(Ordering::Relaxed)
                    .checked_sub(1)
                    .map(|index| SpanId {
                        recorder: self.identity,
                        index,
                    }),
                kind,
                operation,
                start_ns: slot.start.load(Ordering::Relaxed),
                end_ns: (state == 3).then(|| slot.end.load(Ordering::Relaxed)),
                outcome,
            });
        }
        snapshot.validate();
        snapshot
    }
}

impl Snapshot {
    fn validate(&mut self) {
        if self
            .capture_end_ns
            .is_some_and(|end| end < self.capture_start_ns)
        {
            self.issues.push("clock_order");
        }
        if self.open_events != 0 {
            self.issues.push("open_events");
        }
        for event in &self.events {
            if event.start_ns < self.capture_start_ns
                || event.end_ns.is_some_and(|end| {
                    end < event.start_ns || self.capture_end_ns.is_some_and(|capture| end > capture)
                })
            {
                self.issues.push("clock_order");
            }
            if event.outcome == Some(Outcome::Abandoned) {
                self.issues.push("abandoned_event");
            }
            if event.outcome == Some(Outcome::Pending)
                && !matches!(event.kind, Kind::RequestPoll | Kind::HelperPoll)
            {
                self.issues.push("invalid_pending_outcome");
            }
            if let Some(parent) = event.parent {
                let valid = self
                    .events
                    .get(parent.index)
                    .filter(|candidate| candidate.id == parent)
                    .is_some_and(|request| {
                        request.kind == Kind::Request
                            && request.start_ns <= event.start_ns
                            && event
                                .end_ns
                                .zip(request.end_ns)
                                .is_none_or(|(end, request_end)| end <= request_end)
                    });
                if !valid || matches!(event.kind, Kind::Request | Kind::Measurement) {
                    self.issues.push("invalid_request_parent");
                }
            } else if matches!(event.kind, Kind::Preparation | Kind::RequestPoll) {
                self.issues.push("missing_request_parent");
            }
            if event.kind == Kind::Measurement {
                if self.measurement.replace(event.id).is_some() {
                    self.issues.push("multiple_measurement_windows");
                }
                if event.outcome != Some(Outcome::Complete) {
                    self.issues.push("incomplete_measurement_window");
                }
            }
        }
        if self.measurement.is_none() {
            self.issues.push("missing_measurement_window");
        }
        self.validate_window();
        self.collect_gaps();
        self.issues.sort_unstable();
        self.issues.dedup();
        self.instrumented_capture_complete = self.issues.is_empty();
    }

    fn validate_window(&mut self) {
        if let Some(window) = self
            .events
            .iter()
            .find(|event| Some(event.id) == self.measurement)
            && let Some(end) = window.end_ns
        {
            for event in &self.events {
                if event.kind == Kind::Measurement {
                    continue;
                }
                if event.start_ns < end
                    && event.end_ns.is_none_or(|finish| finish > window.start_ns)
                    && (event.start_ns < window.start_ns
                        || event.end_ns.is_none_or(|finish| finish > end))
                {
                    self.issues.push("measurement_boundary_crossing");
                }
            }
        }
    }

    fn collect_gaps(&mut self) {
        let mut grouped = std::collections::BTreeMap::<usize, Vec<&Event>>::new();
        for event in &self.events {
            if let Some(parent) = event.parent {
                grouped.entry(parent.index).or_default().push(event);
            }
        }
        for request in self
            .events
            .iter()
            .filter(|event| event.kind == Kind::Request)
        {
            let Some(end) = request.end_ns else {
                continue;
            };
            let mut children = grouped.remove(&request.id.index).unwrap_or_default();
            children.sort_by_key(|event| event.start_ns);
            let mut cursor = request.start_ns;
            let mut pending = false;
            for child in children {
                let Some(finish) = child.end_ns else {
                    continue;
                };
                if child.start_ns < cursor || finish > end || finish < child.start_ns {
                    self.issues.push("overlapping_or_uncontained_service");
                    continue;
                }
                if child.start_ns > cursor {
                    self.gaps.push(Gap {
                        request: request.id,
                        start_ns: cursor,
                        end_ns: child.start_ns,
                        after_pending: pending,
                    });
                }
                if child.kind == Kind::RequestPoll {
                    pending = child.outcome == Some(Outcome::Pending);
                }
                cursor = finish;
            }
            if cursor < end {
                self.gaps.push(Gap {
                    request: request.id,
                    start_ns: cursor,
                    end_ns: end,
                    after_pending: pending,
                });
            }
        }
    }
}

#[cfg(test)]
#[path = "service_recorder_tests.rs"]
mod tests;
