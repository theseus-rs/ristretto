//! Task-owned roots, required barrier records and optional completed-edge certificates.
use crate::GarbageCollector;
use crate::roots::{Cursor, RootSlots, RootStatistics, Scan};
use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

pub(crate) const MAX_RECORDS: usize = 64;

thread_local! {
    static CURRENT: RefCell<Option<Arc<State>>> = const { RefCell::new(None) };
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn try_lock<T>(mutex: &Mutex<T>) -> Option<MutexGuard<'_, T>> {
    match mutex.try_lock() {
        Ok(guard) => Some(guard),
        Err(std::sync::TryLockError::Poisoned(error)) => Some(error.into_inner()),
        Err(std::sync::TryLockError::WouldBlock) => None,
    }
}

/// A movable execution context. Roots created inside it retain their slots even
/// after the context handle or its original task has gone away. The context holds
/// a weak collector reference; only owning GC handles keep the collector alive.
#[derive(Debug)]
pub struct ExecutionContext(pub(crate) Arc<State>);

impl Clone for ExecutionContext {
    fn clone(&self) -> Self {
        self.0.handles.fetch_add(1, Ordering::Relaxed);
        Self(self.0.clone())
    }
}

impl Drop for ExecutionContext {
    fn drop(&mut self) {
        if self.0.handles.fetch_sub(1, Ordering::AcqRel) == 1
            && let Some(gc) = self.0.collector.upgrade()
        {
            // Escaped roots retain the registry state, but need no unused cache.
            self.0.release_allocations(&gc);
        }
    }
}

impl ExecutionContext {
    /// Run synchronous work with this context's root and barrier storage.
    /// Nested scopes restore the previous context, including during unwinding.
    pub fn with<T>(&self, operation: impl FnOnce() -> T) -> T {
        let previous = CURRENT.with(|current| current.replace(Some(self.0.clone())));
        let _scope = Scope(previous);
        operation()
    }

    /// Install this context only for each poll. Suspension leaves its roots and
    /// records enumerable; resumption may occur on another OS thread. Cancellation
    /// drops the future's owning roots normally, on whichever thread cancels it.
    pub async fn scope<F: Future>(&self, future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        std::future::poll_fn(|cx| self.with(|| future.as_mut().poll(cx))).await
    }

    /// Root storage retained by this context, including escaped owning handles.
    #[must_use]
    pub fn root_statistics(&self) -> RootStatistics {
        self.0.roots.statistics()
    }

    /// Publish all buffered records. At most 64 records are retained per context.
    /// Collection also drains suspended contexts without waiting for their tasks.
    pub fn flush(&self) {
        if let Some(gc) = self.0.collector.upgrade() {
            gc.flush_context(&self.0);
        }
    }
}

struct Scope(Option<Arc<State>>);
impl Drop for Scope {
    fn drop(&mut self) {
        // Drop the outgoing Arc after releasing the TLS borrow: a last-context
        // drop can publish records and must never run inside a RefCell borrow.
        let outgoing = CURRENT.with(|current| current.replace(self.0.take()));
        drop(outgoing);
    }
}

pub(crate) fn current(gc: &GarbageCollector) -> Option<Arc<State>> {
    CURRENT.with(|current| {
        current
            .borrow()
            .as_ref()
            .filter(|context| std::ptr::eq(context.collector.as_ptr(), gc))
            .cloned()
    })
}

/// Borrow only the context's barrier cache without cloning its Arc. The caller
/// must not run hooks, enter scopes or wake futures while this TLS borrow is held.
pub(crate) fn with_barriers<R>(
    gc: &GarbageCollector,
    operation: impl FnOnce(&State) -> R,
) -> Option<R> {
    CURRENT.with(|current| {
        current
            .borrow()
            .as_ref()
            .filter(|state| std::ptr::eq(state.collector.as_ptr(), gc))
            .map(|state| operation(state))
    })
}

/// Borrow the current allocation cache without cloning its Arc. Callers must
/// not invoke Trace hooks, enter scopes, or wake user futures while this TLS
/// borrow is held. Cache range operations and atomic credit reservation comply.
pub(crate) fn with_allocation<R>(
    gc: &GarbageCollector,
    operation: impl FnOnce(&mut crate::allocation::Cache) -> R,
) -> Option<R> {
    CURRENT.with(|current| {
        let current = current.borrow();
        let context = current
            .as_ref()
            .filter(|context| std::ptr::eq(context.collector.as_ptr(), gc))?;
        Some(operation(&mut lock(&context.allocation)))
    })
}

