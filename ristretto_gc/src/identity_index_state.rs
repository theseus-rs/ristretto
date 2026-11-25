//! Owning bucket lists. The parent supplies linearizable owning links.
use super::{Arc, AtomicBool, AtomicUsize, Link, Ordering, OwnedLink, take_owned};
use std::fmt;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub(super) enum Phase {
    #[default]
    Live,
    Deleted,
    Detached,
}

/// An owning successor snapshot; no separate link-state allocation is needed.
#[derive(Clone, Default)]
pub(super) struct Edge {
    pub(super) next: Option<Arc<Node>>,
    pub(super) phase: Phase,
}
impl Edge {
    fn live(next: Option<Arc<Node>>) -> Self {
        Self {
            next,
            phase: Phase::Live,
        }
    }
    fn detached() -> Self {
        Self {
            next: None,
            phase: Phase::Detached,
        }
    }
    pub(super) fn same(&self, other: &Self) -> bool {
        self.phase == other.phase
            && match (&self.next, &other.next) {
                (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

// The first eight immutable bytes are stable pointer identities for link states.
// The native RefCnt adapter uses offsets 2/4 inside this allocation, and recovers
// the Arc data pointer by subtracting that offset. Keep this layout explicit.
#[repr(C, align(8))]
pub(super) struct Node {
    markers: [u8; 8],
    key: u64,
    address: usize,
    next: Link,
    count: Arc<AtomicUsize>,
    linked: Arc<AtomicBool>,
}
const _: () = {
    assert!(align_of::<Node>() >= 8);
    assert!(std::mem::offset_of!(Node, markers) == 0);
};
impl Drop for Node {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::Relaxed);
        // Whole-index teardown and last-owner snapshot release cannot recurse
        // down an arbitrarily long bucket on either the native or Wasm stack.
        let mut next = self.next.swap(Edge::detached()).next;
        while let Some(node) = next {
            let Some(node) = take_owned(node) else { break };
            next = node.next.swap(Edge::detached()).next;
            drop(node);
        }
    }
}

/// An immutable bucket version. Even an empty bucket has a unique owning header;
/// a removed version is never installed again. Node addresses may return as the
/// head, but a delayed insertion can never reuse its earlier header CAS.
struct Head {
    next: Option<Arc<Node>>,
}

struct Attempt {
    expected: Arc<Head>,
    proposed: Arc<Head>,
}

/// A shared insertion intent. Preparation alone is not membership or admission.
/// Retargeting replaces the entire immutable attempt, including its private node.
/// No helper can mutate a successor that another helper has already published.
pub(crate) struct Prepared {
    key: u64,
    address: usize,
    count: Arc<AtomicUsize>,
    linked: Arc<AtomicBool>,
    attempt: OwnedLink<Attempt>,
}

#[derive(Default)]
struct Walk {
    initialized: bool,
    predecessor: Option<Arc<Node>>,
    edge: Edge,
    head: Option<Arc<Head>>,
}

#[derive(Default)]
pub(crate) struct Removal {
    target: Option<(u64, usize)>,
    walk: Walk,
}
impl fmt::Debug for Removal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Removal")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

#[derive(Default)]
pub(crate) struct Cleanup {
    owner: Option<Arc<Dirty>>,
    bucket: usize,
    word: usize,
    pending: usize,
    active: bool,
    walk: Walk,
}
impl Drop for Cleanup {
    fn drop(&mut self) {
        let Some(owner) = &self.owner else { return };
        // Return both unvisited bits and the current bucket after cancellation.
        if self.pending != 0
            && let Some(word) = owner.words.get(self.word - 1)
        {
            word.fetch_or(self.pending, Ordering::Release);
        }
        if self.active {
            owner.mark(self.bucket);
        }
    }
}
impl fmt::Debug for Cleanup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cleanup")
            .field("bucket", &self.bucket)
            .finish_non_exhaustive()
    }
}

enum Visit {
    Live(Arc<Node>, Edge),
    Advance,
    End,
}

