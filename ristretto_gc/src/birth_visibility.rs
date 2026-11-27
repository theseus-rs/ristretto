//! A registry scan may pass a physically indexed descriptor before its publisher
//! decides which epochs own its traversal. Record that skip in the same word
//! as activation so an idle publisher cannot bypass a newly started snapshot.
use super::{AtomicU8, Ordering};

const PRIVATE: u8 = 0;
const PENDING: u8 = 1;
const PASSED: u8 = 2;
const LIVE: u8 = 3;
const RETIRED: u8 = 8;
// This bit stays set across all later attempts. A queue entry for the first
// attempt can therefore use the descriptor itself as its exact ticket.
const RETRIED: u8 = 4;

#[derive(Debug, Default)]
pub(crate) struct BirthVisibility(AtomicU8);

impl BirthVisibility {
    /// The exact address reservation serializes installation, and the caller owns
    /// the private payload. A collector cannot retire it before insertion.
    pub(crate) fn index(&self) {
        assert_eq!(
            self.0.load(Ordering::Relaxed),
            PRIVATE,
            "duplicate publication accounting"
        );
        self.0.store(PENDING, Ordering::Release);
    }

    pub(crate) fn indexed(&self) -> bool {
        self.0.load(Ordering::Acquire) != PRIVATE
    }

    pub(crate) fn visible(&self) -> bool {
        let mut state = self.0.load(Ordering::Acquire);
        while state & !RETRIED == PENDING {
            match self.0.compare_exchange(
                state,
                (state & RETRIED) | PASSED,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return false,
                Err(next) => state = next,
            }
        }
        state & !RETRIED == LIVE
    }

    pub(crate) fn first_attempt(&self) -> bool {
        self.0.load(Ordering::Relaxed) & RETRIED == 0
    }

    pub(crate) fn claim_first(&self) -> bool {
        match self.0.load(Ordering::Acquire) {
            LIVE => true,
            PENDING => {
                self.0
                    .compare_exchange(PENDING, PASSED, Ordering::AcqRel, Ordering::Acquire)
                    == Err(LIVE)
            }
            _ => false,
        }
    }

    /// The sole publisher begins a new phase observation after an invalid
    /// attempt. Its next clock RMW orders this reset before every later cycle.
    /// Scans in an already active cycle are covered by that cycle's birth mark.
    pub(crate) fn retry(&self) {
        let previous = self.0.swap(RETRIED | PENDING, Ordering::AcqRel);
        debug_assert!(matches!(previous & !RETRIED, PENDING | PASSED));
    }

    pub(crate) fn publish_if_unobserved(&self) -> bool {
        match self
            .0
            .compare_exchange(PENDING, LIVE, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => true,
            Err(state) if state == RETRIED | PENDING => self
                .0
                .compare_exchange(
                    RETRIED | PENDING,
                    RETRIED | LIVE,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok(),
            Err(_) => false,
        }
    }

    /// Called under the address shard before detaching a live descriptor.
    pub(crate) fn retire(&self) {
        self.0.store(RETIRED, Ordering::Release);
    }

    /// Only an indexed descriptor that never became public can be rolled
    /// back as a private payload, even if retirement beat publisher unwind.
    pub(crate) fn unpublished(&self) -> bool {
        matches!(self.0.load(Ordering::Acquire) & !RETRIED, PENDING | PASSED)
    }

    pub(crate) fn is_live(&self) -> bool {
        self.0.load(Ordering::Acquire) & !RETRIED == LIVE
    }
}
