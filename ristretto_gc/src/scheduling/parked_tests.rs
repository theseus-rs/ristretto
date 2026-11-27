//! A parked pressure observer or sampled publisher must leave full collection available.
use crate::{Configuration, GarbageCollector, Gc, GcRootGuard, GcWeak, ReferenceKind, Result};
use std::cell::Cell;
use std::future::Future;
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

type Pause = Cell<Option<(Sender<()>, Receiver<()>)>>;

thread_local! {
    static PAUSE: Pause = const { Cell::new(None) };
    static AFTER_SAMPLE_COMMIT: Pause = const { Cell::new(None) };
}

pub(super) fn after_model_read() {
    park(&PAUSE);
}

pub(super) fn after_sample_commit() {
    park(&AFTER_SAMPLE_COMMIT);
}

#[expect(
    clippy::expect_used,
    reason = "a missing peer or timeout invalidates the forced schedule"
)]
fn park(point: &'static std::thread::LocalKey<Pause>) {
    if let Some((entered, release)) = point.take() {
        entered.send(()).expect("observe model acquisition");
        release
            .recv_timeout(Duration::from_secs(10))
            .expect("resume model user");
    }
}

fn recover(gc: &std::sync::Arc<GarbageCollector>) -> Result<Option<GcRootGuard<u64>>> {
    let mut collection = std::pin::pin!(gc.collect_full_async());
    let mut allocation = std::pin::pin!(Gc::try_new_async(gc, 23_u64));
    let mut context = Context::from_waker(Waker::noop());
    let mut complete = false;
    let mut replacement = None;
    for _ in 0..4096 {
        if !complete && let Poll::Ready(result) = collection.as_mut().poll(&mut context) {
            result?;
            complete = true;
        }
        if replacement.is_none()
            && let Poll::Ready(result) = allocation.as_mut().poll(&mut context)
        {
            replacement = Some(result?);
        }
        if complete && replacement.is_some() {
            return Ok(replacement);
        }
    }
    Ok(None)
}

fn capped_heap(counted: bool, size: usize) -> std::sync::Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .threads(0)
            .concurrent_publication(counted)
            .independent_phases(true)
            .independent_marking(true)
            .predictive_collection(true)
            .allocation_threshold(usize::MAX)
            .max_heap_bytes(3 * size)
            .incremental_step_size(2)
            .build(),
    )
}

#[test]
#[expect(clippy::panic_in_result_fn)]
fn parked_pressure_reader_and_sampled_publisher_allow_capped_recovery() -> Result<()> {
    let size = Gc::allocation_size(&41_u64)?;
    let mut stalled = Vec::new();
    for counted in [false, true] {
        for publisher in [false, true] {
            let gc = capped_heap(counted, size);
            let survivor = Gc::try_new(&gc, 41_u64)?;
            let donor = Gc::try_new(&gc, 7_u64)?;
            let weak = GcWeak::new(&donor, ReferenceKind::Weak, 0);
            drop(donor);
            let third = if publisher {
                None
            } else {
                Some(Gc::try_new(&gc, 59_u64)?)
            };
            let before = gc.statistics()?;
            let scheduling = gc.scheduling_statistics();
            let (entered, parked) = mpsc::channel();
            let (resume, released) = mpsc::channel();
            let (done, progress) = mpsc::channel();
            let heap = &gc;
            let (while_parked, replacement, third, observed) = std::thread::scope(|scope| {
                let participant = scope.spawn(move || {
                    PAUSE.set(Some((entered, released)));
                    if publisher {
                        (Some(Gc::try_new(heap, 59_u64).expect("sampled root")), None)
                    } else {
                        (third, Some(heap.scheduling_statistics()))
                    }
                });
                parked
                    .recv_timeout(Duration::from_secs(3))
                    .expect("model acquisition");
                assert_eq!(heap.managed_bytes(), 3 * size);
                let peer = scope.spawn(move || {
                    let result = recover(heap);
                    done.send(matches!(&result, Ok(Some(_))))
                        .expect("progress observer");
                    result
                });
                let while_parked = progress
                    .recv_timeout(Duration::from_secs(2))
                    .unwrap_or(false);
                resume.send(()).expect("resumed model user");
                let (third, observed) = participant.join().expect("model user");
                (
                    while_parked,
                    peer.join().expect("collection peer"),
                    third,
                    observed,
                )
            });
            if !while_parked {
                stalled.push((counted, publisher));
            }
            let replacement = replacement?.expect("full ticket and replacement");
            let third = third.expect("root acquired by publisher");
            if let Some(view) = observed {
                assert_eq!(view.requests, scheduling.requests);
                assert_eq!(view.live_bytes, scheduling.live_bytes);
            }
            assert!(weak.upgrade().is_none());
            assert_eq!((**survivor, **third, **replacement), (41, 59, 23));
            let after = gc.statistics()?;
            assert!(after.full_collections > before.full_collections);
            assert_eq!(after.managed_bytes_freed - before.managed_bytes_freed, size);
            drop((replacement, third, survivor));
            gc.collect_full()?;
            let requests = gc.scheduling_statistics().requests;
            let roots = [
                Gc::try_new(&gc, 1_u64)?,
                Gc::try_new(&gc, 2_u64)?,
                Gc::try_new(&gc, 3_u64)?,
            ];
            assert!(gc.scheduling_statistics().requests > requests);
            drop(roots);
        }
    }
    assert!(
        stalled.is_empty(),
        "pressure model blocked capped recovery: {stalled:?}"
    );
    Ok(())
}

