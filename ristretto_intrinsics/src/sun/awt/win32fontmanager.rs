use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/Win32FontManager.deRegisterFontWithPlatform(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn de_register_font_with_platform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _font_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32FontManager.deRegisterFontWithPlatform(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32FontManager.getEUDCFontFile()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_eudcfont_file<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32FontManager.getEUDCFontFile()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32FontManager.getFontPath(Z)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_font_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _no_type1 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32FontManager.getFontPath(Z)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32FontManager.populateFontFileNameMap0(Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/Locale;)V",
    Any
)]
#[async_method]
pub async fn populate_font_file_name_map0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _locale = parameters.pop_reference()?;
    let _family_to_font_list_map = parameters.pop_reference()?;
    let _font_to_family_map = parameters.pop_reference()?;
    let _font_to_file_map = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/Win32FontManager.populateFontFileNameMap0(Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/Locale;)V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/Win32FontManager.registerFontWithPlatform(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn register_font_with_platform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _font_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32FontManager.registerFontWithPlatform(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_de_register_font_with_platform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            de_register_font_with_platform(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun/awt/Win32FontManager.deRegisterFontWithPlatform(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_eudcfont_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_eudcfont_file(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32FontManager.getEUDCFontFile()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_font_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_path(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/Win32FontManager.getFontPath(Z)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_populate_font_file_name_map0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = populate_font_file_name_map0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32FontManager.populateFontFileNameMap0(Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/HashMap;Ljava/util/Locale;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_register_font_with_platform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            register_font_with_platform(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/Win32FontManager.registerFontWithPlatform(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
