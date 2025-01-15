use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/CGLLayer";

/// Register all native methods for `sun.java2d.opengl.CGLLayer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "blitTexture", "(J)V", blit_texture);
    registry.register(CLASS_NAME, "nativeCreateLayer", "()J", native_create_layer);
    registry.register(CLASS_NAME, "nativeSetScale", "(JD)V", native_set_scale);
    registry.register(
        CLASS_NAME,
        "validate",
        "(JLsun/java2d/opengl/CGLSurfaceData;)V",
        validate,
    );
}

#[async_recursion(?Send)]
async fn blit_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.blitTexture(J)V");
}

#[async_recursion(?Send)]
async fn native_create_layer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.nativeCreateLayer()J");
}

#[async_recursion(?Send)]
async fn native_set_scale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V");
}

#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.opengl.CGLLayer.blitTexture(J)V")]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = blit_texture(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.nativeCreateLayer()J"
    )]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_layer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.nativeSetScale(JD)V"
    )]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_scale(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLLayer.validate(JLsun/java2d/opengl/CGLSurfaceData;)V"
    )]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Arguments::default()).await;
    }
}
