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
    send: arc_swap::ArcSwapOption<mpsc::Sender<Job>>,
    receive: Arc<Mutex<mpsc::Receiver<Job>>>,
    helper_failed: AtomicBool,
    thread: Mutex<Option<thread::JoinHandle<bool>>>,
}

pub(super) struct Batch {
    pub(super) first: (ReclaimTask, usize),
    pub(super) rest: Vec<(ReclaimTask, usize)>,
}

struct Job {
    gc: Arc<GarbageCollector>,
    batch: Batch,
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
        gc.callback_jobs.fetch_add(1, Ordering::AcqRel);
        let job = Job {
            gc: owner,
            batch,
            origin,
        };
        if let Err(mpsc::SendError(job)) = sender.send(job) {
            gc.callback_jobs.fetch_sub(1, Ordering::AcqRel);
            return Some(job.batch);
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
            let Ok(receiver) = runner.receive.try_lock() else {
                break;
            };
            let Ok(job) = receiver.try_recv() else { break };
            drop(receiver);
            if job.run() {
                runner.helper_failed.store(true, Ordering::Release);
            }
        }
    }
}

impl Job {
    fn run(self) -> bool {
        let Self { gc, batch, origin } = self;
        let _cleanup = super::CleanupScope::enter(&gc);
        let mut failed = false;
        for (mut task, budget) in std::iter::once(batch.first).chain(batch.rest) {
            let mut counters = WorkCounters::default();
            let started = now();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let progress = gc.clear_retired(&mut task, budget, &mut counters, None);
                if progress == TraceProgress::Complete {
                    if let Some((domain, epoch)) = origin {
                        gc.record_sweep_reclaimed(domain, epoch, task.metadata.size());
                    }
                } else {
                    task.advance(progress);
                    gc.queue_clearing(task);
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
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| gc.notify_progress()))
                .is_err()
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
        }
        if gc.callback_jobs.fetch_sub(1, Ordering::AcqRel) == 1 {
            gc.wake();
        }
        failed
    }
}

impl Runner {
    fn spawn() -> std::io::Result<Box<Self>> {
        let (send, recv) = mpsc::channel::<Job>();
        let receive = Arc::new(Mutex::new(recv));
        let queue = receive.clone();
        let thread = thread::Builder::new()
            .name("gc-finalizers".into())
            .spawn(move || {
                let mut failed = false;
                loop {
                    let next = lock(&queue).recv();
                    let Ok(job) = next else { break };
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
