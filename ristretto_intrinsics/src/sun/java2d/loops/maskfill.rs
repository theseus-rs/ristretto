use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V",
    Any
)]
#[async_method]
pub async fn draw_aa_pgram<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V",
    Any
)]
#[async_method]
pub async fn fill_aa_pgram<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V",
    Any
)]
#[async_method]
pub async fn mask_fill<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_aa_pgram(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fill_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_aa_pgram(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mask_fill(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
