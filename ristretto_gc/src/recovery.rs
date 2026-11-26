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

#[derive(Debug)]
struct Waiter {
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

#[derive(Debug, Default)]
struct Observations {
    next_id: WaiterId,
    entries: BTreeMap<(usize, WaiterId), Arc<AtomicBool>>,
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
    observations: Mutex<Observations>,
    notified: AtomicU64,
    waiting: AtomicUsize,
    wake_pending: AtomicBool,
    physical_waiting: AtomicUsize,
    minimum_bytes: AtomicUsize,
    minimum_observation: AtomicUsize,
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

impl Recovery {
    pub(crate) fn registered_workers(&self) -> usize {
        self.workers.load(Ordering::Acquire)
    }

    /// Install before requesting a cycle, including before the first future poll.
    pub(crate) fn observe_space(&self, bytes: usize) -> Result<SpaceObservation<'_>> {
        let seen = Arc::new(AtomicBool::new(false));
        let mut observations = lock(&self.observations);
        observations.next_id = observations
            .next_id
            .checked_add(1)
            .filter(|id| *id != WaiterId::MAX)
            .ok_or(Error::OutOfMemory)?;
        let id = observations.next_id;
        observations.entries.insert((bytes, id), seen.clone());
        self.update_observation_minimum(&observations);
        Ok(SpaceObservation {
            recovery: self,
            bytes,
            id,
            seen,
        })
    }

    fn update_observation_minimum(&self, observations: &Observations) {
        self.minimum_observation.store(
            observations
                .entries
                .first_key_value()
                .map_or(usize::MAX, |((bytes, _), _)| *bytes),
            Ordering::Release,
        );
        self.observing
            .store(observations.entries.len(), Ordering::Release);
    }

    /// Use headroom at the refund's atomic linearization point, before a competing
    /// reservation can consume it. This records an opportunity, not owned credit.
    /// No waker or other user callback runs while collector locks are held.
    pub(crate) fn record_headroom(&self, available: usize) {
        if self.observing.load(Ordering::Acquire) == 0
            || available < self.minimum_observation.load(Ordering::Acquire)
        {
            return;
        }
        let mut observations = lock(&self.observations);
        while let Some((&key, _)) = observations.entries.first_key_value() {
            if key.0 > available {
                break;
            }
            if let Some(seen) = observations.entries.remove(&key) {
                seen.store(true, Ordering::Release);
            }
        }
        self.update_observation_minimum(&observations);
    }

    fn register(
        &self,
        ticket: u64,
        id: Option<WaiterId>,
        bytes: Option<usize>,
        waker: &Waker,
        space_seen: Option<&Arc<AtomicBool>>,
        physical: Option<Layout>,
    ) -> Result<WaiterId> {
        let waker = waker.clone();
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
            if let Some(bytes) = bytes {
                waiters.space.insert((bytes, id), ticket);
            }
            if let Some(layout) = physical {
                waiters.physical.insert(id, (ticket, layout));
            }
            let queued = waiters.ready.remove(&(ticket, id));
            let previous = waiters.entries.insert(
                (ticket, id),
                Waiter {
                    bytes,
                    waker,
                    space_seen: space_seen.cloned(),
                },
            );
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
        self.waiter_count() != 0 && available >= self.minimum_bytes.load(Ordering::Acquire)
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
        let (_, entry, _) = waiters.take_fitting_allocation(gc, gc.allocation_headroom())?;
        self.update_counts(&waiters);
        Some(entry.waker)
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
    recovery: &'a Recovery,
    bytes: usize,
    id: WaiterId,
    seen: Arc<AtomicBool>,
}

impl SpaceObservation<'_> {
    pub(crate) fn was_offered(&self) -> bool {
        self.seen.load(Ordering::Acquire)
    }
}

impl Drop for SpaceObservation<'_> {
    fn drop(&mut self) {
        let mut observations = lock(&self.recovery.observations);
        observations.entries.remove(&(self.bytes, self.id));
        self.recovery.update_observation_minimum(&observations);
    }
}

/// A request owns exactly one waiter slot, even when its future migrates executors.
pub(crate) struct CollectionWait<'a> {
    gc: &'a GarbageCollector,
    ticket: u64,
    id: Option<WaiterId>,
    bytes: Option<usize>,
    // Survives notification/removal and migration until this future observes completion.
    space_seen: Option<Arc<AtomicBool>>,
    physical: Option<Layout>,
    physical_assisted: bool,
}

