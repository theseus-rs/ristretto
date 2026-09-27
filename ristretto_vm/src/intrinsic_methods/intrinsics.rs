//! Compiler-confirmed registries shared by VMs selecting the same release.
use crate::Thread;
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_types::{IntrinsicRegistry, Result};
use std::sync::LazyLock;

macro_rules! registry {
    ($name:ident, $version:ident) => {
        pub(crate) static $name: LazyLock<Result<IntrinsicRegistry<Thread>>> =
            LazyLock::new(|| {
                let mut registry = IntrinsicRegistry::new(&$version);
                ristretto_intrinsics::register(&mut registry)?;
                Ok(registry)
            });
    };
}
registry!(JAVA_8_REGISTRY, JAVA_8);
registry!(JAVA_11_REGISTRY, JAVA_11);
registry!(JAVA_17_REGISTRY, JAVA_17);
registry!(JAVA_21_REGISTRY, JAVA_21);
registry!(JAVA_25_REGISTRY, JAVA_25);
