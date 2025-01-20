use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/laf/AquaNativeResources";

/// Register all native methods for `com.apple.laf.AquaNativeResources`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getWindowBackgroundColor",
        "()J",
        get_window_background_color,
    );
}

#[async_recursion(?Send)]
async fn get_window_background_color(
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
