use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/net/SocketOutputStream.init()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn socket_write_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V")
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
        expected = "not yet implemented: java.net.SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V"
    )]
    async fn test_socket_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_write_0(thread, Parameters::default()).await;
    }
}
