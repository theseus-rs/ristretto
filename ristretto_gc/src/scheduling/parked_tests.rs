//! A parked pressure observer or sampled publisher must leave full collection available.
use crate::{Configuration, GarbageCollector, Gc, GcRootGuard, GcWeak, ReferenceKind, Result};
use std::cell::Cell;
use std::future::Future;
use std::sync::mpsc::{self, Receiver, Sender};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

thread_local! {
    static PAUSE: Cell<Option<(Sender<()>, Receiver<()>)>> = const { Cell::new(None) };
}

#[expect(
    clippy::expect_used,
    reason = "a missing peer or timeout invalidates the forced schedule"
)]
pub(super) fn after_model_read() {
    if let Some((entered, release)) = PAUSE.take() {
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

#[test]
#[expect(clippy::panic_in_result_fn)]
fn parked_pressure_reader_and_sampled_publisher_allow_capped_recovery() -> Result<()> {
    let size = Gc::allocation_size(&41_u64)?;
    let mut stalled = Vec::new();
    for counted in [false, true] {
        for publisher in [false, true] {
            let gc = GarbageCollector::with_config(
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
            );
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
