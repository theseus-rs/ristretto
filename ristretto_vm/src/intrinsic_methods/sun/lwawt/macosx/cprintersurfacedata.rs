use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterSurfaceData._flush()V", Any)]
#[async_method]
pub(crate) async fn flush(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterSurfaceData._flush()V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
    Any
)]
#[async_method]
pub(crate) async fn init_ops(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
