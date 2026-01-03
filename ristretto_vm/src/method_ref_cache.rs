//! Method reference cache for invoke instructions.
//!
//! This module provides a thread-safe cache for resolved method references. JPMS access checks
//! are performed at resolution time (once per method ref), not at each invocation.
//!
//! # JPMS Enforcement
//!
//! JPMS (Java Platform Module System) access is enforced at **resolution/link time**, not per
//! invoke instruction execution. When a method reference is first resolved:
//!
//! 1. The declaring class is loaded and resolved
//! 2. **JPMS gates are checked** (readability + exports) before member access checks
//! 3. Normal Java access rules are applied (public/protected/package/private)
//! 4. The resolved method is cached for subsequent invocations
//!
//! This ensures:
//! - Fast "steady state" execution with no locking or module checks per call
//! - Correct error semantics (`IllegalAccessError` at resolution, not at call)
//! - Compliance with JVM specification behavior

use crate::Error::InternalError;
use crate::JavaError::IllegalAccessError;
use crate::Result;
use dashmap::DashMap;
use ristretto_classfile::FieldType;
use ristretto_classloader::{Class, Method, POLYMORPHIC_METHODS};
use std::sync::Arc;

/// Unique identifier for a method reference in the constant pool.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodRefKey {
    /// The class containing the invoke instruction (caller class).
    pub caller_class: String,
    /// Constant pool index of the method/interface method ref.
    pub cp_index: u16,
}

impl MethodRefKey {
    /// Creates a new method ref key.
    #[must_use]
    pub fn new(caller_class: String, cp_index: u16) -> Self {
        Self {
            caller_class,
            cp_index,
        }
    }
}

/// The kind of method invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeKind {
    /// invokestatic - static method call
    Static,
    /// invokespecial - constructor, private, or super call
    Special,
    /// invokevirtual - virtual method dispatch
    Virtual,
    /// invokeinterface - interface method dispatch
    Interface,
}

/// A successfully resolved method reference.
///
/// This contains all information needed to invoke the method without
/// re-resolving or re-checking access.
#[derive(Debug, Clone)]
pub struct ResolvedMethodRef {
    /// The class that declares the method.
    pub declaring_class: Arc<Class>,
    /// The resolved method.
    pub method: Arc<Method>,
    /// The kind of invocation.
    pub invoke_kind: InvokeKind,
    /// Method name (cached for error messages).
    pub method_name: String,
    /// Method descriptor (cached for error messages).
    pub method_descriptor: String,
    /// Whether this is a polymorphic method (e.g., `MethodHandle.invoke`). Cached to avoid
    /// `HashMap` lookup at invocation time.
    pub is_polymorphic: bool,
    /// Number of parameters to pop from the operand stack. For polymorphic methods, this is
    /// computed from the call site descriptor. For regular methods, this is the method's declared
    /// parameter count.
    pub param_count: usize,
    /// Whether the method has a return value to push onto the operand stack. For polymorphic
    /// methods, this is computed from the call site descriptor. For regular methods, this is
    /// whether the method has a return type.
    pub has_return_type: bool,
}

impl ResolvedMethodRef {
    /// Creates a new resolved method reference.
    ///
    /// Computes and caches polymorphic method information to avoid runtime lookups.
    ///
    /// # Arguments
    ///
    /// * `declaring_class` - The class that declares the method
    /// * `method` - The resolved method
    /// * `invoke_kind` - The kind of invocation
    /// * `method_descriptor` - The call site descriptor (may differ from method's for polymorphic methods)
    #[must_use]
    pub fn new(
        declaring_class: Arc<Class>,
        method: Arc<Method>,
        invoke_kind: InvokeKind,
        method_descriptor: String,
    ) -> Self {
        let method_name = method.name().to_string();

        // Check if this is a polymorphic method (cached lookup)
        let is_polymorphic = POLYMORPHIC_METHODS
            .get(&(declaring_class.name(), method.name()))
            .is_some();

        // Compute param_count and has_return_type once during resolution
        let (param_count, has_return_type) = if is_polymorphic {
            // For polymorphic methods, parse the call site descriptor
            match FieldType::parse_method_descriptor(&method_descriptor) {
                Ok((params, return_type)) => (params.len(), return_type.is_some()),
                // Fallback to method's declared parameters if parsing fails
                Err(_) => (method.parameters().len(), method.return_type().is_some()),
            }
        } else {
            (method.parameters().len(), method.return_type().is_some())
        };

        Self {
            declaring_class,
            method,
            invoke_kind,
            method_name,
            method_descriptor,
            is_polymorphic,
            param_count,
            has_return_type,
        }
    }
}

/// Resolution state for a method reference.
#[derive(Debug, Clone)]
pub enum MethodRefState {
    /// Resolution is currently in progress (for detecting recursion).
    Resolving,
    /// Successfully resolved.
    Resolved(ResolvedMethodRef),
    /// Resolution failed with an error.
    Failed(MethodRefError),
}

