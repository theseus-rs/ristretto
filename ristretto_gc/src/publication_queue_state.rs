//! Linked FIFO protocol; the parent supplies owning atomic links and synchronization.
//! A linked node is consumable before its producer advances the advisory tail.
use super::{Arc, AtomicBool, AtomicUsize, Link, Mutex, Ordering, same, take_owned};
use std::fmt;

struct Counts {
    nodes: AtomicUsize,
    reserved: AtomicUsize,
}

struct Node<T> {
    next: Link<Node<T>>,
    // Only the successful head-CAS owner accesses this mutex. There is no second
    // contender; the mutex supplies safe interior mutability, not queue exclusion.
    value: Mutex<Option<T>>,
    // Irreversible evidence of insertion, including after the predecessor link
    // is tombstoned. A consumer publishes this before detaching that link.
    linked: AtomicBool,
    retired: bool,
    counts: Arc<Counts>,
}

impl<T> Node<T> {
    fn new(value: Option<T>, retired: bool, counts: &Arc<Counts>) -> Arc<Self> {
        counts.nodes.fetch_add(1, Ordering::Relaxed);
        Arc::new(Self {
            next: Link::new(None),
            value: Mutex::new(value),
            linked: AtomicBool::new(false),
            retired,
            counts: counts.clone(),
        })
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        self.counts.nodes.fetch_sub(1, Ordering::Relaxed);
        // Queue destruction or an abandoned reader must not recursively destroy
        // an arbitrarily long chain on the native or Wasm stack.
        let mut next = self.next.swap(None);
        while let Some(node) = next {
            if let Some(node) = take_owned(node) {
                next = node.next.swap(None);
                drop(node);
            } else {
                break;
            }
        }
    }
}

/// An owning append intent. Sharing this record permits idempotent helping;
/// preparing it alone does not publish its payload or authorize phase closure.
/// It retains no queue owner or history of completed operations. Last-owner drop
/// cancels an unlinked intent, including after an unsuccessful help attempt.
pub(crate) struct Prepared<T> {
    node: Arc<Node<T>>,
    predecessor: Link<Node<T>>,
}

impl<T> Prepared<T> {
    fn linked(&self) -> bool {
        if self.node.linked.load(Ordering::Acquire) {
            return true;
        }
        if let Some(predecessor) = self.predecessor.load() {
            let next = predecessor.next.load();
            if same(next.as_ref(), Some(&self.node)) {
                self.node.linked.store(true, Ordering::Release);
                return true;
            }
        }
        // A consumer can replace the link with a tombstone between our first
        // flag read and link read. Its earlier Release store must now be visible.
        self.node.linked.load(Ordering::Acquire)
    }
}

impl<T> Drop for Prepared<T> {
    fn drop(&mut self) {
        // No helper can still borrow this record when its last owner drops it.
        // A linked intent is instead charged until the successful consumer pops.
        if !self.linked() {
            self.node.counts.reserved.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

pub(crate) struct Queue<T> {
    head: Link<Node<T>>,
    tail: Link<Node<T>>,
    retired: Arc<Node<T>>,
    counts: Arc<Counts>,
}

impl<T> fmt::Debug for Queue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Queue")
            .field("reserved", &self.pending())
            .field("retained_nodes", &self.retained_nodes())
            .finish_non_exhaustive()
    }
}

#[expect(
    clippy::expect_used,
    reason = "head and tail are initialized once and never reset"
)]
impl<T> Queue<T> {
    pub(crate) fn new() -> Self {
        let counts = Arc::new(Counts {
            nodes: AtomicUsize::new(0),
            reserved: AtomicUsize::new(0),
        });
        let head = Node::new(None, false, &counts);
        let retired = Node::new(None, true, &counts);
        Self {
            head: Link::new(Some(head.clone())),
            tail: Link::new(Some(head)),
            retired,
            counts,
        }
    }

    pub(crate) fn prepare(&self, value: T) -> Prepared<T> {
        let prepared = Prepared {
            node: Node::new(Some(value), false, &self.counts),
            // Choose the first candidate privately, before sharing the record.
            // A stale tail is safe: its non-null link forces a helped retry.
            predecessor: Link::new(self.tail.load()),
        };
        self.counts.reserved.fetch_add(1, Ordering::Release);
        prepared
    }

    /// One bounded protocol attempt, with no node allocation or payload lock. False
    /// means retry, not failure; a caller may yield before its next attempt.
    /// True certifies insertion even if another helper/consumer already finished.
    pub(crate) fn help(&self, prepared: &Prepared<T>) -> bool {
        self.help_observed(prepared, || {}, || {})
    }

