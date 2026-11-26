//! Cancellation-safe collection waiters, independent of a particular executor.
use crate::{Error, GarbageCollector, Result};
use portable_atomic::AtomicU64;
use std::alloc::Layout;
use std::collections::BTreeMap;
use std::future::Future;
use std::ops::Bound::{Excluded, Unbounded};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Waker};

fn lock<T>(value: &Mutex<T>) -> MutexGuard<'_, T> {
    value
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

type WaiterId = u64;

#[derive(Debug, Clone)]
struct GrantWatch {
    request: crate::retirement_grants::Request,
    settlement_only: bool,
}

impl GrantWatch {
    fn ready(&self) -> bool {
        self.request.paid() || (self.settlement_only && self.request.waiting())
    }
}

#[derive(Debug)]
struct Waiter {
    grant: Option<GrantWatch>,
    bytes: Option<usize>,
    waker: Waker,
    space_seen: Option<Arc<AtomicBool>>,
}

#[derive(Debug, Default)]
struct Waiters {
    next_id: WaiterId,
    // Several allocation requests can share a cycle ticket.
    entries: BTreeMap<(u64, WaiterId), Waiter>,
    // Other notifiers can take the next wake while a callback is running.
    ready: BTreeMap<(u64, WaiterId), Waiter>,
    delivering: Vec<std::thread::ThreadId>,
    references: BTreeMap<usize, Waker>,
    ready_references: BTreeMap<usize, Waker>,
    reference_turn: bool,
    space: BTreeMap<(usize, WaiterId), u64>,
    physical: BTreeMap<WaiterId, (u64, Layout)>,
    physical_cursor: WaiterId,
}

// Registrations between two refunds can share one maximum: no earlier offer
// exists in an open group. Each request still compares its exact byte count.
#[derive(Debug)]
struct RefundGroup {
    minimum: AtomicUsize,
    maximum: AtomicUsize,
}

#[derive(Debug)]
struct Observations {
    current: Arc<RefundGroup>,
    earlier: arc_swap::ArcSwapOption<Vec<std::sync::Weak<RefundGroup>>>,
}

impl Waiters {
    fn take_fitting_allocation(
        &mut self,
        gc: &GarbageCollector,
        available: usize,
    ) -> Option<((u64, WaiterId), Waiter, usize)> {
        let (&(bytes, id), &cycle) = self.space.first_key_value()?;
        if bytes > available {
            return None;
        }
        self.space.remove(&(bytes, id));
        let entry = self.entries.remove(&(cycle, id))?;
        if let Some(seen) = &entry.space_seen {
            seen.store(true, Ordering::Release);
        }
        if gc.diagnostics_enabled() {
            gc.diagnostics
                .allocation_early_wakeups
                .fetch_add(1, Ordering::Relaxed);
        }
        Some(((cycle, id), entry, bytes))
    }

    fn queue_ready_grants(&mut self) {
        let eligible = self.entries.extract_if(.., |_, waiter| {
            waiter.grant.as_ref().is_some_and(GrantWatch::ready)
        });
        for (key, waiter) in eligible {
            if let Some(bytes) = waiter.bytes {
                self.space.remove(&(bytes, key.1));
            }
            self.ready.insert(key, waiter);
        }
    }

    fn queue_fitting_allocations(&mut self, gc: &GarbageCollector, mut remaining: usize) {
        while let Some((key, entry, bytes)) = self.take_fitting_allocation(gc, remaining) {
            remaining -= bytes;
            self.ready.insert(key, entry);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Recovery {
    waiters: Mutex<Waiters>,
    observations: arc_swap::ArcSwapOption<Observations>,
    notified: AtomicU64,
    waiting: AtomicUsize,
    wake_pending: AtomicBool,
    pub(crate) grants_pending: AtomicBool,
    physical_waiting: AtomicUsize,
    minimum_bytes: AtomicUsize,
    observing: AtomicUsize,
    workers: AtomicUsize,
    failed: AtomicBool,
}

// A synchronous waker may immediately reenter progress for this collector.
// Its outer loop continues delivery on return. Other helpers can deliver
// concurrently, including while this thread is parked inside user code.
struct DeliveryGuard<'a> {
    recovery: &'a Recovery,
    thread: std::thread::ThreadId,
}

impl<'a> DeliveryGuard<'a> {
    fn enter(recovery: &'a Recovery) -> Option<Self> {
        let thread = std::thread::current().id();
        let mut waiters = lock(&recovery.waiters);
        if waiters.delivering.contains(&thread) {
            return None;
        }
        waiters.delivering.push(thread);
        Some(Self { recovery, thread })
    }
}

impl Drop for DeliveryGuard<'_> {
    fn drop(&mut self) {
        lock(&self.recovery.waiters)
            .delivering
            .retain(|thread| *thread != self.thread);
    }
}

/// Native executor delivery is independent of user tracing callbacks. The
/// dispatch thread never holds a collector owner while waiting for a refund.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
pub(crate) struct RefundNotifier {
    thread: std::thread::Thread,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
    event: Arc<RefundEvent>,
}

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Default)]
struct RefundEvent {
    pending: AtomicBool,
    closed: AtomicBool,
}

#[cfg(not(target_family = "wasm"))]
impl RefundNotifier {
    pub(crate) fn spawn(gc: &GarbageCollector) -> Result<Self> {
        let event = Arc::new(RefundEvent::default());
        let wake = event.clone();
        let weak = gc.weak();
        let handle = std::thread::Builder::new()
            .name("gc-refund".into())
            .spawn(move || {
                while !wake.closed.load(Ordering::Acquire) {
                    if wake.pending.swap(false, Ordering::AcqRel) {
                        Self::dispatch(&weak, &wake);
                    } else {
                        std::thread::park();
                    }
                }
            })
            .map_err(|error| Error::SyncError(error.to_string()))?;
        Ok(Self {
            thread: handle.thread().clone(),
            handle: Mutex::new(Some(handle)),
            event,
        })
    }

