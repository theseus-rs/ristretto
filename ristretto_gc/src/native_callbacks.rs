//! Native finalizer work remains singly owned while the marking workers serve
//! later tickets. Each running pool opens its callback thread on first use.
use super::{GarbageCollector, ReclaimTask, WorkCounters, lock, now};
use crate::TraceProgress;
use once_cell::race::OnceBox;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

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

#[derive(Debug)]
struct Runner {
    send: arc_swap::ArcSwapOption<mpsc::Sender<Submission>>,
    receive: Arc<Mutex<Receiver>>,
    helper_failed: AtomicBool,
    thread: Mutex<Option<thread::JoinHandle<bool>>>,
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
    pub(super) fn send(
        &self,
        gc: &GarbageCollector,
        batch: Batch,
        origin: Option<(usize, u64)>,
    ) -> Option<Batch> {
        let Ok(runner) = self.0.get_or_try_init(Runner::spawn) else {
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
        None
    }

    pub(super) fn join(&self) -> crate::Result<bool> {
        if let Some(runner) = self.0.get() {
            // Disconnect after the pool stops. A sender already holding its own
            // handle can finish publication; the receiver drains all accepted jobs.
            runner.send.store(None);
            if let Some(handle) = lock(&runner.thread).take()
                && handle.thread().id() != thread::current().id()
            {
                let failed = handle.join().map_err(|_| {
                    crate::Error::SyncError("collector callback worker panicked".into())
                })?;
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
    fn spawn() -> std::io::Result<Box<Self>> {
        let (send, submissions) = mpsc::channel();
        let receive = Arc::new(Mutex::new(Receiver {
            submissions,
            remaining: None,
        }));
        let queue = receive.clone();
        let thread = thread::Builder::new()
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
        Ok(Box::new(Self {
            send: arc_swap::ArcSwapOption::from(Some(Arc::new(send))),
            receive,
            helper_failed: AtomicBool::new(false),
            thread: Mutex::new(Some(thread)),
        }))
    }
}

#[cfg(all(test, not(miri), panic = "unwind"))]
mod tests {
    use super::*;
    use crate::{Configuration, Gc, Result, Trace};
    use std::sync::atomic::AtomicUsize;
    use std::time::{Duration, Instant};

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
