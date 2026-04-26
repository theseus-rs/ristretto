use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryConnection.close0(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryConnection.close0(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryConnection.receiveByte0(J)B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn receive_byte0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryConnection.receiveByte0(J)B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryConnection.receivePacket0(J)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn receive_packet0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryConnection.receivePacket0(J)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryConnection.sendByte0(JB)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_byte0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _b = parameters.pop_int()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryConnection.sendByte0(JB)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryConnection.sendPacket0(J[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_packet0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _b = parameters.pop_reference()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryConnection.sendPacket0(J[B)V".to_string(),
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
            "com/sun/tools/jdi/SharedMemoryConnection.close0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_receive_byte0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive_byte0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryConnection.receiveByte0(J)B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_receive_packet0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive_packet0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryConnection.receivePacket0(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_byte0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_byte0(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryConnection.sendByte0(JB)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send_packet0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_packet0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryConnection.sendPacket0(J[B)V",
            result.unwrap_err().to_string()
        );
    }
}
