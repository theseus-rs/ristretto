use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J",
    Any
)]
#[async_method]
pub async fn get_glyph_image_from_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg5 = parameters.pop_int()?;
    let _arg4 = parameters.pop_bool()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/FileFontStrike.initNative()Z", LessThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FileFontStrike.initNative()Z".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_glyph_image_from_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_glyph_image_from_windows(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.FileFontStrike._getGlyphImageFromWindows(Ljava/lang/String;IIIZI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await;
        assert_eq!(
            "sun.font.FileFontStrike.initNative()Z",
            result.unwrap_err().to_string()
        );
    }
}
