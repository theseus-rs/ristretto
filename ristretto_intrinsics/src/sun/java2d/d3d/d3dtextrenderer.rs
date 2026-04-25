use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/d3d/D3DTextRenderer.drawGlyphList(IZZZIFF[J[F)V", Any)]
#[async_method]
pub async fn draw_glyph_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pos_array = parameters.pop_reference()?;
    let _img_array = parameters.pop_reference()?;
    let _glyph_list_orig_y = parameters.pop_float()?;
    let _glyph_list_orig_x = parameters.pop_float()?;
    let _lcd_contrast = parameters.pop_int()?;
    let _rgb_order = parameters.pop_bool()?;
    let _sub_pix_pos = parameters.pop_bool()?;
    let _use_positions = parameters.pop_bool()?;
    let _num_glyphs = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DTextRenderer.drawGlyphList(IZZZIFF[J[F)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_glyph_list(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DTextRenderer.drawGlyphList(IZZZIFF[J[F)V",
            result.unwrap_err().to_string()
        );
    }
}
