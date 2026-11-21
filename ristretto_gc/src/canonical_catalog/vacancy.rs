//! A non-owning, expiring preference for one contended automatic allocation.
//! All writes still go through the allocation's normal identity-checked CAS.
use super::{AtomicU64, AtomicUsize, Ordering::Relaxed};

/// After this many unsuccessful probes, an automatic request may prepare its
/// private backing and ask for a recently reclaimed slot.
pub(crate) const PROBE_INTERVAL: usize = 128;
const OFFER_STRIDE: usize = 128;
pub(super) const DEFERRALS: usize = OFFER_STRIDE - 1;

#[derive(Default)]
pub(crate) struct Preference {
    // Non-reused creation identity. The packed offer carries the slot and a
    // bounded number of remaining deferrals. Zero in either word means idle.
    // These hints may be stale or sampled across different nominations. They
    // never authorize an install, which always compares the actual slot.
    waiter: AtomicU64,
    offer: AtomicUsize,
}

impl Preference {
    pub(crate) fn nominate(&self, identity: u64) -> bool {
        self.waiter
            .compare_exchange(0, identity, Relaxed, Relaxed)
            .is_ok()
    }

    pub(crate) fn offer(&self, slot: usize) {
        if self.waiter.load(Relaxed) != 0
            && let Some(word) = slot
                .checked_mul(OFFER_STRIDE)
                .and_then(|base| base.checked_add(DEFERRALS))
        {
            let _ = self.offer.compare_exchange(0, word, Relaxed, Relaxed);
        }
    }

    pub(crate) fn slot_for(&self, identity: u64) -> Option<usize> {
        if identity == 0 || self.waiter.load(Relaxed) != identity {
            return None;
        }
        let word = self.offer.load(Relaxed);
        (word != 0).then_some(word / OFFER_STRIDE)
    }

    /// A competing installation can win between releasing the slot and
    /// publishing its numeric hint. Let the nominee request a fresh location.
    pub(crate) fn missed(&self, identity: u64, slot: usize) {
        let word = self.offer.load(Relaxed);
        if word != 0 && word / OFFER_STRIDE == slot && self.waiter.load(Relaxed) == identity {
            let _ = self.offer.compare_exchange(word, 0, Relaxed, Relaxed);
        }
    }

    pub(crate) fn defer(&self, slot: usize, identity: u64) -> bool {
        let word = self.offer.load(Relaxed);
        if word == 0 || word / OFFER_STRIDE != slot {
            return false;
        }
        let waiting = self.waiter.load(Relaxed);
        if waiting == 0 || waiting == identity {
            return false;
        }
        if word % OFFER_STRIDE == 1 {
            if self
                .offer
                .compare_exchange(word, 0, Relaxed, Relaxed)
                .is_ok()
            {
                let _ = self.waiter.compare_exchange(waiting, 0, Relaxed, Relaxed);
            }
            return false;
        }
        self.offer
            .compare_exchange(word, word - 1, Relaxed, Relaxed)
            .is_ok()
    }

    pub(crate) fn finish(&self, identity: u64) {
        if self.waiter.load(Relaxed) == identity {
            let word = self.offer.load(Relaxed);
            if self
                .waiter
                .compare_exchange(identity, 0, Relaxed, Relaxed)
                .is_ok()
            {
                let _ = self.offer.compare_exchange(word, 0, Relaxed, Relaxed);
            }
        }
    }
}
