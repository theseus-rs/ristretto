//! Native finalizer work remains singly owned while the marking workers serve
//! later tickets. Each running pool opens its callback thread on first use.
use super::{GarbageCollector, ReclaimTask, WorkCounters, lock, now};
use crate::TraceProgress;
use once_cell::race::OnceBox;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[path = "callback_pages.rs"]
mod callback_pages;
use callback_pages::Pages;
#[path = "pressure_turn.rs"]
mod pressure_turn;

pub(super) struct Turn {
    dispatch: pressure_turn::Turn,
    inbox: std::sync::Weak<Inbox<Job>>,
}

impl Turn {
    pub(super) fn pending(&self) -> bool {
        self.dispatch.pending()
            && self
                .inbox
                .upgrade()
                .is_some_and(|inbox| inbox.jobs.has_available())
    }
}

thread_local! {
    // Native workers are owned by exactly one pool generation for their lifetime.
    static POOL: std::cell::OnceCell<std::sync::Weak<super::WorkerSignals>> = const { std::cell::OnceCell::new() };
}

pub(super) fn register(pool: &Arc<super::WorkerSignals>) {
    POOL.with(|current| {
        let _ = current.set(Arc::downgrade(pool));
    });
}

pub(super) fn pool() -> Option<Arc<super::WorkerSignals>> {
    POOL.with(|current| current.get().and_then(std::sync::Weak::upgrade))
}

#[derive(Debug, Default)]
pub(super) struct Callbacks(OnceBox<Runner>);

const ARMED: usize = 1;
const PUBLISHER: usize = 2;

struct Inbox<T> {
    jobs: Pages<T>,
    // A pre-link notice wakes an idle consumer. When a publisher is in flight,
    // the consumer retries even if that publisher parks before its final notice.
    publishing: AtomicUsize,
}

impl<T> std::fmt::Debug for Inbox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inbox")
            .field("publishing", &self.publishing)
            .finish_non_exhaustive()
    }
}

impl<T> Inbox<T> {
    fn publish(
        &self,
        notice: &mpsc::SyncSender<()>,
        producer: usize,
        first: T,
        rest: impl ExactSizeIterator<Item = T>,
    ) {
        self.publish_observed(notice, producer, first, rest, || {}, || {});
    }

    fn publish_observed(
        &self,
        notice: &mpsc::SyncSender<()>,
        producer: usize,
        first: T,
        rest: impl ExactSizeIterator<Item = T>,
        before_link: impl FnOnce(),
        after_link: impl FnOnce(),
    ) {
        if self.publishing.fetch_add(PUBLISHER, Ordering::AcqRel) & ARMED != 0 {
            let _ = notice.try_send(());
        }
        before_link();
        self.jobs.push(producer, first, rest);
        after_link();
        if self.publishing.fetch_sub(PUBLISHER, Ordering::AcqRel) & ARMED != 0 {
            let _ = notice.try_send(());
        }
    }

    fn has_work(&self) -> bool {
        self.publishing.load(Ordering::Acquire) >= PUBLISHER || self.jobs.has_available()
    }

    // The same atomic orders enrollment and arming. If a publisher saw an idle
    // primary it sends a notice. Otherwise the recheck sees its completed
    // publication, or the primary sees an enrolled publisher and retries.
    fn next_job(&self, notices: &mpsc::Receiver<()>) -> Option<T> {
        loop {
            if let Some(job) = self.jobs.claim() {
                return Some(job);
            }
            let at_arm = self.publishing.fetch_or(ARMED, Ordering::AcqRel);
            if let Some(job) = self.jobs.claim() {
                self.publishing.fetch_and(!ARMED, Ordering::AcqRel);
                return Some(job);
            }
            // Use the arming snapshot: a publisher can finish linking and park
            // between retiring its reservation and sending its final notice.
            let closed = if at_arm < PUBLISHER {
                notices.recv().is_err()
            } else {
                matches!(
                    notices.recv_timeout(std::time::Duration::from_millis(1)),
                    Err(mpsc::RecvTimeoutError::Disconnected)
                )
            };
            self.publishing.fetch_and(!ARMED, Ordering::AcqRel);
            if closed {
                return self.jobs.claim();
            }
        }
    }
}

#[cfg(all(test, not(miri), panic = "unwind"))]
type PressureGate = (mpsc::SyncSender<()>, mpsc::Receiver<()>);

