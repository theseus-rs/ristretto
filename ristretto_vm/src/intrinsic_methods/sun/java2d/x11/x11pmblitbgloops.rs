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
    "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_blit_bg(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V"
    )]
    async fn test_native_blit_bg() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit_bg(thread, Parameters::default()).await;
    }
}
