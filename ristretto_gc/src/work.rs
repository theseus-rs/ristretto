//! Persistent tracing queues. A claim must be protected until its in-flight reservation.
use crate::metadata::MetadataRef;
use arc_swap::ArcSwapOption;
use once_cell::race::OnceBox;
use std::collections::{LinkedList, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

const LOCAL_LIMIT: usize = 256;
// Bound contiguous injector growth to 4 KiB on either pointer width.
const FRESH_SEGMENT_LIMIT: usize = 4096 / size_of::<MetadataRef>();

#[derive(Debug)]
pub(crate) struct MarkItem {
    // Keep the exact allocation identity even if its descriptor address is reused.
    // This Arc owns metadata only; callbacks must acquire a storage lease.
    pub(crate) metadata: MetadataRef,
    pub(crate) offset: usize,
    pub(crate) version: Option<usize>,
    pub(crate) has_young: bool,
    pub(crate) dirty_scan: bool,
}

impl MarkItem {
    pub(crate) fn fresh(metadata: MetadataRef) -> Self {
        Self {
            metadata,
            offset: 0,
            version: None,
            has_young: false,
            dirty_scan: false,
        }
    }

    pub(crate) fn dirty(metadata: MetadataRef) -> Self {
        Self {
            dirty_scan: true,
            ..Self::fresh(metadata)
        }
    }

    fn compact_fresh(&self) -> bool {
        self.version.is_none() && !self.dirty_scan
    }
}

#[derive(Debug, Default)]
struct Queue {
    fresh: VecDeque<MetadataRef>,
    // Seal whole buffers so growing the injector never copies an unbounded
    // backlog under its queue lock. Reuse one emptied buffer.
    #[expect(
        clippy::linkedlist,
        reason = "appending a sealed buffer must never move earlier buffers under the queue lock"
    )]
    full: LinkedList<VecDeque<MetadataRef>>,
    buffered: usize,
    spare: VecDeque<MetadataRef>,
    current: VecDeque<MarkItem>,
}

impl Queue {
    fn len(&self) -> usize {
        self.fresh.len() + self.buffered + self.current.len()
    }

    fn capacity(&self) -> usize {
        self.current.capacity()
            + self.fresh.capacity()
            + self.spare.capacity()
            + self.full.iter().map(VecDeque::capacity).sum::<usize>()
    }

    fn push_fresh(&mut self, metadata: MetadataRef) {
        if self.fresh.len() == FRESH_SEGMENT_LIMIT {
            let buffer = std::mem::replace(&mut self.fresh, std::mem::take(&mut self.spare));
            self.buffered += buffer.len();
            self.full.push_back(buffer);
        }
        self.fresh.push_back(metadata);
    }

    fn pop_buffer(&mut self, steal: bool) -> Option<MetadataRef> {
        let buffer = if steal {
            self.full.back_mut()?
        } else {
            self.full.front_mut()?
        };
        let value = if steal {
            buffer.pop_back()?
        } else {
            buffer.pop_front()?
        };
        self.buffered -= 1;
        if buffer.is_empty() {
            let empty = if steal {
                self.full.pop_back()
            } else {
                self.full.pop_front()
            };
            if let Some(empty) = empty
                && self.spare.capacity() == 0
            {
                self.spare = empty;
            }
        }
        Some(value)
    }

    fn pop(&mut self, steal: bool) -> Option<MarkItem> {
        if steal {
            self.fresh
                .pop_back()
                .or_else(|| self.pop_buffer(true))
                .map(MarkItem::fresh)
                .or_else(|| self.current.pop_back())
        } else {
            self.current.pop_front().or_else(|| {
                let fresh = if self.buffered == 0 {
                    self.fresh.pop_front()
                } else {
                    self.pop_buffer(false)
                };
                fresh.map(MarkItem::fresh)
            })
        }
    }
}

#[derive(Debug, Default)]
#[repr(align(64))]
struct Local {
    queue: Mutex<Queue>,
    victim: AtomicUsize,
    // Updated under the queue lock. Zero can bypass that lock only when the
    // caller also excludes publishers and unreserved claims at termination.
    queued: AtomicUsize,
}

/// The first attempt carries only the exact descriptor. After a retry, each
/// ticket owns a fresh cancellation flag shared by the participating domains.
#[derive(Clone, Debug)]
pub(crate) struct GraphBirth(Attempt);

#[derive(Clone, Debug)]
enum Attempt {
    First(MetadataRef),
    Retry(Arc<RetriedBirth>),
}

#[derive(Debug)]
struct RetriedBirth {
    metadata: MetadataRef,
    cancelled: AtomicBool,
}

impl GraphBirth {
    pub(crate) fn new(metadata: &MetadataRef) -> Self {
        if metadata.birth.first_attempt() {
            Self(Attempt::First(metadata.clone()))
        } else {
            Self(Attempt::Retry(Arc::new(RetriedBirth {
                metadata: metadata.clone(),
                cancelled: AtomicBool::new(false),
            })))
        }
    }

    pub(crate) fn cancel(&self) {
        if let Attempt::Retry(record) = &self.0 {
            record.cancelled.store(true, Ordering::Release);
        }
        // First-attempt tickets become ineligible in the rearm CAS chain. Only
        // the publisher can activate the descriptor, and it must rearm first.
    }

    fn claim(self) -> Option<MarkItem> {
        match self.0 {
            Attempt::First(metadata) => metadata
                .birth
                .claim_first()
                .then(|| MarkItem::fresh(metadata)),
            Attempt::Retry(record) => (!record.cancelled.load(Ordering::Acquire)
                && record.metadata.birth.visible()
                && !record.cancelled.load(Ordering::Acquire))
            .then(|| MarkItem::fresh(record.metadata.clone())),
        }
    }
}

#[path = "graph_birth_queue.rs"]
mod graph_birth_queue;

