//! Numeric identity hints. A preparer claims its identity bit before taking
//! phase admission. A live slot owns that bit until its exact address is removed.
//! An empty bit mask seals the page permanently, even if a reader retains it.
use crate::publication_directory::Directory;
use arc_swap::ArcSwap;
use portable_atomic::AtomicU64;
use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::sync::{Arc, Weak};

const SLOTS: usize = 64;
type Pages = Directory<Arc<Page>>;
type Head = ArcSwap<Pages>;

thread_local! {
    // Weak backing only. TLS teardown and another collector using this thread
    // both fall back to the authoritative directory.
    static RECENT: RefCell<Option<(u64, Weak<Page>)>> = const { RefCell::new(None) };
}

#[derive(Debug)]
struct Page {
    owner: Arc<()>,
    occupied: AtomicU64,
    addresses: [AtomicUsize; SLOTS],
}

impl Page {
    fn new(offset: usize, owner: &Arc<()>) -> Self {
        Self {
            owner: owner.clone(),
            occupied: AtomicU64::new(1 << offset),
            addresses: [const { AtomicUsize::new(0) }; SLOTS],
        }
    }
    #[expect(clippy::indexing_slicing, reason = "locate masks the slot index")]
    fn reserve(&self, offset: usize) -> Result<bool, Option<usize>> {
        let bit = 1 << offset;
        match self.occupied.fetch_update(AcqRel, Acquire, |occupied| {
            (occupied != 0 && occupied & bit == 0).then_some(occupied | bit)
        }) {
            Ok(_) => Ok(true),
            Err(0) => Ok(false),
            Err(_) => {
                let address = self.addresses[offset].load(Acquire);
                Err((address != 0).then_some(address))
            }
        }
    }
    #[expect(clippy::indexing_slicing, reason = "locate masks the slot index")]
    fn clear(&self, offset: usize, address: usize) -> Option<bool> {
        self.addresses[offset]
            .compare_exchange(address, 0, AcqRel, Relaxed)
            .ok()?;
        Some(self.release(offset))
    }

    fn release(&self, offset: usize) -> bool {
        let bit = 1 << offset;
        self.occupied.fetch_and(!bit, AcqRel) == bit
    }
}

/// Sealed pages contain no occupied slots. This owner may run outside all GC
/// guards. Another preparer can replace its page without waiting for unlink.
pub(crate) struct Sealed {
    key: u64,
    page: Arc<Page>,
}

/// Eight independently replaceable numeric pages per ongoing sweep. A failed exact clear always retries in
/// the current directory, including after another domain sealed the old page.
#[derive(Default, Debug)]
pub(crate) struct RetireCursor([Option<(u64, Arc<Page>)>; 8]);

/// Reading an identity grants no access to its address or the managed payload.
#[derive(Debug)]
pub(crate) struct Index {
    owner: Arc<()>,
    heads: Box<[Head]>,
}

impl Index {
    pub(crate) fn new(shards: usize) -> Self {
        Self {
            owner: Arc::new(()),
            heads: (0..crate::index::shard_count(shards))
                .map(|_| Head::from_pointee(Pages::default()))
                .collect(),
        }
    }

