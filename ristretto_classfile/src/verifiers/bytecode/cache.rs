//! # Verification Cache
//!
//! This module provides caching for verification artifacts to avoid
//! redundant parsing and computation.
//!
//! # Cached Artifacts
//!
//! - Decoded `StackMapTable` per method
//! - Parsed method descriptors
//! - Per-method verification results
//! - Constant pool decoding helpers
//!
//! # Thread Safety
//!
//! The cache uses `RwLock` for thread-safe access when enabled.
//!
//! # References
//!
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

use std::collections::HashMap;
use std::sync::RwLock;

use crate::FieldType;

/// Cached verification result for a method.
#[derive(Debug, Clone)]
pub enum CachedResult {
    /// Verification succeeded.
    Success,
    /// Verification failed with the given error message.
    Failed(String),
}

/// Key for identifying a method in the cache.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MethodKey {
    /// Class name.
    pub class_name: String,
    /// Method name.
    pub method_name: String,
    /// Method descriptor.
    pub descriptor: String,
}

impl MethodKey {
    /// Creates a new method key.
    pub fn new(
        class_name: impl Into<String>,
        method_name: impl Into<String>,
        descriptor: impl Into<String>,
    ) -> Self {
        Self {
            class_name: class_name.into(),
            method_name: method_name.into(),
            descriptor: descriptor.into(),
        }
    }
}

/// Parsed method descriptor cache entry.
#[derive(Debug, Clone)]
pub struct ParsedDescriptor {
    /// Parameter types.
    pub parameters: Vec<FieldType>,
    /// Return type (None for void).
    pub return_type: Option<FieldType>,
}

/// Verification cache for reusing parsed artifacts.
///
/// This cache stores verification-related artifacts that are expensive
/// to compute, allowing them to be reused across multiple verification
/// passes or class loads.
#[derive(Debug, Default)]
pub struct VerificationCache {
    /// Whether caching is enabled.
    enabled: bool,

    /// Cached verification results.
    results: RwLock<HashMap<MethodKey, CachedResult>>,

    /// Cached parsed descriptors.
    descriptors: RwLock<HashMap<String, ParsedDescriptor>>,

    /// Cache statistics.
    stats: RwLock<CacheStats>,
}

/// Cache statistics for monitoring.
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// Number of cache hits for results.
    pub result_hits: u64,
    /// Number of cache misses for results.
    pub result_misses: u64,
    /// Number of cache hits for descriptors.
    pub descriptor_hits: u64,
    /// Number of cache misses for descriptors.
    pub descriptor_misses: u64,
}

impl VerificationCache {
    /// Creates a new verification cache.
    #[must_use]
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            results: RwLock::new(HashMap::new()),
            descriptors: RwLock::new(HashMap::new()),
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Creates a disabled cache (no-op).
    #[must_use]
    pub fn disabled() -> Self {
        Self::new(false)
    }

    /// Returns whether caching is enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Gets a cached verification result.
    #[must_use]
    pub fn get_result(&self, key: &MethodKey) -> Option<CachedResult> {
        if !self.enabled {
            return None;
        }

        let guard = self.results.read().ok()?;
        let result = guard.get(key).cloned();

        drop(guard);

        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            if result.is_some() {
                stats.result_hits += 1;
            } else {
                stats.result_misses += 1;
            }
        }

        result
    }

    /// Stores a verification result in the cache.
    pub fn put_result(&self, key: MethodKey, result: CachedResult) {
        if !self.enabled {
            return;
        }

        if let Ok(mut guard) = self.results.write() {
            guard.insert(key, result);
        }
    }

    /// Gets a cached parsed descriptor.
    #[must_use]
    pub fn get_descriptor(&self, descriptor: &str) -> Option<ParsedDescriptor> {
        if !self.enabled {
            return None;
        }

        let guard = self.descriptors.read().ok()?;
        let result = guard.get(descriptor).cloned();

        drop(guard);

        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            if result.is_some() {
                stats.descriptor_hits += 1;
            } else {
                stats.descriptor_misses += 1;
            }
        }

        result
    }

    /// Parses and caches a method descriptor.
    pub fn parse_descriptor(&self, descriptor: &str) -> Option<ParsedDescriptor> {
        // Check cache first
        if let Some(cached) = self.get_descriptor(descriptor) {
            return Some(cached);
        }

        // Parse the descriptor
        let (parameters, return_type) = FieldType::parse_method_descriptor(descriptor).ok()?;

        let parsed = ParsedDescriptor {
            parameters,
            return_type,
        };

        // Cache it
        if self.enabled
            && let Ok(mut guard) = self.descriptors.write()
        {
            guard.insert(descriptor.to_string(), parsed.clone());
        }

        Some(parsed)
    }

    /// Gets cache statistics.
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        self.stats.read().map(|s| s.clone()).unwrap_or_default()
    }

    /// Clears all cached data.
    pub fn clear(&self) {
        if let Ok(mut guard) = self.results.write() {
            guard.clear();
        }
        if let Ok(mut guard) = self.descriptors.write() {
            guard.clear();
        }
        if let Ok(mut stats) = self.stats.write() {
            *stats = CacheStats::default();
        }
    }

    /// Returns the number of cached results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.read().map(|g| g.len()).unwrap_or(0)
    }

    /// Returns the number of cached descriptors.
    #[must_use]
    pub fn descriptor_count(&self) -> usize {
        self.descriptors.read().map(|g| g.len()).unwrap_or(0)
    }
}

