//! Immutable compressed radix tree: at most 64 branch decisions/path copies.
//! Dropping a unique large snapshot is still unbounded work; cleanup is not qualified.
use super::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Record {
    pub identity: u64,
    pub address: usize,
    /// Nonzero immutable work ticket. Membership and these obligations commit together.
    pub work: [u64; 2],
}

enum Kind {
    Leaf(Record),
    Branch { bit: u32, low: Arc<Node>, high: Arc<Node> },
}
struct Node {
    kind: Kind,
    minimum: u64,
    maximum: u64,
    count: usize,
    pending: [usize; 2],
}
impl Node {
    fn leaf(record: Record) -> Arc<Self> {
        Arc::new(Self { minimum: record.identity, maximum: record.identity, count: 1,
            pending: record.work.map(|ticket| usize::from(ticket != 0)), kind: Kind::Leaf(record) })
    }
    fn branch(bit: u32, low: Arc<Self>, high: Arc<Self>) -> Arc<Self> {
        assert!(low.maximum < high.minimum);
        Arc::new(Self { minimum: low.minimum, maximum: high.maximum,
            count: low.count.checked_add(high.count).expect("entry count"),
            pending: std::array::from_fn(|i| low.pending[i].checked_add(high.pending[i]).expect("pending count")),
            kind: Kind::Branch { bit, low, high } })
    }
    fn descend(&self, key: u64) -> &Record {
        match &self.kind {
            Kind::Leaf(record) => record,
            Kind::Branch { bit, low, high } => if key & (1 << bit) == 0 { low.descend(key) } else { high.descend(key) },
        }
    }
    fn replace(&self, record: Record) -> Arc<Self> {
        match &self.kind {
            Kind::Leaf(old) => { assert_eq!(old.identity, record.identity); Self::leaf(record) },
            Kind::Branch { bit, low, high } => if record.identity & (1 << bit) == 0 {
                Self::branch(*bit, low.replace(record), high.clone())
            } else { Self::branch(*bit, low.clone(), high.replace(record)) },
        }
    }
    fn insert(current: &Arc<Self>, record: Record, split: u32) -> Arc<Self> {
        if let Kind::Branch { bit, low, high } = &current.kind {
            if *bit > split {
                return if record.identity & (1 << bit) == 0 {
                    Self::branch(*bit, Self::insert(low, record, split), high.clone())
                } else { Self::branch(*bit, low.clone(), Self::insert(high, record, split)) };
            }
        }
        let leaf = Self::leaf(record);
        if record.identity & (1 << split) == 0 { Self::branch(split, leaf, current.clone()) }
        else { Self::branch(split, current.clone(), leaf) }
    }
    fn remove(current: &Arc<Self>, identity: u64) -> Option<Arc<Self>> {
        match &current.kind {
            Kind::Leaf(record) => (record.identity != identity).then(|| current.clone()),
            Kind::Branch { bit, low, high } => {
                if identity & (1 << bit) == 0 {
                    Some(Self::remove(low, identity).map_or_else(|| high.clone(), |low| Self::branch(*bit, low, high.clone())))
                } else {
                    Some(Self::remove(high, identity).map_or_else(|| low.clone(), |high| Self::branch(*bit, low.clone(), high)))
                }
            }
        }
    }
    fn next(&self, after: u64) -> Option<Record> {
        if self.maximum <= after { return None; }
        match &self.kind {
            Kind::Leaf(record) => Some(*record),
            Kind::Branch { low, high, .. } => low.next(after).or_else(|| high.next(after)),
        }
    }
    fn pending(&self, domain: usize, after: u64) -> Option<Record> {
        if self.pending[domain] == 0 || self.maximum <= after { return None; }
        match &self.kind {
            Kind::Leaf(record) => Some(*record),
            Kind::Branch { low, high, .. } => low.pending(domain, after).or_else(|| high.pending(domain, after)),
        }
    }
}

#[derive(Clone, Default)]
pub struct Tree(Option<Arc<Node>>);
impl Tree {
    pub fn get(&self, identity: u64) -> Option<Record> {
        let record = self.0.as_ref()?.descend(identity);
        (record.identity == identity).then_some(*record)
    }
    pub fn put(&self, record: Record) -> Self {
        assert_ne!(record.identity, 0);
        let Some(root) = &self.0 else { return Self(Some(Node::leaf(record))); };
        let old = root.descend(record.identity);
        if old.identity == record.identity { return Self(Some(root.replace(record))); }
        let split = 63 - (old.identity ^ record.identity).leading_zeros();
        Self(Some(Node::insert(root, record, split)))
    }
    pub fn remove(&self, identity: u64) -> Self { Self(self.0.as_ref().and_then(|node| Node::remove(node, identity))) }
    pub fn count(&self) -> usize { self.0.as_ref().map_or(0, |node| node.count) }
    pub fn pending_count(&self, domain: usize) -> usize { self.0.as_ref().map_or(0, |node| node.pending[domain]) }
    pub fn next(&self, after: u64) -> Option<Record> { self.0.as_ref()?.next(after) }
    pub fn next_pending(&self, domain: usize, after: u64) -> Option<Record> { self.0.as_ref()?.pending(domain, after) }
}
