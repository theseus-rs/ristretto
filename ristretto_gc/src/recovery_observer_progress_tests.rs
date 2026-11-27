//! Park a real registrar immediately before it publishes a refund observation.
use super::*;
use std::cell::Cell;
use std::sync::mpsc;
use std::time::Duration;

thread_local! {
    static REFUND: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
    static PARK: Cell<Option<(mpsc::Sender<()>, mpsc::Receiver<()>)>> = const { Cell::new(None) };
}

pub(super) fn before_publication() {
    if let Some((entered, resume)) = PARK.take() {
        let _ = entered.send(());
        let _ = resume.recv_timeout(Duration::from_secs(10));
    }
}

pub(super) fn after_refund_snapshot() {
    if let Some((entered, resume)) = REFUND.take() {
        let _ = entered.send(());
        let _ = resume.recv_timeout(Duration::from_secs(10));
    }
}

pub(super) fn while_registration_is_parked<T>(gc: &GarbageCollector, f: impl FnOnce() -> T) -> T {
    std::thread::scope(|threads| {
        let (entered, parked) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        threads.spawn(move || {
            PARK.set(Some((entered, resume)));
            assert!(gc.observe_space(1).is_ok());
        });
        assert_eq!(parked.recv_timeout(Duration::from_secs(5)), Ok(()));
        let result = f();
        drop(release);
        result
    })
}

#[test]
fn parked_registration_allows_a_peer_to_enroll_cancel_and_spend_a_refund() {
    let gc = GarbageCollector::with_config(
        crate::Configuration::builder()
            .max_heap_bytes(8)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let charge = gc.reserve_private_bytes(8).unwrap();
    let installed = gc.observe_space(8).unwrap();
    std::thread::scope(|threads| {
        let (send, result) = mpsc::channel();
        let collector = &gc;
        let offered = &installed;
        while_registration_is_parked(&gc, || {
            threads.spawn(move || {
                let mut cx = Context::from_waker(Waker::noop());
                let mut cancelled = Box::pin(collector.make_room_async(8));
                assert!(cancelled.as_mut().poll(&mut cx).is_pending());
                drop(cancelled);
                let mut peer = Box::pin(crate::Gc::try_new_async(collector, 42_u64));
                assert!(peer.as_mut().poll(&mut cx).is_pending());
                drop(charge);
                let Poll::Ready(Ok(root)) = peer.as_mut().poll(&mut cx) else {
                    panic!("the peer should own the returned bytes");
                };
                let _ = send.send((
                    **root,
                    offered.was_offered(),
                    collector.completed_full_ticket(),
                ));
            });
            assert_eq!(
                result.recv_timeout(Duration::from_secs(2)),
                Ok((42, true, 0))
            );
        });
    });
    drop(installed);
    gc.collect_full().unwrap();
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn parked_refund_delivers_to_its_original_snapshot_after_a_replacement() {
    let gc =
        GarbageCollector::with_config(crate::Configuration::builder().max_heap_bytes(8).build());
    let charge = gc.reserve_private_bytes(8).unwrap();
    let prior = gc.observe_space(8).unwrap();
    let cancelled = gc.observe_space(4).unwrap();
    std::thread::scope(|threads| {
        let (entered, parked) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let refunder = threads.spawn(move || {
            REFUND.set(Some((entered, resume)));
            drop(charge);
        });
        parked.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(prior.was_offered());
        let stolen = gc.reserve_private_bytes(8).unwrap();
        drop(cancelled);
        let later = gc.observe_space(8).unwrap();
        drop(release);
        refunder.join().unwrap();
        assert!(prior.was_offered());
        assert!(!later.was_offered());
        drop(stolen);
        assert!(later.was_offered());
    });
    drop(prior);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(gc.recovery.observing.load(Ordering::Acquire), 0);
}

#[test]
fn last_observer_cleanup_preserves_a_concurrent_registration() {
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};
    loom::model(|| {
        // Model a current group with a previous one. A new observer joins
        // the current group, then a refund rotates it into the next history.
        let directory = Arc::new(AtomicUsize::new(1));
        let active = Arc::new(AtomicUsize::new(1));
        let history = Arc::new([AtomicBool::new(true), AtomicBool::new(true)]);
        let dropping = {
            let (directory, active, history) = (directory.clone(), active.clone(), history.clone());
            loom::thread::spawn(move || {
                let captured = directory.load(SeqCst);
                if active.fetch_sub(1, SeqCst) == 1 {
                    history.get(captured - 1).unwrap().store(false, SeqCst);
                }
            })
        };
        active.fetch_add(1, SeqCst);
        let owned = directory.load(SeqCst);
        assert_eq!(owned, 1);
        directory.store(2, SeqCst);
        dropping.join().unwrap();
        assert!(history.get(1).unwrap().load(SeqCst));
        assert_eq!(active.load(SeqCst), 1);
    });
}
