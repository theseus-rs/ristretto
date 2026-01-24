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
    "sun/java2d/jules/JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn tesselate_fill_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I")
}

#[intrinsic_method(
    "sun/java2d/jules/JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn tesselate_stroke_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I"
    )]
    async fn test_tesselate_fill_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_fill_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I"
    )]
    async fn test_tesselate_stroke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tesselate_stroke_native(thread, Parameters::default()).await;
    }
}
