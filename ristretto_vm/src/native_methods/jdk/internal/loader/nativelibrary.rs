use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/loader/NativeLibrary";

/// Register all native methods for `jdk.internal.loader.NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "findEntry0",
        "(JLjava/lang/String;)J",
        find_entry_0,
    );
}

#[async_recursion(?Send)]
async fn find_entry_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.NativeLibrary.findEntry0(JLjava/lang/String;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.NativeLibrary.findEntry0(JLjava/lang/String;)J"
    )]
    async fn test_find_entry_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry_0(thread, Arguments::default()).await;
    }
}
