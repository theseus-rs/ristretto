use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.FcFontManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/FcFontManager";
    registry.register(
        class_name,
        "getFontPathNative",
        "(ZZ)Ljava/lang/String;",
        get_font_path_native,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_font_path_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
