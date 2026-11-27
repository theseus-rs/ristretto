//! Exact primary membership. Writers can batch enrollment behind a shard guard,
//! while scans and terminal removal use its independently published directory.
use super::{Scan, address_bucket, dirty_owners, shard_count};
use crate::ObjectId;
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

#[derive(Debug, Default)]
struct Shard {
    writer: Mutex<()>,
    members: AtomicUsize,
    entries: ArcSwap<Directory<usize>>,
}

#[derive(Debug)]
pub(crate) struct Remembered {
    shards: Box<[Shard]>,
    members: AtomicUsize,
    pub(crate) dirty: dirty_owners::DirtyOwners,
}

#[derive(Debug, Default)]
pub(crate) struct RememberedCursor {
    shard: usize,
    after: Option<u64>,
    limit: Option<u64>,
}

pub(crate) struct RememberedGuard<'a> {
    _writer: MutexGuard<'a, ()>,
    set: &'a Remembered,
}

impl Remembered {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            shards: (0..shard_count(count)).map(|_| Shard::default()).collect(),
            members: AtomicUsize::new(0),
            dirty: dirty_owners::DirtyOwners::default(),
        }
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "a power-of-two shard count masks the index"
    )]
    fn bucket(&self, bucket: usize) -> &Shard {
        &self.shards[bucket & (self.shards.len() - 1)]
    }

    pub(crate) fn may_contain(&self, address: usize) -> bool {
        self.bucket(address_bucket(address))
            .members
            .load(Ordering::Relaxed)
            != 0
    }

    pub(crate) fn may_prune(&self) -> bool {
        self.members.load(Ordering::Relaxed) != 0 || self.dirty.initialized()
    }

    #[cfg(test)]
    pub(crate) fn lock(&self, address: usize) -> RememberedGuard<'_> {
        self.lock_bucket(address_bucket(address))
    }

    #[cfg(test)]
    pub(crate) fn lock_bucket(&self, bucket: usize) -> RememberedGuard<'_> {
        RememberedGuard {
            _writer: self
                .bucket(bucket)
                .writer
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            set: self,
        }
    }

    pub(crate) fn try_lock(&self, address: usize) -> Option<RememberedGuard<'_>> {
        let guard = match self.bucket(address_bucket(address)).writer.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(error)) => error.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        Some(RememberedGuard {
            _writer: guard,
            set: self,
        })
    }

    #[expect(
        clippy::expect_used,
        reason = "distinct live identities fit in addressable memory"
    )]
    pub(crate) fn insert(&self, address: usize, id: ObjectId) {
        let key = id.to_i64().cast_unsigned();
        let shard = self.bucket(address_bucket(address));
        let index = &shard.entries;
        let mut current = index.load_full();
        if current.get(key).is_some() {
            return;
        }
        // Reserve before publishing. In-flight attempts can only overestimate
        // the optional prune hint. A successful remover returns this count.
        self.members.fetch_add(1, Ordering::Relaxed);
        shard.members.fetch_add(1, Ordering::Relaxed);
        loop {
            let next = Arc::new(current.insert(key, address).expect("unique birth fits").0);
            let seen = index.compare_and_swap(&current, next);
            if Arc::ptr_eq(&seen, &current) {
                return;
            }
            if seen.get(key).is_some() {
                shard.members.fetch_sub(1, Ordering::Relaxed);
                self.members.fetch_sub(1, Ordering::Relaxed);
                return;
            }
            current = Arc::clone(&seen);
        }
    }

    pub(crate) fn remove(&self, address: usize, id: ObjectId) -> bool {
        let key = id.to_i64().cast_unsigned();
        let shard = self.bucket(address_bucket(address));
        let index = &shard.entries;
        let mut current = index.load_full();
        while current.get(key) == Some(&address) {
            let next = Arc::new(current.remove(key).0);
            let seen = index.compare_and_swap(&current, next);
            if Arc::ptr_eq(&seen, &current) {
                shard.members.fetch_sub(1, Ordering::Relaxed);
                self.members.fetch_sub(1, Ordering::Relaxed);
                return true;
            }
            current = Arc::clone(&seen);
        }
        false
    }

    pub(crate) fn scan(&self, cursor: &mut RememberedCursor) -> Scan<ObjectId> {
        let Some(shard) = self.shards.get(cursor.shard) else {
            return Scan::Done;
        };
        let entries = shard.entries.load();
        if cursor.after.is_none() {
            cursor.limit = entries.last_key();
        }
        // A stream of new births cannot extend this shard's scan. Writers
        // publishing behind the cursor separately notify the active young epoch.
        if let Some((id, &address)) = entries.next_after(cursor.after)
            && cursor.limit.is_some_and(|limit| id <= limit)
        {
            cursor.after = Some(id);
            return Scan::Entry(address, ObjectId::from_i64(id.cast_signed()));
        }
        cursor.shard += 1;
        cursor.after = None;
        if cursor.shard == self.shards.len() {
            Scan::Done
        } else {
            Scan::Advance
        }
    }

    pub(crate) fn clear(&self) {
        for shard in &self.shards {
            shard.entries.store(Arc::default());
            shard.members.store(0, Ordering::Relaxed);
        }
        self.members.store(0, Ordering::Relaxed);
        self.dirty.clear();
    }
}

impl RememberedGuard<'_> {
    pub(crate) fn insert(&self, address: usize, id: ObjectId) {
        self.set.insert(address, id);
    }

    #[cfg(test)]
    fn identity(&self, address: usize) -> Option<ObjectId> {
        let current = self.set.bucket(address_bucket(address)).entries.load();
        let mut after = None;
        while let Some((id, entry)) = current.next_after(after) {
            if *entry == address {
                return Some(ObjectId::from_i64(id.cast_signed()));
            }
            after = Some(id);
        }
        None
    }

    #[cfg(test)]
    pub(crate) fn contains(&self, address: usize) -> bool {
        self.identity(address).is_some()
    }

    #[cfg(test)]
    pub(crate) fn remove(&self, address: usize) -> bool {
        self.identity(address)
            .is_some_and(|id| self.set.remove(address, id))
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.set.members.load(Ordering::Relaxed) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn late_removal_preserves_a_new_member_at_the_same_address() {
        let set = Remembered::new(1);
        let address = 16;
        // Thread-local ID leases allow the later birth to have a lower number.
        let old = ObjectId::from_i64(255);
        let new = ObjectId::from_i64(4);
        let parked = set.lock(address);
        parked.insert(address, old);
        assert!(set.remove(address, old));
        set.insert(address, new);
        // Simulate a former publisher resuming around its validation and cleanup.
        parked.insert(address, old);
        assert!(set.remove(address, old));
        assert!(!set.remove(address, old));
        let mut scan = RememberedCursor::default();
        assert!(matches!(set.scan(&mut scan), Scan::Entry(16, id) if id == new));
        assert!(matches!(set.scan(&mut scan), Scan::Done));
        assert!(set.remove(address, new));
        assert!(!set.may_prune());
    }

    #[test]
    fn a_stream_of_later_births_does_not_extend_a_young_scan() {
        let set = Remembered::new(1);
        set.insert(16, ObjectId::from_i64(1));
        set.insert(32, ObjectId::from_i64(2));
        let mut scan = RememberedCursor::default();
        assert!(matches!(set.scan(&mut scan), Scan::Entry(16, _)));
        set.insert(48, ObjectId::from_i64(3));
        assert!(matches!(set.scan(&mut scan), Scan::Entry(32, _)));
        set.insert(64, ObjectId::from_i64(4));
        assert!(matches!(set.scan(&mut scan), Scan::Done));
    }
}