    fn dispatch(weak: &std::sync::Weak<GarbageCollector>, event: &RefundEvent) {
        // A wake offers a retry. An executor can discard or delay that retry,
        // so each independently fitting sleeper gets an opportunity.
        while !event.closed.load(Ordering::Acquire) {
            let Some(gc) = weak.upgrade() else { break };
            if !gc.recovery.has_fitting_waiter(gc.allocation_headroom()) {
                break;
            }
            let (send, receive) = std::sync::mpsc::sync_channel::<Waker>(1);
            // Secure an independent delivery context before taking the waiter.
            // A parked executor callback holds neither the collector nor its
            // notification thread. Its siblings can be dispatched immediately.
            if std::thread::Builder::new()
                .name("gc-allocation-wake".into())
                .spawn(move || {
                    if let Ok(waker) = receive.recv() {
                        waker.wake();
                    }
                })
                .is_err()
            {
                drop(gc);
                event.pending.store(true, Ordering::Release);
                std::thread::park_timeout(std::time::Duration::from_millis(1));
                break;
            }
            let waker = gc.recovery.take_refund_waker(&gc);
            drop(gc);
            let Some(waker) = waker else { break };
            let _ = send.send(waker);
        }
    }

    pub(crate) fn join(&self) {
        if let Some(handle) = lock(&self.handle).take()
            && handle.thread().id() != std::thread::current().id()
        {
            let _ = handle.join();
        }
    }

    pub(crate) fn signal(&self) {
        self.event.pending.store(true, Ordering::Release);
        self.thread.unpark();
    }

    pub(crate) fn close(&self) {
        self.event.closed.store(true, Ordering::Release);
        self.signal();
    }
}

impl GarbageCollector {
    pub(crate) fn observe_space(&self, bytes: usize) -> Result<SpaceObservation<'_>> {
        let mut observation = self.recovery.register_observation(bytes)?;
        self.refresh_allocation_interest(&mut observation.context_interest)?;
        // Publish the threshold into the budget's modification order before
        // requesting a cycle. A later refund sees this registration, and
        // credit already available here counts even if stolen before polling.
        if bytes <= self.headroom_after_registration() {
            observation.offered_at_registration = true;
        }
        Ok(observation)
    }
}

impl Recovery {
    #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
    pub(crate) fn with_parked_waiter_index<R>(&self, operation: impl FnOnce() -> R) -> R {
        let _waiters = lock(&self.waiters);
        operation()
    }

    pub(crate) fn registered_workers(&self) -> usize {
        self.workers.load(Ordering::Acquire)
    }

