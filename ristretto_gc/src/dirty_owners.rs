//! An exact-identity fallback for dirty owners whose remembered shard is busy.
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use once_cell::race::OnceBox;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub(crate) struct DirtyOwners {
    overflow: OnceBox<Table>,
}

#[derive(Debug, Default)]
struct Table {
    entries: ArcSwap<Directory<DirtyOwner>>,
    cleanup: AtomicU8,
    cursor: Mutex<Cursor>,
}

const CLEANUP_REQUESTED: u8 = 1;
const CLEANUP_SCANNING: u8 = 2;

#[derive(Debug, Clone, Copy)]
struct DirtyOwner {
    address: usize,
    version: usize,
}

/// Identities are never reused. Hold only the initial upper bound so a
/// suspended young collection does not retain retired directory versions.
#[derive(Debug, Default)]
pub(crate) struct Cursor {
    limit: Option<u64>,
    after: Option<u64>,
}

impl DirtyOwners {
    /// Publish the latest dirtiness for this exact identity. A writer delayed
    /// after its mutation increment must not roll a newer entry back.
    #[expect(
        clippy::expect_used,
        reason = "distinct live IDs fit in the address space"
    )]
    pub(crate) fn publish(&self, id: u64, address: usize, version: usize) {
        let index = &self.overflow.get_or_init(Box::default).entries;
        let mut current = index.load_full();
        loop {
            if current
                .get(id)
                .is_some_and(|entry| entry.version >= version)
            {
                return;
            }
            let next = Arc::new(
                current
                    .insert(id, DirtyOwner { address, version })
                    .expect("live identity fits")
                    .0,
            );
            let seen = index.compare_and_swap(&current, next);
            if Arc::ptr_eq(&seen, &current) {
                return;
            }
            current = Arc::clone(&seen);
        }
    }

    /// Try one directory replacement. A clean trace supplies its starting
    /// version. Terminal sweep can remove any version of an exact dead ID.
    /// A conflicting publisher defers optional pruning or retired-ID cleanup.
    pub(crate) fn try_remove(&self, id: u64, through: usize) -> bool {
        self.remove_after(id, through, || {})
    }

    fn remove_after(&self, id: u64, through: usize, inspected: impl FnOnce()) -> bool {
        let Some(table) = self.overflow.get() else {
            return true;
        };
        let index = &table.entries;
        let current = index.load_full();
        if current.get(id).is_none_or(|entry| entry.version > through) {
            return true;
        }
        inspected();
        let seen = index.compare_and_swap(&current, Arc::new(current.remove(id).0));
        Arc::ptr_eq(&seen, &current)
    }

    pub(crate) fn cursor(&self) -> Cursor {
        Cursor {
            limit: self
                .overflow
                .get()
                .and_then(|table| table.entries.load().last_key()),
            after: None,
        }
    }

    pub(crate) fn next(&self, cursor: &mut Cursor) -> Option<(u64, usize)> {
        let limit = cursor.limit?;
        let current = self.overflow.get()?.entries.load();
        if let Some((id, entry)) = current.next_after(cursor.after)
            && id <= limit
        {
            cursor.after = Some(id);
            return Some((id, entry.address));
        }
        cursor.limit = None;
        None
    }

    /// The address has already been unlinked. A directory conflict leaves
    /// only a numeric ID and address; it cannot delay the payload's refund.
    pub(crate) fn retire(&self, id: u64) {
        self.retire_after(id, || {});
    }

    fn retire_after(&self, id: u64, inspected: impl FnOnce()) {
        if !self.remove_after(id, usize::MAX, inspected) {
            self.request_cleanup();
        }
    }

    fn request_cleanup(&self) {
        if let Some(table) = self.overflow.get() {
            table.cleanup.fetch_or(CLEANUP_REQUESTED, Ordering::Release);
        }
    }

    pub(crate) fn cleanup_pending(&self) -> bool {
        self.overflow
            .get()
            .is_some_and(|table| table.cleanup.load(Ordering::Acquire) != 0)
    }

    /// Inspect at most one identity and attempt at most one directory CAS.
    /// A request arriving during a pass survives its completion, including
    /// when the cursor already passed the newly retired identity.
    pub(crate) fn cleanup_step(&self, live: impl FnOnce(u64, usize) -> bool) {
        let Some(table) = self.overflow.get() else {
            return;
        };
        let Ok(mut cursor) = table.cursor.try_lock() else {
            return;
        };
        let state = table.cleanup.load(Ordering::Acquire);
        if state == 0 {
            return;
        }
        if state & CLEANUP_SCANNING == 0 {
            table.cleanup.swap(CLEANUP_SCANNING, Ordering::AcqRel);
            *cursor = self.cursor();
        }
        if let Some((id, address)) = self.next(&mut cursor) {
            if !live(id, address) {
                self.retire(id);
            }
        } else {
            table.cleanup.fetch_and(!CLEANUP_SCANNING, Ordering::AcqRel);
        }
    }

    pub(crate) fn clear(&self) {
        if let Some(overflow) = self.overflow.get() {
            overflow.entries.store(Arc::default());
            overflow.cleanup.store(0, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clean_scan_cannot_remove_a_later_version_or_roll_back_a_delayed_writer() {
        let owners = DirtyOwners::default();
        owners.publish(7, 16, 2);
        assert!(!owners.remove_after(7, 2, || owners.publish(7, 16, 3)));
        owners.publish(7, 16, 1);
        assert!(owners.try_remove(7, 2));
        assert_eq!(owners.next(&mut owners.cursor()), Some((7, 16)));
        // A primary-only write can advance the verified clean scan's version.
        assert!(owners.try_remove(7, 4));
        assert!(owners.next(&mut owners.cursor()).is_none());
    }

    #[test]
    fn retirement_conflicts_request_bounded_cleanup_even_after_the_cursor_passes() {
        let owners = DirtyOwners::default();
        owners.publish(1, 16, 1);
        owners.publish(3, 48, 1);
        owners.retire_after(3, || owners.publish(2, 32, 1));
        assert!(owners.cleanup_pending());
        owners.cleanup_step(|id, address| {
            assert_eq!((id, address), (1, 16));
            true
        });
        // Force a second failed removal behind the current cursor.
        owners.retire_after(1, || owners.publish(2, 32, 2));
        let mut visited = Vec::new();
        for _ in 0..16 {
            if !owners.cleanup_pending() {
                break;
            }
            let before = visited.len();
            owners.cleanup_step(|id, _| {
                visited.push(id);
                id == 2
            });
            assert!(visited.len() <= before + 1);
        }
        assert!(!owners.cleanup_pending());
        assert_eq!(visited, [2, 3, 1, 2]);
        let mut cursor = owners.cursor();
        assert_eq!(owners.next(&mut cursor), Some((2, 32)));
        assert!(owners.next(&mut cursor).is_none());
    }
}
