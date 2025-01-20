use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/x11/X11Renderer";

/// Register all native methods for `sun.java2d.x11.X11Renderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "XDoPath",
        "(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
        x_do_path,
    );
    registry.register(CLASS_NAME, "XDrawArc", "(JJIIIIII)V", x_draw_arc);
    registry.register(CLASS_NAME, "XDrawLine", "(JJIIII)V", x_draw_line);
    registry.register(CLASS_NAME, "XDrawOval", "(JJIIII)V", x_draw_oval);
    registry.register(CLASS_NAME, "XDrawPoly", "(JJII[I[IIZ)V", x_draw_poly);
    registry.register(CLASS_NAME, "XDrawRect", "(JJIIII)V", x_draw_rect);
    registry.register(
        CLASS_NAME,
        "XDrawRoundRect",
        "(JJIIIIII)V",
        x_draw_round_rect,
    );
    registry.register(CLASS_NAME, "XFillArc", "(JJIIIIII)V", x_fill_arc);
    registry.register(CLASS_NAME, "XFillOval", "(JJIIII)V", x_fill_oval);
    registry.register(CLASS_NAME, "XFillPoly", "(JJII[I[II)V", x_fill_poly);
    registry.register(CLASS_NAME, "XFillRect", "(JJIIII)V", x_fill_rect);
    registry.register(
        CLASS_NAME,
        "XFillRoundRect",
        "(JJIIIIII)V",
        x_fill_round_rect,
    );
    registry.register(
        CLASS_NAME,
        "XFillSpans",
        "(JJLsun/java2d/pipe/SpanIterator;JII)V",
        x_fill_spans,
    );
    registry.register(CLASS_NAME, "devCopyArea", "(JJIIIIII)V", dev_copy_area);
}

#[async_recursion(?Send)]
async fn x_do_path(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V");
}

#[async_recursion(?Send)]
async fn x_draw_arc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_line(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_oval(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V");
}

#[async_recursion(?Send)]
async fn x_draw_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_round_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_arc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_oval(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V");
}

#[async_recursion(?Send)]
async fn x_fill_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_round_rect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_spans(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V");
}

#[async_recursion(?Send)]
async fn dev_copy_area(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V"
    )]
    async fn test_x_do_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_do_path(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V"
    )]
    async fn test_x_draw_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_arc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V")]
    async fn test_x_draw_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_line(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V")]
    async fn test_x_draw_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_oval(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V"
    )]
    async fn test_x_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_poly(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V")]
    async fn test_x_draw_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V"
    )]
    async fn test_x_draw_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_round_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V"
    )]
    async fn test_x_fill_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_arc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V")]
    async fn test_x_fill_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_oval(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V"
    )]
    async fn test_x_fill_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_poly(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V")]
    async fn test_x_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V"
    )]
    async fn test_x_fill_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_round_rect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V"
    )]
    async fn test_x_fill_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_spans(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V"
    )]
    async fn test_dev_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dev_copy_area(thread, Parameters::default()).await;
    }
}
