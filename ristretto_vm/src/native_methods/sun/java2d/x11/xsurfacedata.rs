use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/x11/XSurfaceData";

/// Register all native methods for `sun.java2d.x11.XSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "XCreateGC", "(J)J", x_create_gc);
    registry.register(CLASS_NAME, "XResetClip", "(J)V", x_reset_clip);
    registry.register(
        CLASS_NAME,
        "XSetClip",
        "(JIIIILsun/java2d/pipe/Region;)V",
        x_set_clip,
    );
    registry.register(
        CLASS_NAME,
        "XSetGraphicsExposures",
        "(JZ)V",
        x_set_graphics_exposures,
    );
    registry.register(
        CLASS_NAME,
        "flushNativeSurface",
        "()V",
        flush_native_surface,
    );
    registry.register(
        CLASS_NAME,
        "initOps",
        "(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
        init_ops,
    );
    registry.register(CLASS_NAME, "isDrawableValid", "()Z", is_drawable_valid);
    registry.register(CLASS_NAME, "setInvalid", "()V", set_invalid);
}

#[async_recursion(?Send)]
async fn x_create_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XCreateGC(J)J");
}

#[async_recursion(?Send)]
async fn x_reset_clip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XResetClip(J)V");
}

#[async_recursion(?Send)]
async fn x_set_clip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V");
}

#[async_recursion(?Send)]
async fn x_set_graphics_exposures(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V");
}

#[async_recursion(?Send)]
async fn flush_native_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.flushNativeSurface()V");
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V");
}

#[async_recursion(?Send)]
async fn is_drawable_valid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.isDrawableValid()Z");
}

#[async_recursion(?Send)]
async fn set_invalid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.setInvalid()V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XCreateGC(J)J")]
    async fn test_x_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_create_gc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XResetClip(J)V")]
    async fn test_x_reset_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_reset_clip(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V"
    )]
    async fn test_x_set_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_clip(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V"
    )]
    async fn test_x_set_graphics_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_graphics_exposures(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.flushNativeSurface()V"
    )]
    async fn test_flush_native_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_native_surface(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.isDrawableValid()Z"
    )]
    async fn test_is_drawable_valid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_drawable_valid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.setInvalid()V")]
    async fn test_set_invalid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_invalid(thread, Arguments::default()).await;
    }
}
