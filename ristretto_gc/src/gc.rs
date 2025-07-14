use crate::collector::{GarbageCollector, Trace};
use crate::error::Result;
use crate::gc_box::GcBox;
use crate::pointers::SafePtr;
use crate::root_guard::GcRootGuard;
use crate::{Finalize, GC};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// A garbage-collected pointer type for `T`.
///
/// `Gc<T>` provides shared ownership of a value of type `T`, allocated in the heap. Unlike
/// `Arc<T>`, `Gc<T>` can automatically detect and collect circular references using a concurrent,
/// pauseless garbage collector with pure reachability analysis.
pub struct Gc<T> {
    pub(crate) ptr: NonNull<GcBox<T>>,
    pub(crate) phantom: PhantomData<GcBox<T>>,
}

impl<T> Gc<T> {
    /// Constructs a new `Gc<T>`.
    pub fn new(data: T) -> Self
    where
        T: Send + Sync,
    {
        Self::with_collector(&GC, data)
    }

    /// Constructs a new `Gc<T>` with finalization support.
    pub fn new_with_finalizer(data: T) -> Self
    where
        T: Send + Sync + Finalize,
    {
        Self::with_collector_and_finalizer(&GC, data)
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector.
    ///
    /// # Panics
    ///
    /// if `Box::into_raw` returns a null pointer, which should never happen
    pub fn with_collector(collector: &GarbageCollector, data: T) -> Self
    where
        T: Send + Sync,
    {
        let boxed = Box::new(GcBox::new(data));
        let ptr = Box::into_raw(boxed);

        // Record allocation with the collector
        let size = size_of::<GcBox<T>>();
        collector.record_allocation(size);

        // Register the new object with the collector for reachability analysis. We need to register
        // this as a GcBox<T>, not just T
        collector.register_object::<T>(ptr, size);

        Self {
            ptr: NonNull::new(ptr).expect("Box::into_raw returned null pointer"),
            phantom: PhantomData,
        }
    }

    /// Constructs a new `Gc<T>` with a specific garbage collector and finalization support.
    ///
    /// # Panics
    ///
    /// if `Box::into_raw` returns a null pointer, which should never happen
    pub fn with_collector_and_finalizer(collector: &GarbageCollector, data: T) -> Self
    where
        T: Send + Sync + Finalize,
    {
        let boxed = Box::new(GcBox::new(data));
        let ptr = Box::into_raw(boxed);

        // Record allocation with the collector
        let size = size_of::<GcBox<T>>();
        collector.record_allocation(size);

        // Register the new object with finalizer support
        collector.register_object_with_finalizer::<T>(ptr, size);

        Self {
            ptr: NonNull::new(ptr).expect("Box::into_raw returned null pointer"),
            phantom: PhantomData,
        }
    }

    /// Returns `true` if the two `Gc`s point to the same allocation.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

    /// Returns a raw pointer to the data.
    ///
    /// The caller must ensure that the `Gc` outlives the pointer this function returns, or else it
    /// will end up pointing to garbage.
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        ptr::addr_of!(**self)
    }

    /// Makes a mutable reference into the given `Gc`.
    ///
    /// # Safety
    /// This method is unsafe because it does not check for aliasing.
    /// The caller must ensure no other references to the data exist.
    #[must_use]
    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        // Safety: The caller guarantees no other references exist,
        // and we have a mutable reference to self, so we can safely
        // provide mutable access to the data
        unsafe { &mut self.ptr.as_mut().data }
    }

    pub(crate) fn inner(&self) -> &GcBox<T> {
        // Safety: self.ptr is guaranteed to be valid and non-null
        // because it was created from Box::into_raw and stored in NonNull
        unsafe { self.ptr.as_ref() }
    }

    /// Add this `Gc` object as a root to the global garbage collector.
    /// Returns a `GcRootGuard` that automatically removes the root when dropped.
    ///
    /// # Errors
    ///
    /// If the collector is not initialized or if the object cannot be registered as a root.
    pub fn as_root(&self, collector: &GarbageCollector) -> Result<GcRootGuard<T>>
    where
        T: Trace,
    {
        collector.create_root_guard(self.clone())
    }
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner().data
    }
}

impl<T> Drop for Gc<T> {
    fn drop(&mut self) {
        // Dropping a Gc<T> doesn't immediately free the object since other Gc<T> pointers might
        // still reference it.
    }
}

// Safety: Gc<T> can be sent between threads when T: Send + Sync because:
// 1. The NonNull<GcBox<T>> pointer is just a pointer address
// 2. The actual data access is controlled by the garbage collector
// 3. T is required to be Send + Sync by the constructor bounds
// 4. The GC ensures proper synchronization during object access
unsafe impl<T: Sync + Send> Send for Gc<T> {}

// Safety: Gc<T> can be shared between threads when T: Send + Sync because:
// 1. The NonNull<GcBox<T>> pointer is immutable after construction
// 2. Data access goes through Deref which provides shared references
// 3. T is required to be Send + Sync by the constructor bounds
// 4. The garbage collector handles thread safety for the underlying data
unsafe impl<T: Sync + Send> Sync for Gc<T> {}

impl<T: Default + Send + Sync> Default for Gc<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Display> fmt::Display for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T> fmt::Pointer for Gc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: PartialEq> PartialEq for Gc<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for Gc<T> {}

