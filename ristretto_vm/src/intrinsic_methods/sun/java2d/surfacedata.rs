use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/SurfaceData.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn is_opaque_gray(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_is_opaque_gray() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_opaque_gray(thread, Parameters::default()).await;
    }
}
