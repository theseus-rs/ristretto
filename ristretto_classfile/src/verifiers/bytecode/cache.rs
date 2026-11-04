//! # Verification Cache
//!
//! This module caches method verification results to avoid redundant computation.
//!
//! # Cached Artifacts
//!
//! - Per-method verification results
//!
//! # Thread Safety
//!
//! The cache uses `RwLock` for thread-safe access when enabled.
//!
//! # References
//!
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

use ahash::AHashSet;
use hashbrown::Equivalent;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

use crate::java_string::{JavaStr, JavaString};

/// Cached verification result for a method.
#[derive(Debug, Clone)]
pub enum CachedResult {
    /// Verification succeeded.
    Success,
    /// Verification failed with the given error message.
    Failed(String),
}

/// Key for identifying a method in the cache.
///
/// Uses `Cow<'a, JavaStr>` fields so that lookups can borrow directly from
/// the constant pool (`MethodKey<'borrowed>`) while the cache stores
/// fully owned names and shared exact scope bytes.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MethodKey<'a> {
    /// Class name.
    pub class_name: Cow<'a, JavaStr>,
    /// Method name.
    pub method_name: Cow<'a, JavaStr>,
    /// Method descriptor.
    pub descriptor: Cow<'a, JavaStr>,
    /// Exact class, method, configuration and context identity for result reuse.
    pub scope: Cow<'a, [u8]>,
}

/// Exact scope bytes interned once and shared by cached method results.
pub(crate) type CacheScope = Arc<[u8]>;

#[derive(Debug)]
struct CachedMethodKey {
    class_name: JavaString,
    method_name: JavaString,
    descriptor: JavaString,
    scope: CacheScope,
}

impl PartialEq for CachedMethodKey {
    fn eq(&self, other: &Self) -> bool {
        self.class_name == other.class_name
            && self.method_name == other.method_name
            && self.descriptor == other.descriptor
            && Arc::ptr_eq(&self.scope, &other.scope)
    }
}
impl Eq for CachedMethodKey {}
impl Hash for CachedMethodKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.class_name.hash(state);
        self.method_name.hash(state);
        self.descriptor.hash(state);
        std::ptr::hash(Arc::as_ptr(&self.scope), state);
    }
}

struct MethodKeyLookup<'a, 'b>(&'a MethodKey<'b>, &'a CacheScope);
impl Hash for MethodKeyLookup<'_, '_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.class_name.hash(state);
        self.0.method_name.hash(state);
        self.0.descriptor.hash(state);
        std::ptr::hash(Arc::as_ptr(self.1), state);
    }
}
impl Equivalent<CachedMethodKey> for MethodKeyLookup<'_, '_> {
    fn equivalent(&self, key: &CachedMethodKey) -> bool {
        self.0.class_name.as_ref() == key.class_name.as_java_str()
            && self.0.method_name.as_ref() == key.method_name.as_java_str()
            && self.0.descriptor.as_ref() == key.descriptor.as_java_str()
            && Arc::ptr_eq(self.1, &key.scope)
    }
}

impl MethodKey<'static> {
    /// Creates a new owned method key (allocates three `JavaString`s).
    pub fn new(
        class_name: impl Into<JavaString>,
        method_name: impl Into<JavaString>,
        descriptor: impl Into<JavaString>,
    ) -> Self {
        Self {
            class_name: Cow::Owned(class_name.into()),
            method_name: Cow::Owned(method_name.into()),
            descriptor: Cow::Owned(descriptor.into()),
            scope: Cow::Borrowed(&[]),
        }
    }
}

impl<'a> MethodKey<'a> {
    /// Creates a method key by borrowing from `&JavaStr` references.
    #[must_use]
    pub fn borrowed(
        class_name: &'a JavaStr,
        method_name: &'a JavaStr,
        descriptor: &'a JavaStr,
    ) -> Self {
        Self {
            class_name: Cow::Borrowed(class_name),
            method_name: Cow::Borrowed(method_name),
            descriptor: Cow::Borrowed(descriptor),
            scope: Cow::Borrowed(&[]),
        }
    }

