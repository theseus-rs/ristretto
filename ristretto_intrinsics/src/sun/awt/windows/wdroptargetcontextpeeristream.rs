use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WDropTargetContextPeerIStream.Available(J)I", Any)]
#[async_method]
pub async fn available<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _istream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeerIStream.Available(J)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDropTargetContextPeerIStream.Close(J)V", Any)]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _istream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeerIStream.Close(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDropTargetContextPeerIStream.Read(J)I", Any)]
#[async_method]
pub async fn read<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _istream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeerIStream.Read(J)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDropTargetContextPeerIStream.ReadBytes(J[BII)I", Any)]
#[async_method]
pub async fn read_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _istream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeerIStream.ReadBytes(J[BII)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = available(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeerIStream.Available(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeerIStream.Close(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeerIStream.Read(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_read_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_bytes(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeerIStream.ReadBytes(J[BII)I",
            result.unwrap_err().to_string()
        );
    }
}
