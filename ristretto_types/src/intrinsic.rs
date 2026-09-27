//! Typed intrinsic registration and duplicate validation.

use crate::{BoxFuture, Error, Parameters, Result};
use ahash::AHashMap;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Calling convention for a synchronous intrinsic.
pub type SyncIntrinsicMethod<T> = fn(Arc<T>, Parameters) -> Result<Option<Value>>;
/// Calling convention for a suspending intrinsic.
pub type AsyncIntrinsicMethod<T> =
    fn(Arc<T>, Parameters) -> BoxFuture<'static, Result<Option<Value>>>;

/// An intrinsic specialized for a particular VM thread implementation.
#[derive(Debug)]
pub enum IntrinsicMethod<T> {
    /// Executes without allocating a future.
    Sync(SyncIntrinsicMethod<T>),
    /// Boxes the future at the registry boundary.
    Async(AsyncIntrinsicMethod<T>),
}

impl<T> Copy for IntrinsicMethod<T> {}
impl<T> Clone for IntrinsicMethod<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Registration information for a single intrinsic declaration.
#[derive(Debug)]
pub struct IntrinsicMetadata {
    /// Fully qualified Java signature.
    pub signature: &'static str,
    /// Rust implementation path, for diagnostics.
    pub implementation: &'static str,
    /// Source file relative to the intrinsics crate.
    pub source: &'static str,
    /// Supported release bits: Java 8, 11, 17, 21, and 25, respectively.
    pub versions: u8,
}

impl IntrinsicMetadata {
    /// Whether this declaration applies to the supported release selected by the VM.
    #[must_use]
    pub fn supports(&self, version: &Version) -> bool {
        self.versions & intrinsic_version_bit(version) != 0
    }
}

/// Select the same supported release family as the VM runtime.
#[must_use]
pub fn intrinsic_version_bit(version: &Version) -> u8 {
    match version.major() {
        69.. => 16,
        65.. => 8,
        61.. => 4,
        55.. => 2,
        _ => 1,
    }
}

/// A checked registry containing only compiler-enabled entries for one Java release.
#[derive(Debug)]
pub struct IntrinsicRegistry<T> {
    version: Version,
    methods: AHashMap<&'static str, IntrinsicMethod<T>>,
    metadata: AHashMap<&'static str, &'static IntrinsicMetadata>,
}

impl<T> IntrinsicRegistry<T> {
    /// Create an empty registry for a supported release family.
    #[must_use]
    pub fn new(version: &Version) -> Self {
        Self {
            version: version.clone(),
            methods: AHashMap::default(),
            metadata: AHashMap::default(),
        }
    }

    /// Register a compiler-enabled implementation if it supports this release.
    ///
    /// # Errors
    /// Rejects a second active implementation for the same signature without replacing the first.
    pub fn register(
        &mut self,
        metadata: &'static IntrinsicMetadata,
        method: IntrinsicMethod<T>,
    ) -> Result<()> {
        if !metadata.supports(&self.version) {
            return Ok(());
        }
        if let Some(previous) = self.metadata.get(metadata.signature) {
            return Err(Error::InternalError(format!(
                "Duplicate intrinsic {}: {} ({}) and {} ({})",
                metadata.signature,
                previous.implementation,
                previous.source,
                metadata.implementation,
                metadata.source
            )));
        }
        self.methods.insert(metadata.signature, method);
        self.metadata.insert(metadata.signature, metadata);
        Ok(())
    }

    /// Compiler-confirmed implementations for the selected release.
    #[must_use]
    pub fn methods(&self) -> &AHashMap<&'static str, IntrinsicMethod<T>> {
        &self.methods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::JAVA_21;

    #[test]
    fn duplicate_registration_keeps_the_original() -> Result<()> {
        static META: IntrinsicMetadata = IntrinsicMetadata {
            signature: "pkg/Example.run()V",
            implementation: "run",
            source: "test.rs",
            versions: 31,
        };
        let mut registry = IntrinsicRegistry::<()>::new(&JAVA_21);
        registry.register(&META, IntrinsicMethod::Sync(|_, _| Ok(None)))?;
        assert!(
            registry
                .register(
                    &META,
                    IntrinsicMethod::Sync(|_, _| Err(Error::InternalError("replaced".into())))
                )
                .is_err()
        );
        let method = registry
            .methods()
            .get(META.signature)
            .expect("registered method");
        if let IntrinsicMethod::Sync(function) = method {
            assert_eq!(function(Arc::new(()), Parameters::default())?, None);
        } else {
            panic!("sync method");
        }
        assert_eq!(registry.methods().len(), 1);
        Ok(())
    }
}
