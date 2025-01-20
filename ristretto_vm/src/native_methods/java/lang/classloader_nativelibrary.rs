use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ClassLoader$NativeLibrary";

/// Register all native methods for `java.lang.ClassLoader$NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "unload", "(Ljava/lang/String;Z)V", unload);
        registry.register(CLASS_NAME, "find", "(Ljava/lang/String;)J", find);
        registry.register(CLASS_NAME, "load", "(Ljava/lang/String;Z)V", load);
    } else {
        registry.register(CLASS_NAME, "findEntry", "(Ljava/lang/String;)J", find_entry);
        registry.register(CLASS_NAME, "load0", "(Ljava/lang/String;ZZ)Z", load_0);
        registry.register(CLASS_NAME, "unload", "(Ljava/lang/String;ZJ)V", unload);
    }
}

#[async_recursion(?Send)]
async fn find(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn find_entry(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn load(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z")
}

#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J"
    )]
    async fn test_find() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J"
    )]
    async fn test_find_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V"
    )]
    async fn test_load() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z"
    )]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V"
    )]
    async fn test_unload() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload(thread, Parameters::default()).await;
    }
}
