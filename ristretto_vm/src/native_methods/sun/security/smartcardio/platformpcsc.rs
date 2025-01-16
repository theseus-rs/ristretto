use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/smartcardio/PlatformPCSC";

/// Register all native methods for `sun.security.smartcardio.PlatformPCSC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "initialize",
        "(Ljava/lang/String;)V",
        initialize,
    );
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V"
    )]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize(thread, Arguments::default()).await;
    }
}