#[test]
#[expect(clippy::panic_in_result_fn)]
fn capped_allocation_recovers_before_the_predicting_publisher_requests_collection() -> Result<()> {
    let size = Gc::allocation_size(&41_u64)?;
    for counted in [false, true] {
        let gc = capped_heap(counted, size);
        let survivor = Gc::try_new(&gc, 41_u64)?;
        let donor = Gc::try_new(&gc, 7_u64)?;
        let weak = GcWeak::new(&donor, ReferenceKind::Weak, 0);
        drop(donor);
        assert_eq!(gc.scheduling_statistics().requests, 0);
        let (entered, parked) = mpsc::channel();
        let (resume, released) = mpsc::channel();
        let heap = &gc;
        let (replacement, third) = std::thread::scope(|scope| {
            let publisher = scope.spawn(move || {
                AFTER_SAMPLE_COMMIT.set(Some((entered, released)));
                Gc::try_new(heap, 59_u64)
            });
            parked
                .recv_timeout(Duration::from_secs(3))
                .expect("predictor commit");
            assert_eq!(heap.scheduling_statistics().requests, 1);
            assert_eq!(heap.managed_bytes(), 3 * size);

            // Only the allocation may request and drive recovery.
            let mut allocation = std::pin::pin!(Gc::try_new_async(heap, 23_u64));
            let mut context = Context::from_waker(Waker::noop());
            let result = (0..4096).find_map(|_| match allocation.as_mut().poll(&mut context) {
                Poll::Ready(result) => Some(result),
                Poll::Pending => None,
            });
            assert!(weak.upgrade().is_none());
            assert_eq!(heap.managed_bytes(), 3 * size);
            resume.send(()).expect("resume predictor");
            (result, publisher.join().expect("predicting publisher"))
        });
        let replacement = replacement.expect("capped allocation while publisher is parked")?;
        let third = third?;
        assert_eq!((**survivor, **third, **replacement), (41, 59, 23));
    }
    Ok(())
}

#[test]
fn a_stalled_pre_wrap_winner_cannot_reinstate_its_hint_after_completion() {
    use super::{Ordering, Scheduler};

    let scheduler = &Scheduler::new(8192);
    scheduler.allocated.store(u64::MAX - 8, Ordering::Relaxed);
    let (entered, parked) = mpsc::channel();
    let (resume, released) = mpsc::channel();
    std::thread::scope(|scope| {
        let sampler = scope.spawn(move || {
            AFTER_SAMPLE_COMMIT.set(Some((entered, released)));
            scheduler.allocated(8, 0, None)
        });
        parked
            .recv_timeout(Duration::from_secs(3))
            .expect("pre-wrap sample committed");
        assert!(!scheduler.allocated(1, 0, None));
        scheduler.started(0);
        scheduler.completed(0, 0);
        assert!(scheduler.allocated(128, 8192, None));
        resume.send(()).expect("resume pre-wrap winner");
        assert!(!sampler.join().expect("pre-wrap sample"));
    });
    assert_eq!(scheduler.skip_before.load(Ordering::Relaxed), 0);
    assert_eq!(scheduler.statistics().requests, 1);
}
