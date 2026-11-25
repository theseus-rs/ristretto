//! Independently locked metadata indexes with bounded, resumable enumeration.
#[cfg(test)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Mutex, MutexGuard, TryLockError};

#[path = "nursery.rs"]
mod nursery;
pub(crate) use nursery::{Objects, Sealed, Slot, SweepEntry, YoungCursor, YoungRetirement};

const MAX_SHARDS: usize = 16;

pub(crate) fn shard_count(requested: usize) -> usize {
    requested.clamp(1, MAX_SHARDS).next_power_of_two()
}

pub(crate) fn address_bucket(address: usize) -> usize {
    (address >> 4) ^ (address >> 12) ^ (address >> 20)
}

#[derive(Debug)]
pub(crate) struct Shards<T> {
    entries: Box<[Mutex<T>]>,
}

impl<T: Default> Shards<T> {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            entries: (0..shard_count(count))
                .map(|_| Mutex::new(T::default()))
                .collect(),
        }
    }
}

impl<T> Shards<T> {
    /// Addresses are aligned; fold higher bits so adjacent descriptors spread out.
    pub(crate) fn lock(&self, address: usize) -> MutexGuard<'_, T> {
        self.lock_bucket(address_bucket(address))
    }

    #[expect(clippy::indexing_slicing, reason = "nonempty power-of-two shard count")]
    pub(crate) fn lock_bucket(&self, bucket: usize) -> MutexGuard<'_, T> {
        self.entries[bucket & (self.entries.len() - 1)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn try_lock(&self, address: usize) -> Option<MutexGuard<'_, T>> {
        self.try_lock_bucket(address_bucket(address))
    }

    #[expect(clippy::indexing_slicing, reason = "nonempty power-of-two shard count")]
    pub(crate) fn try_lock_bucket(&self, bucket: usize) -> Option<MutexGuard<'_, T>> {
        match self.entries[bucket & (self.entries.len() - 1)].try_lock() {
            Ok(guard) => Some(guard),
            Err(TryLockError::Poisoned(error)) => Some(error.into_inner()),
            Err(TryLockError::WouldBlock) => None,
        }
    }

    /// Whole-index operations are reserved for statistics and shutdown. Only one
    /// shard is held at a time; this is not an atomic snapshot during publication.
    pub(crate) fn iter(&self) -> impl Iterator<Item = MutexGuard<'_, T>> {
        self.entries.iter().map(|entry| {
            entry
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Cursor {
    shard: usize,
    addresses: [usize; MAX_SHARDS],
    done: u16,
    count: usize,
    previous: Option<(usize, usize)>,
}

#[expect(
    clippy::indexing_slicing,
    reason = "cursor shard is bounded by the index"
)]
impl Cursor {
    fn begin(&mut self, count: usize) -> bool {
        self.previous = None;
        self.count = count;
        u32::from(self.done) != (1_u32 << count) - 1
    }

    fn rotate(&mut self) {
        for distance in 1..=self.count {
            let next = (self.shard + distance) % self.count;
            if self.done & (1 << next) == 0 {
                self.shard = next;
                return;
            }
        }
    }

    fn selected(&mut self, address: usize) {
        self.previous = Some((self.shard, self.addresses[self.shard]));
        self.addresses[self.shard] = address;
    }

    fn exhausted<V>(&mut self) -> Scan<V> {
        self.done |= 1 << self.shard;
        self.rotate();
        if self.begin(self.count) {
            Scan::Advance
        } else {
            Scan::Done
        }
    }

    /// Revisit an entry whose downstream work could not acquire its locks.
    /// Other unfinished shards get a turn; no address or shard is skipped.
    #[expect(clippy::expect_used, reason = "only a selected entry can be retried")]
    pub(crate) fn retry_entry(&mut self) {
        let (shard, address) = self.previous.take().expect("selected index entry");
        self.addresses[shard] = address;
        self.rotate();
    }
}

pub(crate) enum Scan<V> {
    Entry(usize, V),
    Advance,
    Busy,
    Done,
}

impl<T> Shards<T> {
    /// Selects at most one entry. A busy shard retains its cursor and yields to a peer.
    #[expect(
        clippy::indexing_slicing,
        reason = "cursor shard is bounded by the index"
    )]
    fn scan_with<R>(
        &self,
        cursor: &mut Cursor,
        next: impl FnOnce(&T, usize) -> Option<(usize, R)>,
    ) -> Scan<R> {
        if !cursor.begin(self.entries.len()) {
            return Scan::Done;
        }
        let Some(shard) = self.try_lock_bucket(cursor.shard) else {
            cursor.rotate();
            return Scan::Busy;
        };
        if let Some((address, value)) = next(&shard, cursor.addresses[cursor.shard]) {
            cursor.selected(address);
            Scan::Entry(address, value)
        } else {
            cursor.exhausted()
        }
    }
}