    /// Install before requesting a cycle, including before the first future poll.
    fn register_observation(&self, bytes: usize) -> Result<SpaceObservation<'_>> {
        self.observing.fetch_add(1, Ordering::AcqRel);
        let interest = ObservationInterest(self);
        let mut previous = self.observations.load();
        let group = loop {
            #[cfg(all(test, not(target_family = "wasm")))]
            observer_progress_tests::before_publication();
            if let Some(table) = previous.as_ref() {
                let group = &table.current;
                if bytes < group.minimum.load(Ordering::Relaxed) {
                    group.minimum.fetch_min(bytes, Ordering::Relaxed);
                }
                if group.maximum.load(Ordering::Acquire) == 0 {
                    break group.clone();
                }
            }
            let group = Arc::new(RefundGroup {
                minimum: AtomicUsize::new(bytes),
                maximum: AtomicUsize::new(0),
            });
            let mut earlier = Vec::new();
            if let Some(table) = previous.as_ref() {
                let history = table.earlier.load();
                let live = history
                    .iter()
                    .flat_map(|list| list.iter())
                    .filter(|group| group.strong_count() != 0);
                earlier
                    .try_reserve_exact(
                        live.clone()
                            .count()
                            .checked_add(1)
                            .ok_or(Error::OutOfMemory)?,
                    )
                    .map_err(|_| Error::OutOfMemory)?;
                earlier.extend(live.cloned());
                earlier.push(Arc::downgrade(&table.current));
            }
            let updated = Arc::new(Observations {
                current: group.clone(),
                earlier: arc_swap::ArcSwapOption::from(
                    (!earlier.is_empty()).then(|| Arc::new(earlier)),
                ),
            });
            let actual = self
                .observations
                .compare_and_swap(&*previous, Some(updated));
            if actual.as_ref().map(Arc::as_ptr) == previous.as_ref().map(Arc::as_ptr) {
                break group;
            }
            previous = actual;
        };
        Ok(SpaceObservation {
            _interest: interest,
            group,
            bytes,
            offered_at_registration: false,
            seen: None,
            context_interest: None,
        })
    }

    /// Capture one set of registered groups. A registrar that arrives after
    /// sealing starts a new group. Prior requests retain their own maxima
    /// through cancellation, ticket changes, and concurrent directory updates.
    pub(crate) fn record_headroom(&self, available: usize) {
        if available == 0 || self.observing.load(Ordering::Acquire) == 0 {
            return;
        }
        let snapshot = self.observations.load();
        if let Some(table) = snapshot.as_ref() {
            // The first fitting positive maximum closes this group and
            // records the opportunity in the same atomic operation.
            if available >= table.current.minimum.load(Ordering::Acquire) {
                table
                    .current
                    .maximum
                    .fetch_max(available, Ordering::Release);
            }
            #[cfg(all(test, not(target_family = "wasm")))]
            observer_progress_tests::after_refund_snapshot();
            if let Some(earlier) = table.earlier.load().as_ref() {
                for old in earlier.iter().filter_map(std::sync::Weak::upgrade) {
                    if available > old.maximum.load(Ordering::Relaxed) {
                        old.maximum.fetch_max(available, Ordering::Release);
                    }
                }
            }
        }
    }

    fn register(
        &self,
        ticket: u64,
        id: Option<WaiterId>,
        waiter: Waiter,
        physical: Option<Layout>,
    ) -> Result<WaiterId> {
        let (id, previous) = {
            let mut waiters = lock(&self.waiters);
            let id = if let Some(id) = id {
                id
            } else {
                waiters.next_id = waiters
                    .next_id
                    .checked_add(1)
                    .filter(|id| *id != WaiterId::MAX)
                    .ok_or(Error::OutOfMemory)?;
                waiters.next_id
            };
            if let Some(bytes) = waiter.bytes {
                waiters.space.insert((bytes, id), ticket);
            }
            if let Some(layout) = physical {
                waiters.physical.insert(id, (ticket, layout));
            }
            let queued = waiters.ready.remove(&(ticket, id));
            let previous = waiters.entries.insert((ticket, id), waiter);
            self.update_counts(&waiters);
            (id, (previous, queued))
        };
        // Waker vtables can run arbitrary code, including on drop.
        drop(previous);
        Ok(id)
    }

    fn remove(&self, ticket: u64, id: Option<WaiterId>) {
        let Some(id) = id else { return };
        let previous = {
            let mut waiters = lock(&self.waiters);
            let queued = waiters.ready.remove(&(ticket, id));
            let previous = waiters.entries.remove(&(ticket, id));
            waiters.physical.remove(&id);
            if let Some(bytes) = previous.as_ref().and_then(|entry| entry.bytes) {
                waiters.space.remove(&(bytes, id));
            }
            self.update_counts(&waiters);
            (previous, queued)
        };
        drop(previous);
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn has_fitting_waiter(&self, available: usize) -> bool {
        self.waiter_count() != 0
            && (self.grants_pending.load(Ordering::Acquire)
                || self.wake_pending.load(Ordering::Acquire)
                || available >= self.minimum_bytes.load(Ordering::Acquire))
    }

    pub(crate) fn waiter_count(&self) -> usize {
        self.waiting.load(Ordering::Acquire)
    }

    fn update_counts(&self, waiters: &Waiters) {
        self.minimum_bytes.store(
            waiters
                .space
                .first_key_value()
                .map_or(usize::MAX, |((bytes, _), _)| *bytes),
            Ordering::Release,
        );
        self.wake_pending.store(
            !waiters.ready.is_empty() || !waiters.ready_references.is_empty(),
            Ordering::Release,
        );
        self.waiting.store(
            waiters.entries.len() + waiters.ready.len(),
            Ordering::Release,
        );
        self.physical_waiting
            .store(waiters.physical.len(), Ordering::Release);
    }

    #[cfg(not(target_family = "wasm"))]
    fn take_refund_waker(&self, gc: &GarbageCollector) -> Option<Waker> {
        let mut waiters = lock(&self.waiters);
        if self.grants_pending.swap(false, Ordering::AcqRel) {
            waiters.queue_ready_grants();
        }
        // A progress callback may itself be parked. The remaining queued
        // allocations are already eligible and can be delivered independently.
        let entry = waiters
            .ready
            .pop_first()
            .map(|(_, entry)| entry)
            .or_else(|| {
                waiters
                    .take_fitting_allocation(gc, gc.allocation_headroom())
                    .map(|(_, entry, _)| entry)
            });
        self.update_counts(&waiters);
        entry.map(|entry| entry.waker)
    }

    /// Called after releasing all collector locks.
    pub(crate) fn notify_progress(&self, gc: &GarbageCollector) {
        if self.waiter_count() == 0 {
            self.deliver_ready();
            return;
        }
        let ticket = gc.completed_full_ticket();
        let available = gc.allocation_headroom();
        if ticket <= self.notified.load(Ordering::Acquire)
            && available < self.minimum_bytes.load(Ordering::Acquire)
            && !self.grants_pending.load(Ordering::Acquire)
            && self.physical_waiting.load(Ordering::Acquire) == 0
        {
            self.deliver_ready();
            return;
        }
        let mut candidates = [None; 32];
        {
            let mut waiters = lock(&self.waiters);
            self.notified.fetch_max(ticket, Ordering::AcqRel);
            // IDs never reach WaiterId::MAX (register fails before wrapping).
            let later = waiters.entries.split_off(&(ticket, WaiterId::MAX));
            let mut ready = std::mem::replace(&mut waiters.entries, later);
            let mut remaining = available;
            for ((_, id), entry) in &ready {
                waiters.physical.remove(id);
                if let Some(bytes) = entry.bytes {
                    waiters.space.remove(&(bytes, *id));
                    if bytes <= remaining {
                        remaining -= bytes;
                        if let Some(seen) = &entry.space_seen {
                            seen.store(true, Ordering::Release);
                        }
                    }
                }
            }
            if self.grants_pending.swap(false, Ordering::AcqRel) {
                waiters.queue_ready_grants();
            }
            waiters.queue_fitting_allocations(gc, remaining);
            // Publish logical wakes before looking at the heap. A paused
            // region probe must leave unrelated wakes and cancellation accessible.
            waiters.ready.append(&mut ready);
            let cursor = waiters.physical_cursor;
            let ordered = waiters
                .physical
                .range((Excluded(cursor), Unbounded))
                .chain(waiters.physical.range(..=cursor));
            let mut last = cursor;
            for (slot, (&id, &(cycle, layout))) in candidates.iter_mut().zip(ordered) {
                *slot = Some((id, cycle, layout));
                last = id;
            }
            waiters.physical_cursor = last;
            self.update_counts(&waiters);
        }
        if let Some(heap) = gc.region_heap() {
            let mut any_ready = false;
            for candidate in &mut candidates {
                let Some((_, _, layout)) = *candidate else {
                    break;
                };
                let fits = heap.can_allocate(layout);
                #[cfg(all(test, not(target_family = "wasm")))]
                wake_progress_tests::after_physical_probe();
                if fits {
                    any_ready = true;
                } else {
                    *candidate = None;
                }
            }
            if any_ready {
                let mut waiters = lock(&self.waiters);
                for (id, cycle, layout) in candidates.into_iter().flatten() {
                    // Another helper may have delivered this wake, or the future
                    // may have cancelled or re-registered since the heap sample.
                    if waiters.physical.get(&id) != Some(&(cycle, layout)) {
                        continue;
                    }
                    waiters.physical.remove(&id);
                    if let Some(entry) = waiters.entries.remove(&(cycle, id)) {
                        waiters.ready.insert((cycle, id), entry);
                        if gc.diagnostics_enabled() {
                            gc.diagnostics
                                .physical_allocation_early_wakeups
                                .fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                self.update_counts(&waiters);
            }
        }
        self.deliver_ready();
    }

    #[cfg(not(target_family = "wasm"))]
    fn notify_all(&self) {
        {
            let mut waiters = lock(&self.waiters);
            let mut eligible = std::mem::take(&mut waiters.entries);
            waiters.ready.append(&mut eligible);
            waiters.space.clear();
            waiters.physical.clear();
            self.update_counts(&waiters);
        }
        self.deliver_ready();
    }

    fn deliver_ready(&self) {
        if !self.wake_pending.load(Ordering::Acquire) {
            return;
        }
        let Some(_delivery) = DeliveryGuard::enter(self) else {
            return;
        };
        while self.wake_pending.load(Ordering::Acquire) {
            let next = {
                let mut waiters = lock(&self.waiters);
                let reference = !waiters.ready_references.is_empty()
                    && (waiters.ready.is_empty() || waiters.reference_turn);
                waiters.reference_turn = !reference;
                let next = if reference {
                    waiters.ready_references.pop_first().map(|(_, waker)| waker)
                } else {
                    waiters.ready.pop_first().map(|(_, entry)| entry.waker)
                };
                self.update_counts(&waiters);
                next
            };
            if let Some(waker) = next {
                waker.wake();
            }
        }
    }
}

/// Independent of waker registration: an executor may delay the first poll or
/// migrate a future after notification. Cancellation removes the observation.
pub(crate) struct SpaceObservation<'a> {
    _interest: ObservationInterest<'a>,
    group: Arc<RefundGroup>,
    context_interest: Option<crate::context::AllocationInterest>,
    bytes: usize,
    offered_at_registration: bool,
    seen: Option<Arc<AtomicBool>>,
}

impl SpaceObservation<'_> {
    pub(crate) fn was_offered(&self) -> bool {
        self.offered_at_registration
            || self
                .seen
                .as_ref()
                .is_some_and(|seen| seen.load(Ordering::Acquire))
            || self.group.maximum.load(Ordering::Acquire) >= self.bytes
    }
}

struct ObservationInterest<'a>(&'a Recovery);

