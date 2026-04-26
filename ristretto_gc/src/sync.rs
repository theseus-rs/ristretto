//! Cross-target synchronization primitives.
//!
//! On non-wasm targets these re-export `parking_lot::{Mutex, RwLock}` and friends. On
//! `target_family = "wasm"` (which in practice is single-threaded for `wasm32-unknown-unknown` and
//! `wasm32-wasip2`) they are backed by `std::cell::RefCell` because `parking_lot` panics on park
//! and `std::sync::{Mutex, RwLock}` cannot be constructed in a `no_std`-style environment without
//! a working thread parker either.
//!
//! The wasm wrapper purposely declares `unsafe impl Sync` so that the types can be used as
//! drop-in replacements in the rest of the workspace; this is sound because wasm modules built
//! for these targets execute on a single thread.

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
    use std::cell::{Ref, RefCell, RefMut};
    use std::fmt;
    use std::ops::{Deref, DerefMut};

    pub struct RwLock<T: ?Sized>(RefCell<T>);

    // SAFETY: wasm targets the workspace builds for are single-threaded; nothing can actually
    // share these locks across threads.
    unsafe impl<T: ?Sized + Send> Sync for RwLock<T> {}
    unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}

    impl<T> RwLock<T> {
        pub const fn new(value: T) -> Self {
            Self(RefCell::new(value))
        }

        pub fn into_inner(self) -> T {
            self.0.into_inner()
        }
    }

    impl<T: ?Sized> RwLock<T> {
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            RwLockReadGuard(self.0.borrow())
        }

        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            RwLockWriteGuard(self.0.borrow_mut())
        }

        pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
            self.0.try_borrow().ok().map(RwLockReadGuard)
        }

        pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
            self.0.try_borrow_mut().ok().map(RwLockWriteGuard)
        }

        pub fn get_mut(&mut self) -> &mut T {
            self.0.get_mut()
        }

        pub fn data_ptr(&self) -> *mut T {
            self.0.as_ptr()
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

    pub struct RwLockReadGuard<'a, T: ?Sized>(Ref<'a, T>);

    impl<'a, T: ?Sized> RwLockReadGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U>
        where
            F: FnOnce(&T) -> &U,
        {
            MappedRwLockReadGuard(Ref::map(s.0, f))
        }

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U>, Self>
        where
            F: FnOnce(&T) -> Option<&U>,
        {
            match Ref::filter_map(s.0, f) {
                Ok(r) => Ok(MappedRwLockReadGuard(r)),
                Err(orig) => Err(RwLockReadGuard(orig)),
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

    pub struct RwLockWriteGuard<'a, T: ?Sized>(RefMut<'a, T>);

    impl<'a, T: ?Sized> RwLockWriteGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, U>
        where
            F: FnOnce(&mut T) -> &mut U,
        {
            MappedRwLockWriteGuard(RefMut::map(s.0, f))
        }

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, U>, Self>
        where
            F: FnOnce(&mut T) -> Option<&mut U>,
        {
            match RefMut::filter_map(s.0, f) {
                Ok(r) => Ok(MappedRwLockWriteGuard(r)),
                Err(orig) => Err(RwLockWriteGuard(orig)),
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

    pub struct MappedRwLockReadGuard<'a, T: ?Sized>(Ref<'a, T>);

    impl<'a, T: ?Sized> MappedRwLockReadGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockReadGuard<'a, U>
        where
            F: FnOnce(&T) -> &U,
        {
            MappedRwLockReadGuard(Ref::map(s.0, f))
        }

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockReadGuard<'a, U>, Self>
        where
            F: FnOnce(&T) -> Option<&U>,
        {
            match Ref::filter_map(s.0, f) {
                Ok(r) => Ok(MappedRwLockReadGuard(r)),
                Err(orig) => Err(MappedRwLockReadGuard(orig)),
            }
        }
    }

    impl<T: ?Sized> Deref for MappedRwLockReadGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedRwLockReadGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    pub struct MappedRwLockWriteGuard<'a, T: ?Sized>(RefMut<'a, T>);

    impl<'a, T: ?Sized> MappedRwLockWriteGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedRwLockWriteGuard<'a, U>
        where
            F: FnOnce(&mut T) -> &mut U,
        {
            MappedRwLockWriteGuard(RefMut::map(s.0, f))
        }

        pub fn try_map<U: ?Sized, F>(s: Self, f: F) -> Result<MappedRwLockWriteGuard<'a, U>, Self>
        where
            F: FnOnce(&mut T) -> Option<&mut U>,
        {
            match RefMut::filter_map(s.0, f) {
                Ok(r) => Ok(MappedRwLockWriteGuard(r)),
                Err(orig) => Err(MappedRwLockWriteGuard(orig)),
            }
        }
    }

    impl<T: ?Sized> Deref for MappedRwLockWriteGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized> DerefMut for MappedRwLockWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedRwLockWriteGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    pub struct RwLockUpgradableReadGuard<'a, T: ?Sized>(RefMut<'a, T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for RwLockUpgradableReadGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

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

    pub struct Mutex<T: ?Sized>(RefCell<T>);

    // SAFETY: see `RwLock` above.
    unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
    unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

    impl<T> Mutex<T> {
        pub const fn new(value: T) -> Self {
            Self(RefCell::new(value))
        }

        pub fn into_inner(self) -> T {
            self.0.into_inner()
        }
    }

    impl<T: ?Sized> Mutex<T> {
        pub fn lock(&self) -> MutexGuard<'_, T> {
            MutexGuard(self.0.borrow_mut())
        }

        pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
            self.0.try_borrow_mut().ok().map(MutexGuard)
        }

        pub fn get_mut(&mut self) -> &mut T {
            self.0.get_mut()
        }

        pub fn data_ptr(&self) -> *mut T {
            self.0.as_ptr()
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

    pub struct MutexGuard<'a, T: ?Sized>(RefMut<'a, T>);

    impl<'a, T: ?Sized> MutexGuard<'a, T> {
        pub fn map<U: ?Sized, F>(s: Self, f: F) -> MappedMutexGuard<'a, U>
        where
            F: FnOnce(&mut T) -> &mut U,
        {
            MappedMutexGuard(RefMut::map(s.0, f))
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

    pub struct MappedMutexGuard<'a, T: ?Sized>(RefMut<'a, T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MappedMutexGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (**self).fmt(f)
        }
    }

    impl<T: ?Sized> Deref for MappedMutexGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ?Sized> DerefMut for MappedMutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }
}

pub use imp::*;
