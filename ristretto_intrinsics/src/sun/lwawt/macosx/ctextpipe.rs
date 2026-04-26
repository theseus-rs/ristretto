use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
    Any
)]
#[async_method]
pub async fn do_draw_glyphs<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _g_vector = parameters.pop_reference()?;
    let _awt_strike_ptr = parameters.pop_long()?;
    let _jsurfacedata = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
    Any
)]
#[async_method]
pub async fn do_draw_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _str = parameters.pop_reference()?;
    let _awt_strike_ptr = parameters.pop_long()?;
    let _jsurfacedata = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V",
    Any
)]
#[async_method]
pub async fn do_one_unicode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _a_unicode = parameters.pop_int()?;
    let _awt_strike_ptr = parameters.pop_long()?;
    let _jsurfacedata = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V",
    Any
)]
#[async_method]
pub async fn do_unicodes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _length = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _unicodes = parameters.pop_reference()?;
    let _awt_strike_ptr = parameters.pop_long()?;
    let _jsurfacedata = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_draw_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_glyphs(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_draw_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_string(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_one_unicode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_one_unicode(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_unicodes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_unicodes(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V",
            result.unwrap_err().to_string()
        );
    }
}
