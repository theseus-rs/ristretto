use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/module/ModuleBootstrap";

/// Register all native methods for `jdk.internal.module.ModuleBootstrap`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "boot", "()Ljava/lang/ModuleLayer;", boot);
}

#[async_recursion(?Send)]
async fn boot(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // TODO: remove this method once the module system is implemented
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_boot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = boot(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
