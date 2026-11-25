//! Bounded context-local allocation ranges and typed descriptor ownership.
use crate::region::{Block, RegionHeap};
use crate::{Error, Result};
use std::alloc::{Layout, alloc, dealloc};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub(crate) const CHUNK: usize = 16 * 1024;
pub(crate) const SMALL: usize = 1024;
pub(crate) const CREDIT: usize = 4096;

/// One tail for payloads and one for descriptors; each owns its reserved bytes.
#[derive(Debug, Default)]
pub(crate) struct Cache {
    payload: Option<Block>,
    descriptor: Option<Block>,
    pub(crate) credit: usize,
    pub(crate) hits: u64,
    pub(crate) refills: u64,
}

impl Cache {
    pub(crate) fn allocate(
        &mut self,
        heap: &RegionHeap,
        layout: Layout,
        descriptor: bool,
    ) -> Result<Block> {
        if layout.size() > SMALL || layout.align() > 64 {
            return heap.allocate(layout, None).or_else(|_| {
                self.release_ranges();
                heap.allocate(layout, None)
            });
        }
        let tail = if descriptor {
            &mut self.descriptor
        } else {
            &mut self.payload
        };
        if let Some(block) = tail.as_mut().and_then(|tail| tail.take_prefix(layout)) {
            self.hits += 1;
            return Ok(block);
        }
        // Return the previous tail before reserving a refill. A failed optional
        // refill falls back to the exact request, including on very small heaps.
        *tail = None;
        let chunk = Layout::from_size_align(CHUNK, 64).map_err(|_| Error::OutOfMemory)?;
        if let Ok(mut block) = heap.allocate(chunk, None) {
            let allocation = block.take_prefix(layout).ok_or(Error::OutOfMemory)?;
            *tail = Some(block);
            self.refills += 1;
            return Ok(allocation);
        }
        self.release_ranges();
        heap.allocate(layout, None)
    }

    pub(crate) fn retained_bytes(&self) -> usize {
        self.payload.as_ref().map_or(0, Block::len) + self.descriptor.as_ref().map_or(0, Block::len)
    }

    /// Detach unused ownership without freeing ranges under the context mutex.
    /// Usage counters belong to the live cache and are not reset by cleanup.
    pub(crate) fn take_unused(&mut self) -> Self {
        Self {
            payload: self.payload.take(),
            descriptor: self.descriptor.take(),
            credit: std::mem::take(&mut self.credit),
            ..Self::default()
        }
    }

    pub(crate) fn release(&mut self) -> usize {
        self.release_ranges();
        std::mem::take(&mut self.credit)
    }

    fn release_ranges(&mut self) {
        self.payload = None;
        self.descriptor = None;
    }
}

/// Storage owns the entire allocation, including padding and the trailing ZST marker.
pub(crate) enum Storage {
    Global { ptr: NonNull<u8>, layout: Layout },
    Region(Block),
}

// SAFETY: storage uniquely owns a disjoint, uninitialized range. Initialized<T>
// carries T's ownership/auto-trait requirements; no reference is exposed by Storage.
unsafe impl Send for Storage {}
unsafe impl Sync for Storage {}

impl Storage {
    pub(crate) fn global(layout: Layout) -> Result<Self> {
        if layout.size() == 0 {
            return Err(Error::OutOfMemory);
        }
        // SAFETY: descriptor layouts include a marker and are always nonzero.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or(Error::OutOfMemory)?;
        Ok(Self::Global { ptr, layout })
    }

    pub(crate) fn ptr<T>(&self) -> *mut T {
        match self {
            Self::Global { ptr, .. } => ptr.as_ptr().cast(),
            Self::Region(block) => block.ptr(),
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        if let Self::Global { ptr, layout } = self {
            // SAFETY: the initialized value has been destroyed; this owns the allocation.
            unsafe { dealloc(ptr.as_ptr(), *layout) };
        }
    }
}

pub(crate) struct Initialized<T> {
    storage: Storage,
    value: PhantomData<T>,
}

impl<T> Initialized<T> {
    /// # Safety
    /// Storage must have the size and alignment of T and contain no initialized value.
    pub(crate) unsafe fn new(storage: Storage, value: T) -> Self {
        // SAFETY: callers allocate with Layout::new::<T>(), before exposing the range.
        unsafe { storage.ptr::<T>().write(value) };
        Self {
            storage,
            value: PhantomData,
        }
    }

