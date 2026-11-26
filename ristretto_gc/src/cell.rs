//! Managed object locks. Mapped guards retain the mutation hook until the last projection drops.
#![expect(clippy::missing_errors_doc, missing_docs)]
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync;
use std::sync::atomic::{AtomicBool, Ordering};

#[path = "trace_certificate.rs"]
mod trace_certificate;

std::thread_local! { static GUARD_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

/// Whether the caller currently holds a managed object guard.
#[must_use]
pub fn has_guards() -> bool {
    GUARD_DEPTH.with(|depth| depth.get() != 0)
}
fn enter_guard() {
    GUARD_DEPTH.with(|depth| depth.set(depth.get() + 1));
}
fn leave_guard() {
    GUARD_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
}

struct LeaveGuard;
impl Drop for LeaveGuard {
    fn drop(&mut self) {
        leave_guard();
    }
}

/// Acquire a poisoned guard's inner value, matching `parking_lot`'s
/// non-poisoning semantics.
fn unpoison<G>(result: sync::LockResult<G>) -> G {
    match result {
        Ok(guard) => guard,
        Err(poison) => poison.into_inner(),
    }
}

fn try_unpoison<G>(result: sync::TryLockResult<G>) -> Option<G> {
    match result {
        Ok(guard) => Some(guard),
        Err(sync::TryLockError::Poisoned(poison)) => Some(poison.into_inner()),
        Err(sync::TryLockError::WouldBlock) => None,
    }
}

// ----------------------------------------------------------------------
// RwLock
// ----------------------------------------------------------------------

/// A managed cell whose write guards publish mutation barriers before unlocking.
pub struct RwLock<T: ?Sized> {
    hook: Option<Hook<T>>,
    value: sync::RwLock<T>,
}

enum Mutation<'a> {
    Begin,
    Finish {
        owner: crate::TraceOwner<'a>,
        prepared: bool,
    },
}

struct Hook<T: ?Sized> {
    collector: sync::Weak<crate::GarbageCollector>,
    address: usize,
    mutation: fn(&mut T, Mutation<'_>),
    pins: Option<sync::Arc<crate::diagnostics::Counters>>,
    trace_covered: trace_certificate::Certificate,
}

struct PayloadPin<'a> {
    counters: &'a crate::diagnostics::Counters,
    started: Option<std::time::Instant>,
}

impl<'a> PayloadPin<'a> {
    fn start<T: ?Sized>(hook: Option<&'a Hook<T>>) -> Option<Self> {
        let counters = hook?.pins.as_deref()?;
        counters
            .active_payload_guards
            .fetch_add(1, Ordering::Relaxed);
        Some(Self {
            counters,
            started: crate::diagnostics::now(),
        })
    }
}

impl Drop for PayloadPin<'_> {
    fn drop(&mut self) {
        self.counters.payload_guard_holds.record(self.started);
        self.counters
            .active_payload_guards
            .fetch_sub(1, Ordering::Relaxed);
    }
}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            hook: None,
            value: sync::RwLock::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        unpoison(self.value.into_inner())
    }
}

impl<T: ?Sized> RwLock<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        let guard = try_unpoison(self.value.try_read()).unwrap_or_else(|| {
            let gc = self.hook.as_ref().and_then(|hook| hook.collector.upgrade());
            let _measurement = gc.as_ref().map(|gc| {
                gc.diagnostics
                    .payload_read_wait
                    .measure(gc.diagnostics_enabled())
            });
            unpoison(self.value.read())
        });
        let managed = self.hook.is_some();
        if managed {
            enter_guard();
        }
        RwLockReadGuard(guard, managed, PayloadPin::start(self.hook.as_ref()))
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        let guard = try_unpoison(self.value.try_write()).unwrap_or_else(|| {
            let gc = self.hook.as_ref().and_then(|hook| hook.collector.upgrade());
            let _measurement = gc.as_ref().map(|gc| {
                gc.diagnostics
                    .payload_write_wait
                    .measure(gc.diagnostics_enabled())
            });
            unpoison(self.value.write())
        });
        if self.hook.is_some() {
            enter_guard();
        }
        RwLockWriteGuard(
            guard,
            self.hook.as_ref(),
            PayloadPin::start(self.hook.as_ref()),
            false,
            false,
        )
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        try_unpoison(self.value.try_read()).map(|guard| {
            let managed = self.hook.is_some();
            if managed {
                enter_guard();
            }
            RwLockReadGuard(guard, managed, PayloadPin::start(self.hook.as_ref()))
        })
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        try_unpoison(self.value.try_write()).map(|guard| {
            if self.hook.is_some() {
                enter_guard();
            }
            RwLockWriteGuard(
                guard,
                self.hook.as_ref(),
                PayloadPin::start(self.hook.as_ref()),
                false,
                false,
            )
        })
    }

    pub fn get_mut(&mut self) -> &mut T {
        unpoison(self.value.get_mut())
    }
}

