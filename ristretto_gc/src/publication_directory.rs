//! Immutable exact-key directory for the publication candidate.
//!
//! A compressed binary radix path has at most 64 branches. Preparing a new
//! owning root never mutates an older root or holds a directory lock. A value
//! is an identity/storage owner, not implicit permission to access a payload.
//! Last-owner destruction can visit several obsolete paths; this module makes
//! no bounded destruction or allocator-latency claim.
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Directory<V> {
    root: Option<Arc<Node<V>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<V> {
    first: u64,
    last: u64,
    kind: Kind<V>,
}

#[derive(Debug)]
enum Kind<V> {
    Leaf(V),
    Branch {
        bit: u32,
        left: Arc<Node<V>>,
        right: Arc<Node<V>>,
    },
}

impl<V> Clone for Directory<V> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            len: self.len,
        }
    }
}

impl<V> Default for Directory<V> {
    fn default() -> Self {
        Self { root: None, len: 0 }
    }
}

fn split(left: u64, right: u64) -> Option<u32> {
    (left ^ right).checked_ilog2()
}

impl<V> Node<V> {
    fn leaf(key: u64, value: V) -> Arc<Self> {
        Arc::new(Self {
            first: key,
            last: key,
            kind: Kind::Leaf(value),
        })
    }

    fn branch(bit: u32, left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        debug_assert!(left.last < right.first);
        Arc::new(Self {
            first: left.first,
            last: right.last,
            kind: Kind::Branch { bit, left, right },
        })
    }

    fn join(bit: u32, old: &Arc<Self>, new: Arc<Self>) -> Arc<Self> {
        if new.first & (1_u64 << bit) == 0 {
            Self::branch(bit, new, old.clone())
        } else {
            Self::branch(bit, old.clone(), new)
        }
    }

    fn insert(old: &Arc<Self>, new: Arc<Self>) -> Arc<Self> {
        let different = split(old.first, new.first);
        match &old.kind {
            Kind::Leaf(_) => match different {
                Some(bit) => Self::join(bit, old, new),
                None => new,
            },
            Kind::Branch { bit, left, right } => {
                if let Some(different) = different
                    && different > *bit
                {
                    return Self::join(different, old, new);
                }
                if new.first & (1_u64 << bit) == 0 {
                    Self::branch(*bit, Self::insert(left, new), right.clone())
                } else {
                    Self::branch(*bit, left.clone(), Self::insert(right, new))
                }
            }
        }
    }

    fn remove(old: &Arc<Self>, key: u64) -> Option<Arc<Self>> {
        if key < old.first || key > old.last {
            return Some(old.clone());
        }
        match &old.kind {
            Kind::Leaf(_) => (key != old.first).then(|| old.clone()),
            Kind::Branch { bit, left, right } => {
                if key & (1_u64 << bit) == 0 {
                    Self::remove(left, key).map_or_else(
                        || Some(right.clone()),
                        |next| Some(Self::branch(*bit, next, right.clone())),
                    )
                } else {
                    Self::remove(right, key).map_or_else(
                        || Some(left.clone()),
                        |next| Some(Self::branch(*bit, left.clone(), next)),
                    )
                }
            }
        }
    }
}

impl<V> Directory<V> {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn get(&self, key: u64) -> Option<&V> {
        let mut node = self.root.as_deref()?;
        loop {
            if key < node.first || key > node.last {
                return None;
            }
            match &node.kind {
                Kind::Leaf(value) => return (node.first == key).then_some(value),
                Kind::Branch { bit, left, right } => {
                    node = if key & (1_u64 << bit) == 0 {
                        left
                    } else {
                        right
                    };
                }
            }
        }
    }

    /// The cursor is a copied numeric key. It owns no graph/storage permission.
    pub(crate) fn next_after(&self, after: Option<u64>) -> Option<(u64, &V)> {
        let mut node = self.root.as_deref()?;
        if after.is_some_and(|key| node.last <= key) {
            return None;
        }
        loop {
            match &node.kind {
                Kind::Leaf(value) => return Some((node.first, value)),
                Kind::Branch { left, right, .. } => {
                    node = if after.is_none_or(|key| left.last > key) {
                        left
                    } else {
                        right
                    };
                }
            }
        }
    }
}

impl<V: Clone> Directory<V> {
    /// Overflow rejects the replacement before constructing or publishing it.
    pub(crate) fn insert(&self, key: u64, value: V) -> Option<(Self, Option<V>)> {
        let prior = self.get(key).cloned();
        let len = if prior.is_some() {
            self.len
        } else {
            self.len.checked_add(1)?
        };
        let leaf = Node::leaf(key, value);
        let root = match &self.root {
            Some(root) => Node::insert(root, leaf),
            None => leaf,
        };
        Some((
            Self {
                root: Some(root),
                len,
            },
            prior,
        ))
    }

    pub(crate) fn remove(&self, key: u64) -> (Self, Option<V>) {
        let prior = self.get(key).cloned();
        if prior.is_none() {
            return (self.clone(), None);
        }
        let root = self.root.as_ref().and_then(|root| Node::remove(root, key));
        (
            Self {
                root,
                len: self.len - 1,
            },
            prior,
        )
    }
}

#[cfg(test)]
#[path = "publication_directory_tests.rs"]
mod tests;
