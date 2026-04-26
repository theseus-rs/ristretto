use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn accept_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _array = parameters.pop_reference()?;
    let _newfdo = parameters.pop_reference()?;
    let _fdo = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.init()Z", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.UnixDomainSockets.init()Z".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn local_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/UnixDomainSockets.socket0()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn socket_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.UnixDomainSockets.socket0()I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.init()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_address_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.UnixDomainSockets.socket0()I",
            result.unwrap_err().to_string()
        );
    }
}
