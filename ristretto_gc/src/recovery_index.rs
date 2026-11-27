//! Persistent indexes for a single immutable waiter snapshot. A transaction
//! copies only the radix paths changed by its registrations or deliveries.
use super::{GrantWatch, Waiter, WaiterId};
use crate::publication_directory::Directory;
use arc_swap::ArcSwapOption;
use std::alloc::Layout;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Waker;

type Key = (u64, u64);

#[derive(Debug, Clone)]
pub(super) struct Index<V>(Directory<V>);

impl<V> Default for Index<V> {
    fn default() -> Self {
        Self(Directory::default())
    }
}

impl<V: Clone> Index<V> {
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub(super) fn get(&self, key: u64) -> Option<&V> {
        self.0.get(key)
    }
    pub(super) fn next(&self, after: Option<u64>) -> Option<(u64, &V)> {
        self.0.next_after(after)
    }

    #[expect(
        clippy::expect_used,
        clippy::unwrap_in_result,
        reason = "only directory count overflow fails; each live key owns addressable storage"
    )]
    pub(super) fn insert(&mut self, key: u64, value: V) -> Option<V> {
        let (next, old) = self
            .0
            .insert(key, value)
            .expect("addressable registration count");
        self.0 = next;
        old
    }

    pub(super) fn remove(&mut self, key: u64) -> Option<V> {
        let (next, old) = self.0.remove(key);
        self.0 = next;
        old
    }
}

#[derive(Debug, Clone)]
pub(super) struct Pairs<V> {
    groups: Index<Index<V>>,
    len: usize,
}

impl<V> Default for Pairs<V> {
    fn default() -> Self {
        Self {
            groups: Index::default(),
            len: 0,
        }
    }
}

impl<V: Clone> Pairs<V> {
    pub(super) fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub(super) fn next(&self, after: Option<Key>) -> Option<(Key, &V)> {
        if let Some((group, id)) = after
            && let Some((next, value)) = self.groups.get(group).and_then(|ids| ids.next(Some(id)))
        {
            return Some(((group, next), value));
        }
        let (group, ids) = self.groups.next(after.map(|(group, _)| group))?;
        ids.next(None).map(|(id, value)| ((group, id), value))
    }

    pub(super) fn insert(&mut self, (group, id): Key, value: V) {
        let mut ids = self.groups.get(group).cloned().unwrap_or_default();
        if ids.insert(id, value).is_none() {
            self.len += 1;
        }
        self.groups.insert(group, ids);
    }

    pub(super) fn get(&self, (group, id): Key) -> Option<&V> {
        self.groups.get(group)?.get(id)
    }

    pub(super) fn remove(&mut self, (group, id): Key) -> Option<V> {
        let mut ids = self.groups.get(group)?.clone();
        let old = ids.remove(id)?;
        self.len -= 1;
        if ids.is_empty() {
            self.groups.remove(group);
        } else {
            self.groups.insert(group, ids);
        }
        Some(old)
    }

    pub(super) fn pop(&mut self) -> Option<(Key, V)> {
        let (key, _) = self.next(None)?;
        self.remove(key).map(|v| (key, v))
    }
}

#[derive(Debug)]
pub(super) enum Callback {
    Owned(ArcSwapOption<Waker>),
    First(crate::retirement_grants::Request),
}

impl Callback {
    pub(super) fn new(waker: Waker) -> Arc<Self> {
        Arc::new(Self::Owned(ArcSwapOption::from(Some(Arc::new(waker)))))
    }
    pub(super) fn first(request: crate::retirement_grants::Request) -> Arc<Self> {
        Arc::new(Self::First(request))
    }
    pub(super) fn take(&self, recovery: &super::Recovery) -> Option<Arc<Waker>> {
        match self {
            Self::Owned(callback) => callback.swap(None),
            Self::First(request) => request.arm()?.close(recovery).0.map(Arc::new),
        }
    }
}

#[derive(Debug)]
pub(super) struct Entry {
    pub(super) grant: Option<GrantWatch>,
    pub(super) bytes: Option<usize>,
    pub(super) callback: Arc<Callback>,
    pub(super) space_seen: Option<Arc<AtomicBool>>,
}

