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
    "sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config_aa_settings<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_font_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_font_config_aa_settings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config_aa_settings(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_font_config_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_font_config_version(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
