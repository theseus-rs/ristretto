use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.laf.AquaNativeResources`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/laf/AquaNativeResources";
    registry.register(
        class_name,
        "getWindowBackgroundColor",
        "()J",
        get_window_background_color,
    );
}

#[async_recursion(?Send)]
async fn get_window_background_color(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.AquaNativeResources.getWindowBackgroundColor()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/laf/AquaNativeResources";
        assert!(registry
            .method(class_name, "getWindowBackgroundColor", "()J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.AquaNativeResources.getWindowBackgroundColor()J"
    )]
    async fn test_get_window_background_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_window_background_color(thread, Arguments::default()).await;
    }
}
