//! Persistent tracing queues. A claim must be protected until its in-flight reservation.
use crate::metadata::MetadataRef;
use arc_swap::ArcSwapOption;
use once_cell::race::OnceBox;
use std::borrow::Cow;
use std::collections::{LinkedList, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError, Weak};

const LOCAL_LIMIT: usize = 256;
// Bound contiguous injector growth to 4 KiB on either pointer width.
const FRESH_SEGMENT_LIMIT: usize = 4096 / size_of::<MetadataRef>();

#[derive(Debug)]
pub(crate) struct MarkItem<M = MetadataRef> {
    // Keep the exact allocation identity even if its descriptor address is reused.
    // This Arc owns metadata only; callbacks must acquire a storage lease.
    pub(crate) metadata: M,
    pub(crate) offset: usize,
    pub(crate) version: Option<usize>,
    pub(crate) has_young: bool,
    pub(crate) dirty_scan: bool,
}

impl<M> MarkItem<M> {
    fn map<N>(self, convert: impl FnOnce(M) -> N) -> MarkItem<N> {
        MarkItem {
            metadata: convert(self.metadata),
            offset: self.offset,
            version: self.version,
            has_young: self.has_young,
            dirty_scan: self.dirty_scan,
        }
    }
}

impl MarkItem<Cow<'_, MetadataRef>> {
    pub(crate) fn owned(self) -> MarkItem {
        self.map(Cow::into_owned)
    }
}

pub(crate) type Packet = Arc<Group>;
#[derive(Debug)]
pub(crate) struct Group {
    targets: [MetadataRef; 8],
    budgets: [u8; 8],
    prefix: [u16; 9],
}

impl Group {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the callback budget is clamped to 1 through 32"
    )]
    fn new(targets: [&MetadataRef; 8]) -> Self {
        let budgets = targets.map(|m| m.trace.budget.clamp(1, 32) as u8);
        let mut prefix = [0; 9];
        let mut sum = 0;
        for (next, budget) in prefix.iter_mut().skip(1).zip(budgets) {
            sum += u16::from(budget);
            *next = sum;
        }
        Self {
            targets: targets.map(Clone::clone),
            budgets,
            prefix,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Visit {
    packet: Packet,
    start: u8,
    end: u8,
    // Zero means the general retention and storage checks are still required.
    strong_epoch: u64,
}
#[expect(
    clippy::indexing_slicing,
    reason = "packet claims split ranges within the eight immutable targets"
)]
impl Visit {
    fn len(&self) -> usize {
        usize::from(self.end - self.start)
    }

    pub(crate) fn for_each(&self, last_budget: u8, mut visit: impl FnMut(&MetadataRef, usize)) {
        for (i, target) in self.targets().iter().enumerate() {
            let index = i + usize::from(self.start);
            let budget = if last_budget != 0 && index + 1 == usize::from(self.end) {
                last_budget
            } else {
                self.packet.budgets[index]
            };
            visit(target, usize::from(budget));
        }
    }

    fn targets(&self) -> &[MetadataRef] {
        &self.packet.targets[usize::from(self.start)..usize::from(self.end)]
    }
}

const PACKET_SEGMENT: usize = 64;

#[derive(Debug, Default)]
struct Packets {
    front: VecDeque<Visit>,
    #[expect(
        clippy::linkedlist,
        reason = "a sealed segment never moves previous segments under the queue lock"
    )]
    rest: LinkedList<VecDeque<Visit>>,
}
impl Packets {
    fn is_empty(&self) -> bool {
        self.front.is_empty()
    }
    fn front_mut(&mut self) -> Option<&mut Visit> {
        self.front.front_mut()
    }
    fn capacity(&self) -> usize {
        self.front.capacity() + self.rest.iter().map(VecDeque::capacity).sum::<usize>()
    }
    #[expect(
        clippy::expect_used,
        reason = "an empty or full tail just received a writable segment"
    )]
    fn push_back(&mut self, value: Visit) {
        if self.rest.is_empty() && self.front.len() < PACKET_SEGMENT {
            self.front.push_back(value);
            return;
        }
        if self
            .rest
            .back()
            .is_none_or(|tail| tail.len() == PACKET_SEGMENT)
        {
            self.rest.push_back(VecDeque::with_capacity(PACKET_SEGMENT));
        }
        self.rest
            .back_mut()
            .expect("writable packet segment")
            .push_back(value);
    }
    fn pop_front(&mut self) -> Option<Visit> {
        let item = self.front.pop_front()?;
        if self.front.is_empty()
            && let Some(next) = self.rest.pop_front()
        {
            self.front = next;
        }
        Some(item)
    }
}

