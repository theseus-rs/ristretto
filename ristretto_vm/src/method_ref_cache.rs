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
//! - Fast "steady state" execution without module checks per call
//! - Correct error semantics (`IllegalAccessError` at resolution, not at call)
//! - Compliance with JVM specification behavior

use crate::Error::InternalError;
use crate::JavaError::IllegalAccessError;
use crate::reference_cache::ReferenceCache;
use ristretto_classfile::{FieldType, JavaStr};
use ristretto_classloader::{Class, Method, POLYMORPHIC_METHODS};
use std::sync::{Arc, OnceLock};

/// The kind of method invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeKind {
    /// invokestatic; static method call
    Static,
    /// invokespecial; constructor, private, or super call
    Special,
    /// invokevirtual; virtual method dispatch
    Virtual,
    /// invokeinterface; interface method dispatch
    Interface,
}

/// A successfully resolved method reference.
///
/// This contains all information needed to invoke the method without
/// re-resolving or re-checking access.
#[derive(Debug)]
pub struct ResolvedMethodRef {
    /// Symbolic target, which can differ from the method declaring class.
    pub referenced_class: Arc<Class>,
    /// Checked receiver targets, published only after successful dispatch.
    pub dispatch: ReceiverCache,
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
        referenced_class: Arc<Class>,
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
            let d = JavaStr::cow_from_str(&method_descriptor);
            match FieldType::parse_method_descriptor(&d).ok() {
                Some((params, return_type)) => (params.len(), return_type.is_some()),
                // Fallback to method's declared parameters if parsing fails
                None => (method.parameters().len(), method.return_type().is_some()),
            }
        } else {
            (method.parameters().len(), method.return_type().is_some())
        };

        Self {
            referenced_class,
            dispatch: ReceiverCache::default(),
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

/// Each invocation kind is validated independently, even when bytecodes share a CP index.
#[derive(Debug, Default)]
pub struct MethodRefEntry {
    r#static: OnceLock<Arc<ResolvedMethodRef>>,
    special: OnceLock<Arc<ResolvedMethodRef>>,
    r#virtual: OnceLock<Arc<ResolvedMethodRef>>,
    interface: OnceLock<Arc<ResolvedMethodRef>>,
}

impl MethodRefEntry {
    fn slot(&self, kind: InvokeKind) -> &OnceLock<Arc<ResolvedMethodRef>> {
        match kind {
            InvokeKind::Static => &self.r#static,
            InvokeKind::Special => &self.special,
            InvokeKind::Virtual => &self.r#virtual,
            InvokeKind::Interface => &self.interface,
        }
    }

    pub fn get(&self, kind: InvokeKind) -> Option<&Arc<ResolvedMethodRef>> {
        self.slot(kind).get()
    }

    pub fn store(&self, resolved: Arc<ResolvedMethodRef>) -> Arc<ResolvedMethodRef> {
        self.slot(resolved.invoke_kind)
            .get_or_init(|| resolved)
            .clone()
    }
}

/// VM-owned tables retain caller classes so addresses cannot be reused while cached.
pub(crate) type MethodRefCache = ReferenceCache<MethodRefEntry>;

/// A receiver target whose invocation checks have already succeeded.
#[derive(Debug, Clone)]
pub struct ReceiverTarget {
    pub receiver_class: Arc<Class>,
    pub class: Arc<Class>,
    pub method: Arc<Method>,
}

/// The first slot is monomorphic; three additional slots cover small polymorphic sites.
/// Once full, misses use normal lookup without replacing or growing the cache.
#[derive(Debug, Default)]
pub struct ReceiverCache {
    targets: [OnceLock<ReceiverTarget>; 4],
}

impl ReceiverCache {
    pub fn get(&self, receiver_class: &Arc<Class>) -> Option<&ReceiverTarget> {
        for slot in &self.targets {
            let target = slot.get()?;
            if Arc::ptr_eq(&target.receiver_class, receiver_class) {
                return Some(target);
            }
        }
        None
    }

    pub fn store(&self, mut target: ReceiverTarget) {
        for slot in &self.targets {
            if let Some(cached) = slot.get() {
                if Arc::ptr_eq(&cached.receiver_class, &target.receiver_class) {
                    return;
                }
                continue;
            }
            match slot.set(target) {
                Ok(()) => return,
                Err(value) => {
                    // Another thread may just have published this same receiver.
                    if slot.get().is_some_and(|cached| {
                        Arc::ptr_eq(&cached.receiver_class, &value.receiver_class)
                    }) {
                        return;
                    }
                    target = value;
                }
            }
        }
    }
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
