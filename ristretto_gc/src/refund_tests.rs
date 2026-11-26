//! Force one reaper to retain its storage while unrelated allocations recover.
use crate::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result};
use std::cell::Cell;
use std::sync::{Arc, Barrier, mpsc};
use std::time::Duration;

thread_local! {
    static SELECTOR: std::cell::RefCell<Option<SelectorGate>> = const { std::cell::RefCell::new(None) };
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

fn capped_heap(counted: bool, independent: bool) -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .metadata_shards(1)
            .concurrent_publication(counted)
            .independent_phases(independent)
            .max_heap_bytes(64)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    )
}

#[expect(
    clippy::expect_used,
    reason = "the parked thread owns the donor before the fixture continues"
)]
fn park_reaper(
    gc: &Arc<GarbageCollector>,
    registered: bool,
) -> (mpsc::Sender<()>, std::thread::JoinHandle<Result<()>>) {
    let (entered, parked) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    let gc = gc.clone();
    let worker = std::thread::spawn(move || {
        let _registration = registered.then(|| crate::recovery::WorkerRegistration::new(&gc));
        PARK.set(Some((entered, resume)));
        gc.collect_full()
    });
    parked
        .recv_timeout(Duration::from_secs(5))
        .expect("storage taken");
    (release, worker)
}

#[test]
fn independent_storage_is_reclaimable_while_a_private_reaper_is_parked() -> Result<()> {
    recover_with_parked_reaper(false, |gc| {
        allocate_when_woken(gc, 23, &Barrier::new(1))
            .expect("independent storage becomes available")
    })
}

#[test]
fn synchronous_constructor_helps_when_the_registered_worker_is_parked() -> Result<()> {
    recover_with_parked_reaper(true, |gc| Gc::try_new(gc, 23u64))
}

#[expect(
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    clippy::expect_used,
    reason = "forced owner suspension"
)]
fn recover_with_parked_reaper(
    registered: bool,
    allocate: impl Fn(&Arc<GarbageCollector>) -> Result<crate::GcRootGuard<u64>>,
) -> Result<()> {
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = capped_heap(counted, independent);
            let donor = ((1u64, 2u64), (3u64, 4u64));
            let parked_donor = Gc::try_new(&gc, donor)?;
            let weak = GcWeak::new(&parked_donor, ReferenceKind::Weak, 0);
            let other = Gc::try_new(&gc, donor)?;
            drop(parked_donor);
            let (release, worker) = park_reaper(&gc, registered);
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 64);
            drop(other);

            // One helper must reach and release the second donor without the
            // first reaper. Keep its result rooted while using the exact balance.
            let prior = gc.completed_full_ticket();
            assert!(matches!(
                Gc::try_new_once(&gc, 23u64),
                Err((crate::Error::OutOfMemory, 23))
            ));
            assert_eq!(gc.completed_full_ticket(), prior);
            let first = allocate(&gc)?;
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

    fn poll_until_parked(&mut self) {
        for _ in 0..2048 {
            if !self.poll_when_woken() {
                break;
            }
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
fn live_waiters_receive_the_independent_donors_full_charge() -> Result<()> {
    check_independent_donor(DonorSchedule::Direct)
}

#[test]
fn live_waiters_share_an_independent_donor_after_its_callback_lease_ends() -> Result<()> {
    check_independent_donor(DonorSchedule::Leased)
}

#[test]
fn waiters_can_enroll_after_lease_release_before_the_first_reaper_claim() -> Result<()> {
    check_independent_donor(DonorSchedule::EnrollAfterLease)
}

#[derive(Clone, Copy)]
enum DonorSchedule {
    Direct,
    Leased,
    EnrollAfterLease,
}

#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    reason = "the bounded schedule holds an exact registered donor"
)]
fn release_donor<T: crate::Trace + 'static>(
    gc: &GarbageCollector,
    donor: crate::GcRootGuard<T>,
    leased: bool,
) -> Result<()> {
    let metadata = leased.then(|| gc.metadata_for_address(donor.as_ptr().addr()).unwrap());
    let lease = metadata
        .as_ref()
        .map(|metadata| crate::metadata::ObjectMetadata::borrow_callback(metadata, false).unwrap());
    drop(donor);
    if leased {
        let ticket = gc.completed_full_ticket();
        gc.collect_full()?;
        assert!(gc.completed_full_ticket() > ticket);
        assert_eq!(gc.managed_bytes(), 64);
    }
    drop(lease);
    Ok(())
}

