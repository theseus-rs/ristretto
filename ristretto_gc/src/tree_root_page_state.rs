//! Atomic state shared by the ordinary numeric root pages and the Loom models.
use super::{AtomicU64, AtomicUsize, NonZeroU64, OWNED_TREE_ROOT, Ordering};

const SEALED: u64 = 1 << 63;

#[derive(Debug, Default)]
pub(super) struct Occupancy(AtomicU64);

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Claim {
    Reserved,
    InUse,
    Sealed,
}

impl Occupancy {
    pub(super) fn claim(&self, bit: u64) -> Claim {
        match self
            .0
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |bits| {
                (bits & (SEALED | bit) == 0).then_some(bits | bit)
            }) {
            Ok(_) => Claim::Reserved,
            Err(SEALED) => Claim::Sealed,
            Err(_) => Claim::InUse,
        }
    }

    pub(super) fn release(&self, bit: u64) -> bool {
        self.0.fetch_and(!bit, Ordering::AcqRel) == bit
    }

    pub(super) fn seal_empty(&self) -> bool {
        self.0
            .compare_exchange(0, SEALED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(super) fn is_sealed(&self) -> bool {
        self.0.load(Ordering::Acquire) == SEALED
    }

    pub(super) fn slots(&self) -> u64 {
        self.0.load(Ordering::Acquire) & !SEALED
    }
}

#[derive(Debug, Default)]
pub(super) struct Entry {
    generation: AtomicU64,
    address: AtomicUsize,
    // Serial, serial | OWNED_TREE_ROOT, 0, and OWNED_TREE_ROOT distinguish
    // open, removing, vetoed, and detached. The full token keeps ownership.
    decision: AtomicU64,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Scan {
    Root(usize),
    Vacant,
    Retry,
}

impl Entry {
    pub(super) fn publish(&self, generation: NonZeroU64, address: usize) {
        self.address.store(address, Ordering::Release);
        self.decision
            .store(generation.get() & !OWNED_TREE_ROOT, Ordering::Release);
        self.generation.store(generation.get(), Ordering::Release);
    }

    pub(super) fn remove(&self, expected: Option<NonZeroU64>) -> Option<usize> {
        let generation = NonZeroU64::new(self.generation.load(Ordering::Acquire))?;
        if expected.map_or_else(
            || generation.get() & OWNED_TREE_ROOT != 0,
            |exact| exact != generation,
        ) {
            return None;
        }
        // An admitted removal owns the deletion barrier. Veto a competitor
        // before taking the full token, including when its coverage is ready.
        let serial = generation.get() & !OWNED_TREE_ROOT;
        let mut state = self.decision.load(Ordering::Acquire);
        while state != 0 {
            if state & !OWNED_TREE_ROOT != serial {
                return None;
            }
            match self
                .decision
                .compare_exchange(state, 0, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => break,
                Err(seen) => state = seen,
            }
        }
        self.generation
            .compare_exchange(generation.get(), 0, Ordering::AcqRel, Ordering::Acquire)
            .ok()?;
        Some(self.address.load(Ordering::Acquire))
    }

    pub(super) fn begin_removal(&self, generation: NonZeroU64) -> Option<usize> {
        let serial = generation.get() & !OWNED_TREE_ROOT;
        let intent = serial | OWNED_TREE_ROOT;
        if self.generation.load(Ordering::Acquire) != generation.get() {
            return None;
        }
        self.decision
            .compare_exchange(serial, intent, Ordering::AcqRel, Ordering::Acquire)
            .ok()?;
        let address = self.address.load(Ordering::Acquire);
        self.removal_pending(generation).then_some(address)
    }

    pub(super) fn removal_pending(&self, generation: NonZeroU64) -> bool {
        let intent = generation.get() | OWNED_TREE_ROOT;
        self.decision
            .compare_exchange(intent, intent, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(super) fn finish_removal(&self, generation: NonZeroU64) -> bool {
        let intent = generation.get() | OWNED_TREE_ROOT;
        if self
            .decision
            .compare_exchange(intent, OWNED_TREE_ROOT, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        // Only this winner can release occupancy. Compatibility removers see
        // detached, so the numeric position cannot be reused before this store.
        self.generation.store(0, Ordering::Release);
        true
    }

    pub(super) fn scan(&self) -> Scan {
        let generation = self.generation.load(Ordering::Acquire);
        if generation == 0 {
            return Scan::Vacant;
        }
        let serial = generation & !OWNED_TREE_ROOT;
        let state = self.decision.load(Ordering::Acquire);
        let address = self.address.load(Ordering::Acquire);
        if state == serial | OWNED_TREE_ROOT {
            if self
                .decision
                .compare_exchange(state, 0, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Scan::Root(address);
            }
        } else if (state == 0 || state == serial)
            && self.decision.load(Ordering::Acquire) == state
            && self.generation.load(Ordering::Acquire) == generation
        {
            return Scan::Root(address);
        }
        if state == OWNED_TREE_ROOT || self.generation.load(Ordering::Acquire) != generation {
            Scan::Vacant
        } else {
            Scan::Retry
        }
    }

    pub(super) fn read(&self) -> Option<(NonZeroU64, usize)> {
        let generation = NonZeroU64::new(self.generation.load(Ordering::Acquire))?;
        let address = self.address.load(Ordering::Acquire);
        (self.decision.load(Ordering::Acquire) != OWNED_TREE_ROOT
            && self.generation.load(Ordering::Acquire) == generation.get())
        .then_some((generation, address))
    }
}
