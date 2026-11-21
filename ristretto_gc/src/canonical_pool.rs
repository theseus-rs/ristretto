//! Bounded, generation-checked backing for the optional canonical collector.
//!
//! A load/acquisition or compare attempt has a fixed number of atomic operations.
//! Allocation of the backing, arbitrary value destruction, pool teardown and
//! retry policy are outside that bound. A finite pool alone is no progress proof.
use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering};

mod state;
use state::{BITS, MASK, State};

pub const MAX_GENERATION: u64 = state::MAX_GENERATION;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllocationError {
    InvalidSlot,
    Unavailable,
    Contended,
    /// This selected slot cannot allocate again. Existing owners remain valid.
    GenerationExhausted,
}
#[derive(Debug)]
pub struct AllocationFailure<T> {
    pub value: T,
    pub error: AllocationError,
}

/// Requested Arc backing for the frozen Rust 1.99 layout: two atomic usize
/// counters followed by T. This is audited against allocator requests on each
/// target; it is not a stable standard-library layout guarantee or an RSS bound.
pub fn arc_requested_bytes<T>() -> Option<usize> {
    Some(
        Layout::new::<[AtomicUsize; 2]>()
            .extend(Layout::new::<T>())
            .ok()?
            .0
            .pad_to_align()
            .size(),
    )
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Token(u64);
impl Token {
    fn new(index: usize, generation: u64) -> Self {
        Self((generation << BITS) | index as u64)
    }
    fn index(self) -> usize {
        (self.0 & MASK) as usize
    }
    fn generation(self) -> u64 {
        self.0 >> BITS
    }
}

struct Slot<T> {
    state: State,
    owner: AtomicPtr<Pool<T>>,
    value: UnsafeCell<MaybeUninit<T>>,
}
// SAFETY: BUSY excludes acquisition during initialization/destruction. Only
// counted owners can access a live immutable value; reuse waits for destruction.
unsafe impl<T: Send + Sync> Sync for Slot<T> {}

pub struct Pool<T> {
    slots: Box<[Slot<T>]>,
    cursor: AtomicUsize,
    exhausted: AtomicBool,
}
impl<T> Pool<T> {
    pub fn requested_backing_bytes(capacity: usize) -> Option<usize> {
        if !capacity.is_power_of_two() || capacity as u64 > MASK + 1 {
            return None;
        }
        Layout::array::<Slot<T>>(capacity)
            .ok()?
            .size()
            .checked_add(arc_requested_bytes::<Self>()?)
    }
    /// Diagnostic scan, outside any bounded allocation/acquisition attempt.
    pub fn occupied(&self) -> usize {
        self.slots
            .iter()
            .filter(|slot| slot.state.occupied())
            .count()
    }

    pub fn new(capacity: usize) -> Arc<Self> {
        Self::with_generation_limit(capacity, MAX_GENERATION)
    }

    /// Allow this many successful allocations per slot. Setup encodes the
    /// budget in its starting generation; tokens remain private. The limit must
    /// be nonzero and no larger than `MAX_GENERATION`.
    pub fn with_generation_limit(capacity: usize, generation_limit: u64) -> Arc<Self> {
        assert!(capacity.is_power_of_two() && capacity as u64 <= MASK + 1);
        assert!((1..=MAX_GENERATION).contains(&generation_limit));
        let pool = Arc::new(Self {
            slots: (0..capacity)
                .map(|_| Slot {
                    state: State::with_generation_limit(generation_limit),
                    owner: AtomicPtr::new(std::ptr::null_mut()),
                    value: UnsafeCell::new(MaybeUninit::uninit()),
                })
                .collect(),
            cursor: AtomicUsize::new(0),
            exhausted: AtomicBool::new(false),
        });
        for slot in &pool.slots {
            slot.owner
                .store(Arc::as_ptr(&pool).cast_mut(), Ordering::Relaxed);
        }
        pool
    }

    pub fn try_allocate(self: &Arc<Self>, value: T) -> Result<Owned<T>, T> {
        self.try_allocate_status(value)
            .map_err(|failure| failure.value)
    }

    /// A sticky observation of a permanently unusable selected slot. It does
    /// not assert that every slot is exhausted or authorize reuse of any slot.
    pub fn generation_exhaustion_observed(&self) -> bool {
        self.exhausted.load(Ordering::Acquire)
    }

    pub fn try_allocate_status(
        self: &Arc<Self>,
        value: T,
    ) -> Result<Owned<T>, AllocationFailure<T>> {
        let index = self.cursor.fetch_add(1, Ordering::Relaxed) & (self.slots.len() - 1);
        self.try_allocate_at_status(index, value)
    }

    /// Failure only describes this selected slot, not whole-pool exhaustion.
    pub fn try_allocate_at(self: &Arc<Self>, index: usize, value: T) -> Result<Owned<T>, T> {
        self.try_allocate_at_status(index, value)
            .map_err(|failure| failure.value)
    }

    pub fn try_allocate_at_status(
        self: &Arc<Self>,
        index: usize,
        value: T,
    ) -> Result<Owned<T>, AllocationFailure<T>> {
        let Some(slot) = self.slots.get(index) else {
            return Err(AllocationFailure {
                value,
                error: AllocationError::InvalidSlot,
            });
        };
        let generation = match slot.state.claim_available() {
            Ok(generation) => generation,
            Err(error) => {
                if error == AllocationError::GenerationExhausted {
                    self.exhausted.store(true, Ordering::Release);
                }
                return Err(AllocationFailure { value, error });
            }
        };
        // SAFETY: self retains backing. Each initialized slot takes exactly one
        // aggregate pool owner, released after its final value destruction.
        unsafe { Arc::increment_strong_count(Arc::as_ptr(self)) };
        // SAFETY: the successful claim excludes all readers and other writers.
        unsafe { (*slot.value.get()).write(value) };
        slot.state.publish_initialized(generation);
        Ok(Owned {
            slot: NonNull::from(slot),
            token: Token::new(index, generation),
        })
    }

    fn try_acquire(self: &Arc<Self>, token: Token) -> Option<Owned<T>> {
        let slot = self.slots.get(token.index())?;
        slot.state.acquire(token.generation()).then(|| Owned {
            slot: NonNull::from(slot),
            token,
        })
    }
}

/// A strong slot owner. Cloning may report contention instead of retrying inside
/// a cooperative poll. There is intentionally no infallible Clone implementation.
pub struct Owned<T> {
    slot: NonNull<Slot<T>>,
    token: Token,
}
// SAFETY: owning references prevent destruction/reuse; values are immutable.
unsafe impl<T: Send + Sync> Send for Owned<T> {}
// SAFETY: the shared borrow keeps its owning reference alive.
unsafe impl<T: Send + Sync> Sync for Owned<T> {}
impl<T> Owned<T> {
    /// Observe storage lifetime without retaining the value. The observer pins
    /// the entire pool backing, and a failed acquisition may be transient.
    pub fn observe(&self) -> Observer<T> {
        // SAFETY: this count retains an occupied slot and its aggregate pool
        // owner. Transfer one additional Arc count to the observer's backing.
        let pool = unsafe {
            let pointer = self.slot.as_ref().owner.load(Ordering::Relaxed);
            Arc::increment_strong_count(pointer);
            Arc::from_raw(pointer)
        };
        Observer {
            pool,
            token: self.token,
        }
    }

    pub fn try_clone(&self) -> Option<Self> {
        // SAFETY: this owning reference retains backing and initialized storage.
        let slot = unsafe { self.slot.as_ref() };
        slot.state.acquire(self.token.generation()).then(|| Self {
            slot: self.slot,
            token: self.token,
        })
    }
    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        left.slot == right.slot && left.token == right.token
    }
    fn belongs_to(&self, pool: &Arc<Pool<T>>) -> bool {
        // SAFETY: the owning reference retains the immutable owner identity.
        unsafe { self.slot.as_ref() }.owner.load(Ordering::Relaxed) == Arc::as_ptr(pool).cast_mut()
    }
}

