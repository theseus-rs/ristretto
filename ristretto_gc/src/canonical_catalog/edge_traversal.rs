//! Private resumable scans of the complete fixed edge inventory for Full domains.
//! Numeric acknowledgement and runtime inventory admission remain separate APIs.
use super::super::{TraceError, TraceRetention, reference::ObjectKey};
use super::edges::EdgeMembership;
use super::{
    Arc, Attempt, Catalog, Cell, DomainMode, Membership, Outcome, Owned, Owner, Preparation,
    PreparationError, Record, Snapshot, share,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeTraversalProgress {
    Pending,
    Complete,
    Superseded,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeTraversalError {
    Foreign,
    Finished,
    GenerationExhausted,
    CounterExhausted,
}

struct Request<R: Record> {
    preparation: Option<Preparation<R>>,
    attempt: Option<Attempt<R>>,
    acknowledge: bool,
}

/// A stopped scanner retains bounded private work. Any other helper can start a
/// complete scan of this same ticket. No shared progress depends on this owner.
pub struct EdgeTraversal<R: Record = ()> {
    owner: Arc<Owner>,
    source: Owned<Cell<R>>,
    domain: usize,
    epoch: u64,
    ticket: u64,
    retention: TraceRetention,
    cursor: usize,
    first_slot: usize,
    slots: usize,
    request: Option<Request<R>>,
    finished: bool,
}
impl<R: Record> Catalog<R> {
    /// Require a nonzero current ticket in a Full domain. This scan alone does
    /// not prove that the runtime has admitted all of a payload's actual edges.
    #[expect(
        clippy::indexing_slicing,
        reason = "domain is checked before fixed mark-array access"
    )]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "preserve the frozen owning-snapshot admission and destruction boundary"
    )]
    pub fn begin_edge_traversal(
        &self,
        snapshot: Snapshot<R>,
        source: Owned<Cell<R>>,
        domain: usize,
        ticket: u64,
    ) -> Option<EdgeTraversal<R>> {
        if !Arc::ptr_eq(&snapshot.owner, &self.owner)
            || !Arc::ptr_eq(&source.owner, &self.owner)
            || self.edges.is_none()
            || ticket == 0
        {
            return None;
        }
        let phase = snapshot.phase();
        let active = *phase.domains.get(domain)?;
        let Membership::Tracing(marks) = source.membership()? else {
            return None;
        };
        let mark = marks[domain];
        if active.mode != DomainMode::Full || mark.epoch != active.epoch || mark.ticket != ticket {
            return None;
        }
        let range = self.source_edge_range(source.slot())?;
        Some(EdgeTraversal {
            owner: self.owner.clone(),
            source,
            domain,
            epoch: active.epoch,
            ticket,
            retention: mark.retention?,
            cursor: 0,
            first_slot: range.start,
            slots: range.len(),
            request: None,
            finished: false,
        })
    }
}
impl<R: Record> EdgeTraversal<R> {
    /// Observational only; callers cannot advance or reset this cursor.
    pub fn scanned_slots(&self) -> usize {
        self.cursor
    }
    /// False preserves a request whose irreversible commit decision already won.
    pub fn cancel(&self) -> bool {
        self.request
            .as_ref()
            .and_then(|request| request.attempt.as_ref())
            .is_none_or(Attempt::cancel)
    }

