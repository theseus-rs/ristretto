//! Exact address reservations and resumable object and nursery scans.
//! Live root and sweep operations still serialize on the address shard. A
//! publisher owns only its slot, even while all live-operation guards are held.
use super::{Cursor, Scan, Shards, address_bucket, shard_count};
use crate::metadata::MetadataRef;
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use once_cell::race::OnceBox;
use portable_atomic::AtomicU64;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::atomic::{
    AtomicUsize,
    Ordering::{AcqRel, Acquire, Relaxed},
};
use std::sync::{Arc, MutexGuard, OnceLock};

const WORD: usize = 64;
const WORDS: usize = 64;
const PAGE_SIZE: usize = WORD * WORDS;
const PAGE_MASK: usize = PAGE_SIZE - 1;

// Group eight-byte aligned addresses while preserving every low-bit pattern.
// Numeric scan keys never grant permission to dereference an address.
fn position(address: usize) -> usize {
    (address & !(PAGE_SIZE * 8 - 1)) | ((address & 7) * PAGE_SIZE) | ((address >> 3) & PAGE_MASK)
}

fn address_of(key: usize) -> usize {
    (key & !(PAGE_SIZE * 8 - 1)) | ((key & PAGE_MASK) << 3) | ((key / PAGE_SIZE) & 7)
}

// The shard index already encodes the low hash bits of the address. Omit
// those bits from its page key so all shards fill their pages densely.
fn packed_key(address: usize, bits: u32) -> usize {
    let packed = ((address >> bits) & !15) | (address & 15);
    position(packed) + usize::from(bits != 0)
}

fn unpack_key(key: usize, shard: usize, bits: u32) -> usize {
    let packed = address_of(key - usize::from(bits != 0));
    let address = ((packed & !15) << bits) | (packed & 15);
    let mask = (1 << bits) - 1;
    address | (((shard ^ address_bucket(address)) & mask) << 4)
}

type Pages<T> = ArcSwap<Directory<Arc<T>>>;
#[path = "address_cell.rs"]
mod address_cell;
use address_cell::Cell;

#[derive(Debug, Default)]
struct Head {
    pages: Pages<Page>,
    young: Pages<Bitmap>,
    indexed: AtomicUsize,
    shard: usize,
}

pub(crate) struct Sealed {
    key: usize,
    shard: usize,
    objects: Option<Arc<Page>>,
    young: Option<Arc<Bitmap>>,
}

impl Sealed {
    fn unlink(&self, head: &Head) {
        let base = (self.key & !PAGE_MASK) as u64;
        if let Some(page) = &self.objects {
            unlink(&head.pages, base, page);
        }
        if let Some(page) = &self.young {
            unlink(&head.young, base, page);
        }
    }
}

/// The expected owning root and page prevent deletion after either is replaced.
fn unlink<T>(head: &Pages<T>, base: u64, sealed: &Arc<T>) {
    loop {
        let current = head.load();
        if !current.get(base).is_some_and(|p| Arc::ptr_eq(p, sealed)) {
            return;
        }
        let next = Arc::new(current.remove(base).0);
        if Arc::ptr_eq(&head.compare_and_swap(&*current, next), &current) {
            return;
        }
    }
}

#[derive(Debug)]
struct Bitmap {
    // Counts live bits and address claims between pinning and setting their
    // bit. Once this reaches zero a stale directory view cannot reopen it.
    users: AtomicUsize,
    used_words: AtomicU64,
    words: [AtomicU64; WORDS],
}

#[expect(
    clippy::indexing_slicing,
    reason = "the masked offset selects a word inside its page"
)]
impl Bitmap {
    fn new(offset: usize) -> Self {
        let result = Self {
            users: AtomicUsize::new(1),
            used_words: AtomicU64::new(1 << (offset / WORD)),
            words: [const { AtomicU64::new(0) }; WORDS],
        };
        result.words[offset / WORD].store(1 << (offset % WORD), Relaxed);
        result
    }

    fn pin(&self) -> bool {
        self.users
            .try_update(AcqRel, Acquire, |users| (users != 0).then_some(users + 1))
            .is_ok()
    }

    fn unpin(&self) -> bool {
        self.users.fetch_sub(1, AcqRel) == 1
    }

    fn set(&self, offset: usize) -> bool {
        let bit = 1 << (offset % WORD);
        let word = offset / WORD;
        let word_bit = 1 << word;
        if self.used_words.load(Relaxed) & word_bit == 0 {
            self.used_words.fetch_or(word_bit, AcqRel);
        }
        self.words[word].fetch_or(bit, AcqRel) & bit == 0
    }

    fn remove(&self, offset: usize) -> bool {
        let bit = 1 << (offset % WORD);
        self.remove_bit(bit, offset).unwrap_or(false)
    }

    fn remove_bit(&self, bit: u64, offset: usize) -> Option<bool> {
        (self.words[offset / WORD].fetch_and(!bit, AcqRel) & bit != 0).then(|| self.unpin())
    }

    /// Retire exactly the set bits in one physical word. An allocator can
    /// independently hold a pin before publishing a new bit into this word.
    fn remove_mask(&self, word: usize, mask: u64) -> bool {
        let prior = self.words[word].fetch_and(!mask, AcqRel);
        let removed = (prior & mask).count_ones() as usize;
        removed != 0 && self.users.fetch_sub(removed, AcqRel) == removed
    }

