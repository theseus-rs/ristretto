use ristretto_classfile::JAVA_14;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::collections::HashMap;
use std::ptr::null;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
};

const INVALID_HANDLE_VALUE: i64 = -1;
// Keep emulated completion-port handles disjoint from the native Win32 handles used as managed
// file identifiers. In Windows' real kernel handle table those namespaces cannot collide either.
const FIRST_COMPLETION_PORT: i64 = 0x6100_0000_0000_0000;
const ERROR_INVALID_FUNCTION: i32 = 1;
const ERROR_INVALID_HANDLE: i32 = 6;
const ERROR_INVALID_PARAMETER: i32 = 87;

/// Completion packet consumed by the Java `Iocp` event-handler threads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CompletionPacket {
    pub(crate) error: i32,
    pub(crate) bytes_transferred: i32,
    pub(crate) completion_key: i32,
    pub(crate) overlapped: i64,
}

#[derive(Debug)]
struct CompletionPort {
    sender: tokio::sync::mpsc::UnboundedSender<CompletionPacket>,
    receiver: tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<CompletionPacket>>,
}

#[derive(Clone, Copy, Debug)]
struct Association {
    port: i64,
    completion_key: i32,
    closed: bool,
    pending: usize,
    generation: u64,
}

/// Immutable IOCP route captured when an overlapped operation is initiated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CompletionTarget {
    handle: i64,
    port: i64,
    completion_key: i32,
    generation: u64,
}

/// Per-VM emulation of the Windows I/O completion-port namespace.
///
/// Java-visible file and socket handles in Ristretto are managed identifiers. Keeping the IOCP
/// registry beside those identifiers avoids treating them as raw Win32 handles and also isolates
/// independent VM instances used by the test harness.
#[derive(Debug)]
pub(crate) struct IocpState {
    next_port: AtomicI64,
    next_generation: AtomicU64,
    ports: Mutex<HashMap<i64, Arc<CompletionPort>>>,
    associations: Mutex<HashMap<i64, Association>>,
}

impl IocpState {
    fn new() -> Self {
        Self {
            next_port: AtomicI64::new(FIRST_COMPLETION_PORT),
            next_generation: AtomicU64::new(1),
            ports: Mutex::new(HashMap::new()),
            associations: Mutex::new(HashMap::new()),
        }
    }

    fn create_or_associate(
        &self,
        handle: i64,
        existing_port: i64,
        completion_key: i32,
    ) -> std::result::Result<i64, i32> {
        let mut associations = if handle == INVALID_HANDLE_VALUE {
            None
        } else {
            let associations = self
                .associations
                .lock()
                .map_err(|_| ERROR_INVALID_FUNCTION)?;
            if associations
                .get(&handle)
                .is_some_and(|association| !association.closed)
            {
                return Err(ERROR_INVALID_PARAMETER);
            }
            Some(associations)
        };
        let port_handle = if existing_port == 0 {
            let handle = self.next_port.fetch_add(1, Ordering::Relaxed);
            let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
            let port = Arc::new(CompletionPort {
                sender,
                receiver: tokio::sync::Mutex::new(receiver),
            });
            self.ports
                .lock()
                .map_err(|_| ERROR_INVALID_FUNCTION)?
                .insert(handle, port);
            handle
        } else {
            if !self
                .ports
                .lock()
                .map_err(|_| ERROR_INVALID_FUNCTION)?
                .contains_key(&existing_port)
            {
                return Err(ERROR_INVALID_HANDLE);
            }
            existing_port
        };

        // INVALID_HANDLE_VALUE creates a completion port without associating an object.
        if let Some(associations) = associations.as_mut() {
            // Windows may reuse a numeric handle after close. A closed association remains long
            // enough to route cancellation packets, then is replaced when that value is reused.
            associations.insert(
                handle,
                Association {
                    port: port_handle,
                    completion_key,
                    closed: false,
                    pending: 0,
                    generation: self.next_generation.fetch_add(1, Ordering::Relaxed),
                },
            );
        }

        Ok(port_handle)
    }

    fn port(&self, handle: i64) -> std::result::Result<Arc<CompletionPort>, i32> {
        self.ports
            .lock()
            .map_err(|_| ERROR_INVALID_FUNCTION)?
            .get(&handle)
            .cloned()
            .ok_or(ERROR_INVALID_HANDLE)
    }

    fn close(&self, handle: i64) -> bool {
        let removed = self
            .ports
            .lock()
            .is_ok_and(|mut ports| ports.remove(&handle).is_some());
        if removed && let Ok(mut associations) = self.associations.lock() {
            associations.retain(|_, association| association.port != handle);
        }
        removed
    }

