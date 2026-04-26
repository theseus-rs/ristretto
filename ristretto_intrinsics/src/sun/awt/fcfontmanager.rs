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
    "sun/awt/FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_path_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_x11_ge = parameters.pop_bool()?;
    let _no_type1_fonts = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_font_path_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_x11_ge = parameters.pop_bool()?;
    let _no_type1_fonts = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_font_path_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_font_path_native(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.awt.FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_font_path_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_path_native_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
