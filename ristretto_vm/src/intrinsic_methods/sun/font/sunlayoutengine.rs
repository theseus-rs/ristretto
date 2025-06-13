use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/SunLayoutEngine.createFace(Lsun/font/Font2D;J)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_face(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J")
}

#[intrinsic_method("sun/font/SunLayoutEngine.disposeFace(J)V", GreaterThan(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn dispose_face(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.disposeFace(J)V")
}

#[intrinsic_method("sun/font/SunLayoutEngine.initGVIDs()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn init_gv_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.initGVIDs()V")
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_layout(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V"
    )
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn shape(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J"
    )]
    async fn test_create_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_face(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.SunLayoutEngine.disposeFace(J)V")]
    async fn test_dispose_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_face(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.SunLayoutEngine.initGVIDs()V")]
    async fn test_init_gv_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_gv_ids(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V"
    )]
    async fn test_native_layout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_layout(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z"
    )]
    async fn test_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shape(thread, Parameters::default()).await;
    }
}
