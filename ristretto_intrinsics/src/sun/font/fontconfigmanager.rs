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
    "sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _include_fallbacks = parameters.pop_bool()?;
    let _fc_comp_font_array = parameters.pop_reference()?;
    let _fc_info_obj = parameters.pop_reference()?;
    let _locale_str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config_aa_settings<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fc_name_str = parameters.pop_reference()?;
    let _locale_str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigVersion()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config_version<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.FontConfigManager.getFontConfigVersion()I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_font_config_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _include_fallbacks = parameters.pop_bool()?;
    let _fc_comp_font_array = parameters.pop_reference()?;
    let _fc_info_obj = parameters.pop_reference()?;
    let _locale_str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_font_config_aasettings_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fc_name_str = parameters.pop_reference()?;
    let _locale_str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigVersion()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_font_config_version_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/FontConfigManager.getFontConfigVersion()I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_font_config() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_font_config(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_font_config_aa_settings() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_font_config_aa_settings(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_font_config_version() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_font_config_version(thread, Parameters::default()).await;
        assert_eq!(
            "sun.font.FontConfigManager.getFontConfigVersion()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_font_config_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_font_config_aasettings_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config_aasettings_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_font_config_version_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config_version_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/font/FontConfigManager.getFontConfigVersion()I",
            result.unwrap_err().to_string()
        );
    }
}
