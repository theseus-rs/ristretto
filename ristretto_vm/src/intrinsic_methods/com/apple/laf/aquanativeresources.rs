use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/apple/laf/AquaNativeResources.getWindowBackgroundColor()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_window_background_color(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaNativeResources.getWindowBackgroundColor()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaNativeResources.getWindowBackgroundColor()J"
    )]
    async fn test_get_window_background_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_window_background_color(thread, Parameters::default()).await;
    }
}
