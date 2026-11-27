use super::*;

fn insert(index: &Index, key: u64, address: usize) {
    index.insert(index.prepare(key, address));
}
fn remove(index: &Index, key: u64, address: usize) -> bool {
    let mut cursor = Removal::default();
    loop {
        if let Some(result) = index.remove_step(key, address, &mut cursor) {
            return result;
        }
    }
}
fn clean(index: &Index) {
    let mut cursor = Cleanup::default();
    while !index.cleanup_step(&mut cursor) {}
}

#[test]
fn private_cancellation_and_exact_identity_removal() {
    let index = Index::new(1);
    let prepared = index.prepare(1, 100);
    assert_eq!(index.retained_nodes(), 1);
    assert_eq!(index.get(1), None);
    drop(prepared);
    assert_eq!(index.retained_nodes(), 0);
    insert(&index, 1, 100);
    assert!(!remove(&index, 1, 200));
    assert_eq!(index.get(1), Some(100));
    assert!(remove(&index, 1, 100));
    insert(&index, 2, 100);
    assert!(!remove(&index, 1, 100));
    assert_eq!(index.get(2), Some(100));
    assert!(remove(&index, 2, 100));
    clean(&index);
    assert_eq!(index.retained_nodes(), 0);
}

#[test]
fn removal_yields_without_hiding_the_target() {
    let index = Index::new(1);
    for key in 1..=64 {
        insert(&index, key, 100);
    }
    let mut cursor = Removal::default();
    for _ in 0..63 {
        assert_eq!(index.remove_step(1, 100, &mut cursor), None);
        assert_eq!(index.get(1), Some(100));
    }
    assert_eq!(index.remove_step(1, 100, &mut cursor), Some(true));
    assert_eq!(index.get(1), None);
    index.clear();
    assert_eq!(index.retained_nodes(), 0);
}

