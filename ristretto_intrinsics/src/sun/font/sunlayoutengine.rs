use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/SunLayoutEngine.createFace(Lsun/font/Font2D;J)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_face<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _platform_native_font_ptr = parameters.pop_long()?;
    let _font = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/SunLayoutEngine.disposeFace(J)V", GreaterThan(JAVA_8))]
#[async_method]
pub async fn dispose_face<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _face_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.disposeFace(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/SunLayoutEngine.initGVIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_gv_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.initGVIDs()V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_layout<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg16 = parameters.pop_long()?;
    let _arg15 = parameters.pop_long()?;
    let _arg14 = parameters.pop_reference()?;
    let _arg13 = parameters.pop_reference()?;
    let _arg12 = parameters.pop_int()?;
    let _arg11 = parameters.pop_int()?;
    let _arg10 = parameters.pop_int()?;
    let _arg9 = parameters.pop_int()?;
    let _arg8 = parameters.pop_int()?;
    let _arg7 = parameters.pop_int()?;
    let _arg6 = parameters.pop_int()?;
    let _arg5 = parameters.pop_reference()?;
    let _arg4 = parameters.pop_int()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V".to_string()).into())
}

#[intrinsic_method(
    "sun/font/SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _slot = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _start_pt = parameters.pop_reference()?;
    let _base_index = parameters.pop_int()?;
    let _limit = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _script = parameters.pop_int()?;
    let _gvdata = parameters.pop_reference()?;
    let _text = parameters.pop_reference()?;
    let _p_face = parameters.pop_long()?;
    let _matrix = parameters.pop_reference()?;
    let _pt_size = parameters.pop_float()?;
    let _font_strike = parameters.pop_reference()?;
    let _font2_d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_face(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_face(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.SunLayoutEngine.disposeFace(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_gv_ids() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_gv_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun.font.SunLayoutEngine.initGVIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_layout() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_layout(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shape(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Float(0.0),
                Value::Object(None),
                Value::Long(0),
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
            "sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z",
            result.unwrap_err().to_string()
        );
    }
}
