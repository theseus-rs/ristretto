use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/jules/JulesAATileGenerator";

/// Register all native methods for `sun.java2d.jules.JulesAATileGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "freePixmanImgPtr", "(J)V", free_pixman_img_ptr);
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V"
    )]
    async fn test_free_pixman_img_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_pixman_img_ptr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J"
    )]
    async fn test_rasterize_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rasterize_trapezoids_native(thread, Arguments::default()).await;
    }
}
