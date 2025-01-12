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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/image/DataBufferNative";
        assert!(registry
            .method(class_name, "getElem", "(IILsun/java2d/SurfaceData;)I")
            .is_some());
        assert!(registry
            .method(class_name, "setElem", "(IIILsun/java2d/SurfaceData;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.DataBufferNative.getElem(IILsun/java2d/SurfaceData;)I"
    )]
    async fn test_get_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_elem(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.DataBufferNative.setElem(IIILsun/java2d/SurfaceData;)V"
    )]
    async fn test_set_elem() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_elem(thread, Arguments::default()).await;
    }
}
