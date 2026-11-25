//! Epoch and reachability strength. The parent supplies runtime or Loom atomics.
use super::{AtomicU64, Ordering};

/// Epoch zero is unmarked; the low bits encode reference retention strength.
pub(crate) const EPOCH_STRIDE: u64 = 4;
const RETENTION_MASK: u64 = EPOCH_STRIDE - 1;
const BORN_AFTER_RETIREMENT: u64 = 3;

/// Smaller values are stronger. A receiving domain clamps inherited strength to
/// its closed reference boundary before calling `MarkWord::mark`.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Retention {
    Strong = 0,
    Final = 1,
    Phantom = 2,
}

impl Retention {
    pub(crate) fn at_phase(phase: usize) -> Self {
        match phase {
            0 => Self::Strong,
            1 | 2 => Self::Final,
            _ => Self::Phantom,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct MarkWord(AtomicU64);

impl MarkWord {
    pub(crate) fn word(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub(crate) fn marked(&self, epoch: u64) -> bool {
        self.word() & !RETENTION_MASK == epoch
    }

    pub(crate) fn retention(&self, epoch: u64) -> Option<Retention> {
        let word = self.word();
        (word & !RETENTION_MASK == epoch && word & RETENTION_MASK != BORN_AFTER_RETIREMENT)
            .then_some(match word & RETENTION_MASK {
                0 => Retention::Strong,
                1 => Retention::Final,
                _ => Retention::Phantom,
            })
    }

    pub(crate) fn marked_before(&self, epoch: u64, final_retention: bool) -> bool {
        let word = self.word();
        word & !RETENTION_MASK == epoch && word & RETENTION_MASK <= u64::from(final_retention)
    }

    pub(crate) fn mark_black(&self, epoch: u64) {
        self.0.store(epoch, Ordering::Release);
    }

    /// Physically live for this retiring domain, but not a root for the other
    /// generation's next snapshot. No tracing/reference phase remains in this domain.
    pub(crate) fn mark_born(&self, epoch: u64) {
        self.0
            .store(epoch | BORN_AFTER_RETIREMENT, Ordering::Release);
    }

    pub(crate) fn reset(&self) {
        self.0.store(0, Ordering::Release);
    }

    /// True means enqueue a fresh traversal, including when a stronger discovery
    /// supersedes an earlier complete or partial traversal in this same epoch.
    pub(crate) fn mark(&self, epoch: u64, strength: Retention) -> bool {
        let retention = strength as u64;
        self.0
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |word| {
                (word & !RETENTION_MASK != epoch || word & RETENTION_MASK > retention)
                    .then_some(epoch | retention)
            })
            .is_ok()
    }
}
