use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn accept_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I"
    );
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn bind_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V");
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn connect_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I");
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.init()Z", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.init()Z");
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn local_address_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B");
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.socket0()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn socket_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.socket0()I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I"
    )]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.init()Z")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B"
    )]
    async fn test_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.socket0()I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Parameters::default()).await;
    }
}
