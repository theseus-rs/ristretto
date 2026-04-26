use crate::Error::InternalError;
use crate::Result;
use ristretto_gc::sync::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Type erased per VM resource storage.
pub struct ResourceManager {
    resources: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ResourceManager {
    /// Creates a new empty `ResourceManager`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            resources: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a reference to the resource of type `T`, initializing it with `init`
    /// if it does not yet exist.
    ///
    /// # Errors
    ///
    /// Returns an error if a resource was previously stored under the same `TypeId` with a
    /// different concrete type.
    pub fn get_or_init<T: Any + Send + Sync, F: FnOnce() -> T>(&self, init: F) -> Result<Arc<T>> {
        let type_id = TypeId::of::<T>();

        // Fast path: read lock
        {
            let guard = self.resources.read();
            if let Some(resource) = guard.get(&type_id) {
                return resource
                    .clone()
                    .downcast::<T>()
                    .map_err(|_| InternalError("ResourceManager type mismatch".to_string()));
            }
        }

        // Slow path: write lock, double-check
        let mut guard = self.resources.write();
        guard
            .entry(type_id)
            .or_insert_with(|| Arc::new(init()))
            .clone()
            .downcast::<T>()
            .map_err(|_| InternalError("ResourceManager type mismatch".to_string()))
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.resources.read();
        f.debug_struct("ResourceManager")
            .field("resource_count", &guard.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI64, Ordering};

    struct TestResource {
        counter: AtomicI64,
    }

    impl TestResource {
        fn new() -> Self {
            Self {
                counter: AtomicI64::new(0),
            }
        }
    }

    #[test]
    fn test_new() {
        let manager = ResourceManager::new();
        let debug = format!("{manager:?}");
        assert!(debug.contains("resource_count: 0"));
    }

    #[test]
    fn test_default() {
        let manager = ResourceManager::default();
        let debug = format!("{manager:?}");
        assert!(debug.contains("resource_count: 0"));
    }

    #[test]
    fn test_get_or_init() {
        let manager = ResourceManager::new();
        let resource = manager.get_or_init(TestResource::new).expect("resource");
        assert_eq!(resource.counter.load(Ordering::SeqCst), 0);
        resource.counter.store(42, Ordering::SeqCst);

        // Second call returns the same instance
        let resource2 = manager.get_or_init(TestResource::new).expect("resource");
        assert_eq!(resource2.counter.load(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_different_types() {
        let manager = ResourceManager::new();
        let _r1 = manager.get_or_init(TestResource::new).expect("resource");
        let r2 = manager
            .get_or_init(|| "hello".to_string())
            .expect("resource");
        assert_eq!(*r2, "hello");

        let debug = format!("{manager:?}");
        assert!(debug.contains("resource_count: 2"));
    }
}
