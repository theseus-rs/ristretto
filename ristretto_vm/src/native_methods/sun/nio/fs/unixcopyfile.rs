use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.UnixCopyFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/UnixCopyFile";
    registry.register(class_name, "transfer", "(IIJ)V", transfer);
}

#[async_recursion(?Send)]
async fn transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixCopyFile.transfer(IIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/fs/UnixCopyFile";
        assert!(registry.method(class_name, "transfer", "(IIJ)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.UnixCopyFile.transfer(IIJ)V")]
    async fn test_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer(thread, Arguments::default()).await;
    }
}
