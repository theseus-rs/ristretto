use ristretto_classfile::VersionSpecification::{Between, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_permissions<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.close(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.close(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.connect(ILjava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_attach_file_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.getTempDir()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_temp_dir<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.getTempDir()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.read(I[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn read<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.read(I[BII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.sendQuitTo(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_quit_to<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.socket()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn socket<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.socket()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.write(I[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn write<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.write(I[BII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z",
    Between(JAVA_17, JAVA_25)
)]
#[async_method]
pub async fn check_catches_and_send_quit_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _send_sigquit = parameters.pop_bool()?;
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_permissions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_permissions(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_attach_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_attach_file_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_temp_dir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_temp_dir(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_quit_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_quit_to(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_catches_and_send_quit_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::new(vec![Value::Int(1234), Value::from(true)]);
        let result = check_catches_and_send_quit_to(thread, parameters).await;
        assert!(result.is_err());
    }
}