    fn post_to_port(
        &self,
        port_handle: i64,
        packet: CompletionPacket,
    ) -> std::result::Result<(), i32> {
        let port = self.port(port_handle)?;
        port.sender.send(packet).map_err(|_| ERROR_INVALID_HANDLE)
    }

    fn post_for_handle(
        &self,
        handle: i64,
        error: i32,
        bytes_transferred: i32,
        overlapped: i64,
    ) -> std::result::Result<(), i32> {
        let association = self
            .associations
            .lock()
            .map_err(|_| ERROR_INVALID_FUNCTION)?
            .get(&handle)
            .copied()
            .ok_or(ERROR_INVALID_HANDLE)?;
        self.post_to_port(
            association.port,
            CompletionPacket {
                error,
                bytes_transferred,
                completion_key: association.completion_key,
                overlapped,
            },
        )
    }

    fn is_associated(&self, handle: i64) -> bool {
        self.associations.lock().is_ok_and(|associations| {
            associations
                .get(&handle)
                .is_some_and(|association| !association.closed)
        })
    }

    fn begin_operation(&self, handle: i64) -> std::result::Result<CompletionTarget, i32> {
        let mut associations = self
            .associations
            .lock()
            .map_err(|_| ERROR_INVALID_FUNCTION)?;
        let association = associations
            .get_mut(&handle)
            .filter(|association| !association.closed)
            .ok_or(ERROR_INVALID_HANDLE)?;
        association.pending = association
            .pending
            .checked_add(1)
            .ok_or(ERROR_INVALID_FUNCTION)?;
        Ok(CompletionTarget {
            handle,
            port: association.port,
            completion_key: association.completion_key,
            generation: association.generation,
        })
    }

    fn operation_is_open(&self, target: CompletionTarget) -> bool {
        self.associations.lock().is_ok_and(|associations| {
            associations.get(&target.handle).is_some_and(|association| {
                association.generation == target.generation && !association.closed
            })
        })
    }

    fn finish_operation(&self, target: CompletionTarget) {
        if let Ok(mut associations) = self.associations.lock() {
            let remove = if let Some(association) = associations.get_mut(&target.handle)
                && association.generation == target.generation
            {
                association.pending = association.pending.saturating_sub(1);
                association.closed && association.pending == 0
            } else {
                false
            };
            if remove {
                associations.remove(&target.handle);
            }
        }
    }

    fn mark_closed(&self, handle: i64) {
        if let Ok(mut associations) = self.associations.lock() {
            let remove = if let Some(association) = associations.get_mut(&handle) {
                association.closed = true;
                association.pending == 0
            } else {
                false
            };
            if remove {
                associations.remove(&handle);
            }
        }
    }
}

fn state<V: VM + ?Sized>(vm: &V) -> Result<Arc<IocpState>> {
    vm.resource_manager().get_or_init(IocpState::new)
}

/// Creates a completion port or associates an existing VM handle with one.
pub(crate) fn create_or_associate<V: VM + ?Sized>(
    vm: &V,
    handle: i64,
    existing_port: i64,
    completion_key: i32,
) -> Result<i64> {
    state(vm)?
        .create_or_associate(handle, existing_port, completion_key)
        .map_err(windows_error)
}

/// Removes a VM completion port. Windows' close wrappers deliberately ignore invalid handles.
pub(crate) fn close_port<V: VM + ?Sized>(vm: &V, handle: i64) -> Result<bool> {
    Ok(state(vm)?.close(handle))
}

/// Receives the next packet from a VM completion port.
pub(crate) async fn receive<V: VM + ?Sized>(vm: &V, port_handle: i64) -> Result<CompletionPacket> {
    let port = state(vm)?.port(port_handle).map_err(windows_error)?;
    port.receiver
        .lock()
        .await
        .recv()
        .await
        .ok_or_else(|| windows_error(ERROR_INVALID_HANDLE))
}

/// Posts an explicit packet to a VM completion port.
pub(crate) fn post_to_port<V: VM + ?Sized>(
    vm: &V,
    port_handle: i64,
    packet: CompletionPacket,
) -> Result<()> {
    state(vm)?
        .post_to_port(port_handle, packet)
        .map_err(windows_error)
}

/// Returns whether `handle` is currently associated with an open IOCP.
pub(crate) fn is_associated<V: VM + ?Sized>(vm: &V, handle: i64) -> Result<bool> {
    Ok(state(vm)?.is_associated(handle))
}

/// Captures the completion route and accounts for one pending overlapped operation.
pub(crate) fn begin_operation<V: VM + ?Sized>(vm: &V, handle: i64) -> Result<CompletionTarget> {
    state(vm)?.begin_operation(handle).map_err(windows_error)
}

