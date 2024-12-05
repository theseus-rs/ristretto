use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.DataBufferNative`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/DataBufferNative";
    registry.register(
        class_name,
        "getElem",
        "(IILsun/java2d/SurfaceData;)I",
        get_elem,
    );
    registry.register(
        class_name,
        "setElem",
        "(IIILsun/java2d/SurfaceData;)V",
        set_elem,
    );
}

#[async_recursion(?Send)]
async fn get_elem(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I")
}

#[async_recursion(?Send)]
async fn set_elem(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V")
}
