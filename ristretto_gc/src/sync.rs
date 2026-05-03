//! Cross-target synchronization primitives.
//!
//! On non-wasm targets these re-export `parking_lot::{Mutex, RwLock}` and
//! their associated guard types.
//!
//! On `target_family = "wasm"` they are thin wrappers around
//! `std::sync::{Mutex, RwLock}` that:
//!
//! * present the same surface as the `parking_lot` types used elsewhere in the
//!   workspace,
//! * convert poisoned guards into the inner data (matching `parking_lot`'s
//!   non-poisoning behavior), and
//! * provide guard projection (`map`/`try_map` and the `Mapped*` guards) by
//!   pairing the underlying std guard with a projected raw pointer.

#[cfg(not(target_family = "wasm"))]
mod imp {
    pub use parking_lot::{
        MappedMutexGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard, Mutex, MutexGuard, RwLock,
        RwLockReadGuard, RwLockUpgradableReadGuard, RwLockWriteGuard,
    };
}

#[cfg(target_family = "wasm")]
#[expect(clippy::missing_errors_doc, clippy::must_use_candidate, missing_docs)]
mod imp {
    use std::fmt;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::ptr::NonNull;
    use std::sync::{self as ssync};

    /// Empty marker trait used to type-erase a parent guard while keeping it
    /// alive for the duration of a mapped guard.
    trait Erased {}
    impl<T: ?Sized> Erased for T {}

    /// Acquire a poisoned guard's inner value, matching `parking_lot`'s
    /// non-poisoning semantics.
    fn unpoison<G>(result: ssync::LockResult<G>) -> G {
        match result {
            Ok(guard) => guard,
            Err(poison) => poison.into_inner(),
        }
    }

    fn try_unpoison<G>(result: ssync::TryLockResult<G>) -> Option<G> {
        match result {
            Ok(guard) => Some(guard),
            Err(ssync::TryLockError::Poisoned(poison)) => Some(poison.into_inner()),
            Err(ssync::TryLockError::WouldBlock) => None,
        }
    }

    // ----------------------------------------------------------------------
    // RwLock
    // ----------------------------------------------------------------------

    pub struct RwLock<T: ?Sized>(ssync::RwLock<T>);

    impl<T> RwLock<T> {
        pub const fn new(value: T) -> Self {
            Self(ssync::RwLock::new(value))
        }

        pub fn into_inner(self) -> T {
            unpoison(self.0.into_inner())
        }
    }

