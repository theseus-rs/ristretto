use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Shutdown";

/// Register all native methods for `java.lang.Shutdown`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "beforeHalt", "()V", before_halt);
    registry.register(CLASS_NAME, "halt0", "(I)V", halt_0);
}

#[async_recursion(?Send)]
async fn before_halt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn halt_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let code = arguments.pop_int()?;
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_before_halt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = before_halt(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