#[cfg(all(test, not(miri), panic = "unwind"))]
#[expect(clippy::expect_used, reason = "bounded pressure-worker schedule")]
fn pause_pressure(gate: &Mutex<Option<PressureGate>>) {
    if let Some((entered, release)) = lock(gate).take() {
        entered.send(()).expect("observer");
        release
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("resume service");
    }
}

#[derive(Debug)]
struct Runner {
    send: arc_swap::ArcSwapOption<mpsc::SyncSender<()>>,
    inbox: Arc<Inbox<Job>>,
    #[cfg(all(test, not(miri), panic = "unwind"))]
    notices: Arc<Mutex<mpsc::Receiver<()>>>,
    #[cfg(all(test, not(miri), panic = "unwind"))]
    pressure_gate: Arc<Mutex<Option<PressureGate>>>,
    service: Arc<pressure_turn::Service>,
    helper_failed: AtomicBool,
    pressure: mpsc::SyncSender<()>,
    closing: Arc<AtomicBool>,
    threads: Mutex<Option<Threads>>,
}

#[derive(Debug)]
struct Threads {
    primary: thread::JoinHandle<bool>,
    assistant: Option<thread::JoinHandle<bool>>,
}

pub(super) struct Batch {
    pub(super) first: (ReclaimTask, usize),
    pub(super) rest: Vec<(ReclaimTask, usize)>,
}

struct Job {
    gc: Arc<GarbageCollector>,
    task: ReclaimTask,
    budget: usize,
    origin: Option<(usize, u64)>,
}

impl Callbacks {
    pub(super) fn terminal_turn(&self) -> Option<Turn> {
        let runner = self.0.get()?;
        if !runner.inbox.has_work() {
            return None;
        }
        let turn = runner.service.request()?;
        let _ = runner.pressure.try_send(());
        // A peer may take the last job after enrollment, before the pressure
        // assistant acknowledges the turn. Private jobs keep their charge.
        Some(Turn {
            dispatch: turn,
            inbox: Arc::downgrade(&runner.inbox),
        })
    }

    pub(super) fn wake_pressure(&self) {
        if let Some(runner) = self.0.get() {
            let _ = runner.pressure.try_send(());
        }
    }

    pub(super) fn send(
        &self,
        gc: &GarbageCollector,
        batch: Batch,
        origin: Option<(usize, u64)>,
    ) -> Option<Batch> {
        let Ok(runner) = self
            .0
            .get_or_try_init(|| Runner::spawn(gc.this.clone(), gc.configuration.threads))
        else {
            return Some(batch);
        };
        let Some(owner) = gc.this.upgrade() else {
            return Some(batch);
        };
        let Some(sender) = runner.send.load_full() else {
            return Some(batch);
        };
        let count = 1 + batch.rest.len();
        gc.callback_jobs.fetch_add(count, Ordering::AcqRel);
        let make_job = |(task, budget)| Job {
            gc: owner.clone(),
            task,
            budget,
            origin,
        };
        runner.inbox.publish(
            &sender,
            super::WORKER.get().map_or(0, |(_, index)| index),
            make_job(batch.first),
            batch.rest.into_iter().map(make_job),
        );
        if gc.grant_registry.has_demand() {
            let _ = runner.pressure.try_send(());
        }
        None
    }

    pub(super) fn join(&self) -> crate::Result<bool> {
        if let Some(runner) = self.0.get() {
            // Disconnect after the pool stops. A sender already holding its own
            // handle can finish publication; the receiver drains all accepted jobs.
            runner.send.store(None);
            runner.closing.store(true, Ordering::Release);
            runner.service.stop();
            let _ = runner.pressure.try_send(());
            if let Some(handles) = lock(&runner.threads).take() {
                let mut failed = false;
                let mut panicked = false;
                for handle in std::iter::once(handles.primary).chain(handles.assistant) {
                    if handle.thread().id() != thread::current().id() {
                        match handle.join() {
                            Ok(result) => failed |= result,
                            Err(_) => panicked = true,
                        }
                    }
                }
                if panicked {
                    return Err(crate::Error::SyncError(
                        "collector callback worker panicked".into(),
                    ));
                }
                return Ok(failed || runner.helper_failed.load(Ordering::Acquire));
            }
        }
        Ok(false)
    }