impl Drop for ObservationInterest<'_> {
    fn drop(&mut self) {
        // Capture the directory before relinquishing this interest. A new
        // enrolment can share its current group or install a newer directory.
        // Neither can join the earlier groups of this captured directory.
        let snapshot = self.0.observations.load();
        if self.0.observing.fetch_sub(1, Ordering::AcqRel) == 1
            && let Some(table) = snapshot.as_ref()
            && table.earlier.load().is_some()
        {
            table.earlier.store(None);
        }
    }
}

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
pub(crate) mod sync_points {
    use std::cell::Cell;
    thread_local! {
        pub(crate) static ENROLLED: Cell<Option<Box<dyn FnOnce()>>> = const { Cell::new(None) };
        pub(crate) static COVERED: Cell<Option<Box<dyn FnOnce()>>> = const { Cell::new(None) };
    }
}

// Synchronous allocation drives the same charged-interest and ticket state as
// asynchronous allocation. Only a genuinely private donor may need a later
// physical release; the native caller sleeps on a coalesced wake in that case.
pub(crate) fn finish_allocation<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    #[cfg(not(target_family = "wasm"))]
    struct Signal(std::sync::mpsc::SyncSender<()>);
    #[cfg(not(target_family = "wasm"))]
    impl std::task::Wake for Signal {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            let _ = self.0.try_send(());
        }
    }

    #[cfg(not(target_family = "wasm"))]
    let (waker, ready) = {
        let (send, receive) = std::sync::mpsc::sync_channel(1);
        (Waker::from(Arc::new(Signal(send))), receive)
    };
    #[cfg(not(target_family = "wasm"))]
    let mut context = Context::from_waker(&waker);
    #[cfg(target_family = "wasm")]
    let mut context = Context::from_waker(Waker::noop());
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
            return result;
        }
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        if let Some(hook) = sync_points::ENROLLED.take() {
            hook();
        }
        #[cfg(not(target_family = "wasm"))]
        ready
            .recv()
            .map_err(|error| Error::SyncError(error.to_string()))?;
    }
}

/// A request owns exactly one waiter slot, even when its future migrates executors.
pub(crate) struct CollectionWait<'a> {
    grant: Option<GrantWatch>,
    caller_assists: bool,
    gc: &'a GarbageCollector,
    ticket: u64,
    id: Option<WaiterId>,
    bytes: Option<usize>,
    // Survives notification/removal and migration until this future observes completion.
    space_seen: Option<Arc<AtomicBool>>,
    context_interest: Option<&'a mut Option<crate::context::AllocationInterest>>,
    refund_group: Option<&'a RefundGroup>,
    physical: Option<Layout>,
    physical_assisted: bool,
}

impl<'a> CollectionWait<'a> {
    pub(crate) fn new(gc: &'a GarbageCollector, ticket: u64) -> Self {
        Self {
            grant: None,
            caller_assists: false,
            gc,
            ticket,
            id: None,
            bytes: None,
            space_seen: None,
            context_interest: None,
            refund_group: None,
            physical: None,
            physical_assisted: false,
        }
    }

    pub(crate) fn for_space(
        gc: &'a GarbageCollector,
        ticket: u64,
        observation: &'a mut SpaceObservation<'_>,
    ) -> Self {
        Self {
            grant: None,
            caller_assists: false,
            gc,
            ticket,
            id: None,
            bytes: Some(observation.bytes),
            space_seen: Some(
                observation
                    .seen
                    .get_or_insert_with(|| {
                        Arc::new(AtomicBool::new(observation.offered_at_registration))
                    })
                    .clone(),
            ),
            context_interest: Some(&mut observation.context_interest),
            refund_group: Some(&observation.group),
            physical: None,
            physical_assisted: false,
        }
    }

    pub(crate) fn with_grant(
        mut self,
        request: &crate::retirement_grants::Request,
        settlement_only: bool,
        caller_assists: bool,
    ) -> Self {
        self.caller_assists = caller_assists;
        self.grant = Some(GrantWatch {
            request: request.clone(),
            settlement_only,
        });
        self
    }

    pub(crate) fn for_physical(gc: &'a GarbageCollector, ticket: u64, layout: Layout) -> Self {
        let mut wait = Self::new(gc, ticket);
        wait.physical = Some(layout);
        wait
    }

    // true means sufficient headroom was observed or offered; false means completion
    // without such an opportunity. An offer is advisory, not reserved allocation credit.
    fn ready(&self) -> Option<bool> {
        if self.grant.as_ref().is_some_and(GrantWatch::ready)
            || (self.physical_assisted || self.gc.recovery.workers.load(Ordering::Acquire) != 0)
                && self.physical.is_some_and(|layout| {
                    self.gc
                        .region_heap()
                        .is_some_and(|heap| heap.can_allocate(layout))
                })
            || self
                .bytes
                .is_some_and(|bytes| bytes <= self.gc.allocation_headroom())
        {
            Some(true)
        } else if self.gc.completed_full_ticket() >= self.ticket {
            Some(
                self.space_seen
                    .as_ref()
                    .is_some_and(|seen| seen.load(Ordering::Acquire))
                    || self
                        .refund_group
                        .zip(self.bytes)
                        .is_some_and(|(group, bytes)| {
                            group.maximum.load(Ordering::Acquire) >= bytes
                        }),
            )
        } else {
            None
        }
    }
}

