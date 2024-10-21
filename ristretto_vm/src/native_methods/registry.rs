use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::{
    java_io_filedescriptor, java_io_fileinputstream, java_io_fileoutputstream,
    java_io_unixfilesystem, java_io_winntfilesystem, java_lang_class, java_lang_classloader,
    java_lang_double, java_lang_float, java_lang_object, java_lang_ref_reference,
    java_lang_runtime, java_lang_shutdown, java_lang_stacktraceelement, java_lang_system,
    java_lang_thread, java_lang_throwable, jdk_internal_loader_bootloader,
    jdk_internal_loader_nativelibraries, jdk_internal_misc_cds,
    jdk_internal_misc_scopedmemoryaccess, jdk_internal_misc_signal, jdk_internal_misc_unsafe,
    jdk_internal_misc_vm, jdk_internal_reflect_reflection, jdk_internal_util_systemprops_raw,
    sun_io_win32errormode, sun_nio_fs_unixnativedispatcher,
};
use crate::Result;
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};
use tracing::debug;

/// Lazy static reference to the registry.
pub fn registry() -> &'static MethodRegistry {
    static REGISTRY: OnceLock<MethodRegistry> = OnceLock::new();
    REGISTRY.get_or_init(MethodRegistry::default)
}

/// A Rust method is a method that is implemented in Rust and is called from Java code instead of
/// being implemented in Java byte code.
// pub type RustMethod = fn(
//     call_stack: Arc<CallStack>,
//     arguments: Arguments,
// ) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;
pub type RustMethod = fn(
    call_stack: Arc<CallStack>,
    arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;

#[expect(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct MethodRegistry {
    methods: HashMap<String, RustMethod>,
}

impl MethodRegistry {
    /// Create a new registry.
    #[must_use]
    pub fn new() -> Self {
        MethodRegistry {
            methods: HashMap::new(),
        }
    }

    /// Register a new Rust method.
    pub fn register(
        &mut self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: RustMethod,
    ) {
        self.methods.insert(
            format!("{class_name}.{method_name}{method_descriptor}"),
            method,
        );
    }

    /// Get a Rust method by class and method name.
    ///
    /// # Errors
    /// if the method is not found.
    pub fn get(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&RustMethod> {
        let method_signature = format!("{class_name}.{method_name}{method_descriptor}");
        self.methods.get(&method_signature)
    }
}

impl Default for MethodRegistry {
    fn default() -> Self {
        debug!("configuring default method registry");
        let mut registry = MethodRegistry::new();
        java_io_filedescriptor::register(&mut registry);
        java_io_fileinputstream::register(&mut registry);
        java_io_fileoutputstream::register(&mut registry);
        java_io_unixfilesystem::register(&mut registry);
        java_io_winntfilesystem::register(&mut registry);
        java_lang_class::register(&mut registry);
        java_lang_classloader::register(&mut registry);
        java_lang_double::register(&mut registry);
        java_lang_float::register(&mut registry);
        java_lang_object::register(&mut registry);
        java_lang_runtime::register(&mut registry);
        java_lang_system::register(&mut registry);
        java_lang_shutdown::register(&mut registry);
        java_lang_stacktraceelement::register(&mut registry);
        java_lang_thread::register(&mut registry);
        java_lang_throwable::register(&mut registry);
        java_lang_ref_reference::register(&mut registry);
        jdk_internal_loader_bootloader::register(&mut registry);
        jdk_internal_loader_nativelibraries::register(&mut registry);
        jdk_internal_misc_cds::register(&mut registry);
        jdk_internal_misc_scopedmemoryaccess::register(&mut registry);
        jdk_internal_misc_signal::register(&mut registry);
        jdk_internal_misc_unsafe::register(&mut registry);
        jdk_internal_misc_vm::register(&mut registry);
        jdk_internal_reflect_reflection::register(&mut registry);
        jdk_internal_util_systemprops_raw::register(&mut registry);
        sun_io_win32errormode::register(&mut registry);
        sun_nio_fs_unixnativedispatcher::register(&mut registry);
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(clippy::needless_pass_by_value)]
    fn test_none(
        _call_stack: Arc<CallStack>,
        _arguments: Arguments,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
        Box::pin(async move { Ok(None) })
    }

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new();
        registry.register("java.lang.Object", "hashCode", "()I", test_none);
        assert_eq!(registry.methods.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut registry = MethodRegistry::new();
        registry.register("java.lang.Object", "hashCode", "()I", test_none);

        let result = registry.get("java.lang.Object", "hashCode", "()I");
        assert!(result.is_some());

        let result = registry.get("foo", "hashCode", "()I");
        assert!(result.is_none());
    }
}
