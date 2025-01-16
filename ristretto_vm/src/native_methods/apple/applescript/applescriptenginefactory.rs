use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/applescript/AppleScriptEngineFactory";

/// Register all native methods for `apple.applescript.AppleScriptEngineFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
