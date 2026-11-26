use super::{GarbageCollector, HashMap, HashSet, Mutex, RwLock, Trace, TraceProgress, lock};

// SAFETY: all hooks forward to the present value with the same protected owner.
unsafe impl<T: Trace> Trace for Option<T> {
    fn external_size(&self) -> usize {
        self.as_ref().map_or(0, T::external_size)
    }

    fn has_bounded_drop() -> bool {
        T::has_bounded_drop()
    }

    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn edges_rooted_before_unroot(&self) -> bool {
        self.as_ref().is_some_and(Trace::edges_rooted_before_unroot)
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.as_mut().map_or(TraceProgress::Complete, |value| {
            value.clear_step(offset, budget)
        })
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        if let Some(value) = self {
            // SAFETY: this field belongs to the caller's protected owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        if let Some(value) = self {
            // SAFETY: construction protection covers the present value.
            unsafe { value.attach(owner) };
        }
    }
    fn root(&mut self) {
        if let Some(value) = self {
            value.root();
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        if let Some(value) = self {
            value.trace(collector);
        }
    }
}

// SAFETY: every initialized element is traced and participates in ownership
// conversion; clearing only destroys initialized elements under exclusive access.
unsafe impl<T: Trace> Trace for Vec<T> {
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_drop()
    }