impl<T: Default> Default for RwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.try_read() {
            Some(guard) => f.debug_struct("RwLock").field("data", &&*guard).finish(),
            None => f.debug_struct("RwLock").field("data", &"<locked>").finish(),
        }
    }
}

// Fields drop in declaration order: unlock before recording the hold interval.
pub struct RwLockReadGuard<'a, T: ?Sized>(
    sync::RwLockReadGuard<'a, T>,
    bool,
    Option<PayloadPin<'a>>,
);
impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.2.is_none() || self.1);
        if self.1 {
            leave_guard();
        }
    }
}

impl<'a, T: ?Sized> RwLockReadGuard<'a, T> {
    pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U, T>
    where
        F: FnOnce(&T) -> &U,
    {
        // SAFETY: the projected pointer is valid for the lifetime of the
        // parent guard, which is moved into the returned mapped guard and
        // dropped after the projected pointer.
        let ptr: *const T = &raw const *s.0;
        let projected = unsafe { NonNull::from(f(&*ptr)) };
        MappedRwLockReadGuard {
            ptr: projected,
            parent: s,
            _phantom: PhantomData,
        }
    }

    pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U, T>, Self>
    where
        F: FnOnce(&T) -> Option<&U>,
    {
        // SAFETY: `ptr` borrows through the parent guard which we still
        // own; if `f` returns `Some`, the parent is moved into the
        // mapped guard and continues to keep the data alive.
        let ptr: *const T = &raw const *s.0;
        match unsafe { f(&*ptr) } {
            Some(u) => {
                let projected = NonNull::from(u);
                Ok(MappedRwLockReadGuard {
                    ptr: projected,
                    parent: s,
                    _phantom: PhantomData,
                })
            }
            None => Err(s),
        }
    }
}

impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLockReadGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for RwLockReadGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

pub struct RwLockWriteGuard<'a, T: ?Sized>(
    sync::RwLockWriteGuard<'a, T>,
    Option<&'a Hook<T>>,
    Option<PayloadPin<'a>>,
    bool,
    bool,
);

impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        debug_assert!(self.2.is_none() || self.1.is_some());
        if let Some(hook) = self.1 {
            if self.4 {
                hook.trace_covered.revoke();
            }
            // SAFETY: attachment binds this hook to the actual traced owner;
            // rooted movement detaches it before the cell can escape that owner.
            let owner = unsafe { crate::TraceOwner::new_unchecked(&hook.collector, hook.address) };
            let _depth = LeaveGuard;
            (hook.mutation)(
                &mut self.0,
                Mutation::Finish {
                    owner,
                    prepared: self.3,
                },
            );
        }
    }
}

