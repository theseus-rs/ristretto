use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/laf/JRSUIUtils$ScrollBar";

/// Register all native methods for `apple.laf.JRSUIUtils$ScrollBar`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIUtils$ScrollBar.shouldUseScrollToClick()Z"
    )]
    async fn test_should_use_scroll_to_click() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_use_scroll_to_click(thread, Arguments::default()).await;
    }
}