/// Tags are captured under the publication reader. A cycle cannot close with any
/// charged record, and drain verifies each domain's epoch before shading it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Record {
    pub(crate) address: usize,
    pub(crate) epochs: [Option<u64>; 2],
    pub(crate) dirty: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Push {
    Added,
    Coalesced,
    Full,
    Busy,
}

/// Exact epoch-tagged certificates only. The two recent entries preserve the
/// common alternating-edge path; four bounded sets cover larger working sets
/// without scanning every cached record. Eviction only causes a registry check.
#[derive(Debug, Default)]
struct Completed {
    recent: [Option<Record>; 2],
    sets: [[Option<Record>; 4]; 4],
    next: [u8; 4],
    misses: u8,
    bypass: u16,
}

#[expect(
    clippy::indexing_slicing,
    reason = "four sets and four ways use masked indices"
)]
impl Completed {
    fn set(address: usize) -> usize {
        ((address >> 4) ^ (address >> 6) ^ (address >> 12)) & 3
    }

    fn contains(&mut self, record: Record) -> bool {
        if self.recent.contains(&Some(record)) {
            return true;
        }
        if self.bypass != 0 {
            self.bypass -= 1;
            return false;
        }
        if self.sets[Self::set(record.address)].contains(&Some(record)) {
            self.misses = 0;
            return true;
        }
        self.misses += 1;
        if self.misses == 64 {
            // Larger/streaming working sets should pay the old two-entry path
            // while the object registry supplies exact certificates. Probe again
            // after bounded traffic so a later smaller working set can recover.
            self.misses = 0;
            self.bypass = 1024;
        }
        false
    }

    fn insert(&mut self, record: Record) {
        if self.recent[0] == Some(record) {
            return;
        }
        self.recent[1] = self.recent[0];
        self.recent[0] = Some(record);
        if self.bypass != 0 {
            return;
        }
        let set = Self::set(record.address);
        if !self.sets[set].contains(&Some(record)) {
            let way = usize::from(self.next[set]);
            self.sets[set][way] = Some(record);
            self.next[set] = (self.next[set] + 1) & 3;
        }
    }
}

#[derive(Debug)]
struct Buffer {
    records: [Option<Record>; MAX_RECORDS],
    len: usize,
}
impl Default for Buffer {
    fn default() -> Self {
        Self {
            records: [None; MAX_RECORDS],
            len: 0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) id: usize,
    handles: AtomicUsize,
    collector: Weak<GarbageCollector>,
    pub(crate) roots: RootSlots,
    buffer: Mutex<Buffer>,
    // Optional certificates cannot hold the required-record buffer or delay
    // acknowledgement of a drained record. Every access uses try_lock.
    completed: Mutex<Completed>,
    pub(crate) allocation: Mutex<crate::allocation::Cache>,
}

impl State {
    pub(crate) fn release_allocations(&self, gc: &GarbageCollector) {
        let unused = lock(&self.allocation).take_unused();
        Self::release_unused(gc, unused);
    }

    fn try_release_allocations(&self, gc: &GarbageCollector) {
        let Some(mut cache) = try_lock(&self.allocation) else {
            // A mutator still owns this bounded unused capacity. It stays charged;
            // root enumeration and unrelated collection do not depend on release.
            return;
        };
        let unused = cache.take_unused();
        drop(cache);
        Self::release_unused(gc, unused);
    }

    fn release_unused(gc: &GarbageCollector, mut unused: crate::allocation::Cache) {
        let credit = unused.release();
        if credit != 0 {
            gc.cancel_allocation(credit);
        }
    }