impl<'a, T: ?Sized> RwLockWriteGuard<'a, T> {
    /// Projects a writer whose entire protected value needs no graph tracing.
    /// Marking can finish while this projection is held. Ordinary unrestricted
    /// projections retain their conservative tracing behavior.
    /// Relocation still requires the payload lock.
    ///
    /// # Safety
    /// On success, the entire protected `T` must require no tracing action until
    /// the returned guard and all nested projections are dropped. Every mutation
    /// possible through `U` must preserve this property. A primitive field inside
    /// a reference-bearing parent is insufficient. The callback itself runs before
    /// certification and has the usual owning mutable-borrow semantics.
    pub unsafe fn try_map_trace_empty<U: ?Sized, F>(
        s: Self,
        f: F,
    ) -> Result<MappedRwLockWriteGuard<'a, U, T>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
    {
        // SAFETY: requiring no trace actions is a strict subset of the rooted
        // projection contract: there are no strong fields to root independently.
        unsafe { Self::try_map_trace_rooted(s, f) }
    }

    /// Projects a writer whose strong fields remain independently rooted.
    /// Marking can finish without reading the locked parent. Ordinary
    /// unrestricted projections retain their conservative tracing behavior.
    /// Storage stays pinned and relocation still requires the payload lock.
    ///
    /// # Safety
    /// On success, every direct strong edge in the entire protected `T` must have
    /// independent owning protection, including normal root publication/deletion
    /// barriers, until the returned guard and every nested projection are dropped.
    /// Every mutation possible through `U` must preserve this property, including
    /// inserting or moving fields. The parent must need no conditional or custom
    /// tracing actions of its own. A separately rooted target still receives its
    /// normal tracing, including any conditional actions inside that allocation.
    /// Calling `Trace::root` alone does not establish these lifetime invariants.
    /// The callback runs with ordinary owning mutable-borrow semantics before
    /// certification; failure or unwind publishes no certificate.
    pub unsafe fn try_map_trace_rooted<U: ?Sized, F>(
        s: Self,
        f: F,
    ) -> Result<MappedRwLockWriteGuard<'a, U, T>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
    {
        let mut s = s;
        s.prepare_mutation();
        let ptr: *mut T = &raw mut *s.0;
        // SAFETY: the mapped pointer stays protected by the owning parent guard.
        match unsafe { f(&mut *ptr) } {
            Some(u) => {
                let projected = NonNull::from(u);
                if let Some(hook) = s.1 {
                    hook.trace_covered.publish();
                    s.4 = true;
                }
                Ok(MappedRwLockWriteGuard {
                    ptr: projected,
                    parent: s,
                    _phantom: PhantomData,
                })
            }
            None => Err(s),
        }
    }

    /// Borrows the payload without acquiring ownership for all of its strong edges.
    /// The guard still runs its usual publication and deletion hooks when dropped.
    ///
    /// # Safety
    /// The caller must not let a value containing an unrooted strong edge escape its
    /// managed owner, including through a return value, captured state, or unwinding.
    /// Any edge exposed to unrestricted mutable access must first acquire independent
    /// ownership with [`crate::Trace::root`]. Replacing and dropping an edge in place is
    /// permitted; normal barriers and tracing invariants must still be maintained.
    /// This does not certify that a collector may skip tracing the locked payload.
    pub unsafe fn unrooted_mut(&mut self) -> &mut T {
        &mut self.0
    }

    fn prepare_mutation(&mut self) {
        if !self.3 {
            if let Some(hook) = self.1 {
                (hook.mutation)(&mut self.0, Mutation::Begin);
            }
            self.3 = true;
        }
    }

    pub fn map<U: ?Sized, F>(mut s: Self, f: F) -> MappedRwLockWriteGuard<'a, U, T>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        s.prepare_mutation();
        // SAFETY: see `RwLockReadGuard::map`.
        let ptr: *mut T = &raw mut *s.0;
        let projected = unsafe { NonNull::from(f(&mut *ptr)) };
        MappedRwLockWriteGuard {
            ptr: projected,
            parent: s,
            _phantom: PhantomData,
        }
    }

    pub fn try_map<U: ?Sized, F>(
        mut s: Self,
        f: F,
    ) -> Result<MappedRwLockWriteGuard<'a, U, T>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
    {
        s.prepare_mutation();
        // SAFETY: see `RwLockReadGuard::try_map`.
        let ptr: *mut T = &raw mut *s.0;
        match unsafe { f(&mut *ptr) } {
            Some(u) => {
                let projected = NonNull::from(u);
                Ok(MappedRwLockWriteGuard {
                    ptr: projected,
                    parent: s,
                    _phantom: PhantomData,
                })
            }
            None => Err(s),
        }
    }
}

impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.prepare_mutation();
        &mut self.0
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLockWriteGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

pub struct MappedRwLockReadGuard<'a, T: ?Sized, P: ?Sized> {
    ptr: NonNull<T>,
    // Field order matters: `ptr` is dropped before `parent` so the lock
    // is released after the projected pointer is no longer accessible.
    parent: RwLockReadGuard<'a, P>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: ?Sized, P: ?Sized> MappedRwLockReadGuard<'a, T, P> {
    pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U, P>
    where
        F: FnOnce(&T) -> &U,
    {
        // SAFETY: `s.ptr` is valid for the lifetime tied to `s.parent`,
        // which is moved into the returned guard.
        let projected = unsafe { NonNull::from(f(s.ptr.as_ref())) };
        MappedRwLockReadGuard {
            ptr: projected,
            parent: s.parent,
            _phantom: PhantomData,
        }
    }

    pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U, P>, Self>
    where
        F: FnOnce(&T) -> Option<&U>,
    {
        // SAFETY: see `map`.
        match unsafe { f(s.ptr.as_ref()) } {
            Some(u) => Ok(MappedRwLockReadGuard {
                ptr: NonNull::from(u),
                parent: s.parent,
                _phantom: PhantomData,
            }),
            None => Err(s),
        }
    }
}

impl<T: ?Sized, P: ?Sized> Deref for MappedRwLockReadGuard<'_, T, P> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: parent guard still alive; pointer is valid.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized + fmt::Debug, P: ?Sized> fmt::Debug for MappedRwLockReadGuard<'_, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

pub struct MappedRwLockWriteGuard<'a, T: ?Sized, P: ?Sized> {
    ptr: NonNull<T>,
    parent: RwLockWriteGuard<'a, P>,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized, P: ?Sized> MappedRwLockWriteGuard<'a, T, P> {
    pub fn map<U: ?Sized, F>(mut s: Self, f: F) -> MappedRwLockWriteGuard<'a, U, P>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        // SAFETY: see `MappedRwLockReadGuard::map`.
        let projected = unsafe { NonNull::from(f(s.ptr.as_mut())) };
        MappedRwLockWriteGuard {
            ptr: projected,
            parent: s.parent,
            _phantom: PhantomData,
        }
    }

    pub fn try_map<U: ?Sized, F>(
        mut s: Self,
        f: F,
    ) -> Result<MappedRwLockWriteGuard<'a, U, P>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
    {
        // SAFETY: see `MappedRwLockReadGuard::try_map`.
        match unsafe { f(s.ptr.as_mut()) } {
            Some(u) => Ok(MappedRwLockWriteGuard {
                ptr: NonNull::from(u),
                parent: s.parent,
                _phantom: PhantomData,
            }),
            None => Err(s),
        }
    }
}

impl<T: ?Sized, P: ?Sized> Deref for MappedRwLockWriteGuard<'_, T, P> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: parent guard still alive; pointer is valid.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, P: ?Sized> DerefMut for MappedRwLockWriteGuard<'_, T, P> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: parent guard still alive; pointer is valid for writes.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized + fmt::Debug, P: ?Sized> fmt::Debug for MappedRwLockWriteGuard<'_, T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: crate::Trace> RwLock<T> {
    fn bind_owner(&mut self, owner: crate::TraceOwner<'_>) {
        if self.hook.as_ref().is_some_and(|hook| {
            hook.address == owner.address()
                && sync::Weak::ptr_eq(&hook.collector, owner.collector())
        }) {
            return;
        }
        let pins = owner
            .collector()
            .upgrade()
            .and_then(|gc| gc.pin_diagnostics());
        self.hook = Some(Hook {
            collector: owner.collector().clone(),
            address: owner.address(),
            mutation: |value, action| {
                let Mutation::Finish { owner, prepared } = action else {
                    value.root();
                    return;
                };
                if value.can_skip_unroot_after_write() {
                    return;
                }
                if T::may_contain_gc_references()
                    && let Some(gc) = owner.collector().upgrade()
                    && !(prepared
                        && value.edges_rooted_before_unroot()
                        && gc.remember_before_unroot(owner.address()))
                {
                    crate::construction::Roots::finish_write(&gc, owner.address(), || {
                        // SAFETY: the guard owns the payload; released roots stay
                        // registered until its owner notification is visible.
                        unsafe { value.unroot(owner) };
                        value.needs_write_barrier()
                    });
                } else {
                    // SAFETY: the same exclusive guard covers certified demotion
                    // and the unconditionally required hooks of leaf payloads.
                    unsafe { value.unroot(owner) };
                }
            },
            pins,
            trace_covered: trace_certificate::Certificate::default(),
        });
    }
}

