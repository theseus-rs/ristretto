//! Contiguous payload storage whose address may change only under an exclusive owner guard.
use crate::region::{Block, RegionHeap};
use crate::{Error, Result, TraceProgress};
use std::alloc::Layout;
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
enum Storage<T> {
    Host(Vec<T>),
    Region(Block),
}

#[derive(Debug)]
struct Copy {
    destination: Block,
    cursor: usize,
    high_water: usize,
    chunk_elements: usize,
    dirty: Vec<u64>,
}

#[expect(
    clippy::indexing_slicing,
    reason = "bitmap indices derive from validated buffer ranges"
)]
impl Copy {
    fn invalidate(&mut self, range: std::ops::Range<usize>) {
        if range.is_empty() {
            return;
        }
        let first = range.start / self.chunk_elements;
        let last = (range.end - 1) / self.chunk_elements;
        let first_word = first / 64;
        let last_word = last / 64;
        for (offset, bits) in self.dirty[first_word..=last_word].iter_mut().enumerate() {
            let start = if offset == 0 { first % 64 } else { 0 };
            let end = if first_word + offset == last_word {
                last % 64
            } else {
                63
            };
            *bits |= (u64::MAX << start) & (u64::MAX >> (63 - end));
        }
        self.cursor = self.cursor.min(first * self.chunk_elements);
    }

    fn next_dirty(&self, from: usize) -> Option<usize> {
        let first_word = from / 64;
        self.dirty[first_word..]
            .iter()
            .enumerate()
            .find_map(|(offset, word)| {
                let word = if offset == 0 {
                    word & (u64::MAX << (from % 64))
                } else {
                    *word
                };
                (word != 0).then(|| (first_word + offset) * 64 + word.trailing_zeros() as usize)
            })
    }
}

#[derive(Debug)]
enum Transfer {
    Preparing { destination: Block, cursor: usize },
    Compacting(Copy),
}

/// Movable backing storage. References into it borrow the owner and therefore exclude relocation.
/// Large copies are incremental; mutation invalidates copied data before the next commit.
pub struct HeapBuffer<T> {
    storage: Storage<T>,
    length: usize,
    copying: Option<Transfer>,
}

impl<T> HeapBuffer<T> {
    /// Initializes storage directly in the selected heap, without a staging vector.
    /// Unpublished elements remain exclusively owned, including during unwinding.
    ///
    /// # Errors
    /// Returns an allocation or initializer error, dropping the initialized prefix.
    pub fn try_with_fn(
        heap: Option<&RegionHeap>,
        length: usize,
        initialize: impl FnMut(usize) -> Result<T>,
    ) -> Result<Self> {
        Self::try_empty(heap, length)?.initialize(length, initialize)
    }

    /// Initializes directly in a collector, recovering physical space before initialization.
    /// The initializer runs at most once per element, even when allocation is retried.
    /// # Errors
    /// Returns an allocation or initializer error. Recovery never runs under managed guards.
    pub fn try_with_fn_in(
        collector: &crate::GarbageCollector,
        length: usize,
        initialize: impl FnMut(usize) -> Result<T>,
    ) -> Result<Self> {
        let layout = Layout::array::<T>(length).map_err(|_| Error::OutOfMemory)?;
        if !crate::cell::has_guards() {
            collector.make_room(layout.size())?;
        }
        let mut retry = crate::recovery::PhysicalRetry::default();
        loop {
            match Self::try_empty_in(collector, length) {
                Ok(buffer) => return buffer.initialize(length, initialize),
                Err(error) if crate::cell::has_guards() || retry.exhausted(collector) => {
                    return Err(error);
                }
                Err(_) => collector.recover_physical(layout),
            }
        }
    }

