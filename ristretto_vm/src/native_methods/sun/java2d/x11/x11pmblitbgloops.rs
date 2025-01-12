use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.X11PMBlitBgLoops`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/X11PMBlitBgLoops";
    registry.register(class_name, "nativeBlitBg", "(JJJIIIIIII)V", native_blit_bg);
}

#[async_recursion(?Send)]
async fn native_blit_bg(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/x11/X11PMBlitBgLoops";
        assert!(registry
            .method(class_name, "nativeBlitBg", "(JJJIIIIIII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V")]
    async fn test_native_blit_bg() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit_bg(thread, Arguments::default()).await;
    }
}
