use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// A type-keyed container for per-VM extensible state.
///
/// Extensions allows arbitrary `Send + Sync` types to be stored and retrieved by their `TypeId`.
/// This enables subsystems (e.g., audio, networking) to attach per-VM state without modifying the
/// VM struct directly.
#[derive(Debug, Default)]
pub struct Extensions {
    map: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl Extensions {
    /// Create a new, empty Extensions container.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a value by type, or initialize it with the provided closure if absent.
    ///
    /// This is the primary method for accessing extension state. The closure is called at
    /// most once, and the result is cached for subsequent calls.
    ///
    /// # Panics
    ///
    /// Panics if a `TypeId` collision causes a downcast failure (should never happen in practice).
    pub fn get_or_init<T: Any + Send + Sync>(&self, init: impl FnOnce() -> T) -> Arc<T> {
        let type_id = TypeId::of::<T>();

        // Fast path: read lock
        {
            let map = self.map.read();
            if let Some(value) = map.get(&type_id) {
                // Type safety guaranteed by TypeId key
                return Arc::clone(value)
                    .downcast::<T>()
                    .expect("TypeId mismatch in Extensions");
            }
        }

        // Slow path: write lock, double-check
        let mut map = self.map.write();
        if let Some(value) = map.get(&type_id) {
            return Arc::clone(value)
                .downcast::<T>()
                .expect("TypeId mismatch in Extensions");
        }

        let value = Arc::new(init());
        map.insert(type_id, Arc::clone(&value) as Arc<dyn Any + Send + Sync>);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestState {
        value: i32,
    }

    #[derive(Debug)]
    struct OtherState {
        name: String,
    }

    #[test]
    fn test_get_or_init_creates_value() {
        let ext = Extensions::new();
        let state = ext.get_or_init(|| TestState { value: 42 });
        assert_eq!(state.value, 42);
    }

    #[test]
    fn test_get_or_init_returns_same_value() {
        let ext = Extensions::new();
        let state1 = ext.get_or_init(|| TestState { value: 42 });
        let state2 = ext.get_or_init(|| TestState { value: 99 });
        assert_eq!(state1.value, 42);
        assert_eq!(state2.value, 42); // init not called again
    }

    #[test]
    fn test_different_types_independent() {
        let ext = Extensions::new();
        let test = ext.get_or_init(|| TestState { value: 42 });
        let other = ext.get_or_init(|| OtherState {
            name: "hello".to_string(),
        });
        assert_eq!(test.value, 42);
        assert_eq!(other.name, "hello");
    }

    #[test]
    fn test_drops_when_extensions_drops() {
        let arc = {
            let ext = Extensions::new();
            ext.get_or_init(|| TestState { value: 1 })
        };
        // Extensions dropped, arc is the only remaining reference
        assert_eq!(Arc::strong_count(&arc), 1);
    }
}
