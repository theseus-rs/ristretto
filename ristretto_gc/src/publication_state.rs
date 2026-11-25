//! Nonblocking admission and phase publication. The parent supplies native or
//! Loom atomics so the model executes the same state machine as the collector.
use super::{AtomicBool, AtomicU64, Ordering};

const DOMAIN_BITS: u32 = 5;
const DOMAIN_MASK: u64 = (1 << DOMAIN_BITS) - 1;
const ACTIVE_DOMAINS: u64 = 1 | (1 << DOMAIN_BITS);
const PHASE_MASK: u64 = (1 << (DOMAIN_BITS * 2)) - 1;
// One CAS admits and stamps a publisher. Count and phase changes cannot pass
// one another between separate atomic words. Twenty count bits allow 1,048,575
// simultaneous (including nested) admissions; exhaustion fails before admission.
const ACTIVE_UNIT: u64 = PHASE_MASK + 1;
const ACTIVE_MASK: u64 = ((1 << 20) - 1) * ACTIVE_UNIT;
const LOW_MASK: u64 = PHASE_MASK | ACTIVE_MASK;
pub(crate) const REVISION: u64 = LOW_MASK + 1;
pub(crate) const MAX_REVISION: u64 = !LOW_MASK;

#[derive(Debug)]
pub(crate) struct Clock {
    state: AtomicU64,
    limit: u64,
    optimistic: AtomicBool,
}

/// Only the single phase coordinator may create/use an observation. It must be
/// discarded after any failed attempt; work checks happen after observe, before
/// transition. The type deliberately cannot be copied or cloned.
pub(crate) struct Observation(u64);

/// A read-only barrier check may span other publishers, but no phase/revision
/// change. Rebase permanently disables certification to prevent revision ABA.
pub(crate) struct Snapshot(u64);

fn domain_flags(state: u64, index: usize) -> u64 {
    let shift = if index == 0 { 0 } else { DOMAIN_BITS };
    (state >> shift) & DOMAIN_MASK
}

fn stage(state: u64, index: usize) -> Option<usize> {
    let flags = domain_flags(state, index);
    (flags & 1 != 0).then_some(usize::from((flags >> 2).to_le_bytes()[0]))
}

impl Snapshot {
    pub(crate) fn stage(&self, index: usize) -> Option<usize> {
        stage(self.0, index)
    }

    pub(crate) fn flags(&self, index: usize) -> u64 {
        domain_flags(self.0, index)
    }
}

/// Numeric certificates suffice for leaves and graphs after strong tracing.
/// Epoch-bound births instead retain the identity of each active domain.
pub(crate) enum Birth {
    Idle,
    Inline(Snapshot),
    Epochs(Snapshot),
}

impl Birth {
    /// Deletion has no tracing effect once every active domain has entered retirement.
    pub(crate) fn deletion_closed(&self) -> bool {
        match self {
            Self::Idle => true,
            Self::Inline(snapshot) | Self::Epochs(snapshot) => {
                (0..2).all(|i| snapshot.stage(i).is_none_or(|stage| stage == 5))
            }
        }
    }
}

impl Clock {
    pub(crate) fn new(limit: u64) -> Self {
        assert!(limit != 0 && limit & LOW_MASK == 0);
        Self {
            state: AtomicU64::new(0),
            limit,
            optimistic: AtomicBool::new(true),
        }
    }