/// Traversal debt belongs to the exact domain epoch containing this proposal.
/// The record remains committed even after the owning handle has been dropped.
#[derive(Clone, Debug)]
pub(crate) struct RootProposal {
    pub(crate) metadata: MetadataRef,
    pub(crate) record: Arc<crate::roots::Record>,
    pub(crate) index: usize,
    pub(crate) strength: crate::metadata::Retention,
    pub(crate) promotion_age: usize,
}

impl RootProposal {
    fn claim(self, epoch: &BirthEpoch) -> Option<MarkItem> {
        if !self.record.owes_trace() || !self.metadata.birth.is_live() {
            return None;
        }
        let young = self.metadata.age.load(Ordering::Acquire) < self.promotion_age;
        if self.index == 0 && !young {
            return None;
        }
        if epoch.old_only && young {
            // Old-only callbacks scan promoted objects via the handoff. A
            // black post-snapshot birth is never introduced into the old core.
            if !self.metadata.marked(self.index, epoch.number)
                || self.metadata.retention(self.index, epoch.number).is_some()
            {
                self.metadata.mark(self.index, epoch.number, self.strength);
            }
            return None;
        }
        self.metadata
            .mark(self.index, epoch.number, self.strength)
            .then(|| MarkItem::fresh(self.metadata))
    }
}

#[derive(Debug)]
pub(crate) struct BirthEpoch {
    pub(crate) number: u64,
    pub(crate) old_only: bool,
    direct_identity: OnceBox<Arc<()>>,
    roots: OnceBox<graph_birth_queue::Queue<RootProposal>>,
    inbox: OnceBox<graph_birth_queue::Queue<GraphBirth>>,
    strong_closed: AtomicBool,
}

impl BirthEpoch {
    // An obsolete descriptor witness retains only the small token's control
    // block. It can never retain the mark queue or its descriptor graph.
    pub(crate) fn direct_witness(&self) -> std::sync::Weak<()> {
        Arc::downgrade(self.direct_identity.get_or_init(|| Box::new(Arc::new(()))))
    }

    pub(crate) fn matches_witness(&self, witness: &std::sync::Weak<()>) -> bool {
        self.direct_identity
            .get()
            .is_some_and(|token| std::ptr::eq(Arc::as_ptr(token), witness.as_ptr()))
    }
}

#[derive(Debug)]
// Keep the frequently mutated queue header apart from adjacent domain flags.
#[cfg_attr(not(target_family = "wasm"), repr(align(64)))]
pub(crate) struct MarkQueues {
    // Queue zero receives external publication and overflow. At most sixteen
    // worker queues follow it; excess workers share a queue. Wasm uses only zero.
    queues: Box<[Local]>,
    fallback: crate::publication_queue::Queue<MarkItem>,
    fallback_turn: AtomicBool,
    // Partition contended births by descriptor shard. Publishers and consumers
    // can help FIFO links without holding an object or mark-queue lock. Each
    // cold caller prepares privately; one CAS installs the winning FIFO.
    birth_overflow: Box<[OnceBox<crate::publication_queue::Queue<MarkItem>>]>,
    birth_pending: AtomicUsize,
    graph_inbox: ArcSwapOption<BirthEpoch>,
    graph_offered: AtomicBool,
    root_offered: AtomicBool,
    root_turn: AtomicBool,
    graph_turn: AtomicBool,
    overflow_cursor: AtomicUsize,
}

#[derive(Default)]
pub(crate) struct Claim {
    pub(crate) items: [Option<(MarkItem, usize)>; 8],
    pub(crate) stolen: usize,
    pub(crate) misses: usize,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[expect(
    clippy::indexing_slicing,
    reason = "queue indices are reduced to the nonempty queue count"
)]
impl MarkQueues {
    pub(crate) fn new(workers: usize, object_shards: usize) -> Self {
        Self {
            queues: (0..=workers.min(16)).map(|_| Local::default()).collect(),
            fallback: crate::publication_queue::Queue::new(),
            fallback_turn: AtomicBool::new(false),
            birth_overflow: (0..crate::index::shard_count(object_shards))
                .map(|_| OnceBox::new())
                .collect(),
            birth_pending: AtomicUsize::new(0),
            graph_inbox: ArcSwapOption::empty(),
            graph_offered: AtomicBool::new(false),
            root_offered: AtomicBool::new(false),
            root_turn: AtomicBool::new(false),
            graph_turn: AtomicBool::new(false),
            overflow_cursor: AtomicUsize::new(0),
        }
    }

    /// The domain coordinator installs a fresh inbox before publishing strong
    /// tracing. Late producers may retain the old inbox across any later epoch.
    pub(crate) fn open_graph_inbox(&self, number: u64, old_only: bool) {
        self.graph_inbox.store(Some(Arc::new(BirthEpoch {
            number,
            old_only,
            direct_identity: OnceBox::new(),
            roots: OnceBox::new(),
            inbox: OnceBox::new(),
            strong_closed: AtomicBool::new(false),
        })));
    }

    pub(crate) fn close_graph_inbox(&self) {
        self.root_offered.store(false, Ordering::Release);
        self.graph_offered.store(false, Ordering::Release);
        self.graph_inbox.store(None);
    }

    pub(crate) fn finish_strong_births(&self) {
        if let Some(epoch) = self.graph_inbox.load().as_ref() {
            epoch.strong_closed.store(true, Ordering::Release);
        }
        self.graph_offered.store(false, Ordering::Release);
    }

    pub(crate) fn birth_epoch(&self) -> Option<Arc<BirthEpoch>> {
        self.graph_inbox.load_full()
    }

    pub(crate) fn current_birth_epoch(&self, expected: &Arc<BirthEpoch>) -> bool {
        self.graph_inbox
            .load()
            .as_ref()
            .is_some_and(|actual| Arc::ptr_eq(actual, expected))
    }