    fn next(&self, start: usize) -> Option<usize> {
        self.next_word(start)
            .map(|(base, bits)| base + bits.trailing_zeros() as usize)
    }

    fn next_word(&self, start: usize) -> Option<(usize, u64)> {
        let first = start / WORD;
        if first >= WORDS {
            return None;
        }
        // A word enters the summary before its first bit is published. Its
        // summary bit stays set even when the word empties, so removal never
        // races a concurrent insertion to clear the summary.
        let mut candidates = self.used_words.load(Acquire) & (u64::MAX << first);
        while candidates != 0 {
            let word = candidates.trailing_zeros() as usize;
            let mask = if word == first {
                u64::MAX << (start % WORD)
            } else {
                u64::MAX
            };
            let bits = self.words[word].load(Acquire) & mask;
            if bits != 0 {
                return Some((word * WORD, bits));
            }
            candidates &= candidates - 1;
        }
        None
    }

    #[cfg(test)]
    fn contains(&self, key: usize) -> bool {
        let offset = key & PAGE_MASK;
        self.words[offset / WORD].load(Acquire) & (1 << (offset % WORD)) != 0
    }
}

#[derive(Debug)]
struct Page {
    occupied: Bitmap,
    chunks: [OnceBox<[OnceLock<Cell>; WORD]>; WORDS],
}

#[expect(
    clippy::indexing_slicing,
    reason = "key is split into masked word and slot offsets"
)]
impl Page {
    fn new(offset: usize) -> Self {
        Self {
            occupied: Bitmap::new(offset),
            chunks: [const { OnceBox::new() }; WORDS],
        }
    }

    fn entry(&self, key: usize) -> Option<&Cell> {
        self.chunks[(key & PAGE_MASK) / WORD].get()?[key % WORD].get()
    }

    fn vacant_cell(&self, key: usize) -> &OnceLock<Cell> {
        // A concurrent initializer prepares its own private chunk. Readers and
        // other publishers never wait for a stalled chunk allocator.
        let chunk = self.chunks[(key & PAGE_MASK) / WORD]
            .get_or_init(|| Box::new([const { OnceLock::new() }; WORD]));
        &chunk[key % WORD]
    }
}

trait Occupancy {
    fn bitmap(&self) -> &Bitmap;
}
impl Occupancy for Page {
    fn bitmap(&self) -> &Bitmap {
        &self.occupied
    }
}
impl Occupancy for Bitmap {
    fn bitmap(&self) -> &Bitmap {
        self
    }
}

fn reserve<T: Occupancy>(
    head: &Pages<T>,
    key: usize,
    create: impl Fn(usize) -> T,
) -> Option<Arc<T>> {
    let base = (key & !PAGE_MASK) as u64;
    let offset = key & PAGE_MASK;
    loop {
        let current = head.load();
        if let Some(page) = current.get(base)
            && page.bitmap().pin()
        {
            if page.bitmap().set(offset) {
                return Some(page.clone());
            }
            if page.bitmap().unpin() {
                unlink(head, base, page);
            }
            return None;
        }
        let page = Arc::new(create(offset));
        let (next, _) = current.insert(base, page.clone())?;
        if Arc::ptr_eq(&head.compare_and_swap(&*current, Arc::new(next)), &current) {
            return Some(page);
        }
    }
}

/// Holds one occupied bit. Zero occupancy seals a page permanently. Only the
/// last remover unlinks it, and an owning comparison protects its replacement.
pub(crate) struct Slot<'a> {
    head: &'a Head,
    page: Arc<Page>,
    key: usize,
    pub(crate) reserved: bool,
}

impl Slot<'_> {
    pub(crate) fn mark_young(&self) {
        // The address reservation excludes other writers to this young bit.
        // Directory exhaustion is impossible since each distinct key owns a page.
        let _ = reserve(&self.head.young, self.key, Bitmap::new);
    }

    fn clear_young(&self, cache: &mut YoungRetirement) -> Option<Arc<Bitmap>> {
        let base = self.key & !PAGE_MASK;
        let offset = self.key & PAGE_MASK;
        let bit = 1 << (offset % WORD);
        // A detached generation has no set bits and cannot reopen. A cache
        // miss, including an absent bit, must retry the current directory.
        if let Some((shard, key, page)) = &cache.0
            && *shard == self.head.shard
            && *key == base
            && let Some(last) = page.remove_bit(bit, offset)
        {
            return if last {
                cache.0.take().map(|(_, _, page)| page)
            } else {
                None
            };
        }
        let directory = self.head.young.load();
        let page = directory.get(base as u64)?;
        if page.remove_bit(bit, offset)? {
            return Some(page.clone());
        }
        cache.0 = Some((self.head.shard, base, page.clone()));
        None
    }

    pub(crate) fn install(&self, metadata: MetadataRef) {
        let slot = self.page.vacant_cell(self.key);
        if let Some(entry) = slot.get() {
            entry.store(Some(metadata));
        } else {
            // This address has one writer. No prior value can have readers
            // whose ownership needs to be settled by an ArcSwap store.
            assert!(
                slot.set(Cell::from(Some(metadata))).is_ok(),
                "duplicate address initializer"
            );
        }
        self.head.indexed.fetch_add(1, Relaxed);
    }

    fn remove(&self) -> (Option<MetadataRef>, Option<Sealed>) {
        self.remove_cached(&mut YoungRetirement::default())
    }

    fn remove_cached(&self, cache: &mut YoungRetirement) -> (Option<MetadataRef>, Option<Sealed>) {
        let young = self.clear_young(cache);
        let removed = self.page.entry(self.key).and_then(|entry| entry.swap(None));
        let sealed = self.finish_remove(removed.is_some(), young);
        (removed, sealed)
    }

    fn finish_remove(&self, indexed: bool, young: Option<Arc<Bitmap>>) -> Option<Sealed> {
        if indexed {
            self.head.indexed.fetch_sub(1, Relaxed);
        }
        let last = self.page.occupied.remove(self.key & PAGE_MASK);
        (last || young.is_some()).then(|| Sealed {
            key: self.key,
            shard: self.head.shard,
            objects: last.then(|| self.page.clone()),
            young,
        })
    }
}

