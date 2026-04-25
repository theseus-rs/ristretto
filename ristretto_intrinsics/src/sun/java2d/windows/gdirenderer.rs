use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.devCopyArea(Lsun/java2d/windows/GDIWindowSurfaceData;IIIIII)V",
    Any
)]
#[async_method]
pub async fn dev_copy_area<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _wsd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.devCopyArea(Lsun/java2d/windows/GDIWindowSurfaceData;IIIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_draw_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawLine(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
    Any
)]
#[async_method]
pub async fn do_draw_line<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawLine(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
    Any
)]
#[async_method]
pub async fn do_draw_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[IIZ)V",
    Any
)]
#[async_method]
pub async fn do_draw_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isclosed = parameters.pop_bool()?;
    let _npoints = parameters.pop_int()?;
    let _ypointsarray = parameters.pop_reference()?;
    let _xpointsarray = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[IIZ)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
    Any
)]
#[async_method]
pub async fn do_draw_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doDrawRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_draw_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doDrawRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doFillArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_fill_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doFillArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doFillOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
    Any
)]
#[async_method]
pub async fn do_fill_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doFillOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doFillPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[II)V",
    Any
)]
#[async_method]
pub async fn do_fill_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _npoints = parameters.pop_int()?;
    let _ypointsarray = parameters.pop_reference()?;
    let _xpointsarray = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doFillPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[II)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doFillRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
    Any
)]
#[async_method]
pub async fn do_fill_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doFillRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doFillRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_fill_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doFillRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V".to_string()).into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIRenderer.doShape(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIILjava/awt/geom/Path2D$Float;Z)V",
    Any
)]
#[async_method]
pub async fn do_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _p2df = parameters.pop_reference()?;
    let _trans_y = parameters.pop_int()?;
    let _trans_x = parameters.pop_int()?;
    let _color = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIRenderer.doShape(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIILjava/awt/geom/Path2D$Float;Z)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_dev_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dev_copy_area(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.devCopyArea(Lsun/java2d/windows/GDIWindowSurfaceData;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_arc(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_line(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawLine(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_oval(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_poly(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[IIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_draw_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_round_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doDrawRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_fill_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_fill_arc(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doFillArc(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_fill_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_fill_oval(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doFillOval(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_fill_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_fill_poly(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doFillPoly(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;III[I[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_fill_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doFillRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_fill_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_fill_round_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doFillRoundRect(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_do_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_shape(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIRenderer.doShape(Lsun/java2d/windows/GDIWindowSurfaceData;Lsun/java2d/pipe/Region;Ljava/awt/Composite;IIILjava/awt/geom/Path2D$Float;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
