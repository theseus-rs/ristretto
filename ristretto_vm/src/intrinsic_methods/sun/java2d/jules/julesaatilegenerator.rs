use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/jules/JulesAATileGenerator.freePixmanImgPtr(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn free_pixman_img_ptr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V")
}

#[intrinsic_method(
    "sun/java2d/jules/JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn rasterize_trapezoids_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V"
    )]
    async fn test_free_pixman_img_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_pixman_img_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J"
    )]
    async fn test_rasterize_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rasterize_trapezoids_native(thread, Parameters::default()).await;
    }
}
