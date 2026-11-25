//! A busy descriptor must not hold the generation's sweep cursor indefinitely.
#![expect(clippy::panic_in_result_fn)]
use ristretto_gc::{Configuration, GarbageCollector, Gc, Result, Trace, TraceProgress};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct Contended {
    release: Arc<AtomicBool>,
    attempts: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
// SAFETY: this fixture contains no outgoing GC references.
#[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
unsafe impl Trace for Contended {
    fn trace(&self, _: &GarbageCollector) {}
    fn clear_step(&mut self, offset: usize, _: usize) -> TraceProgress {
        self.attempts.fetch_add(1, Ordering::Relaxed);
        // Retain the continuation offset across transfer to the retirement queue.
        assert!(offset == 0 || offset == 7, "clearing lost its continuation");
        match offset {
            0 => TraceProgress::Continue(7),
            7 if !self.release.load(Ordering::Acquire) => TraceProgress::Busy,
            _ => TraceProgress::Complete,
        }
    }
}
impl Drop for Contended {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn busy_clearing_rotates_and_unrelated_generations_finish() -> Result<()> {
    for concurrent in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(concurrent)
                    .independent_phases(independent)
                    .diagnostics(true)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .max_heap_bytes(4096)
                    .build(),
            );
            let release = Arc::new(AtomicBool::new(false));
            let attempts = Arc::new(AtomicUsize::new(0));
            let drops = Arc::new(AtomicUsize::new(0));
            let root = Gc::try_new(
                &gc,
                Contended {
                    release: release.clone(),
                    attempts: attempts.clone(),
                    drops: drops.clone(),
                },
            )?;
            let bytes = gc.managed_bytes();
            drop(root);
            gc.collect();
            for _ in 0..1000 {
                gc.step();
            }
            assert!(attempts.load(Ordering::Acquire) > 1);
            assert_eq!(drops.load(Ordering::Acquire), 0);
            assert_eq!(gc.diagnostics().retired_objects, 1);
            assert_eq!(gc.diagnostics().retired_bytes, bytes);
            for _ in 0..3 {
                let before = gc.statistics()?.full_collections;
                for _ in 0..32 {
                    drop(Gc::try_new(&gc, 42_u64)?);
                }
                gc.collect();
                for _ in 0..2000 {
                    gc.step();
                }
                assert!(gc.statistics()?.full_collections > before);
                assert_eq!(gc.managed_bytes(), bytes);
                assert_eq!(gc.statistics()?.live_objects, 0);
            }
            release.store(true, Ordering::Release);
            for _ in 0..1000 {
                if gc.step() {
                    break;
                }
            }
            assert_eq!(drops.load(Ordering::Acquire), 1);
            assert_eq!(gc.diagnostics().retired_bytes, 0);
            assert_eq!(gc.managed_bytes(), 0);
            gc.stop()?;
            assert_eq!(drops.load(Ordering::Acquire), 1);
        }
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[expect(
    clippy::expect_used,
    clippy::unwrap_in_result,
    reason = "forced-schedule assertions"
)]
mod native {
    use super::*;
    use ristretto_gc::Finalize;
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Callback {
        Finalize,
        Clear,
        Drop,
    }

    struct Held {
        callback: Callback,
        entered: Mutex<Option<mpsc::Sender<()>>>,
        release: Mutex<mpsc::Receiver<()>>,
        finalizations: Arc<AtomicUsize>,
        drops: Arc<AtomicUsize>,
        clears: Arc<AtomicUsize>,
    }
    impl Held {
        fn wait(&self) {
            let entered = self.entered.lock().expect("entered lock").take();
            if let Some(entered) = entered {
                entered.send(()).expect("callback observer");
                self.release
                    .lock()
                    .expect("release lock")
                    .recv_timeout(Duration::from_secs(30))
                    .expect("callback release");
            }
        }
    }
    // SAFETY: this fixture contains no outgoing GC references.
    #[expect(unsafe_code, reason = "Trace requires an explicit ownership contract")]
    unsafe impl Trace for Held {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, offset: usize, _: usize) -> TraceProgress {
            self.clears.fetch_add(1, Ordering::Relaxed);
            if self.callback == Callback::Clear && offset == 0 {
                self.wait();
                // Return after other full epochs have finished. This continuation
                // must enter the retirement queue, never a later sweep's cursor.
                TraceProgress::Continue(7)
            } else {
                assert!(offset == 0 || offset == 7);
                TraceProgress::Complete
            }
        }
    }
    impl Finalize for Held {
        fn finalize(&self) {
            self.finalizations.fetch_add(1, Ordering::Relaxed);
            if self.callback == Callback::Finalize {
                self.wait();
            }
        }
    }
    impl Drop for Held {
        fn drop(&mut self) {
            if self.callback == Callback::Drop {
                self.wait();
            }
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn reclaim_unrelated(gc: &Arc<GarbageCollector>, bytes: usize) -> Result<()> {
        for _ in 0..3 {
            for _ in 0..32 {
                drop(Gc::try_new(gc, 42_u64)?);
            }
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), bytes);
        }
        Ok(())
    }

    #[test]
    fn blocked_reclamation_callbacks_allow_unrelated_full_cycles() -> Result<()> {
        for concurrent in [false, true] {
            for independent in [false, true] {
                for callback in [Callback::Finalize, Callback::Clear, Callback::Drop] {
                    let gc = GarbageCollector::with_config(
                        Configuration::builder()
                            .concurrent_publication(concurrent)
                            .independent_phases(independent)
                            .diagnostics(true)
                            .allocation_threshold(usize::MAX)
                            .max_heap_bytes(4096)
                            .build(),
                    );
                    let (entered, observed) = mpsc::channel();
                    let (release, released) = mpsc::channel();
                    let finalizations = Arc::new(AtomicUsize::new(0));
                    let drops = Arc::new(AtomicUsize::new(0));
                    let clears = Arc::new(AtomicUsize::new(0));
                    drop(Gc::new_with_finalizer(
                        &gc,
                        Held {
                            callback,
                            entered: Mutex::new(Some(entered)),
                            release: Mutex::new(released),
                            finalizations: finalizations.clone(),
                            drops: drops.clone(),
                            clears: clears.clone(),
                        },
                    ));
                    let bytes = gc.managed_bytes();
                    let initial = {
                        let gc = gc.clone();
                        std::thread::spawn(move || gc.collect_full())
                    };
                    observed
                        .recv_timeout(Duration::from_secs(10))
                        .expect("callback entered");
                    let (done, completion) = mpsc::channel();
                    let worker = {
                        let gc = gc.clone();
                        std::thread::spawn(move || {
                            let result = reclaim_unrelated(&gc, bytes);
                            done.send(result).expect("progress observer");
                        })
                    };
                    let result = completion.recv_timeout(Duration::from_secs(10));
                    let retained = gc.diagnostics().retired_bytes;
                    let before_release = drops.load(Ordering::Relaxed);
                    release.send(()).expect("release callback");
                    initial.join().expect("initial collection")?;
                    worker.join().expect("progress worker");
                    result.expect("blocked callback held full collection open")?;
                    assert_eq!(retained, bytes);
                    assert_eq!(before_release, 0);
                    gc.stop()?;
                    assert_eq!(finalizations.load(Ordering::Relaxed), 1);
                    assert_eq!(drops.load(Ordering::Relaxed), 1);
                    assert_eq!(
                        clears.load(Ordering::Relaxed),
                        if callback == Callback::Clear { 2 } else { 1 }
                    );
                    assert_eq!(gc.diagnostics().retired_bytes, 0);
                    assert_eq!(gc.diagnostics().retired_ready, 0);
                    assert_eq!(gc.managed_bytes(), 0);
                }
            }
        }
        Ok(())
    }
}