    /// Initializes directly, awaiting storage recovery and yielding between 64 KiB
    /// initialization batches (at least one element). Cancellation drops only the
    /// initialized prefix and releases the reserved physical range.
    /// # Errors
    /// Returns an allocation/initializer error, or an error for a managed guard.
    pub async fn try_with_fn_in_async(
        collector: &crate::GarbageCollector,
        length: usize,
        mut initialize: impl FnMut(usize) -> Result<T>,
    ) -> Result<Self> {
        if crate::cell::has_guards() {
            return Err(Error::CollectionPhaseError(
                "asynchronous initialization requires released managed guards".into(),
            ));
        }
        let layout = Layout::array::<T>(length).map_err(|_| Error::OutOfMemory)?;
        collector.make_room_async(layout.size()).await?;
        let mut retry = crate::recovery::PhysicalRetry::default();
        let mut probe = crate::allocation::PayloadProbe::default();
        loop {
            let mut wait = None;
            let block = std::future::poll_fn(|cx| {
                if crate::cell::has_guards() {
                    return std::task::Poll::Ready(Err(Error::CollectionPhaseError(
                        "asynchronous initialization requires released managed guards".into(),
                    )));
                }
                let result = collector.poll_payload(layout, &mut probe);
                if result.is_pending() {
                    // At the physical limit a parked shard can hide free space.
                    // Probe for a returned range on every poll while an existing
                    // worker or at most one cooperative step reclaims other ranges.
                    if probe.exact.needs_collection && !retry.exhausted(collector) {
                        let ticket = wait.get_or_insert_with(|| {
                            crate::recovery::CollectionWait::new(
                                collector,
                                collector.request_recovery_cycle(),
                            )
                        });
                        if let std::task::Poll::Ready(completed) =
                            std::pin::Pin::new(ticket).poll(cx)
                        {
                            completed?;
                            wait = None;
                        }
                    }
                    cx.waker().wake_by_ref();
                }
                result
            })
            .await;
            drop(wait);
            let storage = match block {
                Ok(Some(block)) => Ok(Self {
                    storage: Storage::Region(block),
                    length: 0,
                    copying: None,
                }),
                Ok(None) => Self::try_empty(None, length),
                Err(Error::OutOfMemory) => Err(Error::OutOfMemory),
                Err(error) => return Err(error),
            };
            match storage {
                Ok(mut buffer) => {
                    let chunk_elements = (64 * 1024 / size_of::<T>().max(1)).max(1);
                    while buffer.length < length {
                        let end = buffer.length.saturating_add(chunk_elements).min(length);
                        buffer.initialize_to(end, &mut initialize)?;
                        if buffer.length < length {
                            crate::recovery::yield_once().await;
                        }
                    }
                    return Ok(buffer);
                }
                Err(error) if retry.exhausted(collector) => return Err(error),
                Err(_) => collector.recover_physical_async(layout).await?,
            }
        }
    }

    fn try_empty_in(collector: &crate::GarbageCollector, length: usize) -> Result<Self> {
        let layout = Layout::array::<T>(length).map_err(|_| Error::OutOfMemory)?;
        if layout.size() != 0
            && let Some(block) = collector.allocate_payload(layout)?
        {
            return Ok(Self {
                storage: Storage::Region(block),
                length: 0,
                copying: None,
            });
        }
        Self::try_empty(None, length)
    }

    fn try_empty(heap: Option<&RegionHeap>, length: usize) -> Result<Self> {
        let layout = Layout::array::<T>(length).map_err(|_| Error::OutOfMemory)?;
        let Some(heap) = heap.filter(|_| layout.size() != 0) else {
            let mut values = Vec::new();
            values
                .try_reserve_exact(length)
                .map_err(|_| Error::OutOfMemory)?;
            return Ok(values.into());
        };
        let block = heap.allocate(layout, None)?;
        Ok(Self {
            storage: Storage::Region(block),
            length: 0,
            copying: None,
        })
    }

    fn initialize(
        mut self,
        length: usize,
        mut initialize: impl FnMut(usize) -> Result<T>,
    ) -> Result<Self> {
        self.initialize_to(length, &mut initialize)?;
        Ok(self)
    }

