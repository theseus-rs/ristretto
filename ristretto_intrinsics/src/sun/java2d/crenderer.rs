use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V", Any)]
#[async_method]
pub async fn do_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _type_ = parameters.pop_int()?;
    let _angle_extent = parameters.pop_float()?;
    let _angle_start = parameters.pop_float()?;
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
    Any
)]
#[async_method]
pub async fn do_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dh = parameters.pop_int()?;
    let _dw = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sh = parameters.pop_int()?;
    let _sw = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _flipv = parameters.pop_bool()?;
    let _fliph = parameters.pop_bool()?;
    let _img = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V", Any)]
#[async_method]
pub async fn do_line<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y2 = parameters.pop_float()?;
    let _x2 = parameters.pop_float()?;
    let _y1 = parameters.pop_float()?;
    let _x1 = parameters.pop_float()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V", Any)]
#[async_method]
pub async fn do_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _ispolygon = parameters.pop_bool()?;
    let _npoints = parameters.pop_int()?;
    let _ypoints = parameters.pop_reference()?;
    let _xpoints = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V", Any)]
#[async_method]
pub async fn do_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V",
    Any
)]
#[async_method]
pub async fn do_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isfill = parameters.pop_bool()?;
    let _arc_h = parameters.pop_float()?;
    let _arc_w = parameters.pop_float()?;
    let _height = parameters.pop_float()?;
    let _width = parameters.pop_float()?;
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    let _s_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
    Any
)]
#[async_method]
pub async fn do_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _should_apply_offset = parameters.pop_bool()?;
    let _isfill = parameters.pop_bool()?;
    let _winding_rule = parameters.pop_int()?;
    let _j_int_types = parameters.pop_reference()?;
    let _j_float_coordinates = parameters.pop_reference()?;
    let _length = parameters.pop_int()?;
    let _jsurfacedata = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/CRenderer.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_arc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_arc(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doArc(Lsun/java2d/SurfaceData;FFFFFFIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_image(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun.java2d.CRenderer.doImage(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;ZZIIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_line() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_line(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doLine(Lsun/java2d/SurfaceData;FFFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_oval() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_oval(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doOval(Lsun/java2d/SurfaceData;FFFFZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_poly(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doPoly(Lsun/java2d/SurfaceData;[I[IIZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doRect(Lsun/java2d/SurfaceData;FFFFZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_round_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_round_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doRoundRect(Lsun/java2d/SurfaceData;FFFFFFZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_shape(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.CRenderer.doShape(Lsun/java2d/SurfaceData;ILjava/nio/FloatBuffer;Ljava/nio/IntBuffer;IZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
