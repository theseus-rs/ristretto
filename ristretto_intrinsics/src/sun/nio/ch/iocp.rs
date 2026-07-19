use ristretto_classfile::JAVA_14;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Parameters, Result, Thread, VM};
use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::Arc;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
};
use windows_sys::Win32::System::IO::{
    CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED, PostQueuedCompletionStatus,
};
use windows_sys::Win32::System::Threading::INFINITE;

#[derive(Debug)]
struct CompletionStatus {
    error: i32,
    bytes_transferred: i32,
    completion_key: i32,
    overlapped: i64,
}

/// Converts the `jlong` representation used by `OpenJDK` into a Win32 handle.
#[expect(
    clippy::cast_possible_truncation,
    reason = "jlong_to_ptr has the same pointer-width truncation on 32-bit Windows"
)]
fn handle_from_jlong(handle: i64) -> HANDLE {
    handle as isize as *mut c_void
}

/// Converts a Win32 handle into the `jlong` representation used by `OpenJDK`.
fn handle_to_jlong(handle: HANDLE) -> i64 {
    handle as isize as i64
}

/// Formats a Win32 system message using the same fixed-size buffer as `OpenJDK`.
#[expect(unsafe_code)]
fn format_system_message<const BUFFER_LENGTH: usize>(
    error_code: u32,
    ignore_inserts: bool,
    trim_suffix: bool,
) -> Option<String> {
    let mut message = [0_u16; BUFFER_LENGTH];
    let mut flags = FORMAT_MESSAGE_FROM_SYSTEM;
    if ignore_inserts {
        flags |= FORMAT_MESSAGE_IGNORE_INSERTS;
    }
    let length = unsafe {
        FormatMessageW(
            flags,
            null(),
            error_code,
            0,
            message.as_mut_ptr(),
            u32::try_from(message.len()).ok()?,
            null(),
        )
    };
    if length == 0 {
        return None;
    }

    let mut length = usize::try_from(length).ok()?;
    if trim_suffix && length > 3 {
        if message.get(length.wrapping_sub(1)) == Some(&u16::from(b'\n')) {
            length -= 1;
        }
        if message.get(length.wrapping_sub(1)) == Some(&u16::from(b'\r')) {
            length -= 1;
        }
        if message.get(length.wrapping_sub(1)) == Some(&u16::from(b'.')) {
            length -= 1;
        }
    }
    Some(String::from_utf16_lossy(message.get(..length)?))
}

fn io_exception(error_code: u32, default_message: &str) -> ristretto_types::Error {
    // OpenJDK's Windows last-error helper uses a 256-wide-character buffer.
    let message = format_system_message::<256>(error_code, true, true)
        .unwrap_or_else(|| default_message.to_string());
    JavaError::IoException(message).into()
}

/// Resolves Ristretto's synthetic socket descriptor to the Win32 `SOCKET` value expected by
/// `CreateIoCompletionPort`. File handles are already stored as their native Win32 values.
async fn native_io_handle<T: Thread + 'static>(thread: &Arc<T>, handle: i64) -> Result<i64> {
    let Ok(socket_descriptor) = i32::try_from(handle) else {
        return Ok(handle);
    };
    let vm = thread.vm()?;
    // Native file handles can be numerically small enough to overlap the VM's synthetic socket
    // descriptor range. A registered file handle is already native and must take precedence.
    if vm.file_handles().get(&handle).await.is_some() {
        return Ok(handle);
    }
    let Some(socket) = vm.socket_handles().get(&socket_descriptor).await else {
        return Ok(handle);
    };
    #[expect(
        clippy::cast_possible_wrap,
        reason = "Win32 SOCKET bits are passed through a Java long"
    )]
    let native_handle = socket.socket_type.raw_socket() as i64;
    Ok(native_handle)
}

#[expect(unsafe_code)]
fn create_port(
    handle: i64,
    existing_port: i64,
    completion_key: i32,
    concurrency: i32,
) -> Result<i64> {
    // C's conversion from jint to ULONG_PTR sign-extends a negative completion key before
    // converting it to the pointer-sized unsigned value.
    #[expect(
        clippy::cast_sign_loss,
        reason = "matches OpenJDK's jint-to-ULONG_PTR conversion"
    )]
    let completion_key = completion_key as isize as usize;
    let concurrency = u32::from_ne_bytes(concurrency.to_ne_bytes());
    let port = unsafe {
        CreateIoCompletionPort(
            handle_from_jlong(handle),
            handle_from_jlong(existing_port),
            completion_key,
            concurrency,
        )
    };
    if port.is_null() {
        let error_code = unsafe { GetLastError() };
        return Err(io_exception(error_code, "CreateIoCompletionPort failed"));
    }
    Ok(handle_to_jlong(port))
}