    // Keep the element loop outside the async state machine so primitive fills can
    // be optimized as a single batch, while preserving prefix ownership on failure.
    fn initialize_to(
        &mut self,
        length: usize,
        initialize: &mut impl FnMut(usize) -> Result<T>,
    ) -> Result<()> {
        // Select storage once per chunk. Keeping the host and region loops separate
        // avoids a per-element backend branch and makes primitive fills easier to optimize.
        match &mut self.storage {
            Storage::Host(values) => {
                for index in self.length..length {
                    values.push(initialize(index)?);
                    // Commit each successful initializer so error/unwind drops only its prefix.
                    self.length += 1;
                }
            }
            Storage::Region(block) => {
                for index in self.length..length {
                    let value = initialize(index)?;
                    // SAFETY: initialize_to only fills the range reserved by try_empty.
                    // This slot is uninitialized; commit ownership after writing it.
                    unsafe {
                        block.ptr::<T>().add(index).write(value);
                    }
                    self.length += 1;
                }
            }
        }
        Ok(())
    }

    /// Moves backing storage into this heap. Failure leaves the source intact.
    /// # Errors
    /// Returns `OutOfMemory` if the backing storage cannot be reserved.
    pub fn prepare(&mut self, heap: &RegionHeap) -> Result<()> {
        self.prepare_step(heap, usize::MAX)?;
        Ok(())
    }

    /// Prepares at most `bytes` of unpublished storage (at least one element).
    /// Until completion, the source retains every element's ownership. Cancellation
    /// frees only raw destination bytes; intervening mutation cancels partial work.
    /// # Errors
    /// Returns `OutOfMemory` without changing the source if reservation fails.
    pub fn prepare_step(&mut self, heap: &RegionHeap, bytes: usize) -> Result<TraceProgress> {
        if matches!(&self.storage, Storage::Region(block) if heap.owns(block))
            || self.length == 0
            || size_of::<T>() == 0
        {
            self.copying = None;
            return Ok(TraceProgress::Complete);
        }
        if !matches!(&self.copying, Some(Transfer::Preparing { destination, .. }) if heap.owns(destination))
        {
            // Transfers between heaps cannot commit an earlier destination, including
            // partially copied relocation storage from the original heap.
            self.copying = None;
            let layout = Layout::array::<T>(self.length).map_err(|_| Error::OutOfMemory)?;
            let mut destination = heap.allocate(layout, None)?;
            destination.set_staging(true);
            self.copying = Some(Transfer::Preparing {
                destination,
                cursor: 0,
            });
        }
        let source = self.as_ptr();
        let Some(Transfer::Preparing {
            destination,
            cursor,
        }) = &mut self.copying
        else {
            return Ok(TraceProgress::Complete);
        };
        let end = cursor
            .saturating_add((bytes / size_of::<T>()).max(1))
            .min(self.length);
        // SAFETY: the source owns all initialized elements until commit. The raw
        // destination is disjoint, unexposed, and never invokes T's destructor.
        unsafe {
            std::ptr::copy_nonoverlapping(
                source.add(*cursor),
                destination.ptr::<T>().add(*cursor),
                end - *cursor,
            );
        }
        heap.record_staging((end - *cursor) * size_of::<T>());
        *cursor = end;
        if end < self.length {
            return Ok(TraceProgress::Continue(end));
        }
        if let Some(Transfer::Preparing {
            mut destination, ..
        }) = self.copying.take()
        {
            destination.set_staging(false);
            self.commit(destination);
        }
        Ok(TraceProgress::Complete)
    }

