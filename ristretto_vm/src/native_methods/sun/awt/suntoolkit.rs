use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.SunToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/SunToolkit";
    registry.register(class_name, "closeSplashScreen", "()V", close_splash_screen);
}

#[async_recursion(?Send)]
async fn close_splash_screen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.SunToolkit.closeSplashScreen()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/SunToolkit";
        assert!(registry
            .method(class_name, "closeSplashScreen", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.SunToolkit.closeSplashScreen()V")]
    async fn test_close_splash_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_splash_screen(thread, Arguments::default()).await;
    }
}