/// Cached error information for failed method resolution.
#[derive(Debug, Clone)]
pub struct MethodRefError {
    /// The error kind.
    pub kind: MethodRefErrorKind,
    /// Descriptive error message.
    pub message: String,
}

/// Kinds of method resolution errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodRefErrorKind {
    /// JPMS: Module does not read target module.
    ModuleNotReadable,
    /// JPMS: Package is not exported.
    PackageNotExported,
    /// Java access: Member not accessible (private/protected/package).
    MemberNotAccessible,
    /// Method not found.
    NoSuchMethod,
    /// Class/interface mismatch.
    IncompatibleClassChange,
    /// Other internal error.
    InternalError,
}

impl MethodRefError {
    /// Creates a new method ref error.
    #[must_use]
    pub fn new(kind: MethodRefErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    /// Converts this error to a VM error.
    pub fn to_vm_error(&self) -> crate::Error {
        use crate::JavaError::{IncompatibleClassChangeError, NoSuchMethodError};

        match self.kind {
            MethodRefErrorKind::ModuleNotReadable
            | MethodRefErrorKind::PackageNotExported
            | MethodRefErrorKind::MemberNotAccessible => {
                IllegalAccessError(self.message.clone()).into()
            }
            MethodRefErrorKind::NoSuchMethod => NoSuchMethodError(self.message.clone()).into(),
            MethodRefErrorKind::IncompatibleClassChange => {
                IncompatibleClassChangeError(self.message.clone()).into()
            }
            MethodRefErrorKind::InternalError => InternalError(self.message.clone()),
        }
    }
}

/// Thread-safe cache for method reference resolution.
///
/// This cache stores the resolution state of method references from the constant pool.
/// Resolution happens once per method ref, and the result (success or failure) is cached.
///
/// # Thread Safety
///
/// The cache uses `DashMap` for lock-free concurrent access. Multiple threads can
/// resolve different method refs concurrently. If two threads try to resolve the
/// same method ref, one will win and the other will use the cached result.
#[derive(Debug)]
pub struct MethodRefCache {
    /// Maps method ref keys to their resolution states.
    states: DashMap<MethodRefKey, MethodRefState>,
}

impl MethodRefCache {
    /// Creates a new empty method ref cache.
    #[must_use]
    pub fn new() -> Self {
        Self {
            states: DashMap::new(),
        }
    }

    /// Gets a cached resolved method ref, if available.
    ///
    /// # Returns
    ///
    /// - `Some(Ok(resolved))` if the method was successfully resolved
    /// - `Some(Err(error))` if resolution previously failed
    /// - `None` if the method has not been resolved yet
    #[must_use]
    pub fn get(&self, key: &MethodRefKey) -> Option<Result<ResolvedMethodRef>> {
        self.states.get(key).and_then(|state| match &*state {
            MethodRefState::Resolving => None, // Still in progress
            MethodRefState::Resolved(resolved) => Some(Ok(resolved.clone())),
            MethodRefState::Failed(error) => Some(Err(error.to_vm_error())),
        })
    }

    /// Marks a method ref as being resolved (for recursion detection).
    ///
    /// # Returns
    ///
    /// - `true` if the method was successfully marked as resolving
    /// - `false` if the method is already being resolved (recursion detected)
    pub fn mark_resolving(&self, key: MethodRefKey) -> bool {
        use dashmap::mapref::entry::Entry;

        match self.states.entry(key) {
            Entry::Occupied(entry) => !matches!(entry.get(), MethodRefState::Resolving),
            Entry::Vacant(entry) => {
                entry.insert(MethodRefState::Resolving);
                true
            }
        }
    }

    /// Stores a successful resolution result.
    pub fn store_resolved(&self, key: MethodRefKey, resolved: ResolvedMethodRef) {
        self.states.insert(key, MethodRefState::Resolved(resolved));
    }

    /// Stores a failed resolution result.
    pub fn store_failed(&self, key: MethodRefKey, error: MethodRefError) {
        self.states.insert(key, MethodRefState::Failed(error));
    }

    /// Removes a method ref from the cache (e.g., after failed resolution that should be retried).
    pub fn remove(&self, key: &MethodRefKey) {
        self.states.remove(key);
    }

    /// Clears all cached method refs.
    pub fn clear(&self) {
        self.states.clear();
    }

