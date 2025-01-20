use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eawt/Application";

/// Register all native methods for `com.apple.eawt.Application`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeInitializeApplicationDelegate",
        "()V",
        native_initialize_application_delegate,
    );
}

#[async_recursion(?Send)]
async fn native_initialize_application_delegate(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_initialize_application_delegate() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_initialize_application_delegate(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
