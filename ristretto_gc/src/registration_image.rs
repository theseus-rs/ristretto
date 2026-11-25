//! Production representation for a future authoritative ordinary registry.
//!
//! No collector owns this registry yet: activation must join both phase domains,
//! initial roots and construction custody before replacing the old indexes.
//! This module is compiled normally and shares its key validator with the active
//! registration path. It is not a second lookup authority or a public GC backend.
use crate::metadata::MetadataRef;
use crate::publication_directory::Directory;
use crate::registration_key::{Conflict, Key, Observed};
use arc_swap::ArcSwap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

struct Link<T>(ArcSwap<T>);

impl<T> Link<T> {
    fn new(value: Arc<T>) -> Self {
        Self(ArcSwap::from(value))
    }
    fn load(&self) -> Arc<T> {
        self.0.load_full()
    }
    fn compare(&self, expected: &Arc<T>, proposed: Arc<T>) -> bool {
        Arc::ptr_eq(&self.0.compare_and_swap(expected, proposed), expected)
    }
}

#[path = "registration_image_state.rs"]
mod state;
pub(crate) use state::{Error, Status};
pub(crate) type Prepared = state::Prepared<MetadataRef>;
pub(crate) type View = state::View<MetadataRef>;

pub(crate) struct Registry {
    owner: Weak<crate::GarbageCollector>,
    state: state::Registry<MetadataRef>,
}

impl Registry {
    pub(crate) fn new(owner: Weak<crate::GarbageCollector>) -> Self {
        Self {
            owner,
            state: state::Registry::new(),
        }
    }

    pub(crate) fn view(&self) -> View {
        self.state.view()
    }

    pub(crate) fn insert(&self, value: &MetadataRef) -> Result<Prepared, Error> {
        if !Weak::ptr_eq(&self.owner, &value.collector) {
            return Err(Error::ForeignCollector);
        }
        self.state.insert(
            Key {
                address: value.ptr().0,
                id: value.id,
            },
            value.clone(),
        )
    }

    pub(crate) fn remove(&self, key: Key) -> Result<Prepared, Error> {
        self.state.remove(key)
    }

    pub(crate) fn promote(&self, key: Key) -> Result<Prepared, Error> {
        self.state.promote(key)
    }

    pub(crate) fn commit(&self, operation: &Prepared) -> Status {
        self.state.commit(operation)
    }
}

#[cfg(test)]
#[path = "registration_image_tests.rs"]
mod tests;

#[cfg(all(test, not(target_family = "wasm")))]
#[path = "registration_image_model.rs"]
mod model;
