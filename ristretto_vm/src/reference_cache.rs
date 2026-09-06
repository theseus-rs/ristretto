//! VM-owned reference tables, indexed by constant pool index within each caller class.

use dashmap::DashMap;
use ristretto_classloader::Class;
use std::sync::Arc;

/// A table retains its class, preventing pointer reuse and distinguishing defining loaders.
#[derive(Debug)]
pub(crate) struct ClassReferences<T> {
    _class: Arc<Class>,
    entries: Box<[T]>,
}

impl<T> ClassReferences<T> {
    pub(crate) fn get(&self, index: u16) -> Option<&T> {
        self.entries.get(usize::from(index))
    }
}

/// The map is consulted once per frame; hits use the frame's indexed table directly.
#[derive(Debug)]
pub(crate) struct ReferenceCache<T> {
    classes: DashMap<usize, Arc<ClassReferences<T>>>,
}

impl<T: Default> ReferenceCache<T> {
    pub(crate) fn new() -> Self {
        Self {
            classes: DashMap::new(),
        }
    }

    pub(crate) fn for_class(&self, class: &Arc<Class>) -> Arc<ClassReferences<T>> {
        let identity = Arc::as_ptr(class) as usize;
        if let Some(entries) = self.classes.get(&identity) {
            return entries.clone();
        }
        self.classes
            .entry(identity)
            .or_insert_with(|| {
                Arc::new(ClassReferences {
                    _class: class.clone(),
                    // CP indices start at one; the length includes wide constant placeholders.
                    entries: (0..=class.constant_pool().len())
                        .map(|_| T::default())
                        .collect(),
                })
            })
            .clone()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.classes.len()
    }
}