    /// Callers hold publication admission or an authorized phase claim. Reserving
    /// the pending count precedes visibility; release follows publication into
    /// the mark/remembered indexes, never mere removal from this buffer.
    pub(crate) fn push(
        self: &Arc<Self>,
        registry: &Registry,
        record: Record,
        limit: usize,
    ) -> Push {
        let Some(mut buffer) = try_lock(&self.buffer) else {
            return Push::Busy;
        };
        // Check at most two records: managed overwrites alternate a deletion
        // edge and dirty owner. Identical epoch AND kind are required; remembering
        // an owner cannot replace shading an edge. Existing reservations protect
        // merged work until drain. No scan grows with buffer size.
        if buffer
            .records
            .get(buffer.len.saturating_sub(2)..buffer.len)
            .is_some_and(|recent| recent.iter().rev().any(|pending| *pending == Some(record)))
        {
            return Push::Coalesced;
        }
        // Completed edge records need no reservation: the collector verified
        // Strong retention (or exemption from young tracing) in every tagged
        // domain before caching them. Dirty
        // owners cannot use this cache because later mutations need notification.
        if self.satisfies(registry, record) {
            return Push::Coalesced;
        }
        if buffer.len >= limit.min(MAX_RECORDS) {
            return Push::Full;
        }
        // Prepare every lock before publishing a pending reservation or changing
        // the buffer. A producer retains phase admission, so contention must
        // fall back to immediate publication instead of waiting here.
        let mut ready = if buffer.len == 0 {
            let Some(ready) = try_lock(&registry.ready) else {
                return Push::Busy;
            };
            Some(ready)
        } else {
            None
        };
        let index = buffer.len;
        let Some(slot) = buffer.records.get_mut(index) else {
            return Push::Full;
        };
        registry.pending.fetch_add(1, Ordering::AcqRel);
        *slot = Some(record);
        buffer.len += 1;
        if let Some(ready) = &mut ready {
            ready.insert(self.id, self.clone());
        }
        Push::Added
    }

    pub(crate) fn satisfies(&self, registry: &Registry, record: Record) -> bool {
        !record.dirty
            && !registry.completed_disabled.load(Ordering::Acquire)
            && try_lock(&self.completed).is_some_and(|mut cache| cache.contains(record))
    }

    /// Only after Strong retention or exemption from young tracing in all tagged
    /// domains, while the
    /// popped record still owns its pending reservation. Never cache dirty work.
    pub(crate) fn completed(&self, record: Record) {
        debug_assert!(!record.dirty && record.epochs.iter().any(Option::is_some));
        if let Some(mut cache) = try_lock(&self.completed) {
            cache.insert(record);
        }
    }

    pub(crate) fn pop(&self, registry: &Registry) -> Option<Record> {
        let mut buffer = lock(&self.buffer);
        buffer.len = buffer.len.checked_sub(1)?;
        let index = buffer.len;
        let record = buffer.records.get_mut(index)?.take();
        if buffer.len == 0 {
            lock(&registry.ready).remove(&self.id);
        }
        record
    }

    /// A scheduling hint, with no transfer of the record's pending reservation.
    /// A consumer must revalidate the record when it actually claims the slot.
    pub(crate) fn try_peek(&self) -> Option<Record> {
        let buffer = try_lock(&self.buffer)?;
        buffer
            .records
            .get(buffer.len.checked_sub(1)?)?
            .as_ref()
            .copied()
    }

    /// Claim one required record without waiting or dropping it on contention.
    /// The ready entry is removed only after both locks needed by the last pop
    /// are held. Pending ownership transfers to the caller, not to an empty buffer.
    pub(crate) fn try_pop(&self, registry: &Registry) -> Option<Record> {
        self.try_pop_inner(registry, None)
    }

    /// Preparation may release this buffer while acquiring other try-locks.
    /// Claim only a matching record; changed top entries retain their ownership.
    pub(crate) fn try_pop_matching(&self, registry: &Registry, expected: Record) -> Option<Record> {
        self.try_pop_inner(registry, Some(expected))
    }

