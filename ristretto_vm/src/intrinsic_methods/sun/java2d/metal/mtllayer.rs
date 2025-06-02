use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLLayer";

/// Register all intrinsic methods for `sun.java2d.metal.MTLLayer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "nativeSetOpaque", "(JZ)V", native_set_opaque);
    }

    registry.register(CLASS_NAME, "blitTexture", "(J)V", blit_texture);
    registry.register(CLASS_NAME, "nativeCreateLayer", "()J", native_create_layer);
    registry.register(CLASS_NAME, "nativeSetInsets", "(JII)V", native_set_insets);
    registry.register(CLASS_NAME, "nativeSetScale", "(JD)V", native_set_scale);
    registry.register(
        CLASS_NAME,
        "validate",
        "(JLsun/java2d/metal/MTLSurfaceData;)V",
        validate,
    );
}

#[async_recursion(?Send)]
async fn blit_texture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.blitTexture(J)V")
}

#[async_recursion(?Send)]
async fn native_create_layer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeCreateLayer()J")
}

#[async_recursion(?Send)]
async fn native_set_insets(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V")
}

#[async_recursion(?Send)]
async fn native_set_opaque(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V")
}

#[async_recursion(?Send)]
async fn native_set_scale(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetScale(JD)V")
}

#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.metal.MTLLayer.blitTexture(J)V")]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = blit_texture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeCreateLayer()J"
    )]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_layer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V"
    )]
    async fn test_native_set_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_insets(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V"
    )]
    async fn test_native_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_opaque(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetScale(JD)V")]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_scale(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V"
    )]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Parameters::default()).await;
    }
}
