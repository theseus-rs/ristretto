use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.lang.SecurityManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/SecurityManager";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "classDepth",
            "(Ljava/lang/String;)I",
            class_depth,
        );
        registry.register(class_name, "classLoaderDepth0", "()I", class_loader_depth_0);
        registry.register(
            class_name,
            "currentClassLoader0",
            "()Ljava/lang/ClassLoader;",
            current_class_loader_0,
        );
        registry.register(
            class_name,
            "currentLoadedClass0",
            "()Ljava/lang/Class;",
            current_loaded_class_0,
        );
    }

    registry.register(
        class_name,
        "getClassContext",
        "()[Ljava/lang/Class;",
        get_class_context,
    );
}

#[async_recursion(?Send)]
async fn class_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classDepth(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn class_loader_depth_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classLoaderDepth0()I")
}

#[async_recursion(?Send)]
async fn current_class_loader_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;")
}

#[async_recursion(?Send)]
async fn current_loaded_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/SecurityManager";
        assert!(registry
            .method(class_name, "classDepth", "(Ljava/lang/String;)I")
            .is_some());
        assert!(registry
            .method(class_name, "classLoaderDepth0", "()I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "currentClassLoader0",
                "()Ljava/lang/ClassLoader;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "currentLoadedClass0", "()Ljava/lang/Class;")
            .is_some());
        assert!(registry
            .method(class_name, "getClassContext", "()[Ljava/lang/Class;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classDepth(Ljava/lang/String;)I"
    )]
    async fn test_class_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_depth(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classLoaderDepth0()I"
    )]
    async fn test_class_loader_depth_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_loader_depth_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_current_class_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_class_loader_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;"
    )]
    async fn test_current_loaded_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_loaded_class_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;"
    )]
    async fn test_get_class_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_context(thread, Arguments::default()).await;
    }
}