    fn try_pop_inner(&self, registry: &Registry, expected: Option<Record>) -> Option<Record> {
        let mut buffer = try_lock(&self.buffer)?;
        let index = buffer.len.checked_sub(1)?;
        if expected.is_some_and(|expected| buffer.records.get(index) != Some(&Some(expected))) {
            return None;
        }
        let mut ready = if index == 0 {
            Some(try_lock(&registry.ready)?)
        } else {
            None
        };
        let record = buffer.records.get_mut(index)?.take()?;
        buffer.len = index;
        let owner = ready.as_mut().and_then(|ready| ready.remove(&self.id));
        drop(ready);
        drop(buffer);
        drop(owner);
        Some(record)
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if let Some(gc) = self.collector.upgrade() {
            // Nonempty buffers are owned by Registry::ready until drained. Thus
            // the last Arc can disappear only after roots and records are gone.
            // Do not acquire publication here: a collector may drop its last Arc
            // while holding the writer.
            self.roots.trim_empty();
            self.release_allocations(&gc);
            gc.contexts.remove(
                self.id,
                self.roots.statistics(),
                self.roots.resident_counter(),
            );
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RootCursor {
    id: usize,
    slots: Cursor,
    current: Option<Arc<State>>,
}

pub(crate) enum RootScan {
    Root(usize),
    Advance,
    Done,
    Cleanup(ContextCleanup),
}

/// Execute only after releasing publication, the cycle cursor and coordinator.
/// Even dropping the last context owner can lock registries and release ranges.
pub(crate) struct ContextCleanup {
    state: Arc<State>,
    release_allocations: bool,
}

impl ContextCleanup {
    pub(crate) fn run(self, gc: &GarbageCollector) {
        // This maintenance runs after releasing the generation and publication guards.
        self.state.roots.repair_step();
        if self.release_allocations {
            self.state.try_release_allocations(gc);
        }
        drop(self);
    }
}

#[derive(Debug, Default)]
struct RetiredRootStatistics {
    totals: RootStatistics,
    // Only scans retaining a page can outlive their context. Weak counters do
    // not retain pages, and completed counters are pruned at removal/statistics.
    residents: Vec<Weak<AtomicUsize>>,
}

#[derive(Debug, Default)]
pub(crate) struct Registry {
    states: Mutex<BTreeMap<usize, Weak<State>>>,
    published: ArcSwap<BTreeMap<usize, Weak<State>>>,
    ready: Mutex<BTreeMap<usize, Arc<State>>>,
    next: AtomicUsize,
    drain_cursor: AtomicUsize,
    retired_roots: Mutex<RetiredRootStatistics>,
    pub(crate) pending: AtomicUsize,
    // Once an epoch would wrap, cached epoch equality is no longer sufficient.
    // Disable this optional cache before resetting any marks; no unbounded walk.
    pub(crate) completed_disabled: AtomicBool,
}

impl Registry {
    #[expect(clippy::expect_used, reason = "context identities must never wrap")]
    pub(crate) fn attach(&self, gc: &GarbageCollector) -> ExecutionContext {
        let id = self
            .next
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .expect("execution context identity exhausted")
            + 1;
        let state = Arc::new(State {
            id,
            handles: AtomicUsize::new(1),
            collector: gc.weak(),
            roots: RootSlots::default(),
            buffer: Mutex::new(Buffer::default()),
            completed: Mutex::new(Completed::default()),
            allocation: Mutex::new(crate::allocation::Cache::default()),
        });
        let mut states = lock(&self.states);
        states.insert(id, Arc::downgrade(&state));
        let previous = self.published.swap(Arc::new(states.clone()));
        drop(states);
        drop(previous);
        ExecutionContext(state)
    }

    fn next(&self, after: usize) -> Option<(usize, Option<Arc<State>>)> {
        lock(&self.states)
            .range((Excluded(after), Unbounded))
            .next()
            .map(|(&id, state)| (id, state.upgrade()))
    }

    pub(crate) fn remove(&self, id: usize, mut roots: RootStatistics, resident: Weak<AtomicUsize>) {
        let mut states = lock(&self.states);
        states.remove(&id);
        let previous = self.published.swap(Arc::new(states.clone()));
        drop(states);
        drop(previous);
        let mut retired = lock(&self.retired_roots);
        retired
            .residents
            .retain(|counter| counter.strong_count() != 0);
        if roots.resident_pages != 0 {
            retired.residents.push(resident);
        }
        roots.resident_pages = 0;
        roots.resident_page_bytes = 0;
        retired.totals.merge(roots);
    }

    /// Pin one context for its traversal, avoiding registry lookup/owner release
    /// on every root. Setup and final owner release are caller-executed work.
    pub(crate) fn scan(&self, cursor: &mut RootCursor) -> RootScan {
        if let Some(state) = &cursor.current {
            match state.roots.scan(&mut cursor.slots) {
                Scan::Root(address) => return RootScan::Root(address),
                Scan::Advance => return RootScan::Advance,
                Scan::Done => {}
            }
            if let Some(state) = cursor.current.take() {
                return RootScan::Cleanup(ContextCleanup {
                    state,
                    release_allocations: false,
                });
            }
        }
        let states = self.published.load();
        let Some((&id, state)) = states.range((Excluded(cursor.id), Unbounded)).next() else {
            return RootScan::Done;
        };
        let state = state.upgrade();
        drop(states);
        cursor.id = id;
        cursor.slots = Cursor::default();
        if let Some(state) = state {
            cursor.current = Some(state.clone());
            RootScan::Cleanup(ContextCleanup {
                state,
                release_allocations: true,
            })
        } else {
            RootScan::Advance
        }
    }

    /// A scheduling hint only: no entry or a busy registry can return `None`.
    /// Pending reservations, not this hint, determine whether barriers are closed.
    pub(crate) fn try_ready(&self) -> Option<Arc<State>> {
        let ready = try_lock(&self.ready)?;
        let cursor = self.drain_cursor.load(Ordering::Relaxed);
        let (&id, state) = ready
            .range((Excluded(cursor), Unbounded))
            .next()
            .or_else(|| ready.first_key_value())?;
        self.drain_cursor.store(id, Ordering::Relaxed);
        Some(state.clone())
    }

    /// Cold inspection/compatibility paths. Never invoke callbacks under the
    /// registry lock: dropping its last upgraded Arc can unregister the context.
    pub(crate) fn for_each(&self, mut action: impl FnMut(&Arc<State>)) {
        let mut cursor = 0;
        while let Some((id, state)) = self.next(cursor) {
            cursor = id;
            if let Some(state) = state {
                action(&state);
            }
        }
    }

    pub(crate) fn statistics(&self) -> (usize, RootStatistics) {
        let mut count = 0;
        let mut roots = {
            let mut retired = lock(&self.retired_roots);
            let mut roots = retired.totals;
            retired.residents.retain(|counter| {
                let Some(counter) = counter.upgrade() else {
                    return false;
                };
                let pages = counter.load(Ordering::Relaxed);
                roots.resident_pages += pages;
                roots.resident_page_bytes += RootSlots::resident_bytes(pages);
                pages != 0
            });
            roots
        };
        self.for_each(|state| {
            count += 1;
            roots.merge(state.roots.statistics());
        });
        (count, roots)
    }
}

#[cfg(test)]
mod coalescing_tests {
    use super::*;

    #[test]
    fn busy_barrier_insertion_preserves_pending_and_context_ownership() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let registry = &gc.contexts;
        let record = Record {
            address: 0x1000,
            epochs: [Some(4), None],
            dirty: false,
        };
        let owned = Arc::strong_count(&context.0);
        let admission = gc.allocation_guard();
        {
            let buffer = lock(&context.0.buffer);
            assert_eq!(context.0.push(registry, record, 2), Push::Busy);
            assert_eq!(buffer.len, 0);
        }
        {
            let ready = lock(&registry.ready);
            assert_eq!(context.0.push(registry, record, 2), Push::Busy);
            assert!(!ready.contains_key(&context.0.id));
            assert_eq!(lock(&context.0.buffer).len, 0);
        }
        assert_eq!(registry.pending.load(Ordering::Acquire), 0);
        assert_eq!(Arc::strong_count(&context.0), owned);
        assert_eq!(context.0.push(registry, record, 2), Push::Added);
        let other = Record {
            address: 0x2000,
            ..record
        };
        {
            let ready = lock(&registry.ready);
            assert_eq!(context.0.push(registry, record, 2), Push::Coalesced);
            assert_eq!(context.0.push(registry, other, 2), Push::Added);
            assert!(ready.contains_key(&context.0.id));
        }
        assert_eq!(registry.pending.load(Ordering::Acquire), 2);
        assert_eq!(Arc::strong_count(&context.0), owned + 1);
        drop(admission);
        assert_eq!(context.0.try_pop(registry), Some(other));
        assert_eq!(context.0.try_pop(registry), Some(record));
        assert_eq!(registry.pending.load(Ordering::Acquire), 2);
        assert_eq!(Arc::strong_count(&context.0), owned);
        registry.pending.fetch_sub(2, Ordering::AcqRel);
    }

    #[test]
    fn prepared_claim_revalidates_address_epoch_and_kind_without_losing_records() {
        let base = Record {
            address: 0x1000,
            epochs: [Some(4), None],
            dirty: false,
        };
        for second in [
            Record {
                address: 0x2000,
                ..base
            },
            Record {
                epochs: [Some(5), None],
                ..base
            },
            Record {
                dirty: true,
                ..base
            },
        ] {
            let gc = GarbageCollector::new();
            let context = gc.register_context();
            let registry = &gc.contexts;
            let publication = gc.allocation_guard();
            assert_eq!(context.0.push(registry, base, 2), Push::Added);
            let prepared = context.0.try_peek().expect("buffered record");
            assert_eq!(context.0.push(registry, second, 2), Push::Added);
            drop(publication);
            assert_eq!(context.0.try_pop_matching(registry, prepared), None);
            assert_eq!(context.0.try_peek(), Some(second));
            assert_eq!(registry.pending.load(Ordering::Acquire), 2);
            assert_eq!(context.0.try_pop_matching(registry, second), Some(second));
            assert_eq!(context.0.try_pop_matching(registry, base), Some(base));
            assert_eq!(context.0.try_peek(), None);
            assert_eq!(registry.pending.load(Ordering::Acquire), 2);
            registry.pending.fetch_sub(2, Ordering::AcqRel);
        }
    }

    #[test]
    fn an_identical_replacement_transfers_its_own_pending_reservation() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let registry = &gc.contexts;
        let record = Record {
            address: 0x1000,
            epochs: [Some(4), None],
            dirty: false,
        };
        let publication = gc.allocation_guard();
        assert_eq!(context.0.push(registry, record, 1), Push::Added);
        let prepared = context.0.try_peek().expect("original record");
        assert_eq!(context.0.try_pop(registry), Some(record));
        assert_eq!(context.0.push(registry, record, 1), Push::Added);
        drop(publication);
        assert_eq!(context.0.try_pop_matching(registry, prepared), Some(record));
        assert_eq!(registry.pending.load(Ordering::Acquire), 2);
        assert!(registry.try_ready().is_none());
        registry.pending.fetch_sub(2, Ordering::AcqRel);
    }

    #[test]
    fn a_busy_last_record_claim_keeps_the_record_owner_and_pending_reservation() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let registry = &gc.contexts;
        let record = Record {
            address: 0x1000,
            epochs: [Some(4), None],
            dirty: false,
        };
        let publication = gc.allocation_guard();
        assert_eq!(context.0.push(registry, record, 1), Push::Added);
        drop(publication);
        let owned = Arc::strong_count(&context.0);
        {
            let _buffer = lock(&context.0.buffer);
            assert!(context.0.try_pop(registry).is_none());
        }
        {
            let ready = lock(&registry.ready);
            assert!(context.0.try_pop(registry).is_none());
            assert!(ready.contains_key(&context.0.id));
            assert_eq!(lock(&context.0.buffer).len, 1);
            assert_eq!(registry.pending.load(Ordering::Acquire), 1);
            assert_eq!(Arc::strong_count(&context.0), owned);
        }
        assert_eq!(context.0.try_pop(registry), Some(record));
        assert!(registry.try_ready().is_none());
        assert_eq!(registry.pending.load(Ordering::Acquire), 1);
        assert_eq!(Arc::strong_count(&context.0), owned - 1);
        // Simulate application acknowledgment; a popped record still owns it.
        registry.pending.fetch_sub(1, Ordering::AcqRel);
    }

