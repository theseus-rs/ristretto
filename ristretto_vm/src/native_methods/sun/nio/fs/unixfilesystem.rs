use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.UnixFileSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/UnixFileSystem";
    registry.register(class_name, "bufferedCopy0", "(IIJIJ)V", buffered_copy_0);
}

#[async_recursion(?Send)]
async fn buffered_copy_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/fs/UnixFileSystem";
        assert!(registry
            .method(class_name, "bufferedCopy0", "(IIJIJ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V")]
    async fn test_buffered_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = buffered_copy_0(thread, Arguments::default()).await;
    }
}
