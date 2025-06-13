use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/CGLLayer.blitTexture(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn blit_texture(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.blitTexture(J)V");
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeCreateLayer()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_layer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.nativeCreateLayer()J");
}

#[intrinsic_method("sun/java2d/opengl/CGLLayer.nativeSetScale(JD)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_scale(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V");
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn validate(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.opengl.CGLLayer.blitTexture(J)V")]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = blit_texture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.nativeCreateLayer()J"
    )]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_layer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V"
    )]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_scale(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V"
    )]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Parameters::default()).await;
    }
}
