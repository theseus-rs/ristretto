use crate::arguments::Arguments;
use crate::native_methods::{
    java_awt_component, java_awt_container, java_awt_frame, java_awt_toolkit, java_awt_window,
    java_io_filedescriptor, java_io_fileinputstream, java_io_fileoutputstream,
    java_io_unixfilesystem, java_io_winntfilesystem, java_lang_class, java_lang_classloader,
    java_lang_double, java_lang_float, java_lang_invoke_methodhandles, java_lang_object,
    java_lang_processenvironment, java_lang_processimpl, java_lang_ref_reference,
    java_lang_reflect_array, java_lang_runtime, java_lang_securitymanager, java_lang_shutdown,
    java_lang_stacktraceelement, java_lang_stringutf16, java_lang_system, java_lang_thread,
    java_lang_throwable, java_security_accesscontroller, jdk_internal_loader_bootloader,
    jdk_internal_loader_nativelibraries, jdk_internal_misc_cds, jdk_internal_misc_previewfeatures,
    jdk_internal_misc_scopedmemoryaccess, jdk_internal_misc_signal, jdk_internal_misc_unsafe,
    jdk_internal_misc_vm, jdk_internal_module_modulebootstrap, jdk_internal_reflect_reflection,
    jdk_internal_util_systemprops_raw, sun_io_win32errormode, sun_misc_unsafe, sun_misc_vm,
    sun_nio_fs_unixnativedispatcher, sun_reflect_reflection,
};
use crate::thread::Thread;
use crate::Result;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// A Rust method is a method that is implemented in Rust and is called from Java code instead of
/// being implemented in Java byte code.
pub type RustMethod = fn(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>>;

#[expect(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct MethodRegistry {
    java_version: Version,
    methods: HashMap<String, RustMethod>,
}

impl MethodRegistry {
    /// Create a new registry.
    pub fn new(java_version: Version) -> Self {
        let mut method_registry = MethodRegistry {
            java_version,
            methods: HashMap::new(),
        };

        java_awt_component::register(&mut method_registry);
        java_awt_container::register(&mut method_registry);
        java_awt_frame::register(&mut method_registry);
        java_awt_toolkit::register(&mut method_registry);
        java_awt_window::register(&mut method_registry);
        java_io_filedescriptor::register(&mut method_registry);
        java_io_fileinputstream::register(&mut method_registry);
        java_io_fileoutputstream::register(&mut method_registry);
        java_io_unixfilesystem::register(&mut method_registry);
        java_io_winntfilesystem::register(&mut method_registry);
        java_lang_class::register(&mut method_registry);
        java_lang_classloader::register(&mut method_registry);
        java_lang_double::register(&mut method_registry);
        java_lang_float::register(&mut method_registry);
        java_lang_invoke_methodhandles::register(&mut method_registry);
        java_lang_object::register(&mut method_registry);
        java_lang_processenvironment::register(&mut method_registry);
        java_lang_processimpl::register(&mut method_registry);
        java_lang_ref_reference::register(&mut method_registry);
        java_lang_reflect_array::register(&mut method_registry);
        java_lang_runtime::register(&mut method_registry);
        java_lang_securitymanager::register(&mut method_registry);
        java_lang_stringutf16::register(&mut method_registry);
        java_lang_system::register(&mut method_registry);
        java_lang_shutdown::register(&mut method_registry);
        java_lang_stacktraceelement::register(&mut method_registry);
        java_lang_thread::register(&mut method_registry);
        java_lang_throwable::register(&mut method_registry);
        java_security_accesscontroller::register(&mut method_registry);
        jdk_internal_loader_bootloader::register(&mut method_registry);
        jdk_internal_loader_nativelibraries::register(&mut method_registry);
        jdk_internal_misc_cds::register(&mut method_registry);
        jdk_internal_misc_previewfeatures::register(&mut method_registry);
        jdk_internal_misc_scopedmemoryaccess::register(&mut method_registry);
        jdk_internal_misc_signal::register(&mut method_registry);
        jdk_internal_misc_unsafe::register(&mut method_registry);
        jdk_internal_misc_vm::register(&mut method_registry);
        jdk_internal_module_modulebootstrap::register(&mut method_registry);
        jdk_internal_reflect_reflection::register(&mut method_registry);
        jdk_internal_util_systemprops_raw::register(&mut method_registry);
        sun_io_win32errormode::register(&mut method_registry);
        sun_misc_unsafe::register(&mut method_registry);
        sun_misc_vm::register(&mut method_registry);
        sun_nio_fs_unixnativedispatcher::register(&mut method_registry);
        sun_reflect_reflection::register(&mut method_registry);

        method_registry
    }

    /// Get the java version.
    pub fn java_version(&self) -> &Version {
        &self.java_version
    }

    /// Register a new Rust method.
    pub(crate) fn register(
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
    pub(crate) fn method(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&RustMethod> {
        let method_signature = format!("{class_name}.{method_name}{method_descriptor}");
        self.methods.get(&method_signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_recursion::async_recursion;

    #[expect(clippy::needless_pass_by_value)]
    #[async_recursion(?Send)]
    async fn test_none(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
        Ok(None)
    }

    #[tokio::test]
    async fn test_register() -> Result<()> {
        let mut method_registry = MethodRegistry::new(Version::Java21 { minor: 0 });
        let class_name = "java/lang/Object";
        let method_name = "foo";
        let method_descriptor = "()V";
        method_registry.register(class_name, method_name, method_descriptor, test_none);
        let result = method_registry.method(class_name, method_name, method_descriptor);
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method() -> Result<()> {
        let method_registry = MethodRegistry::new(Version::Java21 { minor: 0 });
        let result = method_registry.method("java/lang/Object", "hashCode", "()I");
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_found() -> Result<()> {
        let method_registry = MethodRegistry::new(Version::Java21 { minor: 0 });
        let result = method_registry.method("foo", "hashCode", "()I");
        assert!(result.is_none());
        Ok(())
    }
}
