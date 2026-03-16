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
    "sun/java2d/jules/JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn tesselate_fill_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/jules/JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn tesselate_stroke_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tesselate_fill_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = tesselate_fill_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tesselate_stroke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = tesselate_stroke_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
