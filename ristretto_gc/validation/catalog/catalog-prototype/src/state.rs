//! Pure snapshot transitions; the parent supplies owning atomic head operations.
//! No production MarkWord, index, queue, payload or root operation is replayed.
use super::{Arc, AtomicBool, Link, Ordering};
pub use super::tree::{Record, Tree};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain { pub epoch: u64, pub floor: u8 }
pub struct Head {
    pub revision: u64,
    pub domains: [Domain; 2],
    pub entries: Tree,
    installed: AtomicBool,
}
pub struct Proposal {
    expected: Arc<Head>,
    proposed: Arc<Head>,
}
#[derive(Debug, Eq, PartialEq)]
pub enum Outcome { Committed, Rejected }
pub struct Catalog { head: Link }
impl Catalog {
    pub fn new() -> Self {
        let head = Arc::new(Head { revision: 1, domains: [Domain { epoch: 1, floor: 0 }; 2], entries: Tree::default(), installed: AtomicBool::new(true) });
        Self { head: Link::new(head) }
    }
    pub fn snapshot(&self) -> Arc<Head> { self.head.load() }
    /// No mutable effect occurs while preparing an alternative.
    fn prepare(snapshot: Arc<Head>, entries: Tree, domains: [Domain; 2]) -> Option<Proposal> {
        let revision = snapshot.revision.checked_add(1)?;
        Some(Proposal { expected: snapshot, proposed: Arc::new(Head { revision, domains, entries, installed: AtomicBool::new(false) }) })
    }
    pub fn insert(snapshot: Arc<Head>, identity: u64, address: usize) -> Option<Proposal> {
        if identity == 0 || address == 0 || snapshot.entries.get(identity).is_some() { return None; }
        let work = snapshot.revision.checked_add(1)?;
        let entries = snapshot.entries.put(Record { identity, address, work: [work; 2] });
        let domains = snapshot.domains;
        Self::prepare(snapshot, entries, domains)
    }
    pub fn acknowledge(snapshot: Arc<Head>, identity: u64, domain: usize, ticket: u64) -> Option<Proposal> {
        let mut record = snapshot.entries.get(identity)?;
        if ticket == 0 || *record.work.get(domain)? != ticket { return None; }
        record.work[domain] = 0;
        let entries = snapshot.entries.put(record);
        let domains = snapshot.domains;
        Self::prepare(snapshot, entries, domains)
    }
    pub fn require(snapshot: Arc<Head>, identity: u64, domain: usize) -> Option<Proposal> {
        let mut record = snapshot.entries.get(identity)?;
        *record.work.get_mut(domain)? = snapshot.revision.checked_add(1)?;
        let entries = snapshot.entries.put(record);
        let domains = snapshot.domains;
        Self::prepare(snapshot, entries, domains)
    }
    /// Test transition: one floor step, or a fresh epoch after terminal floor 3.
    pub fn advance(snapshot: Arc<Head>, domain: usize) -> Option<Proposal> {
        if domain >= 2 || snapshot.entries.pending_count(domain) != 0 { return None; }
        let mut domains = snapshot.domains;
        let old = domains[domain];
        domains[domain] = if old.floor == 3 { Domain { epoch: old.epoch.checked_add(1)?, floor: 0 } }
            else { Domain { epoch: old.epoch, floor: old.floor + 1 } };
        let entries = snapshot.entries.clone();
        Self::prepare(snapshot, entries, domains)
    }
    pub fn retire(snapshot: Arc<Head>, identity: u64, address: usize) -> Option<Proposal> {
        let record = snapshot.entries.get(identity)?;
        if record.address != address || record.work != [0, 0] { return None; }
        let entries = snapshot.entries.remove(identity);
        let domains = snapshot.domains;
        Self::prepare(snapshot, entries, domains)
    }
    /// Every replaced head is certified BEFORE the head CAS. A stopped winning
    /// publisher can recover its result even after its entry is later retired.
    pub fn commit(&self, proposal: &Proposal) -> Outcome {
        let current = self.head.load();
        current.installed.store(true, Ordering::Release);
        if Arc::ptr_eq(&current, &proposal.expected) && self.head.compare(&proposal.expected, proposal.proposed.clone()) {
            proposal.proposed.installed.store(true, Ordering::Release);
            return Outcome::Committed;
        }
        // A competing helper may have installed this exact proposal and stopped
        // before certifying it. Recover after the failed CAS, not from the old
        // pre-CAS snapshot. If it was replaced again, that replacement already
        // certified it; otherwise this load observes and certifies it directly.
        self.head.load().installed.store(true, Ordering::Release);
        if proposal.proposed.installed.load(Ordering::Acquire) { Outcome::Committed } else { Outcome::Rejected }
    }
    #[cfg(test)]
    pub fn commit_stopped_after_cas(&self, proposal: &Proposal) -> bool {
        let current = self.head.load();
        current.installed.store(true, Ordering::Release);
        Arc::ptr_eq(&current, &proposal.expected) && self.head.compare(&proposal.expected, proposal.proposed.clone())
    }
}