/// Arena-style allocator for verification frames.
///
/// This provides a pool of reusable frame buffers to reduce allocation
/// overhead during verification.
#[derive(Debug)]
pub struct FramePool {
    /// Pool of reusable local variable buffers.
    locals_pool: Vec<Vec<crate::verifiers::bytecode::type_system::VerificationType>>,
    /// Pool of reusable stack buffers.
    stack_pool: Vec<Vec<crate::verifiers::bytecode::type_system::VerificationType>>,
    /// Maximum pool size.
    max_size: usize,
}

impl FramePool {
    /// Creates a new frame pool with the given maximum size.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            locals_pool: Vec::with_capacity(max_size),
            stack_pool: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Acquires a locals buffer from the pool.
    pub fn acquire_locals(
        &mut self,
        capacity: usize,
    ) -> Vec<crate::verifiers::bytecode::type_system::VerificationType> {
        if let Some(mut buffer) = self.locals_pool.pop() {
            buffer.clear();
            if buffer.capacity() < capacity {
                buffer.reserve(capacity);
            }
            buffer
        } else {
            Vec::with_capacity(capacity)
        }
    }

    /// Returns a locals buffer to the pool.
    pub fn return_locals(
        &mut self,
        buffer: Vec<crate::verifiers::bytecode::type_system::VerificationType>,
    ) {
        if self.locals_pool.len() < self.max_size {
            self.locals_pool.push(buffer);
        }
    }

    /// Acquires a stack buffer from the pool.
    pub fn acquire_stack(
        &mut self,
        capacity: usize,
    ) -> Vec<crate::verifiers::bytecode::type_system::VerificationType> {
        if let Some(mut buffer) = self.stack_pool.pop() {
            buffer.clear();
            if buffer.capacity() < capacity {
                buffer.reserve(capacity);
            }
            buffer
        } else {
            Vec::with_capacity(capacity)
        }
    }

    /// Returns a stack buffer to the pool.
    pub fn return_stack(
        &mut self,
        buffer: Vec<crate::verifiers::bytecode::type_system::VerificationType>,
    ) {
        if self.stack_pool.len() < self.max_size {
            self.stack_pool.push(buffer);
        }
    }

    /// Clears the pool.
    pub fn clear(&mut self) {
        self.locals_pool.clear();
        self.stack_pool.clear();
    }
}

impl Default for FramePool {
    fn default() -> Self {
        Self::new(32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_disabled() {
        let cache = VerificationCache::disabled();
        assert!(!cache.is_enabled());

        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result(key.clone(), CachedResult::Success);

        // Should not cache when disabled
        assert!(cache.get_result(&key).is_none());
    }

    #[test]
    fn test_cache_enabled() {
        let cache = VerificationCache::new(true);
        assert!(cache.is_enabled());

        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result(key.clone(), CachedResult::Success);

        let result = cache.get_result(&key);
        assert!(matches!(result, Some(CachedResult::Success)));
    }

    #[test]
    fn test_descriptor_cache() {
        let cache = VerificationCache::new(true);

        let parsed = cache.parse_descriptor("(II)V");
        assert!(parsed.is_some());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.parameters.len(), 2);
        assert!(parsed.return_type.is_none());

        // Should be cached now
        assert_eq!(cache.descriptor_count(), 1);

        // Second lookup should hit cache
        let _ = cache.parse_descriptor("(II)V");
        let stats = cache.stats();
        assert_eq!(stats.descriptor_hits, 1);
    }

    #[test]
    fn test_cache_clear() {
        let cache = VerificationCache::new(true);

        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result(key.clone(), CachedResult::Success);
        cache.parse_descriptor("(II)V");

        assert_eq!(cache.result_count(), 1);
        assert_eq!(cache.descriptor_count(), 1);

        cache.clear();

        assert_eq!(cache.result_count(), 0);
        assert_eq!(cache.descriptor_count(), 0);
    }

    #[test]
    fn test_frame_pool() {
        let mut pool = FramePool::new(4);

        // Acquire buffers
        let locals = pool.acquire_locals(10);
        let stack = pool.acquire_stack(5);

        assert!(locals.capacity() >= 10);
        assert!(stack.capacity() >= 5);

        // Return to pool
        pool.return_locals(locals);
        pool.return_stack(stack);

        // Acquire again - should reuse
        let locals2 = pool.acquire_locals(5);
        assert!(locals2.capacity() >= 10); // Still has old capacity
    }

    #[test]
    fn test_method_key_traits() {
        let key1 = MethodKey::new("Class", "method", "()V");
        let key2 = MethodKey::new("Class", "method", "()V");
        let key3 = MethodKey::new("Class", "other", "()V");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);

        // Test Debug
        let debug_str = format!("{key1:?}");
        assert!(debug_str.contains("MethodKey"));
        assert!(debug_str.contains("Class"));

        // Test Clone
        let key_clone = key1.clone();
        assert_eq!(key1, key_clone);
    }

