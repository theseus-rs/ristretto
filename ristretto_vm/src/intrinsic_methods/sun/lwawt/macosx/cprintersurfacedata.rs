use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPrinterSurfaceData";

/// Register all intrinsic methods for `sun.lwawt.macosx.CPrinterSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "_flush", "()V", flush);
    registry.register(
        CLASS_NAME,
        "initOps",
        "(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn flush(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData._flush()V")
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterSurfaceData._flush()V"
    )]
    async fn test_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }
}