/// Returns whether the object generation that initiated `target` is still open.
pub(crate) fn operation_is_open<V: VM + ?Sized>(vm: &V, target: CompletionTarget) -> Result<bool> {
    Ok(state(vm)?.operation_is_open(target))
}

/// Marks an associated object closed while retaining its route for cancellation completions.
pub(crate) fn mark_closed<V: VM + ?Sized>(vm: &V, handle: i64) {
    if let Ok(state) = state(vm) {
        state.mark_closed(handle);
    }
}

/// Posts one operation result to the route captured by [`begin_operation`].
pub(crate) fn post_operation<V: VM + ?Sized>(
    vm: &V,
    target: CompletionTarget,
    error: i32,
    bytes_transferred: usize,
    overlapped: i64,
) -> Result<()> {
    let bytes_transferred = u32::try_from(bytes_transferred).unwrap_or(u32::MAX);
    let bytes_transferred = i32::from_ne_bytes(bytes_transferred.to_ne_bytes());
    let state = state(vm)?;
    let result = state
        .post_to_port(
            target.port,
            CompletionPacket {
                error,
                bytes_transferred,
                completion_key: target.completion_key,
                overlapped,
            },
        )
        .map_err(windows_error);
    state.finish_operation(target);
    result
}

/// Finishes accounting for an operation that could not be issued and therefore has no packet.
pub(crate) fn abandon_operation<V: VM + ?Sized>(vm: &V, target: CompletionTarget) -> Result<()> {
    state(vm)?.finish_operation(target);
    Ok(())
}

/// Posts the completion of an overlapped operation issued against `handle`.
pub(crate) fn post_completion<V: VM + ?Sized>(
    vm: &V,
    handle: i64,
    error: i32,
    bytes_transferred: usize,
    overlapped: i64,
) -> Result<()> {
    let bytes_transferred = u32::try_from(bytes_transferred).unwrap_or(u32::MAX);
    let bytes_transferred = i32::from_ne_bytes(bytes_transferred.to_ne_bytes());
    state(vm)?
        .post_for_handle(handle, error, bytes_transferred, overlapped)
        .map_err(windows_error)
}

/// Converts an I/O error into the integer error delivered in an IOCP packet.
pub(crate) fn io_error_code(error: &std::io::Error) -> i32 {
    error.raw_os_error().unwrap_or(ERROR_INVALID_FUNCTION)
}

fn windows_error(code: i32) -> ristretto_types::Error {
    let error_code = u32::from_ne_bytes(code.to_ne_bytes());
    let message =
        format_system_message::<256>(error_code, true, true).unwrap_or_else(|| format_error(code));
    JavaError::IoException(message).into()
}

fn format_error(code: i32) -> String {
    std::io::Error::from_raw_os_error(code).to_string()
}

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

async fn is_valid_io_handle<T: Thread + 'static>(thread: &Arc<T>, handle: i64) -> Result<bool> {
    if handle == INVALID_HANDLE_VALUE {
        return Ok(true);
    }

    let vm = thread.vm()?;
    if vm.file_handles().get(&handle).await.is_some() {
        return Ok(true);
    }
    let Ok(socket_descriptor) = i32::try_from(handle) else {
        return Ok(false);
    };
    Ok(vm.socket_handles().get(&socket_descriptor).await.is_some())
}

#[intrinsic_method("sun/nio/ch/Iocp.close0(J)V", Any)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    // OpenJDK deliberately ignores CloseHandle's result for this native method.
    state(thread.vm()?.as_ref())?.close(handle);
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Iocp.createIoCompletionPort(JJII)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _concurrency = parameters.pop_int()?;
    let completion_key = parameters.pop_int()?;
    let existing_port = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    if !is_valid_io_handle(&thread, handle).await? {
        return Err(windows_error(ERROR_INVALID_HANDLE));
    }
    let port = state(thread.vm()?.as_ref())?
        .create_or_associate(handle, existing_port, completion_key)
        .map_err(windows_error)?;
    Ok(Some(Value::Long(port)))
}

