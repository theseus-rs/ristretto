use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/FontConfigManager";

/// Register all intrinsic methods for `sun.font.FontConfigManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getFontConfig", "(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V", get_font_config);
    registry.register(
        CLASS_NAME,
        "getFontConfigAASettings",
        "(Ljava/lang/String;Ljava/lang/String;)I",
        get_font_config_aa_settings,
    );
    registry.register(
        CLASS_NAME,
        "getFontConfigVersion",
        "()I",
        get_font_config_version,
    );
}

#[async_recursion(?Send)]
async fn get_font_config(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V"
    )
}

#[async_recursion(?Send)]
async fn get_font_config_aa_settings(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
    )
}

#[async_recursion(?Send)]
async fn get_font_config_version(
    _thread: Arc<Thread>,
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
