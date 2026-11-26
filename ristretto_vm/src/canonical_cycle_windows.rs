//! Four passive observations bracket the recorder's actual clock boundaries.
//! Bounds describe primary full epochs, never object destruction or all domains.
use super::{Inner, Outcome, Span, SpanId, Telemetry};
use ristretto_gc::canonical::{Mutator, PrimaryCycles};
use std::rc::Rc;

/// Raw observations around one entry-body measurement event.
/// Missing observations are unavailable evidence, not zero completed cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CycleWindow {
    pub measurement: Option<SpanId>,
    pub before_start: Option<PrimaryCycles>,
    pub after_start: Option<PrimaryCycles>,
    pub before_end: Option<PrimaryCycles>,
    pub after_end: Option<PrimaryCycles>,
}

impl CycleWindow {
    /// Conservative completed primary full-epoch bounds inside the clock window.
    /// Returns None for a missing event/observation or inconsistent counter order.
    /// Observer calls do not help collection, but releasing their temporary owners
    /// can run destruction. These are instrumented execution observations only.
    #[must_use]
    pub fn bounds(self) -> Option<PrimaryCycles> {
        self.measurement?;
        let samples = [
            self.before_start?,
            self.after_start?,
            self.before_end?,
            self.after_end?,
        ];
        // Check existence of a nondecreasing sequence through all intervals.
        // Overlapping intervals may legitimately have decreasing lower bounds.
        let mut minimum = 0;
        for sample in samples {
            minimum = minimum.max(sample.lower);
            if minimum > sample.upper {
                return None;
            }
        }
        Some(PrimaryCycles {
            lower: samples[2].lower.saturating_sub(samples[1].upper),
            upper: samples[3].upper.checked_sub(samples[0].lower)?,
        })
    }
}

/// One recorder cannot combine cycle counters from multiple dedicated heaps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CycleCapture {
    Unobserved,
    Incomplete,
    MultipleWindows,
    /// Sampling reached the end boundary. Individual observations may still be
    /// unavailable; execution outcome and service completeness are separate.
    Complete(CycleWindow),
}

pub(crate) struct CycleMeasurement {
    inner: Rc<Inner>,
    span: Span,
    before_start: Option<PrimaryCycles>,
    after_start: Option<PrimaryCycles>,
}

impl CycleMeasurement {
    pub(crate) fn new(telemetry: &Telemetry, mutator: &mut Mutator<'_>) -> Self {
        let next = if telemetry.0.cycles.get() == CycleCapture::Unobserved {
            CycleCapture::Incomplete
        } else {
            CycleCapture::MultipleWindows
        };
        telemetry.0.cycles.set(next);
        let before_start = mutator.primary_cycles().ok();
        let span = telemetry.measurement();
        let after_start = mutator.primary_cycles().ok();
        Self {
            inner: telemetry.0.clone(),
            span,
            before_start,
            after_start,
        }
    }

    pub(crate) fn finish(self, mutator: &mut Mutator<'_>, outcome: Outcome) {
        let measurement = self.span.id();
        let before_end = mutator.primary_cycles().ok();
        self.span.finish(outcome);
        let after_end = mutator.primary_cycles().ok();
        if self.inner.cycles.get() == CycleCapture::Incomplete {
            self.inner.cycles.set(CycleCapture::Complete(CycleWindow {
                measurement,
                before_start: self.before_start,
                after_start: self.after_start,
                before_end,
                after_end,
            }));
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "finite cycle interval assertions"
)]
mod tests {
    use super::*;
    use crate::canonical_telemetry::Binding;

    fn telemetry() -> std::io::Result<Telemetry> {
        Telemetry::new(
            16,
            Binding {
                process_id: super::super::current_process_id(),
                run_id: "cycle-window-algebra".into(),
                source_sha256: "a".repeat(64),
                binary_sha256: "b".repeat(64),
                workload_sha256: "c".repeat(64),
                configuration_sha256: "d".repeat(64),
            },
        )
    }

    #[test]
    fn bounds_bracket_clocks_and_reject_missing_or_impossible_order() -> std::io::Result<()> {
        let telemetry = telemetry()?;
        let span = telemetry.measurement();
        let mut window = CycleWindow {
            measurement: span.id(),
            before_start: Some(PrimaryCycles { lower: 2, upper: 3 }),
            after_start: Some(PrimaryCycles { lower: 3, upper: 5 }),
            before_end: Some(PrimaryCycles { lower: 8, upper: 9 }),
            after_end: Some(PrimaryCycles {
                lower: 9,
                upper: 10,
            }),
        };
        assert_eq!(window.bounds(), Some(PrimaryCycles { lower: 3, upper: 8 }));
        window.before_end = None;
        assert_eq!(window.bounds(), None);
        window.before_end = Some(PrimaryCycles { lower: 0, upper: 1 });
        assert_eq!(window.bounds(), None);
        window.before_end = Some(PrimaryCycles { lower: 6, upper: 5 });
        assert_eq!(window.bounds(), None);
        window.before_start = Some(PrimaryCycles {
            lower: u64::MAX,
            upper: u64::MAX,
        });
        window.after_start = window.before_start;
        window.before_end = window.before_start;
        window.after_end = window.before_start;
        assert_eq!(window.bounds(), Some(PrimaryCycles { lower: 0, upper: 0 }));
        window.after_start = Some(PrimaryCycles {
            lower: 0,
            upper: u64::MAX,
        });
        assert_eq!(window.bounds(), Some(PrimaryCycles { lower: 0, upper: 0 }));
        window.measurement = None;
        assert_eq!(window.bounds(), None);
        span.finish(Outcome::Complete);
        Ok(())
    }

    #[test]
    fn interval_bounds_enclose_every_small_monotone_clock_history() -> std::io::Result<()> {
        let telemetry = telemetry()?;
        let span = telemetry.measurement();
        // Enumerate actual counter values at six ordered points: observation,
        // start clock, observation, observation, end clock, observation. Choose
        // every interval containing each observed value, independently of bounds().
        let intervals = |value| {
            (0..=value)
                .flat_map(move |lower| (value..=2).map(move |upper| PrimaryCycles { lower, upper }))
                .collect::<Vec<_>>()
        };
        let mut histories = 0;
        for mut encoding in 0..3_u64.pow(6) {
            let history: [u64; 6] = std::array::from_fn(|_| {
                let value = encoding % 3;
                encoding /= 3;
                value
            });
            if !history.is_sorted() {
                continue;
            }
            histories += 1;
            let [before, start, after_start, before_end, end, after] = history;
            let choices = [before, after_start, before_end, after].map(intervals);
            let combinations = choices.iter().map(Vec::len).product::<usize>();
            for mut selection in 0..combinations {
                let [before_start, after_start, before_end, after_end] =
                    choices.each_ref().map(|options| {
                        let selected = options.get(selection % options.len()).copied();
                        selection /= options.len();
                        selected
                    });
                let window = CycleWindow {
                    measurement: span.id(),
                    before_start,
                    after_start,
                    before_end,
                    after_end,
                };
                let bounds = window
                    .bounds()
                    .ok_or_else(|| std::io::Error::other("feasible history rejected"))?;
                assert!(bounds.lower <= end - start && end - start <= bounds.upper);
            }
        }
        assert_eq!(histories, 28);
        span.finish(Outcome::Complete);
        Ok(())
    }
}
