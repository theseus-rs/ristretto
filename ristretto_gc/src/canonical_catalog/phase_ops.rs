//! Frozen guarded tracing and phase transitions for the Q=0 catalog.
#![expect(
    clippy::indexing_slicing,
    reason = "each domain index is checked by get before fixed phase-array access"
)]
use super::super::{RETIREMENT_FLOOR, TraceRetention};
use super::{Attempt, Catalog, Cell, DomainMode, Membership, Owned, Preparation, Record, Snapshot};
impl<R: Record> Catalog<R> {
    pub fn begin_shade(
        &self,
        snapshot: Snapshot<R>,
        cell: Owned<Cell<R>>,
        domain: usize,
        requested: TraceRetention,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.shade(snapshot.phase(), domain, requested).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_trace_acknowledge(
        &self,
        snapshot: Snapshot<R>,
        cell: Owned<Cell<R>>,
        domain: usize,
        ticket: u64,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.acknowledge(snapshot.phase(), domain, ticket).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_trace_retire(
        &self,
        snapshot: Snapshot<R>,
        cell: Owned<Cell<R>>,
        domain: usize,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let phase = snapshot.phase();
        marks.can_retire(phase, domain).ok()?;
        let effect = self.effect(cell, expected, Membership::Retired)?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    pub fn begin_promote(
        &self,
        snapshot: Snapshot<R>,
        cell: Owned<Cell<R>>,
    ) -> Option<Preparation<R>> {
        let expected = cell.state.load()?;
        let Membership::Tracing(marks) = *expected else {
            return None;
        };
        let (marks, phase) = marks.promote(snapshot.phase()).ok()?;
        let effect = self.effect(cell, expected, Membership::Tracing(marks))?;
        self.begin_prepare(snapshot, phase, Some(effect))
    }

    /// Numeric phase activation. Root enumeration and callback obligations are
    /// separate protocols; this operation is not a certificate for either.
    pub fn begin_start(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        mode: DomainMode,
    ) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if mode == DomainMode::Inactive
            || (mode == DomainMode::Old && domain != 1)
            || (mode == DomainMode::Young && domain != 0)
            || entry.mode != DomainMode::Inactive
            || entry.pending != 0
        {
            return None;
        }
        entry.mode = mode;
        entry.epoch = entry.epoch.checked_add(1)?;
        entry.floor = 0;
        phase.root_cursors[domain] = 0;
        phase.reference_cursors[domain] = 0;
        self.begin_prepare(snapshot, phase, None)
    }

    /// Bind a policy's completed sweep to its exact collection epoch. A queued
    /// request must not rebase its Finish onto a later, unswept collection.
    pub fn begin_finish_epoch(
        &self,
        snapshot: Snapshot<R>,
        domain: usize,
        epoch: u64,
    ) -> Option<Preparation<R>> {
        if snapshot.phase().domains.get(domain)?.epoch != epoch {
            return None;
        }
        self.begin_finish(snapshot, domain)
    }
    pub fn begin_finish(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if entry.mode == DomainMode::Inactive
            || entry.floor != RETIREMENT_FLOOR
            || entry.pending != 0
            || phase.root_cursors[domain] != self.root_capacity()
        {
            return None;
        }
        entry.mode = DomainMode::Inactive;
        entry.floor = 0;
        self.begin_prepare(snapshot, phase, None)
    }

    pub fn advance(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Attempt<R>> {
        let preparation = self.begin_advance(snapshot, domain)?;
        self.prepare_now(preparation)
    }

    pub fn begin_advance(&self, snapshot: Snapshot<R>, domain: usize) -> Option<Preparation<R>> {
        let mut phase = snapshot.phase();
        let entry = phase.domains.get_mut(domain)?;
        if entry.mode == DomainMode::Inactive
            || entry.pending != 0
            || entry.floor >= RETIREMENT_FLOOR
            || phase.root_cursors[domain] != self.root_capacity()
        {
            return None;
        }
        entry.floor += 1;
        phase.reference_cursors[domain] = 0;
        self.begin_prepare(snapshot, phase, None)
    }
}

#[cfg(test)]
#[path = "phase_ops_tests.rs"]
mod tests;