    #[test]
    fn registry_contention_does_not_block_snapshot_or_current_context() {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let root = context.with(|| crate::Gc::new(&gc, 42_u64));
        let mut cursor = RootCursor::default();
        let registry = lock(&gc.contexts.states);
        let RootScan::Cleanup(cleanup) = gc.contexts.scan(&mut cursor) else {
            panic!("context preparation missing");
        };
        cleanup.run(&gc);
        assert_eq!(cursor.id, context.0.id);
        let mut observed = None;
        for _ in 0..100 {
            if let RootScan::Root(address) = gc.contexts.scan(&mut cursor) {
                observed = Some(address);
                break;
            }
        }
        assert_eq!(observed, Some(root.as_ptr() as usize));
        drop(registry);
        drop((root, context, cursor));
        assert_eq!(gc.contexts.statistics().0, 0);
    }

    #[test]
    fn a_retired_context_counts_scan_pinned_pages_until_the_cursor_is_dropped() {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let root = context.with(|| crate::Gc::new(&gc, 42_u64));
        let mut cursor = RootCursor::default();
        loop {
            match gc.contexts.scan(&mut cursor) {
                RootScan::Root(_) => break,
                RootScan::Cleanup(cleanup) => cleanup.run(&gc),
                RootScan::Advance => {}
                RootScan::Done => panic!("registered root missing"),
            }
        }
        drop(root);
        drop(context);
        // Final context-owner cleanup and page-cursor release are separate.
        // The collector normally releases the context after finishing its slots.
        drop(cursor.current.take());
        assert_eq!(gc.contexts.statistics().0, 0);
        assert_eq!(gc.root_statistics().pages, 0);
        assert_eq!(gc.root_statistics().resident_pages, 1);
        drop(cursor);
        assert_eq!(gc.root_statistics().resident_pages, 0);
        assert_eq!(gc.root_statistics().resident_page_bytes, 0);
    }