#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unwrap_in_result,
    reason = "forced refund timing"
)]
fn check_independent_donor(schedule: DonorSchedule) -> Result<()> {
    let leased = !matches!(schedule, DonorSchedule::Direct);
    let enroll_after_lease = matches!(schedule, DonorSchedule::EnrollAfterLease);
    let mut premature_results = Vec::new();
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = capped_heap(counted, independent);
            let donor = ((1u64, 2u64), (3u64, 4u64));
            let a = Gc::try_new(&gc, donor)?;
            let b = Gc::try_new(&gc, donor)?;
            drop(a);
            let (release_a, reaper) = park_reaper(&gc, false);
            let enroll = || {
                (1..4)
                    .map(|value| WaitingPeer::new(&gc, value))
                    .collect::<Vec<_>>()
            };
            let mut peers = if enroll_after_lease {
                Vec::new()
            } else {
                enroll()
            };
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
            release_donor(&gc, b, leased)?;
            if enroll_after_lease {
                peers = enroll();
            }
            permit.send(()).unwrap();
            parked_b.recv_timeout(Duration::from_secs(3)).unwrap();
            let ticket = gc.completed_full_ticket();
            gc.collect_full()?;
            assert!(gc.completed_full_ticket() > ticket);
            // A late request has no claim on either privately owned donor.
            let mut late = WaitingPeer::new(&gc, 4);
            late.poll_until_parked();
            for peer in &mut peers {
                peer.poll_until_parked();
            }
            let terminal_oom = peers
                .iter()
                .filter(|peer| matches!(peer.result, Some(Err(crate::Error::OutOfMemory))))
                .count();
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
            if !all_pending {
                premature_results.push((counted, independent, terminal_oom, values.clone()));
            }
            assert!(matches!(late.result, Some(Err(crate::Error::OutOfMemory))));
            assert_eq!(before_release, 64);
            assert_eq!(**designated?, 0);
            if all_pending {
                assert_eq!(values, [Some(1), Some(2), Some(3)]);
                assert_eq!(charged, 64);
            }
            drop(peers);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    assert!(
        premature_results.is_empty(),
        "early terminal results before physical release (counted, independent, OOMs, retained peer values), leased={leased}: {premature_results:?}"
    );
    Ok(())
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "bound every forced grant handoff"
)]
fn synchronous_sleepers_wake_after_a_fully_assigned_private_donor_is_freed() -> Result<()> {
    use crate::recovery::sync_points;
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = capped_heap(counted, independent);
            let donor = ((1u64, 2u64), (3u64, 4u64));
            let a = Gc::try_new(&gc, donor)?;
            let b = Gc::try_new(&gc, donor)?;
            drop(a);
            let (a_release, reaper) = park_reaper(&gc, false);
            let (enroll, enrolled) = mpsc::channel();
            let (sleep, sleeping) = mpsc::channel();
            let (done, results) = mpsc::channel();
            let barrier = Arc::new(Barrier::new(4));
            for n in 1u64..4 {
                let heap = gc.clone();
                let start = barrier.clone();
                let enroll = enroll.clone();
                let sleep = sleep.clone();
                let done = done.clone();
                std::thread::spawn(move || {
                    sync_points::ENROLLED.set(Some(Box::new(move || {
                        enroll.send(()).unwrap();
                        start.wait();
                    })));
                    sync_points::COVERED.set(Some(Box::new(move || {
                        sleep.send(()).unwrap();
                    })));
                    done.send(Gc::try_new(&heap, n)).unwrap();
                });
            }
            for _ in 1..4 {
                enrolled.recv_timeout(Duration::from_secs(3)).unwrap();
            }
            let (b_enter, b_parked) = mpsc::channel();
            let (b_release, b_wait) = mpsc::channel();
            let (last, ready) = mpsc::channel();
            let (begin, go) = mpsc::channel();
            let heap = gc.clone();
            let assisting = std::thread::spawn(move || {
                PARK.set(Some((b_enter, b_wait)));
                allocate_after_first_poll(&heap, 0, || {
                    last.send(()).unwrap();
                    go.recv().unwrap();
                })
            });
            ready.recv_timeout(Duration::from_secs(3)).unwrap();
            drop(b);
            begin.send(()).unwrap();
            b_parked.recv_timeout(Duration::from_secs(3)).unwrap();
            gc.collect_full()?;
            barrier.wait();
            for _ in 1..4 {
                sleeping.recv_timeout(Duration::from_secs(3)).unwrap();
            }
            assert!(results.try_recv().is_err());
            assert_eq!(gc.managed_bytes(), 64);
            assert!(matches!(
                Gc::try_new(&gc, 4u64),
                Err(crate::Error::OutOfMemory)
            ));
            b_release.send(()).unwrap();
            let first = assisting.join().unwrap().unwrap()?;
            let mut roots = Vec::new();
            for _ in 1..4 {
                roots.push(results.recv_timeout(Duration::from_secs(3)).unwrap()?);
            }
            let mut values: Vec<_> = roots.iter().map(|r| ***r).collect();
            values.sort_unstable();
            assert_eq!(values, [1, 2, 3]);
            assert_eq!(**first, 0);
            assert_eq!(gc.managed_bytes(), 64);
            a_release.send(()).unwrap();
            reaper.join().unwrap()?;
            drop((first, roots));
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn a_peer_finishes_the_same_released_donor_while_its_selector_is_parked() -> Result<()> {
    for point in [
        SelectionPoint::Progress,
        SelectionPoint::BeforeBinding,
        SelectionPoint::AfterBinding,
        SelectionPoint::BeforeClaim,
    ] {
        parked_selector_schedule(point, false)?;
    }
    Ok(())
}

#[test]
fn cancelling_the_bound_request_before_checkpoint_commit_refunds_once() -> Result<()> {
    parked_selector_schedule(SelectionPoint::AfterBinding, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionPoint {
    Progress,
    BeforeBinding,
    AfterBinding,
    BeforeClaim,
}

type SelectorGate = (SelectionPoint, mpsc::Sender<()>, mpsc::Receiver<()>);

#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    reason = "bounded interleaving before physical destruction"
)]
fn parked_selector_schedule(point: SelectionPoint, cancel_first: bool) -> Result<()> {
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = capped_heap(counted, independent);
            let value = ((1u64, 2u64), (3u64, 4u64));
            let live = Gc::try_new(&gc, value)?;
            let retiring = Gc::try_new(&gc, value)?;
            let mut peers: Vec<_> = (0..4).map(|v| WaitingPeer::new(&gc, v)).collect();
            release_donor(&gc, retiring, true)?;
            let (enter, parked) = mpsc::channel();
            let (resume, release) = mpsc::channel();
            let heap = gc.clone();
            let selector = std::thread::spawn(move || {
                SELECTOR.replace(Some((point, enter, release)));
                for _ in 0..64 {
                    heap.step();
                    if SELECTOR.with_borrow(Option::is_none) {
                        break;
                    }
                }
            });
            parked.recv_timeout(Duration::from_secs(3)).unwrap();
            if cancel_first {
                drop(peers.remove(0));
            }
            for _ in 0..2048 {
                gc.step();
                for peer in &mut peers {
                    peer.poll_when_woken();
                }
                if peers.iter().all(|p| p.result.is_some()) {
                    break;
                }
            }
            let outcomes: Vec<_> = peers
                .iter()
                .map(|p| {
                    p.result
                        .as_ref()
                        .and_then(|r| r.as_ref().ok())
                        .map(|v| ***v)
                })
                .collect();
            let charged = gc.managed_bytes();
            resume.send(()).unwrap();
            selector.join().unwrap();
            let expected = if cancel_first {
                vec![Some(1), Some(2), Some(3)]
            } else {
                vec![Some(0), Some(1), Some(2), Some(3)]
            };
            assert_eq!(
                outcomes, expected,
                "survivors must spend this donor before the first selector resumes"
            );
            assert_eq!(charged, if cancel_first { 56 } else { 64 });
            assert_eq!(
                gc.managed_bytes(),
                charged,
                "stale helpers cannot refund twice"
            );
            drop(peers);
            drop(live);
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[expect(clippy::expect_used, reason = "bounded selector suspension")]
pub(super) fn park_selection(point: SelectionPoint) {
    let gate = SELECTOR.with_borrow_mut(|slot| {
        if slot.as_ref().is_some_and(|(at, _, _)| *at == point) {
            slot.take()
        } else {
            None
        }
    });
    if let Some((_, entered, release)) = gate {
        entered.send(()).expect("announce selector gate");
        release
            .recv_timeout(Duration::from_secs(5))
            .expect("resume selector");
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "a returned donor covers the final waiter after a long prefix"
)]
fn terminal_allocation_assists_a_released_donor_past_nonfitting_requests() -> Result<()> {
    let gc = capped_heap(false, false);
    let value = ((1u64, 2u64), (3u64, 4u64));
    let live = Gc::try_new(&gc, value)?;
    let retiring = Gc::try_new(&gc, value)?;
    let prefix: Vec<_> = (0..4096)
        .map(|_| gc.grant_registry.register(&gc, 40))
        .collect::<Result<_>>()?;
    release_donor(&gc, retiring, true)?;
    let root = allocate_when_woken(&gc, 23, &Barrier::new(1))
        .expect("active helper must finish the cutoff")?;
    assert_eq!(**root, 23);
    assert_eq!(gc.managed_bytes(), 40);
    drop((prefix, live, root));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}
