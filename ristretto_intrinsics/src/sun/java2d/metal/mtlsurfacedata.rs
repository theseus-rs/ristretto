use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.clearWindow()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn clear_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.clearWindow()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.getMTLTexturePointer(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_mtl_texture_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initFlipBackbuffer(J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_flip_backbuffer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initRTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_r_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLSurfaceData.initTexture(JZII)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_texture<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clear_window(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_mtl_texture_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_mtl_texture_pointer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_flip_backbuffer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_r_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_r_texture(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_texture(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