    pub(crate) fn propose_graph(&self, epoch: &BirthEpoch, graph: &GraphBirth) {
        let queue = epoch
            .inbox
            .get_or_init(|| Box::new(graph_birth_queue::Queue::new()));
        self.graph_offered.store(true, Ordering::Release);
        queue.push(graph.clone());
    }

    fn with_graph_inbox<T>(
        &self,
        f: impl FnOnce(&graph_birth_queue::Queue<GraphBirth>) -> T,
    ) -> Option<T> {
        if !self.graph_offered.load(Ordering::Acquire) {
            return None;
        }
        let current = self.graph_inbox.load();
        let epoch = current.as_ref()?;
        if epoch.strong_closed.load(Ordering::Acquire) {
            return None;
        }
        epoch.inbox.get().map(f)
    }

    pub(crate) fn propose_root(&self, epoch: &BirthEpoch, proposal: RootProposal) {
        let queue = epoch
            .roots
            .get_or_init(|| Box::new(graph_birth_queue::Queue::new()));
        self.root_offered.store(true, Ordering::Release);
        queue.push(proposal);
    }

    fn with_roots<R>(
        &self,
        visit: impl FnOnce(&BirthEpoch, &graph_birth_queue::Queue<RootProposal>) -> R,
    ) -> Option<R> {
        if !self.root_offered.load(Ordering::Acquire) {
            return None;
        }
        let view = self.graph_inbox.load();
        let epoch = view.as_ref()?;
        epoch.roots.get().map(|queue| visit(epoch, queue))
    }

    fn claim_roots(&self, limit: usize, remaining: &mut usize, claim: &mut Claim) -> bool {
        self.with_roots(|epoch, queue| {
            Self::claim_proposals(queue, limit, remaining, claim, |item| item.claim(epoch))
        })
        .unwrap_or(false)
    }

    fn claim_graph(&self, limit: usize, remaining: &mut usize, claim: &mut Claim) -> bool {
        self.with_graph_inbox(|queue| {
            Self::claim_proposals(queue, limit, remaining, claim, GraphBirth::claim)
        })
        .unwrap_or(false)
    }

    fn claim_proposals<T>(
        queue: &graph_birth_queue::Queue<T>,
        limit: usize,
        remaining: &mut usize,
        claim: &mut Claim,
        claim_one: impl Fn(T) -> Option<MarkItem>,
    ) -> bool {
        if *remaining == 0 {
            return false;
        }
        let mut live = 0;
        for _ in 0..limit.clamp(1, 8) {
            if *remaining == 0 {
                break;
            }
            let Some(graph) = queue.pop() else { break };
            if let Some(item) = claim_one(graph) {
                let budget = (*remaining).min(32).min(item.metadata.trace.budget.max(1));
                *remaining -= budget;
                claim.items[live] = Some((item, budget));
                live += 1;
            } else {
                *remaining -= 1;
            }
        }
        live != 0
    }

    pub(crate) fn worker_queue(&self, worker: usize) -> usize {
        if self.queues.len() == 1 {
            0
        } else {
            1 + worker % (self.queues.len() - 1)
        }
    }

    pub(crate) fn push(&self, local: usize, item: MarkItem) {
        if item.compact_fresh() {
            self.push_fresh(local, item.metadata.ptr().0, item.metadata);
        } else if let Err(item) = self.try_push_any(local, item) {
            self.fallback.push(item);
        }
    }

    /// Phase admission spans the append. Multiple publishers may append to the
    /// same overflow FIFO, including while a consumer holds a predecessor.
    fn push_fresh(&self, local: usize, address: usize, metadata: MetadataRef) {
        if let Err(item) = self.try_push_any(local, MarkItem::fresh(metadata)) {
            let shard = crate::index::address_bucket(address) & (self.birth_overflow.len() - 1);
            let queue = self.birth_overflow[shard]
                .get_or_init(|| Box::new(crate::publication_queue::Queue::new()));
            self.birth_pending.fetch_add(1, Ordering::Release);
            queue.push(item);
        }
    }

    /// Select a queue before inspecting edges. The callback produces work
    /// only for fresh marks. A busy or full worker queue can spill the group
    /// into the injector without repeating each edge's admission checks.
    pub(crate) fn append_mature<'a>(
        &self,
        local: usize,
        limit: usize,
        edges: impl Iterator<Item = Option<&'a MetadataRef>>,
        mut mark: impl FnMut(&MetadataRef) -> Option<bool>,
    ) -> usize {
        let local = local % self.queues.len();
        // A group can use the same destinations as a scalar push. Select the
        // destination before marking any edge so a failed probe is retryable.
        let others = (1..self.queues.len())
            .map(|offset| (local + offset) % self.queues.len())
            .filter(|&index| index != 0);
        for source in std::iter::once(local)
            .chain((local != 0).then_some(0))
            .chain(others)
        {
            let Ok(mut queue) = self.queues[source].queue.try_lock() else {
                continue;
            };
            if source != 0 && queue.len().saturating_add(limit) > LOCAL_LIMIT {
                continue;
            }
            let mut handled = 0;
            for edge in edges {
                let Some(metadata) = edge else { break };
                let Some(fresh) = mark(metadata) else { break };
                if fresh {
                    queue.push_fresh(metadata.clone());
                }
                handled += 1;
            }
            self.queues[source]
                .queued
                .store(queue.len(), Ordering::Release);
            return handled;
        }
        0
    }

    fn try_push_any(&self, local: usize, item: MarkItem) -> Result<(), MarkItem> {
        let local = local % self.queues.len();
        let mut item = match self.try_push(local, item) {
            Ok(()) => return Ok(()),
            Err(item) => item,
        };
        if local != 0 {
            item = match self.try_push(0, item) {
                Ok(()) => return Ok(()),
                Err(item) => item,
            };
        }
        // Other worker queues are already valid publication destinations. Probe
        // each at most once before allocating a fallback node; never wait for a
        // holder or exceed a worker queue's bound. A full hint may conservatively
        // skip newly freed capacity, and try_push rechecks under the queue lock.
        for source in (1..self.queues.len())
            .map(|offset| (local + offset) % self.queues.len())
            .filter(|&source| source != 0)
        {
            if self.queues[source].queued.load(Ordering::Acquire) >= LOCAL_LIMIT {
                continue;
            }
            item = match self.try_push(source, item) {
                Ok(()) => return Ok(()),
                Err(item) => item,
            };
        }
        let item = if item.compact_fresh() {
            MarkItem::fresh(item.metadata)
        } else {
            item
        };
        Err(item)
    }