    /// Packs using the heap's destination policy, copying at most `bytes` per call.
    /// A caller must hold the containing managed object's exclusive guard.
    #[expect(
        clippy::indexing_slicing,
        reason = "bitmap covers exactly the nonempty payload"
    )]
    pub fn compact_step(&mut self, heap: &RegionHeap, bytes: usize) -> TraceProgress {
        if matches!(&self.copying, Some(Transfer::Preparing { .. })) {
            self.copying = None;
        }
        if bytes == 0 || self.length == 0 || size_of::<T>() == 0 {
            self.copying = None;
            return TraceProgress::Complete;
        }
        if let (Storage::Region(source), Some(Transfer::Compacting(copy))) =
            (&self.storage, &self.copying)
            && !heap.continue_relocation(source, &copy.destination)
        {
            self.copying = None;
        }
        if self.copying.is_none() {
            let Storage::Region(source) = &self.storage else {
                return TraceProgress::Complete;
            };
            let Ok(layout) = Layout::array::<T>(self.length) else {
                return TraceProgress::Complete;
            };
            let Ok(destination) = heap.allocate(layout, Some(source)) else {
                return TraceProgress::Complete;
            };
            // One bit per 1 KiB chunk (or per oversized element). Fallible metadata
            // reservation must leave the source untouched if it cannot be satisfied.
            let chunk_elements = (1024 / size_of::<T>()).max(1);
            let chunks = self.length.div_ceil(chunk_elements);
            let words = chunks.div_ceil(64);
            let mut dirty = Vec::new();
            if dirty.try_reserve_exact(words).is_err() {
                return TraceProgress::Complete;
            }
            dirty.resize(words, u64::MAX);
            if !chunks.is_multiple_of(64) {
                dirty[words - 1] = (1_u64 << (chunks % 64)) - 1;
            }
            self.copying = Some(Transfer::Compacting(Copy {
                destination,
                cursor: 0,
                high_water: 0,
                chunk_elements,
                dirty,
            }));
        }
        let source = self.as_ptr();
        let Some(Transfer::Compacting(copy)) = &mut self.copying else {
            return TraceProgress::Complete;
        };
        let mut remaining = (bytes / size_of::<T>()).max(1);
        loop {
            let chunk = copy.cursor / copy.chunk_elements;
            let chunk_end = (chunk + 1)
                .saturating_mul(copy.chunk_elements)
                .min(self.length);
            let end = copy.cursor.saturating_add(remaining).min(chunk_end);
            // SAFETY: copy.destination is raw, unexposed storage. Intermediate copies have
            // no Rust ownership and are never read or dropped. Writes dirty their chunks
            // under the same exclusive owner guard, so publication cannot lose a write.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    source.add(copy.cursor),
                    copy.destination.ptr::<T>().add(copy.cursor),
                    end - copy.cursor,
                );
            }
            heap.record_copy((end - copy.cursor) * size_of::<T>());
            heap.record_recopy(
                copy.high_water.min(end).saturating_sub(copy.cursor) * size_of::<T>(),
            );
            remaining -= end - copy.cursor;
            copy.high_water = copy.high_water.max(end);
            copy.cursor = end;
            if end == chunk_end {
                copy.dirty[chunk / 64] &= !(1_u64 << (chunk % 64));
                let Some(next) = copy.next_dirty(chunk + 1) else {
                    break;
                };
                copy.cursor = next * copy.chunk_elements;
            }
            if remaining == 0 {
                return TraceProgress::Continue(copy.cursor);
            }
        }
        // invalidate always moves the cursor back to the earliest affected
        // chunk, so there cannot be an outstanding dirty chunk before it.
        if let Some(Transfer::Compacting(mut copy)) = self.copying.take() {
            if let Storage::Region(source) = &mut self.storage {
                source.transfer_ownership(&mut copy.destination);
            }
            self.commit(copy.destination);
            heap.record_relocation();
        }
        TraceProgress::Complete
    }

    fn commit(&mut self, destination: Block) {
        let old = std::mem::replace(&mut self.storage, Storage::Region(destination));
        if let Storage::Host(mut values) = old {
            // SAFETY: ownership of all initialized elements was moved to the destination.
            unsafe { values.set_len(0) };
        }
        // A Region Block owns raw bytes, not T destructors. Retiring it cannot double-drop.
    }

    fn invalidate(&mut self, range: std::ops::Range<usize>) {
        if matches!(&self.copying, Some(Transfer::Preparing { .. })) {
            self.copying = None;
        }
        if !range.is_empty()
            && let Storage::Region(block) = &self.storage
        {
            block.record_mutation();
        }
        if let Some(Transfer::Compacting(copy)) = &mut self.copying {
            copy.invalidate(range);
        }
    }

    /// Returns a mutable element and invalidates the affected part of an in-progress move.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        self.invalidate(index..index + 1);
        // SAFETY: index is initialized and the owner is exclusively borrowed.
        Some(unsafe { &mut *self.ptr().add(index) })
    }

    /// Mutably borrows a range, invalidating only the affected relocation chunks.
    pub fn get_range_mut(&mut self, range: std::ops::Range<usize>) -> Option<&mut [T]> {
        if range.start > range.end || range.end > self.length {
            return None;
        }
        if !range.is_empty() {
            self.invalidate(range.clone());
        }
        // SAFETY: the checked initialized range belongs exclusively to this buffer.
        Some(unsafe { std::slice::from_raw_parts_mut(self.ptr().add(range.start), range.len()) })
    }

    /// Drops a suffix without revisiting it during final destruction.
    pub fn truncate(&mut self, length: usize) {
        if length >= self.length {
            return;
        }
        self.copying = None;
        // Commit the shorter initialized range before invoking user destructors.
        // Unwinding must neither expose nor destroy an already dropped suffix again.
        let previous_length = std::mem::replace(&mut self.length, length);
        match &mut self.storage {
            Storage::Host(values) => values.truncate(length),
            Storage::Region(block) => {
                // SAFETY: the suffix is initialized and exclusively owned. The shorter
                // length below excludes it from final destruction.
                unsafe {
                    std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                        block.ptr::<T>().add(length),
                        previous_length - length,
                    ));
                };
            }
        }
    }

    fn ptr(&self) -> *mut T {
        match &self.storage {
            Storage::Host(values) => values.as_ptr().cast_mut(),
            Storage::Region(block) => block.ptr(),
        }
    }

    /// Allocated element capacity charged to the managed object.
    #[must_use]
    pub fn capacity(&self) -> usize {
        match &self.storage {
            Storage::Host(values) => values.capacity(),
            Storage::Region(_) => self.length,
        }
    }

    /// Advisory region identity for relocation selection; no storage lease or root.
    #[must_use]
    pub fn relocation_region(&self, heap: &RegionHeap) -> Option<u64> {
        match &self.storage {
            Storage::Region(block) if heap.owns(block) => Some(block.id()),
            _ => None,
        }
    }
}

