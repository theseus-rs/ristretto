use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.jules.JulesAATileGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/jules/JulesAATileGenerator";
    registry.register(class_name, "freePixmanImgPtr", "(J)V", free_pixman_img_ptr);
    registry.register(
        class_name,
        "rasterizeTrapezoidsNative",
        "(J[I[II[BII)J",
        rasterize_trapezoids_native,
    );
}

#[async_recursion(?Send)]
async fn free_pixman_img_ptr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V")
}

#[async_recursion(?Send)]
async fn rasterize_trapezoids_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/jules/JulesAATileGenerator";
        assert!(registry
            .method(class_name, "freePixmanImgPtr", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "rasterizeTrapezoidsNative", "(J[I[II[BII)J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V")]
    async fn test_free_pixman_img_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_pixman_img_ptr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J"
    )]
    async fn test_rasterize_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rasterize_trapezoids_native(thread, Arguments::default()).await;
    }
}
