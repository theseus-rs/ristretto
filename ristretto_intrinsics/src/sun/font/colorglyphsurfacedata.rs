use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/ColorGlyphSurfaceData.initOps()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.ColorGlyphSurfaceData.initOps()V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/font/ColorGlyphSurfaceData.setCurrentGlyph(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_current_glyph<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _img_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(thread, Parameters::default()).await;
        assert_eq!(
            "sun.font.ColorGlyphSurfaceData.initOps()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_current_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_current_glyph(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V",
            result.unwrap_err().to_string()
        );
    }
}
