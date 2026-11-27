//! A clock-free collector workload polled once per browser host turn.
//! No export performs synchronous full collection or waits for a worker.
#![expect(
    unsafe_code,
    reason = "named WebAssembly exports for the local browser harness"
)]
use ristretto_gc::{
    Configuration, ExecutionContext, Finalize, GarbageCollector, Gc, GcRootGuard, GcWeak,
    ReferenceKind, Result, RootStorage, Trace,
};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

type Allocation = Pin<Box<dyn Future<Output = Result<GcRootGuard<Payload>>>>>;
type Collection = Pin<Box<dyn Future<Output = Result<()>>>>;
const FAILED: u32 = u32::MAX;
const LIVE: usize = 128;

struct Payload([u64; 4]);
impl Payload {
    fn new(sequence: u64) -> Self {
        Self([sequence, 42, !sequence, sequence ^ 0x5a5a_1234])
    }
    fn verify(&self, sequence: u64) {
        assert_eq!(self.0, Self::new(sequence).0);
    }
}
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Payload {
    fn trace(&self, _: &GarbageCollector) {}
    fn may_contain_gc_references() -> bool {
        false
    }
}
struct Finalized(Arc<AtomicUsize>);
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Finalized {
    fn trace(&self, _: &GarbageCollector) {}
    fn may_contain_gc_references() -> bool {
        false
    }
}
impl Finalize for Finalized {
    fn finalize(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Running,
    Collecting,
    Checking,
    Done,
}

struct Workload {
    gc: Arc<GarbageCollector>,
    context: Option<ExecutionContext>,
    control: bool,
    pending: Option<Allocation>,
    collection: Option<Collection>,
    roots: VecDeque<(u64, Gc<Payload>)>,
    weak: VecDeque<GcWeak<Payload>>,
    finalized: Arc<AtomicUsize>,
    completed: u64,
    polls: u64,
    steps: u64,
    checksum: u64,
    failed: u64,
    phase: Phase,
}

impl Workload {
    fn new(policy: u32, heap_mib: u32) -> Self {
        let candidate = policy == 1;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(0)
                .allocation_threshold(128 * 1024)
                .max_heap_bytes(heap_mib as usize * 1024 * 1024)
                .incremental_step_size(8)
                .max_pause_time_us(0)
                .root_storage(if candidate {
                    RootStorage::Contexts
                } else {
                    RootStorage::Tree
                })
                .concurrent_publication(candidate)
                .local_allocation_buffers(candidate)
                .barrier_buffer_size(if candidate { 32 } else { 0 })
                // The browser drives every collection unit on this thread;
                // worker claim gates and stealing queues add no parallel work.
                .overlapping_generations(candidate)
                .mark_batch_size(if candidate { 8 } else { 1 })
                .diagnostics(true)
                .build(),
        );
        let context = gc.register_context();
        let finalized = Arc::new(AtomicUsize::new(0));
        if policy != 2 {
            context.with(|| drop(Gc::new_with_finalizer(&gc, Finalized(finalized.clone()))));
            gc.collect();
        }
        Self {
            gc,
            context: Some(context),
            control: policy == 2,
            pending: None,
            collection: None,
            roots: VecDeque::with_capacity(LIVE + 1),
            weak: VecDeque::with_capacity(LIVE + 1),
            finalized,
            completed: 0,
            polls: 0,
            steps: 0,
            checksum: 0,
            failed: 0,
            phase: Phase::Running,
        }
    }

    fn poll(&mut self, sequence: u32) -> u32 {
        if self.phase != Phase::Running || self.failed != 0 || self.completed != u64::from(sequence)
        {
            self.failed += 1;
            return FAILED;
        }
        let Some(context) = self.context.take() else {
            self.failed += 1;
            return FAILED;
        };
        // Match an executor poll: publication, ownership transfers and drops all
        // run inside the same context, which is restored before yielding to JS.
        let result = context.with(|| self.poll_in_context(sequence));
        self.context = Some(context);
        result
    }

