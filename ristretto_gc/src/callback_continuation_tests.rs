//! Foreground reclamation while the native primary is parked after publication.
use super::*;
use crate::{Configuration, Gc, Result, Trace};
use std::time::Duration;

thread_local! {
    static CONTINUATION_PAUSE: std::cell::RefCell<Option<super::super::ClaimPause>> =
        const { std::cell::RefCell::new(None) };
}

#[expect(clippy::expect_used, reason = "bounded post-publication test hook")]
pub(super) fn after_continuation_publication() {
    if let Some(pause) = CONTINUATION_PAUSE.take() {
        pause.entered.send(()).expect("notify foreground");
        pause
            .resume
            .recv_timeout(Duration::from_secs(10))
            .expect("resume callback");
    }
}

struct Continued {
    hook: Mutex<Option<super::super::ClaimPause>>,
    calls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
    finalizers: Arc<AtomicUsize>,
}

// SAFETY: this fixture contains only counters and test synchronization.
#[expect(unsafe_code)]
unsafe impl Trace for Continued {
    fn trace(&self, _: &GarbageCollector) {}
    fn trace_work(&self) -> usize {
        8
    }
    fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
        let count = self.calls.fetch_add(1, Ordering::SeqCst);
        if let Some(pause) = lock(&self.hook).take() {
            assert_eq!((count, offset, budget), (0, 0, 1));
            CONTINUATION_PAUSE.set(Some(pause));
            TraceProgress::Continue(7)
        } else {
            assert_eq!((count, offset, budget), (1, 7, 1));
            TraceProgress::Complete
        }
    }
}

impl crate::Finalize for Continued {
    fn finalize(&self) {
        self.finalizers.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for Continued {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "bounded parked primary schedule"
)]
fn foreground_steps_can_reclaim_a_continuation_while_its_primary_is_parked() -> Result<()> {
    let heap = GarbageCollector::with_config(
        Configuration::builder()
            .threads(0)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let (notify, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let calls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let finalizers = Arc::new(AtomicUsize::new(0));
    drop(Gc::new_with_finalizer(
        &heap,
        Continued {
            hook: Mutex::new(Some(super::super::ClaimPause {
                entered: notify,
                resume: release,
            })),
            calls: calls.clone(),
            drops: drops.clone(),
            finalizers: finalizers.clone(),
        },
    ));
    heap.collect();
    let task = (0..16_384)
        .find_map(|_| {
            heap.step();
            lock(&heap.domains[1].cycle).reclaim.pop_front()
        })
        .expect("one real swept task");
    // Install the real callback pool without starting marking workers. There
    // is no cap demand to wake its pressure assistant.
    heap.retired_ready.fetch_add(1, Ordering::AcqRel);
    let generation = Arc::new(super::super::WorkerSignals::new(1));
    heap.worker_signals.store(Some(generation.clone()));
    let callbacks = &generation.callbacks;
    assert!(
        callbacks
            .send(
                &heap,
                Batch {
                    first: (task, 1),
                    rest: vec![]
                },
                None
            )
            .is_none()
    );
    parked
        .recv_timeout(Duration::from_secs(5))
        .expect("primary published continuation");
    let charge = heap.managed_bytes();
    let publicly_completed = (0..2048).any(|_| {
        heap.step();
        drops.load(Ordering::Acquire) == 1 && heap.managed_bytes() == 0
    });
    // Always finish the original callback before checking the foreground result.
    resume.send(()).expect("resume primary");
    assert!(!callbacks.join()?);
    heap.stop()?;
    assert_eq!(finalizers.load(Ordering::Acquire), 1);
    assert_eq!(calls.load(Ordering::Acquire), 2);
    assert_eq!(drops.load(Ordering::Acquire), 1);
    assert_eq!(heap.callback_jobs.load(Ordering::Acquire), 0);
    assert_eq!(heap.retired_ready.load(Ordering::Acquire), 0);
    assert!(
        charge > 0 && publicly_completed,
        "foreground did not release the parked primary's published continuation"
    );
    Ok(())
}