#[expect(unsafe_code)]
fn close_handle(handle: i64) {
    // OpenJDK deliberately ignores CloseHandle's result for this native method.
    unsafe {
        CloseHandle(handle_from_jlong(handle));
    }
}

#[expect(unsafe_code)]
fn wait_for_completion(completion_port: i64) -> std::result::Result<CompletionStatus, u32> {
    let mut bytes_transferred = 0_u32;
    let mut completion_key = 0_usize;
    let mut overlapped: *mut OVERLAPPED = null_mut();
    let succeeded = unsafe {
        GetQueuedCompletionStatus(
            handle_from_jlong(completion_port),
            &raw mut bytes_transferred,
            &raw mut completion_key,
            &raw mut overlapped,
            INFINITE,
        )
    };
    let error_code = if succeeded == 0 {
        unsafe { GetLastError() }
    } else {
        0
    };

    // A null OVERLAPPED on failure means that no completion packet was dequeued. If an
    // OVERLAPPED is present, the API call successfully dequeued a failed I/O operation and its
    // error code belongs in CompletionStatus rather than being thrown here.
    if succeeded == 0 && overlapped.is_null() {
        return Err(error_code);
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        reason = "OpenJDK stores the pointer-sized completion key in a jint field"
    )]
    let completion_key = completion_key as i32;
    Ok(CompletionStatus {
        error: i32::from_ne_bytes(error_code.to_ne_bytes()),
        bytes_transferred: i32::from_ne_bytes(bytes_transferred.to_ne_bytes()),
        completion_key,
        overlapped: overlapped as isize as i64,
    })
}

#[expect(unsafe_code)]
fn post_completion(completion_port: i64, completion_key: i32) -> std::result::Result<(), u32> {
    let completion_key = u32::from_ne_bytes(completion_key.to_ne_bytes());
    let succeeded = unsafe {
        PostQueuedCompletionStatus(
            handle_from_jlong(completion_port),
            0,
            completion_key as usize,
            null(),
        )
    };
    if succeeded == 0 {
        return Err(unsafe { GetLastError() });
    }
    Ok(())
}

#[intrinsic_method("sun/nio/ch/Iocp.close0(J)V", Any)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    close_handle(handle);
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Iocp.createIoCompletionPort(JJII)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let concurrency = parameters.pop_int()?;
    let completion_key = parameters.pop_int()?;
    let existing_port = parameters.pop_long()?;
    let handle = native_io_handle(&thread, parameters.pop_long()?).await?;
    let port = create_port(handle, existing_port, completion_key, concurrency)?;
    Ok(Some(Value::Long(port)))
}

#[intrinsic_method("sun/nio/ch/Iocp.getErrorMessage(I)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_error_message<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let error_code = parameters.pop_int()?;
    let error_code = u32::from_ne_bytes(error_code.to_ne_bytes());
    // JDK 8 through 13 returned FormatMessageW's trailing punctuation/newlines. JDK 14 and newer
    // remove a final period followed by CR/LF.
    let trim_suffix = thread.vm()?.java_class_file_version() >= &JAVA_14;
    let Some(message) = format_system_message::<255>(error_code, false, trim_suffix) else {
        return Ok(Some(Value::Object(None)));
    };
    Ok(Some(thread.intern_string(&message).await?))
}

#[intrinsic_method(
    "sun/nio/ch/Iocp.getQueuedCompletionStatus(JLsun/nio/ch/Iocp$CompletionStatus;)V",
    Any
)]
#[async_method]
pub async fn get_queued_completion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let status = parameters.pop_reference()?;
    let completion_port = parameters.pop_long()?;
    let completion = tokio::task::spawn_blocking(move || wait_for_completion(completion_port))
        .await
        .map_err(|error| {
            InternalError(format!(
                "GetQueuedCompletionStatus blocking task failed: {error}"
            ))
        })?
        .map_err(|error_code| io_exception(error_code, "GetQueuedCompletionStatus failed"))?;

    let status = status.ok_or(JavaError::NullPointerException(None))?;
    let mut guard = status.write();
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "Iocp.getQueuedCompletionStatus: status is not an object".to_string(),
        ));
    };
    object.set_value("error", Value::Int(completion.error))?;
    object.set_value("bytesTransferred", Value::Int(completion.bytes_transferred))?;
    object.set_value("completionKey", Value::Int(completion.completion_key))?;
    object.set_value("overlapped", Value::Long(completion.overlapped))?;
    Ok(None)
}