    fn has_bounded_drop() -> bool {
        // Direct destruction visits the entire vector if elements have drop glue.
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
    fn clear_step(&mut self, _offset: usize, budget: usize) -> TraceProgress {
        // Clearing elements with no drop glue only changes the length.
        if !std::mem::needs_drop::<T>() {
            self.clear();
            return TraceProgress::Complete;
        }
        self.truncate(self.len().saturating_sub(budget.max(1)));
        if self.is_empty() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(0)
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self {
            // SAFETY: initialized elements share the caller's protected owner.
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
    fn trace(&self, collector: &GarbageCollector) {
        if T::may_contain_gc_references() {
            for item in self {
                item.trace(collector);
            }
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        let end = offset.saturating_add(budget.max(1)).min(self.len());
        if let Some(items) = self.get(offset..end) {
            T::trace_slice(items, collector);
        }
        if end == self.len() {
            TraceProgress::Complete
        } else {
            TraceProgress::Continue(end)
        }
    }

    fn external_size(&self) -> usize {
        self.capacity()
            .saturating_mul(size_of::<T>())
            .saturating_add(T::slice_external_size(self))
    }
}

// SAFETY: keys remain independently owning because the map cannot expose them
// mutably for ownership conversion. Values are traced and converted in place.
unsafe impl<K: Trace, V: Trace, S: ::std::hash::BuildHasher> Trace for HashMap<K, V, S> {
    fn root(&mut self) {
        for value in self.values_mut() {
            value.root();
        }
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self.values_mut() {
            // SAFETY: values belong to this exclusively borrowed traced owner.
            unsafe { value.unroot(owner) };
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        for value in self.values_mut() {
            // SAFETY: construction protection covers the embedded values.
            unsafe { value.attach(owner) };
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        for (key, value) in self {
            key.trace(collector);
            value.trace(collector);
        }
    }
}

// SAFETY: set keys are never demoted and remain independently owning; tracing
// still visits them. Their ownership cannot be mutated through the set API.
unsafe impl<T: Trace, S: ::std::hash::BuildHasher> Trace for HashSet<T, S> {
    fn trace(&self, collector: &GarbageCollector) {
        for item in self {
            item.trace(collector);
        }
    }
}

// SAFETY: every component is traced and receives the same ownership context.
unsafe impl<A: Trace, B: Trace> Trace for (A, B) {
    fn external_size(&self) -> usize {
        self.0
            .external_size()
            .saturating_add(self.1.external_size())
    }

    fn needs_clear() -> bool {
        A::needs_clear() || B::needs_clear()
    }
    fn root(&mut self) {
        self.0.root();
        self.1.root();
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: both components belong to the protected tuple owner.
        unsafe {
            self.0.unroot(owner);
            self.1.unroot(owner);
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: construction protection covers both components.
        unsafe {
            self.0.attach(owner);
            self.1.attach(owner);
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
    }
}

// SAFETY: every component is traced and receives the same ownership context.
unsafe impl<A: Trace, B: Trace, C: Trace> Trace for (A, B, C) {
    fn external_size(&self) -> usize {
        self.0
            .external_size()
            .saturating_add(self.1.external_size())
            .saturating_add(self.2.external_size())
    }

    fn needs_clear() -> bool {
        A::needs_clear() || B::needs_clear() || C::needs_clear()
    }
    fn root(&mut self) {
        self.0.root();
        self.1.root();
        self.2.root();
    }
    unsafe fn unroot(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: all components belong to the protected tuple owner.
        unsafe {
            self.0.unroot(owner);
            self.1.unroot(owner);
            self.2.unroot(owner);
        }
    }
    unsafe fn attach(&mut self, owner: crate::TraceOwner<'_>) {
        // SAFETY: construction protection covers all components.
        unsafe {
            self.0.attach(owner);
            self.1.attach(owner);
            self.2.attach(owner);
        }
    }
    fn trace(&self, collector: &GarbageCollector) {
        self.0.trace(collector);
        self.1.trace(collector);
        self.2.trace(collector);
    }
}

// SAFETY: strings contain no managed edges and retain ordinary Rust storage.
unsafe impl Trace for String {
    fn has_bounded_drop() -> bool {
        true
    }

    fn has_bounded_cleanup() -> bool {
        true
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _collector: &GarbageCollector) {}
    fn external_size(&self) -> usize {
        self.capacity()
    }
}

// SAFETY: borrowed string data contains no managed edges.
unsafe impl Trace for &str {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace_work(&self) -> usize {
        1
    }
    fn trace(&self, _collector: &GarbageCollector) {}
}

macro_rules! impl_trace_primitive {
    ($($t:ty),*) => {
        $(
            // SAFETY: primitive scalars contain no managed edges.
            unsafe impl Trace for $t {
                fn may_contain_gc_references() -> bool { false }
                fn needs_clear() -> bool { false }
                fn slice_external_size(_: &[Self]) -> usize { 0 }

                fn trace_work(&self) -> usize { 1 }
                fn trace(&self, _collector: &GarbageCollector) {}
            }
        )*
    };
}

impl_trace_primitive!(
    bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

// SAFETY: payload access is locked and fields are never demoted. Ordinary mutex
// mutation lacks managed barriers, so payload fields remain independently owning.
unsafe impl<T> Trace for Mutex<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if !T::may_contain_gc_references() {
            return;
        }
        match self.lock() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        match self.try_lock() {
            Ok(guard) => guard.trace_step(collector, offset, budget),
            Err(std::sync::TryLockError::Poisoned(error)) => {
                error.into_inner().trace_step(collector, offset, budget)
            }
            Err(std::sync::TryLockError::WouldBlock) => TraceProgress::Busy,
        }
    }

    fn external_size(&self) -> usize {
        lock(self).external_size()
    }
}

// SAFETY: payload access is locked and fields remain independently owning.
unsafe impl<T> Trace for RwLock<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if !T::may_contain_gc_references() {
            return;
        }
        match self.read() {
            Ok(guard) => guard.trace(collector),
            Err(poisoned) => poisoned.into_inner().trace(collector),
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        match self.try_read() {
            Ok(guard) => guard.trace_step(collector, offset, budget),
            Err(std::sync::TryLockError::Poisoned(error)) => {
                error.into_inner().trace_step(collector, offset, budget)
            }
            Err(std::sync::TryLockError::WouldBlock) => TraceProgress::Busy,
        }
    }

    fn external_size(&self) -> usize {
        self.read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .external_size()
    }
}

// SAFETY: payload access is locked and fields remain independently owning.
unsafe impl<T> Trace for crate::sync::Mutex<T>
where
    T: Trace,
{
    fn has_bounded_cleanup() -> bool {
        T::has_bounded_cleanup()
    }

    fn needs_clear() -> bool {
        T::needs_clear()
    }

    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        self.get_mut().clear_step(offset, budget)
    }

    fn may_contain_gc_references() -> bool {
        T::may_contain_gc_references()
    }
    fn trace(&self, collector: &GarbageCollector) {
        if T::may_contain_gc_references() {
            self.lock().trace(collector);
        }
    }

    fn trace_step(
        &self,
        collector: &GarbageCollector,
        offset: usize,
        budget: usize,
    ) -> TraceProgress {
        if !T::may_contain_gc_references() {
            return TraceProgress::Complete;
        }
        self.try_lock().map_or(TraceProgress::Busy, |value| {
            value.trace_step(collector, offset, budget)
        })
    }

    fn external_size(&self) -> usize {
        self.lock().external_size()
    }
}
