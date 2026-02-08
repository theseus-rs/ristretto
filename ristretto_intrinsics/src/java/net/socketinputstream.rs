use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/net/SocketInputStream.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_read_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I"
    )]
    async fn test_socket_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_read_0(thread, Parameters::default()).await;
    }
}