    #[expect(
        clippy::indexing_slicing,
        reason = "a masked index selects a power-of-two shard"
    )]
    fn head(&self, key: u64) -> &Head {
        let shard = usize::from(key.to_le_bytes()[0]) & (self.heads.len() - 1);
        &self.heads[shard]
    }

    fn locate(&self, id: u64) -> (&Head, u64, usize) {
        let key = id / SLOTS as u64;
        (
            self.head(key),
            key,
            usize::from(id.to_le_bytes()[0]) % SLOTS,
        )
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "consume the cleanup task and release its page after the directory retry"
    )]
    pub(crate) fn unlink(&self, sealed: Sealed) {
        #[cfg(all(test, not(target_family = "wasm"), not(miri)))]
        tests::pause_unlink();
        let head = self.head(sealed.key);
        loop {
            let current = head.load();
            if !current
                .get(sealed.key)
                .is_some_and(|p| Arc::ptr_eq(p, &sealed.page))
            {
                return;
            }
            let next = Arc::new(current.remove(sealed.key).0);
            if Arc::ptr_eq(&head.compare_and_swap(&*current, next), &current) {
                return;
            }
        }
    }

    #[expect(
        clippy::expect_used,
        reason = "each directory entry already occupies more than one byte of address space"
    )]
    pub(crate) fn prepare(&self, id: u64) -> Result<Slot<'_>, Option<usize>> {
        let (head, key, offset) = self.locate(id);
        let cached = RECENT
            .try_with(|slot| {
                slot.borrow()
                    .as_ref()
                    .filter(|(previous, _)| *previous == key)
                    .and_then(|(_, page)| page.upgrade())
            })
            .ok()
            .flatten();
        if let Some(page) = cached
            && Arc::ptr_eq(&self.owner, &page.owner)
            && page.reserve(offset)?
        {
            return Ok(Slot {
                index: self,
                key,
                offset,
                page,
                reserved: true,
            });
        }
        let page = loop {
            let current = head.load();
            if let Some(page) = current.get(key)
                && page.reserve(offset)?
            {
                break page.clone();
            }
            let page = Arc::new(Page::new(offset, &self.owner));
            let (next, _) = current
                .insert(key, page.clone())
                .expect("identity page capacity");
            if Arc::ptr_eq(&head.compare_and_swap(&*current, Arc::new(next)), &current) {
                break page;
            }
        };
        let _ = RECENT.try_with(|slot| slot.replace(Some((key, Arc::downgrade(&page)))));
        Ok(Slot {
            index: self,
            key,
            offset,
            page,
            reserved: true,
        })
    }

    #[expect(clippy::indexing_slicing, reason = "locate masks the slot index")]
    pub(crate) fn get(&self, id: u64) -> Option<usize> {
        let (head, key, offset) = self.locate(id);
        let pages = head.load();
        let address = pages.get(key)?.addresses[offset].load(Acquire);
        (address != 0).then_some(address)
    }

    /// Close the numeric slot while registry removal is admitted. The returned
    /// task performs the structural unlink after release of collector guards.
    pub(crate) fn retire(
        &self,
        id: u64,
        address: usize,
        cursor: &mut RetireCursor,
    ) -> Option<Sealed> {
        if address == 0 {
            return None;
        }
        let (head, key, offset) = self.locate(id);
        let slot = cursor.0.get_mut(usize::from(key.to_le_bytes()[0]) % 8)?;
        if let Some((prior, page)) = slot.as_ref()
            && *prior == key
            && Arc::ptr_eq(&self.owner, &page.owner)
            && let Some(empty) = page.clear(offset, address)
        {
            return if empty {
                slot.take().map(|(key, page)| Sealed { key, page })
            } else {
                None
            };
        }
        let pages = head.load();
        let page = pages.get(key)?;
        if page.clear(offset, address)? {
            *slot = None;
            Some(Sealed {
                key,
                page: page.clone(),
            })
        } else {
            *slot = Some((key, page.clone()));
            None
        }
    }

    pub(crate) fn remove(&self, id: u64, address: usize) {
        if let Some(sealed) = self.retire(id, address, &mut RetireCursor::default()) {
            self.unlink(sealed);
        }
    }

    #[cfg(test)]
    pub(crate) fn test_set(&self, id: u64, address: Option<usize>) {
        if let Some(old) = self.get(id) {
            self.remove(id, old);
        }
        if let Some(address) = address {
            self.prepare(id)
                .expect("vacant fixture slot")
                .install(address);
        }
    }
}

pub(crate) struct Slot<'a> {
    index: &'a Index,
    key: u64,
    offset: usize,
    page: Arc<Page>,
    reserved: bool,
}

#[expect(clippy::indexing_slicing, reason = "locate masks the slot index")]
impl Slot<'_> {
    pub(crate) fn install(mut self, address: usize) {
        assert_ne!(address, 0, "identity requires a nonzero address");
        // No competing reservation can publish to this slot until exact removal.
        self.page.addresses[self.offset].store(address, Release);
        self.reserved = false;
    }
}
impl Drop for Slot<'_> {
    fn drop(&mut self) {
        if self.reserved && self.page.release(self.offset) {
            self.index.unlink(Sealed {
                key: self.key,
                page: self.page.clone(),
            });
        }
    }
}

#[cfg(test)]
#[path = "identity_pages_tests.rs"]
mod tests;
