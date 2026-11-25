// Exact multi-index images; the parent supplies linearizable owning head links.
use super::{Arc, AtomicBool, Directory, Key, Link, Observed, Ordering};

#[derive(Clone, Debug)]
struct Entry<V> {
    key: Key,
    value: V,
}

/// One immutable membership image, never a root or payload-access certificate.
#[derive(Clone, Debug)]
struct Image<V> {
    objects: Directory<Entry<V>>,
    identities: Directory<usize>,
    young: Directory<crate::ObjectId>,
}

impl<V> Default for Image<V> {
    fn default() -> Self {
        Self {
            objects: Directory::default(),
            identities: Directory::default(),
            young: Directory::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Error {
    Conflict(super::Conflict),
    Missing,
    Capacity,
    ForeignCollector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Status {
    Installed,
    Foreign,
    Stale,
    Retry,
}

struct Head<V> {
    image: Image<V>,
    // Certifies this exact head was selected, not payload/root/phase completion.
    installed: AtomicBool,
}

pub(crate) struct View<V>(Arc<Head<V>>);

impl<V> Clone for View<V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<V> View<V> {
    fn observed(&self, key: Key) -> Observed {
        Observed {
            object: self
                .0
                .image
                .objects
                .get(key.address as u64)
                .map(|entry| entry.key.id),
            identity: self
                .0
                .image
                .identities
                .get(key.id.to_i64().cast_unsigned())
                .copied(),
            young: self.0.image.young.get(key.address as u64).is_some(),
        }
    }

    /// The inverse mapping must agree in this same image. This returns identity
    /// storage only; callers still need current phase/root access authorization.
    pub(crate) fn get(&self, key: Key) -> Option<&V> {
        let entry = self.0.image.objects.get(key.address as u64)?;
        (entry.key == key
            && self.0.image.identities.get(key.id.to_i64().cast_unsigned()) == Some(&key.address))
        .then_some(&entry.value)
    }

    pub(crate) fn by_identity(&self, id: crate::ObjectId) -> Option<(Key, &V)> {
        let address = *self.0.image.identities.get(id.to_i64().cast_unsigned())?;
        let key = Key { address, id };
        Some((key, self.get(key)?))
    }

    pub(crate) fn is_young(&self, key: Key) -> bool {
        self.get(key).is_some() && self.0.image.young.get(key.address as u64) == Some(&key.id)
    }

    /// A copied cursor owns no payload permission. Insertion behind the cursor
    /// still needs the collector's coupled traversal barrier during activation.
    pub(crate) fn next(&self, after: Option<usize>) -> Option<(Key, &V)> {
        let (_, entry) = self
            .0
            .image
            .objects
            .next_after(after.map(|address| address as u64))?;
        Some((entry.key, self.get(entry.key)?))
    }

    pub(crate) fn len(&self) -> usize {
        self.0.image.objects.len()
    }
}

/// The expected and proposed allocations stay owned, excluding address ABA.
/// Dropping a never-installed proposal has no membership effect. Dropping an
/// installed proposal is not cancellation/removal: that needs a fresh exact-key
/// transition and, when activated, the collector's retirement decision.
pub(crate) struct Prepared<V> {
    registry: Arc<()>,
    expected: Arc<Head<V>>,
    proposed: Arc<Head<V>>,
}

pub(crate) struct Registry<V> {
    identity: Arc<()>,
    head: Link<Head<V>>,
}

impl<V: Clone> Registry<V> {
    pub(crate) fn new() -> Self {
        Self {
            identity: Arc::new(()),
            head: Link::new(Arc::new(Head {
                image: Image::default(),
                installed: AtomicBool::new(true),
            })),
        }
    }

    pub(crate) fn view(&self) -> View<V> {
        View(self.head.load())
    }

    pub(crate) fn insert(&self, key: Key, value: V) -> Result<Prepared<V>, Error> {
        let view = self.view();
        key.vacant(view.observed(key)).map_err(Error::Conflict)?;
        let mut image = view.0.image.clone();
        image.objects = image
            .objects
            .insert(key.address as u64, Entry { key, value })
            .ok_or(Error::Capacity)?
            .0;
        image.identities = image
            .identities
            .insert(key.id.to_i64().cast_unsigned(), key.address)
            .ok_or(Error::Capacity)?
            .0;
        image.young = image
            .young
            .insert(key.address as u64, key.id)
            .ok_or(Error::Capacity)?
            .0;
        Ok(self.prepare(view, image))
    }

    pub(crate) fn remove(&self, key: Key) -> Result<Prepared<V>, Error> {
        let view = self.view();
        view.get(key).ok_or(Error::Missing)?;
        let mut image = view.0.image.clone();
        image.objects = image.objects.remove(key.address as u64).0;
        image.identities = image.identities.remove(key.id.to_i64().cast_unsigned()).0;
        image.young = image.young.remove(key.address as u64).0;
        Ok(self.prepare(view, image))
    }

    pub(crate) fn promote(&self, key: Key) -> Result<Prepared<V>, Error> {
        let view = self.view();
        if !view.is_young(key) {
            return Err(Error::Missing);
        }
        let mut image = view.0.image.clone();
        image.young = image.young.remove(key.address as u64).0;
        Ok(self.prepare(view, image))
    }

    fn prepare(&self, expected: View<V>, image: Image<V>) -> Prepared<V> {
        Prepared {
            registry: self.identity.clone(),
            expected: expected.0,
            proposed: Arc::new(Head {
                image,
                installed: AtomicBool::new(false),
            }),
        }
    }

    /// At most one owning head load and CAS. No retry loop or lock is held here.
    /// Preparation/path destruction and the owning-link implementation have no
    /// wall-clock bound. Every replacement certifies its predecessor before CAS.
    pub(crate) fn commit(&self, operation: &Prepared<V>) -> Status {
        self.commit_observed(operation, || {}, || {})
    }

    pub(crate) fn commit_observed(
        &self,
        operation: &Prepared<V>,
        before_compare: impl FnOnce(),
        after_compare: impl FnOnce(),
    ) -> Status {
        if !Arc::ptr_eq(&self.identity, &operation.registry) {
            return Status::Foreign;
        }
        if operation.proposed.installed.load(Ordering::Acquire) {
            return Status::Installed;
        }
        let observed = self.head.load();
        if Arc::ptr_eq(&observed, &operation.proposed) {
            operation.proposed.installed.store(true, Ordering::Release);
            return Status::Installed;
        }
        if !Arc::ptr_eq(&observed, &operation.expected) {
            // Observing the changed head acquires its predecessor certification.
            return if operation.proposed.installed.load(Ordering::Acquire) {
                Status::Installed
            } else {
                Status::Stale
            };
        }
        observed.installed.store(true, Ordering::Release);
        before_compare();
        if !self
            .head
            .compare(&operation.expected, operation.proposed.clone())
        {
            return Status::Retry;
        }
        after_compare();
        operation.proposed.installed.store(true, Ordering::Release);
        Status::Installed
    }
}
