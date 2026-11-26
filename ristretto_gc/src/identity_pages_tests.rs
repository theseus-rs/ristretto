use super::*;

#[cfg(all(not(target_family = "wasm"), not(miri)))]
thread_local! {
    static UNLINK: std::cell::Cell<Option<(std::sync::mpsc::Sender<()>, std::sync::mpsc::Receiver<()>)>>
        = const { std::cell::Cell::new(None) };
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
pub(super) fn pause_unlink() {
    if let Some((entered, resume)) = UNLINK.take() {
        entered
            .send(())
            .expect("collector at last identity release");
        resume
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("resume collector");
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn allocate_when_woken(
    gc: &Arc<crate::GarbageCollector>,
    value: u64,
    start: &std::sync::Barrier,
) -> Option<crate::Result<crate::GcRootGuard<u64>>> {
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
    let mut allocation = Box::pin(crate::Gc::try_new_async(gc, value));
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

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
#[expect(
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn,
    reason = "forced stopped-collector schedule"
)]
fn capped_peer_reclaims_while_the_last_identity_page_is_being_unlinked() -> crate::Result<()> {
    use crate::{Configuration, GarbageCollector, Gc, GcWeak, ReferenceKind};
    use std::{
        sync::{Barrier, mpsc},
        thread,
        time::Duration,
    };
    const PEERS: usize = 4;
    let payload = ((7_u64, 7_u64), (7_u64, 7_u64));
    let bytes = Gc::allocation_size(&payload)?;
    assert_eq!(bytes, PEERS * Gc::allocation_size(&23_u64)?);
    let mut stalls = Vec::new();
    for counted in [false, true] {
        for independent in [false, true] {
            let gc = GarbageCollector::with_config(
                Configuration::builder()
                    .metadata_shards(1)
                    .concurrent_publication(counted)
                    .independent_phases(independent)
                    .max_heap_bytes(bytes)
                    .allocation_threshold(usize::MAX)
                    .incremental_step_size(1)
                    .max_pause_time_us(0)
                    .build(),
            );
            let dead = Gc::try_new(&gc, payload)?;
            let weak = GcWeak::new(&dead, ReferenceKind::Weak, 0);
            drop(dead);
            gc.collect();
            let (entered, parked) = mpsc::channel();
            let (resume, resumed) = mpsc::channel();
            let worker_gc = gc.clone();
            let worker = thread::spawn(move || {
                UNLINK.set(Some((entered, resumed)));
                while !worker_gc.step() {}
            });
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("page sealed");
            let (send, done) = mpsc::channel();
            let start = Arc::new(Barrier::new(PEERS));
            let peers = (0..PEERS)
                .map(|i| {
                    let (gc, start, send) = (gc.clone(), start.clone(), send.clone());
                    thread::spawn(move || {
                        let value = 23 + i as u64;
                        let result = allocate_when_woken(&gc, value, &start);
                        let _ = send.send((value, result));
                    })
                })
                .collect::<Vec<_>>();
            drop(send);
            // Keep every successful root until all peers have finished, so none
            // can use another peer's allocation credit.
            let deadline = std::time::Instant::now() + Duration::from_secs(4);
            let results = (0..PEERS)
                .map_while(|_| {
                    done.recv_timeout(deadline.checked_duration_since(std::time::Instant::now())?)
                        .ok()
                })
                .collect::<Vec<_>>();
            resume.send(()).expect("resume cleanup");
            worker.join().expect("collector");
            for peer in peers {
                peer.join().expect("peer");
            }
            if results.len() != PEERS || results.iter().any(|(_, root)| root.is_none()) {
                stalls.push((counted, independent));
            }
            for (value, result) in results {
                if let Some(root) = result {
                    assert_eq!(**root?, value);
                }
            }
            assert!(weak.upgrade().is_none());
            gc.collect_full()?;
            assert_eq!(gc.managed_bytes(), 0);
        }
    }
    assert!(
        stalls.is_empty(),
        "capped allocation waited for identity eviction under {stalls:?}"
    );
    Ok(())
}

#[test]
fn an_identity_can_have_only_one_preparer_and_cancellation_releases_its_bit() {
    let index = Index::new(16);
    for id in [1, 63, 64, 1024, u64::MAX] {
        let cancelled = index.prepare(id).expect("vacant");
        assert!(matches!(index.prepare(id), Err(None)));
        assert_eq!(index.get(id), None);
        drop(cancelled);
        let winner = index.prepare(id).expect("cancelled reservation");
        winner.install(11);
        assert!(matches!(index.prepare(id), Err(Some(11))));
        index.remove(id, 12);
        index.remove(id, 0);
        assert_eq!(index.get(id), Some(11));
        index.remove(id, 11);
        assert!(index.heads.iter().all(|head| head.load().len() == 0));
    }
}

#[test]
fn a_delayed_last_release_cannot_unlink_a_replacement_page() {
    let index = Index::new(1);
    index.prepare(1).expect("vacant").install(7);
    let sealed = index
        .retire(1, 7, &mut RetireCursor::default())
        .expect("last slot");
    index.prepare(2).expect("vacant").install(11);
    index.prepare(1).expect("vacant").install(13);
    index.unlink(sealed);
    index.remove(1, 7);
    assert_eq!(index.get(1), Some(13));
    assert_eq!(index.get(2), Some(11));
    index.remove(1, 13);
    index.remove(2, 11);
    assert!(index.heads.iter().all(|h| h.load().len() == 0));
}

#[test]
fn retirement_refreshes_its_cached_page_after_replacement() {
    let index = Index::new(1);
    let mut cursor = RetireCursor::default();
    index.prepare(1).expect("vacant").install(11);
    index.prepare(2).expect("vacant").install(22);
    assert!(index.retire(1, 11, &mut cursor).is_none());
    assert!(cursor.0.iter().any(Option::is_some));
    index.remove(2, 22);
    index.prepare(3).expect("replacement").install(33);
    let sealed = index
        .retire(3, 33, &mut cursor)
        .expect("replacement's last slot");
    index.unlink(sealed);
    assert!(index.heads.iter().all(|head| head.load().len() == 0));
}

#[test]
fn cached_identity_pages_do_not_cross_collectors() {
    let a = Index::new(1);
    let b = Index::new(1);
    a.prepare(1).expect("first heap").install(11);
    b.prepare(1).expect("second heap").install(22);
    assert_eq!(a.get(1), Some(11));
    assert_eq!(b.get(1), Some(22));
    a.remove(1, 11);
    assert_eq!(b.get(1), Some(22));
    b.remove(1, 22);
}