impl<'a> CollectionWait<'a> {
    pub(crate) fn new(gc: &'a GarbageCollector, ticket: u64) -> Self {
        Self {
            gc,
            ticket,
            id: None,
            bytes: None,
            space_seen: None,
            physical: None,
            physical_assisted: false,
        }
    }

    pub(crate) fn for_space(
        gc: &'a GarbageCollector,
        ticket: u64,
        observation: &SpaceObservation<'_>,
    ) -> Self {
        Self {
            gc,
            ticket,
            id: None,
            bytes: Some(observation.bytes),
            space_seen: Some(observation.seen.clone()),
            physical: None,
            physical_assisted: false,
        }
    }

    pub(crate) fn for_physical(gc: &'a GarbageCollector, ticket: u64, layout: Layout) -> Self {
        let mut wait = Self::new(gc, ticket);
        wait.physical = Some(layout);
        wait
    }

    // true means sufficient headroom was observed or offered; false means completion
    // without such an opportunity. An offer is advisory, not reserved allocation credit.
    fn ready(&self) -> Option<bool> {
        if (self.physical_assisted || self.gc.recovery.workers.load(Ordering::Acquire) != 0)
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
                    .is_some_and(|seen| seen.load(Ordering::Acquire)),
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
            this.bytes,
            context.waker(),
            this.space_seen.as_ref(),
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
        if gc.recovery.workers.load(Ordering::Acquire) != 0 {
            #[cfg(not(target_family = "wasm"))]
            if let Some(bytes) = this.bytes {
                // Register first, then install delivery and check again. A worker
                // may have started during this poll, or credit may already fit.
                gc.ensure_refund_notifier()?;
                if bytes <= gc.headroom_after_registration() {
                    gc.recovery.remove(this.ticket, this.id);
                    return Poll::Ready(Ok(true));
                }
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
    fn reclaimed_headroom_survives_competition_before_first_poll() -> Result<()> {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(8)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        gc.reserve_allocation(8)?;
        for released in [4, 8] {
            let observation = gc.recovery.observe_space(8)?;
            // Install the observation before requesting collection. Neither an
            // executor poll nor a waker registration has happened yet.
            let ticket = gc.completed_full_ticket() + 1;
            gc.collect();
            gc.cancel_allocation(released);
            gc.reserve_allocation(released)?;
            gc.collect_full()?;
            let mut wait = CollectionWait::for_space(&gc, ticket, &observation);
            assert!(matches!(
                Pin::new(&mut wait).poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(Ok(seen)) if seen == (released == 8)
            ));
        }
        gc.cancel_allocation(8);
        assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 0);
        assert!(lock(&gc.recovery.observations).entries.is_empty());
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn refund_records_a_stolen_offer_even_with_the_waiter_index_parked() -> Result<()> {
        use std::sync::mpsc;
        use std::time::Duration;
        let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(8).build());
        let charge = gc.reserve_private_bytes(8)?;
        let fits = gc.recovery.observe_space(8)?;
        let large = gc.recovery.observe_space(9)?;
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
    fn parked_observations_leave_ticket_wakes_and_cancellation_available() -> Result<()> {
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
            let parked = lock(&gc.recovery.observations);
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
            drop(parked);
            assert_eq!(progress, Ok(true));
        });
        assert_eq!(gc.recovery.waiter_count(), 0);
        drop(worker);
        Ok(())
    }

    #[test]
    fn cancelling_an_unpolled_space_observation_removes_it() -> Result<()> {
        let recovery = Recovery::default();
        let small = recovery.observe_space(4)?;
        let large = recovery.observe_space(8)?;
        drop(small);
        assert_eq!(recovery.minimum_observation.load(Ordering::Acquire), 8);
        recovery.record_headroom(4);
        assert!(!large.seen.load(Ordering::Acquire));
        drop(large);
        assert_eq!(recovery.observing.load(Ordering::Acquire), 0);
        assert!(lock(&recovery.observations).entries.is_empty());
        Ok(())
    }
}

#[cfg(test)]
#[path = "recovery_wake_progress_tests.rs"]
mod wake_progress_tests;

#[path = "recovery_reference_wait.rs"]
mod reference_wait;
