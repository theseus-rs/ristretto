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

#[intrinsic_method("sun/java2d/xr/XIDGenerator.bufferXIDs([II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn buffer_x_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _array_size = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XIDGenerator.bufferXIDs([II)V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XIDGenerator.bufferXIDs([II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn buffer_xids_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _array_size = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XIDGenerator.bufferXIDs([II)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_x_ids() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = buffer_x_ids(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XIDGenerator.bufferXIDs([II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_buffer_xids_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = buffer_xids_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XIDGenerator.bufferXIDs([II)V",
            result.unwrap_err().to_string()
        );
    }
}
