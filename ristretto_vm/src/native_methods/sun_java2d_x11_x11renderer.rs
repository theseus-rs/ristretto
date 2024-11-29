use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.x11.X11Renderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/x11/X11Renderer";
    registry.register(
        class_name,
        "XDoPath",
        "(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
        x_do_path,
    );
    registry.register(class_name, "XDrawArc", "(JJIIIIII)V", x_draw_arc);
    registry.register(class_name, "XDrawLine", "(JJIIII)V", x_draw_line);
    registry.register(class_name, "XDrawOval", "(JJIIII)V", x_draw_oval);
    registry.register(class_name, "XDrawPoly", "(JJII[I[IIZ)V", x_draw_poly);
    registry.register(class_name, "XDrawRect", "(JJIIII)V", x_draw_rect);
    registry.register(
        class_name,
        "XDrawRoundRect",
        "(JJIIIIII)V",
        x_draw_round_rect,
    );
    registry.register(class_name, "XFillArc", "(JJIIIIII)V", x_fill_arc);
    registry.register(class_name, "XFillOval", "(JJIIII)V", x_fill_oval);
    registry.register(class_name, "XFillPoly", "(JJII[I[II)V", x_fill_poly);
    registry.register(class_name, "XFillRect", "(JJIIII)V", x_fill_rect);
    registry.register(
        class_name,
        "XFillRoundRect",
        "(JJIIIIII)V",
        x_fill_round_rect,
    );
    registry.register(
        class_name,
        "XFillSpans",
        "(JJLsun/java2d/pipe/SpanIterator;JII)V",
        x_fill_spans,
    );
    registry.register(class_name, "devCopyArea", "(JJIIIIII)V", dev_copy_area);
}

#[async_recursion(?Send)]
async fn x_do_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_arc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_line(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_oval(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_draw_round_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_arc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_oval(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_round_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_fill_spans(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn dev_copy_area(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
