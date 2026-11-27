//! Reduced models of the *gated* buffered-barrier handshake, not a model of the
//! entire collector or evidence that the publication gate can be removed.
#![cfg(not(target_family = "wasm"))]
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::sync::{Arc, Mutex, RwLock};
use loom::thread;

#[derive(Default)]
struct Snapshot {
    publication: RwLock<()>,
    rooted: AtomicBool,
    pending: AtomicUsize,
    ready: Mutex<Option<Arc<AtomicBool>>>,
    marked: AtomicBool,
}

#[test]
fn cancellation_transfers_ownership_before_phase_closure() {
    loom::model(|| {
        let state = Arc::new(Snapshot::default());
        state.rooted.store(true, Ordering::Release);
        let producer = {
            let state = state.clone();
            thread::spawn(move || {
                let _reader = state.publication.read().unwrap();
                let record = Arc::new(AtomicBool::new(true));
                state.pending.fetch_add(1, Ordering::AcqRel);
                *state.ready.lock().unwrap() = Some(record.clone());
                state.rooted.store(false, Ordering::Release);
                // Cancellation drops the producer's last local owner. The ready
                // registry must own the record until collector publication.
                drop(record);
            })
        };
        {
            let _writer = state.publication.write().unwrap();
            if state.pending.load(Ordering::Acquire) == 0 {
                assert!(state.rooted.load(Ordering::Acquire));
            } else {
                let record = state.ready.lock().unwrap().take().unwrap();
                assert!(record.load(Ordering::Acquire));
                state.marked.store(true, Ordering::Release);
                state.pending.fetch_sub(1, Ordering::AcqRel);
                assert!(state.marked.load(Ordering::Acquire));
            }
        }
        producer.join().unwrap();
        assert!(state.marked.load(Ordering::Acquire) || state.pending.load(Ordering::Acquire) == 1);
    });
}

#[test]
fn popped_records_remain_reserved_until_mark_publication() {
    loom::model(|| {
        let state = Arc::new(Snapshot::default());
        state.pending.store(1, Ordering::Release);
        *state.ready.lock().unwrap() = Some(Arc::new(AtomicBool::new(true)));
        let flush = {
            let state = state.clone();
            thread::spawn(move || {
                let record = state.ready.lock().unwrap().take().unwrap();
                assert!(record.load(Ordering::Acquire));
                state.marked.store(true, Ordering::Release);
                state.pending.fetch_sub(1, Ordering::AcqRel);
            })
        };
        {
            let _writer = state.publication.write().unwrap();
            // Queue emptiness alone never authorizes closing a snapshot.
            if state.pending.load(Ordering::Acquire) == 0 {
                assert!(state.marked.load(Ordering::Acquire));
            }
        }
        flush.join().unwrap();
        assert_eq!(state.pending.load(Ordering::Acquire), 0);
        assert!(state.marked.load(Ordering::Acquire));
    });
}

#[test]
fn initial_mark_claim_hands_off_its_reservation_before_closure() {
    loom::model(|| {
        let claims = Arc::new(RwLock::new(()));
        let queued = Arc::new(Mutex::new(true));
        let flight = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicBool::new(false));
        let worker = {
            let (claims, queued, flight, completed) = (
                claims.clone(),
                queued.clone(),
                flight.clone(),
                completed.clone(),
            );
            thread::spawn(move || {
                let claim = claims.read().unwrap();
                assert!(std::mem::replace(&mut *queued.lock().unwrap(), false));
                flight.fetch_add(1, Ordering::AcqRel);
                drop(claim);
                completed.store(true, Ordering::Release);
                flight.fetch_sub(1, Ordering::AcqRel);
            })
        };
        {
            let _closure = claims.write().unwrap();
            if !*queued.lock().unwrap() && flight.load(Ordering::Acquire) == 0 {
                assert!(completed.load(Ordering::Acquire));
            }
        }
        worker.join().unwrap();
    });
}

#[test]
fn remembered_pruning_cannot_erase_a_newer_mutation() {
    loom::model(|| {
        let version = Arc::new(Mutex::new(0_usize));
        let remembered = Arc::new(Mutex::new(true));
        let writer = {
            let (version, remembered) = (version.clone(), remembered.clone());
            thread::spawn(move || {
                *version.lock().unwrap() += 1;
                // Notification may be delayed after advancing the version.
                *remembered.lock().unwrap() = true;
            })
        };
        {
            let version = version.lock().unwrap();
            // The scan observed version zero. Hold the object shard across both
            // this comparison and removal, matching prune_remembered.
            if *version == 0 {
                *remembered.lock().unwrap() = false;
            }
        }
        writer.join().unwrap();
        assert!(*remembered.lock().unwrap());
    });
}