    pub(super) fn help_ready(&self, jobs: usize) {
        let Some(runner) = self.0.get() else { return };
        // Retain a sender through the last acknowledgement: shutdown's receive
        // loop cannot see disconnection while this helper owns a callback.
        let Some(_participation) = runner.send.load_full() else {
            return;
        };
        for _ in 0..jobs {
            let Some(job) = runner.inbox.jobs.claim() else {
                break;
            };
            if job.run() {
                runner.helper_failed.store(true, Ordering::Release);
            }
        }
    }
}

impl Job {
    fn run(self) -> bool {
        let Self {
            gc,
            mut task,
            budget,
            origin,
        } = self;
        let _cleanup = super::CleanupScope::enter(&gc);
        let mut failed = false;
        let mut counters = WorkCounters::default();
        let started = now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let progress = gc.clear_retired(&mut task, budget, &mut counters, None);
            if progress == TraceProgress::Complete {
                if let Some((domain, epoch)) = origin {
                    gc.record_sweep_reclaimed(domain, epoch, task.metadata.size());
                }
            } else {
                gc.queue_clearing(task, progress);
            }
            gc.retired_ready.fetch_sub(1, Ordering::AcqRel);
            gc.record_step(started, None, true, &mut counters);
            progress != TraceProgress::Complete
        }));
        if result.is_err() {
            failed = true;
            gc.recovery.callback_failed();
        }
        if result.unwrap_or(true) {
            gc.wake();
        }
        // The next job may block indefinitely. Deliver this job's refund and
        // progress, and contain any executor unwind, before invoking that job.
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gc.notify_progress())).is_err()
        {
            failed = true;
            if gc.recovery.callback_failed() {
                // Waiters on unfinished tickets can now observe failure.
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    gc.notify_progress();
                }));
            }
            gc.wake();
        }
        if gc.callback_jobs.fetch_sub(1, Ordering::AcqRel) == 1 {
            gc.wake();
        }
        failed
    }
}

impl Runner {
    fn spawn(
        heap: std::sync::Weak<GarbageCollector>,
        workers: usize,
    ) -> std::io::Result<Box<Self>> {
        let (send, notices) = mpsc::sync_channel(1);
        let notices = Arc::new(Mutex::new(notices));
        let waiting = notices.clone();
        let inbox = Arc::new(Inbox::<Job> {
            jobs: Pages::new(workers),
            publishing: AtomicUsize::new(0),
        });
        let queue = inbox.clone();
        let primary = thread::Builder::new()
            .name("gc-finalizers".into())
            .spawn(move || {
                let mut failed = false;
                let mut cursor = queue.jobs.cursor();
                loop {
                    let next = cursor.claim().or_else(|| queue.next_job(&lock(&waiting)));
                    let Some(job) = next else { break };
                    failed |= job.run();
                }
                failed
            })?;
        let (pressure, requested) = mpsc::sync_channel(1);
        let closing = Arc::new(AtomicBool::new(false));
        let stopping = closing.clone();
        let queue = inbox.clone();
        // Only an allocation at the logical cap activates the second consumer.
        // Its queue token is advisory. Ownership and refunds stay with Job::run.
        #[cfg(all(test, not(miri), panic = "unwind"))]
        let gate = Arc::new(Mutex::new(None::<PressureGate>));
        #[cfg(all(test, not(miri), panic = "unwind"))]
        let pressure_gate = gate.clone();
        let service = Arc::new(pressure_turn::Service::default());
        let dispatch = pressure_turn::Assistant(service.clone());
        let assistant = thread::Builder::new()
            .name("gc-pressure-finalizers".into())
            .spawn(move || {
                let mut failed = false;
                let mut cursor = queue.jobs.cursor();
                while requested.recv().is_ok() && !stopping.load(Ordering::Acquire) {
                    #[cfg(all(test, not(miri), panic = "unwind"))]
                    pause_pressure(&gate);
                    loop {
                        // Hold the heap only to check demand. A sleeping assistant
                        // does not keep the collector alive.
                        let turn = dispatch.0.snapshot();
                        if stopping.load(Ordering::Acquire)
                            || !heap
                                .upgrade()
                                .is_some_and(|gc| gc.grant_registry.has_demand())
                        {
                            dispatch.0.examined(turn);
                            break;
                        }
                        let next = cursor.claim();
                        let Some(job) = next else {
                            dispatch.0.examined(turn);
                            break;
                        };
                        dispatch.0.running(true);
                        dispatch.0.examined(turn);
                        failed |= job.run();
                        dispatch.0.running(false);
                    }
                }
                failed
            })
            // The primary can still drain all accepted jobs if the OS refuses
            // the optional thread. Allocation keeps its ordinary finite retry.
            .ok();
        Ok(Box::new(Self {
            send: arc_swap::ArcSwapOption::from(Some(Arc::new(send))),
            inbox,
            #[cfg(all(test, not(miri), panic = "unwind"))]
            notices,
            #[cfg(all(test, not(miri), panic = "unwind"))]
            pressure_gate,
            service,
            helper_failed: AtomicBool::new(false),
            pressure,
            closing,
            threads: Mutex::new(Some(Threads { primary, assistant })),
        }))
    }
}

