use super::*;
use crate::{Configuration, GarbageCollector, Gc, Result, Trace};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

fn assert_structure<V>(node: &Node<V>, parent_bit: Option<u32>) -> usize {
    match &node.kind {
        Kind::Leaf(_) => {
            assert_eq!(node.first, node.last);
            0
        }
        Kind::Branch { bit, left, right } => {
            assert!(*bit < 64);
            assert!(parent_bit.is_none_or(|parent| *bit < parent));
            assert_eq!(split(node.first, node.last), Some(*bit));
            assert_eq!(left.first & (1_u64 << bit), 0);
            assert_ne!(right.first & (1_u64 << bit), 0);
            assert!(left.last < right.first);
            1 + assert_structure(left, Some(*bit)).max(assert_structure(right, Some(*bit)))
        }
    }
}

#[test]
fn every_key_bit_and_extreme_preserve_snapshot_order_and_bounded_paths() {
    let mut directory = Directory::default();
    let mut oracle = BTreeMap::new();
    let keys = [0, u64::MAX]
        .into_iter()
        .chain((0..64).map(|bit| 1_u64 << bit));
    let mut snapshots = Vec::new();
    for (value, key) in keys.enumerate() {
        let (next, previous) = directory.insert(key, value).expect("finite directory");
        assert_eq!(previous, oracle.insert(key, value));
        snapshots.push((directory, oracle.len() - usize::from(previous.is_none())));
        directory = next;
        let root = directory.root.as_ref().expect("nonempty");
        assert!(assert_structure(root, None) <= 64);
        for (key, value) in &oracle {
            assert_eq!(directory.get(*key), Some(value));
        }
        let mut cursor = None;
        for (expected_key, expected_value) in &oracle {
            let (key, value) = directory.next_after(cursor).expect("next exact key");
            assert_eq!((key, value), (*expected_key, expected_value));
            cursor = Some(key);
        }
        assert!(directory.next_after(cursor).is_none());
    }
    for (snapshot, len) in snapshots {
        assert_eq!(snapshot.len(), len);
    }
    for key in oracle.keys().rev() {
        let snapshot = directory.clone();
        let (next, value) = directory.remove(*key);
        assert_eq!(value.as_ref(), oracle.get(key));
        assert_eq!(snapshot.get(*key), value.as_ref());
        assert!(next.get(*key).is_none());
        directory = next;
    }
    assert_eq!(directory.len(), 0);
    assert!(directory.root.is_none());
}

#[test]
fn absent_keys_never_alias_and_replacement_does_not_change_old_values() {
    let first = Directory::default().insert(0x1000, 7).expect("one entry").0;
    let second = first.insert(0x1000, 11).expect("replacement");
    assert_eq!(second.1, Some(7));
    assert_eq!(second.0.len(), 1);
    assert_eq!(first.get(0x1000), Some(&7));
    assert_eq!(second.0.get(0x1000), Some(&11));
    for key in [0, 0xfff, 0x1001, u64::MAX] {
        assert!(second.0.get(key).is_none());
        let (unchanged, removed) = second.0.remove(key);
        assert!(removed.is_none());
        assert_eq!(unchanged.get(0x1000), Some(&11));
    }
    let exhausted = Directory::<u8> {
        root: None,
        len: usize::MAX,
    };
    assert!(exhausted.insert(1, 1).is_none());
    assert!(exhausted.root.is_none());
}

#[test]
fn enumeration_after_deleted_and_absent_keys_matches_numeric_order() {
    let mut directory = Directory::default();
    let mut oracle = BTreeMap::new();
    for key in [3, 8, 15, 16, 17, 63, 64, 65, 127, 128, 250] {
        directory = directory.insert(key, key).expect("finite keys").0;
        oracle.insert(key, key);
    }
    for removed in [16, 63, 3, 250] {
        directory = directory.remove(removed).0;
        oracle.remove(&removed);
        for cursor in 0..=255 {
            let expected = oracle
                .range((
                    std::ops::Bound::Excluded(cursor),
                    std::ops::Bound::Unbounded,
                ))
                .next();
            assert_eq!(
                directory.next_after(Some(cursor)),
                expected.map(|(key, value)| (*key, value))
            );
        }
        assert!(
            directory
                .root
                .as_ref()
                .is_none_or(|root| assert_structure(root, None) <= 64)
        );
    }
}

struct Payload(Arc<AtomicUsize>);
// SAFETY: the counter owns no managed references.
unsafe impl Trace for Payload {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Payload {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
#[expect(
    clippy::panic_in_result_fn,
    reason = "assert exact finite GC fixture and stale descriptor ownership"
)]
fn old_directory_storage_identities_do_not_retain_retired_payload_credit() -> Result<()> {
    let gc = GarbageCollector::with_config(Configuration {
        allocation_threshold: usize::MAX,
        ..Configuration::default()
    });
    let dropped = Arc::new(AtomicUsize::new(0));
    let first = Gc::try_new(&gc, Payload(dropped.clone()))?;
    let second = Gc::try_new(&gc, Payload(dropped.clone()))?;
    let size = Gc::allocation_size(&Payload(Arc::new(AtomicUsize::new(0))))?;
    let mut directory = Directory::default();
    for value in [&first, &second] {
        let address = value.as_ptr() as usize;
        let metadata = gc.metadata_for_address(address).expect("live descriptor");
        directory = directory
            .insert(address as u64, metadata)
            .expect("two identities")
            .0;
    }
    let parked_snapshot = directory.clone();
    drop(second);
    gc.collect_full()?;
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), size);
    drop(first);
    gc.stop()?;
    assert_eq!(dropped.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(parked_snapshot.len(), 2);
    // No payload access is authorized by either stale directory. Their values
    // remain inert descriptors; their eventual drop must not repeat destruction.
    drop((directory, parked_snapshot));
    assert_eq!(dropped.load(Ordering::Relaxed), 2);
    Ok(())
}
