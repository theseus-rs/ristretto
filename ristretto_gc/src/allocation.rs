//! Bounded context-local allocation ranges and typed descriptor ownership.
use crate::region::{AllocationProbe, Block, RegionHeap};
use crate::{Error, Result};
use portable_atomic::AtomicU64;
use std::alloc::{Layout, alloc, dealloc};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::{
    AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};
use std::sync::{Arc, Mutex, Weak};
use std::task::Poll;

pub(crate) const CHUNK: usize = 16 * 1024;
pub(crate) const SMALL: usize = 1024;
pub(crate) const CREDIT: usize = 4096;

/// One tail for payloads and one for descriptors; each owns its reserved bytes.
#[derive(Debug, Default)]
pub(crate) struct Cache {
    payload: Option<Block>,
    descriptor: Option<Block>,
    pub(crate) hits: u64,
    pub(crate) refills: u64,
}

/// A suspended allocation owns search positions only; cache tails remain owned
/// by the context that supplied them, including across executor migration.
#[derive(Default)]
pub(crate) struct PayloadProbe {
    pub(crate) exact: AllocationProbe,
    refill: AllocationProbe,
}

impl Cache {
    fn try_return_tail(tail: &mut Option<Block>) -> bool {
        if tail.as_mut().is_some_and(|block| !block.try_return()) {
            return false;
        }
        *tail = None;
        true
    }