impl From<Waiter> for Entry {
    fn from(waiter: Waiter) -> Self {
        Self {
            grant: waiter.grant,
            bytes: waiter.bytes,
            callback: Callback::new(waiter.waker),
            space_seen: waiter.space_seen,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Ready {
    pub(super) entry: Arc<Entry>,
    pub(super) offered: bool,
}

impl Ready {
    pub(super) fn observe(&self) {
        if self.offered
            && let Some(seen) = &self.entry.space_seen
        {
            seen.store(true, Ordering::Release);
        }
    }
    pub(super) fn take(self, recovery: &super::Recovery) -> Option<Arc<Waker>> {
        self.observe();
        self.entry.callback.take(recovery)
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct Waiters {
    pub(super) first_count: usize,
    pub(super) recheck: bool,
    pub(super) exit: u64,
    pub(super) notified: u64,
    pub(super) grants: u64,
    pub(super) entries: Pairs<Arc<Entry>>,
    pub(super) ready: Pairs<Ready>,
    pub(super) space: Pairs<u64>,
    pub(super) physical: Index<(u64, Layout)>,
    pub(super) physical_cursor: WaiterId,
}

impl Waiters {
    pub(super) fn queue_exit(&mut self, exit: u64) {
        self.exit = exit;
        while let Some((key, entry)) = self.entries.pop() {
            self.ready.insert(
                key,
                Ready {
                    entry,
                    offered: false,
                },
            );
        }
        self.space = Pairs::default();
        self.physical = Index::default();
    }

    pub(super) fn count(&self) -> usize {
        self.entries.len + self.ready.len
    }
    pub(super) fn wake_pending(&self) -> bool {
        !self.ready.is_empty()
    }
    pub(super) fn fits(&self, bytes: usize) -> bool {
        self.space
            .next(None)
            .is_some_and(|((min, _), _)| min <= bytes as u64)
    }

    pub(super) fn take_fitting(&mut self, available: usize) -> Option<(Key, Arc<Entry>, usize)> {
        let ((size, id), &ticket) = self.space.next(None)?;
        if size > available as u64 {
            return None;
        }
        self.space.remove((size, id));
        let waiter = self.entries.remove((ticket, id))?;
        self.physical.remove(id);
        Some(((ticket, id), waiter, usize::try_from(size).ok()?))
    }

    pub(super) fn queue_grants(&mut self, generation: u64) {
        if self.grants == generation && generation != u64::MAX {
            return;
        }
        self.grants = generation;
        let snapshot = self.entries.clone();
        let mut last = None;
        while let Some((key, entry)) = snapshot.next(last) {
            if entry.grant.as_ref().is_some_and(GrantWatch::ready) {
                self.entries.remove(key);
                self.physical.remove(key.1);
                if let Some(bytes) = entry.bytes {
                    self.space.remove((bytes as u64, key.1));
                }
                self.ready.insert(
                    key,
                    Ready {
                        entry: entry.clone(),
                        offered: false,
                    },
                );
            }
            last = Some(key);
        }
    }
}

#[derive(Clone)]
pub(super) struct PhysicalCandidate {
    pub(super) id: WaiterId,
    pub(super) ticket: u64,
    pub(super) layout: Layout,
    entry: Arc<Entry>,
}

impl Waiters {
    pub(super) fn select_physical(&mut self) -> [Option<PhysicalCandidate>; 32] {
        let mut candidates = std::array::from_fn(|_| None);
        let mut cursor = Some(self.physical_cursor);
        for slot in candidates.iter_mut().take(self.physical.len()) {
            let Some((id, &(ticket, layout))) = self
                .physical
                .next(cursor)
                .or_else(|| self.physical.next(None))
            else {
                break;
            };
            if let Some(entry) = self.entries.get((ticket, id)) {
                *slot = Some(PhysicalCandidate {
                    id,
                    ticket,
                    layout,
                    entry: entry.clone(),
                });
            }
            cursor = Some(id);
            self.physical_cursor = id;
        }
        candidates
    }

    pub(super) fn promote_physical(&mut self, candidates: &[Option<PhysicalCandidate>]) -> u64 {
        let mut count = 0;
        for candidate in candidates.iter().flatten() {
            let key = (candidate.ticket, candidate.id);
            if !self
                .entries
                .get(key)
                .is_some_and(|live| Arc::ptr_eq(live, &candidate.entry))
            {
                continue;
            }
            self.physical.remove(candidate.id);
            if let Some(entry) = self.entries.remove(key) {
                self.ready.insert(
                    key,
                    Ready {
                        entry,
                        offered: false,
                    },
                );
                count += 1;
            }
        }
        count
    }

    // A committed Ready owns a durable offer. Publish its receipt before
    // unlinking, including when a concurrent helper wins the root CAS.
    pub(super) fn pop_ready(&mut self) -> Option<Ready> {
        let (key, ready) = self.ready.next(None)?;
        ready.observe();
        if let Callback::First(request) = &*ready.entry.callback
            && let Some(arm) = request.arm()
        {
            // This committed ready record can never need migration again, even
            // if the thread stops before taking the actual callback.
            arm.indexed(request);
            self.first_count -= 1;
        }
        self.ready.remove(key)
    }
}
