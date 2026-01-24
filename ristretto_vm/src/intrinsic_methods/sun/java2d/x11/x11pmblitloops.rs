use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_blit(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn update_bitmask(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V"
    )]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V"
    )]
    async fn test_update_bitmask() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bitmask(thread, Parameters::default()).await;
    }
}
