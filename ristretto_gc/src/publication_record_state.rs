//! Unique preparation identities with one irreversible binding per private payload.
use super::{AtomicU64, Ordering};

pub(super) struct State {
    next: AtomicU64,
    bound: AtomicU64,
}

// Deliberately non-cloneable. A concrete record owns this token together with the
// exact Core from which it was reserved; scalar values are not public authority.
pub(super) struct Ticket(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReserveError {
    Contended,
    Exhausted,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            next: AtomicU64::new(1),
            bound: AtomicU64::new(0),
        }
    }

    /// One CAS attempt. Failed preparation can discard a ticket before binding;
    /// no consumed identity is reused, including after cancellation.
    pub(super) fn reserve(&self) -> Result<Ticket, ReserveError> {
        let current = self.next.load(Ordering::Relaxed);
        let next = current.checked_add(1).ok_or(ReserveError::Exhausted)?;
        self.next
            .compare_exchange(current, next, Ordering::Relaxed, Ordering::Relaxed)
            .map(|_| Ticket(current))
            .map_err(|_| ReserveError::Contended)
    }

    /// Called only while a matching graph handle is admitted. A different record
    /// can never replace the winner, even after the winner's operation cancels.
    /// Repeated helpers of the winning record need no private resource extraction.
    pub(super) fn bind(&self, ticket: &Ticket) -> bool {
        match self
            .bound
            .compare_exchange(0, ticket.0, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => true,
            Err(bound) => bound == ticket.0,
        }
    }

    #[cfg(test)]
    pub(super) fn exhaust_for_test(&self) {
        self.next.store(u64::MAX, Ordering::Relaxed);
    }
}

#[cfg(test)]
impl Ticket {
    pub(super) fn value_for_test(&self) -> u64 {
        self.0
    }
}
