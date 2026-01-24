use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_do_path(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V"
    );
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_arc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_line(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_oval(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_poly(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_rect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_draw_round_rect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_arc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_oval(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_poly(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_rect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_round_rect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_fill_spans(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V");
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn dev_copy_area(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
