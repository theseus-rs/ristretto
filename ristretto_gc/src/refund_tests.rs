//! Force one reaper to retain its storage while unrelated allocations recover.
use crate::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind, Result};
use std::cell::Cell;
use std::sync::{Arc, Barrier, mpsc};
use std::time::Duration;

thread_local! {
    static WAITER_PUBLICATION: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
    static REGION_RETURN: Cell<Option<mpsc::Sender<()>>> = const { Cell::new(None) };
    static TERMINAL_CLOSE: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
    static TERMINAL: Cell<bool> = const { Cell::new(false) };
    static SELECTOR: std::cell::RefCell<Option<SelectorGate>> = const { std::cell::RefCell::new(None) };
    static PARK: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
}

pub(crate) fn before_region_return() {
    if let Some(entered) = REGION_RETURN.take() {
        let _ = entered.send(());
    }
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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
        parked_selector_schedule(point, false, true)?;
    }
    Ok(())
}

#[test]
fn cancelling_the_bound_request_before_checkpoint_commit_refunds_once() -> Result<()> {
    parked_selector_schedule(SelectionPoint::AfterBinding, true, true)
}

#[test]
fn ordinary_and_released_donors_stay_public_while_receipt_preparation_is_parked() -> Result<()> {
    for leased in [false, true] {
        parked_selector_schedule(SelectionPoint::BeforePrepare, false, leased)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionPoint {
    BeforePrepare,
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
fn parked_selector_schedule(point: SelectionPoint, cancel_first: bool, leased: bool) -> Result<()> {
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = capped_heap(counted, independent);
            let value = ((1u64, 2u64), (3u64, 4u64));
            let live = Gc::try_new(&gc, value)?;
            let retiring = Gc::try_new(&gc, value)?;
            let mut peers: Vec<_> = (0..4).map(|v| WaitingPeer::new(&gc, v)).collect();
            release_donor(&gc, retiring, leased)?;
            if !leased {
                gc.collect();
            }
            let (enter, parked) = mpsc::channel();
            let (resume, release) = mpsc::channel();
            let heap = gc.clone();
            let selector = std::thread::spawn(move || {
                SELECTOR.replace(Some((point, enter, release)));
                for _ in 0..4096 {
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
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .max_heap_bytes(64)
            .diagnostics(true)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let value = ((1u64, 2u64), (3u64, 4u64));
    let live = Gc::try_new(&gc, value)?;
    let retiring = Gc::try_new(&gc, value)?;
    let prefix: Vec<_> = (0..4096)
        .map(|_| gc.grant_registry.register(&gc, 40))
        .collect::<Result<_>>()?;
    release_donor(&gc, retiring, true)?;
    let mut peer = WaitingPeer::new(&gc, 23);
    for _ in 0..20_000 {
        let before = gc.diagnostics().caller_steps.count;
        assert!(
            peer.poll_when_woken(),
            "a cooperative helper must retain a wake"
        );
        assert!(
            gc.diagnostics().caller_steps.count <= before + 1,
            "one collection batch per poll"
        );
        if peer.result.is_some() {
            break;
        }
    }
    let root = peer
        .result
        .take()
        .expect("active helper must finish the cutoff")?;
    assert_eq!(**root, 23);
    assert_eq!(gc.managed_bytes(), 40);
    drop((prefix, live, root));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[expect(clippy::panic_in_result_fn, reason = "assert forced recovery outcomes")]
fn advisory_buffer_recovery(blocking: bool) -> Result<()> {
    for counted in [false, true] {
        for pledged in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .concurrent_publication(counted)
                    .max_heap_bytes(64)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .diagnostics(true)
                    .build(),
            );
            let payload = ((1_u64, 2_u64), (3_u64, 4_u64));
            let live = Gc::try_new(&gc, payload)?;
            let donor = Gc::try_new(&gc, payload)?;
            let prefix: Vec<_> = (0..4096)
                .map(|_| gc.grant_registry.register(&gc, 40))
                .collect::<Result<_>>()?;
            let claimant = pledged
                .then(|| gc.grant_registry.register(&gc, 32))
                .transpose()?;
            release_donor(&gc, donor, true)?;
            let result = if blocking {
                crate::HeapBuffer::try_with_fn_in(&gc, 8, |_| Ok(7_u8))
            } else {
                let mut future = Box::pin(crate::HeapBuffer::try_with_fn_in_async(&gc, 8, |_| {
                    Ok(7_u8)
                }));
                let ready = Arc::new(Runnable(AtomicBool::new(true)));
                let waker = Waker::from(ready.clone());
                let mut completed = None;
                for _ in 0..20_000 {
                    assert!(
                        ready.0.swap(false, Ordering::AcqRel),
                        "a helper lost its wake"
                    );
                    let before = gc.diagnostics().caller_steps.count;
                    let poll = future.as_mut().poll(&mut Context::from_waker(&waker));
                    assert!(gc.diagnostics().caller_steps.count <= before + 1);
                    if let Poll::Ready(value) = poll {
                        completed = Some(value);
                        break;
                    }
                }
                completed.ok_or_else(|| {
                    crate::Error::CollectionPhaseError(
                        "buffer recovery exhausted its poll budget".into(),
                    )
                })?
            };
            if let Some(claimant) = claimant {
                assert!(matches!(result, Err(crate::Error::OutOfMemory)));
                assert_eq!(gc.managed_bytes(), 64);
                let reservation = claimant.take().ok_or_else(|| {
                    crate::Error::CollectionPhaseError("donor was not paid to the claimant".into())
                })?;
                drop(reservation);
            } else {
                assert_eq!(&*result?, &[7_u8; 8]);
            }
            assert_eq!(gc.managed_bytes(), 32);
            drop((prefix, live));
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

#[test]
fn asynchronous_buffer_preflight_helps_physically_released_donors() -> Result<()> {
    advisory_buffer_recovery(false)
}

#[test]
fn synchronous_buffer_preflight_helps_physically_released_donors() -> Result<()> {
    advisory_buffer_recovery(true)
}

#[expect(clippy::expect_used, reason = "bounded forced allocation schedule")]
pub(super) fn before_terminal_close() {
    if let Some((entered, resume)) = TERMINAL_CLOSE.take() {
        entered.send(()).expect("observer of the terminal decision");
        resume
            .recv_timeout(Duration::from_secs(5))
            .expect("resume the decision");
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "bounded physical-refund schedule"
)]
fn terminal_allocation_close_rechecks_backing_released_by_an_independent_owner() -> Result<()> {
    struct HostValue(u64);
    // SAFETY: this value contains only a primitive. Its host destructor owns no GC edge.
    #[expect(unsafe_code)]
    unsafe impl crate::Trace for HostValue {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn needs_clear() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for HostValue {
        fn drop(&mut self) {
            std::hint::black_box(self.0);
        }
    }
    let charge = Gc::allocation_size(&11_u64)?;
    assert_eq!(charge, Gc::allocation_size(&HostValue(17))?);
    for counted in [false, true] {
        for available in [0, charge - 1, charge] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .threads(1)
                    .concurrent_publication(counted)
                    .max_heap_bytes(charge)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .build(),
            );
            let released = Gc::try_new(&gc, HostValue(17))?;
            assert_eq!(gc.managed_bytes(), charge);
            gc.start();
            let (entered, ready) = mpsc::channel();
            let (resume, wait) = mpsc::channel();
            let peer = {
                let gc = gc.clone();
                std::thread::spawn(move || {
                    TERMINAL_CLOSE.set(Some((entered, wait)));
                    allocate_after_first_poll(&gc, 11, || {})
                })
            };
            let reached = ready.recv_timeout(Duration::from_secs(4));
            drop(released);
            gc.collect_full()?;
            let deadline = Instant::now() + Duration::from_secs(2);
            while gc.managed_bytes() != 0 {
                assert!(Instant::now() < deadline, "physical donor was not returned");
                std::thread::yield_now();
            }
            let retained = gc.reserve_bytes(charge - available)?;
            assert_eq!(gc.managed_bytes(), charge - available);
            let _ = resume.send(());
            assert!(reached.is_ok());
            let result = peer
                .join()
                .map_err(|_| crate::Error::CollectionPhaseError("allocation peer panicked".into()))?
                .ok_or_else(|| {
                    crate::Error::CollectionPhaseError("allocation peer timed out".into())
                })?;
            if available == charge {
                assert_eq!(**result?, 11);
            } else {
                assert!(matches!(result, Err(crate::Error::OutOfMemory)));
            }
            drop(retained);
            gc.stop()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    Ok(())
}

pub(super) fn terminal_snapshot_captured() {
    TERMINAL.set(true);
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "resume a terminal poll on a different guard and credit state"
)]
fn terminal_resume_rechecks_spendable_credit_and_managed_guards() -> Result<()> {
    for refund in [false, true] {
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .max_heap_bytes(64)
                .diagnostics(true)
                .incremental_step_size(1)
                .allocation_threshold(usize::MAX)
                .max_pause_time_us(0)
                .build(),
        );
        let reservation = gc.reserve_bytes(32)?;
        let donor = Gc::try_new(&gc, ((1u64, 2u64), (3u64, 4u64)))?;
        let prefix: Vec<_> = (0..4096)
            .map(|_| gc.grant_registry.register(&gc, 40))
            .collect::<Result<_>>()?;
        release_donor(&gc, donor, true)?;
        let mut peer = WaitingPeer::new(&gc, 23);
        TERMINAL.set(false);
        for _ in 0..10_000 {
            assert!(peer.poll_when_woken());
            if TERMINAL.get() {
                break;
            }
        }
        assert!(
            TERMINAL.replace(false),
            "snapshot captured before the guarded poll"
        );
        let guard_heap = GarbageCollector::new();
        let lock = Gc::try_new(&guard_heap, crate::sync::RwLock::new(0u64))?;
        let guard = (!refund).then(|| lock.read());
        if refund {
            drop(reservation);
        }
        let steps = gc.diagnostics().caller_steps.count;
        assert!(peer.poll_when_woken());
        assert_eq!(gc.diagnostics().caller_steps.count, steps);
        let result = peer
            .result
            .take()
            .expect("credit or guard decides the resumed poll");
        if refund {
            assert_eq!(**result?, 23);
        } else {
            assert!(matches!(result, Err(crate::Error::OutOfMemory)));
        }
        drop((guard, prefix));
    }
    Ok(())
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "force a concurrent directory publication at the physical claim"
)]
fn publishing_a_second_returned_lease_cannot_revoke_the_first_physical_claim() -> Result<()> {
    let gc = capped_heap(false, false);
    let value = ((1u64, 2u64), (3u64, 4u64));
    let first = Gc::try_new(&gc, value)?;
    let second = Gc::try_new(&gc, value)?;
    let a = gc.metadata_for_address(first.as_ptr().addr()).unwrap();
    let b = gc.metadata_for_address(second.as_ptr().addr()).unwrap();
    let lease_a = crate::metadata::ObjectMetadata::borrow_callback(&a, false).unwrap();
    let lease_b = crate::metadata::ObjectMetadata::borrow_callback(&b, false).unwrap();
    drop((first, second));
    gc.collect_full()?;
    drop(lease_a);
    let (enter, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    std::thread::scope(|scope| {
        let heap = &gc;
        let worker = scope.spawn(move || {
            SELECTOR.replace(Some((SelectionPoint::BeforeClaim, enter, release)));
            heap.step();
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        drop(lease_b);
        resume.send(()).unwrap();
        worker.join().unwrap();
    });
    assert_eq!(
        gc.managed_bytes(),
        32,
        "the original step still owns and frees its donor"
    );
    for _ in 0..32 {
        if gc.step() {
            break;
        }
    }
    assert_eq!(gc.managed_bytes(), 0);
    assert!(!gc.has_released_donor_jobs());
    Ok(())
}

#[test]
#[expect(clippy::panic_in_result_fn, reason = "bounded forced schedule")]
fn a_busy_descriptor_registry_does_not_strand_an_unrelated_native_allocation() -> Result<()> {
    #[derive(Default)]
    struct Counts {
        cleared: AtomicUsize,
        dropped: AtomicUsize,
    }
    struct Tracked(Arc<Counts>);
    // SAFETY: the payload holds only host counters, with no managed edges.
    unsafe impl crate::Trace for Tracked {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, _: usize, _: usize) -> crate::TraceProgress {
            self.0.cleared.fetch_add(1, Ordering::Relaxed);
            crate::TraceProgress::Complete
        }
    }
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
    let counts = Arc::new(Counts::default());
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .local_allocation_buffers(true)
            .region_shards(1)
            .max_heap_bytes(24)
            .allocation_threshold(usize::MAX)
            .diagnostics(true)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let global = Gc::try_new(&gc, 11_u64)?;
    let context = gc.register_context();
    let regional = context.with(|| Gc::try_new(&gc, Tracked(counts.clone())))?;
    let keep = context.with(|| Gc::try_new(&gc, 17_u64))?;
    assert_eq!(gc.managed_bytes(), 24);
    assert_eq!(gc.diagnostics().descriptor_occupied_bytes, 16);
    let root = recover_past_region_return(&gc, global, regional, false)?;
    drop((root, keep, context));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(counts.cleared.load(Ordering::Acquire), 1);
    assert_eq!(counts.dropped.load(Ordering::Acquire), 1);
    Ok(())
}

#[expect(
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic_in_result_fn,
    reason = "bounded forced schedule"
)]
fn recover_past_region_return<T: crate::Trace + 'static>(
    gc: &Arc<GarbageCollector>,
    global: crate::GcRootGuard<u64>,
    regional: crate::GcRootGuard<T>,
    payload_registry: bool,
) -> Result<crate::GcRootGuard<u64>> {
    let cap = gc.managed_bytes();
    let global_meta = gc.metadata_for_address(global.as_ptr().addr()).unwrap();
    let region_meta = gc.metadata_for_address(regional.as_ptr().addr()).unwrap();
    let global_lease =
        crate::metadata::ObjectMetadata::borrow_callback(&global_meta, false).unwrap();
    let region_lease =
        crate::metadata::ObjectMetadata::borrow_callback(&region_meta, false).unwrap();
    drop((global, regional));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), cap);
    let (entered, parked) = mpsc::channel();
    let (resume, release) = mpsc::channel();
    let (at_region, attempted) = mpsc::channel();
    let finished = AtomicBool::new(false);
    let recovered = std::thread::scope(|scope| {
        let heap = gc;
        scope.spawn(move || {
            let wait = || {
                entered.send(()).unwrap();
                release.recv_timeout(Duration::from_secs(8)).unwrap();
            };
            if payload_registry {
                heap.region_heap().unwrap().with_parked_registry(wait);
            } else {
                heap.with_parked_descriptor_registry(wait);
            }
        });
        parked.recv_timeout(Duration::from_secs(3)).unwrap();
        drop(region_lease);
        let done = &finished;
        scope.spawn(move || {
            let _worker = crate::recovery::WorkerRegistration::new(heap);
            REGION_RETURN.set(Some(at_region));
            while !done.load(Ordering::Acquire) {
                heap.step();
                std::thread::yield_now();
            }
        });
        let observed = attempted.recv_timeout(Duration::from_secs(3));
        let mut peer = WaitingPeer::new(gc, 23);
        peer.poll_until_parked();
        drop(global_lease);
        let deadline = Instant::now() + Duration::from_secs(1);
        while peer.result.is_none() && Instant::now() < deadline {
            peer.poll_when_woken();
            std::thread::yield_now();
        }
        let recovered = peer.result.take();
        let held_bytes = gc.managed_bytes();
        finished.store(true, Ordering::Release);
        resume.send(()).unwrap();
        assert!(
            observed.is_ok(),
            "worker must attempt returning regional storage"
        );
        assert_eq!(held_bytes, cap);
        recovered
    });
    let root =
        recovered.expect("unrelated global allocation must finish before registry release")?;
    assert_eq!(**root, 23);
    Ok(root)
}

#[test]
#[expect(clippy::panic_in_result_fn, reason = "bounded forced schedule")]
fn a_busy_payload_registry_does_not_strand_an_unrelated_native_allocation() -> Result<()> {
    let example: crate::HeapBuffer<u8> = vec![0; 256].into();
    let size = Gc::allocation_size(&example)?;
    drop(example);
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .region_shards(1)
            .region_layout(256, 0)
            .max_heap_bytes(size + 8)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let global = Gc::try_new(&gc, 11u64)?;
    let buffer =
        crate::HeapBuffer::try_with_fn(gc.region_heap().map(Arc::as_ref), 256, |_| Ok(7u8))?;
    let regional = Gc::try_new(&gc, buffer)?;
    assert_eq!(gc.managed_bytes(), size + 8);
    assert_eq!(gc.region_heap().unwrap().statistics().occupied_bytes, 256);
    let root = recover_past_region_return(&gc, global, regional, true)?;
    drop(root);
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[expect(clippy::panic_in_result_fn, reason = "bounded forced schedule")]
fn recover_past_wrapped_buffer<T: crate::Trace + Send + Sync + 'static>(
    wrap: impl Fn(crate::HeapBuffer<u8>) -> T,
) -> Result<()> {
    let example: crate::HeapBuffer<u8> = vec![0; 256].into();
    let size = Gc::allocation_size(&wrap(example))?;
    let ballast = "x".repeat(256);
    let cap = size + 8 + Gc::allocation_size(&ballast)?;
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .compact_heap(true)
            .region_shards(1)
            .region_layout(256, 0)
            .max_heap_bytes(cap)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let ballast = Gc::try_new(&gc, ballast)?;
    let global = Gc::try_new(&gc, 11u64)?;
    let buffer =
        crate::HeapBuffer::try_with_fn(gc.region_heap().map(Arc::as_ref), 256, |_| Ok(7u8))?;
    let regional = Gc::try_new(&gc, wrap(buffer))?;
    assert_eq!(gc.managed_bytes(), cap);
    assert_eq!(
        gc.region_heap().map(|h| h.statistics().occupied_bytes),
        Some(256)
    );
    let root = recover_past_region_return(&gc, global, regional, true)?;
    drop((root, ballast));
    gc.collect_full()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

#[test]
fn poisoned_standard_locks_preserve_the_inner_clear_cursor() {
    use crate::{Trace, TraceProgress};
    struct Cursor;
    // SAFETY: this probe has no references into the managed heap.
    unsafe impl Trace for Cursor {
        fn trace(&self, _: &GarbageCollector) {}
        fn clear_step(&mut self, offset: usize, budget: usize) -> TraceProgress {
            match (offset, budget) {
                (0, 1) => TraceProgress::Busy,
                (0, 2) => TraceProgress::Continue(2),
                (2, 1) => TraceProgress::Complete,
                _ => TraceProgress::Continue(usize::MAX),
            }
        }
    }
    fn exercise(mut value: impl Trace) {
        assert_eq!(value.clear_step(0, 1), TraceProgress::Busy);
        assert_eq!(value.clear_step(0, 2), TraceProgress::Continue(2));
        assert_eq!(value.clear_step(2, 1), TraceProgress::Complete);
    }
    let mutex = std::sync::Mutex::new(Cursor);
    let rwlock = std::sync::RwLock::new(Cursor);
    std::thread::scope(|scope| {
        assert!(
            scope
                .spawn(|| {
                    let _mutex = mutex.lock();
                    let _write = rwlock.write();
                    panic!("poison both wrappers");
                })
                .join()
                .is_err()
        );
    });
    assert!(mutex.is_poisoned() && rwlock.is_poisoned());
    exercise(mutex);
    exercise(rwlock);
}

#[test]
fn a_busy_optional_payload_registry_does_not_strand_an_unrelated_native_allocation() -> Result<()> {
    recover_past_wrapped_buffer(Some)
}

#[test]
fn a_busy_payload_registry_inside_a_standard_mutex_allows_recovery() -> Result<()> {
    recover_past_wrapped_buffer(std::sync::Mutex::new)
}

#[test]
fn a_busy_payload_registry_inside_a_standard_rwlock_allows_recovery() -> Result<()> {
    recover_past_wrapped_buffer(std::sync::RwLock::new)
}

#[test]
fn a_busy_payload_registry_inside_a_collector_mutex_allows_recovery() -> Result<()> {
    recover_past_wrapped_buffer(crate::sync::Mutex::new)
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "single registered worker schedule"
)]
fn sole_native_worker_recovers_a_global_donor_while_a_descriptor_registry_remains_busy()
-> Result<()> {
    struct Enter(Arc<mpsc::Sender<()>>);
    // SAFETY: only the owned host channel is reachable.
    unsafe impl crate::Trace for Enter {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for Enter {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }

    let (entered, dropping) = mpsc::channel();
    let gc = GarbageCollector::with_config(
        Configuration::builder()
            .threads(1)
            .local_allocation_buffers(true)
            .region_shards(1)
            .max_heap_bytes(16)
            .allocation_threshold(usize::MAX)
            .incremental_step_size(1)
            .max_pause_time_us(0)
            .build(),
    );
    let global = Gc::try_new(&gc, 7u64)?;
    let regional = gc
        .register_context()
        .with(|| Gc::try_new(&gc, Enter(Arc::new(entered))))?;
    let a = gc.metadata_for_address(global.as_ptr().addr()).unwrap();
    let b = gc.metadata_for_address(regional.as_ptr().addr()).unwrap();
    let global_lease = crate::metadata::ObjectMetadata::borrow_callback(&a, false).unwrap();
    let region_lease = crate::metadata::ObjectMetadata::borrow_callback(&b, false).unwrap();
    drop((global, regional));
    gc.collect_full()?;
    gc.start();
    let mut recovered = None;
    gc.with_parked_descriptor_registry(|| {
        drop(region_lease);
        dropping.recv_timeout(Duration::from_secs(3)).unwrap();
        assert_eq!(gc.registered_workers(), 1);
        let mut waiter = WaitingPeer::new(&gc, 29);
        waiter.poll_until_parked();
        drop(global_lease);
        let deadline = Instant::now() + Duration::from_secs(2);
        while waiter.result.is_none() && Instant::now() < deadline {
            waiter.poll_when_woken();
            std::thread::yield_now();
        }
        recovered = waiter.result;
    });
    let result = recovered.expect("native worker reached an independent donor")?;
    assert_eq!(**result, 29);
    drop(result);
    gc.stop()?;
    assert_eq!(gc.managed_bytes(), 0);
    Ok(())
}

pub(crate) fn before_waiter_publication() {
    if let Some((entered, resume)) = WAITER_PUBLICATION.take() {
        assert!(entered.send(()).is_ok(), "waiting for the publisher");
        assert!(
            resume.recv_timeout(Duration::from_secs(8)).is_ok(),
            "resume the publisher"
        );
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    reason = "bounded native worker and executor schedule"
)]
fn sleeping_allocation_finishes_while_an_independent_waiter_is_still_publishing() -> Result<()> {
    // Hold the collector's actual marking callback through the refund. Native
    // finalizers can run independently while this waiter race is being tested.
    struct Park {
        entered: mpsc::Sender<()>,
        resume: std::sync::Mutex<Option<mpsc::Receiver<()>>>,
    }
    // SAFETY: the test fixture contains no managed edges.
    unsafe impl crate::Trace for Park {
        fn trace(&self, _: &GarbageCollector) {
            if let Some(resume) = self.resume.lock().unwrap().take() {
                self.entered.send(()).unwrap();
                resume.recv_timeout(Duration::from_secs(10)).unwrap();
            }
        }
    }
    for allocation_writer in [false, true] {
        let (started, in_trace) = mpsc::channel();
        let (finish, release_worker) = mpsc::channel();
        let payload = Park {
            entered: started,
            resume: std::sync::Mutex::new(Some(release_worker)),
        };
        let cap = Gc::allocation_size(&payload)? + 8;
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .threads(1)
                .diagnostics(true)
                .max_heap_bytes(cap)
                .allocation_threshold(usize::MAX)
                .build(),
        );
        let charge = gc.reserve_private_bytes(8)?;
        let pinned = Gc::try_new(&gc, payload)?;
        gc.start();
        gc.collect();
        in_trace.recv_timeout(Duration::from_secs(3)).unwrap();
        let (asleep, sleeping) = mpsc::channel();
        let (entered, parked) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let (done, completed) = mpsc::channel();
        let observed = std::thread::scope(|scope| {
            let heap = &gc;
            scope.spawn(move || {
                let mut peer = WaitingPeer::new(heap, 57);
                peer.poll_until_parked();
                assert_eq!(heap.diagnostics().allocation_waiters, 1);
                asleep.send(()).unwrap();
                let deadline = Instant::now() + Duration::from_secs(4);
                while peer.result.is_none() && Instant::now() < deadline {
                    peer.poll_when_woken();
                    std::thread::yield_now();
                }
                done.send(peer.result).unwrap();
            });
            sleeping.recv_timeout(Duration::from_secs(3)).unwrap();
            scope.spawn(move || {
                WAITER_PUBLICATION.set(Some((entered, resumed)));
                if allocation_writer {
                    let mut other = WaitingPeer::new(heap, 73);
                    other.poll_until_parked();
                } else {
                    let mut other = Box::pin(crate::recovery::CollectionWait::new(heap, u64::MAX));
                    assert!(
                        other
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .is_pending()
                    );
                }
            });
            parked.recv_timeout(Duration::from_secs(3)).unwrap();
            drop(charge);
            let outcome = completed.recv_timeout(Duration::from_secs(1));
            resume.send(()).unwrap();
            outcome
        });
        finish.send(()).unwrap();
        let root = observed
            .expect("sleeping peer must return while registration is paused")
            .expect("executor returned a value")?;
        assert_eq!(**root, 57);
        assert_eq!(gc.diagnostics().allocation_waiters, 0);
        drop((root, pinned));
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
    }
    Ok(())
}
