use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/loader/RawNativeLibraries";

/// Register all intrinsic methods for `jdk.internal.loader.RawNativeLibraries`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "load0",
        "(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z",
        load_0,
    );
    registry.register(CLASS_NAME, "unload0", "(Ljava/lang/String;J)V", unload_0);
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z"
    )
}

#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z"
    )]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V"
    )]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_0(thread, Parameters::default()).await;
    }
}
