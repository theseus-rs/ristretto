//! Partial rooting must never publish complete tracing coverage.
#![cfg(not(target_family = "wasm"))]
#![expect(
    unsafe_code,
    reason = "fixture forwards both strong edges and certifies only owning slices"
)]
#![expect(clippy::expect_used, clippy::panic_in_result_fn)]
use ristretto_gc::cell::{RwLock, RwLockWriteGuard};
use ristretto_gc::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result, Trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

struct Pause {
    armed: AtomicBool,
    entered: mpsc::Sender<()>,
    resume: Mutex<mpsc::Receiver<()>>,
}
struct PartialRoots {
    edges: [Gc<u64>; 2],
    pause: Arc<Pause>,
}
// SAFETY: both strong fields forward paired ownership conversions and tracing.
unsafe impl Trace for PartialRoots {
    fn trace(&self, gc: &GarbageCollector) {
        for edge in &self.edges {
            edge.trace(gc);
        }
    }
    fn root(&mut self) {
        self.edges[0].root();
        if self.pause.armed.swap(false, Ordering::AcqRel) {
            self.pause.entered.send(()).expect("observer live");
            self.pause
                .resume
                .lock()
                .expect("pause lock")
                .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
                .expect("rooting released");
        }
        self.edges[1].root();
    }
    unsafe fn unroot(&mut self, owner: ristretto_gc::TraceOwner<'_>) {
        for edge in &mut self.edges {
            // SAFETY: each field is embedded in the caller's protected parent.
            unsafe { Trace::unroot(edge, owner) };
        }
    }
}

fn finish(gc: &GarbageCollector) -> bool {
    (0..1024).any(|_| gc.step())
}

#[test]
fn complete_coverage_waits_for_every_child_root_and_survives_guard_cancellation() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration {
        allocation_threshold: usize::MAX,
        incremental_step_size: 1,
        max_pause_time_us: 0,
        ..Configuration::default()
    });
    let left = Gc::try_new(&gc, 7_u64)?.into_rooted();
    let right = Gc::try_new(&gc, 11_u64)?.into_rooted();
    let left_weak = GcWeak::new(&left, ReferenceKind::Weak, 0);
    let right_weak = GcWeak::new(&right, ReferenceKind::Weak, 0);
    let (entered, reached) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let pause = Arc::new(Pause {
        armed: AtomicBool::new(true),
        entered,
        resume: Mutex::new(release),
    });
    let owner = Gc::try_new(
        &gc,
        RwLock::new(PartialRoots {
            edges: [left, right],
            pause,
        }),
    )?;
    let (covered, published) = mpsc::channel();
    let (cancel, cancelled) = mpsc::channel::<()>();
    let finished = Arc::new(AtomicBool::new(false));
    let guard_finished = finished.clone();
    std::thread::scope(|scope| {
        let borrowed = &owner;
        let worker = scope.spawn(move || {
            // SAFETY: preparation roots both fields, and the slice permits only owning Gc values.
            let guard = unsafe {
                RwLockWriteGuard::try_map_trace_rooted(borrowed.write(), |value| {
                    Some(value.edges.as_mut_slice())
                })
            }
            .unwrap_or_else(|_| panic!("owning slice"));
            covered.send(()).expect("observer live");
            let _ = cancelled.recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }));
            drop(guard);
            guard_finished.store(true, Ordering::Release);
        });
        reached
            .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
            .expect("first child rooted");
        gc.collect();
        let premature = finish(&gc);
        resume.send(()).expect("resume second child rooting");
        published
            .recv_timeout(Duration::from_secs(if cfg!(miri) { 120 } else { 20 }))
            .expect("all child roots published");
        let completed = finish(&gc);
        let alive = left_weak.upgrade().is_some() && right_weak.upgrade().is_some();
        let held = !finished.load(Ordering::Acquire);
        drop(cancel); // cancellation drops the projected guard normally
        worker.join().expect("guard owner");
        assert!(
            !premature,
            "partial rooting was treated as complete graph coverage"
        );
        assert!(
            completed && held,
            "completed coverage still awaited the parked owner"
        );
        assert!(
            alive,
            "a child was reclaimed before its ownership was restored"
        );
    });
    let ordinary = owner.write();
    assert_eq!(
        (**owner).trace_step(&gc, 0, 1),
        ristretto_gc::TraceProgress::Busy
    );
    drop(ordinary);
    gc.collect_full()?;
    assert!(left_weak.upgrade().is_some() && right_weak.upgrade().is_some());
    drop(owner);
    gc.stop()?;
    assert!(left_weak.upgrade().is_none() && right_weak.upgrade().is_none());
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
