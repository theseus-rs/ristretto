//! Bounded context-local allocation ranges and typed descriptor ownership.
use crate::region::{Block, RegionHeap};
use crate::{Error, Result};
use portable_atomic::AtomicU64;
use std::alloc::{Layout, alloc, dealloc};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::{Arc, Mutex, Weak};

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

/// Small global allocations share backing with other objects of the same size
/// class. A slot owner keeps the page alive. The cache has weak ownership so
/// the last returned slot releases the backing even if the heap is idle.
#[derive(Debug)]
pub(crate) struct SmallAllocations {
    budget: Arc<crate::region::Budget>,
    pages: [Mutex<Weak<SmallPage>>; 4],
}

impl SmallAllocations {
    pub(crate) fn new(budget: Arc<crate::region::Budget>) -> Self {
        Self {
            budget,
            pages: Default::default(),
        }
    }

    pub(crate) fn allocate(&self, layout: Layout) -> Option<Storage> {
        let size = layout.size().max(layout.align());
        if layout.size() == 0 || size > 64 {
            return None;
        }
        let stride = size.max(8).next_power_of_two();
        let class = (stride.trailing_zeros() - 3) as usize;
        let pages = self.pages.get(class)?;
        let cached = pages.try_lock().ok()?.upgrade();
        if let Some(page) = &cached {
            let bits = page.free.load(Relaxed);
            if bits != 0 {
                // A concurrent claim can take the observed slot. Fall back to
                // exact backing without replacing a partially occupied page.
                return page.claim(bits);
            }
        }
        let page_layout = Layout::from_size_align(64 * stride, 64).ok()?;
        let credit = self.budget.reserve(page_layout.size(), false).ok()?;
        let backing = RawAllocation::new(page_layout).ok()?;
        let page = Arc::new(SmallPage {
            backing,
            _credit: credit,
            stride,
            free: AtomicU64::new(u64::MAX ^ 1),
        });
        let mut head = pages.try_lock().ok()?;
        if head.strong_count() != 0
            && cached
                .as_ref()
                .is_none_or(|old| head.as_ptr() != Arc::as_ptr(old))
        {
            return None;
        }
        let previous = std::mem::replace(&mut *head, Arc::downgrade(&page));
        drop(head);
        drop(previous);
        Some(Storage::Small(page, 0))
    }
}

#[derive(Debug)]
pub(crate) struct SmallPage {
    // Field order returns host backing before releasing its budget credit.
    backing: RawAllocation,
    _credit: crate::region::BackingCredit,
    stride: usize,
    free: AtomicU64,
}

impl SmallPage {
    fn claim(self: &Arc<Self>, bits: u64) -> Option<Storage> {
        let index = bits.trailing_zeros();
        // A failed claim can use a different page or exact global backing.
        // Acquire pairs with the previous owner's release after destruction.
        self.free
            .compare_exchange(bits, bits & (bits - 1), Acquire, Relaxed)
            .ok()?;
        Some(Storage::Small(self.clone(), index))
    }
}

#[derive(Debug)]
pub(crate) struct RawAllocation {
    ptr: NonNull<u8>,
    layout: Layout,
}

// SAFETY: this owns raw storage, never a reference to an initialized value.
// Access requires unique Storage ownership of the complete allocation or slot.
unsafe impl Send for RawAllocation {}
unsafe impl Sync for RawAllocation {}

impl RawAllocation {
    fn new(layout: Layout) -> Result<Self> {
        if layout.size() == 0 {
            return Err(Error::OutOfMemory);
        }
        // SAFETY: the layout is valid and nonzero.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or(Error::OutOfMemory)?;
        Ok(Self { ptr, layout })
    }
}

impl Drop for RawAllocation {
    fn drop(&mut self) {
        // SAFETY: the last owner releases the allocation with its original layout.
        unsafe { dealloc(self.ptr.as_ptr(), self.layout) };
    }
}

/// Storage owns the entire allocation, including padding and the trailing ZST marker.
pub(crate) enum Storage {
    Global(RawAllocation),
    Small(Arc<SmallPage>, u32),
    Region(Block),
}

impl Storage {
    pub(crate) fn global(layout: Layout) -> Result<Self> {
        Ok(Self::Global(RawAllocation::new(layout)?))
    }

    pub(crate) fn ptr<T>(&self) -> *mut T {
        match self {
            Self::Global(allocation) => allocation.ptr.as_ptr().cast(),
            Self::Small(page, index) => page
                .backing
                .ptr
                .as_ptr()
                .wrapping_add(*index as usize * page.stride)
                .cast(),
            Self::Region(block) => block.ptr(),
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        if let Self::Small(page, index) = self {
            let bit = 1 << *index;
            let previous = page.free.fetch_or(bit, Release);
            debug_assert_eq!(previous & bit, 0, "double slot return");
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
    use std::sync::atomic::AtomicUsize;

    #[repr(align(128))]
    struct Value {
        drops: Arc<AtomicUsize>,
        panic: bool,
    }
    impl Drop for Value {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Relaxed);
            assert!(!self.panic, "injected descriptor destructor panic");
        }
    }

    #[test]
    fn small_page_returns_slots_and_releases_backing_while_cached() {
        let pool = SmallAllocations::new(crate::region::Budget::new(512));
        let layout = Layout::new::<u64>();
        let mut slots: Vec<_> = (0..64)
            .map(|_| pool.allocate(layout).expect("page slot"))
            .collect();
        assert!(pool.allocate(layout).is_none());
        let hole = slots.swap_remove(7);
        let address = hole.ptr::<u64>();
        drop(hole);
        let reused = pool.allocate(layout).expect("returned slot");
        assert_eq!(address, reused.ptr::<u64>());
        assert!(slots.iter().all(|s| s.ptr::<u64>() != address));
        drop((reused, slots));
        let old = pool.pages[0].lock().unwrap().clone();
        assert!(old.upgrade().is_none());
        assert!(pool.allocate(layout).is_some());
        assert!(pool.allocate(Layout::new::<Value>()).is_none());
    }

    #[test]
    fn small_page_busy_cache_does_not_block_exact_allocation_fallback() {
        let pool = SmallAllocations::new(crate::region::Budget::new(512));
        let layout = Layout::new::<u64>();
        let _held = pool.pages[0].lock().unwrap();
        assert!(pool.allocate(layout).is_none());
        assert!(Storage::global(layout).is_ok());
        assert!(pool.allocate(Layout::new::<()>()).is_none());
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
            assert_eq!(drops.load(Relaxed), 0);
            drop(erased);
            assert_eq!(drops.load(Relaxed), 1);
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
        assert_eq!(drops.load(Relaxed), 1);
        assert_eq!(heap.statistics().occupied_bytes, 0);
        Ok(())
    }
}
