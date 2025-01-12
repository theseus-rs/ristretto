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
    todo!("sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V");
}

#[async_recursion(?Send)]
async fn x_draw_arc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_line(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_oval(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V");
}

#[async_recursion(?Send)]
async fn x_draw_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_draw_round_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_arc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_oval(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V");
}

#[async_recursion(?Send)]
async fn x_fill_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_round_rect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_fill_spans(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V");
}

#[async_recursion(?Send)]
async fn dev_copy_area(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/x11/X11Renderer";
        assert!(registry
            .method(
                class_name,
                "XDoPath",
                "(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "XDrawArc", "(JJIIIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XDrawLine", "(JJIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XDrawOval", "(JJIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XDrawPoly", "(JJII[I[IIZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "XDrawRect", "(JJIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XDrawRoundRect", "(JJIIIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XFillArc", "(JJIIIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XFillOval", "(JJIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XFillPoly", "(JJII[I[II)V")
            .is_some());
        assert!(registry
            .method(class_name, "XFillRect", "(JJIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "XFillRoundRect", "(JJIIIIII)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "XFillSpans",
                "(JJLsun/java2d/pipe/SpanIterator;JII)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "devCopyArea", "(JJIIIIII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V"
    )]
    async fn test_x_do_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_do_path(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V")]
    async fn test_x_draw_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_arc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V")]
    async fn test_x_draw_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_line(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V")]
    async fn test_x_draw_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_oval(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V")]
    async fn test_x_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_poly(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V")]
    async fn test_x_draw_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V")]
    async fn test_x_draw_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_draw_round_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V")]
    async fn test_x_fill_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_arc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V")]
    async fn test_x_fill_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_oval(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V")]
    async fn test_x_fill_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_poly(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V")]
    async fn test_x_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V")]
    async fn test_x_fill_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_round_rect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V"
    )]
    async fn test_x_fill_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_fill_spans(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V")]
    async fn test_dev_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dev_copy_area(thread, Arguments::default()).await;
    }
}