impl Future for CollectionWait<'_> {
    type Output = Result<bool>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let gc = this.gc;
        if let Some(interest) = this.context_interest.as_deref_mut() {
            gc.refresh_allocation_interest(interest)?;
        }
        if let Some(available) = this.ready() {
            gc.recovery.remove(this.ticket, this.id);
            return Poll::Ready(Ok(available));
        }
        if crate::cell::has_guards() {
            return Poll::Ready(Err(Error::CollectionPhaseError(
                "asynchronous collection polled while holding a managed guard".into(),
            )));
        }
        if gc.recovery.failed.load(Ordering::Acquire) {
            return Poll::Ready(Err(Error::SyncError("collector worker panicked".into())));
        }
        this.id = Some(gc.recovery.register(
            this.ticket,
            this.id,
            Waiter {
                bytes: this.bytes,
                space_seen: this.space_seen.clone(),
                waker: context.waker().clone(),
                grant: this.grant.clone(),
            },
            this.physical,
        )?);
        // Registration precedes both checks, closing completion and worker-exit races.
        if let Some(available) = this.ready() {
            gc.recovery.remove(this.ticket, this.id);
            return Poll::Ready(Ok(available));
        }
        if gc.recovery.failed.load(Ordering::Acquire) {
            gc.recovery.remove(this.ticket, this.id);
            return Poll::Ready(Err(Error::SyncError("collector worker panicked".into())));
        }
        let settlement_only = this
            .grant
            .as_ref()
            .is_some_and(|grant| grant.settlement_only && grant.request.private());
        let wait_for_donor =
            settlement_only && !(cfg!(target_family = "wasm") && this.caller_assists);
        if wait_for_donor
            || (!this.caller_assists && gc.recovery.workers.load(Ordering::Acquire) != 0)
        {
            #[cfg(not(target_family = "wasm"))]
            if let Some(bytes) = this.bytes {
                // Register first, then install delivery and check again. A worker
                // may have started during this poll, or credit may already fit.
                gc.ensure_refund_notifier(settlement_only)?;
                if bytes <= gc.headroom_after_registration()
                    || this.grant.as_ref().is_some_and(GrantWatch::ready)
                {
                    gc.recovery.remove(this.ticket, this.id);
                    return Poll::Ready(Ok(true));
                }
            }
            #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
            if this.caller_assists
                && wait_for_donor
                && let Some(hook) = sync_points::COVERED.take()
            {
                hook();
            }
            return Poll::Pending;
        }
        gc.recovery.remove(this.ticket, this.id);
        if gc.recovery.failed.load(Ordering::Acquire) {
            return Poll::Ready(Err(Error::SyncError("collector worker panicked".into())));
        }
        if gc.diagnostics_enabled() {
            gc.diagnostics.async_assists.fetch_add(1, Ordering::Relaxed);
        }
        gc.step();
        // A readiness sample cannot prove the backing allocator will succeed.
        // Cooperative retries must still advance collection on each new wait,
        // even if OS allocation failed while the arena budget had space.
        this.physical_assisted = true;
        if let Some(available) = this.ready() {
            Poll::Ready(Ok(available))
        } else {
            // No workers: one bounded work batch, then return to the host executor.
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Drop for CollectionWait<'_> {
    fn drop(&mut self) {
        self.gc.recovery.remove(self.ticket, self.id);
    }
}

#[cfg(not(target_family = "wasm"))]
pub(crate) struct WorkerRegistration(std::sync::Weak<GarbageCollector>);

#[cfg(not(target_family = "wasm"))]
impl WorkerRegistration {
    pub(crate) fn new(gc: &GarbageCollector) -> Self {
        gc.recovery.workers.fetch_add(1, Ordering::AcqRel);
        Self(gc.weak())
    }
}

#[cfg(not(target_family = "wasm"))]
impl Drop for WorkerRegistration {
    fn drop(&mut self) {
        if let Some(gc) = self.0.upgrade() {
            if std::thread::panicking() {
                gc.recovery.failed.store(true, Ordering::Release);
            }
            if gc.recovery.workers.fetch_sub(1, Ordering::AcqRel) == 1
                || gc.recovery.failed.load(Ordering::Acquire)
            {
                // A waiting task can switch to cooperative progress after workers stop.
                gc.recovery.notify_all();
            }
        }
    }
}

/// Physical reservations are advisory until an allocator actually owns its block.
/// Continuing reclamation can be consumed by competitors between collection and
/// retry. Only consecutive cycles without returned ordinary ranges/backing spend
/// the failure allowance. Moving live payloads alone must not reset it: evacuation
/// can repeatedly replace backing without making a failed allocation possible.
/// Sustained competition does not promise bounded waiting.
#[derive(Default)]
pub(crate) struct PhysicalRetry {
    progress: Option<(u64, u64)>,
    completed: Option<u64>,
    progress_in_cycle: bool,
    failures: usize,
}

impl PhysicalRetry {
    fn progress(gc: &GarbageCollector) -> (u64, u64) {
        gc.region_heap()
            .map_or((0, 0), |heap| heap.recovery_progress())
    }

    pub(crate) fn exhausted(&mut self, gc: &GarbageCollector) -> bool {
        let progress = Self::progress(gc);
        if self.progress.is_some_and(|previous| progress != previous) {
            self.failures = 0;
            self.progress_in_cycle = true;
        }
        self.progress = Some(progress);
        let completed = gc.completed_full_ticket();
        if self.completed.is_some_and(|previous| previous != completed) {
            if !self.progress_in_cycle {
                self.failures = self.failures.saturating_add(1);
            }
            self.progress_in_cycle = false;
        }
        self.completed = Some(completed);
        self.failures >= 2
    }
}

