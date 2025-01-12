use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.jar.JarFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/jar/JarFile";
    registry.register(
        class_name,
        "getMetaInfEntryNames",
        "()[Ljava/lang/String;",
        get_meta_inf_entry_names,
    );
}

#[async_recursion(?Send)]
async fn get_meta_inf_entry_names(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/util/jar/JarFile";
        assert!(registry
            .method(class_name, "getMetaInfEntryNames", "()[Ljava/lang/String;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;"
    )]
    async fn test_get_meta_inf_entry_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_meta_inf_entry_names(thread, Arguments::default()).await;
    }
}
