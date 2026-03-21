use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_do_path<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_line<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_spans<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn dev_copy_area<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_do_path() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_do_path(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_arc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_arc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_line() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_line(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_oval() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_oval(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_poly() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_poly(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_draw_round_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_round_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_arc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_arc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_oval() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_oval(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_poly() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_poly(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_round_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_round_rect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_fill_spans() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_spans(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dev_copy_area() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dev_copy_area(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
