use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.FontConfigManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/FontConfigManager";
    registry.register(class_name, "getFontConfig", "(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V", get_font_config);
    registry.register(
        class_name,
        "getFontConfigAASettings",
        "(Ljava/lang/String;Ljava/lang/String;)I",
        get_font_config_aa_settings,
    );
    registry.register(
        class_name,
        "getFontConfigVersion",
        "()I",
        get_font_config_version,
    );
}

#[async_recursion(?Send)]
async fn get_font_config(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V")
}

#[async_recursion(?Send)]
async fn get_font_config_aa_settings(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
    )
}

#[async_recursion(?Send)]
async fn get_font_config_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.FontConfigManager.getFontConfigVersion()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/FontConfigManager";
        assert!(registry
            .method(
                class_name,
                "getFontConfig",
                "(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getFontConfigAASettings",
                "(Ljava/lang/String;Ljava/lang/String;)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getFontConfigVersion", "()I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.FontConfigManager.getFontConfig(Ljava/lang/String;Lsun/font/FontConfigManager$FontConfigInfo;[Lsun/font/FontConfigManager$FcCompFont;Z)V"
    )]
    async fn test_get_font_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.FontConfigManager.getFontConfigAASettings(Ljava/lang/String;Ljava/lang/String;)I"
    )]
    async fn test_get_font_config_aa_settings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config_aa_settings(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.FontConfigManager.getFontConfigVersion()I")]
    async fn test_get_font_config_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_config_version(thread, Arguments::default()).await;
    }
}
