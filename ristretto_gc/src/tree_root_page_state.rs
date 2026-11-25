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
}

impl Entry {
    pub(super) fn publish(&self, generation: NonZeroU64, address: usize) {
        self.address.store(address, Ordering::Release);
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
        self.generation
            .compare_exchange(generation.get(), 0, Ordering::AcqRel, Ordering::Acquire)
            .ok()?;
        Some(self.address.load(Ordering::Acquire))
    }

    pub(super) fn read(&self) -> Option<(NonZeroU64, usize)> {
        let generation = NonZeroU64::new(self.generation.load(Ordering::Acquire))?;
        let address = self.address.load(Ordering::Acquire);
        (self.generation.load(Ordering::Acquire) == generation.get())
            .then_some((generation, address))
    }
}
