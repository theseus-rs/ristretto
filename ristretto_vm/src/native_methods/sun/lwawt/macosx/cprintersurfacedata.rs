use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPrinterSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPrinterSurfaceData";
    registry.register(class_name, "_flush", "()V", flush);
    registry.register(
        class_name,
        "initOps",
        "(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn flush(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData._flush()V")
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPrinterSurfaceData";
        assert!(registry.method(class_name, "_flush", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "initOps",
                "(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterSurfaceData._flush()V")]
    async fn test_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Arguments::default()).await;
    }
}
