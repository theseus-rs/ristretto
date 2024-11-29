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
    todo!()
}

#[async_recursion(?Send)]
async fn rasterize_trapezoids_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
