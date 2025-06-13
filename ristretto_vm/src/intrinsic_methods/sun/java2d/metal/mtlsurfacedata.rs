use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.clearWindow()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn clear_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.clearWindow()V");
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.getMTLTexturePointer(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_mtl_texture_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J");
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initFlipBackbuffer(J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_flip_backbuffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z");
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_ops(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V");
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initRTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_r_texture(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z");
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_texture(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.clearWindow()V")]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J")]
    async fn test_get_mtl_texture_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtl_texture_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z")]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_flip_backbuffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z")]
    async fn test_init_r_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_r_texture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z")]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_texture(thread, Parameters::default()).await;
    }
}
