//! Bounded worker-local relocation queues. Claims are protected until reserved.
use crate::metadata::MetadataRef;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, TryLockError};

const LOCAL_LIMIT: usize = 64;

#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) metadata: MetadataRef,
    pub(crate) attempts: usize,
}

#[derive(Debug, Default)]
#[repr(align(64))]
struct Local {
    queue: Mutex<VecDeque<Task>>,
    victim: AtomicUsize,
}

#[derive(Debug)]
pub(crate) struct Queues {
    queues: Box<[Local]>,
    queued: AtomicUsize,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[expect(
    clippy::indexing_slicing,
    reason = "indices are reduced to nonempty queue count"
)]
impl Queues {
    pub(crate) fn new(workers: usize) -> Self {
        Self {
            queues: (0..=workers.min(16)).map(|_| Local::default()).collect(),
            queued: AtomicUsize::new(0),
        }
    }

    pub(crate) fn worker_queue(&self, worker: usize) -> usize {
        if self.queues.len() == 1 {
            0
        } else {
            1 + worker % (self.queues.len() - 1)
        }
    }

    pub(crate) fn push(&self, local: usize, task: Task) {
        let local = local % self.queues.len();
        let mut queue = lock(&self.queues[local].queue);
        if local != 0 && queue.len() >= LOCAL_LIMIT {
            drop(queue);
            queue = lock(&self.queues[0].queue);
        }
        queue.push_back(task);
        self.queued.fetch_add(1, Ordering::Release);
    }

    /// Probe each source once. The caller either holds the claim reader until
    /// reserving in-flight work, or already owns its batch's reservation.
    pub(crate) fn claim(&self, local: usize) -> (Option<Task>, usize, usize) {
        let local = local % self.queues.len();
        let start = self.queues[local].victim.load(Ordering::Relaxed) % self.queues.len();
        let sources = std::iter::once(local)
            .chain((local != 0).then_some(0))
            .chain(
                (0..self.queues.len())
                    .map(|offset| (start + offset) % self.queues.len())
                    .filter(|&index| index != local && index != 0),
            );
        let mut misses = 0;
        for source in sources {
            let mut queue = match self.queues[source].queue.try_lock() {
                Ok(queue) => queue,
                Err(TryLockError::Poisoned(error)) => error.into_inner(),
                Err(TryLockError::WouldBlock) => {
                    misses += 1;
                    continue;
                }
            };
            let task = if source == local {
                queue.pop_front()
            } else {
                queue.pop_back()
            };
            if let Some(task) = task {
                self.queued.fetch_sub(1, Ordering::AcqRel);
                self.queues[local]
                    .victim
                    .store(source + 1, Ordering::Relaxed);
                return (Some(task), usize::from(source != local), misses);
            }
        }
        (None, 0, misses)
    }

    /// Exact only when the coordinator excludes enumeration and the claim writer
    /// excludes admission, with both generations' in-flight work equal to zero.
    pub(crate) fn len(&self) -> usize {
        self.queued.load(Ordering::Acquire)
    }

    pub(crate) fn capacity(&self) -> usize {
        self.queues
            .iter()
            .map(|local| lock(&local.queue).capacity())
            .sum()
    }

    pub(crate) fn trim_empty(&self) {
        for local in &self.queues {
            let mut queue = lock(&local.queue);
            if queue.is_empty() {
                let retired = std::mem::take(&mut *queue);
                drop(queue);
                drop(retired);
            }
        }
    }

    pub(crate) fn clear(&self) {
        for local in &self.queues {
            let retired = std::mem::take(&mut *lock(&local.queue));
            self.queued.fetch_sub(retired.len(), Ordering::AcqRel);
            drop(retired);
        }
    }
}
