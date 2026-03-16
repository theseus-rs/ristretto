use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/SunLayoutEngine.createFace(Lsun/font/Font2D;J)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_face<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/SunLayoutEngine.disposeFace(J)V", GreaterThan(JAVA_8))]
#[async_method]
pub async fn dispose_face<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.disposeFace(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/SunLayoutEngine.initGVIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_gv_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.initGVIDs()V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_layout<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn shape<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_face(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispose_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_face(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_gv_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_gv_ids(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_layout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_layout(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shape(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