    impl<T: ?Sized> RwLock<T> {
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            RwLockReadGuard(unpoison(self.0.read()))
        }

        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            RwLockWriteGuard(unpoison(self.0.write()))
        }

        pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
            try_unpoison(self.0.try_read()).map(RwLockReadGuard)
        }

        pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
            try_unpoison(self.0.try_write()).map(RwLockWriteGuard)
        }

        pub fn get_mut(&mut self) -> &mut T {
            unpoison(self.0.get_mut())
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

    pub struct RwLockReadGuard<'a, T: ?Sized>(ssync::RwLockReadGuard<'a, T>);

    impl<'a, T: ?Sized> RwLockReadGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U>
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
                parent: Box::new(s.0),
                _phantom: PhantomData,
            }
        }

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U>, Self>
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
                        parent: Box::new(s.0),
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

    pub struct RwLockWriteGuard<'a, T: ?Sized>(ssync::RwLockWriteGuard<'a, T>);

    impl<'a, T: ?Sized> RwLockWriteGuard<'a, T> {
        pub fn map<U: ?Sized, F>(mut s: Self, f: F) -> MappedRwLockWriteGuard<'a, U>
        where
            F: FnOnce(&mut T) -> &mut U,
        {
            // SAFETY: see `RwLockReadGuard::map`.
            let ptr: *mut T = &raw mut *s.0;
            let projected = unsafe { NonNull::from(f(&mut *ptr)) };
            MappedRwLockWriteGuard {
                ptr: projected,
                parent: Box::new(s.0),
                _phantom: PhantomData,
            }
        }

        pub fn try_map<U: ?Sized, F>(
            mut s: Self,
            f: F,
        ) -> Result<MappedRwLockWriteGuard<'a, U>, Self>
        where
            F: FnOnce(&mut T) -> Option<&mut U>,
        {
            // SAFETY: see `RwLockReadGuard::try_map`.
            let ptr: *mut T = &raw mut *s.0;
            match unsafe { f(&mut *ptr) } {
                Some(u) => {
                    let projected = NonNull::from(u);
                    Ok(MappedRwLockWriteGuard {
                        ptr: projected,
                        parent: Box::new(s.0),
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
            &mut self.0
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLockWriteGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    pub struct MappedRwLockReadGuard<'a, T: ?Sized> {
        ptr: NonNull<T>,
        // Field order matters: `ptr` is dropped before `parent` so the lock
        // is released after the projected pointer is no longer accessible.
        parent: Box<dyn Erased + 'a>,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T: ?Sized> MappedRwLockReadGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U>
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

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U>, Self>
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

    impl<T: ?Sized> Deref for MappedRwLockReadGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            // SAFETY: parent guard still alive; pointer is valid.
            unsafe { self.ptr.as_ref() }
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedRwLockReadGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    pub struct MappedRwLockWriteGuard<'a, T: ?Sized> {
        ptr: NonNull<T>,
        parent: Box<dyn Erased + 'a>,
        _phantom: PhantomData<&'a mut T>,
    }

    impl<'a, T: ?Sized> MappedRwLockWriteGuard<'a, T> {
        pub fn map<U: ?Sized, F>(mut s: Self, f: F) -> MappedRwLockWriteGuard<'a, U>
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
        ) -> Result<MappedRwLockWriteGuard<'a, U>, Self>
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

    impl<T: ?Sized> Deref for MappedRwLockWriteGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            // SAFETY: parent guard still alive; pointer is valid.
            unsafe { self.ptr.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for MappedRwLockWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            // SAFETY: parent guard still alive; pointer is valid for writes.
            unsafe { self.ptr.as_mut() }
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedRwLockWriteGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    /// Upgradable read guards aren't natively provided by `std::sync`. Since
    /// no caller in the workspace currently uses them on wasm, this is a
    /// thin wrapper around a write guard that preserves the `upgrade` API.
    pub struct RwLockUpgradableReadGuard<'a, T: ?Sized>(ssync::RwLockWriteGuard<'a, T>);

    impl<'a, T: ?Sized> RwLockUpgradableReadGuard<'a, T> {
        pub fn upgrade(s: Self) -> RwLockWriteGuard<'a, T> {
            RwLockWriteGuard(s.0)
        }
    }

    impl<T: ?Sized> Deref for RwLockUpgradableReadGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLockUpgradableReadGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    // ----------------------------------------------------------------------
    // Mutex
    // ----------------------------------------------------------------------

    pub struct Mutex<T: ?Sized>(ssync::Mutex<T>);

    impl<T> Mutex<T> {
        pub const fn new(value: T) -> Self {
            Self(ssync::Mutex::new(value))
        }

        pub fn into_inner(self) -> T {
            unpoison(self.0.into_inner())
        }
    }

    impl<T: ?Sized> Mutex<T> {
        pub fn lock(&self) -> MutexGuard<'_, T> {
            MutexGuard(unpoison(self.0.lock()))
        }

        pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
            try_unpoison(self.0.try_lock()).map(MutexGuard)
        }

        pub fn get_mut(&mut self) -> &mut T {
            unpoison(self.0.get_mut())
        }
    }

    impl<T: Default> Default for Mutex<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: fmt::Debug + ?Sized> fmt::Debug for Mutex<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.try_lock() {
                Some(guard) => f.debug_struct("Mutex").field("data", &&*guard).finish(),
                None => f.debug_struct("Mutex").field("data", &"<locked>").finish(),
            }
        }
    }

    pub struct MutexGuard<'a, T: ?Sized>(ssync::MutexGuard<'a, T>);

    impl<'a, T: ?Sized> MutexGuard<'a, T> {
        pub fn map<U: ?Sized, F>(mut s: Self, f: F) -> MappedMutexGuard<'a, U>
        where
            F: FnOnce(&mut T) -> &mut U,
        {
            // SAFETY: see `RwLockWriteGuard::map`.
            let ptr: *mut T = &raw mut *s.0;
            let projected = unsafe { NonNull::from(f(&mut *ptr)) };
            MappedMutexGuard {
                ptr: projected,
                parent: Box::new(s.0),
                _phantom: PhantomData,
            }
        }
    }

    impl<T: ?Sized> Deref for MutexGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    pub struct MappedMutexGuard<'a, T: ?Sized> {
        ptr: NonNull<T>,
        parent: Box<dyn Erased + 'a>,
        _phantom: PhantomData<&'a mut T>,
    }

    impl<T: ?Sized> Deref for MappedMutexGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            // SAFETY: parent guard still alive; pointer is valid.
            unsafe { self.ptr.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for MappedMutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            // SAFETY: parent guard still alive; pointer is valid for writes.
            unsafe { self.ptr.as_mut() }
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedMutexGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }
}

pub use imp::*;
