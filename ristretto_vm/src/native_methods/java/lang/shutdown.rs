use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Shutdown`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Shutdown";
    registry.register(class_name, "beforeHalt", "()V", before_halt);
    registry.register(class_name, "halt0", "(I)V", halt_0);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Shutdown";
        assert!(registry.method(class_name, "beforeHalt", "()V").is_some());
        assert!(registry.method(class_name, "halt0", "(I)V").is_some());
    }

    #[tokio::test]
    async fn test_before_halt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = before_halt(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
