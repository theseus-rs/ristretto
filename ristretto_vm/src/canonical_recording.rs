//! Optional per-invocation recorder. Without the feature these guards contain no
//! state and every operation is a no-op; ordinary runs never acquire a recorder.
#[cfg(feature = "experimental-canonical-telemetry")]
use crate::canonical_telemetry::{CycleMeasurement, Span as RecordedSpan, Telemetry};
#[cfg(feature = "experimental-canonical-telemetry")]
pub(super) use crate::canonical_telemetry::{Kind, Operation, Outcome, SpanId};
use ristretto_gc::canonical::Mutator;

#[cfg(not(feature = "experimental-canonical-telemetry"))]
#[derive(Clone, Copy)]
pub(super) enum Kind {
    Preparation,
    RequestPoll,
    HelperPoll,
}
#[cfg(not(feature = "experimental-canonical-telemetry"))]
#[derive(Clone, Copy)]
pub(super) enum Operation {
    Allocation,
    ChildRead,
    EdgeWrite,
    Maintenance,
    PrimitiveArrayPreparation,
}
#[cfg(not(feature = "experimental-canonical-telemetry"))]
#[derive(Clone, Copy)]
pub(super) enum Outcome {
    Complete,
    Pending,
    Failed,
}
#[cfg(not(feature = "experimental-canonical-telemetry"))]
pub(super) type SpanId = ();

#[derive(Clone, Default)]
pub(super) struct Recording {
    #[cfg(feature = "experimental-canonical-telemetry")]
    telemetry: Option<Telemetry>,
}
pub(super) struct Span {
    #[cfg(feature = "experimental-canonical-telemetry")]
    inner: Option<RecordedSpan>,
}
pub(super) struct Measurement {
    #[cfg(feature = "experimental-canonical-telemetry")]
    inner: Option<CycleMeasurement>,
}
#[cfg_attr(
    not(feature = "experimental-canonical-telemetry"),
    expect(
        clippy::unused_self,
        reason = "feature-disabled recorder preserves the same zero-state interface"
    )
)]
impl Recording {
    #[cfg(feature = "experimental-canonical-telemetry")]
    pub(super) fn new(telemetry: &Telemetry) -> Self {
        Self {
            telemetry: Some(telemetry.clone()),
        }
    }
    pub(super) fn request(&self, operation: Operation) -> Span {
        #[cfg(feature = "experimental-canonical-telemetry")]
        {
            Span {
                inner: self
                    .telemetry
                    .as_ref()
                    .map(|telemetry| telemetry.request(operation)),
            }
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        {
            let _ = operation;
            Span {}
        }
    }
    pub(super) fn service(&self, kind: Kind, operation: Operation, parent: Option<SpanId>) -> Span {
        #[cfg(feature = "experimental-canonical-telemetry")]
        {
            Span {
                inner: self
                    .telemetry
                    .as_ref()
                    .map(|telemetry| telemetry.service(kind, operation, parent)),
            }
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        {
            let _ = (kind, operation, parent);
            Span {}
        }
    }
    pub(super) fn measurement(&self, mutator: &mut Mutator<'_>) -> Measurement {
        #[cfg(feature = "experimental-canonical-telemetry")]
        {
            Measurement {
                inner: self
                    .telemetry
                    .as_ref()
                    .map(|telemetry| CycleMeasurement::new(telemetry, mutator)),
            }
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        {
            let _ = mutator;
            Measurement {}
        }
    }
    pub(super) fn prepare<T, E>(
        &self,
        operation: Operation,
        parent: Option<SpanId>,
        prepare: impl FnOnce() -> Result<T, E>,
    ) -> Result<T, E> {
        let span = self.service(Kind::Preparation, operation, parent);
        let result = prepare();
        span.finish_result(&result);
        result
    }
}

impl Measurement {
    #[cfg_attr(
        not(feature = "experimental-canonical-telemetry"),
        expect(
            clippy::unused_self,
            reason = "feature-disabled measurement is zero state"
        )
    )]
    pub(super) fn finish_result<T, E>(self, mutator: &mut Mutator<'_>, result: &Result<T, E>) {
        #[cfg(feature = "experimental-canonical-telemetry")]
        if let Some(inner) = self.inner {
            inner.finish(
                mutator,
                if result.is_ok() {
                    Outcome::Complete
                } else {
                    Outcome::Failed
                },
            );
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        let _ = (mutator, result);
    }
}
#[cfg_attr(
    not(feature = "experimental-canonical-telemetry"),
    expect(
        clippy::unused_self,
        reason = "feature-disabled guard preserves the same zero-state interface"
    )
)]
impl Span {
    pub(super) fn id(&self) -> Option<SpanId> {
        #[cfg(feature = "experimental-canonical-telemetry")]
        {
            self.inner.as_ref().and_then(RecordedSpan::id)
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        {
            None
        }
    }
    pub(super) fn finish(self, outcome: Outcome) {
        #[cfg(feature = "experimental-canonical-telemetry")]
        {
            if let Some(span) = self.inner {
                span.finish(outcome);
            }
        }
        #[cfg(not(feature = "experimental-canonical-telemetry"))]
        {
            let _ = outcome;
        }
    }
    pub(super) fn finish_result<T, E>(self, result: &Result<T, E>) {
        self.finish(if result.is_ok() {
            Outcome::Complete
        } else {
            Outcome::Failed
        });
    }
}