pub(crate) async fn yield_once() {
    let mut yielded = false;
    std::future::poll_fn(|context| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await;
}

#[cfg(test)]
mod tests {
    #![expect(clippy::panic_in_result_fn)]
    use super::*;
    use crate::Configuration;

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn synchronous_buffer_uses_refunded_credit_during_a_parked_trace() -> Result<()> {
        use crate::{Gc, HeapBuffer, Trace};
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        struct ParkedTrace(mpsc::Sender<()>, Mutex<Option<mpsc::Receiver<()>>>);
        // SAFETY: the callback only holds channels and has no outgoing managed references.
        #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
        unsafe impl Trace for ParkedTrace {
            fn trace(&self, _: &GarbageCollector) {
                if let Some(release) = lock(&self.1).take() {
                    self.0.send(()).expect("trace observer");
                    let _ = release.recv();
                }
            }
        }

        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let payload = ParkedTrace(entered, Mutex::new(Some(release)));
        let size = Gc::allocation_size(&payload)?;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(size + 64)
                .allocation_threshold(usize::MAX)
                .incremental_step_size(1)
                .build(),
        );
        let owner = Gc::try_new(&gc, payload)?;
        let charge = gc.reserve_private_bytes(64)?;
        let marking_gc = gc.clone();
        let marking = std::thread::spawn(move || marking_gc.collect_full());
        let timeout = Duration::from_secs(if cfg!(miri) { 120 } else { 10 });
        parked
            .recv_timeout(timeout)
            .map_err(|error| Error::SyncError(format!("trace never parked: {error}")))?;

        let (finished, result) = mpsc::channel();
        let allocating_gc = gc.clone();
        let allocation = std::thread::spawn(move || {
            let buffer = HeapBuffer::try_with_fn_in(&allocating_gc, 64, |_| Ok(7_u8));
            finished
                .send(buffer.map(|buffer| buffer[63]))
                .expect("allocation observer");
        });
        let started = Instant::now();
        while gc.recovery.observing.load(Ordering::Acquire) == 0 {
            assert!(
                started.elapsed() < timeout,
                "allocator did not enter recovery"
            );
            std::thread::yield_now();
        }
        drop(charge);
        let value = result.recv_timeout(timeout);
        let completed = gc.completed_full_ticket();
        drop(resume);
        marking
            .join()
            .map_err(|_| Error::SyncError("marking thread panicked".into()))??;
        allocation
            .join()
            .map_err(|_| Error::SyncError("allocation thread panicked".into()))?;
        assert_eq!(
            value
                .map_err(|error| Error::SyncError(format!("allocation during trace: {error}")))??,
            7
        );
        assert_eq!(completed, 0);
        drop(owner);
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn bounded_physical_probes_reach_a_small_waiter_behind_large_requests() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .diagnostics(true)
                .max_heap_bytes(1024)
                .region_layout(1024, 128)
                .build(),
        );
        let heap = gc.region_heap().expect("heap");
        let mut tail = heap.allocate(Layout::new::<[u8; 1024]>(), None)?;
        let small = tail.take_prefix(Layout::new::<[u8; 64]>()).expect("prefix");
        let worker = WorkerRegistration::new(&gc);
        let mut context = Context::from_waker(Waker::noop());
        let mut large: Vec<_> = (0..40)
            .map(|_| {
                Box::pin(CollectionWait::for_physical(
                    &gc,
                    1,
                    Layout::new::<[u8; 128]>(),
                ))
            })
            .collect();
        for wait in &mut large {
            assert!(wait.as_mut().poll(&mut context).is_pending());
        }
        let mut last = Box::pin(CollectionWait::for_physical(
            &gc,
            1,
            Layout::new::<[u8; 64]>(),
        ));
        assert!(last.as_mut().poll(&mut context).is_pending());
        drop(small);
        gc.recovery.notify_progress(&gc);
        assert_eq!(gc.diagnostics().physical_allocation_early_wakeups, 0);
        gc.recovery.notify_progress(&gc);
        assert_eq!(gc.diagnostics().physical_allocation_early_wakeups, 1);
        assert!(matches!(
            last.as_mut().poll(&mut context),
            Poll::Ready(Ok(true))
        ));
        assert_eq!(gc.recovery.waiter_count(), 40);
        drop(large);
        drop(last);
        assert_eq!(gc.recovery.waiter_count(), 0);
        assert!(lock(&gc.recovery.waiters).physical.is_empty());
        drop(tail);
        drop(worker);
        assert!(heap.trim(32));
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn physical_space_wakes_before_completion_and_survives_a_stolen_offer() -> Result<()> {
        struct Wakes {
            gc: std::sync::Weak<GarbageCollector>,
            count: AtomicUsize,
        }
        impl std::task::Wake for Wakes {
            fn wake(self: Arc<Self>) {
                self.wake_by_ref();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                let gc = self.gc.upgrade().expect("collector");
                assert!(
                    gc.recovery.waiters.try_lock().is_ok(),
                    "waker under waiter mutex"
                );
                assert!(
                    gc.region_heap().expect("heap").pressure().is_some(),
                    "waker under arena mutex"
                );
                self.count.fetch_add(1, Ordering::Relaxed);
            }
        }
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .cost_aware_relocation(true)
                .diagnostics(true)
                .region_layout(1024, 128)
                .max_heap_bytes(2048)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let heap = gc.region_heap().expect("heap");
        let mut occupied = Some(crate::HeapBuffer::try_with_fn(Some(heap), 2048, |_| {
            Ok(7_u8)
        })?);
        let worker = WorkerRegistration::new(&gc); // Deterministic fake native worker.
        let notifications = Arc::new(Wakes {
            gc: gc.weak(),
            count: AtomicUsize::new(0),
        });
        let waker = Waker::from(notifications.clone());
        let mut context = Context::from_waker(&waker);
        let initialized = AtomicUsize::new(0);
        let mut future = Box::pin(crate::HeapBuffer::try_with_fn_in_async(&gc, 128, |_| {
            initialized.fetch_add(1, Ordering::Relaxed);
            Ok(9_u8)
        }));
        assert!(future.as_mut().poll(&mut context).is_pending()); // Retry yield.
        assert!(future.as_mut().poll(&mut context).is_pending()); // Registration.
        assert_eq!(gc.recovery.waiter_count(), 1);
        for _ in 0..3 {
            drop(occupied.take());
            let before = notifications.count.load(Ordering::Relaxed);
            gc.recovery.notify_progress(&gc);
            assert!(notifications.count.load(Ordering::Relaxed) > before);
            assert_eq!(gc.recovery.waiter_count(), 0);
            occupied = Some(crate::HeapBuffer::try_with_fn(Some(heap), 2048, |_| {
                Ok(7_u8)
            })?);
            assert!(future.as_mut().poll(&mut context).is_pending());
            assert_eq!(gc.recovery.waiter_count(), 1);
            assert_eq!(initialized.load(Ordering::Relaxed), 0);
        }
        drop(occupied);
        gc.recovery.notify_progress(&gc);
        let Poll::Ready(buffer) = future.as_mut().poll(&mut context) else {
            panic!("physical recovery should finish before its collection ticket");
        };
        let buffer = buffer?;
        assert!(buffer.iter().all(|value| *value == 9));
        assert_eq!(initialized.load(Ordering::Relaxed), 128);
        assert_eq!(gc.completed_full_ticket(), 0);
        assert_eq!(gc.diagnostics().physical_allocation_early_wakeups, 4);
        assert_eq!(gc.recovery.waiter_count(), 0);
        assert!(lock(&gc.recovery.waiters).physical.is_empty());
        let mut cancelled = Box::pin(CollectionWait::for_physical(
            &gc,
            1,
            Layout::new::<[u8; 4096]>(),
        ));
        assert!(cancelled.as_mut().poll(&mut context).is_pending());
        assert_eq!(gc.recovery.waiter_count(), 1);
        drop(cancelled);
        assert_eq!(gc.recovery.waiter_count(), 0);
        assert!(lock(&gc.recovery.waiters).physical.is_empty());
        drop(future);
        drop(buffer);
        drop(worker);
        gc.collect_full()?;
        assert_eq!(heap.statistics().shared_committed_bytes, 0);
        Ok(())
    }

    #[test]
    fn cooperative_physical_hints_still_advance_collection_and_cancellation_unlinks() -> Result<()>
    {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .diagnostics(true)
                .region_layout(1024, 128)
                .max_heap_bytes(2048)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let layout = Layout::from_size_align(128, 1).map_err(|_| Error::OutOfMemory)?;
        let mut context = Context::from_waker(Waker::noop());
        let mut retry = PhysicalRetry::default();
        assert!(!retry.exhausted(&gc));
        // Model OS allocation failure despite sufficient arena credit. An early
        // hint must not turn repeated failures into a threadless busy loop.
        for _ in 0..1000 {
            if retry.exhausted(&gc) {
                break;
            }
            gc.collect();
            let ticket = gc.completed_full_ticket() + 1;
            let mut wait = Box::pin(CollectionWait::for_physical(&gc, ticket, layout));
            let before = gc.diagnostics().caller_steps.count;
            assert!(wait.as_mut().poll(&mut context).is_ready());
            assert_eq!(gc.diagnostics().caller_steps.count, before + 1);
            drop(wait);
            assert_eq!(gc.recovery.waiter_count(), 0);
            assert!(lock(&gc.recovery.waiters).physical.is_empty());
        }
        assert!(retry.exhausted(&gc));
        assert!(gc.completed_full_ticket() >= 2);
        Ok(())
    }

    #[test]
    fn live_evacuation_alone_does_not_keep_an_impossible_allocation_waiting() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .compact_heap(true)
                .cost_aware_relocation(true)
                .relocation_reserve_bytes(1024)
                .region_layout(1024, 128)
                .max_heap_bytes(4096)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let root = crate::Gc::try_new(
            &gc,
            crate::sync::RwLock::new(crate::HeapBuffer::try_with_fn_in(&gc, 512, |_| Ok(7_u8))?),
        )?;
        let mut future =
            std::pin::pin!(crate::HeapBuffer::try_with_fn_in_async(&gc, 3000, |_| Ok(
                9_u8
            ),));
        let mut context = Context::from_waker(Waker::noop());
        let mut finished = false;
        for _ in 0..1000 {
            if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
                assert!(matches!(result, Err(Error::OutOfMemory)));
                finished = true;
                break;
            }
        }
        assert!(
            finished,
            "evacuation does not manufacture recovery progress"
        );
        assert!(root.read().iter().all(|value| *value == 7));
        assert!(gc.region_heap().expect("heap").statistics().relocations >= 2);
        drop(root);
        gc.collect_full()?;
        assert_eq!(
            gc.region_heap()
                .expect("heap")
                .statistics()
                .shared_committed_bytes,
            0
        );
        Ok(())
    }

    #[test]
    fn a_refund_stolen_during_the_first_yield_does_not_spend_an_empty_cycle() -> Result<()> {
        use std::task::Wake;
        struct Runnable(AtomicBool);
        impl Wake for Runnable {
            fn wake(self: Arc<Self>) {
                self.0.store(true, Ordering::Release);
            }
        }
        for advisory in [false, true] {
            for refund in [false, true] {
                let gc = GarbageCollector::with_config(
                    Configuration::builder()
                        .max_heap_bytes(8)
                        .allocation_threshold(usize::MAX)
                        .build(),
                );
                let mut charge = Some(gc.reserve_exact_bytes(8)?);
                let runnable = Arc::new(Runnable(AtomicBool::new(false)));
                let waker = Waker::from(runnable.clone());
                let mut cx = Context::from_waker(&waker);
                let mut future = Box::pin(async {
                    if advisory {
                        gc.make_room_async(8).await
                    } else {
                        crate::Gc::try_new_async(&gc, 42_u64).await.map(drop)
                    }
                });
                assert!(future.as_mut().poll(&mut cx).is_pending());
                assert_eq!(gc.completed_full_ticket(), 0);
                if refund {
                    drop(charge.take());
                    charge = Some(gc.reserve_exact_bytes(8)?);
                }
                let mut terminal = false;
                for _ in 0..1024 {
                    assert!(
                        runnable.0.swap(false, Ordering::AcqRel),
                        "poll requires a real wake"
                    );
                    if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
                        assert!(matches!(result, Err(Error::OutOfMemory)));
                        terminal = true;
                        break;
                    }
                }
                assert!(terminal);
                assert_eq!(
                    gc.completed_full_ticket(),
                    if refund { 3 } else { 2 },
                    "advisory={advisory} refund={refund}"
                );
                drop((future, charge));
                gc.stop()?;
                assert_eq!(gc.managed_bytes(), 0);
            }
        }
        Ok(())
    }

    #[test]
    fn cancelling_during_the_first_yield_restores_context_batching() -> Result<()> {
        let batch = crate::allocation::CREDIT;
        for advisory in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .max_heap_bytes(batch)
                    .local_allocation_buffers(true)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let scope = gc.register_context();
            let charge = gc.reserve_exact_bytes(batch)?;
            let mut future = Box::pin(async {
                if advisory {
                    gc.make_room_async(8).await
                } else {
                    crate::Gc::try_new_async(&gc, 42_u64).await.map(drop)
                }
            });
            let mut cx = Context::from_waker(Waker::noop());
            assert!(scope.with(|| future.as_mut().poll(&mut cx)).is_pending());
            assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 1);
            drop(future);
            assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 0);
            assert_eq!(gc.recovery.waiter_count(), 0);
            drop(charge);
            let next = scope.with(|| gc.reserve_bytes(8))?;
            assert_eq!(scope.0.available_credit(), batch - 8);
            drop((next, scope));
            gc.collect_full()?;
            assert_eq!(gc.completed_full_ticket(), 1);
            assert_eq!(gc.managed_bytes(), 0);
        }
        Ok(())
    }

    #[test]
    fn reclaimed_headroom_survives_competition_before_first_poll() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(8)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        gc.reserve_allocation(8)?;
        for released in [4, 8] {
            let mut observation = gc.observe_space(8)?;
            // Install the observation before requesting collection. Neither an
            // executor poll nor a waker registration has happened yet.
            let ticket = gc.completed_full_ticket() + 1;
            gc.collect();
            gc.cancel_allocation(released);
            gc.reserve_allocation(released)?;
            gc.collect_full()?;
            let mut wait = CollectionWait::for_space(&gc, ticket, &mut observation);
            assert!(matches!(
                Pin::new(&mut wait).poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(Ok(seen)) if seen == (released == 8)
            ));
        }
        gc.cancel_allocation(8);
        assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 0);
        assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 0);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn refund_records_a_stolen_offer_even_with_the_waiter_index_parked() -> Result<()> {
        use std::sync::mpsc;
        use std::time::Duration;
        let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(8).build());
        let charge = gc.reserve_private_bytes(8)?;
        let fits = gc.observe_space(8)?;
        let large = gc.observe_space(9)?;
        std::thread::scope(|scope| {
            let parked = lock(&gc.recovery.waiters);
            let (done, recv) = mpsc::channel();
            scope.spawn(move || {
                drop(charge);
                let _ = done.send(());
            });
            let finished = recv.recv_timeout(Duration::from_secs(2)).is_ok();
            drop(parked);
            assert!(finished, "refund stalled with an unrelated waiter writer");
        });
        assert_eq!(gc.managed_bytes(), 0);
        let stolen = gc.reserve_private_bytes(8)?;
        assert!(fits.was_offered());
        assert!(!large.was_offered());
        drop(stolen);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn parked_observations_leave_ticket_wakes_and_cancellation_available() {
        use std::sync::mpsc;
        use std::time::Duration;
        let gc = GarbageCollector::new();
        let worker = WorkerRegistration::new(&gc);
        let mut cancelled = Box::pin(CollectionWait::new(&gc, 10));
        assert!(
            cancelled
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        let mut completed = Box::pin(CollectionWait::new(&gc, 1));
        assert!(
            completed
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        std::thread::scope(|scope| {
            observer_progress_tests::while_registration_is_parked(&gc, || {
                let (sent, received) = mpsc::channel();
                let collector = &gc;
                scope.spawn(move || {
                    drop(cancelled);
                    collector.collect_full().expect("full cycle completes");
                    let ready = completed
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()));
                    let _ = sent.send(ready.is_ready());
                });
                let progress = received.recv_timeout(Duration::from_secs(2));
                assert_eq!(progress, Ok(true));
            });
        });
        assert_eq!(gc.recovery.waiter_count(), 0);
        drop(worker);
    }

    #[test]
    fn opportunity_at_registration_survives_a_thief_before_the_first_poll() -> Result<()> {
        let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(8).build());
        let mut early = gc.observe_space(8)?;
        let mut too_large = gc.observe_space(9)?;
        let stolen = gc.reserve_private_bytes(8)?;
        let mut late = gc.observe_space(8)?;
        let ticket = gc.completed_full_ticket() + 1;
        gc.collect_full()?;
        for (observation, expected) in [
            (&mut early, true),
            (&mut too_large, false),
            (&mut late, false),
        ] {
            let mut wait = CollectionWait::for_space(&gc, ticket, observation);
            assert!(matches!(
                Pin::new(&mut wait).poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(Ok(actual)) if actual == expected
            ));
        }
        drop(stolen);
        Ok(())
    }

    #[test]
    fn registrations_share_only_future_refunds_and_apply_their_own_thresholds() -> Result<()> {
        let recovery = Recovery::default();
        let small = recovery.register_observation(4)?;
        let big = recovery.register_observation(8)?;
        assert!(Arc::ptr_eq(&small.group, &big.group));
        recovery.record_headroom(4);
        let later = recovery.register_observation(4)?;
        assert!(!Arc::ptr_eq(&small.group, &later.group));
        assert!(small.was_offered());
        assert!(!big.was_offered());
        assert!(!later.was_offered());
        drop(small);
        recovery.record_headroom(8);
        assert!(big.was_offered());
        assert!(later.was_offered());
        drop((big, later));
        assert!(
            recovery
                .observations
                .load()
                .as_ref()
                .is_none_or(|table| table.earlier.load().is_none())
        );
        Ok(())
    }

    #[test]
    fn cancelling_an_unpolled_space_observation_removes_it() -> Result<()> {
        let recovery = Recovery::default();
        let small = recovery.register_observation(4)?;
        let large = recovery.register_observation(8)?;
        drop(small);
        recovery.record_headroom(4);
        assert!(!large.was_offered());
        drop(large);
        assert_eq!(recovery.observing.load(Ordering::Acquire), 0);
        let next = recovery.register_observation(4)?;
        assert!(!next.was_offered());
        Ok(())
    }
}

#[cfg(test)]
#[path = "recovery_wake_progress_tests.rs"]
mod wake_progress_tests;

#[path = "recovery_reference_wait.rs"]
mod reference_wait;

#[cfg(all(test, not(target_family = "wasm")))]
#[path = "recovery_observer_progress_tests.rs"]
mod observer_progress_tests;