    pub(crate) fn ptr(&self) -> *mut T {
        self.storage.ptr()
    }

    /// Move typed storage ownership into metadata without allocating a closure.
    /// The bounds remain necessary even though the erased owner has no T field.
    pub(crate) fn erase(self) -> Erased
    where
        T: Send + Sync + 'static,
    {
        let this = std::mem::ManuallyDrop::new(self);
        // SAFETY: this consumes the only initialized owner and suppresses its
        // destructor. Moving storage cannot panic; Erased now owns both the T
        // destructor and backing allocation, with the same Send/Sync bounds.
        let storage = unsafe { std::ptr::read(&raw const this.storage) };
        Erased {
            storage,
            drop_value: drop_value::<T>,
        }
    }
}

impl<T> Drop for Initialized<T> {
    fn drop(&mut self) {
        // SAFETY: this is the sole owner of the initialized descriptor. Storage
        // releases the range afterward, including if T's destructor unwinds.
        unsafe { std::ptr::drop_in_place(self.ptr()) };
    }
}

/// The backing allocation and a monomorphized destructor replace a boxed `FnOnce`.
/// Storage's automatic field drop also runs if the value's destructor unwinds.
pub(crate) struct Erased {
    storage: Storage,
    drop_value: unsafe fn(*mut u8),
}

unsafe fn drop_value<T>(ptr: *mut u8) {
    // SAFETY: erase installs this function only for storage initialized as T.
    unsafe { std::ptr::drop_in_place(ptr.cast::<T>()) };
}

impl Drop for Erased {
    fn drop(&mut self) {
        // SAFETY: this is the unique initialized owner until the destructor
        // returns (or unwinds). Backing storage is released afterward.
        unsafe { (self.drop_value)(self.storage.ptr()) };
    }
}

#[cfg(test)]
#[expect(clippy::panic_in_result_fn, reason = "assert allocation ownership")]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[repr(align(128))]
    struct Value {
        drops: Arc<AtomicUsize>,
        panic: bool,
    }
    impl Drop for Value {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
            assert!(!self.panic, "injected descriptor destructor panic");
        }
    }

    #[test]
    fn erasure_preserves_alignment_and_exactly_once_drop() -> Result<()> {
        for region in [false, true] {
            let heap = RegionHeap::new(4096, 8192, 0);
            let layout = Layout::new::<Value>();
            let storage = if region {
                Storage::Region(heap.allocate(layout, None)?)
            } else {
                Storage::global(layout)?
            };
            let drops = Arc::new(AtomicUsize::new(0));
            // SAFETY: storage has the complete nonzero Value layout.
            let initialized = unsafe {
                Initialized::new(
                    storage,
                    Value {
                        drops: drops.clone(),
                        panic: false,
                    },
                )
            };
            let pointer = initialized.ptr();
            let erased = initialized.erase();
            assert_eq!(erased.storage.ptr::<Value>(), pointer);
            assert_eq!(pointer as usize % 128, 0);
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            drop(erased);
            assert_eq!(drops.load(Ordering::Relaxed), 1);
            assert_eq!(heap.statistics().occupied_bytes, 0);
        }
        Ok(())
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn erased_destructor_unwind_releases_region_storage() -> Result<()> {
        let heap = RegionHeap::new(4096, 8192, 0);
        let layout = Layout::new::<Value>();
        let storage = Storage::Region(heap.allocate(layout, None)?);
        let drops = Arc::new(AtomicUsize::new(0));
        // SAFETY: the region owns a disjoint, correctly aligned Value allocation.
        let erased = unsafe {
            Initialized::new(
                storage,
                Value {
                    drops: drops.clone(),
                    panic: true,
                },
            )
        }
        .erase();
        assert_eq!(heap.statistics().occupied_bytes, layout.size());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(erased)));
        assert!(result.is_err());
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(heap.statistics().occupied_bytes, 0);
        Ok(())
    }
}
