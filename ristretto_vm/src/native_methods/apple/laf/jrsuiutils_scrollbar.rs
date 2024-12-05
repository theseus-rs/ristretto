use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.laf.JRSUIUtils$ScrollBar`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/laf/JRSUIUtils$ScrollBar";
    registry.register(
        class_name,
        "shouldUseScrollToClick",
        "()Z",
        should_use_scroll_to_click,
    );
}

#[async_recursion(?Send)]
async fn should_use_scroll_to_click(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIUtils$ScrollBar.shouldUseScrollToClick()Z")
}