pub(crate) struct Index {
    buckets: Box<[OwnedLink<Head>]>,
    dirty: Arc<Dirty>,
    count: Arc<AtomicUsize>,
}
struct Dirty {
    words: Box<[AtomicUsize]>,
}
impl Dirty {
    #[expect(
        clippy::indexing_slicing,
        reason = "bucket bitmap covers every table entry"
    )]
    fn mark(&self, bucket: usize) {
        self.words[bucket / usize::BITS as usize]
            .fetch_or(1 << (bucket % usize::BITS as usize), Ordering::Release);
    }
}
impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityIndex")
            .field("retained_nodes", &self.retained_nodes())
            .finish_non_exhaustive()
    }
}

#[expect(
    clippy::expect_used,
    reason = "bucket and proposal headers are never empty"
)]
impl Index {
    pub(crate) fn new(buckets: usize) -> Self {
        assert!(buckets.is_power_of_two());
        Self {
            buckets: (0..buckets)
                .map(|_| OwnedLink::new(Some(Arc::new(Head { next: None }))))
                .collect(),
            dirty: Arc::new(Dirty {
                words: (0..buckets.div_ceil(usize::BITS as usize))
                    .map(|_| AtomicUsize::new(0))
                    .collect(),
            }),
            count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn bucket_index(&self, key: u64) -> usize {
        // Mix both halves without truncating the identity on 32-bit Wasm.
        let bytes = key.to_le_bytes();
        let low = u16::from_le_bytes([bytes[0], bytes[1]]);
        let high = u16::from_le_bytes([bytes[4], bytes[5]]);
        usize::from(low ^ high) & (self.buckets.len() - 1)
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "bucket and bitmap indices are bounded by the nonempty table"
    )]
    fn bucket(&self, key: u64) -> &OwnedLink<Head> {
        &self.buckets[self.bucket_index(key)]
    }

    fn dirty(&self, key: u64) {
        self.dirty.mark(self.bucket_index(key));
    }

    pub(crate) fn prepare(&self, key: u64, address: usize) -> Prepared {
        let linked = Arc::new(AtomicBool::new(false));
        let attempt = self.proposal(
            key,
            address,
            &linked,
            self.bucket(key).load().expect("bucket header"),
        );
        Prepared {
            key,
            address,
            count: self.count.clone(),
            linked,
            attempt: OwnedLink::new(Some(attempt)),
        }
    }

    fn proposal(
        &self,
        key: u64,
        address: usize,
        linked: &Arc<AtomicBool>,
        expected: Arc<Head>,
    ) -> Arc<Attempt> {
        self.count.fetch_add(1, Ordering::Relaxed);
        let node = Arc::new(Node {
            markers: [0; 8],
            key,
            address,
            next: Link::new(Edge::live(expected.next.clone())),
            count: self.count.clone(),
            linked: linked.clone(),
        });
        Arc::new(Attempt {
            expected,
            proposed: Arc::new(Head { next: Some(node) }),
        })
    }

    /// The caller supplies a globally unique, never previously inserted key.
    /// This does not announce a phase obligation or replace collector admission.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "consume the insertion intent"
    )]
    pub(crate) fn insert(&self, prepared: Prepared) {
        while !self.help(&prepared) {}
    }

    /// One help attempt. False means retry; it is not a cancellation result.
    /// Retargeting can allocate a private proposal and does not bound the host
    /// allocator. The caller still supplies existing collector admission.
    pub(crate) fn help(&self, prepared: &Prepared) -> bool {
        self.help_observed(prepared, || {}, || {})
    }

    fn help_observed(
        &self,
        prepared: &Prepared,
        before_compare: impl FnOnce(),
        after_compare: impl FnOnce(),
    ) -> bool {
        assert!(Arc::ptr_eq(&self.count, &prepared.count));
        if prepared.linked.load(Ordering::Acquire) {
            return true;
        }
        let root = self.bucket(prepared.key);
        let attempt = prepared.attempt.load().expect("insertion attempt");
        let observed = root.load().expect("bucket header");
        if Arc::ptr_eq(&observed, &attempt.proposed) {
            prepared.linked.store(true, Ordering::Release);
            return true;
        }
        if !Arc::ptr_eq(&observed, &attempt.expected) {
            // Every modifier certifies the previous head before its release CAS.
            // A successful insertion may have been removed since our first flag
            // read. Recheck after observing the changed header before retargeting.
            if prepared.linked.load(Ordering::Acquire) {
                return true;
            }
            let next = self.proposal(prepared.key, prepared.address, &prepared.linked, observed);
            prepared.attempt.compare(&attempt, next);
            return false;
        }
        before_compare();
        if !Self::change_head(root, &attempt.expected, attempt.proposed.clone()) {
            return false;
        }
        after_compare();
        prepared.linked.store(true, Ordering::Release);
        true
    }

    #[cfg(test)]
    pub(crate) fn test_help(
        &self,
        prepared: &Prepared,
        before_compare: impl FnOnce(),
        after_compare: impl FnOnce(),
    ) -> bool {
        self.help_observed(prepared, before_compare, after_compare)
    }

    #[cfg(test)]
    pub(crate) fn test_insert(&self, prepared: Prepared, before_compare: impl FnOnce()) {
        self.insert_observed(prepared, before_compare);
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "consume the exclusive insertion capability"
    )]
    #[cfg(test)]
    fn insert_observed(&self, prepared: Prepared, before_compare: impl FnOnce()) {
        let mut before_compare = Some(before_compare);
        while !self.help_observed(
            &prepared,
            || {
                if let Some(before_compare) = before_compare.take() {
                    before_compare();
                }
            },
            || {},
        ) {}
    }

    fn change_head(root: &OwnedLink<Head>, expected: &Arc<Head>, proposed: Arc<Head>) -> bool {
        if let Some(node) = &expected.next {
            node.linked.store(true, Ordering::Release);
        }
        root.compare(expected, proposed)
    }

    fn restart(root: &OwnedLink<Head>, walk: &mut Walk) {
        walk.predecessor = None;
        let head = root.load().expect("bucket header");
        walk.edge = Edge::live(head.next.clone());
        walk.head = Some(head);
        walk.initialized = true;
    }

    fn unlink(root: &OwnedLink<Head>, walk: &mut Walk, node: &Arc<Node>, deleted: &Edge) {
        // A predecessor's next link only advances, then becomes deleted/detached.
        // The expected owning successor prevents pointer reuse. Deletion and its
        // frozen successor share one CAS; a detached predecessor cannot unlink
        // another still-reachable node through an earlier snapshot.
        let replacement = Edge::live(deleted.next.clone());
        let changed = if let Some(predecessor) = &walk.predecessor {
            predecessor.next.compare(&walk.edge, replacement.clone())
        } else {
            let head = Arc::new(Head {
                next: deleted.next.clone(),
            });
            if Self::change_head(root, walk.head.as_ref().expect("walk header"), head.clone()) {
                walk.head = Some(head);
                true
            } else {
                false
            }
        };
        if changed {
            node.next.compare(deleted, Edge::detached());
            walk.edge = replacement;
        } else {
            Self::restart(root, walk);
        }
    }

    /// At most one list node and one physical-unlink attempt per visit. Holding
    /// a cursor owns its nodes; it neither pins an epoch nor excludes a writer.
    fn visit(root: &OwnedLink<Head>, walk: &mut Walk) -> Visit {
        if !walk.initialized {
            Self::restart(root, walk);
        }
        let Some(node) = walk.edge.next.clone() else {
            return Visit::End;
        };
        let next = node.next.load();
        if next.phase == Phase::Detached {
            Self::restart(root, walk);
            return Visit::Advance;
        }
        if next.phase == Phase::Deleted {
            Self::unlink(root, walk, &node, &next);
            Visit::Advance
        } else {
            Visit::Live(node, next)
        }
    }

    fn advance(walk: &mut Walk, node: Arc<Node>, next: Edge) {
        walk.predecessor = Some(node);
        walk.edge = next;
    }

    pub(crate) fn get(&self, key: u64) -> Option<usize> {
        let root = self.bucket(key);
        let mut walk = Walk::default();
        loop {
            match Self::visit(root, &mut walk) {
                Visit::Live(node, _next) if node.key == key => return Some(node.address),
                Visit::Live(node, next) => Self::advance(&mut walk, node, next),
                Visit::Advance => {}
                Visit::End => return None,
            }
        }
    }

    /// None means yield and retain this cursor; Some certifies the exact key and
    /// address was retired or absent. No logical removal precedes this final unit.
    pub(crate) fn remove_step(
        &self,
        key: u64,
        address: usize,
        cursor: &mut Removal,
    ) -> Option<bool> {
        self.remove_observed(key, address, cursor, || {})
    }

    #[cfg(test)]
    pub(crate) fn test_remove_step(
        &self,
        key: u64,
        address: usize,
        cursor: &mut Removal,
        after_mark: impl FnOnce(),
    ) -> Option<bool> {
        self.remove_observed(key, address, cursor, after_mark)
    }

    fn remove_observed(
        &self,
        key: u64,
        address: usize,
        cursor: &mut Removal,
        after_mark: impl FnOnce(),
    ) -> Option<bool> {
        if cursor.target != Some((key, address)) {
            *cursor = Removal {
                target: Some((key, address)),
                walk: Walk::default(),
            };
        }
        let root = self.bucket(key);
        match Self::visit(root, &mut cursor.walk) {
            Visit::Live(node, next) if node.key == key && node.address == address => {
                let deleted = Edge {
                    next: next.next.clone(),
                    phase: Phase::Deleted,
                };
                if !node.next.compare(&next, deleted.clone()) {
                    Self::restart(root, &mut cursor.walk);
                    return None;
                }
                self.dirty(key);
                after_mark();
                Self::unlink(root, &mut cursor.walk, &node, &deleted);
                *cursor = Removal::default();
                Some(true)
            }
            Visit::Live(node, next) => {
                Self::advance(&mut cursor.walk, node, next);
                None
            }
            Visit::Advance => None,
            Visit::End => {
                *cursor = Removal::default();
                Some(false)
            }
        }
    }

    /// One node or at most 64 empty bitmap words per collector work unit.
    /// Taking dirty bits before traversal preserves subsequent notifications.
    /// Concurrent insertions ahead of this cursor are covered by the next pass
    /// and their publication barriers; this does not certify graph closure.
    pub(crate) fn cleanup_step(&self, cursor: &mut Cleanup) -> bool {
        if let Some(owner) = &cursor.owner {
            assert!(Arc::ptr_eq(owner, &self.dirty));
        } else {
            cursor.owner = Some(self.dirty.clone());
        }
        for _ in 0..64 {
            if !cursor.active {
                if cursor.pending == 0 {
                    let Some(word) = self.dirty.words.get(cursor.word) else {
                        return true;
                    };
                    cursor.pending = word.swap(0, Ordering::AcqRel);
                    cursor.word += 1;
                    if cursor.pending == 0 {
                        continue;
                    }
                }
                let bit = cursor.pending.trailing_zeros() as usize;
                cursor.pending &= cursor.pending - 1;
                cursor.bucket = (cursor.word - 1) * usize::BITS as usize + bit;
                cursor.active = true;
            }
            let Some(root) = self.buckets.get(cursor.bucket) else {
                return true;
            };
            match Self::visit(root, &mut cursor.walk) {
                Visit::Live(node, next) => Self::advance(&mut cursor.walk, node, next),
                Visit::Advance => {}
                Visit::End => {
                    cursor.active = false;
                    cursor.walk = Walk::default();
                }
            }
            return false;
        }
        false
    }

    #[cfg(test)]
    pub(super) fn test_edge(&self, key: u64) -> Edge {
        Edge::live(self.bucket(key).load().expect("bucket header").next.clone())
    }

    #[cfg(test)]
    pub(crate) fn test_dirty_buckets(&self) -> u32 {
        self.dirty
            .words
            .iter()
            .map(|word| word.load(Ordering::Acquire).count_ones())
            .sum()
    }

    #[cfg(test)]
    pub(crate) fn test_snapshot(&self, key: u64) -> Removal {
        let mut cursor = Removal::default();
        Self::restart(self.bucket(key), &mut cursor.walk);
        cursor
    }

    /// Fault injection only: simulate stale identity hints without relying on
    /// allocator address reuse. Tests invoke this without outstanding operations.
    #[cfg(test)]
    pub(crate) fn test_set(&self, key: u64, address: Option<usize>) {
        if let Some(old) = self.get(key) {
            let mut removal = Removal::default();
            while self.remove_step(key, old, &mut removal).is_none() {}
        }
        if let Some(address) = address {
            self.insert(self.prepare(key, address));
        }
    }

    /// Shutdown only; no concurrent publishers may still use the collector.
    pub(crate) fn clear(&self) {
        for bucket in &self.buckets {
            let head = bucket.load().expect("bucket header");
            if let Some(node) = &head.next {
                node.linked.store(true, Ordering::Release);
            }
            bucket.swap(Some(Arc::new(Head { next: None })));
        }
    }

    pub(crate) fn retained_nodes(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}
