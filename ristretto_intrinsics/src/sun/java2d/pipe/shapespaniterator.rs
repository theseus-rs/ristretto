use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.addSegment(I[F)V", Any)]
#[async_method]
pub async fn add_segment<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _coords = parameters.pop_reference()?;
    let _type_ = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.appendPoly([I[IIII)V", Any)]
#[async_method]
pub async fn append_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _yoff = parameters.pop_int()?;
    let _xoff = parameters.pop_int()?;
    let _n_points = parameters.pop_int()?;
    let _y_points = parameters.pop_reference()?;
    let _x_points = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.closePath()V", Any)]
#[async_method]
pub async fn close_path<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.closePath()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.curveTo(FFFFFF)V", Any)]
#[async_method]
pub async fn curve_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y3 = parameters.pop_float()?;
    let _x3 = parameters.pop_float()?;
    let _y2 = parameters.pop_float()?;
    let _x2 = parameters.pop_float()?;
    let _y1 = parameters.pop_float()?;
    let _x1 = parameters.pop_float()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.dispose()V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.pipe.ShapeSpanIterator.dispose()V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getNativeConsumer()J", Any)]
#[async_method]
pub async fn get_native_consumer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getNativeIterator()J", Any)]
#[async_method]
pub async fn get_native_iterator<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getPathBox([I)V", Any)]
#[async_method]
pub async fn get_path_box<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pathbox = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.intersectClipBox(IIII)V", Any)]
#[async_method]
pub async fn intersect_clip_box<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hiy = parameters.pop_int()?;
    let _hix = parameters.pop_int()?;
    let _loy = parameters.pop_int()?;
    let _lox = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.lineTo(FF)V", Any)]
#[async_method]
pub async fn line_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.moveTo(FF)V", Any)]
#[async_method]
pub async fn move_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_float()?;
    let _x = parameters.pop_float()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.nextSpan([I)Z", Any)]
#[async_method]
pub async fn next_span<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _spanbox = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.pathDone()V", Any)]
#[async_method]
pub async fn path_done<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.pathDone()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.quadTo(FFFF)V", Any)]
#[async_method]
pub async fn quad_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y2 = parameters.pop_float()?;
    let _x2 = parameters.pop_float()?;
    let _y1 = parameters.pop_float()?;
    let _x1 = parameters.pop_float()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setNormalize(Z)V", Any)]
#[async_method]
pub async fn set_normalize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _adjust = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setOutputAreaXYXY(IIII)V", Any)]
#[async_method]
pub async fn set_output_area_xyxy<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hiy = parameters.pop_int()?;
    let _hix = parameters.pop_int()?;
    let _loy = parameters.pop_int()?;
    let _lox = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setRule(I)V", Any)]
#[async_method]
pub async fn set_rule<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rule = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.setRule(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.skipDownTo(I)V", Any)]
#[async_method]
pub async fn skip_down_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_segment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_segment(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_append_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = append_poly(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_close_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_path(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.closePath()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_curve_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = curve_to(
            thread,
            Parameters::new(vec![
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_consumer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_consumer(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_iterator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_iterator(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_path_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_path_box(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_intersect_clip_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = intersect_clip_box(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_line_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = line_to(
            thread,
            Parameters::new(vec![Value::Float(0.0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_move_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_to(
            thread,
            Parameters::new(vec![Value::Float(0.0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_next_span() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = next_span(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_path_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = path_done(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.pathDone()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_quad_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = quad_to(
            thread,
            Parameters::new(vec![
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
                Value::Float(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_normalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_normalize(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_output_area_xyxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_output_area_xyxy(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_rule() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_rule(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.setRule(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_skip_down_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = skip_down_to(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V",
            result.unwrap_err().to_string()
        );
    }
}
