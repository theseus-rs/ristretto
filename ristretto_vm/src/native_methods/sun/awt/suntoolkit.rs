use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/SunToolkit";

/// Register all native methods for `sun.awt.SunToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "closeSplashScreen", "()V", close_splash_screen);
}

#[async_recursion(?Send)]
async fn close_splash_screen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.SunToolkit.closeSplashScreen()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.SunToolkit.closeSplashScreen()V")]
    async fn test_close_splash_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_splash_screen(thread, Arguments::default()).await;
    }
}
