use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIUtils$ScrollBar.shouldUseScrollToClick()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn should_use_scroll_to_click(
    _thread: Arc<Thread>,
    _parameters: Parameters,
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
        let _ = should_use_scroll_to_click(thread, Parameters::default()).await;
    }
}