    pub(crate) fn enter(&self) -> Admission<'_> {
        self.admit::<true>();
        Admission(self)
    }

    /// The caller has installed the initial root (if any) and all registry
    /// entries. The descriptor remains private until its visibility decision. An idle
    /// birth has no traversal debt. Release the index writes to the RMW that
    /// starts the next cycle, including its lock-free nursery enumeration.
    pub(crate) fn indexed_birth(&self, leaf: bool) -> Birth {
        let state = self.state.fetch_or(0, Ordering::SeqCst);
        if state & ACTIVE_DOMAINS == 0 {
            return Birth::Idle;
        }
        let snapshot = Snapshot(state & !ACTIVE_MASK);
        let strong_graph = !leaf && (snapshot.stage(0) == Some(0) || snapshot.stage(1) == Some(0));
        if !strong_graph
            && state & MAX_REVISION != self.limit
            && self.optimistic.load(Ordering::SeqCst)
        {
            Birth::Inline(snapshot)
        } else {
            Birth::Epochs(snapshot)
        }
    }

    /// Publish the armed root slot to later cycle starts. An active snapshot
    /// still requires exact strong coverage in each domain before activation.
    pub(crate) fn indexed_root(&self) -> Birth {
        self.indexed_birth(true)
    }

    /// The caller holds each active epoch's owning identity and must validate
    /// them after this operation. Unrelated publisher stamps do not change
    /// those identities. A linked traversal invalidates an earlier empty sample.
    /// At saturation no coordinator observation can commit until after rebase.
    #[expect(clippy::needless_pass_by_value, reason = "consume the birth snapshot")]
    pub(crate) fn certify_epochs(&self, snapshot: Snapshot, linked: bool) -> bool {
        let mut current = self.state.load(Ordering::SeqCst);
        while current & PHASE_MASK == snapshot.0 & PHASE_MASK {
            if !linked {
                return true;
            }
            let next = if current & MAX_REVISION == self.limit {
                current
            } else {
                current + REVISION
            };
            match self.state.compare_exchange_weak(
                current,
                next,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
        false
    }

    /// Admit a pure edge barrier. It must call `Admission::changed` before
    /// leaving if it publishes marking work or changes retention. A completed
    /// no-op need not invalidate the coordinator's earlier observation.
    pub(crate) fn enter_barrier(&self) -> Admission<'_> {
        self.admit::<false>();
        Admission(self)
    }

    fn admit<const STAMP: bool>(&self) {
        let mut observed = self.state.load(Ordering::SeqCst);
        loop {
            assert!(
                observed & ACTIVE_MASK != ACTIVE_MASK,
                "publication admission count exhausted"
            );
            let stamp = STAMP && observed & MAX_REVISION != self.limit;
            let next = observed + ACTIVE_UNIT + if stamp { REVISION } else { 0 };
            match self.state.compare_exchange_weak(
                observed,
                next,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return,
                Err(current) => observed = current,
            }
        }
    }

    pub(crate) fn active(&self) -> usize {
        ((self.state.load(Ordering::SeqCst) & ACTIVE_MASK) / ACTIVE_UNIT) as usize
    }

    pub(crate) fn flags(&self, index: usize) -> u64 {
        domain_flags(self.state.load(Ordering::SeqCst), index)
    }

    #[cfg(test)]
    pub(crate) fn disable_certification(&self) {
        self.optimistic.store(false, Ordering::SeqCst);
    }

    pub(crate) fn snapshot(&self) -> Option<Snapshot> {
        if !self.optimistic.load(Ordering::SeqCst) {
            return None;
        }
        let state = self.state.load(Ordering::SeqCst);
        (state & MAX_REVISION != self.limit).then_some(Snapshot(state & !ACTIVE_MASK))
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "consume the certification token"
    )]
    pub(crate) fn unchanged(&self, snapshot: Snapshot) -> bool {
        self.state.load(Ordering::SeqCst) & !ACTIVE_MASK == snapshot.0
            && self.optimistic.load(Ordering::SeqCst)
    }

    pub(crate) fn observe(&self) -> Option<Observation> {
        let observed = self.state.load(Ordering::SeqCst);
        if observed & MAX_REVISION == self.limit {
            if observed & ACTIVE_MASK == 0 {
                // A stalled read-only checker can outlive the coordinator's
                // observation. Never let it certify across numeric revision reuse.
                self.optimistic.store(false, Ordering::SeqCst);
                let _ = self.state.compare_exchange(
                    observed,
                    observed & PHASE_MASK,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
            }
            // Rebase never changes a phase. No observation may survive it.
            return None;
        }
        // Must precede every pending/in-flight/queue inspection. Otherwise an
        // already stamped publisher could finish after an empty queue sample.
        (observed & ACTIVE_MASK == 0).then_some(Observation(observed))
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "consume each observation exactly once"
    )]
    pub(crate) fn transition(&self, observed: Observation, index: usize, flags: u64) -> bool {
        assert!(flags <= DOMAIN_MASK);
        let shift = if index == 0 { 0 } else { DOMAIN_BITS };
        // Stamp every transition, including a same-flags transition. A barrier
        // snapshot must distinguish complete phase cycles from no phase change.
        let next = ((observed.0 & !(DOMAIN_MASK << shift)) | (flags << shift)) + REVISION;
        self.state
            .compare_exchange(observed.0, next, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }
}

pub(crate) struct Admission<'a>(&'a Clock);

impl Admission<'_> {
    /// Stamp a publication while its count still excludes phase changes.
    pub(crate) fn changed(&self) {
        let mut observed = self.0.state.load(Ordering::SeqCst);
        while observed & MAX_REVISION != self.0.limit {
            match self.0.state.compare_exchange_weak(
                observed,
                observed + REVISION,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return,
                Err(current) => observed = current,
            }
        }
        // Saturation already rejects observations until a quiescent rebase.
    }
}

impl Drop for Admission<'_> {
    fn drop(&mut self) {
        self.0.state.fetch_sub(ACTIVE_UNIT, Ordering::SeqCst);
    }
}
