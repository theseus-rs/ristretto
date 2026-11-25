//! Unique initialized storage before publication, through cancellation and reclamation.
use crate::allocation_budget::Reservation;
use crate::{Error, Result};
use std::alloc::{Layout, alloc};
use std::ptr::NonNull;

/// A provenance-preserving address; it grants no independent access or lifetime.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Pointer(NonNull<()>);

impl Pointer {
    pub(crate) fn get(self) -> *mut () {
        self.0.as_ptr()
    }
}

// SAFETY: only Allocation::new constructs this wrapper, for Send + Sync payloads.
// Its users retain the matching allocation owner while invoking typed operations.
unsafe impl Send for Pointer {}
// SAFETY: copied addresses provide no mutable access or independent ownership.
unsafe impl Sync for Pointer {}

pub(crate) struct Allocation {
    pointer: Pointer,
    destroy: unsafe fn(Pointer),
    _reservation: Reservation,
}

impl Allocation {
    pub(crate) fn new<T: Send + Sync>(
        value: T,
        reservation: Reservation,
    ) -> Result<(Self, NonNull<T>)> {
        let layout = Layout::new::<T>();
        let pointer = if layout.size() == 0 {
            NonNull::dangling()
        } else {
            // SAFETY: Layout::new is valid and nonzero here. A null result leaves
            // value and the reservation owned by this frame and drops both.
            NonNull::new(unsafe { alloc(layout) }.cast::<T>()).ok_or(Error::OutOfMemory)?
        };
        // SAFETY: fresh correctly aligned storage (or a valid ZST location).
        unsafe { pointer.as_ptr().write(value) };
        Ok((
            Self {
                pointer: Pointer(pointer.cast()),
                destroy: destroy::<T>,
                _reservation: reservation,
            },
            pointer,
        ))
    }

    pub(crate) fn pointer(&self) -> Pointer {
        self.pointer
    }
}

unsafe fn destroy<T>(pointer: Pointer) {
    // SAFETY: this operation was paired with initialized T by Allocation::new,
    // and its unique owning Allocation invokes it once before refunding credit.
    unsafe { drop(Box::from_raw(pointer.get().cast::<T>())) };
}

impl Drop for Allocation {
    fn drop(&mut self) {
        // SAFETY: the unique owner has not transferred the destructor elsewhere.
        // Field drop refunds the charge even if T::drop unwinds.
        unsafe { (self.destroy)(self.pointer) };
    }
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "tests propagate setup errors and assert ownership"
)]
mod tests {
    use super::*;
    use crate::allocation_budget::AllocationBudget;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn descriptor_move_preserves_provenance_and_refunds_once() -> Result<()> {
        struct Payload(Arc<AtomicUsize>);
        impl Drop for Payload {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let budget = Arc::new(AllocationBudget::new(size_of::<Payload>()));
        let drops = Arc::new(AtomicUsize::new(0));
        let (allocation, pointer) = Allocation::new(
            Payload(drops.clone()),
            budget.reserve(size_of::<Payload>())?,
        )?;
        // SAFETY: allocation is the unique storage owner and has not begun cleanup.
        assert!(Arc::ptr_eq(unsafe { &pointer.as_ref().0 }, &drops));
        let moved = Some(allocation);
        drop(moved);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(budget.used(), 0);
        Ok(())
    }
}
