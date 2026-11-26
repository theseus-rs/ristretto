//! Force one reaper to retain its storage while unrelated allocations recover.
use crate::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result};
use std::cell::Cell;
use std::sync::{Arc, Barrier, mpsc};
use std::time::Duration;

thread_local! {
    static PARK: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
}

#[expect(clippy::expect_used, reason = "bounded forced schedule")]
pub(crate) fn before_storage_drop() {
    if let Some((entered, resume)) = PARK.take() {
        entered.send(()).expect("observe private reaper");
        resume
            .recv_timeout(Duration::from_secs(10))
            .expect("resume private reaper");
    }
}

pub(crate) fn allocate_when_woken(
    gc: &Arc<GarbageCollector>,
    value: u64,
    start: &Barrier,
) -> Option<Result<crate::GcRootGuard<u64>>> {
    use std::{
        future::Future,
        sync::mpsc,
        task::{Context, Poll, Wake, Waker},
        time::{Duration, Instant},
    };
    struct WakePeer(mpsc::SyncSender<()>);
    impl Wake for WakePeer {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            let _ = self.0.try_send(());
        }
    }
    let (send, notifications) = mpsc::sync_channel(1);
    let waker = Waker::from(Arc::new(WakePeer(send)));
    let mut cx = Context::from_waker(&waker);
    let mut allocation = Box::pin(Gc::try_new_async(gc, value));
    let first = allocation.as_mut().poll(&mut cx);
    // Every competing request must observe a full heap before any peer assists.
    start.wait();
    assert!(first.is_pending());
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let left = deadline.checked_duration_since(Instant::now())?;
        notifications.recv_timeout(left).ok()?;
        if let Poll::Ready(result) = allocation.as_mut().poll(&mut cx) {
            return Some(result);
        }
    }
}

#[test]
#[expect(
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    reason = "forced owner suspension"
)]
fn independent_storage_is_reclaimable_while_a_private_reaper_is_parked() -> Result<()> {
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .metadata_shards(1)
                    .concurrent_publication(counted)
                    .independent_phases(independent)
                    .max_heap_bytes(64)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let donor = ((1u64, 2u64), (3u64, 4u64));
            let parked_donor = Gc::try_new(&gc, donor)?;
            let weak = GcWeak::new(&parked_donor, ReferenceKind::Weak, 0);
            let other = Gc::try_new(&gc, donor)?;
            drop(parked_donor);
            let (entered, parked) = mpsc::channel();
            let (release, resume) = mpsc::channel();
            let worker_gc = gc.clone();
            let worker = std::thread::spawn(move || {
                PARK.set(Some((entered, resume)));
                worker_gc.collect_full()
            });
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("storage taken");
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 64);
            drop(other);

            // One helper must reach and release the second donor without the
            // first reaper. Keep its result rooted while using the exact balance.
            let first = allocate_when_woken(&gc, 23, &Barrier::new(1))
                .expect("independent storage becomes available")?;
            assert_eq!(gc.managed_bytes(), 40);
            let roots = [
                first,
                Gc::try_new(&gc, 24u64)?,
                Gc::try_new(&gc, 25u64)?,
                Gc::try_new(&gc, 26u64)?,
            ];
            assert!(
                roots
                    .iter()
                    .zip(23u64..)
                    .all(|(root, value)| ***root == value)
            );
            assert_eq!(gc.managed_bytes(), 64);
            assert!(matches!(
                Gc::try_new(&gc, 27u64),
                Err(crate::Error::OutOfMemory)
            ));
            release.send(()).expect("release");
            worker.join().expect("collector")?;
            assert_eq!(gc.managed_bytes(), 32);
            let recovered = Gc::try_new(&gc, 27u64)?;
            assert_eq!(**recovered, 27);
            drop((roots, recovered));
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}
