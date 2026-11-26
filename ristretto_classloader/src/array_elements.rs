//! Reference-array cards. Point stores demote and revisit only the affected region.
use crate::Value;
use crate::card_set::CardSet;
use ristretto_gc::{GarbageCollector, Trace, TraceProgress};
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};
use std::sync::Mutex;

const CARD_SIZE: usize = 32;

/// Reference-array storage with dirty regions for publication and generational tracing.
#[derive(Debug)]
pub struct ArrayElements {
    values: ristretto_gc::HeapBuffer<Value>,
    dirty: Option<Range<usize>>,
    cards: Mutex<CardSet>,
    owner: usize,
    mutation: usize,
    #[cfg(test)]
    scan_mutation: std::sync::atomic::AtomicUsize,
}
impl ArrayElements {
    /// Java default fields contain only numeric zeroes and null references.
    pub(crate) fn from_default_fields(values: ristretto_gc::HeapBuffer<Value>) -> Self {
        debug_assert!(
            values
                .iter()
                .all(|value| !matches!(value, Value::Object(Some(_))))
        );
        Self::with_cards(values, CardSet::default())
    }

    fn with_cards(values: ristretto_gc::HeapBuffer<Value>, cards: CardSet) -> Self {
        Self {
            dirty: (!cards.is_empty()).then_some(0..values.len()),
            values,
            cards: Mutex::new(cards),
            owner: 0,
            mutation: 0,
            #[cfg(test)]
            scan_mutation: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub(crate) fn prepare(
        &mut self,
        heap: &ristretto_gc::region::RegionHeap,
    ) -> ristretto_gc::Result<()> {
        self.values.prepare(heap)
    }
    pub(crate) fn prepare_step(
        &mut self,
        heap: &ristretto_gc::region::RegionHeap,
        bytes: usize,
    ) -> ristretto_gc::Result<TraceProgress> {
        self.values.prepare_step(heap, bytes)
    }
    pub(crate) fn relocation_region(&self, heap: &ristretto_gc::region::RegionHeap) -> Option<u64> {
        self.values.relocation_region(heap)
    }
    pub(crate) fn compact_step(
        &mut self,
        heap: &ristretto_gc::region::RegionHeap,
        bytes: usize,
    ) -> TraceProgress {
        self.values.compact_step(heap, bytes)
    }

    pub(crate) fn try_clone(&self) -> crate::Result<Self> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.values.len())
            .map_err(|_| ristretto_gc::Error::OutOfMemory)?;
        values.extend(self.values.iter().cloned());
        Ok(values.into())
    }

    fn dirty(&mut self, index: usize) {
        if index >= self.values.len() {
            return;
        }
        self.mutation = self.mutation.wrapping_add(1);
        if let Some(range) = &mut self.dirty {
            range.start = range.start.min(index);
            range.end = range.end.max(index + 1);
        } else {
            self.dirty = Some(index..index + 1);
        }
        self.cards
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(index / CARD_SIZE);
    }