const CACHE_BUCKETS: usize = 64;
type CacheBucket = Box<[Option<Weak<Group>>; 16]>;
type OwnerBucket = [ArcSwapOption<Group>; 16];
#[derive(Debug)]
struct Owners {
    buckets: [OnceBox<OwnerBucket>; CACHE_BUCKETS],
    allocated: portable_atomic::AtomicU64,
}
impl Default for Owners {
    fn default() -> Self {
        Self {
            buckets: [const { OnceBox::new() }; CACHE_BUCKETS],
            allocated: portable_atomic::AtomicU64::new(0),
        }
    }
}
#[expect(
    clippy::indexing_slicing,
    reason = "the cache key is masked to the fixed number of slots"
)]
impl Owners {
    fn install(&self, index: usize, packet: Packet) {
        let word = index / 16;
        let bucket = self.buckets[word]
            .get_or_init(|| Box::new(std::array::from_fn(|_| ArcSwapOption::empty())));
        bucket[index % 16].store(Some(packet));
        if self.allocated.load(Ordering::Relaxed) & (1 << word) == 0 {
            self.allocated.fetch_or(1 << word, Ordering::Release);
        }
    }
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

#[derive(Debug)]
struct Queue {
    packets: Packets,
    packet_items: usize,
    groups: [Option<CacheBucket>; CACHE_BUCKETS],
    prefer_scalar: bool,
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

impl Default for Queue {
    fn default() -> Self {
        Self {
            packets: Packets::default(),
            packet_items: 0,
            groups: std::array::from_fn(|_| None),
            prefer_scalar: false,
            fresh: VecDeque::new(),
            full: LinkedList::new(),
            buffered: 0,
            spare: VecDeque::new(),
            current: VecDeque::new(),
        }
    }
}

impl Queue {
    fn len(&self) -> usize {
        self.fresh.len() + self.buffered + self.current.len() + self.packet_items
    }

    fn capacity(&self) -> usize {
        self.current.capacity()
            + self.packets.capacity() * 8
            + self.fresh.capacity()
            + self.spare.capacity()
            + self.full.iter().map(VecDeque::capacity).sum::<usize>()
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "the bucket and slot partition a masked cache key"
    )]
    fn cached_targets(&self, targets: &[Option<&MetadataRef>; 8]) -> Option<Packet> {
        let key = targets[0]?.id.to_i64().cast_unsigned() >> 3;
        let index = (key as usize) & (CACHE_BUCKETS * 16 - 1);
        let packet = self.groups[index / 16].as_ref()?[index % 16]
            .as_ref()?
            .upgrade()?;
        packet
            .targets
            .iter()
            .zip(targets)
            .all(|(known, target)| target.is_some_and(|m| MetadataRef::ptr_eq(known, m)))
            .then_some(packet)
    }

    fn push_packet(&mut self, packet: Packet, strong_epoch: u64) {
        self.packets.push_back(Visit {
            packet,
            start: 0,
            end: 8,
            strong_epoch,
        });
        self.packet_items += 8;
    }