// SAFETY: tracing and relocation use the payload lock. Hooks bind nested values
// to the actual owner; rooting detaches hooks before unrestricted movement.
unsafe impl<T: crate::Trace> crate::Trace for RwLock<T> {
    fn has_bounded_cleanup() -> bool {
        // Retirement has exclusive access. Dropping the hook only releases GC bookkeeping.
        T::has_bounded_cleanup()
    }

    fn root(&mut self) {
        self.hook = None;
        self.get_mut().root();
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // Install first so a partially demoting hook that unwinds cannot leave
        // an unbound cell containing edges inside the still-managed parent.
        self.bind_owner(owner);
        // SAFETY: exclusive access and the parent's context extend to the value.
        unsafe { self.get_mut().unroot(owner) };
    }
    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace_work(&self) -> usize {
        self.read().trace_work()
    }
    fn prepare(&mut self, heap: &crate::region::RegionHeap) -> crate::Result<()> {
        self.get_mut().prepare(heap)
    }
    fn prepare_step(
        &mut self,
        heap: &crate::region::RegionHeap,
        bytes: usize,
    ) -> crate::Result<crate::TraceProgress> {
        self.get_mut().prepare_step(heap, bytes)
    }
    fn compact_guarded(
        &self,
        heap: &crate::region::RegionHeap,
        bytes: usize,
    ) -> crate::TraceProgress {
        // Bypass mutation barriers: relocation changes neither edges nor their ownership.
        try_unpoison(self.value.try_write()).map_or(crate::TraceProgress::Busy, |mut value| {
            value.compact_step(heap, bytes)
        })
    }