    /// Converts this key into an owned `MethodKey<'static>` suitable for cache storage.
    /// Only allocates when the `Cow` fields are borrowed; owned fields are moved.
    #[must_use]
    pub fn into_owned(self) -> MethodKey<'static> {
        MethodKey {
            class_name: Cow::Owned(self.class_name.into_owned()),
            method_name: Cow::Owned(self.method_name.into_owned()),
            descriptor: Cow::Owned(self.descriptor.into_owned()),
            scope: Cow::Owned(self.scope.into_owned()),
        }
    }
}

/// Verification cache for reusing method results.
///
/// Results are scoped to exact class, method, configuration, and context identities
/// so repeated verification can safely reuse them.
#[derive(Debug, Default)]
pub struct VerificationCache {
    /// Whether caching is enabled.
    enabled: bool,

    /// Cached verification results keyed by names and an interned scope identity.
    ///
    /// Uses `hashbrown::HashMap` with `Equivalent` trait to allow
    /// zero-allocation lookups using a borrowed `MethodKey<'_>`.
    results: RwLock<hashbrown::HashMap<CachedMethodKey, CachedResult, ahash::RandomState>>,

    /// Scope interning compares complete bytes before sharing an identity.
    scopes: RwLock<AHashSet<CacheScope>>,

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
}

impl VerificationCache {
    /// Creates a new verification cache.
    #[must_use]
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            scopes: RwLock::new(AHashSet::new()),
            results: RwLock::new(hashbrown::HashMap::with_hasher(
                ahash::RandomState::default(),
            )),
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

    /// Intern complete scope bytes; hash collisions never establish identity.
    pub(crate) fn intern_scope(&self, bytes: Vec<u8>) -> CacheScope {
        if let Ok(mut scopes) = self.scopes.write() {
            if let Some(scope) = scopes.get(bytes.as_slice()) {
                return Arc::clone(scope);
            }
            let scope: CacheScope = bytes.into();
            scopes.insert(Arc::clone(&scope));
            return scope;
        }
        bytes.into()
    }

    /// Gets a cached result using exact scope bytes from a public method key.
    #[must_use]
    pub fn get_result(&self, key: &MethodKey<'_>) -> Option<CachedResult> {
        if !self.enabled {
            return None;
        }
        let scope = self.scopes.read().ok()?.get(key.scope.as_ref()).cloned();
        let Some(scope) = scope else {
            self.record_result_lookup(false);
            return None;
        };
        self.get_result_scoped(key, &scope)
    }

    pub(crate) fn get_result_scoped(
        &self,
        key: &MethodKey<'_>,
        scope: &CacheScope,
    ) -> Option<CachedResult> {
        if !self.enabled {
            return None;
        }
        let guard = self.results.read().ok()?;
        let result = guard.get(&MethodKeyLookup(key, scope)).cloned();
        drop(guard);
        self.record_result_lookup(result.is_some());
        result
    }

    /// Stores a result, sharing an exact interned scope with other methods.
    pub fn put_result(&self, key: &MethodKey<'_>, result: CachedResult) {
        if !self.enabled {
            return;
        }
        let scope = self.intern_scope(key.scope.to_vec());
        self.put_result_scoped(key, &scope, result);
    }

    pub(crate) fn put_result_scoped(
        &self,
        key: &MethodKey<'_>,
        scope: &CacheScope,
        result: CachedResult,
    ) {
        if !self.enabled {
            return;
        }
        let owned_key = CachedMethodKey {
            class_name: key.class_name.clone().into_owned(),
            method_name: key.method_name.clone().into_owned(),
            descriptor: key.descriptor.clone().into_owned(),
            scope: Arc::clone(scope),
        };
        if let Ok(mut guard) = self.results.write() {
            guard.insert(owned_key, result);
        }
    }