#[intrinsic_method("sun/nio/ch/Iocp.getErrorMessage(I)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_error_message<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let error_code = parameters.pop_int()?;
    let native_error_code = u32::from_ne_bytes(error_code.to_ne_bytes());
    // JDK 8 through 13 returned FormatMessageW's trailing punctuation/newlines. JDK 14 and newer
    // remove a final period followed by CR/LF.
    let trim_suffix = thread.vm()?.java_class_file_version() >= &JAVA_14;
    let Some(message) = format_system_message::<255>(native_error_code, false, trim_suffix) else {
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let status = parameters
        .pop_reference()?
        .ok_or(JavaError::NullPointerException(None))?;
    let completion_port = parameters.pop_long()?;
    let port = state(thread.vm()?.as_ref())?
        .port(completion_port)
        .map_err(windows_error)?;
    let packet = port
        .receiver
        .lock()
        .await
        .recv()
        .await
        .ok_or_else(|| windows_error(ERROR_INVALID_HANDLE))?;

    let mut guard = status.write();
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "Iocp.getQueuedCompletionStatus: status is not an object".to_string(),
        ));
    };
    object.set_value("error", Value::Int(packet.error))?;
    object.set_value("bytesTransferred", Value::Int(packet.bytes_transferred))?;
    object.set_value("completionKey", Value::Int(packet.completion_key))?;
    object.set_value("overlapped", Value::Long(packet.overlapped))?;
    Ok(None)
}

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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let completion_key = parameters.pop_int()?;
    let completion_port = parameters.pop_long()?;
    state(thread.vm()?.as_ref())?
        .post_to_port(
            completion_port,
            CompletionPacket {
                error: 0,
                bytes_transferred: 0,
                completion_key,
                overlapped: 0,
            },
        )
        .map_err(windows_error)?;
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
                Value::Long(INVALID_HANDLE_VALUE),
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
        assert_eq!(
            None,
            close0(thread, Parameters::new(vec![Value::Long(0)])).await?
        );
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
    async fn test_get_queued_completion_status_null_status() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::JavaError(
                JavaError::NullPointerException(_)
            ))
        ));
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

    #[tokio::test]
    async fn test_create_post_and_close_completion_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let port = create_io_completion_port(
            thread.clone(),
            Parameters::new(vec![
                Value::Long(INVALID_HANDLE_VALUE),
                Value::Long(0),
                Value::Int(0),
                Value::Int(1),
            ]),
        )
        .await?
        .expect("port")
        .as_i64()?;
        assert!(port > 0);

        post_queued_completion_status(
            thread.clone(),
            Parameters::new(vec![Value::Long(port), Value::Int(42)]),
        )
        .await?;

        let port_state = state(thread.vm()?.as_ref())?
            .port(port)
            .map_err(windows_error)?;
        let packet = port_state
            .receiver
            .lock()
            .await
            .recv()
            .await
            .expect("packet");
        assert_eq!(42, packet.completion_key);
        assert_eq!(0, packet.overlapped);

        close0(thread, Parameters::new(vec![Value::Long(port)])).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_associated_completion() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let vm = thread.vm()?;
        let state = state(vm.as_ref())?;
        let port = state
            .create_or_associate(INVALID_HANDLE_VALUE, 0, 0)
            .map_err(windows_error)?;
        state
            .create_or_associate(123, port, 7)
            .map_err(windows_error)?;
        post_completion(vm.as_ref(), 123, 0, 9, 456)?;
        let packet = state
            .port(port)
            .map_err(windows_error)?
            .receiver
            .lock()
            .await
            .recv()
            .await
            .expect("packet");
        assert_eq!(
            CompletionPacket {
                error: 0,
                bytes_transferred: 9,
                completion_key: 7,
                overlapped: 456,
            },
            packet
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_association_close_and_handle_reuse() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let vm = thread.vm()?;
        let state = state(vm.as_ref())?;
        let first_port = state
            .create_or_associate(INVALID_HANDLE_VALUE, 0, 0)
            .map_err(windows_error)?;
        state
            .create_or_associate(321, first_port, 11)
            .map_err(windows_error)?;
        assert_eq!(
            Err(ERROR_INVALID_PARAMETER),
            state.create_or_associate(321, first_port, 12)
        );

        let pending = begin_operation(vm.as_ref(), 321)?;
        mark_closed(vm.as_ref(), 321);
        assert!(!is_associated(vm.as_ref(), 321)?);
        post_operation(vm.as_ref(), pending, ERROR_INVALID_HANDLE, 0, 654)?;
        let cancellation = state
            .port(first_port)
            .map_err(windows_error)?
            .receiver
            .lock()
            .await
            .recv()
            .await
            .expect("cancellation packet");
        assert_eq!(11, cancellation.completion_key);
        assert_eq!(654, cancellation.overlapped);

        let second_port = state
            .create_or_associate(INVALID_HANDLE_VALUE, 0, 0)
            .map_err(windows_error)?;
        state
            .create_or_associate(321, second_port, 22)
            .map_err(windows_error)?;
        assert!(is_associated(vm.as_ref(), 321)?);
        post_completion(vm.as_ref(), 321, 0, 1, 987)?;
        let reused = state
            .port(second_port)
            .map_err(windows_error)?
            .receiver
            .lock()
            .await
            .recv()
            .await
            .expect("reused-handle packet");
        assert_eq!(22, reused.completion_key);
        assert_eq!(987, reused.overlapped);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        assert_eq!(None, init_ids(thread, Parameters::default()).await?);
        Ok(())
    }
}