    fn try_push(&self, local: usize, item: MarkItem) -> Result<(), MarkItem> {
        let mut queue = match self.queues[local].queue.try_lock() {
            Ok(queue) => queue,
            Err(TryLockError::Poisoned(error)) => error.into_inner(),
            Err(TryLockError::WouldBlock) => return Err(item),
        };
        if local != 0 && queue.len() >= LOCAL_LIMIT {
            return Err(item);
        }
        if item.compact_fresh() {
            queue.push_fresh(item.metadata);
        } else {
            queue.current.push_back(item);
        }
        self.queues[local]
            .queued
            .store(queue.len(), Ordering::Release);
        Ok(())
    }

    fn claim_fallback(
        &self,
        local: usize,
        limit: usize,
        remaining: &mut usize,
        claim: &mut Claim,
    ) -> bool {
        if self.birth_pending.load(Ordering::Acquire) == 0 {
            return Self::claim_linked(&self.fallback, local, limit, remaining, claim) != 0;
        }
        // Source zero is ordinary overflow. Rotate across all sources so a
        // permanent stream of births cannot starve ordinary continuations.
        let sources = self.birth_overflow.len() + 1;
        let start = self.overflow_cursor.fetch_add(1, Ordering::Relaxed) % sources;
        for offset in 0..sources {
            let source = (start + offset) % sources;
            if source == 0 {
                if Self::claim_linked(&self.fallback, local, limit, remaining, claim) != 0 {
                    return true;
                }
            } else if let Some(queue) = self.birth_overflow[source - 1].get()
                && let count = Self::claim_linked(queue, local, limit, remaining, claim)
                && count != 0
            {
                self.birth_pending.fetch_sub(count, Ordering::AcqRel);
                return true;
            }
        }
        false
    }

    fn claim_linked(
        queue: &crate::publication_queue::Queue<MarkItem>,
        local: usize,
        limit: usize,
        remaining: &mut usize,
        claim: &mut Claim,
    ) -> usize {
        if queue.pending() == 0 {
            return 0;
        }
        let mut count = 0;
        for slot in claim.items.iter_mut().take(limit.clamp(1, 8)) {
            if *remaining == 0 {
                break;
            }
            let Some(item) = queue.pop() else {
                break;
            };
            let budget = (*remaining).min(32).min(item.metadata.trace.budget.max(1));
            *remaining -= budget;
            *slot = Some((item, budget));
            count += 1;
        }
        if local != 0 {
            claim.stolen = count;
        }
        count
    }

    /// Probe each queue at most once, claiming a bounded batch from the first
    /// available source. Contention is not evidence that the generation is empty.
    #[cfg(test)]
    pub(crate) fn claim(&self, local: usize, limit: usize, remaining: &mut usize) -> Claim {
        let mut claim = Claim::default();
        self.claim_into(local, limit, remaining, &mut claim);
        claim
    }

    pub(crate) fn claim_into(
        &self,
        local: usize,
        limit: usize,
        remaining: &mut usize,
        result: &mut Claim,
    ) {
        let local = local % self.queues.len();
        if self.root_offered.load(Ordering::Acquire)
            && !self.root_turn.fetch_xor(true, Ordering::Relaxed)
            && self.claim_roots(limit, remaining, result)
        {
            return;
        }
        if self.graph_offered.load(Ordering::Acquire)
            && !self.graph_turn.fetch_xor(true, Ordering::Relaxed)
            && self.claim_graph(limit, remaining, result)
        {
            return;
        }
        if (self.fallback.pending() != 0 || self.birth_pending.load(Ordering::Acquire) != 0)
            && !self.fallback_turn.fetch_xor(true, Ordering::Relaxed)
            && self.claim_fallback(local, limit, remaining, result)
        {
            return;
        }
        let start = self.queues[local].victim.load(Ordering::Relaxed) % self.queues.len();
        let sources = std::iter::once(local)
            .chain((local != 0).then_some(0))
            .chain(
                (0..self.queues.len())
                    .map(|offset| (start + offset) % self.queues.len())
                    .filter(|&index| index != local && index != 0),
            );
        for source in sources {
            // An empty hint can miss a concurrent publication, so it only skips
            // this claim attempt. Closure still uses its publication/claim and
            // pending/in-flight handshake. Avoid locking an idle producer queue.
            if self.queues[source].queued.load(Ordering::Acquire) == 0 {
                continue;
            }
            let mut queue = match self.queues[source].queue.try_lock() {
                Ok(queue) => queue,
                Err(TryLockError::Poisoned(error)) => error.into_inner(),
                Err(TryLockError::WouldBlock) => {
                    result.misses += 1;
                    continue;
                }
            };
            let steal = source != local;
            let mut count = 0;
            for slot in result.items.iter_mut().take(limit.clamp(1, 8)) {
                if *remaining == 0 {
                    break;
                }
                let Some(item) = queue.pop(steal) else {
                    break;
                };
                let budget = (*remaining).min(32).min(item.metadata.trace.budget.max(1));
                *remaining -= budget;
                *slot = Some((item, budget));
                count += 1;
            }
            if count != 0 {
                self.queues[source]
                    .queued
                    .store(queue.len(), Ordering::Release);
                if steal {
                    result.stolen = count;
                    self.queues[local]
                        .victim
                        .store(source + 1, Ordering::Relaxed);
                }
                return;
            }
        }
        if !self.claim_fallback(local, limit, remaining, result)
            && !self.claim_graph(limit, remaining, result)
        {
            self.claim_roots(limit, remaining, result);
        }
    }

