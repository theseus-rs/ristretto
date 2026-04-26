use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V",
    Any
)]
#[async_method]
pub async fn draw_aa_pgram<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lw2 = parameters.pop_double()?;
    let _lw1 = parameters.pop_double()?;
    let _dy2 = parameters.pop_double()?;
    let _dx2 = parameters.pop_double()?;
    let _dy1 = parameters.pop_double()?;
    let _dx1 = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _comp = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V",
    Any
)]
#[async_method]
pub async fn fill_aa_pgram<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dy2 = parameters.pop_double()?;
    let _dx2 = parameters.pop_double()?;
    let _dy1 = parameters.pop_double()?;
    let _dx1 = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _comp = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/loops/MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V",
    Any
)]
#[async_method]
pub async fn mask_fill<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _mask = parameters.pop_reference()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _comp = parameters.pop_reference()?;
    let _s_data = parameters.pop_reference()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_aa_pgram(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.MaskFill.DrawAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDDDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_fill_aa_pgram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_aa_pgram(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.MaskFill.FillAAPgram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;DDDDDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mask_fill(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.MaskFill.MaskFill(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Ljava/awt/Composite;IIII[BII)V",
            result.unwrap_err().to_string()
        );
    }
}
