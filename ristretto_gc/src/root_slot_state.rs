//! Generation claims for root deletion. The parent supplies native or Loom atomics.
use super::{AtomicU64, AtomicUsize, Ordering};

// A managed allocation has at least one byte, including for a ZST. Its
// one-past-end pointer must fit in the address space.
pub(crate) const PENDING_ROOT: usize = usize::MAX;

pub(crate) fn arm_root(slot: &AtomicUsize) {
    slot.store(PENDING_ROOT, Ordering::Release);
}

pub(crate) fn activate_root(slot: &AtomicUsize, address: usize) -> bool {
    slot.compare_exchange(PENDING_ROOT, address, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
}

pub(crate) fn scan_address(slot: &AtomicUsize) -> usize {
    let value = slot.load(Ordering::Acquire);
    if value != PENDING_ROOT {
        return value;
    }
    // A successful skip prevents this attempt from activating. If the
    // publisher won, trace the address returned by the failed CAS.
    match slot.compare_exchange(PENDING_ROOT, 0, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => 0,
        Err(address) => address,
    }
}

pub(crate) const TAKEN: u64 = u64::MAX;

#[derive(Debug, Default)]
pub(crate) struct Slot {
    pub(crate) address: AtomicUsize,
    pub(crate) generation: AtomicU64,
}

impl Slot {
    /// The registry writer has acquired the free occupancy bit. The previous
    /// deletion cleared the address before releasing that bit to the writer.
    pub(crate) fn prepare_generation(&self, generation: u64) {
        debug_assert!(generation != 0 && generation != TAKEN);
        debug_assert_eq!(self.address.load(Ordering::Relaxed), 0);
        self.generation.store(generation, Ordering::Release);
    }

    /// Observe an address with one nonreused generation. The writer publishes
    /// a new generation with Release after the previous address was cleared.
    /// This prevents pairing that new generation with the previous address.
    pub(crate) fn matching_generation(&self, address: usize) -> Option<u64> {
        let generation = self.generation.load(Ordering::Acquire);
        if generation == 0
            || generation == TAKEN
            || self.address.load(Ordering::Acquire) != address
            || self.generation.load(Ordering::Acquire) != generation
        {
            return None;
        }
        Some(generation)
    }

    /// Exactly one remover claims this generation. A stale owner must never
    /// clear an address belonging to a later reuse of the same slot.
    pub(crate) fn take(&self, generation: u64) -> Option<usize> {
        debug_assert!(generation != 0 && generation != TAKEN);
        self.generation
            .compare_exchange(generation, TAKEN, Ordering::AcqRel, Ordering::Acquire)
            .ok()?;
        // The successful generation claim excludes other removers. A writer
        // cannot reuse this slot until the caller later releases its occupied
        // bit, so address clearing needs no second read-modify-write operation.
        let address = self.address.load(Ordering::Acquire);
        self.address.store(0, Ordering::Release);
        Some(address)
    }
}