    /// A termination decision additionally excludes publication and new claims,
    /// and checks both generations' reservations before inspecting every queue.
    /// Zero counts published under queue locks can bypass those locks, provided
    /// the caller also excludes publishers and unreserved claims. A busy queue
    /// with a nonzero count remains unknown; a terminal caller yields on None.
    pub(crate) fn try_is_empty(&self) -> Option<bool> {
        // Prepared fallback nodes are charged before their publication. A
        // private, unlinked intent is not tracing work; admission/claim protection
        // still excludes closure across the actual append. Inspect the links so
        // private preparation alone cannot strand a generation at termination.
        if self.with_roots(|_, queue| !queue.is_empty()) == Some(true)
            || self.with_graph_inbox(|queue| !queue.is_empty()) == Some(true)
            || self.fallback.pending() != 0 && !self.fallback.is_empty()
            || self.birth_pending.load(Ordering::Acquire) != 0
                && self
                    .birth_overflow
                    .iter()
                    .filter_map(OnceBox::get)
                    .any(|q| !q.is_empty())
        {
            return Some(false);
        }
        for local in &self.queues {
            if local.queued.load(Ordering::Acquire) == 0 {
                continue;
            }
            let queue = match local.queue.try_lock() {
                Ok(queue) => queue,
                Err(TryLockError::Poisoned(error)) => error.into_inner(),
                Err(TryLockError::WouldBlock) => return None,
            };
            if queue.len() != 0 {
                return Some(false);
            }
        }
        Some(true)
    }

    pub(crate) fn statistics(&self) -> (usize, usize) {
        // The epoch still owns its slot pages after strong tracing closes.
        // Detached epochs retained only by a producer are outside this sample.
        let graph_capacity = self
            .graph_inbox
            .load()
            .as_ref()
            .and_then(|epoch| epoch.inbox.get())
            .map_or(0, graph_birth_queue::Queue::capacity)
            + self.with_roots(|_, q| q.capacity()).unwrap_or_default();
        self.queues.iter().fold(
            (
                self.fallback.pending()
                    + self.with_roots(|_, q| q.pending()).unwrap_or_default()
                    + self.birth_pending.load(Ordering::Acquire)
                    + self
                        .with_graph_inbox(graph_birth_queue::Queue::pending)
                        .unwrap_or_default(),
                graph_capacity
                    + self.fallback.retained_nodes().saturating_sub(2)
                    + self
                        .birth_overflow
                        .iter()
                        .filter_map(OnceBox::get)
                        .map(|q| q.retained_nodes().saturating_sub(2))
                        .sum::<usize>(),
            ),
            |(queued, capacity), local| {
                let queue = lock(&local.queue);
                (queued + queue.len(), capacity + queue.capacity())
            },
        )
    }

    /// Advisory samples include reserved fallback publication; they are not a
    /// snapshot and do not authorize termination.
    pub(crate) fn queued_sample(&self) -> Option<usize> {
        self.queues.iter().try_fold(
            self.fallback
                .pending()
                .checked_add(self.birth_pending.load(Ordering::Acquire))?
                .checked_add(self.with_roots(|_, q| q.pending()).unwrap_or_default())?
                .checked_add(
                    self.with_graph_inbox(graph_birth_queue::Queue::pending)
                        .unwrap_or_default(),
                )?,
            |count, local| count.checked_add(local.queued.load(Ordering::Acquire)),
        )
    }

    pub(crate) fn trim_empty(&self) {
        for local in &self.queues {
            let mut queue = lock(&local.queue);
            if queue.len() == 0 {
                let retired = std::mem::take(&mut *queue);
                drop(queue);
                drop(retired);
            }
        }
    }