    /// Gets the number of cached method refs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Checks if the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// Resolves a method reference with caching.
    ///
    /// This is the main entry point for method resolution. It:
    /// 1. Checks if the method is already cached (returns cached result)
    /// 2. Marks the method as resolving (for recursion detection)
    /// 3. Calls the resolver function
    /// 4. Caches and returns the result
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for the method reference
    /// * `resolver` - Async function that performs the actual resolution
    ///
    /// # Returns
    ///
    /// The resolved method reference, or an error if resolution failed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Recursive method resolution is detected
    /// - The resolver function fails
    pub async fn resolve_with_cache<F, Fut>(
        &self,
        key: MethodRefKey,
        resolve_fn: F,
    ) -> Result<ResolvedMethodRef>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<ResolvedMethodRef>>,
    {
        // Check current state
        if let Some(state) = self.states.get(&key) {
            return match &*state {
                MethodRefState::Resolving => Err(InternalError(format!(
                    "Recursive method resolution detected for class '{}' at index {}",
                    key.caller_class, key.cp_index
                ))),
                MethodRefState::Resolved(cached) => Ok(cached.clone()),
                MethodRefState::Failed(error) => Err(error.to_vm_error()),
            };
        }

        // Mark as resolving
        self.states.insert(key.clone(), MethodRefState::Resolving);

        // Perform resolution
        let result = resolve_fn().await;

        // Update cache based on result
        match &result {
            Ok(method_ref) => {
                self.states
                    .insert(key, MethodRefState::Resolved(method_ref.clone()));
            }
            Err(error) => {
                // Convert to cached error
                let cached_error =
                    MethodRefError::new(MethodRefErrorKind::InternalError, error.to_string());
                self.states
                    .insert(key, MethodRefState::Failed(cached_error));
            }
        }

        result
    }
}

impl Default for MethodRefCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_ref_key_equality() {
        let key1 = MethodRefKey::new("com/example/Test".to_string(), 10);
        let key2 = MethodRefKey::new("com/example/Test".to_string(), 10);
        let key3 = MethodRefKey::new("com/example/Test".to_string(), 11);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_method_ref_cache_new() {
        let cache = MethodRefCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_method_ref_cache_get_nonexistent() {
        let cache = MethodRefCache::new();
        let key = MethodRefKey::new("Test".to_string(), 1);
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_method_ref_error_kinds() {
        let error = MethodRefError::new(
            MethodRefErrorKind::ModuleNotReadable,
            "test error".to_string(),
        );
        let vm_error = error.to_vm_error();
        assert!(format!("{vm_error:?}").contains("IllegalAccessError"));

        let error = MethodRefError::new(
            MethodRefErrorKind::NoSuchMethod,
            "method not found".to_string(),
        );
        let vm_error = error.to_vm_error();
        assert!(format!("{vm_error:?}").contains("NoSuchMethodError"));
    }

    #[test]
    fn test_invoke_kind() {
        assert_eq!(InvokeKind::Static, InvokeKind::Static);
        assert_ne!(InvokeKind::Static, InvokeKind::Virtual);
    }

    #[test]
    fn test_mark_resolving() {
        let cache = MethodRefCache::new();
        let key = MethodRefKey::new("Test".to_string(), 1);

        // First mark should succeed
        assert!(cache.mark_resolving(key.clone()));

        // Cache should now have the entry
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_store_failed() {
        let cache = MethodRefCache::new();
        let key = MethodRefKey::new("Test".to_string(), 1);
        let error = MethodRefError::new(
            MethodRefErrorKind::NoSuchMethod,
            "Method not found".to_string(),
        );

        cache.store_failed(key.clone(), error);

        let result = cache.get(&key);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_remove() {
        let cache = MethodRefCache::new();
        let key = MethodRefKey::new("Test".to_string(), 1);
        let error = MethodRefError::new(
            MethodRefErrorKind::NoSuchMethod,
            "Method not found".to_string(),
        );

        cache.store_failed(key.clone(), error);
        assert_eq!(cache.len(), 1);

        cache.remove(&key);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_clear() {
        let cache = MethodRefCache::new();
        let key1 = MethodRefKey::new("Test1".to_string(), 1);
        let key2 = MethodRefKey::new("Test2".to_string(), 2);
        let error = MethodRefError::new(
            MethodRefErrorKind::NoSuchMethod,
            "Method not found".to_string(),
        );

        cache.store_failed(key1, error.clone());
        cache.store_failed(key2, error);
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert!(cache.is_empty());
    }

    #[tokio::test]
    async fn test_resolve_with_cache_caches_failure() {
        let cache = MethodRefCache::new();
        let key = MethodRefKey::new("Test".to_string(), 1);

        // First resolution fails
        let result = cache
            .resolve_with_cache(key.clone(), || async {
                Err(crate::JavaError::NoSuchMethodError("test".to_string()).into())
            })
            .await;
        assert!(result.is_err());

        // Subsequent resolution should return cached error without calling resolver
        let result = cache
            .resolve_with_cache(key, || async {
                panic!("Resolver should not be called for cached failure")
            })
            .await;
        assert!(result.is_err());
    }
}
