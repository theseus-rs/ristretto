use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.loader.RawNativeLibraries`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/RawNativeLibraries";
    registry.register(
        class_name,
        "load0",
        "(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z",
        load_0,
    );
    registry.register(class_name, "unload0", "(Ljava/lang/String;J)V", unload_0);
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/loader/RawNativeLibraries";
        assert!(registry
            .method(class_name, "load0", "(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "unload0", "(Ljava/lang/String;J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z"
    )]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V"
    )]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_0(thread, Arguments::default()).await;
    }
}