#[cfg(all(test, not(miri), panic = "unwind"))]
mod claim_pause {
    use std::cell::RefCell;
    use std::sync::mpsc;
    use std::time::Duration;

    pub(super) struct Pause {
        pub(super) after: bool,
        pub(super) entered: mpsc::Sender<()>,
        pub(super) release: mpsc::Receiver<()>,
    }
    thread_local! {
        pub(super) static PAUSE: RefCell<Option<Pause>> = const { RefCell::new(None) };
    }
    #[expect(clippy::expect_used, reason = "bounded forced claim schedule")]
    pub(super) fn at(after: bool) {
        PAUSE.with_borrow_mut(|pause| {
            if pause.as_ref().is_some_and(|p| p.after == after) {
                let pause = pause.take().expect("matching pause");
                pause.entered.send(()).expect("claim observer");
                pause
                    .release
                    .recv_timeout(Duration::from_secs(10))
                    .expect("release claimant");
            }
        });
    }
}

#[cfg(all(test, not(miri), panic = "unwind"))]
mod tests {
    use super::*;
    use crate::{Configuration, Gc, Result, Trace};
    use std::sync::atomic::AtomicUsize;
    use std::time::{Duration, Instant};

    struct Donor(Arc<AtomicUsize>);
    // SAFETY: the counter has no managed edges.
    #[expect(unsafe_code)]
    unsafe impl Trace for Donor {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Donor {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Release);
        }
    }

    #[test]
    fn terminal_allocation_observes_pressure_service_before_denial() -> Result<()> {
        delayed_pressure(PressureSchedule::Available)
    }

    #[test]
    fn cancelled_terminal_dispatch_leaves_a_subsequent_allocation_eligible() -> Result<()> {
        delayed_pressure(PressureSchedule::Cancel)
    }

    #[test]
    fn a_contended_notice_receiver_leaves_jobs_available_to_the_assistant() -> Result<()> {
        delayed_pressure(PressureSchedule::Contended)
    }

    #[test]
    fn parked_callback_with_no_queued_sibling_needs_no_assistant_turn() -> Result<()> {
        delayed_pressure(PressureSchedule::Empty)
    }

    #[test]
    fn a_peer_claiming_the_last_job_completes_an_enrolled_dispatch() -> Result<()> {
        delayed_pressure(PressureSchedule::LastClaim)
    }

    #[derive(Clone, Copy)]
    enum PressureSchedule {
        Empty,
        LastClaim,
        Available,
        Cancel,
        Contended,
    }

    fn finish_poll<F: Future>(
        mut future: std::pin::Pin<&mut F>,
        cx: &mut std::task::Context<'_>,
        deadline: Instant,
    ) -> F::Output {
        loop {
            if let std::task::Poll::Ready(result) = future.as_mut().poll(cx) {
                return result;
            }
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
    }

    fn poll_while_service_is_parked<F: Future>(
        mut request: std::pin::Pin<&mut F>,
        cx: &mut std::task::Context<'_>,
    ) -> Option<F::Output> {
        for _ in 0..2048 {
            if let std::task::Poll::Ready(result) = request.as_mut().poll(cx) {
                return Some(result);
            }
            thread::yield_now();
        }
        None
    }

    fn poll_until_dispatch<F: Future>(
        mut request: std::pin::Pin<&mut F>,
        cx: &mut std::task::Context<'_>,
        service: &pressure_turn::Service,
        earlier: usize,
        deadline: Instant,
    ) -> Option<F::Output> {
        loop {
            if let std::task::Poll::Ready(result) = request.as_mut().poll(cx) {
                return Some(result);
            }
            if service.snapshot() > earlier {
                return None;
            }
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
    }

    #[expect(
        clippy::expect_used,
        clippy::unwrap_in_result,
        reason = "forced primary callback schedule"
    )]
    fn parked_primary(
        donor_size: usize,
    ) -> Result<(Arc<GarbageCollector>, mpsc::Sender<()>, usize)> {
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let older = Peer {
            count: Arc::new(AtomicUsize::new(0)),
            entered,
            resume: Arc::new(Mutex::new(release)),
        };
        let cap = Gc::allocation_size(&older)? + donor_size;
        let heap = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(cap)
                .threads(1)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        drop(Gc::try_new(&heap, older)?);
        heap.start();
        heap.collect();
        parked
            .recv_timeout(Duration::from_secs(3))
            .expect("primary entered");
        Ok((heap, resume, cap))
    }

    #[expect(
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        clippy::expect_used,
        reason = "bounded service, exact heap charges and physical refund"
    )]
    fn delayed_pressure(schedule: PressureSchedule) -> Result<()> {
        use std::task::{Context, Waker};
        let drops = Arc::new(AtomicUsize::new(0));
        let donor = Donor(drops.clone());
        let (heap, resume, cap) = parked_primary(Gc::allocation_size(&donor)?)?;
        let empty = matches!(schedule, PressureSchedule::Empty);
        let retained = empty.then_some(Gc::try_new(&heap, donor)?);
        let mut cycle = std::pin::pin!(heap.collect_full_async());
        let mut cx = Context::from_waker(Waker::noop());
        let deadline = Instant::now() + Duration::from_secs(4);
        finish_poll(cycle.as_mut(), &mut cx, deadline)?;
        while heap.callback_jobs.load(Ordering::Acquire) != if empty { 1 } else { 2 } {
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        let pool = heap.worker_signals.load_full().expect("pool");
        let runner = pool.callbacks.0.get().expect("runner");
        let contended = matches!(schedule, PressureSchedule::Contended);
        let receiver = contended.then(|| lock(&runner.notices));
        let (notified, seen) = mpsc::sync_channel(1);
        let (allow, assistant) = mpsc::channel();
        *lock(&runner.pressure_gate) = Some((notified, assistant));
        let mut request = Box::pin(Gc::try_new_async(&heap, 11_u64));
        let mut early =
            poll_until_dispatch(request.as_mut(), &mut cx, &runner.service, 0, deadline);
        if empty || matches!(schedule, PressureSchedule::LastClaim) {
            seen.recv_timeout(Duration::from_secs(3))
                .expect("assistant stopped before probing");
            let last = (!empty).then(|| {
                assert!(early.is_none() && runner.service.snapshot() > 0);
                runner.inbox.jobs.claim().expect("last published callback")
            });
            early = early.or_else(|| poll_while_service_is_parked(request.as_mut(), &mut cx));
            let turn_count = runner.service.snapshot();
            let charged = heap.managed_bytes();
            let destroyed = drops.load(Ordering::Acquire);
            drop(request);
            if let Some(job) = last {
                assert!(!job.run());
            }
            drop(retained);
            allow.send(()).expect("resume assistant");
            resume.send(()).expect("resume primary");
            heap.stop()?;
            assert!(
                matches!(early, Some(Err(crate::Error::OutOfMemory))),
                "allocator kept polling a parked assistant with no published job"
            );
            assert_eq!(turn_count == 0, empty);
            assert_eq!((charged, destroyed), (cap, 0));
            assert_eq!(drops.load(Ordering::Acquire), 1);
            assert_eq!(heap.managed_bytes(), 0);
            return Ok(());
        }
        if matches!(schedule, PressureSchedule::Cancel) && early.is_none() {
            drop(request);
            assert!(!heap.grant_registry.has_demand());
            let preceding = runner.service.snapshot();
            request = Box::pin(Gc::try_new_async(&heap, 11_u64));
            early = poll_until_dispatch(
                request.as_mut(),
                &mut cx,
                &runner.service,
                preceding,
                deadline,
            );
        }
        seen.recv_timeout(Duration::from_secs(3))
            .expect("assistant saw demand");
        let pending = early.is_none();
        let charged = heap.managed_bytes();
        let before = drops.load(Ordering::Acquire);
        allow.send(()).expect("resume assistant");
        let result = if let Some(result) = early {
            result
        } else {
            // Re-poll only once the donor has been destroyed and its charge refunded.
            while heap.callback_jobs.load(Ordering::Acquire) == 2 {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
            finish_poll(request.as_mut(), &mut cx, deadline)
        };
        drop(receiver);
        resume.send(()).expect("resume primary");
        let value = result.map(|value| **value);
        heap.stop()?;
        assert!(
            pending,
            "allocation closed before the pressure worker could dispatch"
        );
        assert_eq!((charged, before), (cap, 0));
        assert_eq!(value?, 11);
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(heap.managed_bytes(), 0);
        Ok(())
    }

    struct Peer {
        count: Arc<AtomicUsize>,
        entered: mpsc::Sender<()>,
        resume: Arc<Mutex<mpsc::Receiver<()>>>,
    }

    #[test]
    fn primary_rechecks_linked_jobs_while_their_publisher_is_parked_before_notification() {
        let inbox = Inbox {
            jobs: Pages::new(1),
            publishing: AtomicUsize::new(0),
        };
        inbox.publishing.fetch_or(ARMED, Ordering::AcqRel);
        let (notice, waiting) = mpsc::sync_channel(1);
        let (linked, seen) = mpsc::channel();
        let (continue_link, advance) = mpsc::channel();
        let (finish, release) = mpsc::channel();
        let (obtained, completed) = mpsc::channel();
        thread::scope(|scope| {
            let queue = &inbox;
            scope.spawn(move || {
                queue.publish_observed(
                    &notice,
                    0,
                    1_u8,
                    std::iter::once(2),
                    || {
                        advance
                            .recv_timeout(Duration::from_secs(5))
                            .expect("resume linking");
                    },
                    || {
                        linked.send(()).expect("observer");
                        release
                            .recv_timeout(Duration::from_secs(5))
                            .expect("resume notification");
                    },
                );
            });
            // Consume the initial reservation notice before any work is visible.
            waiting
                .recv_timeout(Duration::from_secs(5))
                .expect("reservation notified");
            assert!(inbox.jobs.claim().is_none());
            continue_link.send(()).expect("publisher");
            seen.recv_timeout(Duration::from_secs(5))
                .expect("public batch");
            scope.spawn(move || {
                let first = queue.next_job(&waiting);
                obtained
                    .send((first, queue.jobs.claim()))
                    .expect("observer");
            });
            let result = completed.recv_timeout(Duration::from_secs(2));
            finish.send(()).expect("publisher");
            assert_eq!(result, Ok((Some(1), Some(2))));
        });
    }

    const PEERS: usize = 128;
    struct Counted(Arc<[AtomicUsize; PEERS]>, usize);
    // SAFETY: only a host counter and ordinal are retained.
    #[expect(unsafe_code)]
    unsafe impl Trace for Counted {
        fn trace(&self, _: &GarbageCollector) {}
    }
    #[expect(clippy::expect_used, reason = "one counter per fixture peer")]
    impl Drop for Counted {
        fn drop(&mut self) {
            self.0
                .get(self.1)
                .expect("valid ordinal")
                .fetch_add(1, Ordering::Release);
        }
    }

    fn capped_request(
        heap: &Arc<GarbageCollector>,
        mut pending: impl FnMut(),
    ) -> Option<crate::GcRootGuard<u64>> {
        use std::task::{Context, Poll, Waker};
        let mut allocation = Box::pin(Gc::try_new_async(heap, 11_u64));
        let mut cx = Context::from_waker(Waker::noop());
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match allocation.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result.ok(),
                Poll::Pending if Instant::now() < deadline => {
                    pending();
                    thread::yield_now();
                }
                Poll::Pending => return None,
            }
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "bounded assistant and physical refund schedule"
    )]
    fn request_with_parked_assistant(
        heap: &Arc<GarbageCollector>,
        runner: &Runner,
        outstanding: usize,
        charged: usize,
    ) -> Option<crate::GcRootGuard<u64>> {
        let (entered, service) = mpsc::sync_channel(1);
        let (allow, release) = mpsc::channel();
        *lock(&runner.pressure_gate) = Some((entered, release));
        let mut examined = false;
        let request = capped_request(heap, || {
            if examined || runner.service.snapshot() == 0 {
                return;
            }
            service
                .recv_timeout(Duration::from_secs(3))
                .expect("pressure signal received");
            assert_eq!(heap.callback_jobs.load(Ordering::Acquire), outstanding);
            assert_eq!(heap.managed_bytes(), charged);
            examined = true;
            allow.send(()).expect("resume pressure worker");
            let deadline = Instant::now() + Duration::from_secs(3);
            while heap.callback_jobs.load(Ordering::Acquire) == outstanding {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
        });
        assert!(examined);
        request
    }

    fn parked_claim(after: bool, at_cap: bool) -> Result<(usize, usize, bool)> {
        parked_pressure(after, at_cap, false)
    }
    #[expect(
        clippy::expect_used,
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        reason = "bounded claim schedule and exact physical-retirement assertions"
    )]
    fn parked_pressure(
        after: bool,
        at_cap: bool,
        stop_service: bool,
    ) -> Result<(usize, usize, bool)> {
        use std::task::{Context, Poll, Waker};
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let older = Peer {
            count: Arc::new(AtomicUsize::new(0)),
            entered,
            resume: Arc::new(Mutex::new(release)),
        };
        let older_charge = Gc::allocation_size(&older)?;
        let counts = Arc::new(std::array::from_fn(|_| AtomicUsize::new(0)));
        let peers: Vec<_> = (0..PEERS).map(|id| Counted(counts.clone(), id)).collect();
        let peer_charge = Gc::allocation_size(peers.first().expect("nonempty peer cohort"))?;
        let heap = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(older_charge + PEERS * peer_charge)
                .threads(1)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        drop(Gc::try_new(&heap, older)?);
        heap.start();
        heap.collect();
        parked
            .recv_timeout(Duration::from_secs(4))
            .expect("primary callback entered");
        for peer in peers {
            drop(Gc::try_new(&heap, peer)?);
        }
        let mut ticket = Box::pin(heap.collect_full_async());
        let mut cx = Context::from_waker(Waker::noop());
        let deadline = Instant::now() + Duration::from_secs(4);
        loop {
            if let Poll::Ready(result) = ticket.as_mut().poll(&mut cx) {
                result?;
                break;
            }
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        while heap.callback_jobs.load(Ordering::Acquire) != PEERS + 1 {
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        let pool = heap
            .worker_signals
            .load_full()
            .expect("active native service");
        let (claimed, suspended) = mpsc::channel();
        let (finish, continue_claim) = mpsc::channel();
        let other = pool.clone();
        let stalled = thread::spawn(move || {
            claim_pause::PAUSE.with_borrow_mut(|p| {
                *p = Some(claim_pause::Pause {
                    after,
                    entered: claimed,
                    release: continue_claim,
                });
            });
            other.callbacks.help_ready(1);
        });
        suspended
            .recv_timeout(Duration::from_secs(4))
            .expect("claimant suspended");
        // A real allocation at the cap has to obtain storage from a peer
        // callback before the parked consumer resumes. It cannot use the
        // older destructor or any job already taken by the parked claimant.
        let request = if stop_service {
            request_with_parked_assistant(
                &heap,
                pool.callbacks.0.get().expect("runner"),
                PEERS + 1,
                older_charge + peer_charge * PEERS,
            )
        } else {
            at_cap.then(|| capped_request(&heap, || {})).flatten()
        };
        pool.callbacks.help_ready(PEERS);
        let completed = counts
            .iter()
            .filter(|v| v.load(Ordering::Acquire) == 1)
            .count();
        let funded = request.as_ref().is_some_and(|value| ***value == 11);
        let request_charge = usize::from(funded) * Gc::allocation_size(&11_u64)?;
        let charged_peers = (heap.managed_bytes() - older_charge - request_charge) / peer_charge;
        finish.send(()).expect("resume claimant");
        stalled.join().expect("claimant joined");
        resume.send(()).expect("resume primary callback");
        drop(request);
        heap.stop()?;
        assert_eq!(heap.managed_bytes(), 0);
        assert!(counts.iter().all(|v| v.load(Ordering::Acquire) == 1));
        Ok((completed, charged_peers, funded))
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify the physical refund after dispatch"
    )]
    fn terminal_allocation_observes_a_runnable_pressure_assistant_before_closing() -> Result<()> {
        assert_eq!(parked_pressure(false, true, true)?, (128, 0, true));
        Ok(())
    }
    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "verify the forced claim outcome")]
    fn independent_callbacks_remain_available_before_a_parked_claim() -> Result<()> {
        assert_eq!(parked_claim(false, false)?, (128, 0, false));
        Ok(())
    }

    #[test]
    #[expect(clippy::panic_in_result_fn, reason = "verify the forced claim outcome")]
    fn independent_callbacks_remain_available_after_a_parked_claim() -> Result<()> {
        assert_eq!(parked_claim(true, false)?, (127, 1, false));
        Ok(())
    }
    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify physical allocation at the cap"
    )]
    fn capped_allocation_progresses_before_a_parked_callback_claim() -> Result<()> {
        assert!(parked_claim(false, true)?.2);
        Ok(())
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        reason = "verify physical allocation at the cap"
    )]
    fn capped_allocation_progresses_after_a_parked_callback_claim() -> Result<()> {
        assert!(parked_claim(true, true)?.2);
        Ok(())
    }

    // SAFETY: the callback controls have no managed edges.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Peer {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> TraceProgress {
            assert_ne!(
                self.count.load(Ordering::Acquire),
                1,
                "intentional second clearing failure"
            );
            TraceProgress::Complete
        }
    }
    #[expect(
        clippy::expect_used,
        reason = "the first destructor participates in the forced shutdown schedule"
    )]
    impl Drop for Peer {
        fn drop(&mut self) {
            if self.count.fetch_add(1, Ordering::AcqRel) == 0 {
                self.entered.send(()).expect("first drop observer");
                lock(&self.resume)
                    .recv_timeout(Duration::from_secs(5))
                    .expect("resume first drop");
            }
        }
    }

    #[test]
    #[expect(
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        reason = "a closed schedule channel fails the forced shutdown test"
    )]
    fn disconnection_drains_the_accepted_suffix_after_its_first_callback_parks() -> Result<()> {
        let heap = GarbageCollector::with_config(
            Configuration::builder()
                .incremental_step_size(1)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let count = Arc::new(AtomicUsize::new(0));
        let (entered, parked) = mpsc::channel();
        let (start, resume) = mpsc::channel();
        let resume = Arc::new(Mutex::new(resume));
        for _ in 0..3 {
            drop(Gc::try_new(
                &heap,
                Peer {
                    count: count.clone(),
                    entered: entered.clone(),
                    resume: resume.clone(),
                },
            )?);
        }
        // Capture three real swept tasks before submitting a single channel message.
        heap.collect();
        let mut captured = Vec::new();
        for _ in 0..16_384 {
            heap.step();
            let mut cycle = lock(&heap.domains[1].cycle);
            captured.extend(cycle.reclaim.drain(..).map(|task| (task, 1)));
            if captured.len() == 3 {
                break;
            }
        }
        assert_eq!(captured.len(), 3);
        heap.retired_ready.fetch_add(3, Ordering::AcqRel);
        let callbacks = Arc::new(Callbacks::default());
        let mut captured = captured.into_iter();
        assert!(
            callbacks
                .send(
                    &heap,
                    Batch {
                        first: captured.next().expect("first task"),
                        rest: captured.collect(),
                    },
                    None
                )
                .is_none()
        );
        parked
            .recv_timeout(Duration::from_secs(5))
            .expect("callback parked");
        let pool = callbacks.clone();
        let closer = thread::spawn(move || pool.join());
        let runner = callbacks.0.get().expect("callback runner");
        let deadline = Instant::now() + Duration::from_secs(3);
        while runner.send.load().is_some() {
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        {
            let notices = lock(&runner.notices);
            loop {
                match notices.try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    Err(mpsc::TryRecvError::Empty) | Ok(()) => {
                        assert!(Instant::now() < deadline);
                        thread::yield_now();
                    }
                }
            }
            assert_eq!(runner.inbox.jobs.remaining_in_head(), 2);
        }
        start.send(()).expect("release first drop");
        let failed = closer.join().expect("join owner")?;
        assert!(failed, "the second callback's panic must reach shutdown");
        assert_eq!(count.load(Ordering::Acquire), 3);
        assert_eq!(heap.callback_jobs.load(Ordering::Acquire), 0);
        assert_eq!(heap.managed_bytes(), 0);
        heap.stop()?;
        Ok(())
    }
}