    pub(crate) fn clear(&self) {
        self.close_graph_inbox();
        for local in &self.queues {
            let mut queue = lock(&local.queue);
            let retired = std::mem::take(&mut *queue);
            local.queued.store(0, Ordering::Release);
            drop(queue);
            drop(retired);
        }
        while let Some(item) = self.fallback.pop() {
            drop(item);
        }
        for queue in self.birth_overflow.iter().filter_map(OnceBox::get) {
            while let Some(item) = queue.pop() {
                self.birth_pending.fetch_sub(1, Ordering::AcqRel);
                drop(item);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn with_held_injector(&self, action: impl FnOnce()) {
        self.with_held_injector_result(action);
    }

    #[cfg(test)]
    pub(crate) fn fallback_pending(&self) -> usize {
        self.fallback.pending() + self.birth_pending.load(Ordering::Acquire)
    }

    #[cfg(test)]
    pub(crate) fn with_held_injector_result<T>(&self, action: impl FnOnce() -> T) -> T {
        let _queue = lock(&self.queues[0].queue);
        action()
    }
}

#[cfg(test)]
#[expect(clippy::indexing_slicing, reason = "fixed queue indices in fixtures")]
mod tests {
    use super::*;
    use crate::metadata::ObjectMetadata;

    fn item(offset: usize, version: Option<usize>) -> MarkItem {
        let layout = crate::gc::Allocation::<u64>::layout();
        let storage = crate::allocation::Storage::global(layout).expect("fixture storage");
        // SAFETY: the storage has the complete descriptor's size and alignment.
        let allocation = unsafe {
            crate::allocation::Initialized::new(storage, crate::gc::Allocation::new(0_u64))
        };
        let mut metadata = ObjectMetadata::new_for_gc(
            allocation,
            layout.size(),
            crate::ObjectId::allocate().expect("fixture identity"),
        );
        metadata.trace.budget = 1;
        MarkItem {
            metadata: MetadataRef::new(metadata),
            offset,
            version,
            has_young: true,
            dirty_scan: false,
        }
    }

    #[test]
    fn mature_group_can_use_a_peer_without_consuming_edges_on_failure() {
        let queues = MarkQueues::new(2, 1);
        let edges: [_; 8] = std::array::from_fn(|_| item(0, None).metadata);
        let local = lock(&queues.queues[1].queue);
        let injector = lock(&queues.queues[0].queue);
        let peer = lock(&queues.queues[2].queue);
        assert_eq!(
            queues.append_mature(1, edges.len(), edges.iter().map(Some), |_| panic!(
                "no destination"
            )),
            0
        );
        drop(peer);
        assert_eq!(
            queues.append_mature(1, edges.len(), edges.iter().map(Some), |_| Some(true)),
            edges.len()
        );
        let mut peer = lock(&queues.queues[2].queue);
        for edge in edges {
            assert_eq!(
                peer.pop(false).expect("published edge").metadata.id,
                edge.id
            );
        }
        assert!(peer.pop(false).is_none());
        drop((injector, local));
    }

    #[test]
    fn segmented_injector_preserves_fifo_and_steal_order_across_buffers() {
        let queues = MarkQueues::new(1, 1);
        let mut expected = VecDeque::new();
        for _ in 0..=(3 * FRESH_SEGMENT_LIMIT) {
            let metadata = item(0, None).metadata;
            expected.push_back(metadata.id);
            queues.push_fresh(0, 16, metadata);
        }
        let mut local = 0;
        while !expected.is_empty() {
            let id = if local == 0 {
                expected.pop_front()
            } else {
                expected.pop_back()
            };
            let mut remaining = 1;
            let claim = queues.claim(local, 1, &mut remaining);
            let (found, budget) = claim
                .items
                .into_iter()
                .flatten()
                .next()
                .expect("queued birth");
            assert_eq!(Some(found.metadata.id), id);
            assert_eq!((claim.stolen, budget, remaining), (local, 1, 0));
            local ^= 1;
        }
        assert_eq!(queues.try_is_empty(), Some(true));
        queues.trim_empty();
        assert_eq!(queues.statistics(), (0, 0));
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn births_can_use_an_overflow_fifo_while_its_first_initializer_is_parked() {
        use std::sync::mpsc;
        use std::time::Duration;
        let queues = MarkQueues::new(0, 1);
        let mark = item(0, None);
        let id = mark.metadata.id;
        let _injector = lock(&queues.queues[0].queue);
        std::thread::scope(|scope| {
            let (entered, parked) = mpsc::channel();
            let (resume, receiver) = mpsc::channel();
            let first = &queues.birth_overflow[0];
            scope.spawn(move || {
                first.get_or_init(|| {
                    entered.send(()).expect("initializer paused");
                    receiver
                        .recv_timeout(Duration::from_secs(10))
                        .expect("resume initializer");
                    Box::new(crate::publication_queue::Queue::new())
                });
            });
            parked
                .recv_timeout(Duration::from_secs(5))
                .expect("first initializer");
            let (done, published) = mpsc::channel();
            let worker = &queues;
            scope.spawn(move || {
                worker.push_fresh(0, 16, mark.metadata);
                done.send(()).expect("published");
            });
            let completed = published.recv_timeout(Duration::from_secs(2)).is_ok();
            let claimed = completed.then(|| {
                queues
                    .claim(0, 1, &mut 1)
                    .items
                    .into_iter()
                    .flatten()
                    .next()
                    .expect("claim before first initializer resumes")
                    .0
                    .metadata
                    .id
            });
            resume.send(()).expect("resume first initializer");
            assert_eq!(claimed, Some(id));
        });
    }

    #[test]
    fn busy_workers_drain_each_birth_fifo_and_ordinary_continuations() {
        let queues = MarkQueues::new(1, 4);
        let guards: Vec<_> = queues
            .queues
            .iter()
            .map(|local| lock(&local.queue))
            .collect();
        let mut expected = [const { VecDeque::new() }; 3];
        for offset in 0..10 {
            for (source, fifo) in expected.iter_mut().enumerate() {
                let mark = item(offset, (source == 0).then_some(7));
                fifo.push_back(mark.metadata.id);
                if source == 0 {
                    queues.push(0, mark);
                } else {
                    queues.push_fresh(0, 16 * source, mark.metadata);
                }
            }
        }
        assert_eq!(queues.try_is_empty(), Some(false));
        for n in 0..30 {
            assert_eq!(queues.queued_sample(), Some(30 - n));
            let mut remaining = 1;
            let claim = queues.claim(1, 1, &mut remaining);
            let (mark, budget) = claim.items.into_iter().flatten().next().expect("overflow");
            let source = expected
                .iter()
                .position(|fifo| fifo.front() == Some(&mark.metadata.id))
                .expect("each source must preserve FIFO order");
            expected[source].pop_front();
            assert_eq!(mark.version, (source == 0).then_some(7));
            assert_eq!((budget, remaining, claim.stolen), (1, 0, 1));
            if n == 4 {
                assert!(expected.iter().all(|fifo| fifo.len() < 10));
            }
        }
        assert_eq!(queues.try_is_empty(), Some(true));
        assert_eq!(queues.queued_sample(), Some(0));
        drop(guards);
        assert_eq!(queues.statistics(), (0, 0));
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn competing_publishers_and_workers_deliver_each_mark_item_once() {
        use std::sync::Barrier;
        use std::time::{Duration, Instant};
        const SOURCES: usize = 5;
        const ROWS: usize = 512;
        fn publish(queues: &MarkQueues, source: usize, offset: usize) -> crate::ObjectId {
            let mark = item(offset, (source == 0).then_some(9));
            let id = mark.metadata.id;
            if source == 0 {
                queues.push(0, mark);
            } else {
                queues.push_fresh(0, source * 16, mark.metadata);
            }
            id
        }
        let queues = MarkQueues::new(1, 4);
        let finished = AtomicUsize::new(0);
        let start = Barrier::new(SOURCES + 4);
        let injector = lock(&queues.queues[0].queue);
        std::thread::scope(|scope| {
            // Seed the ordinary worker and every overflow source before releasing
            // any consumer. Subsequent publications compete with live claims.
            let mut expected = Vec::new();
            for n in 0..LOCAL_LIMIT {
                let mark = item(n, Some(7));
                expected.push(mark.metadata.id);
                queues.push(1, mark);
            }
            for source in 0..SOURCES {
                expected.push(publish(&queues, source, 0));
            }
            assert_eq!(queues.fallback.pending(), 1);
            assert_eq!(queues.birth_pending.load(Ordering::Acquire), SOURCES - 1);
            let mut publishers = Vec::new();
            for source in 0..SOURCES {
                let (queues, start, finished) = (&queues, &start, &finished);
                publishers.push(scope.spawn(move || {
                    let mut ids = Vec::new();
                    start.wait();
                    for n in 0..ROWS {
                        ids.push(publish(queues, source, n));
                    }
                    finished.fetch_add(1, Ordering::Release);
                    ids
                }));
            }
            let mut consumers = Vec::new();
            for _ in 0..4 {
                consumers.push(scope.spawn(|| {
                    let mut ids = Vec::new();
                    let deadline = Instant::now() + Duration::from_secs(20);
                    start.wait();
                    loop {
                        let mut budget = 8;
                        let claim = queues.claim(0, 8, &mut budget);
                        if budget == 8 {
                            if finished.load(Ordering::Acquire) == SOURCES
                                && queues.queued_sample() == Some(0)
                            {
                                break;
                            }
                            std::thread::yield_now();
                        }
                        assert!(Instant::now() < deadline, "overflow delivery stopped");
                        ids.extend(
                            claim
                                .items
                                .into_iter()
                                .flatten()
                                .map(|(m, _)| m.metadata.id),
                        );
                    }
                    ids
                }));
            }
            for publisher in publishers {
                expected.extend(publisher.join().expect("publisher"));
            }
            let mut actual: Vec<_> = consumers
                .into_iter()
                .flat_map(|t| t.join().expect("consumer"))
                .collect();
            expected.sort_unstable();
            actual.sort_unstable();
            assert_eq!(expected.len(), LOCAL_LIMIT + SOURCES * (ROWS + 1));
            assert_eq!(actual, expected);
            assert_eq!(queues.queued_sample(), Some(0));
            assert_eq!(queues.try_is_empty(), Some(true));
        });
        drop(injector);
        queues.trim_empty();
        assert_eq!(queues.statistics(), (0, 0));
    }

    #[test]
    fn prepared_work_is_private_until_helped_and_consumed_once() {
        let queues = MarkQueues::new(0, 1);
        let prepared = queues.fallback.prepare(item(42, Some(7)));
        assert_eq!(queues.fallback.pending(), 1);
        assert_eq!(queues.try_is_empty(), Some(true));
        while !queues.fallback.help(&prepared) {}
        assert_eq!(queues.try_is_empty(), Some(false));
        let mut remaining = 8;
        let claim = queues.claim(0, 8, &mut remaining);
        let (item, budget) = claim
            .items
            .into_iter()
            .flatten()
            .next()
            .expect("prepared work");
        assert_eq!(
            (item.offset, item.version, item.has_young, budget),
            (42, Some(7), true, 1)
        );
        assert!(queues.fallback.help(&prepared));
        assert_eq!(queues.try_is_empty(), Some(true));
        assert_eq!(queues.fallback.pending(), 0);
        drop(prepared);
        assert_eq!(queues.fallback.retained_nodes(), 2);
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn held_injector_does_not_block_required_publication() {
        use std::sync::{Arc, mpsc};
        use std::time::Duration;
        let queues = Arc::new(MarkQueues::new(0, 1));
        let (finished, result) = mpsc::channel();
        let (completed, worker) = queues.with_held_injector_result(|| {
            let queues = queues.clone();
            let worker = std::thread::spawn(move || {
                queues.push(0, item(42, Some(7)));
                finished.send(()).expect("published work");
            });
            (result.recv_timeout(Duration::from_secs(1)), worker)
        });
        worker.join().expect("publisher");
        assert_eq!(
            completed,
            Ok(()),
            "queue owner blocked required publication"
        );
        let mut remaining = 8;
        let claim = queues.claim(0, 8, &mut remaining);
        let (item, cost) = claim
            .items
            .into_iter()
            .flatten()
            .next()
            .expect("published item");
        assert_eq!((item.offset, item.version, cost), (42, Some(7), 1));
        assert_eq!(queues.try_is_empty(), Some(true));
    }

    #[test]
    fn busy_producer_queues_use_spare_capacity_before_allocating_fallback() {
        let queues = MarkQueues::new(3, 1);
        for offset in 0..LOCAL_LIMIT {
            queues.push(2, item(offset, Some(7)));
        }
        let local = lock(&queues.queues[1].queue);
        let injector = lock(&queues.queues[0].queue);
        queues.push(1, item(42, Some(9)));
        assert_eq!(queues.fallback.pending(), 0);
        assert_eq!(lock(&queues.queues[3].queue).len(), 1);
        assert_eq!(lock(&queues.queues[2].queue).len(), LOCAL_LIMIT);
        drop((local, injector));
        let mut remaining = 8;
        let claim = queues.claim(3, 1, &mut remaining);
        let (found, budget) = claim
            .items
            .into_iter()
            .flatten()
            .next()
            .expect("spare work");
        assert_eq!(
            (found.offset, found.version, found.has_young, budget),
            (42, Some(9), true, 1)
        );
        assert_eq!(remaining, 7);

        // When every ordinary queue is busy, the publication still reaches the
        // independently consumable fallback without waiting for any holder.
        let guards: Vec<_> = queues
            .queues
            .iter()
            .map(|local| lock(&local.queue))
            .collect();
        queues.push(1, item(43, Some(10)));
        assert_eq!(queues.fallback.pending(), 1);
        let claim = queues.claim(1, 1, &mut remaining);
        let (item, budget) = claim
            .items
            .into_iter()
            .flatten()
            .next()
            .expect("fallback work");
        assert_eq!(
            (item.offset, item.version, item.has_young, budget),
            (43, Some(10), true, 1)
        );
        drop(guards);
    }

    #[test]
    fn overflow_and_stealing_preserve_continuations_and_work_budgets() {
        let queues = MarkQueues::new(4, 1);
        for offset in 0..300 {
            queues.push(1, item(offset, Some(42)));
        }
        assert_eq!(lock(&queues.queues[1].queue).len(), LOCAL_LIMIT);
        assert_eq!(lock(&queues.queues[0].queue).len(), 300 - LOCAL_LIMIT);
        let mut offsets = Vec::new();
        while queues.try_is_empty() == Some(false) {
            let mut remaining = 3;
            let claim = queues.claim(2, 8, &mut remaining);
            assert!((1..=3).contains(&claim.stolen));
            assert_eq!(remaining + claim.stolen, 3);
            for (item, budget) in claim.items.into_iter().flatten() {
                assert_eq!(budget, 1);
                assert_eq!(item.version, Some(42));
                assert!(item.has_young);
                offsets.push(item.offset);
            }
        }
        offsets.sort_unstable();
        assert_eq!(offsets, (0..300).collect::<Vec<_>>());
        assert!(queues.statistics().1 > 0);
        queues.trim_empty();
        assert_eq!(queues.statistics(), (0, 0));
    }

    #[test]
    fn contended_local_and_injector_do_not_hide_a_victim() {
        for nonempty in [false, true] {
            let queues = MarkQueues::new(4, 1);
            if nonempty {
                queues.push(0, item(1, Some(7)));
                queues.push(1, item(2, Some(7)));
            }
            queues.push(4, item(9, None));
            let local = lock(&queues.queues[1].queue);
            let injector = lock(&queues.queues[0].queue);
            let mut remaining = 8;
            let claim = queues.claim(1, 8, &mut remaining);
            assert_eq!(claim.misses, if nonempty { 2 } else { 0 });
            assert_eq!(claim.stolen, 1);
            assert_eq!(remaining, 7);
            assert_eq!(claim.items.into_iter().flatten().count(), 1);
            drop((local, injector));
            // A previously observed empty queue must accept and expose later
            // publication; a failed nonempty claim must preserve its old work.
            queues.push(0, item(3, Some(7)));
            let claim = queues.claim(0, 8, &mut remaining);
            assert_eq!(
                claim.items.into_iter().flatten().count(),
                if nonempty { 2 } else { 1 }
            );
            if nonempty {
                assert_eq!(
                    queues
                        .claim(1, 8, &mut remaining)
                        .items
                        .into_iter()
                        .flatten()
                        .count(),
                    1
                );
            }
            assert_eq!(queues.try_is_empty(), Some(true));
        }
    }

    #[test]
    fn fallback_and_ordinary_queues_both_advance_and_preserve_continuations() {
        let queues = MarkQueues::new(0, 1);
        queues.with_held_injector(|| {
            for offset in 0..8 {
                queues.push(0, item(offset, Some(7)));
            }
        });
        for offset in 100..108 {
            queues.push(0, item(offset, Some(9)));
        }
        assert_eq!(queues.queued_sample(), Some(16));
        let mut found = Vec::new();
        for _ in 0..16 {
            let mut remaining = 1;
            let claim = queues.claim(0, 8, &mut remaining);
            let (item, budget) = claim
                .items
                .into_iter()
                .flatten()
                .next()
                .expect("available queue work");
            assert_eq!(budget, 1);
            assert!(item.has_young);
            assert_eq!(item.version, Some(if item.offset < 100 { 7 } else { 9 }));
            found.push(item.offset);
        }
        assert_eq!(found.iter().take(8).filter(|&&n| n < 100).count(), 4);
        assert_eq!(
            found
                .iter()
                .copied()
                .filter(|&n| n < 100)
                .collect::<Vec<_>>(),
            (0..8).collect::<Vec<_>>()
        );
        assert_eq!(
            found
                .iter()
                .copied()
                .filter(|&n| n >= 100)
                .collect::<Vec<_>>(),
            (100..108).collect::<Vec<_>>()
        );
        assert_eq!(queues.try_is_empty(), Some(true));
        queues.trim_empty();
        assert_eq!(queues.statistics(), (0, 0));
    }

    #[test]
    fn worker_queue_count_and_capacity_are_bounded() {
        let queues = MarkQueues::new(usize::MAX, 1);
        assert_eq!(queues.queues.len(), 17);
        assert_eq!(queues.worker_queue(usize::MAX), 16);
        assert_eq!(MarkQueues::new(0, 1).worker_queue(usize::MAX), 0);
    }

    #[test]
    fn busy_single_queue_preserves_work_and_budget_without_certifying_empty() {
        let queues = MarkQueues::new(0, 1);
        queues.push(0, item(42, Some(7)));
        queues.with_held_injector(|| {
            let mut budget = 8;
            let claim = queues.claim(0, 8, &mut budget);
            assert_eq!(budget, 8);
            assert_eq!(claim.misses, 1);
            assert!(claim.items.iter().all(Option::is_none));
            assert_eq!(queues.try_is_empty(), None);
        });
        let mut budget = 8;
        let mut claim = queues.claim(0, 8, &mut budget);
        let (item, cost) = claim.items[0].take().expect("preserved work");
        assert_eq!(
            (item.offset, item.version, cost, budget),
            (42, Some(7), 1, 7)
        );
        assert!(claim.items.iter().all(Option::is_none));
        assert_eq!(queues.try_is_empty(), Some(true));
        queues.with_held_injector(|| assert_eq!(queues.try_is_empty(), Some(true)));
    }
}
