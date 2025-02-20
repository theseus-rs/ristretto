use crate::Result;
use crate::native_methods::registry::{JAVA_8, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/SecurityManager";

/// Register all native methods for `java.lang.SecurityManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "classDepth",
            "(Ljava/lang/String;)I",
            class_depth,
        );
        registry.register(CLASS_NAME, "classLoaderDepth0", "()I", class_loader_depth_0);
        registry.register(
            CLASS_NAME,
            "currentClassLoader0",
            "()Ljava/lang/ClassLoader;",
            current_class_loader_0,
        );
        registry.register(
            CLASS_NAME,
            "currentLoadedClass0",
            "()Ljava/lang/Class;",
            current_loaded_class_0,
        );
    }

    if registry.java_major_version() <= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "getClassContext",
            "()[Ljava/lang/Class;",
            get_class_context,
        );
    }
}

#[async_recursion(?Send)]
async fn class_depth(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classDepth(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn class_loader_depth_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classLoaderDepth0()I")
}

#[async_recursion(?Send)]
async fn current_class_loader_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;")
}

#[async_recursion(?Send)]
async fn current_loaded_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_context(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classDepth(Ljava/lang/String;)I"
    )]
    async fn test_class_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_depth(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classLoaderDepth0()I"
    )]
    async fn test_class_loader_depth_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_loader_depth_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_current_class_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_class_loader_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;"
    )]
    async fn test_current_loaded_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_loaded_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;"
    )]
    async fn test_get_class_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_context(thread, Parameters::default()).await;
    }
}