    fn poll_in_context(&mut self, sequence: u32) -> u32 {
        self.polls += 1;
        if self.control {
            self.completed += 1;
            self.checksum += 42;
            return 1;
        }
        if self.pending.is_none() {
            let gc = self.gc.clone();
            self.pending = Some(Box::pin(async move {
                Gc::try_new_async(&gc, Payload::new(u64::from(sequence))).await
            }));
        }
        let Some(future) = &mut self.pending else {
            return FAILED;
        };
        let result = future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()));
        let Poll::Ready(result) = result else {
            return 0;
        };
        self.pending = None;
        let Ok(root) = result else {
            self.failed += 1;
            return FAILED;
        };
        root.verify(u64::from(sequence));
        if sequence % 16 == 1 {
            self.weak
                .push_back(GcWeak::new(&root, ReferenceKind::Weak, 0));
            if self.weak.len() > LIVE {
                self.weak.pop_front();
            }
        }
        if sequence.is_multiple_of(8) {
            self.roots
                .push_back((u64::from(sequence), root.into_rooted()));
            if self.roots.len() > LIVE
                && let Some((sequence, old)) = self.roots.pop_front()
            {
                old.verify(sequence);
            }
        }
        self.completed += 1;
        self.checksum += 42;
        1
    }

    fn step(&mut self) -> u32 {
        if self.control {
            return 1;
        }
        self.steps += 1;
        u32::from(self.gc.step())
    }

    fn finish(&mut self) -> u32 {
        if self.phase == Phase::Done {
            return 1;
        }
        if self.phase == Phase::Running {
            self.phase = Phase::Collecting;
        }
        self.pending = None; // Cancel an unfinished request without publishing it.
        for _ in 0..32 {
            if let Some((sequence, root)) = self.roots.pop_front() {
                root.verify(sequence);
            } else {
                break;
            }
        }
        if !self.roots.is_empty() {
            return 0;
        }
        // Detached contexts release cached credits when pending barriers drain.
        self.context = None;
        if self.phase == Phase::Collecting {
            let future = self.collection.get_or_insert_with(|| {
                let gc = self.gc.clone();
                Box::pin(async move {
                    gc.collect_full_async().await?;
                    gc.collect_full_async().await
                })
            });
            let Poll::Ready(result) = future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
            else {
                return 0;
            };
            self.collection = None;
            if result.is_err() {
                self.failed += 1;
                return FAILED;
            }
            self.phase = Phase::Checking;
        }
        for _ in 0..32 {
            if let Some(weak) = self.weak.pop_front() {
                assert!(weak.upgrade().is_none());
            } else {
                break;
            }
        }
        if !self.weak.is_empty() {
            return 0;
        }
        let Ok(stats) = self.gc.statistics() else {
            return FAILED;
        };
        assert_eq!(stats.live_objects, 0);
        assert_eq!(stats.bytes_managed, 0);
        assert_eq!(self.gc.diagnostics().retired_bytes, 0);
        assert_eq!(
            self.finalized.load(Ordering::Relaxed),
            usize::from(!self.control)
        );
        assert_eq!(self.checksum, self.completed * 42);
        self.phase = Phase::Done;
        1
    }

    fn metric(&self, index: u32) -> u64 {
        match index {
            0 => self.completed,
            1 => self.polls,
            2 => self.steps,
            3 => self
                .gc
                .statistics()
                .map_or(u64::MAX, |s| s.bytes_managed as u64),
            4 => self
                .gc
                .statistics()
                .map_or(u64::MAX, |s| s.live_objects as u64),
            5 => self.gc.diagnostics().retired_bytes as u64,
            6 => self.gc.root_statistics().pages as u64,
            7 => self.gc.diagnostics().pending_barrier_records as u64,
            8 => self.finalized.load(Ordering::Relaxed) as u64,
            9 => self.failed,
            10 => self.checksum,
            11 => u64::from(self.pending.is_some()),
            12 => u64::from(self.phase == Phase::Done),
            _ => u64::MAX,
        }
    }
}

thread_local! {
    static WORKLOAD: RefCell<Option<Workload>> = const { RefCell::new(None) };
}

/// Initialize baseline (0), candidate (1), or an allocation-free observer control (2).
#[unsafe(no_mangle)]
pub extern "C" fn gc_browser_init(policy: u32, heap_mib: u32) -> u32 {
    if policy > 2 || !(1..=1024).contains(&heap_mib) {
        return FAILED;
    }
    WORKLOAD.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_some() {
            return FAILED;
        }
        *slot = Some(Workload::new(policy, heap_mib));
        1
    })
}

/// Poll one sequential request. 0 means pending, 1 complete, `u32::MAX` failure.
#[unsafe(no_mangle)]
pub extern "C" fn gc_browser_poll(sequence: u32) -> u32 {
    WORKLOAD.with(|slot| {
        slot.borrow_mut()
            .as_mut()
            .map_or(FAILED, |s| s.poll(sequence))
    })
}

/// Perform one budgeted maintenance step.
#[unsafe(no_mangle)]
pub extern "C" fn gc_browser_step() -> u32 {
    WORKLOAD.with(|slot| slot.borrow_mut().as_mut().map_or(FAILED, Workload::step))
}

/// Cancel unfinished work and poll bounded root release and two full collections.
#[unsafe(no_mangle)]
pub extern "C" fn gc_browser_finish() -> u32 {
    WORKLOAD.with(|slot| slot.borrow_mut().as_mut().map_or(FAILED, Workload::finish))
}

/// Read a counter from the metric schema used by the browser harness.
#[unsafe(no_mangle)]
pub extern "C" fn gc_browser_metric(index: u32) -> u64 {
    WORKLOAD.with(|slot| slot.borrow().as_ref().map_or(u64::MAX, |s| s.metric(index)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_and_cooperative_teardown_preserve_values_and_clear_weak_edges() {
        for policy in 0..=2 {
            let mut state = Workload::new(policy, 1);
            for _ in 0..100_000 {
                if state.completed == 4096 {
                    break;
                }
                assert_ne!(
                    state.poll(u32::try_from(state.completed).unwrap_or(u32::MAX)),
                    FAILED
                );
                state.step();
            }
            assert_eq!(state.completed, 4096);
            for _ in 0..100_000 {
                if state.finish() == 1 {
                    break;
                }
            }
            assert_eq!(state.phase, Phase::Done);
            assert_eq!(state.failed, 0);
        }
    }
}
