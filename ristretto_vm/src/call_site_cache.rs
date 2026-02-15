//! Call site cache for invokedynamic instruction.
//!
//! This module provides a thread-safe cache that tracks the resolution state of invokedynamic call
//! sites. When multiple threads attempt to resolve the same call site concurrently, only the first
//! thread performs the resolution while others wait for the result.

use crate::Result;
use dashmap::DashMap;
use ristretto_classloader::Value;
use std::sync::Arc;
use tokio::sync::Notify;

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
    /// Call site resolution is currently in progress; waiters are notified when done
    InProgress(Arc<Notify>),
    /// Call site has been successfully resolved
    Resolved(Value),
}

/// Thread-safe cache for invokedynamic call site resolution
#[derive(Debug)]
pub struct CallSiteCache {
    /// Maps call site keys to their resolution states
    states: DashMap<CallSiteKey, CallSiteState>,
}

impl CallSiteCache {
    /// Create a new empty call site cache
    pub fn new() -> Self {
        Self {
            states: DashMap::new(),
        }
    }

    /// Resolves a call site given a key and a resolver function.
    ///
    /// This method checks if the call site is already being resolved or has been resolved
    /// (returning cached result). If another thread is currently resolving this call site,
    /// this method waits for that resolution to complete and returns the cached result.
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
    /// Returns an error if the resolver function fails
    pub async fn resolve_with_cache<F, Fut>(&self, key: CallSiteKey, resolver: F) -> Result<Value>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value>>,
    {
        use tracing::debug;

        debug!("CallSiteCache: Checking cache for key: {key:?}");

        // Check current state
        loop {
            let wait_notify = {
                if let Some(ref state) = self.states.get(&key) {
                    match &**state {
                        CallSiteState::InProgress(notify) => {
                            // Another thread is resolving this call site; wait for it
                            debug!("CallSiteCache: Another thread resolving, waiting: {key:?}");
                            Some(notify.clone())
                        }
                        CallSiteState::Resolved(value) => {
                            debug!("CallSiteCache: Returning cached result for key: {key:?}");
                            return Ok(value.clone());
                        }
                    }
                } else {
                    debug!("CallSiteCache: Key not found in cache, will resolve: {key:?}");
                    None
                }
            }; // DashMap guard is dropped here

            if let Some(notify) = wait_notify {
                notify.notified().await;
            } else {
                break;
            }
        }

        // Mark as in progress with a Notify so other threads can wait
        let notify = Arc::new(Notify::new());
        debug!("CallSiteCache: Marking as InProgress: {key:?}");
        self.states
            .insert(key.clone(), CallSiteState::InProgress(notify.clone()));

        // Perform resolution
        debug!("CallSiteCache: Starting resolution for key: {key:?}");
        let result = resolver().await;
        debug!(
            "CallSiteCache: Resolution completed for key: {key:?}, success: {}",
            result.is_ok()
        );

        // Update cache based on result
        if let Ok(value) = &result {
            // Store successful resolution
            debug!("CallSiteCache: Caching successful result for key: {key:?}",);
            self.states
                .insert(key, CallSiteState::Resolved(value.clone()));
        } else {
            // Remove in-progress marker on failure to allow retry
            debug!("CallSiteCache: Removing failed resolution from cache for key: {key:?}");
            self.states.remove_if(&key, |_, state| {
                matches!(state, CallSiteState::InProgress(_))
            });
        }

        // Notify all waiters that resolution is complete (or failed)
        notify.notify_waiters();

        result
    }

    /// Clear all cached call sites
    pub fn clear(&self) {
        self.states.clear();
    }

    /// Get the number of cached call sites
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
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
    async fn test_call_site_cache_concurrent_resolution() -> Result<()> {
        let cache = Arc::new(CallSiteCache::new());
        let key = CallSiteKey::new("TestClass".to_string(), 42);
        let expected_value = Value::Object(None);

        // Spawn two tasks that try to resolve the same key concurrently
        let cache1 = cache.clone();
        let key1 = key.clone();
        let expected1 = expected_value.clone();
        let handle1 = tokio::spawn(async move {
            cache1
                .resolve_with_cache(key1, || async { Ok(expected1) })
                .await
        });

        let cache2 = cache.clone();
        let key2 = key.clone();
        let expected2 = expected_value.clone();
        let handle2 = tokio::spawn(async move {
            cache2
                .resolve_with_cache(key2, || async { Ok(expected2) })
                .await
        });

        let result1 = handle1.await.unwrap()?;
        let result2 = handle2.await.unwrap()?;

        assert_eq!(result1, expected_value);
        assert_eq!(result2, expected_value);
        Ok(())
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
