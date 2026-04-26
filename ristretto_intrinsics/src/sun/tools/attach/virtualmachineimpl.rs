#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "macos")]
use ristretto_classfile::JAVA_17;
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_21;
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_25;
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::Between;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_classfile::VersionSpecification::Equal;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Value;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::async_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::JavaError;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Thread;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::{Parameters, Result};
#[cfg(not(target_family = "wasm"))]
use std::sync::Arc;

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_permissions<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.close(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.close(I)V".to_string(),
    )
    .into())
}
#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.connect(ILjava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_attach_file_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "macos")]
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
#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.read(I[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn read<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf_len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.read(I[BII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.sendQuitTo(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn send_quit_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "macos")]
#[intrinsic_method("sun/tools/attach/VirtualMachineImpl.sendQuitTo(I)V", Equal(JAVA_11))]
#[async_method]
pub async fn send_quit_to_macos<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V".to_string(),
    )
    .into())
}
#[cfg(target_family = "unix")]
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
#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.write(I[BII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn write<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf_len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.write(I[BII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn check_catches_and_send_quit_to<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _throw_if_not_ready = parameters.pop_bool()?;
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tools.attach.VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.closePipe(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_pipe<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.closePipe(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_process = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect_pipe<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn create_pipe<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pipename = parameters.pop_reference()?;
    let _ver = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn create_pipe_windows_v11_v21_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pipename = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn enqueue<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _args = parameters.pop_reference()?;
    let _pipename = parameters.pop_reference()?;
    let _cmd = parameters.pop_reference()?;
    let _ver = parameters.pop_int()?;
    let _stub = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn enqueue_windows_v11_v21_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _args = parameters.pop_reference()?;
    let _pipename = parameters.pop_reference()?;
    let _cmd = parameters.pop_reference()?;
    let _stub = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.generateStub()[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_stub<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.generateStub()[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.init()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.init()V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.openProcess(I)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn open_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.openProcess(I)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn read_pipe<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ba_len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _ba = parameters.pop_reference()?;
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn write_pipe<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _length = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.closePipe(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_pipe_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.closePipe(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_process_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_process = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect_pipe_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn create_pipe_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pipename = parameters.pop_reference()?;
    let _ver = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn create_pipe_windows_v11_v21_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pipename = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn enqueue_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _args = parameters.pop_reference()?;
    let _pipename = parameters.pop_reference()?;
    let _cmd = parameters.pop_reference()?;
    let _ver = parameters.pop_int()?;
    let _stub = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_method]
pub async fn enqueue_windows_v11_v21_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _args = parameters.pop_reference()?;
    let _pipename = parameters.pop_reference()?;
    let _cmd = parameters.pop_reference()?;
    let _stub = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.generateStub()[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_stub_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.generateStub()[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.init()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/tools/attach/VirtualMachineImpl.init()V".to_string())
            .into(),
    )
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.openProcess(I)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn open_process_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pid = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.openProcess(I)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn read_pipe_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ba_len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _ba = parameters.pop_reference()?;
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn write_pipe_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _length = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    let _h_pipe = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_check_permissions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_permissions(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.close(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_create_attach_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_attach_file_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_get_temp_dir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_temp_dir(thread, Parameters::default()).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.getTempDir()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.read(I[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_send_quit_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send_quit_to(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_socket() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket(thread, Parameters::default()).await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.socket()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.write(I[BII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_check_catches_and_send_quit_to() {
        let (_vm, thread) = crate::test::java25_thread().await.expect("thread");
        let result = check_catches_and_send_quit_to(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.tools.attach.VirtualMachineImpl.checkCatchesAndSendQuitTo(IZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_pipe(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.closePipe(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_process(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect_pipe(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_pipe(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_pipe_windows_v11_v21_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            create_pipe_windows_v11_v21_v1(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enqueue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enqueue(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enqueue_windows_v11_v21_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enqueue_windows_v11_v21_v1(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_stub(thread, Parameters::default()).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.generateStub()[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_process(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.openProcess(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_pipe(
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
            "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_pipe(
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
            "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_pipe_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_pipe_windows_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.closePipe(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_process_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            close_process_windows_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.closeProcess(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect_pipe_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            connect_pipe_windows_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.connectPipe(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_pipe_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_pipe_windows_v25(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.createPipe(ILjava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_pipe_windows_v11_v21_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            create_pipe_windows_v11_v21_v2(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.createPipe(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enqueue_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enqueue_windows_v25(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.enqueue(J[BILjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enqueue_windows_v11_v21_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enqueue_windows_v11_v21_v2(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.enqueue(J[BLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_stub_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_stub_windows_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.generateStub()[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_windows_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_process_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            open_process_windows_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/tools/attach/VirtualMachineImpl.openProcess(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_pipe_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_pipe_windows_ge_v11(
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
            "sun/tools/attach/VirtualMachineImpl.readPipe(J[BII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write_pipe_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_pipe_windows_v25(
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
            "sun/tools/attach/VirtualMachineImpl.writePipe(J[BII)V",
            result.unwrap_err().to_string()
        );
    }
}
