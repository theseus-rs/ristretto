use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/NativeStrike.createNullScalerContext()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_null_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeStrike.createNullScalerContext()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/NativeStrike.createScalerContext([BID)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_scaler_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale = parameters.pop_double()?;
    let _pt_size = parameters.pop_int()?;
    let _name_bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.NativeStrike.createScalerContext([BID)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/NativeStrike.getMaxGlyph(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_max_glyph<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.font.NativeStrike.getMaxGlyph(J)I".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeStrike.createNullScalerContext()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_null_scaler_context_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/NativeStrike.createNullScalerContext()J".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/font/NativeStrike.createScalerContext([BID)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_scaler_context_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale = parameters.pop_double()?;
    let _pt_size = parameters.pop_int()?;
    let _name_bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/font/NativeStrike.createScalerContext([BID)J".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/font/NativeStrike.getMaxGlyph(J)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_max_glyph_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_scaler_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/font/NativeStrike.getMaxGlyph(J)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_null_scaler_context() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_null_scaler_context(thread, Parameters::default()).await;
        assert_eq!(
            "sun.font.NativeStrike.createNullScalerContext()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_scaler_context() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_scaler_context(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.font.NativeStrike.createScalerContext([BID)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_max_glyph() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_max_glyph(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.font.NativeStrike.getMaxGlyph(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_null_scaler_context_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_null_scaler_context_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/font/NativeStrike.createNullScalerContext()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_scaler_context_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_scaler_context_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun/font/NativeStrike.createScalerContext([BID)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_max_glyph_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_max_glyph_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/font/NativeStrike.getMaxGlyph(J)I",
            result.unwrap_err().to_string()
        );
    }
}