    fn relocation_region(&self, heap: &crate::region::RegionHeap) -> Option<u64> {
        try_unpoison(self.value.try_read()).and_then(|value| value.relocation_region(heap))
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> crate::TraceProgress {
        self.get_mut().clear_step(offset, budget)
    }
    fn heap_owner(&self) -> Option<sync::Arc<dyn crate::HeapOwner>> {
        self.read().heap_owner()
    }
    fn trace(&self, gc: &crate::GarbageCollector) {
        if T::may_contain_gc_references() {
            if self
                .hook
                .as_ref()
                .is_some_and(|hook| hook.trace_covered.certify())
            {
                return;
            }
            self.read().trace(gc);
        }
    }

    fn trace_step(
        &self,
        gc: &crate::GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> crate::TraceProgress {
        if !T::may_contain_gc_references() {
            return crate::TraceProgress::Complete;
        }
        if let Some(value) = self.try_read() {
            return value.trace_step(gc, offset, budget);
        }
        if self
            .hook
            .as_ref()
            .is_some_and(|hook| hook.trace_covered.certify())
        {
            return crate::TraceProgress::Complete;
        }
        crate::TraceProgress::Busy
    }

    fn external_size(&self) -> usize {
        self.read().external_size()
    }

    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        self.bind_owner(owner);
        // SAFETY: construction protection and the owner context cover the value.
        unsafe { self.get_mut().attach(owner) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GarbageCollector, Gc};

    #[test]
    fn only_attached_cells_suppress_collection_assistance() {
        let metadata = RwLock::new(1_u64);
        let host = metadata.read();
        assert!(!has_guards());
        drop(host);
        let host = metadata.write();
        assert!(!has_guards());
        drop(host);

        let gc = GarbageCollector::new();
        let managed = Gc::new(&gc, RwLock::new(2_u64));
        let guard = RwLockReadGuard::map(managed.read(), |value| value);
        assert!(has_guards());
        drop(metadata.write());
        assert!(has_guards(), "host guard must not decrement managed depth");
        drop(guard);
        assert!(!has_guards());
        let guard = RwLockWriteGuard::map(managed.write(), |value| value);
        assert!(has_guards());
        drop(guard);
        assert!(!has_guards());
    }

    #[test]
    fn narrow_mutation_preserves_barriers_and_later_whole_borrow_ownership() {
        let gc = GarbageCollector::new();
        let first = Gc::new(&gc, 42_u64).into_rooted();
        let first_weak = crate::GcWeak::new(&first, crate::ReferenceKind::Weak, 0);
        let second = Gc::new(&gc, 17_u64).into_rooted();
        let second_weak = crate::GcWeak::new(&second, crate::ReferenceKind::Weak, 0);
        let owner = Gc::new(&gc, RwLock::new(vec![Some(first), Some(second)])).into_rooted();
        let mut guard = owner.write();
        // SAFETY: drop the previous edge in place; no unrooted edge escapes.
        (unsafe { guard.unrooted_mut() })[1] = None;
        let moved = std::mem::take(&mut *guard);
        drop(guard);
        drop(owner);
        for _ in 0..3 {
            gc.collect_full().expect("collection");
        }
        assert!(second_weak.upgrade().is_none());
        assert!(first_weak.upgrade().is_some());
        assert_eq!(**moved[0].as_ref().expect("child"), 42);
        drop(moved);
        gc.stop().expect("stop");
        assert!(first_weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }

    #[test]
    fn whole_value_moves_through_mutable_and_failed_mapped_borrows_keep_roots() {
        for mapped in [false, true] {
            let gc = GarbageCollector::new();
            let child = Gc::new(&gc, 42_u64).into_rooted();
            let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
            let owner = Gc::new(&gc, RwLock::new(vec![Some(child)])).into_rooted();
            let moved = if mapped {
                let mut moved = None;
                let result = RwLockWriteGuard::try_map(owner.write(), |value| {
                    moved = Some(std::mem::take(value));
                    None::<&mut Vec<Option<Gc<u64>>>>
                });
                assert!(result.is_err());
                drop(result);
                moved.expect("callback extracted the value")
            } else {
                std::mem::take(&mut *owner.write())
            };
            drop(owner);
            for _ in 0..3 {
                gc.collect_full().expect("collection");
                assert!(weak.upgrade().is_some());
            }
            assert_eq!(**moved[0].as_ref().expect("moved child"), 42);
            drop(moved);
            for _ in 0..3 {
                gc.collect_full().expect("collection");
            }
            assert!(weak.upgrade().is_none());
            gc.stop().expect("stop");
            assert_eq!(gc.managed_bytes(), 0);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn whole_value_moved_during_mapping_survives_unwind() {
        let gc = GarbageCollector::new();
        let child = Gc::new(&gc, 42_u64).into_rooted();
        let weak = crate::GcWeak::new(&child, crate::ReferenceKind::Weak, 0);
        let owner = Gc::new(&gc, RwLock::new(vec![Some(child)])).into_rooted();
        let mut moved = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard: MappedRwLockWriteGuard<'_, Vec<Option<Gc<u64>>>, _> =
                RwLockWriteGuard::map(owner.write(), |value| {
                    moved = Some(std::mem::take(value));
                    panic!("cancel after extracting the whole value");
                });
        }));
        assert!(result.is_err());
        drop(owner);
        for _ in 0..3 {
            gc.collect_full().expect("collection");
            assert!(weak.upgrade().is_some());
        }
        drop(moved);
        gc.stop().expect("stop");
        assert!(weak.upgrade().is_none());
        assert_eq!(gc.managed_bytes(), 0);
    }
}
