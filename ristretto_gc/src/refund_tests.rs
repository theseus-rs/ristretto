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
    allocate_after_first_poll(gc, value, || {
        start.wait();
    })
}

fn allocate_after_first_poll(
    gc: &Arc<GarbageCollector>,
    value: u64,
    first_polled: impl FnOnce(),
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
    first_polled();
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

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Instant;

struct Runnable(AtomicBool);
impl Wake for Runnable {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

struct WaitingPeer<'a> {
    runnable: Arc<Runnable>,
    waker: Waker,
    future: std::pin::Pin<Box<dyn Future<Output = Result<crate::GcRootGuard<u64>>> + 'a>>,
    result: Option<Result<crate::GcRootGuard<u64>>>,
}

impl<'a> WaitingPeer<'a> {
    fn new(gc: &'a Arc<GarbageCollector>, value: u64) -> Self {
        let runnable = Arc::new(Runnable(AtomicBool::new(false)));
        let waker = Waker::from(runnable.clone());
        let mut future = Box::pin(Gc::try_new_async(gc, value));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        Self {
            runnable,
            waker,
            future,
            result: None,
        }
    }

    fn poll_when_woken(&mut self) -> bool {
        if self.result.is_some() || !self.runnable.0.swap(false, Ordering::AcqRel) {
            return false;
        }
        if let Poll::Ready(result) = self
            .future
            .as_mut()
            .poll(&mut Context::from_waker(&self.waker))
        {
            self.result = Some(result);
        }
        true
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "forced refund timing"
)]
fn live_waiters_receive_the_independent_donors_full_charge() -> Result<()> {
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .independent_phases(independent)
                    .metadata_shards(1)
                    .max_heap_bytes(64)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let donor = ((1u64, 2u64), (3u64, 4u64));
            let a = Gc::try_new(&gc, donor)?;
            let b = Gc::try_new(&gc, donor)?;
            drop(a);
            let (enter_a, parked_a) = mpsc::channel();
            let (release_a, resume_a) = mpsc::channel();
            let heap = gc.clone();
            let reaper = std::thread::spawn(move || {
                PARK.set(Some((enter_a, resume_a)));
                heap.collect_full()
            });
            parked_a.recv_timeout(Duration::from_secs(3)).unwrap();
            let mut peers: Vec<_> = (1..4).map(|value| WaitingPeer::new(&gc, value)).collect();
            let (enter_b, parked_b) = mpsc::channel();
            let (release_b, resume_b) = mpsc::channel();
            let (enrolled, first_poll) = mpsc::channel();
            let (permit, permitted) = mpsc::channel();
            let heap = gc.clone();
            let helper = std::thread::spawn(move || {
                PARK.set(Some((enter_b, resume_b)));
                allocate_after_first_poll(&heap, 0, || {
                    enrolled.send(()).unwrap();
                    permitted.recv().unwrap();
                })
            });
            first_poll.recv_timeout(Duration::from_secs(3)).unwrap();
            drop(b);
            permit.send(()).unwrap();
            parked_b.recv_timeout(Duration::from_secs(3)).unwrap();
            let ticket = gc.completed_full_ticket();
            gc.collect_full()?;
            assert!(gc.completed_full_ticket() > ticket);
            // A late request has no claim on either privately owned donor.
            let mut late = WaitingPeer::new(&gc, 4);
            for _ in 0..2048 {
                if !late.poll_when_woken() {
                    break;
                }
            }
            for peer in &mut peers {
                for _ in 0..2048 {
                    if !peer.poll_when_woken() {
                        break;
                    }
                }
            }
            let all_pending = peers.iter().all(|peer| peer.result.is_none());
            let before_release = gc.managed_bytes();
            release_b.send(()).unwrap();
            let designated = helper.join().unwrap().expect("donor B resumed");
            let deadline = Instant::now() + Duration::from_secs(3);
            while peers.iter().any(|peer| peer.result.is_none()) && Instant::now() < deadline {
                for peer in &mut peers {
                    peer.poll_when_woken();
                }
                std::thread::yield_now();
            }
            let values: Vec<_> = peers
                .iter()
                .map(|peer| {
                    peer.result
                        .as_ref()
                        .and_then(|r| r.as_ref().ok())
                        .map(|root| ***root)
                })
                .collect();
            let charged = gc.managed_bytes();
            release_a.send(()).unwrap();
            reaper.join().unwrap()?;
            assert!(
                all_pending,
                "covered requests must wait for physical deallocation"
            );
            assert!(matches!(late.result, Some(Err(crate::Error::OutOfMemory))));
            assert_eq!(before_release, 64);
            assert_eq!(**designated?, 0);
            assert_eq!(values, [Some(1), Some(2), Some(3)]);
            assert_eq!(charged, 64);
            drop(peers);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}
