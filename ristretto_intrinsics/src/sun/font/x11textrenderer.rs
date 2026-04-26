use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn do_draw_glyph_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gl = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn do_draw_glyph_list_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gl = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_draw_glyph_list() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = do_draw_glyph_list(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_do_draw_glyph_list_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_glyph_list_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/font/X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
            result.unwrap_err().to_string()
        );
    }
}
