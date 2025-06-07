use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_elem(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I")
}

#[intrinsic_method(
    "sun/awt/image/DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_elem(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I"
    )]
    async fn test_get_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_elem(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V"
    )]
    async fn test_set_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_elem(thread, Parameters::default()).await;
    }
}
