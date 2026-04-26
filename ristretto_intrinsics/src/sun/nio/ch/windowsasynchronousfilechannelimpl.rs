use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, Equal};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousFileChannelImpl.close0(J)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousFileChannelImpl.close0(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.lockFile(JJJZJ)I", Any)]
#[async_method]
pub async fn lock_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _shared = parameters.pop_bool()?;
    let _size = parameters.pop_long()?;
    let _pos = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousFileChannelImpl.lockFile(JJJZJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.readFile(JJIJJ)I", Any)]
#[async_method]
pub async fn read_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _offset = parameters.pop_long()?;
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousFileChannelImpl.readFile(JJIJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousFileChannelImpl.writeFile(JJIJJ)I", Any)]
#[async_method]
pub async fn write_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _offset = parameters.pop_long()?;
    let _len = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousFileChannelImpl.writeFile(JJIJJ)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousFileChannelImpl.close0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lock_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousFileChannelImpl.lockFile(JJJZJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousFileChannelImpl.readFile(JJIJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_file(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousFileChannelImpl.writeFile(JJIJJ)I",
            result.unwrap_err().to_string()
        );
    }
}
