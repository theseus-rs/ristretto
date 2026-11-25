//! Cold registration path when every ordinary page writer is occupied.
use super::{RootStatistics, Scan};
use crate::publication_directory::Directory;
use arc_swap::ArcSwapOption;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub(crate) struct Record {
    pub(super) address: AtomicUsize,
    pub(super) published: AtomicBool,
}

impl Record {
    #[cfg(test)]
    pub(super) fn publish(&self, address: usize) {
        debug_assert_ne!(address, 0);
        self.published.store(true, Ordering::Relaxed);
        self.address.store(address, Ordering::Release);
    }

    pub(crate) fn take_address(&self) -> Option<usize> {
        let address = self.address.swap(0, Ordering::AcqRel);
        (address != 0).then_some(address)
    }

    pub(super) fn key(record: &Arc<Self>) -> usize {
        Arc::as_ptr(record).addr()
    }
}

#[derive(Debug, Default)]
pub(super) struct Overflow {
    // The empty registry does not allocate a directory. A retained view owns
    // record identities only; removal clears the address in every view.
    records: ArcSwapOption<Directory<Arc<Record>>>,
}

impl Overflow {
    #[expect(
        clippy::expect_used,
        reason = "each indexed key owns a distinct live allocation"
    )]
    pub(super) fn reserve(&self) -> Arc<Record> {
        let record = Arc::new(Record::default());
        let key = Record::key(&record) as u64;
        self.records.rcu(|current| {
            let empty = Directory::default();
            let directory = current.as_deref().unwrap_or(&empty);
            let (next, old) = directory
                .insert(key, record.clone())
                .expect("overflow root capacity");
            debug_assert!(old.is_none());
            Some(Arc::new(next))
        });
        record
    }

    pub(super) fn remove(&self, record: &Arc<Record>) {
        let key = Record::key(record) as u64;
        self.records.rcu(|current| {
            let directory = current.as_ref()?;
            if !directory
                .get(key)
                .is_some_and(|entry| Arc::ptr_eq(entry, record))
            {
                return current.clone();
            }
            let (next, _) = directory.remove(key);
            (next.len() != 0).then(|| Arc::new(next))
        });
    }

    pub(super) fn scan(&self, after: &mut usize) -> Scan {
        let view = self.records.load();
        let Some((_, record)) = view
            .as_ref()
            .and_then(|dir| dir.next_after(Some(*after as u64)))
        else {
            return Scan::Done;
        };
        *after = Record::key(record);
        match super::scan_address(&record.address) {
            0 => Scan::Advance,
            address => Scan::Root(address),
        }
    }

    pub(super) fn matching(&self, address: usize) -> Vec<Arc<Record>> {
        let mut result = Vec::new();
        self.for_each(|record| {
            if record.address.load(Ordering::Acquire) == address {
                result.push(record.clone());
            }
        });
        result
    }

    pub(super) fn count(&self, stats: &mut RootStatistics) {
        self.for_each(|record| {
            let address = record.address.load(Ordering::Acquire);
            stats.slot_roots += usize::from(address != 0 && address != super::PENDING_ROOT);
            stats.reserved_slots += usize::from(
                (address == 0 || address == super::PENDING_ROOT)
                    && !record.published.load(Ordering::Acquire),
            );
            stats.overflow_records += 1;
        });
        stats.slot_capacity += stats.overflow_records;
        stats.overflow_record_bytes = stats.overflow_records * size_of::<Record>();
    }

    fn for_each(&self, mut visit: impl FnMut(&Arc<Record>)) {
        let view = self.records.load();
        if let Some(directory) = view.as_deref() {
            let mut after = None;
            while let Some((key, record)) = directory.next_after(after) {
                after = Some(key);
                visit(record);
            }
        }
    }
}

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
mod tests {
    use super::*;
    use std::sync::Barrier;

    #[test]
    fn overlapping_insertions_and_removals_preserve_each_live_record() {
        const WORKERS: usize = 8;
        let registry = Overflow::default();
        let meeting = Barrier::new(WORKERS + 1);
        let old_view = std::thread::scope(|scope| {
            for value in 1..=WORKERS {
                let (registry, meeting) = (&registry, &meeting);
                scope.spawn(move || {
                    meeting.wait();
                    let registration = registry.reserve();
                    registration.publish(value);
                    meeting.wait();
                    meeting.wait();
                    assert_eq!(registration.take_address(), Some(value));
                    registry.remove(&registration);
                });
            }
            meeting.wait();
            meeting.wait();
            let view = registry.records.load_full().unwrap();
            for value in 1..=WORKERS {
                assert_eq!(registry.matching(value).len(), 1);
            }
            meeting.wait();
            // This view remains live as each worker unlinks its registration.
            // It can only retain the numeric records.
            view
        });
        assert_eq!(old_view.len(), WORKERS);
        let mut after = None;
        while let Some((key, record)) = old_view.next_after(after) {
            assert_eq!(record.address.load(Ordering::Acquire), 0);
            after = Some(key);
        }
        assert!(registry.records.load().is_none());
    }
}