    /// Returns a mutable, owning element and records its card for the next young collection.
    /// Moving the value out of this borrow preserves its graph independently of this array.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.dirty(index);
        let value = self.values.get_mut(index)?;
        value.root();
        Some(value)
    }

    /// Replaces an element without exposing its previous value outside the container.
    /// This avoids acquiring an owning root for the discarded value. Its deletion
    /// barrier and the replacement's dirty card retain the usual write semantics.
    ///
    /// # Errors
    /// Returns the unchanged input when the index is outside the array.
    pub fn set(&mut self, index: usize, value: Value) -> Result<(), Value> {
        self.dirty(index);
        let Some(element) = self.values.get_mut(index) else {
            return Err(value);
        };
        *element = value;
        Ok(())
    }

    #[expect(unsafe_code, reason = "forward protected ownership during attachment")]
    pub(crate) unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        // SAFETY: construction protection covers all initialized elements.
        unsafe { self.unroot(owner) };
    }

    /// True when the owner binding changed or the whole value was rooted for movement.
    #[expect(
        unsafe_code,
        reason = "demote only dirty elements in their actual owner"
    )]
    pub(crate) unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) -> bool {
        let rebound = self.owner != owner.address();
        self.owner = owner.address();
        if let Some(range) = self.dirty.take()
            && let Some(values) = self.values.get_range_mut(range)
        {
            for value in values {
                // SAFETY: these initialized values belong to the protected owner.
                unsafe { value.unroot(owner) };
            }
        }
        rebound
    }

    /// Ownership follows an entire array/instance moved out of a managed guard.
    /// Record the full demotion range first so unwind also cleans partial rooting.
    pub(crate) fn root(&mut self) {
        self.owner = 0;
        self.dirty = Some(0..self.values.len());
        for value in &mut self.values {
            value.root();
        }
    }

    pub(crate) fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        let elements = self.values.clear_step(offset, budget);
        if elements != TraceProgress::Complete {
            // Values still present may depend on the old-to-young card map.
            return elements;
        }
        // Card metadata also needs bounded destruction.
        for _ in 0..budget.max(1) {
            if self
                .cards
                .get_mut()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .pop_last()
                .is_none()
            {
                break;
            }
        }
        if self
            .cards
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .is_empty()
        {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(0)
        }
    }

    pub(crate) fn trace_step(
        &self,
        gc: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        let old = gc.is_young_collection() && !gc.is_young_allocation(self.owner);
        let start = if old {
            let cards = self
                .cards
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(card) = cards.next_from(offset / CARD_SIZE) else {
                return TraceProgress::Complete;
            };
            offset.max(card * CARD_SIZE)
        } else {
            offset
        };
        #[cfg(test)]
        self.scan_mutation
            .store(self.mutation, std::sync::atomic::Ordering::Relaxed);
        let card_end = (start / CARD_SIZE + 1)
            .saturating_mul(CARD_SIZE)
            .min(self.values.len());
        let end = start.saturating_add(budget.max(1)).min(card_end);
        // Only a complete card on an old owner can leave the remembered set.
        // Every edge still needs tracing, including those after a young child.
        let mut can_prune = old && end == card_end && start.is_multiple_of(CARD_SIZE);
        if let Some(values) = self.values.get(start..end) {
            for value in values {
                value.trace(gc);
                if can_prune
                    && let Value::Object(Some(reference)) = value
                    && gc.is_young_allocation(reference.as_ptr() as usize)
                {
                    can_prune = false;
                }
            }
        }
        if can_prune {
            self.cards
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(start / CARD_SIZE);
        }
        if end == self.values.len() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(end)
        }
    }
}

impl From<ristretto_gc::HeapBuffer<Value>> for ArrayElements {
    fn from(values: ristretto_gc::HeapBuffer<Value>) -> Self {
        let cards = values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                matches!(value, Value::Object(Some(_))).then_some(index / CARD_SIZE)
            })
            .collect();
        Self::with_cards(values, cards)
    }
}
impl From<Box<[Value]>> for ArrayElements {
    fn from(values: Box<[Value]>) -> Self {
        Self::from(ristretto_gc::HeapBuffer::from(values))
    }
}
impl From<Vec<Value>> for ArrayElements {
    fn from(values: Vec<Value>) -> Self {
        Self::from(ristretto_gc::HeapBuffer::from(values))
    }
}
impl Clone for ArrayElements {
    fn clone(&self) -> Self {
        self.values.to_vec().into()
    }
}
impl Deref for ArrayElements {
    type Target = [Value];
    fn deref(&self) -> &[Value] {
        &self.values
    }
}
impl DerefMut for ArrayElements {
    fn deref_mut(&mut self) -> &mut [Value] {
        self.mutation = self.mutation.wrapping_add(1);
        self.dirty = Some(0..self.values.len());
        self.cards
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .extend(0..self.values.len().div_ceil(CARD_SIZE));
        for value in &mut self.values {
            value.root();
        }
        &mut self.values
    }
}
impl Index<usize> for ArrayElements {
    type Output = Value;
    fn index(&self, index: usize) -> &Value {
        self.values.index(index)
    }
}
impl IndexMut<usize> for ArrayElements {
    #[expect(
        clippy::expect_used,
        reason = "IndexMut panics for an out-of-bounds index"
    )]
    fn index_mut(&mut self, index: usize) -> &mut Value {
        self.get_mut(index).expect("index out of bounds")
    }
}
impl AsRef<[Value]> for ArrayElements {
    fn as_ref(&self) -> &[Value] {
        self
    }
}
impl AsMut<[Value]> for ArrayElements {
    fn as_mut(&mut self) -> &mut [Value] {
        self
    }
}
impl PartialEq for ArrayElements {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}
impl Eq for ArrayElements {}
impl<'a> IntoIterator for &'a ArrayElements {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
impl<'a> IntoIterator for &'a mut ArrayElements {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}
impl IntoIterator for ArrayElements {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;
    fn into_iter(mut self) -> Self::IntoIter {
        // The iterator may outlive the original managed owner, including while
        // unconsumed elements remain in its backing storage.
        for value in &mut self.values {
            value.root();
        }
        self.values.into_iter()
    }
}

#[cfg(test)]
mod ownership_tests;

#[cfg(test)]
mod tests {
    #![expect(clippy::panic_in_result_fn, clippy::unwrap_in_result)]
    use super::*;
    use crate::Reference;
    use ristretto_gc::sync::RwLock;
    use ristretto_gc::{Configuration, Gc, GcWeak, ReferenceKind};
    use std::sync::Arc;