#[cfg(test)]
impl<V> Shards<BTreeMap<usize, V>> {
    pub(crate) fn scan<R>(&self, cursor: &mut Cursor, read: impl FnOnce(&V) -> R) -> Scan<R> {
        self.scan_with(cursor, |map, after| {
            map.range((Excluded(after), Unbounded))
                .next()
                .map(|(&k, v)| (k, read(v)))
        })
    }
}

impl Shards<BTreeSet<usize>> {
    pub(crate) fn scan(&self, cursor: &mut Cursor) -> Scan<()> {
        self.scan_with(cursor, |set, after| {
            set.range((Excluded(after), Unbounded))
                .next()
                .map(|&k| (k, ()))
        })
    }
}

/// An empty shard can skip optional post-trace pruning. Enumeration and sweep
/// retirement still acquire the shard and inspect its authoritative set.
#[derive(Debug)]
pub(crate) struct Remembered {
    sets: Shards<BTreeSet<usize>>,
    nonempty: Box<[std::sync::atomic::AtomicBool]>,
}

impl Remembered {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            sets: Shards::new(count),
            nonempty: (0..shard_count(count)).map(|_| false.into()).collect(),
        }
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "the index is masked by the power-of-two shard count"
    )]
    fn hint(&self, bucket: usize) -> &std::sync::atomic::AtomicBool {
        &self.nonempty[bucket & (self.nonempty.len() - 1)]
    }

    pub(crate) fn may_contain(&self, address: usize) -> bool {
        self.hint(address_bucket(address))
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub(crate) fn lock(&self, address: usize) -> RememberedGuard<'_> {
        self.lock_bucket(address_bucket(address))
    }

    pub(crate) fn lock_bucket(&self, bucket: usize) -> RememberedGuard<'_> {
        RememberedGuard {
            set: self.sets.lock_bucket(bucket),
            hint: self.hint(bucket),
        }
    }

    pub(crate) fn try_lock(&self, address: usize) -> Option<RememberedGuard<'_>> {
        Some(RememberedGuard {
            set: self.sets.try_lock(address)?,
            hint: self.hint(address_bucket(address)),
        })
    }

    pub(crate) fn scan(&self, cursor: &mut Cursor) -> Scan<()> {
        self.sets.scan(cursor)
    }

    pub(crate) fn clear(&self) {
        for (mut set, hint) in self.sets.iter().zip(&self.nonempty) {
            set.clear();
            hint.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

pub(crate) struct RememberedGuard<'a> {
    set: MutexGuard<'a, BTreeSet<usize>>,
    hint: &'a std::sync::atomic::AtomicBool,
}

impl std::ops::Deref for RememberedGuard<'_> {
    type Target = BTreeSet<usize>;
    fn deref(&self) -> &Self::Target {
        &self.set
    }
}
impl RememberedGuard<'_> {
    pub(crate) fn insert(&mut self, address: usize) -> bool {
        if self.set.is_empty() {
            self.hint.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.set.insert(address)
    }
    pub(crate) fn remove(&mut self, address: usize) -> bool {
        let removed = self.set.remove(&address);
        if removed && self.set.is_empty() {
            self.hint.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn busy_shards_rotate_without_certifying_completion_or_losing_entries() {
        for count in [1, 2, 16] {
            let map = Shards::<BTreeMap<usize, usize>>::new(count);
            let set = Shards::<BTreeSet<usize>>::new(count);
            let mut expected = BTreeSet::new();
            for shard in 0..count {
                let first = if shard == 0 { count << 4 } else { shard << 4 };
                for address in [first, first + (1 << 24)] {
                    map.lock(address).insert(address, address);
                    set.lock(address).insert(address);
                    expected.insert(address);
                }
            }
            let mut map_cursor = Cursor::default();
            let mut set_cursor = Cursor::default();
            let held_map = map.lock_bucket(0);
            let held_set = set.lock_bucket(0);
            let mut found_map = BTreeSet::new();
            let mut found_set = BTreeSet::new();
            for _ in 0..count * 4 + 4 {
                match map.scan(&mut map_cursor, |value| *value) {
                    Scan::Entry(address, value) => {
                        assert_eq!(address, value);
                        assert!(found_map.insert(address));
                    }
                    Scan::Advance | Scan::Busy => {}
                    Scan::Done => panic!("held shard was skipped"),
                }
                match set.scan(&mut set_cursor) {
                    Scan::Entry(address, ()) => {
                        assert!(found_set.insert(address));
                    }
                    Scan::Advance | Scan::Busy => {}
                    Scan::Done => panic!("held shard was skipped"),
                }
            }
            assert_eq!(found_map.len(), (count - 1) * 2);
            assert_eq!(found_set, found_map);
            drop((held_map, held_set));
            for _ in 0..4 {
                if let Scan::Entry(address, _) = map.scan(&mut map_cursor, |value| *value) {
                    assert!(found_map.insert(address));
                }
                if let Scan::Entry(address, ()) = set.scan(&mut set_cursor) {
                    assert!(found_set.insert(address));
                }
            }
            assert_eq!(found_map, expected);
            assert_eq!(found_set, expected);
            assert!(matches!(
                map.scan(&mut map_cursor, |value| *value),
                Scan::Done
            ));
            assert!(matches!(set.scan(&mut set_cursor), Scan::Done));
        }
        println!("per-index cursor bytes: {}", size_of::<Cursor>());
    }

    #[test]
    fn retry_revisits_only_the_uncommitted_entry_after_other_shards_progress() {
        let map = Shards::<BTreeMap<usize, usize>>::new(2);
        map.lock(32).insert(32, 42);
        map.lock(16).insert(16, 7);
        let mut cursor = Cursor::default();
        assert!(matches!(
            map.scan(&mut cursor, |value| *value),
            Scan::Entry(32, 42)
        ));
        cursor.retry_entry();
        assert!(matches!(
            map.scan(&mut cursor, |value| *value),
            Scan::Entry(16, 7)
        ));
        assert!(matches!(
            map.scan(&mut cursor, |value| *value),
            Scan::Advance
        ));
        // A failed downstream operation owns no entry: a replacement published
        // at the same address must be looked up afresh when the cursor retries.
        map.lock(32).insert(32, 43);
        assert!(matches!(
            map.scan(&mut cursor, |value| *value),
            Scan::Entry(32, 43)
        ));
        assert!(matches!(map.scan(&mut cursor, |value| *value), Scan::Done));
    }

    #[test]
    fn sparse_scans_charge_each_shard_and_survive_removal() {
        let index = Shards::<BTreeMap<usize, usize>>::new(16);
        index.lock(240).insert(240, 24);
        let mut cursor = Cursor::default();
        for _ in 0..15 {
            assert!(matches!(
                index.scan(&mut cursor, |value| *value),
                Scan::Advance
            ));
        }
        assert!(matches!(
            index.scan(&mut cursor, |value| *value),
            Scan::Entry(240, 24)
        ));
        index.lock(240).remove(&240);
        // Reusing a visited address must not cause a duplicate visit or a loop.
        index.lock(240).insert(240, 25);
        assert!(matches!(
            index.scan(&mut cursor, |value| *value),
            Scan::Done
        ));
        assert!(matches!(
            index.scan(&mut cursor, |value| *value),
            Scan::Done
        ));
        let mut cursor = Cursor::default();
        let mut seen = Vec::new();
        loop {
            match index.scan(&mut cursor, |value| *value) {
                Scan::Entry(address, value) => seen.push((address, value)),
                Scan::Advance | Scan::Busy => {}
                Scan::Done => break,
            }
        }
        assert_eq!(seen, [(240, 25)]);
    }

    #[test]
    fn distinct_shards_can_be_held_together() {
        let index = Shards::<BTreeMap<usize, usize>>::new(16);
        let first = index.lock(16);
        let second = index.lock(32);
        assert!(first.is_empty() && second.is_empty());
    }

    #[test]
    fn set_scans_bound_empty_work_and_leave_late_insertions_for_the_next_pass() {
        let index = Shards::<BTreeSet<usize>>::new(16);
        index.lock(240).insert(240);
        let mut cursor = Cursor::default();
        for _ in 0..15 {
            assert!(matches!(index.scan(&mut cursor), Scan::Advance));
        }
        assert!(matches!(index.scan(&mut cursor), Scan::Entry(240, ())));
        index.lock(240).remove(&240);
        index.lock(240).insert(240);
        index.lock(16).insert(16); // A shard already visited by this pass.
        assert!(matches!(index.scan(&mut cursor), Scan::Done));
        assert!(matches!(index.scan(&mut cursor), Scan::Done));
        let mut cursor = Cursor::default();
        let mut found = Vec::new();
        let mut visits = 0;
        loop {
            visits += 1;
            match index.scan(&mut cursor) {
                Scan::Entry(address, ()) => found.push(address),
                Scan::Advance | Scan::Busy => {}
                Scan::Done => break,
            }
        }
        assert_eq!(found, [16, 240]);
        assert_eq!(visits, 18); // Two entries and sixteen exhausted shards.
        let first = index.lock(16);
        let second = index.lock(240);
        assert!(first.contains(&16) && second.contains(&240));
    }

    #[test]
    fn direct_configuration_extremes_have_bounded_storage() {
        for (requested, expected) in [(0, 1), (1, 1), (3, 4), (16, 16), (usize::MAX, 16)] {
            assert_eq!(Shards::<()>::new(requested).entries.len(), expected);
        }
    }
}