    #[test]
    fn context_cursor_retains_only_its_current_owner_until_enumeration_releases_it() {
        let gc = GarbageCollector::with_config(
            crate::Configuration::builder()
                .root_storage(crate::RootStorage::Contexts)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let context = gc.register_context();
        let owner = Arc::downgrade(&context.0);
        let root = context.with(|| crate::Gc::new(&gc, 42_u64));
        let mut cursor = RootCursor::default();
        loop {
            match gc.contexts.scan(&mut cursor) {
                RootScan::Root(_) => break,
                RootScan::Cleanup(cleanup) => cleanup.run(&gc),
                RootScan::Advance => {}
                RootScan::Done => panic!("registered root missing"),
            }
        }
        drop((root, context));
        assert_eq!(owner.strong_count(), 1);
        assert_eq!(gc.contexts.statistics().0, 1);
        assert_eq!(gc.root_statistics().slot_roots, 0);
        assert_eq!(gc.root_statistics().pages, 1);
        for _ in 0..100 {
            match gc.contexts.scan(&mut cursor) {
                RootScan::Cleanup(cleanup) => cleanup.run(&gc),
                RootScan::Advance => {}
                RootScan::Done => break,
                RootScan::Root(_) => panic!("unexpected context root work"),
            }
        }
        assert!(cursor.current.is_none());
        assert_eq!(owner.strong_count(), 0);
        assert_eq!(gc.contexts.statistics().0, 0);
        assert_eq!(gc.root_statistics().resident_pages, 0);
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "forced cache-contention assertions"
    )]
    fn held_optional_cache_does_not_block_required_records_or_full_collection() -> crate::Result<()>
    {
        for concurrent in [false, true] {
            let gc = GarbageCollector::with_config(
                crate::Configuration::builder()
                    .concurrent_publication(concurrent)
                    .allocation_threshold(usize::MAX)
                    .promotion_age(1)
                    .barrier_buffer_size(32)
                    .build(),
            );
            let context = gc.register_context();
            let root = crate::Gc::try_new(&gc, 42_u64)?;
            gc.collect_full()?; // old targets are exempt from any young snapshot
            context.with(|| -> crate::Result<()> {
                let cache = lock(&context.0.completed);
                let record = Record {
                    address: root.as_ptr() as usize,
                    epochs: [Some(4), None],
                    dirty: false,
                };
                // A stale young record is safe to drain; its old target also
                // exercises optional certificate insertion before acknowledgement.
                let publication = gc.allocation_guard();
                assert_eq!(context.0.push(&gc.contexts, record, 32), Push::Added);
                drop(publication);
                context.flush();
                assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
                for _ in 0..3 {
                    drop(crate::Gc::try_new(&gc, 99_u64)?);
                    gc.collect_full()?;
                    assert_eq!(gc.statistics()?.live_objects, 1);
                    assert_eq!(**root, 42);
                }
                drop(cache);
                Ok(())
            })?;
            drop((root, context));
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn completed_working_set_preserves_epochs_kind_and_rollover_disable() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let records: Vec<_> = (0..16)
            .map(|index| Record {
                address: 0x1000 + index * 64,
                epochs: [Some(4), Some(8)],
                dirty: false,
            })
            .collect();
        for &record in &records {
            context.0.completed(record);
        }
        for &record in &records {
            assert!(context.0.satisfies(&gc.contexts, record));
            for changed in [
                Record {
                    dirty: true,
                    ..record
                },
                Record {
                    epochs: [Some(5), Some(8)],
                    ..record
                },
                Record {
                    epochs: [Some(4), Some(9)],
                    ..record
                },
                Record {
                    epochs: [None, Some(8)],
                    ..record
                },
            ] {
                assert!(!context.0.satisfies(&gc.contexts, changed));
            }
        }
        gc.contexts
            .completed_disabled
            .store(true, Ordering::Release);
        for &record in &records {
            assert!(!context.0.satisfies(&gc.contexts, record));
            assert_eq!(context.0.push(&gc.contexts, record, 32), Push::Added);
        }
        for _ in &records {
            assert!(context.0.pop(&gc.contexts).is_some());
            gc.contexts.pending.fetch_sub(1, Ordering::AcqRel);
        }
        assert_eq!(gc.contexts.pending.load(Ordering::Acquire), 0);
    }

    #[test]
    fn completed_cache_eviction_never_certifies_an_unseen_address() {
        let mut completed = Completed::default();
        for index in 0..512 {
            completed.insert(Record {
                address: 0x1000 + index * 64,
                epochs: [Some(4), Some(8)],
                dirty: false,
            });
        }
        let retained = (0..512)
            .filter(|index| {
                completed.contains(Record {
                    address: 0x1000 + index * 64,
                    epochs: [Some(4), Some(8)],
                    dirty: false,
                })
            })
            .count();
        assert!((2..=18).contains(&retained));
        for index in 0..512 {
            assert!(!completed.contains(Record {
                address: 0x1001 + index * 64,
                epochs: [Some(4), Some(8)],
                dirty: false,
            }));
        }
    }

    #[test]
    fn completed_cache_recovers_after_a_stream_becomes_a_small_working_set() {
        let mut completed = Completed::default();
        let record = |index: usize| Record {
            address: 0x1000 + index * 64,
            epochs: [Some(4), Some(8)],
            dirty: false,
        };
        for index in 0..64 {
            let next = record(index);
            assert!(!completed.contains(next));
            completed.insert(next);
        }
        assert_ne!(completed.bypass, 0);
        let mut hits = 0;
        for index in 0..2048 {
            let next = record(index & 3);
            if completed.contains(next) {
                hits += 1;
            } else {
                completed.insert(next);
            }
        }
        assert!(
            hits > 1000,
            "extended cache did not recover after bounded bypass"
        );
        for index in 0..4 {
            assert!(completed.contains(record(index)));
        }
    }

    #[test]
    fn alternating_owner_and_edge_keep_distinct_reservations() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let registry = &gc.contexts;
        let edge = Record {
            address: 42,
            epochs: [Some(4), Some(8)],
            dirty: false,
        };
        let owner = Record {
            dirty: true,
            ..edge
        };
        assert_eq!(context.0.push(registry, edge, 2), Push::Added);
        assert_eq!(context.0.push(registry, owner, 2), Push::Added);
        for _ in 0..100 {
            assert_eq!(context.0.push(registry, edge, 2), Push::Coalesced);
            assert_eq!(context.0.push(registry, owner, 2), Push::Coalesced);
        }
        assert_eq!(registry.pending.load(Ordering::Acquire), 2);
        assert_eq!(context.0.pop(registry), Some(owner));
        assert_eq!(context.0.pop(registry), Some(edge));
        registry.pending.fetch_sub(2, Ordering::AcqRel);
    }