    #[expect(
        clippy::expect_used,
        clippy::indexing_slicing,
        reason = "a full group has eight filled entries and the cache index is masked"
    )]
    fn publish_group(
        &mut self,
        owners: &Owners,
        fresh: &[Option<&MetadataRef>; 8],
        count: usize,
        strong_epoch: u64,
    ) {
        if count == 8 {
            let targets = fresh.map(|m| m.expect("complete group"));
            let key = targets[0].id.to_i64().cast_unsigned() >> 3;
            let index = (key as usize) & (CACHE_BUCKETS * 16 - 1);
            let bucket = self.groups[index / 16]
                .get_or_insert_with(|| Box::new(std::array::from_fn(|_| None)));
            let slot = &mut bucket[index % 16];
            // This path installs a group only when the exact early cache
            // lookup missed. The queue retains a weak reference after sweep.
            let packet = Arc::new(Group::new(targets));
            *slot = Some(Arc::downgrade(&packet));
            owners.install(index, packet.clone());
            self.push_packet(packet, strong_epoch);
        } else {
            for metadata in fresh.iter().take(count).flatten() {
                self.push_fresh((*metadata).clone());
            }
        }
    }

    #[expect(
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::cast_possible_truncation,
        reason = "the nonempty front range covers at most eight callbacks, each costing at most 32"
    )]
    fn claim_group(&mut self, result: &mut Claim, limit: usize, remaining: &mut usize) -> bool {
        let Some(front) = self.packets.front_mut() else {
            return false;
        };
        let start = usize::from(front.start);
        let end = usize::from(front.end).min(start + limit);
        if end == start || *remaining == 0 {
            return false;
        }
        let base = front.packet.prefix[start];
        let cost = usize::from(front.packet.prefix[end] - base);
        let (end, last_budget) = if cost <= *remaining {
            *remaining -= cost;
            (end, 0)
        } else {
            // Prefixes are strictly increasing. Only the last item in a
            // truncated claim can receive less than its constructor's budget.
            let allowed = base + *remaining as u16;
            let index =
                start + front.packet.prefix[start + 1..=end].partition_point(|&p| p < allowed);
            let tail = (allowed - front.packet.prefix[index]) as u8;
            *remaining = 0;
            (index + 1, tail)
        };
        let count = end - start;
        let split = end as u8;
        let visit = if split == front.end {
            self.packets.pop_front().expect("claimed packet")
        } else {
            let start = std::mem::replace(&mut front.start, split);
            Visit {
                packet: front.packet.clone(),
                start,
                end: split,
                strong_epoch: front.strong_epoch,
            }
        };
        self.packet_items -= count;
        result.packet = Some((visit, last_budget));
        true
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
    owners: Owners,
    queue: Mutex<Queue>,
    // The lane mutex serializes strong discoveries. Readers at full mark
    // closure sample the tally without acquiring a possibly parked lane.
    mature_strong: portable_atomic::AtomicU64,
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
    collector: Weak<crate::GarbageCollector>,
    offers: AtomicUsize,
    pub(crate) mature_deletions: crate::mature_deletions::MatureDeletions,
    direct_identity: OnceBox<Arc<()>>,
    roots: OnceBox<graph_birth_queue::Queue<RootProposal>>,
    inbox: OnceBox<graph_birth_queue::Queue<GraphBirth>>,
    strong_closed: AtomicBool,
}

#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
impl Drop for BirthEpoch {
    fn drop(&mut self) {
        crate::collector::before_epoch_destruction();
    }
}

// The high bit stops admission permanently. A read-only epoch witness never
// contributes to the count and cannot keep disposal runnable.
const SEALED: usize = 1 << (usize::BITS - 1);

struct Offer<'a>(&'a Arc<BirthEpoch>);

impl Drop for Offer<'_> {
    fn drop(&mut self) {
        if self.0.offers.fetch_sub(1, Ordering::AcqRel) & SEALED != 0 {
            self.0.signal();
        }
    }
}

pub(crate) struct DetachedEpoch(Arc<BirthEpoch>);

impl Drop for DetachedEpoch {
    fn drop(&mut self) {
        if self.0.has_publishers() || self.0.has_visible_proposals() {
            self.0.signal();
        }
    }
}

impl BirthEpoch {
    fn begin_offer(self: &Arc<Self>) -> Option<Offer<'_>> {
        self.offers
            .try_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                (n < SEALED - 1).then(|| n + 1)
            })
            .ok()?;
        Some(Offer(self))
    }

    fn signal(self: &Arc<Self>) {
        if let Some(gc) = self.collector.upgrade() {
            gc.schedule_stale_epoch(self);
        }
    }

    pub(crate) fn has_publishers(&self) -> bool {
        self.offers.load(Ordering::Acquire) & !SEALED != 0
    }

    /// An advisory limit for repeated deletion barriers. Concurrent publishers
    /// may each contribute one additional in-flight offer before observing it.
    pub(crate) fn has_root_capacity(&self, limit: usize) -> bool {
        self.roots.get().is_none_or(|roots| roots.pending() < limit)
    }

    pub(crate) fn has_visible_proposals(&self) -> bool {
        self.roots.get().is_some_and(|q| !q.is_empty())
            || self.inbox.get().is_some_and(|q| !q.is_empty())
    }

    #[cfg(test)]
    pub(crate) fn proposal_count(&self) -> usize {
        self.roots
            .get()
            .map_or(0, graph_birth_queue::Queue::pending)
            + self
                .inbox
                .get()
                .map_or(0, graph_birth_queue::Queue::pending)
    }

    pub(crate) fn discard_one(&self) -> bool {
        self.roots
            .get()
            .and_then(graph_birth_queue::Queue::pop)
            .is_some()
            || self
                .inbox
                .get()
                .and_then(graph_birth_queue::Queue::pop)
                .is_some()
    }

    // An obsolete descriptor witness retains only the small token's control
    // block. It can never retain the mark queue or its descriptor graph.
    pub(crate) fn direct_witness(&self) -> Weak<()> {
        Arc::downgrade(self.direct_identity.get_or_init(|| Box::new(Arc::new(()))))
    }

    pub(crate) fn matches_witness(&self, witness: &Weak<()>) -> bool {
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
    packet: Option<(Visit, u8)>,
    pub(crate) items: [Option<(MarkItem, usize)>; 8],
    pub(crate) stolen: usize,
    pub(crate) misses: usize,
}