    pub(crate) fn poll_payload(
        &mut self,
        heap: &RegionHeap,
        layout: Layout,
        probe: &mut PayloadProbe,
    ) -> Poll<Result<Block>> {
        if layout.size() <= SMALL && layout.align() <= 64 {
            if let Some(block) = self
                .payload
                .as_mut()
                .and_then(|tail| tail.take_prefix(layout))
            {
                self.hits += 1;
                return Poll::Ready(Ok(block));
            }
            if Self::try_return_tail(&mut self.payload) {
                let chunk = Layout::from_size_align(CHUNK, 64).map_err(|_| Error::OutOfMemory)?;
                if let Poll::Ready(Ok(mut block)) = probe.refill.poll(heap, chunk) {
                    let first = block.take_prefix(layout).ok_or(Error::OutOfMemory)?;
                    self.payload = Some(block);
                    self.refills += 1;
                    return Poll::Ready(Ok(first));
                }
            }
        }
        let result = probe.exact.poll(heap, layout);
        if matches!(&result, Poll::Ready(Err(_)))
            && (self.payload.is_some() || self.descriptor.is_some())
        {
            // Either a tail was returned, or its registry remains busy. Probe
            // again without an owned reservation after giving the executor a turn.
            Self::try_return_tail(&mut self.payload);
            Self::try_return_tail(&mut self.descriptor);
            return Poll::Pending;
        }
        result
    }

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
            ..Self::default()
        }
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
    pages: [Mutex<Weak<SmallPage>>; 5],
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
        self.allocate_class(class, stride)
    }

    /// Callers use this class only for the collector's one descriptor layout.
    pub(crate) fn allocate_metadata(&self, layout: Layout) -> Option<Storage> {
        if layout.align() > 64 || layout.size() == 0 {
            return None;
        }
        self.allocate_class(4, layout.pad_to_align().size())
    }

    fn allocate_class(&self, class: usize, stride: usize) -> Option<Storage> {
        let pages = self.pages.get(class)?;
        let cached = pages.try_lock().ok()?.upgrade();
        if let Some(page) = &cached {
            if page.stride != stride {
                return None;
            }
            match page.claim() {
                Ok(storage) => return Some(storage),
                Err(SlotMiss::Busy) => return None,
                Err(SlotMiss::Full) => {}
            }
        }
        // Aim for one 4 KiB backing block for the smallest classes. A small
        // remaining arena budget can still admit the original 64-slot page.
        let preferred = if class < 4 {
            (4096 / stride).clamp(64, 512) / 64
        } else {
            1
        };
        let (words, layout, credit) = [preferred, 1].into_iter().find_map(|words| {
            let layout = Layout::from_size_align(stride.checked_mul(64 * words)?, 64).ok()?;
            self.budget
                .reserve(layout.size(), false)
                .ok()
                .map(|credit| (words, layout, credit))
        })?;
        let backing = RawAllocation::new(layout).ok()?;
        let page = Arc::new(SmallPage {
            backing,
            _credit: credit,
            stride,
            words,
            current: AtomicUsize::new(0),
            free: std::array::from_fn(|i| {
                AtomicU64::new(if i == 0 { u64::MAX ^ 1 } else { u64::MAX })
            }),
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
    words: usize,
    current: AtomicUsize,
    free: [AtomicU64; 8],
}

enum SlotMiss {
    Full,
    Busy,
}

impl SmallPage {
    fn claim(self: &Arc<Self>) -> std::result::Result<Storage, SlotMiss> {
        let start = self.current.load(Relaxed);
        for n in 0..self.words {
            let word = (start + n) & (self.words - 1);
            let free = self.free.get(word).ok_or(SlotMiss::Full)?;
            let bits = free.load(Relaxed);
            if bits == 0 {
                continue;
            }
            let offset = bits.trailing_zeros() as usize;
            free.compare_exchange(bits, bits & (bits - 1), Acquire, Relaxed)
                .map_err(|_| SlotMiss::Busy)?;
            if n != 0 {
                self.current.store(word, Relaxed);
            }
            return Ok(Storage::Small(self.clone(), word * 64 + offset));
        }
        Err(SlotMiss::Full)
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
    Small(Arc<SmallPage>, usize),
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
                .wrapping_add(*index * page.stride)
                .cast(),
            Self::Region(block) => block.ptr(),
        }
    }
}

impl Drop for Storage {
    #[expect(
        clippy::indexing_slicing,
        reason = "only a successful claim in this word can construct this slot index"
    )]
    fn drop(&mut self) {
        if let Self::Small(page, index) = self {
            let bit = 1 << (*index % 64);
            let word = &page.free[*index / 64];
            let previous = word.fetch_or(bit, Release);
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
            drop_value: std::mem::needs_drop::<T>().then_some(drop_value::<T>),
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
    drop_value: Option<unsafe fn(*mut u8)>,
}

unsafe fn drop_value<T>(ptr: *mut u8) {
    // SAFETY: erase installs this function only for storage initialized as T.
    unsafe { std::ptr::drop_in_place(ptr.cast::<T>()) };
}

impl Erased {
    pub(crate) fn is_regional(&self) -> bool {
        matches!(self.storage, Storage::Region(_))
    }

    pub(crate) fn try_return_region(&mut self) -> bool {
        if let Storage::Region(block) = &mut self.storage {
            block.try_return()
        } else {
            false
        }
    }

    pub(crate) fn destroy_value(&mut self) {
        if let Some(drop_value) = self.drop_value.take() {
            // SAFETY: erase installed this destructor for the initialized T.
            // Taking it first prevents a second call on retry or unwind.
            unsafe { drop_value(self.storage.ptr()) };
        }
    }

    pub(crate) fn direct_refund(&self, bytes: usize) -> bool {
        self.drop_value.is_none()
            && matches!(&self.storage, Storage::Global(raw) if raw.layout.size() == bytes)
    }
}

impl Drop for Erased {
    fn drop(&mut self) {
        self.destroy_value();
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
    fn small_page_reuses_high_words_and_refunds_the_entire_page() {
        let budget = crate::region::Budget::new(4096);
        let pool = SmallAllocations::new(budget.clone());
        let layout = Layout::new::<u64>();
        let mut slots: Vec<_> = (0..512)
            .map(|_| Some(pool.allocate(layout).expect("multiword slot")))
            .collect();
        assert!(pool.allocate(layout).is_none());
        for index in [63, 64, 127, 511] {
            let slot = slots.get_mut(index).expect("known slot");
            let owner = slot.take().expect("occupied");
            let address = owner.ptr::<u64>();
            drop(owner);
            let replacement = pool.allocate(layout).expect("only free slot");
            assert_eq!(replacement.ptr::<u64>(), address);
            *slot = Some(replacement);
        }
        let cached = pool.pages[0].lock().unwrap().clone();
        let last = slots.pop().unwrap();
        drop(slots);
        assert!(budget.reserve(4096, false).is_err());
        drop(last);
        assert!(cached.upgrade().is_none());
        assert!(budget.reserve(4096, false).is_ok());
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
