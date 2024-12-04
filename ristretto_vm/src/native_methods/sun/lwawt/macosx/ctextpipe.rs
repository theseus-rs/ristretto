use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CTextPipe`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CTextPipe";
    registry.register(
        class_name,
        "doDrawGlyphs",
        "(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
        do_draw_glyphs,
    );
    registry.register(
        class_name,
        "doDrawString",
        "(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
        do_draw_string,
    );
    registry.register(
        class_name,
        "doOneUnicode",
        "(Lsun/java2d/SurfaceData;JCFF)V",
        do_one_unicode,
    );
    registry.register(
        class_name,
        "doUnicodes",
        "(Lsun/java2d/SurfaceData;J[CIIFF)V",
        do_unicodes,
    );
}

#[async_recursion(?Send)]
async fn do_draw_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn do_draw_string(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn do_one_unicode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn do_unicodes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
