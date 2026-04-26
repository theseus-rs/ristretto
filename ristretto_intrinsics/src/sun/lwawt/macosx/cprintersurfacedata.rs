use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterSurfaceData._flush()V", Any)]
#[async_method]
pub async fn flush<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterSurfaceData._flush()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
    Any
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _j_graphics_state_object = parameters.pop_reference()?;
    let _j_graphics_state = parameters.pop_reference()?;
    let _ns_ref = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flush() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterSurfaceData._flush()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterSurfaceData.initOps(JLjava/nio/ByteBuffer;[Ljava/lang/Object;II)V",
            result.unwrap_err().to_string()
        );
    }
}
