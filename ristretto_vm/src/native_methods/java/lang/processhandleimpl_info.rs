use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ProcessHandleImpl$Info`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ProcessHandleImpl$Info";
    registry.register(class_name, "info0", "(J)V", info_0);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn info_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl$Info.info0(J)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/ProcessHandleImpl$Info";
        assert!(registry.method(class_name, "info0", "(J)V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ProcessHandleImpl$Info.info0(J)V")]
    async fn test_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = info_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
