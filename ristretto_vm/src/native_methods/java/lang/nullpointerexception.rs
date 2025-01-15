use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/NullPointerException";

/// Register all native methods for `java.lang.NullPointerException`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getExtendedNPEMessage",
        "()Ljava/lang/String;",
        get_extended_npe_message,
    );
}

#[async_recursion(?Send)]
async fn get_extended_npe_message(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;"
    )]
    async fn test_get_extended_npe_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_extended_npe_message(thread, Arguments::default()).await;
    }
}