impl Drop for Slot<'_> {
    fn drop(&mut self) {
        if self.reserved
            && let (_, Some(sealed)) = self.remove()
        {
            sealed.unlink(self.head);
        }
    }
}

/// Only promotion under the live-operation shard installs an owner here.
#[derive(Debug, Default)]
pub(crate) struct MatureWords(BTreeMap<usize, Box<MatureWord>>);

#[derive(Debug)]
struct MatureWord {
    occupied: u64,
    entries: [Option<MetadataRef>; WORD],
}

#[expect(
    clippy::indexing_slicing,
    reason = "slot offset is reduced modulo WORD"
)]
impl MatureWords {
    fn insert(&mut self, key: usize, metadata: &MetadataRef) {
        let word = self.0.entry(key / WORD).or_insert_with(|| {
            Box::new(MatureWord {
                occupied: 0,
                entries: std::array::from_fn(|_| None),
            })
        });
        let offset = key % WORD;
        word.occupied |= 1 << offset;
        word.entries[offset] = Some(metadata.clone());
    }

    fn remove(&mut self, key: usize) {
        let Some(word) = self.0.get_mut(&(key / WORD)) else {
            return;
        };
        let offset = key % WORD;
        word.entries[offset] = None;
        word.occupied &= !(1 << offset);
        if word.occupied == 0 {
            self.0.remove(&(key / WORD));
        }
    }
}

pub(crate) struct Membership<'a> {
    mature: MutexGuard<'a, MatureWords>,
    slot: Option<Slot<'a>>,
    metadata: Option<MetadataRef>,
    address: usize,
}

pub(crate) enum SweepEntry<'a> {
    Survivors(usize),
    Object(Membership<'a>),
}

impl Membership<'_> {
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "match existing map lookup syntax"
    )]
    pub(crate) fn get(&self, address: &usize) -> Option<&MetadataRef> {
        self.metadata
            .as_ref()
            .filter(|m| *address == self.address && m.birth.visible())
    }

    pub(crate) fn promote(&mut self, cache: &mut YoungRetirement) -> Option<Sealed> {
        let slot = self.slot.as_ref()?;
        self.mature.insert(slot.key, self.metadata.as_ref()?);
        let page = slot.clear_young(cache)?;
        Some(Sealed {
            key: slot.key,
            shard: slot.head.shard,
            objects: None,
            young: Some(page),
        })
    }

    // The caller has observed LIVE under the guard. Other live operations on
    // this address are excluded, and a new constructor cannot claim its bit.
    pub(crate) fn retire(
        &mut self,
        cache: &mut YoungRetirement,
    ) -> (Option<MetadataRef>, Option<Sealed>) {
        if let Some(metadata) = self.metadata.take() {
            if metadata.age.load(Acquire) != 0
                && let Some(slot) = &self.slot
            {
                self.mature.remove(slot.key);
            }
            metadata.birth.retire();
        }
        self.slot
            .as_ref()
            .map_or((None, None), |slot| slot.remove_cached(cache))
    }

    #[cfg(test)]
    pub(crate) fn occupied(&self) -> bool {
        self.metadata.is_some()
    }
}

#[derive(Debug, Default)]
pub(crate) struct YoungRetirement(Option<(usize, usize, Arc<Bitmap>)>);

#[derive(Debug, Default)]
pub(crate) struct YoungCursor {
    position: Cursor,
    page: Option<(usize, usize, Arc<Bitmap>)>,
}

impl YoungCursor {
    pub(crate) fn retry_entry(&mut self) {
        self.position.retry_entry();
    }
}

#[derive(Debug)]
pub(crate) struct Objects {
    guards: Shards<MatureWords>,
    shard_bits: u32,
    pages: Box<[Head]>,
}

impl Deref for Objects {
    type Target = Shards<MatureWords>;
    fn deref(&self) -> &Self::Target {
        &self.guards
    }
}