impl<T> HeapBuffer<T> {
    /// Transfers elements to an ordinary vector, releasing the region range afterward.
    #[must_use]
    pub fn into_vec(mut self) -> Vec<T> {
        let length = self.length;
        // Reserve before changing ownership, so allocation failure leaves self intact.
        let mut result = Vec::with_capacity(length);
        let storage = std::mem::replace(&mut self.storage, Storage::Host(Vec::new()));
        self.length = 0;
        match storage {
            Storage::Host(values) => values,
            Storage::Region(block) => {
                // SAFETY: the source owns length initialized elements, ownership transfers
                // to the vector; Block only frees raw storage and self now has length zero.
                unsafe {
                    std::ptr::copy_nonoverlapping(block.ptr::<T>(), result.as_mut_ptr(), length);
                    result.set_len(length);
                }
                result
            }
        }
    }
}
impl<T> IntoIterator for HeapBuffer<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}
impl<'a, T> IntoIterator for &'a HeapBuffer<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut HeapBuffer<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> From<Vec<T>> for HeapBuffer<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            length: values.len(),
            storage: Storage::Host(values),
            copying: None,
        }
    }
}
impl<T> From<Box<[T]>> for HeapBuffer<T> {
    fn from(values: Box<[T]>) -> Self {
        values.into_vec().into()
    }
}
impl<T: Clone> Clone for HeapBuffer<T> {
    fn clone(&self) -> Self {
        self.to_vec().into()
    }
}
impl<T> Deref for HeapBuffer<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        // SAFETY: storage owns length initialized elements; borrowing self prevents a move.
        unsafe { std::slice::from_raw_parts(self.ptr(), self.length) }
    }
}
impl<T> DerefMut for HeapBuffer<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.invalidate(0..self.length);
        // SAFETY: exclusive access to all initialized elements.
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.length) }
    }
}
impl<T: fmt::Debug> fmt::Debug for HeapBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}
impl<T: PartialEq> PartialEq for HeapBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
impl<T: Eq> Eq for HeapBuffer<T> {}
impl<T> AsRef<[T]> for HeapBuffer<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}
impl<T> AsMut<[T]> for HeapBuffer<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
impl<T> Drop for HeapBuffer<T> {
    fn drop(&mut self) {
        if let Storage::Region(block) = &self.storage {
            // SAFETY: only the current published storage owns initialized elements.
            unsafe {
                std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                    block.ptr::<T>(),
                    self.length,
                ));
            };
        }
    }
}