impl<T: PartialOrd> PartialOrd for Gc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Ord> Ord for Gc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash> Hash for Gc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T> Borrow<T> for Gc<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: Send + Sync> From<T> for Gc<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Trace> Trace for Gc<T> {
    fn trace(&self, collector: &GarbageCollector) {
        // Mark this object as reachable in the object registry
        let ptr = SafePtr::from_ptr(self.ptr.as_ptr().cast::<u8>());

        // Check if this object was already marked to prevent infinite recursion in cycles
        if collector.try_mark_object(ptr) {
            // Only trace the contents if this is the first time we're marking this object
            // This prevents infinite recursion in cyclic object graphs
            (**self).trace(collector);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_creation_and_access() {
        let gc = Gc::new(42);
        assert_eq!(*gc, 42);
    }

    #[test]
    fn test_creation_with_different_types() {
        let gc_int = Gc::new(123);
        let gc_string = Gc::new("Hello, World!".to_string());
        let gc_vec = Gc::new(vec![1, 2, 3, 4, 5]);
        let gc_tuple = Gc::new((1, "test", 1.23));

        assert_eq!(*gc_int, 123);
        assert_eq!(*gc_string, "Hello, World!");
        assert_eq!(*gc_vec, vec![1, 2, 3, 4, 5]);
        assert_eq!(*gc_tuple, (1, "test", 1.23));
    }

    #[test]
    fn test_clone() {
        let gc1 = Gc::new(42);
        let gc2 = gc1.clone();

        assert_eq!(*gc1, 42);
        assert_eq!(*gc2, 42);
        // Clones point to the same object
        assert!(Gc::ptr_eq(&gc1, &gc2));
    }

    #[test]
    fn test_multiple_clones() {
        let gc1 = Gc::new("shared data".to_string());
        let gc2 = gc1.clone();
        let gc3 = gc1.clone();
        let gc4 = gc2.clone();

        // All clones should point to the same object
        assert!(Gc::ptr_eq(&gc1, &gc2));
        assert!(Gc::ptr_eq(&gc1, &gc3));
        assert!(Gc::ptr_eq(&gc1, &gc4));

        // Verify data access works correctly
        assert_eq!(*gc1, "shared data");
        assert_eq!(*gc2, "shared data");
        assert_eq!(*gc3, "shared data");
        assert_eq!(*gc4, "shared data");
    }

    #[test]
    fn test_drop_behavior() {
        // Test that dropping clones doesn't affect the data
        let gc1 = Gc::new(vec![1, 2, 3]);
        let gc2 = gc1.clone();

        drop(gc1);
        // gc2 should still be accessible
        assert_eq!(*gc2, vec![1, 2, 3]);
    }

    #[test]
    fn test_equality() {
        let gc1 = Gc::new(42);
        let gc2 = Gc::new(42);
        let gc3 = gc1.clone();

        // Value equality
        assert_eq!(gc1, gc2);
        assert_eq!(gc1, gc3);

        // Pointer equality: gc1 and gc3 point to same object, gc2 is different
        assert!(Gc::ptr_eq(&gc1, &gc3));
        assert!(!Gc::ptr_eq(&gc1, &gc2));
    }

    #[test]
    fn test_with_complex_types() {
        let mut map = HashMap::new();
        map.insert("key1", 10);
        map.insert("key2", 20);

        let gc_map = Gc::new(map);
        let gc_map_clone = gc_map.clone();

        assert_eq!(gc_map.get("key1"), Some(&10));
        assert_eq!(gc_map_clone.get("key2"), Some(&20));
        assert!(Gc::ptr_eq(&gc_map, &gc_map_clone));
    }

    #[test]
    fn test_as_ptr() {
        let gc = Gc::new(42);
        let ptr = gc.as_ptr();

        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_ptr_eq() {
        let gc1 = Gc::new(42);
        let gc2 = Gc::new(42);
        let gc3 = gc1.clone();

        assert!(gc1.ptr_eq(&gc3)); // Same allocation
        assert!(!gc1.ptr_eq(&gc2)); // Different allocations
    }

    #[test]
    fn test_borrow() {
        let gc = Gc::new("test string".to_string());
        let borrowed: &String = gc.borrow();

        assert_eq!(borrowed, "test string");
        assert_eq!(borrowed.len(), 11);
    }

    #[test]
    fn test_as_ref() {
        let gc = Gc::new(vec![1, 2, 3, 4, 5]);
        let vec_ref: &Vec<i32> = gc.as_ref();

        assert_eq!(vec_ref.len(), 5);
        assert_eq!(vec_ref[2], 3);
    }

    #[test]
    fn test_from_trait() {
        let gc: Gc<i32> = 42.into();
        assert_eq!(*gc, 42);
    }

    #[test]
    fn test_debug_display() {
        let gc = Gc::new(42);
        let debug_str = format!("{gc:?}");
        let display_str = format!("{gc}");

        assert_eq!(debug_str, "42");
        assert_eq!(display_str, "42");
    }

    #[test]
    fn test_pointer_format() {
        let gc = Gc::new(42);
        let ptr_str = format!("{gc:p}");

        // Should format as a pointer (starts with 0x)
        assert!(ptr_str.starts_with("0x"));
    }

    #[test]
    fn test_ordering() {
        let gc1 = Gc::new(10);
        let gc2 = Gc::new(20);
        let gc3 = Gc::new(10);

        assert!(gc1 < gc2);
        assert!(gc2 > gc1);
        assert_eq!(gc1, gc3);
    }

    #[test]
    fn test_hash() {
        let gc1 = Gc::new(42);
        let gc2 = Gc::new(42);
        let gc3 = Gc::new(43);

        let mut set = HashSet::new();
        set.insert(gc1.clone());
        set.insert(gc2);
        set.insert(gc3);

        // Should contain 2 unique values (42 and 43)
        assert_eq!(set.len(), 2);
        assert!(set.contains(&gc1));
    }
}