    fn record_result_lookup(&self, hit: bool) {
        let Ok(mut stats) = self.stats.write() else {
            return;
        };
        if hit {
            stats.result_hits += 1;
        } else {
            stats.result_misses += 1;
        }
    }

    /// Gets cache statistics.
    #[must_use]
    pub fn stats(&self) -> CacheStats {
        self.stats.read().map(|s| s.clone()).unwrap_or_default()
    }

    /// Clears all cached data.
    pub fn clear(&self) {
        if let Ok(mut scopes) = self.scopes.write() {
            scopes.clear();
        }
        if let Ok(mut guard) = self.results.write() {
            guard.clear();
        }
        if let Ok(mut stats) = self.stats.write() {
            *stats = CacheStats::default();
        }
    }

    /// Returns the number of cached results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.read().map_or(0, |g| g.len())
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
        cache.put_result(&key, CachedResult::Success);

        // Should not cache when disabled
        assert!(cache.get_result(&key).is_none());
    }

    #[test]
    fn test_cache_enabled() {
        let cache = VerificationCache::new(true);
        assert!(cache.is_enabled());

        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result(&key, CachedResult::Success);

        let result = cache.get_result(&key);
        assert!(matches!(result, Some(CachedResult::Success)));
    }

    #[test]
    fn replacing_results_preserves_exact_scope_isolation() {
        let cache = VerificationCache::new(true);
        let mut key = MethodKey::new("Test", "foo", "()V");
        key.scope = Cow::Owned(vec![1, 2, 3]);
        cache.put_result(&key, CachedResult::Success);
        cache.put_result(&key, CachedResult::Failed("changed result".to_string()));
        assert_eq!(cache.result_count(), 1);
        assert!(matches!(
            cache.get_result(&key),
            Some(CachedResult::Failed(message)) if message == "changed result"
        ));

        let mut other = key.clone();
        other.scope = Cow::Owned(vec![1, 2, 4]);
        cache.put_result(&other, CachedResult::Success);
        assert_eq!(cache.result_count(), 2);
        assert!(matches!(
            cache.get_result(&other),
            Some(CachedResult::Success)
        ));
        assert!(matches!(
            cache.get_result(&key),
            Some(CachedResult::Failed(_))
        ));
    }

    #[test]
    fn disabled_cache_ignores_scoped_results() {
        let cache = VerificationCache::disabled();
        let scope = cache.intern_scope(vec![1, 2, 3]);
        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result_scoped(&key, &scope, CachedResult::Success);
        assert!(cache.get_result_scoped(&key, &scope).is_none());
        assert_eq!(cache.result_count(), 0);
    }

    #[test]
    fn test_cache_clear() {
        let cache = VerificationCache::new(true);

        let key = MethodKey::new("Test", "foo", "()V");
        cache.put_result(&key, CachedResult::Success);

        assert_eq!(cache.result_count(), 1);

        cache.clear();

        assert_eq!(cache.result_count(), 0);
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

        let stats = cache.stats();
        assert_eq!(stats.result_misses, 1);
        assert_eq!(stats.result_hits, 0);
    }

    #[cfg(panic = "unwind")]
    #[test]
    fn test_cache_stats_poisoned_lock_is_ignored() {
        let cache = VerificationCache::new(true);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = cache.stats.write().unwrap();
            panic!("poison stats lock");
        }));
        assert!(result.is_err());

        cache.record_result_lookup(true);
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
    #[test]
    fn exact_scopes_are_shared_and_clear_does_not_reuse_live_identities() {
        let cache = VerificationCache::new(true);
        let first = cache.intern_scope(vec![1, 2, 3]);
        let equal = cache.intern_scope(vec![1, 2, 3]);
        let different = cache.intern_scope(vec![1, 2, 4]);
        assert!(Arc::ptr_eq(&first, &equal));
        assert!(!Arc::ptr_eq(&first, &different));
        cache.clear();
        let after_clear = cache.intern_scope(vec![1, 2, 3]);
        assert!(!Arc::ptr_eq(&first, &after_clear));
    }
}