/// Non-owning value observation, not a logical GC weak reference. A None result
/// can mean contention; it is not a certificate of destruction during concurrency.
pub struct Observer<T> {
    pool: Arc<Pool<T>>,
    token: Token,
}
impl<T> Observer<T> {
    pub fn try_load(&self) -> Option<Owned<T>> {
        self.pool.try_acquire(self.token)
    }
}
impl<T> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: a count for this exact generation prevents destruction/reuse.
        unsafe { (*self.slot.as_ref().value.get()).assume_init_ref() }
    }
}

struct Release<T> {
    slot: NonNull<Slot<T>>,
    token: Token,
    pool: *const Pool<T>,
}
impl<T> Drop for Release<T> {
    fn drop(&mut self) {
        // SAFETY: the aggregate owner retains backing through completed drop
        // glue, including unwind. No slot access follows its owner decrement.
        unsafe {
            self.slot
                .as_ref()
                .state
                .publish_free(self.token.generation());
            Arc::decrement_strong_count(self.pool);
        }
    }
}
impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        // SAFETY: this owner retains backing and an initialized value.
        let slot = unsafe { self.slot.as_ref() };
        if !slot.state.release() {
            return;
        }
        let release = Release {
            slot: self.slot,
            token: self.token,
            pool: slot.owner.load(Ordering::Relaxed),
        };
        // SAFETY: final release leaves BUSY, excluding acquisition and reuse.
        unsafe { (*slot.value.get()).assume_init_drop() };
        drop(release);
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod models;
#[cfg(test)]
mod tests;
