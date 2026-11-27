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
// SlotRoot packs a generation above a six-bit slot number. These two bits
// cannot occur in an issued generation, including its final usable value.
const REMOVING: u64 = 1 << 63;
const VETOED: u64 = 1 << 62;
const TAGS: u64 = REMOVING | VETOED;

#[derive(Debug, Default)]
pub(crate) struct Slot {
    pub(crate) address: AtomicUsize,
    pub(crate) generation: AtomicU64,
}

impl Slot {
    /// The registry writer has acquired the free occupancy bit. The previous
    /// deletion cleared the address before releasing that bit to the writer.
    pub(crate) fn prepare_generation(&self, generation: u64) {
        debug_assert!(generation != 0 && generation & TAGS == 0);
        debug_assert_eq!(self.address.load(Ordering::Relaxed), 0);
        self.generation.store(generation, Ordering::Release);
    }

    /// Observe an address with one nonreused generation. The writer publishes
    /// a new generation with Release after the previous address was cleared.
    /// This prevents pairing that new generation with the previous address.
    pub(crate) fn matching_generation(&self, address: usize) -> Option<u64> {
        let state = self.generation.load(Ordering::Acquire);
        let generation = state & !TAGS;
        if state == 0
            || state == TAKEN
            || self.address.load(Ordering::Acquire) != address
            || self.generation.load(Ordering::Acquire) & !TAGS != generation
        {
            return None;
        }
        Some(generation)
    }

    /// A changed tag on this generation requires a revisit. When the whole
    /// generation was taken or replaced, its remover owned deletion coverage
    /// and a new registration owes independent birth or insertion coverage.
    pub(crate) fn scan(&self) -> Option<usize> {
        let state = self.generation.load(Ordering::Acquire);
        if state == TAKEN || state == 0 {
            return Some(0);
        }
        let address = scan_address(&self.address);
        let observed = if state & REMOVING != 0 {
            match self.generation.compare_exchange(
                state,
                (state & !TAGS) | VETOED,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(address),
                Err(seen) => seen,
            }
        } else {
            let seen = self.generation.load(Ordering::Acquire);
            if state == seen {
                return Some(address);
            }
            seen
        };
        (observed == TAKEN || (observed & !TAGS) != (state & !TAGS)).then_some(0)
    }

    pub(crate) fn begin_removal(&self, generation: u64) -> Option<usize> {
        self.generation
            .compare_exchange(
                generation,
                generation | REMOVING,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .ok()?;
        let address = self.address.load(Ordering::Acquire);
        // Compatibility can take, release and reuse the slot while this owner
        // is paused after its CAS. Do not return the address loaded from a reused slot.
        (self.generation.load(Ordering::Acquire) == (generation | REMOVING)).then_some(address)
    }

    pub(crate) fn removal_pending(&self, generation: u64) -> bool {
        let intent = generation | REMOVING;
        self.generation
            .compare_exchange(intent, intent, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn finish_removal(&self, generation: u64) -> bool {
        if self
            .generation
            .compare_exchange(
                generation | REMOVING,
                TAKEN,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_err()
        {
            return false;
        }
        self.address.store(0, Ordering::Release);
        true
    }

    /// Exactly one remover claims this generation. A stale token cannot act on
    /// its replacement, even if it was paused with removal intent installed.
    pub(crate) fn take(&self, generation: u64) -> Option<usize> {
        debug_assert!(generation != 0 && generation & TAGS == 0);
        let mut state = self.generation.load(Ordering::Acquire);
        loop {
            if state == TAKEN || state & !TAGS != generation {
                return None;
            }
            match self.generation.compare_exchange(
                state,
                TAKEN,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(seen) => state = seen,
            }
        }
        let address = self.address.load(Ordering::Acquire);
        self.address.store(0, Ordering::Release);
        Some(address)
    }
}
