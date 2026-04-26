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
    "sun/java2d/jules/JulesAATileGenerator.freePixmanImgPtr(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_pixman_img_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixman_img_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/jules/JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn rasterize_trapezoids_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y_off = parameters.pop_int()?;
    let _x_off = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    let _trap_cnt = parameters.pop_int()?;
    let _trap_pos = parameters.pop_reference()?;
    let _traps = parameters.pop_reference()?;
    let _pixman_image_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_pixman_img_ptr() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_pixman_img_ptr(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.jules.JulesAATileGenerator.freePixmanImgPtr(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_rasterize_trapezoids_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = rasterize_trapezoids_native(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.jules.JulesAATileGenerator.rasterizeTrapezoidsNative(J[I[II[BII)J",
            result.unwrap_err().to_string()
        );
    }
}
