use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ready = parameters.pop_bool()?;
    let _block = parameters.pop_bool()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/sctp/SctpChannelImpl.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpChannelImpl.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.receive0(ILsun/nio/ch/sctp/ResultContainer;JIZ)I",
    Any
)]
#[async_method]
pub async fn receive0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _peek = parameters.pop_bool()?;
    let _length = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _result_container = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpChannelImpl.receive0(ILsun/nio/ch/sctp/ResultContainer;JIZ)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.send0(IJILjava/net/InetAddress;IIIZI)I",
    Any
)]
#[async_method]
pub async fn send0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ppid = parameters.pop_int()?;
    let _unordered = parameters.pop_bool()?;
    let _stream_number = parameters.pop_int()?;
    let _assoc_id = parameters.pop_int()?;
    let _port = parameters.pop_int()?;
    let _addr = parameters.pop_reference()?;
    let _length = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpChannelImpl.send0(IJILjava/net/InetAddress;IIIZI)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_check_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_connect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpChannelImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_receive0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpChannelImpl.receive0(ILsun/nio/ch/sctp/ResultContainer;JIZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_send0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpChannelImpl.send0(IJILjava/net/InetAddress;IIIZI)I",
            result.unwrap_err().to_string()
        );
    }
}
