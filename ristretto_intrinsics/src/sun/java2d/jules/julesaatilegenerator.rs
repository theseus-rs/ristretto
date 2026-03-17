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
    "sun/java2d/jules/JulesAATileGenerator.freePixmanImgPtr(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_pixman_img_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/jules/JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn rasterize_trapezoids_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_pixman_img_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_pixman_img_ptr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rasterize_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = rasterize_trapezoids_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
