use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_blit<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn update_bitmask<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_blit(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_bitmask() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = update_bitmask(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
