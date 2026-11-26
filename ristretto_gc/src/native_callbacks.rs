//! Native finalizer work remains singly owned while the marking workers serve
//! later tickets. Each running pool opens its callback thread on first use.
use super::{GarbageCollector, ReclaimTask, WorkCounters, lock, now};
use crate::TraceProgress;
use once_cell::race::OnceBox;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

thread_local! {
    // Native workers are owned by exactly one pool generation for their lifetime.
    static POOL: std::cell::OnceCell<std::sync::Weak<super::WorkerSignals>> = const { std::cell::OnceCell::new() };
    static CALLBACK: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn register(pool: &Arc<super::WorkerSignals>) {
    POOL.with(|current| {
        let _ = current.set(Arc::downgrade(pool));
    });
}

pub(super) fn pool() -> Option<Arc<super::WorkerSignals>> {
    POOL.with(|current| current.get().and_then(std::sync::Weak::upgrade))
}

pub(super) fn executing(gc: &GarbageCollector) -> bool {
    CALLBACK.get() == std::ptr::from_ref(gc).addr()
}

#[derive(Debug, Default)]
pub(super) struct Callbacks(OnceBox<Runner>);

#[derive(Debug)]
struct Runner {
    send: arc_swap::ArcSwapOption<mpsc::Sender<Job>>,
    thread: Mutex<Option<thread::JoinHandle<crate::Result<()>>>>,
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
        task: ReclaimTask,
        budget: usize,
        origin: Option<(usize, u64)>,
    ) -> Option<ReclaimTask> {
        let Ok(runner) = self.0.get_or_try_init(Runner::spawn) else {
            return Some(task);
        };
        let Some(owner) = gc.this.upgrade() else {
            return Some(task);
        };
        let Some(sender) = runner.send.load_full() else {
            return Some(task);
        };
        gc.callback_jobs.fetch_add(1, Ordering::AcqRel);
        let job = Job {
            gc: owner,
            task,
            budget,
            origin,
        };
        if let Err(mpsc::SendError(job)) = sender.send(job) {
            gc.callback_jobs.fetch_sub(1, Ordering::AcqRel);
            return Some(job.task);
        }
        None
    }

    pub(super) fn join(&self) -> crate::Result<()> {
        if let Some(runner) = self.0.get() {
            // Disconnect after the pool stops. A sender already holding its own
            // handle can finish publication; the receiver drains all accepted jobs.
            runner.send.store(None);
            if let Some(handle) = lock(&runner.thread).take()
                && handle.thread().id() != thread::current().id()
            {
                handle.join().map_err(|_| {
                    crate::Error::SyncError("collector callback worker panicked".into())
                })??;
            }
        }
        Ok(())
    }
}

impl Runner {
    fn spawn() -> std::io::Result<Box<Self>> {
        let (send, recv) = mpsc::channel::<Job>();
        let thread = thread::Builder::new()
            .name("gc-finalizers".into())
            .spawn(move || {
                let mut failed = false;
                while let Ok(job) = recv.recv() {
                    let Job {
                        gc,
                        mut task,
                        budget,
                        origin,
                    } = job;
                    CALLBACK.set(Arc::as_ptr(&gc).addr());
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
                    }));
                    CALLBACK.set(0);
                    gc.callback_jobs.fetch_sub(1, Ordering::AcqRel);
                    if result.is_err() {
                        failed = true;
                        gc.recovery.callback_failed();
                    }
                    gc.wake();
                    gc.notify_progress();
                }
                if failed {
                    Err(crate::Error::SyncError(
                        "collector callback panicked".into(),
                    ))
                } else {
                    Ok(())
                }
            })?;
        Ok(Box::new(Self {
            send: arc_swap::ArcSwapOption::from(Some(Arc::new(send))),
            thread: Mutex::new(Some(thread)),
        }))
    }
}