    #[cfg(test)]
    pub(crate) fn install_pending_for_test(&self, catalog: &Catalog<R>) -> bool {
        self.request
            .as_ref()
            .and_then(|request| request.attempt.as_ref())
            .is_some_and(|attempt| catalog.install_and_stop(attempt))
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "private traversal retains an admitted fixed domain index"
    )]
    pub fn poll(
        &mut self,
        catalog: &Catalog<R>,
        context: &R::Context,
    ) -> Result<EdgeTraversalProgress, EdgeTraversalError> {
        if !Arc::ptr_eq(&self.owner, &catalog.owner) {
            return Err(EdgeTraversalError::Foreign);
        }
        if self.finished {
            return Err(EdgeTraversalError::Finished);
        }
        if self.request.is_some() {
            return self.poll_request(catalog, context);
        }
        let Some(snapshot) = catalog.snapshot_in(context) else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        let Some(membership) = self.source.membership() else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        let phase = snapshot.phase();
        if phase.revision == u64::MAX {
            return Err(EdgeTraversalError::CounterExhausted);
        }
        let current = matches!(membership, Membership::Tracing(marks)
            if marks[self.domain].epoch == self.epoch
                && marks[self.domain].ticket == self.ticket
                && marks[self.domain].retention == Some(self.retention))
            && phase.domains[self.domain].epoch == self.epoch
            && phase.domains[self.domain].mode == DomainMode::Full;
        if !current {
            self.finished = true;
            return Ok(EdgeTraversalProgress::Superseded);
        }
        if self.cursor == self.slots {
            if let Some(source) = share(&self.source)
                && let Some(preparation) =
                    catalog.begin_trace_acknowledge(snapshot, source, self.domain, self.ticket)
            {
                self.request = Some(Request {
                    preparation: Some(preparation),
                    attempt: None,
                    acknowledge: true,
                });
            }
            return Self::pending(catalog);
        }
        let Ok(edge) = catalog.discover_edge(self.first_slot + self.cursor) else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        let source = ObjectKey {
            identity: self.source.identity,
            slot: self.source.slot(),
        };
        let target = if let Some(edge) = edge {
            let Some(state) = edge.membership() else {
                return Ok(EdgeTraversalProgress::Pending);
            };
            match state {
                EdgeMembership::Live(state) if state.source == source => state.target,
                _ => None,
            }
        } else {
            None
        };
        let Some(target) = target else {
            self.cursor += 1;
            return Ok(EdgeTraversalProgress::Pending);
        };
        let Ok(Some(cell)) = catalog.discover(target.slot) else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        if cell.identity != target.identity {
            return Ok(EdgeTraversalProgress::Pending);
        }
        let Some(expected) = cell.state.load() else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        let Membership::Tracing(marks) = *expected else {
            return Ok(EdgeTraversalProgress::Pending);
        };
        match marks.shade(phase, self.domain, self.retention) {
            Ok((next, phase)) => {
                if let Some(effect) = catalog.effect(cell, expected, Membership::Tracing(next))
                    && let Some(preparation) = catalog.begin_prepare(snapshot, phase, Some(effect))
                {
                    self.request = Some(Request {
                        preparation: Some(preparation),
                        attempt: None,
                        acknowledge: false,
                    });
                }
                Self::pending(catalog)
            }
            Err(TraceError::AlreadyMarked) => {
                self.cursor += 1;
                Ok(EdgeTraversalProgress::Pending)
            }
            Err(TraceError::Exhausted) => Err(EdgeTraversalError::CounterExhausted),
            Err(TraceError::Conflict | TraceError::InvalidDomain) => {
                Ok(EdgeTraversalProgress::Pending)
            }
        }
    }

    fn pending(catalog: &Catalog<R>) -> Result<EdgeTraversalProgress, EdgeTraversalError> {
        if catalog.generation_exhaustion_observed() {
            Err(EdgeTraversalError::GenerationExhausted)
        } else {
            Ok(EdgeTraversalProgress::Pending)
        }
    }

    fn poll_request(
        &mut self,
        catalog: &Catalog<R>,
        context: &R::Context,
    ) -> Result<EdgeTraversalProgress, EdgeTraversalError> {
        let request = self.request.as_mut().expect("retained request");
        if let Some(preparation) = &mut request.preparation {
            match preparation.poll(catalog) {
                Ok(Some(attempt)) => {
                    request.attempt = Some(attempt);
                    request.preparation = None;
                }
                Ok(None) => {}
                Err(PreparationError::GenerationExhausted) => {
                    return Err(EdgeTraversalError::GenerationExhausted);
                }
                Err(PreparationError::Foreign | PreparationError::Finished) => {
                    unreachable!("same unfinished traversal preparation")
                }
            }
            return Ok(EdgeTraversalProgress::Pending);
        }
        match catalog.commit_in(request.attempt.as_ref().expect("prepared attempt"), context) {
            Outcome::Pending => Ok(EdgeTraversalProgress::Pending),
            Outcome::Committed => {
                let acknowledge = request.acknowledge;
                self.request = None;
                if acknowledge {
                    self.finished = true;
                    Ok(EdgeTraversalProgress::Complete)
                } else {
                    self.cursor += 1;
                    Ok(EdgeTraversalProgress::Pending)
                }
            }
            Outcome::Retry | Outcome::Cancelled => {
                self.request = None;
                Ok(EdgeTraversalProgress::Pending)
            }
        }
    }
}

#[cfg(test)]
#[path = "edge_traversal_tests.rs"]
mod tests;