    #[test]
    fn coalescing_preserves_kind_epoch_and_in_flight_reservations() {
        let gc = GarbageCollector::new();
        let context = gc.register_context();
        let registry = &gc.contexts;
        let record = Record {
            address: 42,
            epochs: [Some(4), None],
            dirty: false,
        };
        assert_eq!(context.0.push(registry, record, 1), Push::Added);
        assert_eq!(context.0.push(registry, record, 1), Push::Coalesced);
        assert_eq!(
            context.0.push(
                registry,
                Record {
                    dirty: true,
                    ..record
                },
                1
            ),
            Push::Full
        );
        assert_eq!(
            context.0.push(
                registry,
                Record {
                    epochs: [Some(8), None],
                    ..record
                },
                1
            ),
            Push::Full
        );
        assert_eq!(context.0.pop(registry), Some(record));
        // Popped work still owns its reservation. A subsequent duplicate must
        // acquire another reservation, since the first flusher may be suspended.
        assert_eq!(context.0.push(registry, record, 1), Push::Added);
        assert_eq!(registry.pending.load(Ordering::Acquire), 2);
        assert_eq!(context.0.pop(registry), Some(record));
        registry.pending.fetch_sub(2, Ordering::AcqRel);
    }
}

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
#[path = "context_progress_tests.rs"]
mod progress_tests;
