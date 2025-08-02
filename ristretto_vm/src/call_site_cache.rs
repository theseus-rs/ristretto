//! Call site cache for invokedynamic instruction.
//!
//! This module provides a thread-safe cache that tracks the resolution state of invokedynamic call
//! sites.

use crate::Error::{InternalError, PoisonedLock};
use crate::Result;
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// Unique identifier for an invokedynamic call site
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallSiteKey {
    /// Class name containing the invokedynamic instruction
    pub class_name: String,
    /// Constant pool index of the invokedynamic instruction
    pub instruction_index: u16,
}

impl CallSiteKey {
    /// Create a new call site key
    pub fn new(class_name: String, instruction_index: u16) -> Self {
        Self {
            class_name,
            instruction_index,
        }
    }
}

/// State of an invokedynamic call site resolution
#[derive(Debug, Clone)]
pub enum CallSiteState {
    /// Call site resolution is currently in progress
    InProgress,
    /// Call site has been successfully resolved
    Resolved(Value),
}

/// Thread-safe cache for invokedynamic call site resolution
#[derive(Debug)]
pub struct CallSiteCache {
    /// Maps call site keys to their resolution states
    states: RwLock<HashMap<CallSiteKey, CallSiteState>>,
}

impl CallSiteCache {
    /// Create a new empty call site cache
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    /// Resolves a call site given a key and a resolver function.
    ///
    /// This method checks if the call site is already being resolved or has been resolved
    /// (returning cached result). If neither, it marks the call site as in progress, executes the
    /// resolver function, and caches the result.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the call site
    /// * `resolver` - Function that performs the actual call site resolution
    ///
    /// # Returns
    ///
    /// The resolved call site value
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Recursive call site resolution is detected
    /// - The resolver function fails
    /// - Cache operations fail
    pub async fn resolve_with_cache<F, Fut>(&self, key: CallSiteKey, resolver: F) -> Result<Value>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value>>,
    {
        use tracing::debug;

        debug!("CallSiteCache: Checking cache for key: {key:?}");

        // Check current state
        {
            let map = self
                .states
                .read()
                .map_err(|error| PoisonedLock(format!("Failed to acquire cache lock: {error}")))?;

            match map.get(&key) {
                Some(CallSiteState::InProgress) => {
                    debug!("CallSiteCache: RECURSION DETECTED for key: {key:?}",);
                    return Err(InternalError(format!(
                        "Recursive invokedynamic call site resolution detected for class '{}' at index {}",
                        key.class_name, key.instruction_index
                    )));
                }
                Some(CallSiteState::Resolved(value)) => {
                    debug!("CallSiteCache: Returning cached result for key: {key:?}");
                    return Ok(value.clone());
                }
                None => {
                    debug!("CallSiteCache: Key not found in cache, will resolve: {key:?}");
                    // Call site not yet resolved, continue to resolution
                }
            }
        }

        // Mark as in progress
        {
            let mut map = self
                .states
                .write()
                .map_err(|error| PoisonedLock(format!("Failed to acquire cache lock: {error}")))?;
            debug!("CallSiteCache: Marking as InProgress: {key:?}");
            map.insert(key.clone(), CallSiteState::InProgress);
        }

        // Perform resolution
        debug!("CallSiteCache: Starting resolution for key: {key:?}");
        let result = resolver().await;
        debug!(
            "CallSiteCache: Resolution completed for key: {key:?}, success: {}",
            result.is_ok()
        );

        // Update cache based on result
        match &result {
            Ok(value) => {
                // Store successful resolution
                let mut map = self.states.write().map_err(|error| {
                    PoisonedLock(format!("Failed to acquire cache lock: {error}"))
                })?;
                debug!("CallSiteCache: Caching successful result for key: {key:?}",);
                map.insert(key, CallSiteState::Resolved(value.clone()));
            }
            Err(_) => {
                // Remove in-progress marker on failure to allow retry
                let mut map = self.states.write().map_err(|error| {
                    PoisonedLock(format!("Failed to acquire cache lock: {error}"))
                })?;
                debug!("CallSiteCache: Removing failed resolution from cache for key: {key:?}");
                map.remove(&key);
            }
        }

        result
    }

    /// Clear all cached call sites
    pub fn clear(&self) -> Result<()> {
        let mut map = self
            .states
            .write()
            .map_err(|error| PoisonedLock(format!("Failed to acquire cache lock: {error}")))?;
        map.clear();
        Ok(())
    }

    /// Get the number of cached call sites
    pub fn len(&self) -> Result<usize> {
        let map = self
            .states
            .read()
            .map_err(|error| PoisonedLock(format!("Failed to acquire cache lock: {error}")))?;
        Ok(map.len())
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> Result<bool> {
        let map = self
            .states
            .read()
            .map_err(|error| PoisonedLock(format!("Failed to acquire cache lock: {error}")))?;
        Ok(map.is_empty())
    }
}

impl Default for CallSiteCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_call_site_cache_basic_resolution() -> Result<()> {
        let cache = CallSiteCache::new();
        let key = CallSiteKey::new("TestClass".to_string(), 42);
        let expected_value = Value::Object(None);

        let result = cache
            .resolve_with_cache(key.clone(), || async { Ok(expected_value.clone()) })
            .await?;

        assert_eq!(result, expected_value);
        Ok(())
    }

    #[tokio::test]
    async fn test_call_site_cache_prevents_recursion() {
        let cache = Arc::new(CallSiteCache::new());
        let key = CallSiteKey::new("TestClass".to_string(), 42);

        let cache_clone = cache.clone();
        let key_clone = key.clone();

        let result = cache
            .resolve_with_cache(key, || async move {
                // This should detect recursion and fail
                cache_clone
                    .resolve_with_cache(key_clone, || async { Ok(Value::Object(None)) })
                    .await
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .expect_err("Should return an error")
                .to_string()
                .contains("Recursive invokedynamic")
        );
    }

    #[tokio::test]
    async fn test_call_site_cache_returns_cached_result() -> Result<()> {
        let cache = CallSiteCache::new();
        let key = CallSiteKey::new("TestClass".to_string(), 42);
        let expected_value = Value::Object(None);

        // First resolution
        let result1 = cache
            .resolve_with_cache(key.clone(), || async { Ok(expected_value.clone()) })
            .await?;

        // Second resolution should return cached result
        let result2 = cache
            .resolve_with_cache(key, || async {
                panic!("Resolver should not be called for cached result")
            })
            .await?;

        assert_eq!(result1, result2);
        Ok(())
    }
}
