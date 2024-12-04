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
    todo!()
}

#[async_recursion(?Send)]
async fn get_font_config_aa_settings(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_font_config_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