impl<T> FromIterator<T> for HeapBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::from_iter(iter).into()
    }
}

// SAFETY: initialized elements are traced and converted in place. Rooting
// restores ownership before storage can move; preparation/compaction preserve
// initialization and existing guarded relocation invariants.
unsafe impl<T: crate::Trace> crate::Trace for HeapBuffer<T> {
    fn has_bounded_cleanup() -> bool {
        // Clearing returns each owned block cooperatively before ordinary destruction.
        !std::mem::needs_drop::<T>()
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace_work(&self) -> usize {
        if !T::may_contain_gc_references() && !std::mem::needs_drop::<T>() {
            1
        } else {
            32
        }
    }
    fn trace(&self, collector: &crate::GarbageCollector) {
        if T::may_contain_gc_references() {
            for value in self {
                value.trace(collector);
            }
        }
    }
    fn trace_step(
        &self,
        collector: &crate::GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        let end = offset.saturating_add(budget.max(1)).min(self.len());
        if let Some(values) = self.get(offset..end) {
            for value in values {
                value.trace(collector);
            }
        }
        if end == self.len() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(end)
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self {
            // SAFETY: elements belong to the same exclusively borrowed owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self {
            // SAFETY: construction protection covers every initialized element.
            unsafe { value.attach(owner) };
        }
    }
    fn root(&mut self) {
        for value in self {
            value.root();
        }
    }
    fn clear_step(&mut self, _: usize, budget: usize) -> TraceProgress {
        if let Some(transfer) = &mut self.copying {
            let destination = match transfer {
                Transfer::Preparing { destination, .. } => destination,
                Transfer::Compacting(copy) => &mut copy.destination,
            };
            // Staged bytes still belong to the source. Return their raw range
            // before truncating, including when its registry is contended.
            if !destination.try_return() {
                return TraceProgress::Busy;
            }
            self.copying = None;
        }
        let keep = if std::mem::needs_drop::<T>() {
            self.len().saturating_sub(budget.max(1))
        } else {
            0
        };
        self.truncate(keep);
        if !self.is_empty() {
            return TraceProgress::Continue(0);
        }
        if let Storage::Region(block) = &mut self.storage {
            if !block.try_return() {
                return TraceProgress::Busy;
            }
            self.storage = Storage::Host(Vec::new());
        }
        TraceProgress::Complete
    }
    fn external_size(&self) -> usize {
        self.capacity() * size_of::<T>()
    }
    fn prepare(&mut self, heap: &RegionHeap) -> Result<()> {
        Self::prepare(self, heap)
    }
    fn prepare_step(&mut self, heap: &RegionHeap, bytes: usize) -> Result<TraceProgress> {
        Self::prepare_step(self, heap, bytes)
    }
    fn compact_step(&mut self, heap: &RegionHeap, bytes: usize) -> TraceProgress {
        Self::compact_step(self, heap, bytes)
    }
    fn relocation_region(&self, heap: &RegionHeap) -> Option<u64> {
        Self::relocation_region(self, heap)
    }
}
