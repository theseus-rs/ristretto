//! Unique initialized storage before publication, through cancellation and reclamation.
use crate::allocation_budget::Reservation;
use crate::{Error, Result};
use std::alloc::{Layout, alloc};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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

#[derive(Debug)]
struct LiveCharge {
    counter: Arc<AtomicUsize>,
    bytes: usize,
}
impl Drop for LiveCharge {
    fn drop(&mut self) {
        self.counter.fetch_sub(self.bytes, Ordering::Relaxed);
    }
}

pub(crate) struct Allocation {
    pointer: Pointer,
    destroy: unsafe fn(Pointer),
    reservation: Reservation,
    live: Option<LiveCharge>,
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
                reservation,
                live: None,
            },
            pointer,
        ))
    }

    pub(crate) fn pointer(&self) -> Pointer {
        self.pointer
    }

    pub(crate) fn size(&self) -> usize {
        self.reservation.bytes()
    }

    pub(crate) fn attach_live_counter(&mut self, counter: &Arc<AtomicUsize>) {
        assert!(self.live.is_none(), "allocation already registered");
        let bytes = self.size();
        counter.fetch_add(bytes, Ordering::Relaxed);
        self.live = Some(LiveCharge {
            counter: counter.clone(),
            bytes,
        });
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
        // Field drop refunds both charges even if T::drop unwinds.
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
    use crate::metadata::ObjectMetadata;

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
        let metadata = ObjectMetadata::new_for_gc(allocation);
        // SAFETY: metadata is the unique storage owner and has not begun cleanup.
        assert!(Arc::ptr_eq(unsafe { &pointer.as_ref().0 }, &drops));
        let moved = Some(metadata);
        drop(moved);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        assert_eq!(budget.used(), 0);
        Ok(())
    }

    #[test]
    fn finalizer_precedes_once_only_storage_release() -> Result<()> {
        struct Payload(Arc<AtomicUsize>);
        impl crate::Finalize for Payload {
            fn finalize(&self) {
                assert_eq!(self.0.fetch_add(1, Ordering::SeqCst), 0);
            }
        }
        impl Drop for Payload {
            fn drop(&mut self) {
                assert_eq!(self.0.fetch_add(1, Ordering::SeqCst), 1);
            }
        }
        let budget = Arc::new(AllocationBudget::new(size_of::<Payload>()));
        let events = Arc::new(AtomicUsize::new(0));
        let (allocation, _) = Allocation::new(
            Payload(events.clone()),
            budget.reserve(size_of::<Payload>())?,
        )?;
        // SAFETY: this allocation was constructed from the exact same Payload.
        let metadata = unsafe { ObjectMetadata::new_for_gc_with_finalizer::<Payload>(allocation) };
        metadata.drop_object();
        metadata.drop_object();
        drop(metadata);
        assert_eq!(events.load(Ordering::SeqCst), 2);
        assert_eq!(budget.used(), 0);
        Ok(())
    }
}