#[expect(clippy::indexing_slicing, reason = "shard and page offsets are masked")]
impl Objects {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            guards: Shards::new(count),
            shard_bits: shard_count(count).ilog2(),
            pages: (0..shard_count(count))
                .map(|shard| Head {
                    shard,
                    ..Head::default()
                })
                .collect(),
        }
    }

    fn head(&self, address: usize) -> &Head {
        &self.pages[address_bucket(address) & (self.pages.len() - 1)]
    }

    pub(crate) fn reserve(&self, address: usize) -> Option<Slot<'_>> {
        let head = self.head(address);
        let key = packed_key(address, self.shard_bits);
        let page = reserve(&head.pages, key, Page::new)?;
        Some(Slot {
            head,
            page,
            key,
            reserved: true,
        })
    }

    /// Mark traversal needs only the visible descriptor. Keep the same shard
    /// ordering as sweep, without acquiring a second owner of its address page.
    pub(crate) fn live(&self, address: usize) -> Option<MetadataRef> {
        let _guard = self.guards.lock(address);
        self.lookup(address)
    }

    /// An owning snapshot of the exact cell. It grants no graph or payload access.
    pub(crate) fn lookup(&self, address: usize) -> Option<MetadataRef> {
        let key = packed_key(address, self.shard_bits);
        let pages = self.head(address).pages.load();
        let page = pages.get((key & !PAGE_MASK) as u64)?;
        page.entry(key)?.load_full().filter(|m| m.birth.visible())
    }

    fn membership<'a>(
        &'a self,
        address: usize,
        guard: MutexGuard<'a, MatureWords>,
    ) -> Membership<'a> {
        let head = self.head(address);
        let key = packed_key(address, self.shard_bits);
        let pages = head.pages.load();
        let slot = pages.get((key & !PAGE_MASK) as u64).map(|page| Slot {
            head,
            page: page.clone(),
            key,
            reserved: false,
        });
        let metadata = slot.as_ref().and_then(|s| s.page.entry(key)?.load_full());
        Membership {
            mature: guard,
            slot,
            metadata,
            address,
        }
    }

    pub(crate) fn lock(&self, address: usize) -> Membership<'_> {
        self.membership(address, self.guards.lock(address))
    }

    pub(crate) fn try_lock(&self, address: usize) -> Option<Membership<'_>> {
        Some(self.membership(address, self.guards.try_lock(address)?))
    }

    pub(crate) fn scan<R>(
        &self,
        cursor: &mut Cursor,
        read: impl FnOnce(&MetadataRef) -> R,
    ) -> Scan<R> {
        self.scan_page(
            cursor,
            || false,
            |metadata, _, _, _, guard| {
                let result = read(metadata);
                drop(guard);
                result
            },
        )
    }

    // All reads in a batch use the same live-operation guard. New births
    // behind the cursor have the usual per-domain publication obligations.
    fn scan_page<'a, R>(
        &'a self,
        cursor: &mut Cursor,
        mature_covered: impl FnOnce() -> bool,
        read: impl FnOnce(
            &MetadataRef,
            &Arc<Page>,
            usize,
            &mut Cursor,
            MutexGuard<'a, MatureWords>,
        ) -> R,
    ) -> Scan<R> {
        if !cursor.begin(self.pages.len()) {
            return Scan::Done;
        }
        let Some(guard) = self.guards.try_lock_bucket(cursor.shard) else {
            cursor.rotate();
            return Scan::Busy;
        };
        let head = &self.pages[cursor.shard];
        // A constructor installs nursery membership before publishing its cell.
        // This shard excludes both a local promotion and a live removal between
        // the certificate check and selection. Both sources use the same packed
        // address cursor, so invalidation resumes at the first unvisited entry.
        let next = if mature_covered() {
            match Self::next(&head.young, cursor, None, || {}) {
                Scan::Entry(key, _) => {
                    let pages = head.pages.load();
                    let Some(page) = pages.get((key & !PAGE_MASK) as u64) else {
                        return Scan::Advance;
                    };
                    Scan::Entry(key, page.clone())
                }
                Scan::Advance => Scan::Advance,
                Scan::Busy => Scan::Busy,
                Scan::Done => Scan::Done,
            }
        } else {
            Self::next(&head.pages, cursor, None, || {})
        };
        match next {
            Scan::Entry(key, page) => {
                let entry = page.entry(key).map(Cell::load);
                if let Some(metadata) = entry
                    .as_deref()
                    .and_then(Option::as_ref)
                    .filter(|m| m.birth.visible())
                {
                    Scan::Entry(
                        unpack_key(key, cursor.shard, self.shard_bits),
                        read(metadata, &page, key, cursor, guard),
                    )
                } else {
                    Scan::Advance
                }
            }
            Scan::Advance => Scan::Advance,
            Scan::Busy => Scan::Busy,
            Scan::Done => Scan::Done,
        }
    }

    /// A mature survivor already marked in this exact epoch has no remaining
    /// age, promotion or retirement work. Stop before the first other entry
    /// so the normal sweep can revalidate and perform its side effects.
    pub(crate) fn scan_sweep(
        &self,
        cursor: &mut Cursor,
        domain: usize,
        epoch: u64,
        promotion_age: usize,
        limit: usize,
        coverage_current: impl FnOnce() -> bool,
    ) -> Scan<SweepEntry<'_>> {
        let survivor =
            |m: &MetadataRef| m.age.load(Acquire) == promotion_age && m.marked(domain, epoch);
        self.scan_page(
            cursor,
            coverage_current,
            |metadata, page, key, cursor, guard| {
                if !survivor(metadata) {
                    return SweepEntry::Object(Membership {
                        mature: guard,
                        metadata: Some(metadata.clone()),
                        address: unpack_key(key, cursor.shard, self.shard_bits),
                        slot: Some(Slot {
                            head: &self.pages[cursor.shard],
                            page: page.clone(),
                            key,
                            reserved: false,
                        }),
                    });
                }
                // Promotion and removal of every cached slot use this shard.
                let base = key & !PAGE_MASK;
                let mut start = (key & PAGE_MASK) + 1;
                let mut count = 1;
                'words: while count < limit {
                    let Some((word, mut bits)) = page.occupied.next_word(start) else {
                        break;
                    };
                    let cached = guard.0.get(&((base | word) / WORD));
                    while bits != 0 {
                        let bit = bits.trailing_zeros() as usize;
                        let next = word + bit;
                        let survives = if let Some(m) = cached.and_then(|w| w.entries[bit].as_ref())
                        {
                            m.marked(domain, epoch)
                        } else {
                            page.entry(next).is_some_and(|entry| {
                                entry
                                    .load()
                                    .as_ref()
                                    .is_some_and(|m| m.birth.is_live() && survivor(m))
                            })
                        };
                        if !survives {
                            break 'words;
                        }
                        cursor.selected(base | next);
                        count += 1;
                        if count == limit {
                            break 'words;
                        }
                        bits &= bits - 1;
                    }
                    start = word + WORD;
                }
                SweepEntry::Survivors(count)
            },
        )
    }

    /// Reuse the guarded address page for a consecutive white entry. Leave
    /// the cursor unchanged on a miss so normal scanning handles boundaries,
    /// new reservations and survivors with the usual rules.
    pub(crate) fn next_white(
        &self,
        member: &mut Membership<'_>,
        cursor: &mut Cursor,
        domain: usize,
        epoch: u64,
    ) -> Option<usize> {
        let slot = member.slot.as_mut()?;
        let offset = slot.page.occupied.next((slot.key & PAGE_MASK) + 1)?;
        let metadata = slot.page.entry(offset)?.load_full()?;
        if !metadata.birth.is_live() || metadata.marked(domain, epoch) {
            return None;
        }
        slot.key = (slot.key & !PAGE_MASK) | offset;
        cursor.selected(slot.key);
        let address = unpack_key(slot.key, cursor.shard, self.shard_bits);
        member.address = address;
        member.metadata = Some(metadata);
        Some(address)
    }

    /// A proven unaged white can transfer its registry owner directly to
    /// clearing. Keep the ordinary cursor and retry without effects on a miss.
    #[cfg(not(feature = "experimental-identity-index"))]
    pub(crate) fn take_white_word(
        &self,
        member: &mut Membership<'_>,
        cursor: &mut Cursor,
        epoch: u64,
        output: &mut [Option<MetadataRef>],
        mut revoke: impl FnMut(usize, &MetadataRef) -> bool,
    ) -> (usize, Option<Sealed>) {
        let Some(slot) = member.slot.as_mut() else {
            return (0, None);
        };
        let Some((word_base, mut candidates)) =
            slot.page.occupied.next_word((slot.key & PAGE_MASK) + 1)
        else {
            return (0, None);
        };
        let base = slot.key & !PAGE_MASK;
        let mut mask = 0;
        let mut count = 0;
        for destination in output {
            if candidates == 0 {
                break;
            }
            let offset = word_base + candidates.trailing_zeros() as usize;
            let key = base | offset;
            let address = unpack_key(key, cursor.shard, self.shard_bits);
            let Some(entry) = slot.page.entry(offset) else {
                break;
            };
            let Some(removed) = entry.take_if(|m| {
                if !m.birth.is_live()
                    || m.age.load(Acquire) != 0
                    || m.marked(1, epoch)
                    || !revoke(address, m)
                {
                    return false;
                }
                m.birth.retire();
                true
            }) else {
                break;
            };
            *destination = Some(removed);
            mask |= candidates.isolate_lowest_one();
            candidates &= candidates - 1;
            slot.key = key;
            member.address = address;
            cursor.selected(key);
            count += 1;
        }
        if count == 0 {
            return (0, None);
        }
        // All selected births have been revoked. Keep their object reservations
        // until the nursery update is complete, so reuse cannot lose a new bit.
        let word = word_base / WORD;
        let directory = slot.head.young.load();
        let young = directory
            .get(base as u64)
            .filter(|page| page.remove_mask(word, mask))
            .cloned();
        slot.head.indexed.fetch_sub(count, Relaxed);
        let last = slot.page.occupied.remove_mask(word, mask);
        let sealed = (last || young.is_some()).then(|| Sealed {
            key: slot.key,
            shard: slot.head.shard,
            objects: last.then(|| slot.page.clone()),
            young,
        });
        (count, sealed)
    }

    pub(crate) fn scan_young(&self, cursor: &mut YoungCursor) -> Scan<()> {
        self.scan_young_with(cursor, || {})
    }

    pub(crate) fn scan_young_with(
        &self,
        young: &mut YoungCursor,
        observed: impl FnOnce(),
    ) -> Scan<()> {
        if !young.position.begin(self.pages.len()) {
            return Scan::Done;
        }
        match Self::next(
            &self.pages[young.position.shard].young,
            &mut young.position,
            Some(&mut young.page),
            observed,
        ) {
            Scan::Entry(key, _) => {
                Scan::Entry(unpack_key(key, young.position.shard, self.shard_bits), ())
            }
            Scan::Advance => Scan::Advance,
            Scan::Busy => Scan::Busy,
            Scan::Done => Scan::Done,
        }
    }

    fn next<T: Occupancy>(
        directory: &Pages<T>,
        cursor: &mut Cursor,
        cache: Option<&mut Option<(usize, usize, Arc<T>)>>,
        observed: impl FnOnce(),
    ) -> Scan<Arc<T>> {
        let Some(start) = cursor.addresses[cursor.shard].checked_add(1) else {
            return cursor.exhausted();
        };
        let base = start & !PAGE_MASK;
        if let Some((shard, key, page)) = cache.as_deref().and_then(Option::as_ref)
            && *shard == cursor.shard
            && *key == base
            && let Some(offset) = page.bitmap().next(start & PAGE_MASK)
        {
            let key = base | offset;
            cursor.selected(key);
            return Scan::Entry(key, page.clone());
        }
        let pages = directory.load();
        observed();
        let selected = pages.get(base as u64).and_then(|p| {
            p.bitmap()
                .next(start & PAGE_MASK)
                .map(|offset| (base | offset, p))
        });
        let (key, page) = if let Some(entry) = selected {
            entry
        } else {
            let Some((base, page)) = pages.next_after(Some(base as u64)) else {
                return cursor.exhausted();
            };
            #[expect(
                clippy::cast_possible_truncation,
                reason = "all page keys originate from usize addresses"
            )]
            let base = base as usize;
            let Some(offset) = page.bitmap().next(0) else {
                cursor.addresses[cursor.shard] = base | PAGE_MASK;
                return Scan::Advance;
            };
            (base | offset, page)
        };
        cursor.selected(key);
        if let Some(cache) = cache {
            *cache = Some((cursor.shard, key & !PAGE_MASK, page.clone()));
        }
        Scan::Entry(key, page.clone())
    }

    pub(crate) fn len(&self) -> usize {
        self.pages
            .iter()
            .map(|head| head.indexed.load(Relaxed))
            .sum()
    }

    pub(crate) fn unlink(&self, sealed: &Sealed) {
        sealed.unlink(&self.pages[sealed.shard]);
    }

    pub(crate) fn for_each(&self, mut visit: impl FnMut(&MetadataRef)) {
        for (index, head) in self.pages.iter().enumerate() {
            let _guard = self.guards.lock_bucket(index);
            let pages = head.pages.load();
            let mut after = None;
            while let Some((key, page)) = pages.next_after(after) {
                let mut start = 0;
                while let Some(offset) = page.occupied.next(start) {
                    if let Some(entry) = page.entry(offset).and_then(Cell::load_full) {
                        visit(&entry);
                    }
                    start = offset + 1;
                }
                after = Some(key);
            }
        }
    }

    /// Called only from the collector destructor, with no outstanding publishers.
    pub(crate) fn drain(&self) -> Vec<MetadataRef> {
        let mut entries = Vec::new();
        for head in &self.pages {
            self.guards.lock_bucket(head.shard).0.clear();
            let pages = head.pages.swap(Arc::default());
            head.young.store(Arc::default());
            head.indexed.store(0, Relaxed);
            let mut after = None;
            while let Some((key, page)) = pages.next_after(after) {
                for chunk in page.chunks.iter().filter_map(OnceBox::get) {
                    entries.extend(chunk.iter().filter_map(|entry| entry.get()?.swap(None)));
                }
                after = Some(key);
            }
        }
        entries
    }

    #[cfg(test)]
    pub(crate) fn contains_young(&self, address: usize) -> bool {
        let key = packed_key(address, self.shard_bits);
        self.head(address)
            .young
            .load()
            .get((key & !PAGE_MASK) as u64)
            .is_some_and(|p| p.contains(key))
    }

    // Fixtures can construct partial write sets. Keep their occupancy bit as
    // long as either synthetic index is present, exactly like a reservation.
    #[cfg(test)]
    fn edit_for_test(&self, address: usize, edit: impl FnOnce(&Slot<'_>)) {
        let mut point = self.lock(address);
        point.mature.remove(packed_key(address, self.shard_bits));
        let mut slot = self.reserve(address).or(point.slot).expect("address slot");
        slot.reserved = false;
        edit(&slot);
        if !self.contains_young(address)
            && slot
                .page
                .entry(slot.key)
                .and_then(Cell::load_full)
                .is_none()
        {
            slot.reserved = true;
        }
    }

    #[cfg(test)]
    pub(crate) fn set_young_for_test(&self, address: usize, young: bool) {
        self.edit_for_test(address, |slot| {
            if young {
                slot.mark_young();
            } else if let Some(page) = slot.clear_young(&mut YoungRetirement::default()) {
                unlink(&slot.head.young, (slot.key & !PAGE_MASK) as u64, &page);
            }
        });
    }

    #[cfg(test)]
    pub(crate) fn replace_for_test(
        &self,
        address: usize,
        entry: Option<MetadataRef>,
    ) -> Option<MetadataRef> {
        let mut old = None;
        self.edit_for_test(address, |s| {
            let present = entry.is_some();
            old = s
                .page
                .vacant_cell(s.key)
                .get_or_init(|| Cell::from(None))
                .swap(entry);
            match (old.is_some(), present) {
                (false, true) => {
                    s.head.indexed.fetch_add(1, Relaxed);
                }
                (true, false) => {
                    s.head.indexed.fetch_sub(1, Relaxed);
                }
                _ => {}
            }
        });
        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn update(objects: &Objects, address: usize, present: bool) {
        objects.set_young_for_test(address, present);
    }

    #[test]
    fn numeric_scan_keys_round_trip_every_alignment() {
        let span = PAGE_SIZE * 8;
        for base in [0, span, usize::MAX & !(span - 1)] {
            for offset in 0..span {
                let address = base | offset;
                assert_eq!(address_of(position(address)), address);
                assert_eq!(position(address_of(address)), address);
            }
        }
    }

    #[test]
    fn packed_keys_preserve_exact_addresses_and_the_scan_sentinel() {
        let span = PAGE_SIZE * 8;
        for shards in [1_usize, 2, 4, 8, 16] {
            let bits = shards.ilog2();
            for base in [0, span, 1 << 20, usize::MAX & !(span - 1)] {
                for offset in 0..span {
                    let address = base | offset;
                    let shard = address_bucket(address) & (shards - 1);
                    let key = packed_key(address, bits);
                    assert_eq!(unpack_key(key, shard, bits), address);
                    assert!(key != 0 || address == 0);
                }
            }
        }
    }

    #[test]
    fn sweep_batches_stop_at_budget_young_and_white_entries() {
        use crate::{GarbageCollector, Gc};
        let gc = GarbageCollector::new();
        let kept = Gc::new(&gc, 1_u64);
        let young = Gc::new(&gc, 2_u64);
        let white = Gc::new(&gc, 3_u64);
        let meta = |root: &crate::GcRootGuard<u64>| {
            gc.metadata_for_address(root.as_ptr().addr())
                .expect("metadata")
        };
        let (kept, young, white) = (meta(&kept), meta(&young), meta(&white));
        kept.age.store(2, Relaxed);
        white.age.store(2, Relaxed);
        kept.mark_black(1, 4);
        young.mark_black(1, 4);
        let objects = Objects::new(1);
        for (address, m) in [
            (8, &kept),
            (16, &kept),
            (24, &kept),
            (32, &young),
            (40, &white),
            (48, &kept),
        ] {
            objects.replace_for_test(address, Some(m.clone()));
        }
        let mut cursor = Cursor::default();
        for (address, count) in [(8, 2), (24, 1), (32, 0), (40, 0), (48, 1)] {
            let Scan::Entry(actual, entry) = objects.scan_sweep(&mut cursor, 1, 4, 2, 2, || false)
            else {
                panic!("expected sweep entry");
            };
            assert_eq!(actual, address);
            match entry {
                SweepEntry::Survivors(n) => assert_eq!(n, count),
                SweepEntry::Object(member) => {
                    assert_eq!(count, 0);
                    assert!(member.get(&address).is_some());
                }
            }
        }
        assert!(matches!(
            objects.scan_sweep(&mut cursor, 1, 4, 2, 2, || false),
            Scan::Done
        ));
    }

    #[test]
    fn sparse_bitmap_matches_live_offsets_after_word_reuse() {
        let page = Bitmap::new(0);
        let mut occupied = BTreeSet::from([0]);
        for offset in [1, WORD - 1, WORD, 8 * WORD + 1, PAGE_SIZE - 1] {
            assert!(page.pin() && page.set(offset));
            occupied.insert(offset);
        }
        for offset in [0, WORD, 8 * WORD + 1] {
            assert!(!page.remove(offset));
            occupied.remove(&offset);
        }
        // The middle word has an existing summary bit but a new live slot.
        assert!(page.pin() && page.set(8 * WORD + 2));
        occupied.insert(8 * WORD + 2);
        for start in 0..=PAGE_SIZE {
            assert_eq!(page.next(start), occupied.range(start..).next().copied());
        }
    }

    #[test]
    fn a_stalled_page_claim_cannot_lose_a_peer_in_another_word() {
        let page = Bitmap::new(1);
        assert!(page.pin()); // A publisher has not selected its slot yet.
        assert!(!page.remove(1));
        assert_eq!(page.next(0), None);
        assert!(page.pin());
        assert!(page.set(WORD + 3));
        assert!(!page.unpin()); // First claimant abandons its pin.
        assert_eq!(page.next(0), Some(WORD + 3));
        assert!(page.remove(WORD + 3));
        assert!(!page.pin());
        assert_eq!(page.next(0), None);
    }

    #[test]
    fn late_unlink_preserves_a_new_occupant_at_the_identical_address() {
        let objects = Objects::new(1);
        let mut original = objects.reserve(8).expect("exact address");
        original.mark_young();
        original.reserved = false;
        let (_, sealed) = original.remove();
        let replacement = objects.reserve(8).expect("sealed page replaced");
        replacement.mark_young();
        objects.unlink(&sealed.expect("last occupant"));
        assert!(objects.contains_young(8));
        assert!(objects.reserve(8).is_none());
        drop(replacement);
        assert!(
            objects
                .pages
                .iter()
                .all(|head| head.pages.load().len() == 0)
        );
    }

    #[test]
    fn stale_scan_cannot_republish_a_retired_bitmap_page() {
        let objects = Objects::new(1);
        update(&objects, 8, true);
        let mut cursor = YoungCursor::default();
        // One reader owns the old directory. Retire its last bit and recreate
        // the very same page key before allowing it to inspect its bitmap.
        let old = objects.scan_young_with(&mut cursor, || {
            update(&objects, 8, false);
            update(&objects, 16, true);
        });
        assert!(matches!(old, Scan::Done | Scan::Advance));
        assert!(!objects.contains_young(8));
        assert!(objects.contains_young(16));
        let mut fresh = YoungCursor::default();
        assert!(matches!(
            objects.scan_young(&mut fresh),
            Scan::Entry(16, ())
        ));
        assert!(matches!(objects.scan_young(&mut fresh), Scan::Done));
        update(&objects, 16, false);
        assert!(
            objects
                .pages
                .iter()
                .all(|slot| slot.pages.load().len() == 0)
        );
    }

    #[test]
    fn stale_retirement_cursor_clears_the_current_generation_only() {
        let objects = Objects::new(1);
        let first = objects.reserve(8).expect("first address");
        let second = objects.reserve(16).expect("same page");
        first.mark_young();
        second.mark_young();
        let mut stale = YoungRetirement::default();
        assert!(first.clear_young(&mut stale).is_none());
        assert!(stale.0.is_some());
        assert!(objects.contains_young(16));
        let sealed = second
            .clear_young(&mut YoungRetirement::default())
            .expect("last young member");
        first.mark_young();
        second.mark_young();
        unlink(&first.head.young, 0, &sealed);
        assert!(first.clear_young(&mut stale).is_none());
        assert!(!objects.contains_young(8));
        assert!(objects.contains_young(16));
        let last = second.clear_young(&mut stale).expect("replacement seals");
        assert!(!Arc::ptr_eq(&sealed, &last));
        unlink(&first.head.young, 0, &last);
        assert!(matches!(
            objects.scan_young(&mut YoungCursor::default()),
            Scan::Done
        ));
        assert!(objects.reserve(8).is_none());
    }

    #[test]
    fn a_cached_nursery_page_refreshes_after_unlink_and_cursor_retry() {
        let objects = Objects::new(1);
        let mut cursor = YoungCursor::default();
        update(&objects, 8, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(8, ())
        ));
        update(&objects, 8, false);
        update(&objects, 16, true);
        cursor.retry_entry();
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(16, ())
        ));
        update(&objects, 24, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(24, ())
        ));
        assert!(matches!(objects.scan_young(&mut cursor), Scan::Done));
    }

    #[cfg(all(not(target_family = "wasm"), not(miri)))]
    #[test]
    fn a_peer_observes_replacement_page_and_another_slot_through_publication() {
        use std::thread;
        use std::time::{Duration, Instant};

        let objects = Objects::new(1);
        let mut cursor = YoungCursor::default();
        update(&objects, 8, true);
        assert!(matches!(
            objects.scan_young(&mut cursor),
            Scan::Entry(8, ())
        ));
        let slot = objects.pages.first().expect("one shard");
        let original = slot.young.load().get(0).expect("first page").clone();
        thread::scope(|scope| {
            scope.spawn(|| {
                update(&objects, 8, false);
                update(&objects, 16, true);
                // Same page, different slot. The second insertion reuses the root.
                update(&objects, 24, true);
            });
            let deadline = Instant::now() + Duration::from_secs(5);
            // The reader never joins the writer or receives a ready signal.
            // Observe the new directory and the second slot directly.
            let page = loop {
                if let Some(p) = slot.young.load().get(0)
                    && !Arc::ptr_eq(&original, p)
                {
                    break p.clone();
                }
                assert!(Instant::now() < deadline, "replacement was not published");
                thread::yield_now();
            };
            assert!(page.contains(position(16)));
            while !page.contains(position(24)) {
                assert!(Instant::now() < deadline, "slot was not published");
                thread::yield_now();
            }
            cursor.retry_entry();
            assert!(matches!(
                objects.scan_young(&mut cursor),
                Scan::Entry(16, ())
            ));
            assert!(matches!(
                objects.scan_young(&mut cursor),
                Scan::Entry(24, ())
            ));
            assert!(matches!(objects.scan_young(&mut cursor), Scan::Done));
        });
    }

    #[test]
    fn nursery_cursor_preserves_word_page_and_address_space_boundaries() {
        for shards in [1, 2, 16] {
            let objects = Objects::new(shards);
            let keys = [
                1,
                2,
                7,
                8,
                9,
                PAGE_SIZE - 1,
                PAGE_SIZE,
                PAGE_SIZE + 64,
                2 * PAGE_SIZE - 1,
                4 * PAGE_SIZE + 1,
                usize::MAX - 1,
                usize::MAX,
            ];
            for key in keys {
                assert!(!objects.contains_young(key));
                update(&objects, key, true);
                assert!(objects.contains_young(key));
            }
            let mut cursor = YoungCursor::default();
            let mut seen = BTreeSet::new();
            loop {
                match objects.scan_young(&mut cursor) {
                    Scan::Entry(key, ()) => {
                        assert!(seen.insert(key));
                        update(&objects, key, false);
                    }
                    Scan::Advance => {}
                    Scan::Done => break,
                    Scan::Busy => panic!("nursery directory required an index lock"),
                }
            }
            assert_eq!(seen, keys.into_iter().collect());
            assert!(
                objects
                    .pages
                    .iter()
                    .all(|slot| slot.pages.load().len() == 0)
            );
        }
    }
}
