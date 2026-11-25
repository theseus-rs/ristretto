//! Strong descriptor ownership independent of payload retirement.
use super::ObjectMetadata;
use crate::allocation::{SmallAllocations, Storage};
use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering, fence};

/// The metadata address is also the base address used by the registry and
/// the publication header. Each allocation keeps its exact backing owner.
#[repr(C)]
struct Entry {
    metadata: ManuallyDrop<ObjectMetadata>,
    owners: AtomicUsize,
    storage: ManuallyDrop<Storage>,
}

pub(crate) struct MetadataRef(NonNull<Entry>);

// SAFETY: the counter grants shared ObjectMetadata access and serializes
// the final drop. Its fields, including the backing owner, are Send + Sync.
unsafe impl Send for MetadataRef where Entry: Send + Sync {}
unsafe impl Sync for MetadataRef where Entry: Send + Sync {}

impl MetadataRef {
    #[cfg(test)]
    pub(crate) fn new(metadata: ObjectMetadata) -> Self {
        Self::allocate(metadata, None)
    }

    pub(crate) fn allocate(metadata: ObjectMetadata, pool: Option<&SmallAllocations>) -> Self {
        let layout = Layout::new::<Entry>();
        let storage = pool
            .and_then(|pool| pool.allocate_metadata(layout))
            .or_else(|| Storage::global(layout).ok())
            .unwrap_or_else(|| std::alloc::handle_alloc_error(layout));
        let pointer = storage.ptr::<Entry>();
        // SAFETY: a freshly claimed slot or independent global allocation
        // has the exact size/alignment and no initialized Entry or readers.
        unsafe {
            pointer.write(Entry {
                metadata: ManuallyDrop::new(metadata),
                owners: AtomicUsize::new(1),
                storage: ManuallyDrop::new(storage),
            });
            Self(NonNull::new_unchecked(pointer))
        }
    }

    pub(crate) fn as_ptr(this: &Self) -> *const ObjectMetadata {
        this.0.as_ptr().cast()
    }

    pub(crate) fn ptr_eq(left: &Self, right: &Self) -> bool {
        left.0 == right.0
    }

    fn owners(&self) -> &AtomicUsize {
        // SAFETY: this owner keeps the counter live, including during clone.
        unsafe { &(*self.0.as_ptr()).owners }
    }

    pub(crate) fn get_mut(this: &mut Self) -> Option<&mut ObjectMetadata> {
        if this.owners().load(Ordering::Acquire) != 1 {
            return None;
        }
        // SAFETY: there are no weak owners. A published address in an owning
        // registry/publication or an ArcSwap reader debt has a backing owner.
        Some(unsafe { &mut *this.0.as_ptr().cast::<ObjectMetadata>() })
    }

    /// # Safety
    /// The pointer must retain the original entry-wide provenance from this
    /// type's `as_ptr`. A shared owner must exclude unique mutable access to
    /// the descriptor and keep the entry live until this increment finishes.
    pub(crate) unsafe fn clone_from_ptr(metadata: *const ObjectMetadata) -> Self {
        // SAFETY: repr(C) places the descriptor at byte zero of Entry.
        let borrowed = ManuallyDrop::new(Self(unsafe {
            NonNull::new_unchecked(metadata.cast_mut().cast())
        }));
        Self::clone(&borrowed)
    }
}

impl Clone for MetadataRef {
    fn clone(&self) -> Self {
        if self.owners().fetch_add(1, Ordering::Relaxed) >= isize::MAX as usize {
            std::process::abort();
        }
        Self(self.0)
    }
}

impl Deref for MetadataRef {
    type Target = ObjectMetadata;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a strong owner grants shared access to its initialized value.
        unsafe { &*Self::as_ptr(self) }
    }
}

impl std::fmt::Debug for MetadataRef {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(formatter)
    }
}

impl Drop for MetadataRef {
    fn drop(&mut self) {
        if self.owners().fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        fence(Ordering::Acquire);
        // SAFETY: this is the sole final owner. Move the backing owner out
        // before dropping metadata; unwinding also releases the slot after
        // destruction. No access to the Entry outlives that local owner.
        unsafe {
            let ptr = self.0.as_ptr();
            let _storage = ManuallyDrop::take(&mut (*ptr).storage);
            ManuallyDrop::drop(&mut (*ptr).metadata);
        }
    }
}

// SAFETY: into_ptr transfers one count; from_ptr reconstructs that exact
// count. The representation preserves provenance and uses no sentinel address.
unsafe impl arc_swap::RefCnt for MetadataRef {
    type Base = ObjectMetadata;
    fn into_ptr(me: Self) -> *mut Self::Base {
        let pointer = Self::as_ptr(&me).cast_mut();
        std::mem::forget(me);
        pointer
    }
    fn as_ptr(me: &Self) -> *mut Self::Base {
        Self::as_ptr(me).cast_mut()
    }
    unsafe fn from_ptr(pointer: *const Self::Base) -> Self {
        // SAFETY: the ArcSwap contract supplies a previously transferred count.
        Self(unsafe { NonNull::new_unchecked(pointer.cast_mut().cast()) })
    }
}

#[cfg(test)]
mod tests {
    use super::MetadataRef;
    use crate::{GarbageCollector, Gc};

    #[test]
    fn arc_swap_reader_owns_retired_metadata_until_it_releases_its_guard() -> crate::Result<()> {
        let gc = GarbageCollector::new();
        let root = Gc::try_new(&gc, 7_u64)?.into_rooted();
        let metadata = gc
            .traced_edge(root.as_ptr().addr(), root.identity())
            .expect("visible descriptor");
        let id = metadata.id;
        let cell = arc_swap::ArcSwapAny::<Option<MetadataRef>>::from(Some(metadata));
        let reader = cell.load();
        drop(root);
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.bytes_managed, 0);
        let mut removed = cell.swap(None).expect("exact owner");
        assert_eq!(reader.as_ref().expect("old descriptor").id, id);
        assert!(MetadataRef::get_mut(&mut removed).is_none());
        drop(reader);
        assert!(MetadataRef::get_mut(&mut removed).is_some());
        Ok(())
    }
}
