use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_permissions(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.close(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.close(I)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.connect(ILjava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn connect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_attach_file_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.getTempDir()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_temp_dir(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.getTempDir()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.read(I[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn read(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.read(I[BII)I")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.sendQuitTo(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn send_quit_to(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.socket()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn socket(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.socket()I")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.write(I[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn write(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.write(I[BII)V")
}

#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_catches_and_send_quit_to(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _send_sigquit = parameters.pop_bool()?;
    let _pid = parameters.pop_int()?;
    todo!("sun.tools.attach.VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V"
    )]
    async fn test_check_permissions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_permissions(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.close(I)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V"
    )]
    async fn test_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V"
    )]
    async fn test_create_attach_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_attach_file_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.getTempDir()Ljava/lang/String;"
    )]
    async fn test_get_temp_dir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_temp_dir(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.read(I[BII)I"
    )]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V"
    )]
    async fn test_send_quit_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_quit_to(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.socket()I")]
    async fn test_socket() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.write(I[BII)V"
    )]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tools.attach.VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z"
    )]
    async fn test_check_catches_and_send_quit_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::new(vec![Value::Int(1234), Value::from(true)]);
        let _ = check_catches_and_send_quit_to(thread, parameters).await;
    }
}
