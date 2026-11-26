//! Fixed backing for owning immutable values. Isolated experiment, not GC storage.
//! Pool creation still allocates. ArcSwap bookkeeping is outside this pool.
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering, fence};

mod slot_state;
use slot_state::SlotState;

#[repr(C, align(8))]
pub struct Slot<T> {
    state: SlotState,
    owner: AtomicPtr<Pool<T>>,
    value: UnsafeCell<MaybeUninit<T>>,
}
// SAFETY: a successful occupied CAS gives exclusive initialization ownership.
// Published values are immutable until the last owning reference drops. Reuse
// follows destruction and the occupied Release store; no weak references exist.
unsafe impl<T: Send + Sync> Sync for Slot<T> {}

pub struct Pool<T> {
    slots: Box<[Slot<T>]>,
    cursor: AtomicUsize,
}
impl<T> Pool<T> {
    pub fn new(capacity: usize) -> Arc<Self> {
        assert!(capacity.is_power_of_two());
        let pool = Arc::new(Self {
            slots: (0..capacity)
                .map(|_| Slot {
                    state: SlotState::new(),
                    owner: AtomicPtr::new(std::ptr::null_mut()),
                    value: UnsafeCell::new(MaybeUninit::uninit()),
                })
                .collect(),
            cursor: AtomicUsize::new(0),
        });
        let owner = Arc::as_ptr(&pool).cast_mut();
        for slot in &pool.slots {
            slot.owner.store(owner, Ordering::Relaxed);
        }
        pool
    }
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// One selected slot, not an exhaustion verdict for the entire pool.
    pub fn try_allocate(self: &Arc<Self>, value: T) -> Result<Owned<T>, T> {
        let index = self.cursor.fetch_add(1, Ordering::Relaxed) & (self.capacity() - 1);
        self.try_allocate_at(index, value)
    }
    /// No allocator, retry loop, callback or shared queue runs after reservation.
    pub fn try_allocate_at(self: &Arc<Self>, index: usize, value: T) -> Result<Owned<T>, T> {
        let Some(slot) = self.slots.get(index) else {
            return Err(value);
        };
        if !slot.state.claim() {
            return Err(value);
        }
        // One aggregate strong pool owner per occupied slot, irrespective of the
        // slot's reference count. Its last reference releases this pool owner.
        // SAFETY: self holds a live strong count through initialization. The raw
        // owner is the exact Arc allocation initialized by new; it never changes.
        unsafe {
            Arc::increment_strong_count(Arc::as_ptr(self));
        }
        // SAFETY: occupied excludes reuse until destruction has completed, and
        // this fresh owner has not yet escaped. All previous values are dropped.
        unsafe {
            (*slot.value.get()).write(value);
        }
        Ok(Owned {
            slot: NonNull::from(slot),
        })
    }
    /// Diagnostic scan; not part of a bounded allocation/help attempt.
    pub fn occupied(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| slot.state.is_occupied())
            .count()
    }
}
impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        // Occupied slots each retain an aggregate Arc owner. No live value can
        // remain when the pool's last strong owner disappears.
        debug_assert!(self.slots.iter().all(|slot| !slot.state.is_occupied()));
    }
}

/// One strong reference to an immutable pool slot. There is no Weak operation.
pub struct Owned<T> {
    slot: NonNull<Slot<T>>,
}
// SAFETY: owned values are immutable, and refcount/slot ownership are atomic.
// Destruction can run on any final owner's thread, requiring T: Send as well.
unsafe impl<T: Send + Sync> Send for Owned<T> {}
// SAFETY: a borrowed owner keeps the initialized immutable T and backing alive.
unsafe impl<T: Send + Sync> Sync for Owned<T> {}
impl<T> Owned<T> {
    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        left.slot == right.slot
    }
    pub fn as_ptr(this: &Self) -> *const Slot<T> {
        this.slot.as_ptr()
    }
    pub fn into_raw(this: Self) -> *mut Slot<T> {
        let pointer = this.slot.as_ptr();
        std::mem::forget(this);
        pointer
    }
    /// # Safety
    /// The pointer must carry one unconsumed owning count from into_raw.
    pub unsafe fn from_raw(pointer: *const Slot<T>) -> Self {
        // SAFETY: the caller transfers the original nonnull owning pointer.
        Self {
            slot: unsafe { NonNull::new_unchecked(pointer.cast_mut()) },
        }
    }
}
impl<T> Clone for Owned<T> {
    fn clone(&self) -> Self {
        // SAFETY: this owner keeps the slot initialized and unreusable.
        let slot = unsafe { self.slot.as_ref() };
        if slot.state.share() >= isize::MAX as usize {
            std::process::abort();
        }
        Self { slot: self.slot }
    }
}
impl<T> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: this owning reference prevents destruction and slot reuse.
        unsafe { (*self.slot.as_ref().value.get()).assume_init_ref() }
    }
}

struct ReleaseSlot<T> {
    slot: NonNull<Slot<T>>,
    owner: *const Pool<T>,
}
impl<T> Drop for ReleaseSlot<T> {
    fn drop(&mut self) {
        // SAFETY: the aggregate pool owner keeps this slot's backing alive. T's
        // drop glue has finished (also when unwinding), so reuse can start now.
        unsafe {
            self.slot.as_ref().state.publish_free();
        }
        // Do not access the slot after this decrement; the pool may now be freed.
        // SAFETY: consume the one owner reserved at successful initialization.
        unsafe {
            Arc::decrement_strong_count(self.owner);
        }
    }
}
impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        // SAFETY: this owner retains initialized storage through its decrement.
        let slot = unsafe { self.slot.as_ref() };
        if !slot.state.release() {
            return;
        }
        let release = ReleaseSlot {
            slot: self.slot,
            owner: slot.owner.load(Ordering::Relaxed),
        };
        // SAFETY: the reference count is zero, but occupied remains true through
        // destruction. No new owner can reserve or read the value until release.
        unsafe {
            (*slot.value.get()).assume_init_drop();
        }
        drop(release);
    }
}

// SAFETY: every raw pointer is an actual aligned Slot allocation with its own
// strong count. Equal pointers have the same refcount while any owner/ArcSwap
// protection exists. Reuse requires all old owners to be dropped, preventing ABA
// against an owning expected value. Raw ownership conversion preserves provenance
// and never changes the count; clone increments it. No empty/tagged pointer exists.
unsafe impl<T> arc_swap::RefCnt for Owned<T> {
    type Base = Slot<T>;
    fn into_ptr(me: Self) -> *mut Slot<T> {
        Self::into_raw(me)
    }
    fn as_ptr(me: &Self) -> *mut Slot<T> {
        me.slot.as_ptr()
    }
    unsafe fn from_ptr(pointer: *const Slot<T>) -> Self {
        // SAFETY: ArcSwap returns an owning raw count produced above.
        unsafe { Self::from_raw(pointer) }
    }
}

#[cfg(test)]
mod tests;
