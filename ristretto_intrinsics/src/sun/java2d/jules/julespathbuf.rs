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
    "sun/java2d/jules/JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn tesselate_fill_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _clip_height = parameters.pop_int()?;
    let _clip_width = parameters.pop_int()?;
    let _clip_low_y = parameters.pop_int()?;
    let _clip_low_x = parameters.pop_int()?;
    let _winding_rule = parameters.pop_int()?;
    let _x_trap_array_length = parameters.pop_int()?;
    let _x_trap_array = parameters.pop_reference()?;
    let _op_cnt = parameters.pop_int()?;
    let _point_cnt = parameters.pop_int()?;
    let _ops = parameters.pop_reference()?;
    let _point_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/jules/JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn tesselate_stroke_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _clip_height = parameters.pop_int()?;
    let _clip_width = parameters.pop_int()?;
    let _clip_low_y = parameters.pop_int()?;
    let _clip_low_x = parameters.pop_int()?;
    let _m12 = parameters.pop_double()?;
    let _m11 = parameters.pop_double()?;
    let _m10 = parameters.pop_double()?;
    let _m02 = parameters.pop_double()?;
    let _m01 = parameters.pop_double()?;
    let _m00 = parameters.pop_double()?;
    let _offset = parameters.pop_double()?;
    let _dash_cnt = parameters.pop_int()?;
    let _dash_array = parameters.pop_reference()?;
    let _miter_limit = parameters.pop_double()?;
    let _line_join = parameters.pop_int()?;
    let _line_cap = parameters.pop_int()?;
    let _line_width = parameters.pop_double()?;
    let _x_trap_array_length = parameters.pop_int()?;
    let _x_trap_array = parameters.pop_reference()?;
    let _op_cnt = parameters.pop_int()?;
    let _point_cnt = parameters.pop_int()?;
    let _ops = parameters.pop_reference()?;
    let _point_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tesselate_fill_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = tesselate_fill_native(
            thread,
            Parameters::new(vec![
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
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.jules.JulesPathBuf.tesselateFillNative([I[BII[IIIIIII)[I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_tesselate_stroke_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = tesselate_stroke_native(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Double(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Double(0.0),
                Value::Object(None),
                Value::Int(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.jules.JulesPathBuf.tesselateStrokeNative([I[BII[IIDIID[DIDDDDDDDIIII)[I",
            result.unwrap_err().to_string()
        );
    }
}