impl Claim {
    pub(crate) fn strong_packet(&self, epoch: u64) -> bool {
        epoch != 0
            && self
                .packet
                .as_ref()
                .is_some_and(|(visit, _)| visit.strong_epoch == epoch)
    }

    pub(crate) fn take_packet(&mut self) -> Option<(Visit, u8)> {
        self.packet.take()
    }

    pub(crate) fn count(&self) -> usize {
        if let Some((visit, _)) = &self.packet {
            return visit.len();
        }
        self.items.iter().flatten().count()
    }

    pub(crate) fn for_each(
        &mut self,
        mut visit: impl FnMut(MarkItem<Cow<'_, MetadataRef>>, usize),
    ) {
        if let Some((packet, last_budget)) = self.packet.take() {
            packet.for_each(last_budget, |target, budget| {
                visit(
                    MarkItem {
                        metadata: Cow::Borrowed(target),
                        offset: 0,
                        version: None,
                        has_young: false,
                        dirty_scan: false,
                    },
                    budget,
                );
            });
            return;
        }
        for (item, budget) in self.items.iter_mut().filter_map(Option::take) {
            visit(item.map(Cow::Owned), budget);
        }
    }
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
    pub(crate) fn open_graph_inbox(
        &self,
        number: u64,
        old_only: bool,
        collector: Weak<crate::GarbageCollector>,
    ) {
        self.graph_inbox.store(Some(Arc::new(BirthEpoch {
            number,
            old_only,
            collector,
            offers: AtomicUsize::new(0),
            mature_deletions: crate::mature_deletions::MatureDeletions::default(),
            direct_identity: OnceBox::new(),
            roots: OnceBox::new(),
            inbox: OnceBox::new(),
            strong_closed: AtomicBool::new(false),
        })));
    }

    pub(crate) fn close_graph_inbox(&self) -> Option<DetachedEpoch> {
        self.root_offered.store(false, Ordering::Release);
        self.graph_offered.store(false, Ordering::Release);
        let epoch = self.graph_inbox.swap(None)?;
        if epoch.old_only {
            epoch.mature_deletions.abandon();
        }
        epoch.offers.fetch_or(SEALED, Ordering::AcqRel);
        Some(DetachedEpoch(epoch))
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

    pub(crate) fn propose_graph(&self, epoch: &Arc<BirthEpoch>, graph: &GraphBirth) -> bool {
        let Some(_offer) = epoch.begin_offer() else {
            return false;
        };
        let queue = epoch
            .inbox
            .get_or_init(|| Box::new(graph_birth_queue::Queue::new()));
        self.graph_offered.store(true, Ordering::Release);
        queue.push(graph.clone());
        true
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

    pub(crate) fn propose_root(&self, epoch: &Arc<BirthEpoch>, proposal: RootProposal) -> bool {
        let Some(_offer) = epoch.begin_offer() else {
            return false;
        };
        let queue = epoch
            .roots
            .get_or_init(|| Box::new(graph_birth_queue::Queue::new()));
        self.root_offered.store(true, Ordering::Release);
        queue.push(proposal);
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        crate::collector::pause_after_proposal_link();
        true
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
        strong_epoch: u64,
        edges: impl Iterator<Item = Option<&'a MetadataRef>>,
        mut mark: impl FnMut(&MetadataRef, bool) -> Option<bool>,
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
            let mut targets = [None; 8];
            let mut count = 0;
            let mut source_edges = edges.take(limit.min(8));
            let inputs: [_; 8] = std::array::from_fn(|_| source_edges.next().flatten());
            let cached = queue.cached_targets(&inputs);
            for edge in inputs {
                let Some(metadata) = edge else { break };
                let Some(fresh) = mark(metadata, cached.is_some()) else {
                    break;
                };
                if fresh {
                    targets[count] = Some(metadata);
                    count += 1;
                }
                handled += 1;
            }
            if count == 8
                && let Some(packet) = cached
            {
                queue.push_packet(packet, strong_epoch);
            } else {
                queue.publish_group(&self.queues[source].owners, &targets, count, strong_epoch);
            }
            if strong_epoch != 0 && count != 0 {
                let tally = &self.queues[source].mature_strong;
                tally.store(
                    tally.load(Ordering::Relaxed) + count as u64,
                    Ordering::Release,
                );
            }
            self.queues[source]
                .queued
                .store(queue.len(), Ordering::Release);
            return handled;
        }
        0
    }

    pub(crate) fn reset_mature_strong(&self) {
        // Full-epoch initialization excludes all reserved marking callbacks.
        for local in &self.queues {
            local.mature_strong.store(0, Ordering::Release);
        }
    }

    pub(crate) fn mature_strong(&self) -> u64 {
        self.queues
            .iter()
            .map(|local| local.mature_strong.load(Ordering::Acquire))
            .sum()
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
        if *remaining == 0 {
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
            // Keep a suffix available to another helper when a thief parks
            // inside its first user trace. A final item remains stealable.
            let limit = if steal {
                limit.min(queue.len().div_ceil(2))
            } else {
                limit
            }
            .clamp(1, 8);
            let other = queue.len() != queue.packet_items;
            let packet_turn = !other || !queue.prefer_scalar;
            queue.prefer_scalar = !queue.packets.is_empty() && packet_turn;
            if packet_turn
                && !queue.packets.is_empty()
                && queue.claim_group(result, limit, remaining)
            {
                self.queues[source]
                    .queued
                    .store(queue.len(), Ordering::Release);
                if steal {
                    result.stolen = result.count();
                    self.queues[local]
                        .victim
                        .store(source + 1, Ordering::Relaxed);
                }
                return;
            }
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

    pub(crate) fn take_cache_entry(&self, cursor: &mut usize) -> crate::index::Scan<Packet> {
        let index = *cursor;
        let Some(local) = self.queues.get(index / (CACHE_BUCKETS * 16)) else {
            return crate::index::Scan::Done;
        };
        let bucket = index / 16 % CACHE_BUCKETS;
        let mask = local.owners.allocated.load(Ordering::Acquire) & (u64::MAX << bucket);
        if mask & (1 << bucket) == 0 {
            let next = mask.trailing_zeros() as usize;
            *cursor += (next - bucket) * 16 - index % 16;
            return crate::index::Scan::Advance;
        }
        *cursor += 1;
        let packet = local.owners.buckets[bucket]
            .get()
            .and_then(|slots| slots[index % 16].swap(None));
        packet.map_or(crate::index::Scan::Advance, |p| {
            crate::index::Scan::Entry(index, p)
        })
    }

    fn release_caches(&self) {
        let mut cursor = 0;
        while !matches!(self.take_cache_entry(&mut cursor), crate::index::Scan::Done) {}
    }

    #[cfg(test)]
    pub(crate) fn cached_groups(&self) -> usize {
        self.queues
            .iter()
            .flat_map(|lane| lane.owners.buckets.iter().filter_map(OnceBox::get))
            .flat_map(|bucket| bucket.iter())
            .filter(|slot| slot.load().is_some())
            .count()
    }

    pub(crate) fn trim_empty(&self) {
        self.release_caches();
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
        self.release_caches();
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
        budgeted_item(offset, version, 1)
    }

    fn budgeted_item(offset: usize, version: Option<usize>, budget: usize) -> MarkItem {
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
        metadata.trace.budget = budget;
        MarkItem {
            metadata: MetadataRef::new(metadata),
            offset,
            version,
            has_young: true,
            dirty_scan: false,
        }
    }

    #[test]
    fn group_budget_matches_scalar_assignments_for_every_slice() {
        for costs in [[1, 2, 5, 4, 13, 8, 31, 32], [32; 8]] {
            let edges = costs.map(|cost| budgeted_item(0, None, cost).metadata);
            let packet = Arc::new(Group::new(edges.each_ref()));
            let mut queue = Queue::default();
            for start in 0..8 {
                for limit in 0..=8 {
                    for allowance in 0..=257 {
                        queue.packets.push_back(Visit {
                            packet: packet.clone(),
                            start: u8::try_from(start).unwrap(),
                            end: 8,
                            strong_epoch: 17,
                        });
                        queue.packet_items = 8 - start;
                        let mut left = allowance;
                        let mut expected = Vec::new();
                        for (i, &cost) in costs.iter().enumerate().skip(start).take(limit) {
                            if left == 0 {
                                break;
                            }
                            let assigned = left.min(cost);
                            expected.push((edges[i].id, assigned));
                            left -= assigned;
                        }
                        let mut remaining = allowance;
                        let mut claim = Claim::default();
                        assert_eq!(
                            queue.claim_group(&mut claim, limit, &mut remaining),
                            !expected.is_empty()
                        );
                        let mut got = Vec::new();
                        claim.for_each(|item, budget| got.push((item.metadata.id, budget)));
                        assert_eq!(got, expected);
                        assert_eq!(remaining, left);
                        assert_eq!(queue.packet_items, 8 - start - expected.len());
                        if let Some(suffix) = queue.packets.pop_front() {
                            assert_eq!(usize::from(suffix.start), start + expected.len());
                            assert_eq!(suffix.end, 8);
                        } else {
                            assert_eq!(start + expected.len(), 8);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn strong_tally_counts_only_fresh_mature_edges_across_lanes_and_epochs() {
        let queues = MarkQueues::new(2, 1);
        let edges: [_; 8] = std::array::from_fn(|_| item(0, None).metadata);
        assert_eq!(
            queues.append_mature(1, 8, 17, edges.iter().map(Some), |_, _| Some(true)),
            8
        );
        let mut marks = [Some(false), Some(true), None].into_iter();
        assert_eq!(
            queues.append_mature(2, 8, 17, edges.iter().map(Some), |_, _| marks
                .next()
                .flatten()),
            2
        );
        // Weaker discoveries publish tasks but do not certify strong survival.
        assert_eq!(
            queues.append_mature(0, 8, 0, edges.iter().map(Some), |_, _| Some(true)),
            8
        );
        let parked = lock(&queues.queues[1].queue);
        assert_eq!(queues.mature_strong(), 9);
        queues.reset_mature_strong();
        assert_eq!(queues.mature_strong(), 0);
        assert_eq!(
            queues.append_mature(1, 8, 20, edges.iter().map(Some), |_, _| Some(true)),
            8
        );
        assert_eq!(queues.mature_strong(), 8);
        drop(parked);
    }

    #[test]
    fn mature_group_can_use_a_peer_without_consuming_edges_on_failure() {
        let queues = MarkQueues::new(2, 1);
        let edges: [_; 8] = std::array::from_fn(|_| item(0, None).metadata);
        let local = lock(&queues.queues[1].queue);
        let injector = lock(&queues.queues[0].queue);
        let peer = lock(&queues.queues[2].queue);
        assert_eq!(
            queues.append_mature(1, edges.len(), 0, edges.iter().map(Some), |_, _| panic!(
                "no destination"
            )),
            0
        );
        drop(peer);
        assert_eq!(
            queues.append_mature(1, edges.len(), 0, edges.iter().map(Some), |_, _| Some(true)),
            edges.len()
        );
        let mut claim = queues.claim(2, 8, &mut 8);
        let mut found = Vec::new();
        claim.for_each(|item, _| found.push(item.metadata.id));
        assert_eq!(found, edges.map(|edge| edge.id));
        drop((injector, local));
    }

    #[test]
    fn parked_remote_claim_leaves_its_packet_suffix_and_last_item_stealable() {
        let queues = MarkQueues::new(2, 1);
        let edges: [_; 8] = std::array::from_fn(|_| item(0, None).metadata);
        assert_eq!(
            queues.append_mature(0, 8, 37, edges.iter().map(Some), |_, _| Some(true)),
            8
        );
        let mut claims = Vec::new();
        for (worker, allowance, expected) in [(1, 8, 4), (2, 1, 1), (1, 8, 2), (2, 8, 1)] {
            assert_eq!(queues.try_is_empty(), Some(false));
            let mut remaining = allowance;
            let claim = queues.claim(worker, 8, &mut remaining);
            assert_eq!(claim.stolen, expected);
            assert_eq!(claim.count(), expected);
            assert!(claim.strong_packet(37));
            assert!(!claim.strong_packet(38));
            assert_eq!(remaining, allowance - expected);
            claims.push(claim);
        }
        assert_eq!(queues.try_is_empty(), Some(true));
        let mut found = Vec::new();
        for claim in &mut claims {
            claim.for_each(|item, budget| {
                assert_eq!(budget, 1);
                found.push(item.metadata.id);
            });
        }
        assert_eq!(found, edges.map(|edge| edge.id));
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
