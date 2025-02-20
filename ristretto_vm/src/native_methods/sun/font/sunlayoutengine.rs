use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/SunLayoutEngine";

/// Register all native methods for `sun.font.SunLayoutEngine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "initGVIDs", "()V", init_gv_ids);
        registry.register(CLASS_NAME, "nativeLayout", "(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V", native_layout);
    } else {
        registry.register(
            CLASS_NAME,
            "createFace",
            "(Lsun/font/Font2D;J)J",
            create_face,
        );
        registry.register(CLASS_NAME, "disposeFace", "(J)V", dispose_face);
        registry.register(CLASS_NAME, "shape", "(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z", shape);
    }
}

#[async_recursion(?Send)]
async fn create_face(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J")
}

#[async_recursion(?Send)]
async fn dispose_face(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.disposeFace(J)V")
}

#[async_recursion(?Send)]
async fn init_gv_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.initGVIDs()V")
}

#[async_recursion(?Send)]
async fn native_layout(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V"
    )
}

#[async_recursion(?Send)]
async fn shape(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
