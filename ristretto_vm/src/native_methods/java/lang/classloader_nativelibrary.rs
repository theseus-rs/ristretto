use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.lang.ClassLoader$NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader$NativeLibrary";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "find", "(Ljava/lang/String;)J", find);
        registry.register(class_name, "load", "(Ljava/lang/String;Z)V", load);
    } else {
        registry.register(class_name, "findEntry", "(Ljava/lang/String;)J", find_entry);
        registry.register(class_name, "load0", "(Ljava/lang/String;ZZ)Z", load_0);
    }

    registry.register(class_name, "unload", "(Ljava/lang/String;ZJ)V", unload);
}

#[async_recursion(?Send)]
async fn find(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn find_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn load(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z")
}

#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/ClassLoader$NativeLibrary";
        assert!(registry
            .method(class_name, "findEntry", "(Ljava/lang/String;)J")
            .is_some());
        assert!(registry
            .method(class_name, "load0", "(Ljava/lang/String;ZZ)Z")
            .is_some());
        assert!(registry
            .method(class_name, "unload", "(Ljava/lang/String;ZJ)V")
            .is_some());
    }

    #[test]
    fn test_register_java_8() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/ClassLoader$NativeLibrary";
        assert!(registry
            .method(class_name, "find", "(Ljava/lang/String;)J")
            .is_some());
        assert!(registry
            .method(class_name, "load", "(Ljava/lang/String;Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.find(Ljava/lang/String;)J"
    )]
    async fn test_find() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J"
    )]
    async fn test_find_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V"
    )]
    async fn test_load() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z"
    )]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V"
    )]
    async fn test_unload() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload(thread, Arguments::default()).await;
    }
}
