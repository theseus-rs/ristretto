use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLSurfaceData";

/// Register all intrinsic methods for `sun.java2d.metal.MTLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "clearWindow", "()V", clear_window);
    registry.register(
        CLASS_NAME,
        "getMTLTexturePointer",
        "(J)J",
        get_mtl_texture_pointer,
    );
    registry.register(
        CLASS_NAME,
        "initFlipBackbuffer",
        "(J)Z",
        init_flip_backbuffer,
    );
    registry.register(
        CLASS_NAME,
        "initOps",
        "(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V",
        init_ops,
    );
    registry.register(CLASS_NAME, "initRTexture", "(JZII)Z", init_r_texture);
    registry.register(CLASS_NAME, "initTexture", "(JZII)Z", init_texture);
}

#[async_recursion(?Send)]
async fn clear_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.clearWindow()V");
}

#[async_recursion(?Send)]
async fn get_mtl_texture_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J");
}

#[async_recursion(?Send)]
async fn init_flip_backbuffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z");
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V");
}

#[async_recursion(?Send)]
async fn init_r_texture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z");
}

#[async_recursion(?Send)]
async fn init_texture(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.clearWindow()V")]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.getMTLTexturePointer(J)J")]
    async fn test_get_mtl_texture_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtl_texture_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initFlipBackbuffer(J)Z")]
    async fn test_init_flip_backbuffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_flip_backbuffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.metal.MTLSurfaceData.initOps(Lsun/java2d/metal/MTLGraphicsConfig;JJJIIZ)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initRTexture(JZII)Z")]
    async fn test_init_r_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_r_texture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLSurfaceData.initTexture(JZII)Z")]
    async fn test_init_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_texture(thread, Parameters::default()).await;
    }
}
