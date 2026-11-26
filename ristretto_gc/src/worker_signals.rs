//! Demand belongs to the pool generation. Each worker acknowledges its own
//! bit, so a peer going idle cannot consume its notification.
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[derive(Debug)]
pub(super) struct WorkerSignals {
    pub(super) closed: AtomicBool,
    pub(super) workers: Box<[Signal]>,
    pub(super) callbacks: super::native_callbacks::Callbacks,
}

impl WorkerSignals {
    pub(super) fn new(count: usize) -> Self {
        Self {
            closed: AtomicBool::new(false),
            callbacks: super::native_callbacks::Callbacks::default(),
            workers: (0..count).map(|_| Signal::default()).collect(),
        }
    }

    pub(super) fn wake(&self) {
        for worker in &self.workers {
            // An RMW keeps preceding coalesced publishers in the release sequence.
            worker.pending.swap(true, Ordering::Release);
            // Linux implements unpark with an atomic and a futex wake. Other
            // native targets keep the publishing thread out of platform parkers.
            #[cfg(target_os = "linux")]
            if let Some(thread) = worker.thread.get() {
                thread.unpark();
            }
        }
    }

    pub(super) fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.wake();
    }
}

#[derive(Debug, Default)]
pub(super) struct Signal {
    pending: AtomicBool,
    #[cfg(target_os = "linux")]
    thread: std::sync::OnceLock<thread::Thread>,
    #[cfg(test)]
    pub(super) waiting: AtomicBool,
}

impl Signal {
    #[cfg_attr(
        not(target_os = "linux"),
        expect(
            clippy::unused_self,
            reason = "the polling fallback has no platform parker"
        )
    )]
    pub(super) fn register(&self) {
        #[cfg(target_os = "linux")]
        self.thread.get_or_init(thread::current);
    }

    pub(super) fn wait(&self) {
        // A Trace callback may consume its thread's park token. Only this
        // check acknowledges demand, immediately before returning to the worker.
        while !self.pending.swap(false, Ordering::AcqRel) {
            #[cfg(test)]
            self.waiting.store(true, Ordering::Release);
            #[cfg(target_os = "linux")]
            thread::park();
            #[cfg(not(target_os = "linux"))]
            thread::sleep(std::time::Duration::from_millis(1));
        }
        #[cfg(test)]
        self.waiting.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier, mpsc};
    use std::time::Duration;

    #[test]
    fn old_pool_wakes_and_a_consumed_park_token_preserve_new_work() {
        let old = WorkerSignals::new(1);
        old.wake();
        let pool = Arc::new(WorkerSignals::new(1));
        let ready = Arc::new(Barrier::new(2));
        let released = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        let consumer = {
            let pool = pool.clone();
            let ready = ready.clone();
            let released = released.clone();
            thread::spawn(move || {
                let worker = pool.workers.first().expect("one worker");
                worker.register();
                ready.wait();
                while !released.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                // Trace code runs on the same OS thread and can consume its park token.
                thread::current().unpark();
                thread::park();
                worker.wait();
                tx.send(pool.closed.load(Ordering::Acquire))
                    .expect("observer");
            })
        };
        ready.wait();
        pool.wake();
        pool.wake();
        old.close();
        released.store(true, Ordering::Release);
        let closed = rx.recv_timeout(Duration::from_secs(5));
        pool.close();
        consumer.join().expect("worker returned");
        assert_eq!(closed, Ok(false));
    }
}