/// `OpenJDK` uses this method only to cache JNI field identifiers. Ristretto resolves fields by
/// name when writing the completion status, so no initialization is required.
#[intrinsic_method("sun/nio/ch/Iocp.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Iocp.postQueuedCompletionStatus(JI)V", Any)]
#[async_method]
pub async fn post_queued_completion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let completion_key = parameters.pop_int()?;
    let completion_port = parameters.pop_long()?;
    post_completion(completion_port, completion_key)
        .map_err(|error_code| io_exception(error_code, "PostQueuedCompletionStatus"))?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;

    async fn new_completion_status(thread: &Arc<ristretto_vm::Thread>) -> Result<Value> {
        let vm = thread.vm()?;
        let class = thread.class("sun/nio/ch/Iocp$CompletionStatus").await?;
        let object = Object::new(class)?;
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(object),
        ))
    }

    async fn new_port(thread: Arc<ristretto_vm::Thread>) -> Result<i64> {
        let port = create_io_completion_port(
            thread,
            Parameters::new(vec![
                Value::Long(-1),
                Value::Long(0),
                Value::Int(0),
                Value::Int(1),
            ]),
        )
        .await?
        .ok_or_else(|| InternalError("createIoCompletionPort returned void".to_string()))?
        .as_i64()?;
        Ok(port)
    }

    #[tokio::test]
    async fn test_close0_ignores_invalid_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = close0(thread, Parameters::new(vec![Value::Long(0)])).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_and_close_io_completion_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let port = new_port(thread.clone()).await?;
        assert_ne!(0, port);
        close0(thread, Parameters::new(vec![Value::Long(port)])).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_io_completion_port_invalid_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = create_io_completion_port(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(1),
            ]),
        )
        .await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(JavaError::IoException(_)))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_associate_managed_socket_with_io_completion_port() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let port = new_port(thread.clone()).await?;
        let socket_descriptor = crate::sun::nio::ch::net::socket_0(
            thread.clone(),
            Parameters::new(vec![
                Value::from(false),
                Value::from(true),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await?
        .ok_or_else(|| InternalError("Net.socket0 returned void".to_string()))?
        .as_i32()?;

        let result = create_io_completion_port(
            thread.clone(),
            Parameters::new(vec![
                Value::Long(i64::from(socket_descriptor)),
                Value::Long(port),
                Value::Int(27),
                Value::Int(0),
            ]),
        )
        .await?;
        assert_eq!(Some(Value::Long(port)), result);

        vm.socket_handles().remove(&socket_descriptor).await;
        close0(thread, Parameters::new(vec![Value::Long(port)])).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_error_message() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_error_message(thread, Parameters::new(vec![Value::Int(2)])).await?;
        let message = result.ok_or_else(|| InternalError("missing result".to_string()))?;
        assert!(!message.as_string()?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_error_message_unknown_code_returns_null() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_error_message(thread, Parameters::new(vec![Value::Int(i32::MAX)])).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_and_get_queued_completion_status() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let port = new_port(thread.clone()).await?;
        let status = new_completion_status(&thread).await?;

        post_queued_completion_status(
            thread.clone(),
            Parameters::new(vec![Value::Long(port), Value::Int(-17)]),
        )
        .await?;
        get_queued_completion_status(
            thread.clone(),
            Parameters::new(vec![Value::Long(port), status.clone()]),
        )
        .await?;

        let object = status.as_object_ref()?;
        assert_eq!(0, object.value("error")?.as_i32()?);
        assert_eq!(0, object.value("bytesTransferred")?.as_i32()?);
        assert_eq!(-17, object.value("completionKey")?.as_i32()?);
        assert_eq!(0, object.value("overlapped")?.as_i64()?);
        drop(object);
        close0(thread, Parameters::new(vec![Value::Long(port)])).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_queued_completion_status_invalid_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let status = new_completion_status(&thread).await?;
        let result =
            get_queued_completion_status(thread, Parameters::new(vec![Value::Long(0), status]))
                .await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(JavaError::IoException(_)))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_queued_completion_status_invalid_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = post_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(JavaError::IoException(_)))
        ));
        Ok(())
    }
}
