//! Native finalizer work remains singly owned while the marking workers serve
//! later tickets. Each running pool opens its callback thread on first use.
use super::{GarbageCollector, ReclaimTask, WorkCounters, lock, now};
use crate::TraceProgress;
use once_cell::race::OnceBox;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[path = "pressure_turn.rs"]
mod pressure_turn;
pub(super) use pressure_turn::Turn;

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
    send: arc_swap::ArcSwapOption<mpsc::Sender<Submission>>,
    receive: Arc<Mutex<Receiver>>,
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

struct Submission {
    gc: Arc<GarbageCollector>,
    batch: Batch,
    origin: Option<(usize, u64)>,
}

struct Remaining {
    gc: Arc<GarbageCollector>,
    entries: std::vec::IntoIter<(ReclaimTask, usize)>,
    origin: Option<(usize, u64)>,
}

struct Receiver {
    submissions: mpsc::Receiver<Submission>,
    remaining: Option<Remaining>,
}

impl std::fmt::Debug for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let remaining = self
            .remaining
            .as_ref()
            .map_or(0, |batch| batch.entries.len());
        f.debug_struct("Receiver")
            .field("remaining", &remaining)
            .finish_non_exhaustive()
    }
}

impl Receiver {
    // Many helpers may share a batch. Move exactly one item into the caller's
    // job while its unclaimed siblings remain accessible behind this guard.
    fn claim(&mut self, wait: bool) -> Option<Job> {
        if let Some(batch) = &mut self.remaining {
            let (task, budget) = batch.entries.next()?;
            let job = Job {
                gc: batch.gc.clone(),
                task,
                budget,
                origin: batch.origin,
            };
            if batch.entries.len() == 0 {
                self.remaining = None;
            }
            return Some(job);
        }
        let Submission { gc, batch, origin } = if wait {
            self.submissions.recv().ok()?
        } else {
            self.submissions.try_recv().ok()?
        };
        if !batch.rest.is_empty() {
            self.remaining = Some(Remaining {
                gc: gc.clone(),
                entries: batch.rest.into_iter(),
                origin,
            });
        }
        let (task, budget) = batch.first;
        Some(Job {
            gc,
            task,
            budget,
            origin,
        })
    }
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
        let turn = runner.service.request()?;
        let _ = runner.pressure.try_send(());
        Some(turn)
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
        let Ok(runner) = self.0.get_or_try_init(|| Runner::spawn(gc.this.clone())) else {
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
        let submission = Submission {
            gc: owner,
            batch,
            origin,
        };
        if let Err(mpsc::SendError(submission)) = sender.send(submission) {
            gc.callback_jobs.fetch_sub(count, Ordering::AcqRel);
            return Some(submission.batch);
        }
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
            let Ok(mut receiver) = runner.receive.try_lock() else {
                break;
            };
            let Some(job) = receiver.claim(false) else {
                break;
            };
            drop(receiver);
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
    fn spawn(heap: std::sync::Weak<GarbageCollector>) -> std::io::Result<Box<Self>> {
        let (send, submissions) = mpsc::channel();
        let receive = Arc::new(Mutex::new(Receiver {
            submissions,
            remaining: None,
        }));
        let queue = receive.clone();
        let primary = thread::Builder::new()
            .name("gc-finalizers".into())
            .spawn(move || {
                let mut failed = false;
                loop {
                    let next = lock(&queue).claim(true);
                    let Some(job) = next else { break };
                    failed |= job.run();
                }
                failed
            })?;
        let (pressure, requested) = mpsc::sync_channel(1);
        let closing = Arc::new(AtomicBool::new(false));
        let stopping = closing.clone();
        let queue = receive.clone();
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
                let dispatch = dispatch;
                let mut failed = false;
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
                        // An idle primary can be asleep inside the receiver.
                        // Report this probe without waiting for its mutex.
                        let next =
                            super::try_lock(&queue).and_then(|mut receiver| receiver.claim(false));
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
            receive,
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
    fn a_contended_callback_receiver_cannot_hold_terminal_allocation_open() -> Result<()> {
        delayed_pressure(PressureSchedule::Contended)
    }

    #[derive(Clone, Copy)]
    enum PressureSchedule {
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
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        clippy::expect_used,
        reason = "bounded service, exact heap charges and physical refund"
    )]
    fn delayed_pressure(schedule: PressureSchedule) -> Result<()> {
        use std::task::{Context, Waker};
        let (entered, parked) = mpsc::channel();
        let (resume, release) = mpsc::channel();
        let older = Peer {
            count: Arc::new(AtomicUsize::new(0)),
            entered,
            resume: Arc::new(Mutex::new(release)),
        };
        let drops = Arc::new(AtomicUsize::new(0));
        let donor = Donor(drops.clone());
        let cap = Gc::allocation_size(&older)? + Gc::allocation_size(&donor)?;
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
        drop(Gc::try_new(&heap, donor)?);
        let mut cycle = std::pin::pin!(heap.collect_full_async());
        let mut cx = Context::from_waker(Waker::noop());
        let deadline = Instant::now() + Duration::from_secs(4);
        finish_poll(cycle.as_mut(), &mut cx, deadline)?;
        while heap.callback_jobs.load(Ordering::Acquire) != 2 {
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        let pool = heap.worker_signals.load_full().expect("pool");
        let runner = pool.callbacks.0.get().expect("runner");
        let contended = matches!(schedule, PressureSchedule::Contended);
        let receiver = contended.then(|| lock(&runner.receive));
        let (notified, seen) = mpsc::sync_channel(1);
        let (allow, assistant) = mpsc::channel();
        *lock(&runner.pressure_gate) = Some((notified, assistant));
        let mut request = Box::pin(Gc::try_new_async(&heap, 11_u64));
        let mut early =
            poll_until_dispatch(request.as_mut(), &mut cx, &runner.service, 0, deadline);
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
            while !contended && heap.callback_jobs.load(Ordering::Acquire) == 2 {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
            finish_poll(request.as_mut(), &mut cx, deadline)
        };
        if contended {
            assert_eq!(heap.managed_bytes(), cap);
            assert_eq!(drops.load(Ordering::Acquire), 0);
        }
        drop(receiver);
        resume.send(()).expect("resume primary");
        let value = result.map(|value| **value);
        heap.stop()?;
        assert!(
            pending,
            "allocation closed before the pressure worker could dispatch"
        );
        assert_eq!((charged, before), (cap, 0));
        if contended {
            assert!(matches!(value, Err(crate::Error::OutOfMemory)));
        } else {
            assert_eq!(value?, 11);
        }
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(heap.managed_bytes(), 0);
        Ok(())
    }

    struct Peer {
        count: Arc<AtomicUsize>,
        entered: mpsc::Sender<()>,
        resume: Arc<Mutex<mpsc::Receiver<()>>>,
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
            let receiver = lock(&runner.receive);
            // ArcSwap may have published None just before its displaced sender
            // drops. Observe the channel's own disconnection while the first task is parked.
            loop {
                match receiver.submissions.try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    Err(mpsc::TryRecvError::Empty) => {
                        assert!(Instant::now() < deadline);
                        thread::yield_now();
                    }
                    Ok(_) => panic!("the test submitted exactly one batch"),
                }
            }
            assert_eq!(
                receiver
                    .remaining
                    .as_ref()
                    .expect("accepted suffix")
                    .entries
                    .len(),
                2
            );
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