#[test]
fn stopped_cursor_does_not_retain_a_growing_history() {
    let index = Index::new(1);
    for key in 1..=3 {
        insert(&index, key, 100);
    }
    let mut cursor = Removal::default();
    assert_eq!(index.remove_step(1, 100, &mut cursor), None);
    for key in 1..=3 {
        assert!(remove(&index, key, 100));
    }
    clean(&index);
    assert_eq!(index.retained_nodes(), 2);
    for key in 4..1004 {
        insert(&index, key, 100);
        assert!(remove(&index, key, 100));
        assert_eq!(index.retained_nodes(), 2);
    }
    assert_eq!(index.remove_step(1, 100, &mut cursor), None);
    assert_eq!(index.remove_step(1, 100, &mut cursor), Some(false));
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn helpers_finish_while_the_original_remover_is_stopped() {
    use std::sync::mpsc;
    use std::time::Duration;
    for key in [1, 2] {
        let index = Arc::new(Index::new(1));
        for item in 1..=3 {
            insert(&index, item, 100);
        }
        let (marked, observed) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let worker_index = index.clone();
        let worker = std::thread::spawn(move || {
            let mut cursor = Removal::default();
            loop {
                if let Some(result) = worker_index.test_remove_step(key, 100, &mut cursor, || {
                    marked.send(()).unwrap();
                    resume.recv_timeout(Duration::from_secs(10)).unwrap();
                }) {
                    return result;
                }
            }
        });
        observed.recv_timeout(Duration::from_secs(10)).unwrap();
        for item in 1..=3 {
            remove(&index, item, 100);
        }
        clean(&index);
        let retained = index.retained_nodes();
        assert!(retained <= 3);
        for item in 4..1004 {
            insert(&index, item, 100);
            assert!(remove(&index, item, 100));
            assert_eq!(index.retained_nodes(), retained);
        }
        insert(&index, 1004, 100); // Address reuse with a different identity.
        release.send(()).unwrap();
        assert!(worker.join().unwrap());
        assert_eq!(index.get(1004), Some(100));
        assert!(remove(&index, 1004, 100));
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn concurrent_colliding_buckets_and_exact_retirement() {
    let index = Arc::new(Index::new(1));
    let workers: Vec<_> = (0..4)
        .map(|worker| {
            let index = index.clone();
            std::thread::spawn(move || {
                for offset in 1..=500 {
                    let key = worker * 500 + offset;
                    insert(&index, key, 100);
                    assert_eq!(index.get(key), Some(100));
                    assert!(remove(&index, key, 100));
                    assert_eq!(index.get(key), None);
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    clean(&index);
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn deep_bucket_destruction_uses_bounded_stack() {
    std::thread::Builder::new()
        .stack_size(128 * 1024)
        .spawn(|| {
            let index = Index::new(1);
            for key in 1..=100_000 {
                insert(&index, key, 100);
            }
            index.clear();
            assert_eq!(index.retained_nodes(), 0);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn cancelled_cleanup_returns_unvisited_and_active_dirty_buckets() {
    let index = Index::new(128);
    for key in [1, 2, 65, 129] {
        insert(&index, key, 100);
    }
    for key in [1, 2, 65] {
        assert!(remove(&index, key, 100));
    }
    assert_eq!(index.test_dirty_buckets(), 3);
    let mut cursor = Cleanup::default();
    assert!(!index.cleanup_step(&mut cursor)); // Live 129 in bucket 1.
    assert_eq!(index.test_dirty_buckets(), 1); // Two bits owned by cursor.
    drop(cursor);
    assert_eq!(index.test_dirty_buckets(), 3);
    clean(&index);
    assert_eq!(index.test_dirty_buckets(), 0);
    assert_eq!(index.retained_nodes(), 1);
}

#[test]
fn embedded_markers_preserve_owning_borrows_and_empty_states() {
    let index = Index::new(1);
    insert(&index, 1, 100);
    let live = index.test_edge(1);
    let node = live.next.as_ref().unwrap().clone();
    let deleted = Edge {
        next: Some(node.clone()),
        phase: Phase::Deleted,
    };
    let link = Link::new(live.clone());
    let ordinary = ArcSwapOption::from(Some(node.clone()));
    // Exhaust ArcSwap's fast borrow slots to exercise the fallback path as well
    // as shared debt bookkeeping across ordinary and interior Arc identities.
    let live_borrows: Vec<_> = (0..20).map(|_| link.0.load()).collect();
    let ordinary_borrows: Vec<_> = (0..20).map(|_| ordinary.load()).collect();
    assert!(link.compare(&live, deleted.clone()));
    let deleted_borrows: Vec<_> = (0..20).map(|_| link.0.load()).collect();
    assert!(remove(&index, 1, 100));
    for phase in [Phase::Live, Phase::Deleted, Phase::Detached, Phase::Live] {
        let expected = link.load();
        let empty = Edge { next: None, phase };
        assert!(link.compare(&expected, empty.clone()));
        assert!(link.load().same(&empty));
    }
    ordinary.store(None);
    assert!(live_borrows.iter().all(|value| value.same(&live)));
    assert!(deleted_borrows.iter().all(|value| value.same(&deleted)));
    assert!(
        ordinary_borrows
            .iter()
            .all(|value| Arc::ptr_eq(value.as_ref().unwrap(), &node))
    );
    drop((
        live_borrows,
        deleted_borrows,
        ordinary_borrows,
        live,
        deleted,
        node,
    ));
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn marker_borrows_survive_concurrent_state_changes_and_unlinking() {
    let index = Index::new(1);
    insert(&index, 1, 100);
    let live = index.test_edge(1);
    let node = live.next.as_ref().unwrap().clone();
    let deleted = Edge {
        next: Some(node.clone()),
        phase: Phase::Deleted,
    };
    let link = Arc::new(Link::new(live.clone()));
    let reader_link = link.clone();
    let reader_node = node.clone();
    let reader = std::thread::spawn(move || {
        for _ in 0..16 {
            let borrows: Vec<_> = (0..12).map(|_| reader_link.0.load()).collect();
            std::thread::yield_now();
            for edge in borrows {
                if let Some(node) = &edge.next {
                    assert!(Arc::ptr_eq(node, &reader_node));
                    assert!(edge.phase != Phase::Detached);
                }
            }
        }
    });
    assert!(remove(&index, 1, 100));
    for _ in 0..16 {
        link.swap(deleted.clone());
        link.swap(Edge {
            next: None,
            phase: Phase::Detached,
        });
        link.swap(live.clone());
    }
    reader.join().unwrap();
    drop((link, live, deleted, node));
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn native_head_return_preserves_a_private_insert_and_a_delayed_removal() {
    use std::sync::mpsc;
    use std::time::Duration;
    for deleting in [false, true] {
        let index = Arc::new(Index::new(1));
        insert(&index, 1, 100);
        let (captured, observed) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let worker_index = index.clone();
        let worker = std::thread::spawn(move || {
            let pause = || {
                captured.send(()).unwrap();
                resume.recv_timeout(Duration::from_secs(10)).unwrap();
            };
            if deleting {
                assert_eq!(
                    worker_index.test_remove_step(1, 100, &mut Removal::default(), pause),
                    Some(true)
                );
            } else {
                worker_index.test_insert(worker_index.prepare(3, 100), pause);
            }
        });
        observed.recv_timeout(Duration::from_secs(10)).unwrap();
        insert(&index, 2, 100);
        assert!(remove(&index, 2, 100));
        release.send(()).unwrap();
        worker.join().unwrap();
        assert_eq!(index.get(1), if deleting { None } else { Some(100) });
        assert_eq!(index.get(3), if deleting { None } else { Some(100) });
        clean(&index);
        assert_eq!(index.retained_nodes(), if deleting { 0 } else { 2 });
    }
}

#[test]
fn shared_intents_cancel_or_remain_completed_after_retirement() {
    let index = Index::new(1);
    let prepared = Arc::new(index.prepare(1, 100));
    let helper = prepared.clone();
    drop(prepared);
    assert_eq!(index.retained_nodes(), 1);
    drop(helper);
    assert_eq!(index.retained_nodes(), 0);

    let prepared = index.prepare(2, 100);
    insert(&index, 3, 100); // Invalidate the initial proposal.
    assert!(!index.help(&prepared)); // Retarget, without publishing.
    assert_eq!(index.get(2), None);
    assert!(index.help(&prepared));
    assert!(remove(&index, 2, 100));
    assert!(remove(&index, 3, 100));
    assert!(index.help(&prepared));
    assert_eq!(index.get(2), None);
    drop(prepared);
    clean(&index);
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_helpers_preserve_intent_ownership_through_retirement() {
    let index = Arc::new(Index::new(1));
    let prepared = Arc::new(index.prepare(1, 100));
    insert(&index, 2, 100);
    let other = index.clone();
    let record = prepared.clone();
    let worker = std::thread::spawn(move || {
        while !other.help(&record) {}
        record
    });
    while !index.help(&prepared) {}
    let record = worker.join().unwrap();
    assert!(remove(&index, 1, 100));
    assert!(remove(&index, 2, 100));
    drop(prepared);
    assert!(index.help(&record));
    assert_eq!(index.get(1), None);
    drop(record);
    clean(&index);
    assert_eq!(index.retained_nodes(), 0);
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn shared_insertion_survives_stopped_helpers_and_bounded_history() {
    use std::sync::mpsc;
    use std::time::Duration;
    for after_compare in [false, true] {
        let index = Arc::new(Index::new(1));
        insert(&index, 1, 100);
        let prepared = Arc::new(index.prepare(2, 100));
        let (captured, observed) = mpsc::channel();
        let (release, resume) = mpsc::channel();
        let worker_index = index.clone();
        let worker_prepared = prepared.clone();
        let worker = std::thread::spawn(move || {
            let pause = || {
                captured.send(()).unwrap();
                resume.recv_timeout(Duration::from_secs(10)).unwrap();
            };
            if after_compare {
                worker_index.test_help(&worker_prepared, || {}, pause)
            } else {
                worker_index.test_help(&worker_prepared, pause, || {})
            }
        });
        observed.recv_timeout(Duration::from_secs(10)).unwrap();
        if !after_compare {
            assert!(index.help(&prepared));
        }
        // A different head modifier, not a second helper, must certify the
        // insertion if the original winner stopped immediately after its CAS.
        insert(&index, 3, 100);
        assert!(remove(&index, 2, 100));
        assert!(remove(&index, 1, 100));
        assert!(remove(&index, 3, 100));
        clean(&index);
        let retained = index.retained_nodes();
        assert_eq!(retained, 2); // The intent's candidate and expected head.
        for key in 4..1004 {
            insert(&index, key, 100);
            assert!(remove(&index, key, 100));
            assert_eq!(index.retained_nodes(), retained);
        }
        assert!(index.help(&prepared));
        insert(&index, 1004, 100);
        release.send(()).unwrap();
        assert_eq!(worker.join().unwrap(), after_compare);
        assert!(index.help(&prepared));
        assert_eq!(index.get(2), None);
        assert_eq!(index.get(1004), Some(100));
        drop(prepared);
        assert!(remove(&index, 1004, 100));
        clean(&index);
        assert_eq!(index.retained_nodes(), 0);
    }
}

#[cfg(all(not(target_family = "wasm"), not(miri)))]
#[test]
fn competing_helpers_insert_each_shared_intent_exactly_once() {
    let index = Arc::new(Index::new(8));
    let records: Arc<Vec<_>> = Arc::new((1..=500).map(|key| index.prepare(key, 100)).collect());
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let index = index.clone();
            let records = records.clone();
            std::thread::spawn(move || {
                for record in records.iter() {
                    while !index.help(record) {}
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    for key in 1..=500 {
        assert_eq!(index.get(key), Some(100));
        assert!(remove(&index, key, 100));
        assert!(!remove(&index, key, 100));
    }
    drop(records);
    clean(&index);
    assert_eq!(index.retained_nodes(), 0);
}
