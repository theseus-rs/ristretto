use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V"
    )
}

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config_aa_settings<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
    )
}

#[intrinsic_method(
    "sun/font/FontConfigManager.getFontConfigVersion()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_font_config_version<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.FontConfigManager.getFontConfigVersion()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V"
    )]
    async fn test_get_font_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
    )]
    async fn test_get_font_config_aa_settings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config_aa_settings(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.FontConfigManager.getFontConfigVersion()I"
    )]
    async fn test_get_font_config_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config_version(thread, Parameters::default()).await;
    }
}
