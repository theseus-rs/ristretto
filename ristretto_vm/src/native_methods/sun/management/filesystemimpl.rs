use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/FileSystemImpl";

/// Register all native methods for `sun.management.FileSystemImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "isAccessUserOnly0",
        "(Ljava/lang/String;)Z",
        is_access_user_only_0,
    );
}

#[async_recursion(?Send)]
async fn is_access_user_only_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z"
    )]
    async fn test_is_access_user_only_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_access_user_only_0(thread, Arguments::default()).await;
    }
}