    fn help_observed(
        &self,
        prepared: &Prepared<T>,
        before_link: impl FnOnce(),
        after_link: impl FnOnce(),
    ) -> bool {
        assert!(
            Arc::ptr_eq(&self.counts, &prepared.node.counts),
            "append record belongs to another queue"
        );
        if prepared.node.linked.load(Ordering::Acquire) {
            return true;
        }
        let predecessor = prepared.predecessor.load().expect("append predecessor");
        let next = predecessor.next.load();
        if same(next.as_ref(), Some(&prepared.node)) {
            prepared.node.linked.store(true, Ordering::Release);
            self.tail
                .compare(Some(&predecessor), Some(prepared.node.clone()));
            return true;
        }
        if let Some(next) = next {
            // This predecessor can never accept our node: links change only
            // from null to a node, then to the non-null tombstone. A delayed
            // helper holding the old predecessor therefore cannot insert there.
            if prepared.node.linked.load(Ordering::Acquire) {
                return true;
            }
            if !next.retired {
                self.tail.compare(Some(&predecessor), Some(next));
            }
            prepared
                .predecessor
                .compare(Some(&predecessor), self.tail.load());
            return false;
        }
        before_link();
        if !predecessor.next.compare(None, Some(prepared.node.clone())) {
            return false;
        }
        // The shared predecessor fixes this record's single insertion location.
        // Another helper can certify this link without waiting for either store.
        after_link();
        prepared.node.linked.store(true, Ordering::Release);
        self.tail
            .compare(Some(&predecessor), Some(prepared.node.clone()));
        true
    }

    // Keep the contention-only append protocol out of ordinary marking's code.
    #[cold]
    #[inline(never)]
    pub(crate) fn push(&self, value: T) {
        let prepared = self.prepare(value);
        while !self.help(&prepared) {}
    }

    #[cfg(test)]
    fn push_observed(&self, value: T, before: impl FnOnce(), after: impl FnOnce()) {
        let prepared = self.prepare(value);
        self.test_help(&prepared, before, after);
    }

    pub(crate) fn pop(&self) -> Option<T> {
        self.pop_after_head(|| {})
    }

    #[expect(
        clippy::unwrap_in_result,
        reason = "head is initialized once and never reset"
    )]
    fn pop_after_head(&self, observed: impl FnOnce()) -> Option<T> {
        let mut observed = Some(observed);
        loop {
            let head = self.head.load().expect("queue head");
            if let Some(observed) = observed.take() {
                observed();
            }
            let tail = self.tail.load();
            let next = head.next.load();
            if !same(self.head.load().as_ref(), Some(&head)) {
                continue;
            }
            let next = next?;
            if next.retired {
                continue;
            }
            if same(tail.as_ref(), Some(&head)) {
                self.tail.compare(tail.as_ref(), Some(next));
                continue;
            }
            if self.head.compare(Some(&head), Some(next.clone())) {
                next.linked.store(true, Ordering::Release);
                // Keep a non-null tombstone, never reset this link to empty: a
                // delayed producer must not append to an already detached head.
                head.next.swap(Some(self.retired.clone()));
                self.counts.reserved.fetch_sub(1, Ordering::AcqRel);
                return next
                    .value
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .take();
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        loop {
            let head = self.head.load().expect("queue head");
            let next = head.next.load();
            if same(self.head.load().as_ref(), Some(&head)) {
                return next.is_none();
            }
        }
    }

    /// Includes private prepared records. This scheduling/diagnostic count can
    /// skip an empty queue when zero, but a nonzero count does not prove linked
    /// work exists. Termination still needs publication/claim protection and a
    /// validated link inspection; private preparation alone must not exclude it.
    pub(crate) fn pending(&self) -> usize {
        self.counts.reserved.load(Ordering::Acquire)
    }
    /// Includes the empty head, shared tombstone, and nodes held by delayed readers.
    pub(crate) fn retained_nodes(&self) -> usize {
        self.counts.nodes.load(Ordering::Acquire)
    }

    #[cfg(test)]
    pub(crate) fn test_help(
        &self,
        prepared: &Prepared<T>,
        before: impl FnOnce(),
        after: impl FnOnce(),
    ) {
        let mut before = Some(before);
        let mut after = Some(after);
        while !self.help_observed(
            prepared,
            || {
                if let Some(before) = before.take() {
                    before();
                }
            },
            || {
                if let Some(after) = after.take() {
                    after();
                }
            },
        ) {}
    }

    #[cfg(test)]
    pub(crate) fn test_push_after_link(&self, value: T, linked: impl FnOnce()) {
        self.push_observed(value, || {}, linked);
    }
    #[cfg(test)]
    pub(crate) fn test_push_before_link(&self, value: T, prepared: impl FnOnce()) {
        self.push_observed(value, prepared, || {});
    }
    #[cfg(test)]
    pub(crate) fn test_pop_after_head(&self, observed: impl FnOnce()) -> Option<T> {
        self.pop_after_head(observed)
    }
}