    #[test]
    fn test_cached_result_traits() {
        let success = CachedResult::Success;
        let failed = CachedResult::Failed("error".to_string());

        // Test Debug
        assert!(format!("{success:?}").contains("Success"));
        assert!(format!("{failed:?}").contains("Failed"));

        // Test Clone
        let success_clone = success.clone();
        assert!(matches!(success_clone, CachedResult::Success));
    }

    #[test]
    fn test_parsed_descriptor_traits() {
        let desc = ParsedDescriptor {
            parameters: vec![],
            return_type: None,
        };

        // Test Debug
        assert!(format!("{desc:?}").contains("ParsedDescriptor"));

        // Test Clone
        let desc_clone = desc.clone();
        assert!(desc_clone.parameters.is_empty());
        assert!(desc_clone.return_type.is_none());
    }

    #[test]
    fn test_cache_default() {
        let cache = VerificationCache::default();
        assert!(!cache.is_enabled()); // Default bool is false
    }

    #[test]
    fn test_cache_stats_misses() {
        let cache = VerificationCache::new(true);
        let key = MethodKey::new("Test", "foo", "()V");

        // Result miss
        assert!(cache.get_result(&key).is_none());

        // Descriptor miss (via get_descriptor directly)
        assert!(cache.get_descriptor("()V").is_none());

        let stats = cache.stats();
        assert_eq!(stats.result_misses, 1);
        assert_eq!(stats.descriptor_misses, 1);
        assert_eq!(stats.result_hits, 0);
        assert_eq!(stats.descriptor_hits, 0);
    }

    #[test]
    fn test_parse_descriptor_invalid() {
        let cache = VerificationCache::new(true);
        let result = cache.parse_descriptor("invalid");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_descriptor_disabled() {
        let cache = VerificationCache::disabled();
        let result = cache.parse_descriptor("(II)V");
        assert!(result.is_some()); // Should still parse

        // But not cache
        assert_eq!(cache.descriptor_count(), 0);
    }

    #[test]
    fn test_frame_pool_default() {
        let pool = FramePool::default();
        assert_eq!(pool.max_size, 32);
    }

    #[test]
    fn test_frame_pool_resize() {
        let mut pool = FramePool::new(1);

        // Acquire and return a small buffer
        let locals = pool.acquire_locals(5);
        pool.return_locals(locals);

        // Acquire a larger buffer - should reuse and resize
        let locals = pool.acquire_locals(10);
        assert!(locals.capacity() >= 10);

        pool.return_locals(locals);

        // Same for stack
        let stack = pool.acquire_stack(5);
        pool.return_stack(stack);

        let stack = pool.acquire_stack(10);
        assert!(stack.capacity() >= 10);
    }

    #[test]
    fn test_frame_pool_limit() {
        let mut pool = FramePool::new(1);

        let l1 = pool.acquire_locals(1);
        let l2 = pool.acquire_locals(1);

        pool.return_locals(l1);
        // Pool is now full (size 1)
        pool.return_locals(l2);
        // Should be dropped, not added to pool

        assert_eq!(pool.locals_pool.len(), 1);

        // Same for stack
        let s1 = pool.acquire_stack(1);
        let s2 = pool.acquire_stack(1);

        pool.return_stack(s1);
        pool.return_stack(s2);

        assert_eq!(pool.stack_pool.len(), 1);
    }

    #[test]
    fn test_frame_pool_clear() {
        let mut pool = FramePool::new(5);
        let l = pool.acquire_locals(1);
        pool.return_locals(l);
        let s = pool.acquire_stack(1);
        pool.return_stack(s);

        assert_eq!(pool.locals_pool.len(), 1);
        assert_eq!(pool.stack_pool.len(), 1);

        pool.clear();

        assert_eq!(pool.locals_pool.len(), 0);
        assert_eq!(pool.stack_pool.len(), 0);
    }

    #[test]
    fn test_cache_stats_traits() {
        let stats = CacheStats::default();
        let debug_str = format!("{stats:?}");
        assert!(debug_str.contains("CacheStats"));

        let stats_clone = stats.clone();
        assert_eq!(stats.result_hits, stats_clone.result_hits);
    }

    #[test]
    fn test_verification_cache_debug() {
        let cache = VerificationCache::new(true);
        let debug_str = format!("{cache:?}");
        assert!(debug_str.contains("VerificationCache"));
    }
}