    #[test]
    fn incremental_clear_retains_cards_until_all_values_have_been_removed() {
        let mut array = ArrayElements::from(vec![Value::Int(1); CARD_SIZE + 1]);
        array.cards.get_mut().unwrap().insert(0);
        assert_eq!(array.clear_step(0, 1), TraceProgress::Continue(0));
        assert_eq!(array.len(), CARD_SIZE);
        assert_eq!(array.cards.get_mut().unwrap().next_from(0), Some(0));
        assert_eq!(array.clear_step(0, CARD_SIZE + 1), TraceProgress::Complete);
        assert!(array.cards.get_mut().unwrap().is_empty());
    }

    struct Array(ArrayElements);
    // SAFETY: all strong fields are traced and paired root/demotion hooks forward the owner.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Array {
        fn root(&mut self) {
            self.0.root();
        }
        fn trace(&self, gc: &GarbageCollector) {
            for value in &self.0.values {
                value.trace(gc);
            }
        }
        fn trace_step(&self, gc: &GarbageCollector, offset: usize, budget: usize) -> TraceProgress {
            self.0.trace_step(gc, offset, budget)
        }
        unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            // SAFETY: the wrapper forwards its actual protected owner.
            unsafe { self.0.unroot(owner) };
        }
        unsafe fn attach(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
            // SAFETY: the wrapper forwards construction protection and ownership.
            unsafe { self.0.attach(owner) };
        }
        fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
            self.0.clear_step(offset, budget)
        }
    }
    fn collector(budget: usize) -> Arc<GarbageCollector> {
        GarbageCollector::with_config(
            Configuration::builder()
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .promotion_age(3)
                .incremental_step_size(budget)
                .max_pause_time_us(0)
                .build(),
        )
    }
    fn young(gc: &GarbageCollector) {
        gc.collect_young();
        while !gc.step() {}
    }

    #[test]
    fn extracted_element_owns_its_graph_after_source_teardown() -> crate::Result<()> {
        let gc = collector(8);
        let target = Value::new_object(&gc, Reference::ByteArray(vec![42; 16].into()));
        let weak = match &target {
            Value::Object(Some(value)) => GcWeak::new(value, ReferenceKind::Weak, 0),
            _ => unreachable!(),
        };
        let array = Gc::new(&gc, RwLock::new(Array(vec![target].into())));
        let extracted = std::mem::replace(
            array.write().0.get_mut(0).expect("array index"),
            Value::Object(None),
        );
        drop(array);
        for _ in 0..3 {
            gc.collect_full()?;
        }
        assert!(weak.upgrade().is_some(), "moved value must own a root");
        assert_eq!(extracted.as_byte_vec_ref()?.as_ref(), &[42; 16]);
        drop(extracted);
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[test]
    #[expect(
        unsafe_code,
        reason = "narrow primitive-only updates cannot escape heap edges"
    )]
    fn indexed_writes_recopy_only_the_affected_payload_chunk() -> ristretto_gc::Result<()> {
        let bytes = 256 * size_of::<Value>();
        let heap = ristretto_gc::region::RegionHeap::new(bytes, bytes * 2, 0);
        let hole =
            ristretto_gc::HeapBuffer::try_with_fn(Some(&heap), 256, |_| Ok(Value::Object(None)))?;
        let values =
            ristretto_gc::HeapBuffer::try_with_fn(Some(&heap), 256, |_| Ok(Value::Object(None)))?;
        let gc = collector(8);
        let array = Gc::new(&gc, RwLock::new(Array(ArrayElements::from(values))));
        drop(hole);
        let chunk_bytes = (1024 / size_of::<Value>()) * size_of::<Value>();
        {
            let mut guard = array.write();
            // SAFETY: only null/primitive values are present and none escape;
            // the managed writer publishes the changed range on release.
            let elements = unsafe { &mut guard.unrooted_mut().0 };
            assert!(matches!(
                elements.compact_step(&heap, 4 * chunk_bytes),
                TraceProgress::Continue(_)
            ));
            elements[0] = Value::Int(7);
        }
        {
            let mut guard = array.write();
            // SAFETY: relocation neither extracts values nor changes graph edges.
            let elements = unsafe { &mut guard.unrooted_mut().0 };
            while matches!(
                elements.compact_step(&heap, 4096),
                TraceProgress::Continue(_)
            ) {}
        }
        assert_eq!(array.read().0[0], Value::Int(7));
        assert_eq!(heap.statistics().bytes_recopied, chunk_bytes as u64);
        drop(array);
        gc.stop()?;
        Ok(())
    }

    #[test]
    fn sparse_old_array_scans_only_dirty_cards() -> ristretto_gc::Result<()> {
        let gc = collector(32);
        let array = Gc::new(
            &gc,
            RwLock::new(Array(vec![Value::Object(None); 1_000_000].into())),
        )
        .into_rooted();
        gc.collect_full()?;
        young(&gc);
        young(&gc);
        young(&gc);
        let target = Value::new_object(&gc, Reference::ByteArray(vec![42; 1024].into()));
        let weak = match &target {
            Value::Object(Some(value)) => GcWeak::new(value, ReferenceKind::Weak, 0),
            _ => unreachable!(),
        };
        *array.write().0.get_mut(999_990).expect("array index") = target;
        let before = gc.statistics()?.collection_steps;
        young(&gc);
        assert!(gc.statistics()?.collection_steps - before < 20);
        assert!(weak.upgrade().is_some());
        *array.write().0.get_mut(999_990).expect("array index") = Value::Object(None);
        young(&gc);
        assert!(weak.upgrade().is_none());
        Ok(())
    }

    #[test]
    fn writes_between_scan_slices_do_not_lose_a_dirty_card() -> ristretto_gc::Result<()> {
        let gc = collector(1);
        let array = Gc::new(
            &gc,
            RwLock::new(Array(vec![Value::Object(None); 32].into())),
        )
        .into_rooted();
        gc.collect_full()?;
        young(&gc);
        young(&gc);
        young(&gc);
        *array.write().0.get_mut(0).expect("array index") = Value::Object(None);
        gc.collect_young();
        let mutation = array.read().0.mutation;
        while array
            .read()
            .0
            .scan_mutation
            .load(std::sync::atomic::Ordering::Relaxed)
            != mutation
        {
            gc.step();
        }
        // Slot zero has already been scanned. Publish a child into that slot before
        // the rest of this card is scanned, forcing the card version to change.
        let target = Value::new_object(&gc, Reference::ByteArray(vec![42; 1024].into()));
        let weak = match &target {
            Value::Object(Some(value)) => GcWeak::new(value, ReferenceKind::Weak, 0),
            _ => unreachable!(),
        };
        *array.write().0.get_mut(0).expect("array index") = target;
        while !gc.step() {}
        assert!(array.read().0.cards.lock().expect("cards").contains(0));
        young(&gc);
        assert!(weak.upgrade().is_some());
        Ok(())
    }
}
