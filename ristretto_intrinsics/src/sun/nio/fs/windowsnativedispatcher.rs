#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    reason = "Win32 HANDLE/LUID values are transported through Java long bit patterns; API imports stay beside their platform calls"
)]

use crate::sun::nio::ch::iocp::{self, CompletionPacket};
use crate::sun::nio::fs::common::{throw_windows_exception, windows_error_code};
use crate::sun::nio::fs::managed_files;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::collections::HashMap;
use std::mem::{size_of, size_of_val};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

fn last_windows_error() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(1)
}

struct CopyCancelContext {
    memory: *const ristretto_types::NativeMemory,
    address: i64,
}

#[expect(unsafe_code)]
unsafe extern "system" fn copy_progress(
    _total_file_size: i64,
    _total_bytes_transferred: i64,
    _stream_size: i64,
    _stream_bytes_transferred: i64,
    _stream_number: u32,
    _callback_reason: u32,
    _source_file: windows_sys::Win32::Foundation::HANDLE,
    _destination_file: windows_sys::Win32::Foundation::HANDLE,
    data: *const std::ffi::c_void,
) -> u32 {
    if data.is_null() {
        return windows_sys::Win32::Storage::FileSystem::PROGRESS_CONTINUE;
    }
    let context = unsafe { &*data.cast::<CopyCancelContext>() };
    let memory = unsafe { &*context.memory };
    if memory
        .read_i32(context.address)
        .is_some_and(|value| value != 0)
    {
        windows_sys::Win32::Storage::FileSystem::PROGRESS_CANCEL
    } else {
        windows_sys::Win32::Storage::FileSystem::PROGRESS_CONTINUE
    }
}

#[derive(Clone, Default)]
struct GuestSecurityDescriptor {
    owner: Option<Vec<u8>>,
    owner_address: i64,
    dacl: Option<Vec<u8>>,
    dacl_address: i64,
}

#[derive(Default)]
struct GuestSecurityDescriptors(Mutex<HashMap<i64, GuestSecurityDescriptor>>);

fn security_descriptors<V: VM + ?Sized>(vm: &V) -> Result<Arc<GuestSecurityDescriptors>> {
    vm.resource_manager()
        .get_or_init(GuestSecurityDescriptors::default)
}

enum HostSecurityDescriptor {
    Absolute {
        descriptor: windows_sys::Win32::Security::SECURITY_DESCRIPTOR,
        _owner: Option<Vec<u8>>,
        _dacl: Option<Vec<u8>>,
    },
    Relative(Vec<u8>),
}

impl HostSecurityDescriptor {
    fn pointer(&mut self) -> windows_sys::Win32::Security::PSECURITY_DESCRIPTOR {
        match self {
            Self::Absolute { descriptor, .. } => std::ptr::from_mut(descriptor).cast(),
            Self::Relative(descriptor) => descriptor.as_mut_ptr().cast(),
        }
    }
}

fn host_security_descriptor<V: VM + ?Sized>(
    vm: &V,
    address: i64,
) -> Result<Option<HostSecurityDescriptor>> {
    if address == 0 {
        return Ok(None);
    }
    let state = security_descriptors(vm)?
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
        .get(&address)
        .cloned();
    if let Some(mut state) = state {
        let mut descriptor = windows_sys::Win32::Security::SECURITY_DESCRIPTOR {
            Revision: 1,
            ..Default::default()
        };
        if let Some(owner) = state.owner.as_mut() {
            descriptor.Owner = owner.as_mut_ptr().cast();
        }
        if let Some(dacl) = state.dacl.as_mut() {
            descriptor.Control |= windows_sys::Win32::Security::SE_DACL_PRESENT;
            descriptor.Dacl = dacl.as_mut_ptr().cast();
        }
        return Ok(Some(HostSecurityDescriptor::Absolute {
            descriptor,
            _owner: state.owner,
            _dacl: state.dacl,
        }));
    }
    Ok(Some(HostSecurityDescriptor::Relative(
        read_guest_allocation(vm, address)?,
    )))
}

#[derive(Default)]
struct GuestOverlappedOperation {
    result: Mutex<Option<(i32, u32)>>,
    notification: tokio::sync::Notify,
}

#[derive(Default)]
struct GuestOverlappedOperations(Mutex<HashMap<(i64, i64), Arc<GuestOverlappedOperation>>>);

fn overlapped_operations<V: VM + ?Sized>(vm: &V) -> Result<Arc<GuestOverlappedOperations>> {
    vm.resource_manager()
        .get_or_init(GuestOverlappedOperations::default)
}

struct BackupContexts {
    next: AtomicI64,
    pointers: Mutex<HashMap<i64, usize>>,
}

impl Default for BackupContexts {
    fn default() -> Self {
        Self {
            next: AtomicI64::new(1),
            pointers: Mutex::new(HashMap::new()),
        }
    }
}

fn backup_contexts<V: VM + ?Sized>(vm: &V) -> Result<Arc<BackupContexts>> {
    vm.resource_manager().get_or_init(BackupContexts::default)
}

fn begin_guest_overlapped<V: VM + ?Sized>(
    vm: &V,
    handle: i64,
    overlapped: i64,
) -> Result<Arc<GuestOverlappedOperation>> {
    let operation = Arc::new(GuestOverlappedOperation::default());
    let operations = overlapped_operations(vm)?;
    let mut operations = operations
        .0
        .lock()
        .map_err(|_| InternalError("poisoned overlapped operation map".to_string()))?;
    if let Some(previous) = operations.get(&(handle, overlapped))
        && previous
            .result
            .lock()
            .map_err(|_| InternalError("poisoned overlapped result".to_string()))?
            .is_none()
    {
        return Err(InternalError(
            "overlapped operation already pending".to_string(),
        ));
    }
    operations.insert((handle, overlapped), operation.clone());
    Ok(operation)
}

fn finish_guest_overlapped(operation: &GuestOverlappedOperation, error: i32, transferred: u32) {
    if let Ok(mut result) = operation.result.lock() {
        *result = Some((error, transferred));
    }
    operation.notification.notify_waiters();
}

fn read_guest_sid<V: VM + ?Sized>(vm: &V, address: i64) -> Result<Vec<u8>> {
    if address == 0 {
        return Err(InternalError("null SID address".to_string()));
    }
    let header = vm
        .native_memory()
        .try_read_bytes(address, 8)
        .ok_or_else(|| InternalError("invalid SID address".to_string()))?;
    let sub_authorities = usize::from(
        header
            .get(1)
            .copied()
            .ok_or_else(|| InternalError("truncated SID header".to_string()))?,
    );
    let length = 8usize
        .checked_add(
            sub_authorities
                .checked_mul(4)
                .ok_or_else(|| InternalError("SID sub-authority length overflow".to_string()))?,
        )
        .ok_or_else(|| InternalError("SID length overflow".to_string()))?;
    vm.native_memory()
        .try_read_bytes(address, length)
        .ok_or_else(|| InternalError("truncated SID".to_string()))
}

fn sid_string(sid: &[u8]) -> Result<String> {
    if sid.len() < 8 {
        return Err(InternalError("truncated SID".to_string()));
    }
    let revision = sid
        .first()
        .copied()
        .ok_or_else(|| InternalError("truncated SID revision".to_string()))?;
    let count = usize::from(
        sid.get(1)
            .copied()
            .ok_or_else(|| InternalError("truncated SID count".to_string()))?,
    );
    let authority = sid
        .get(2..8)
        .ok_or_else(|| InternalError("truncated SID authority".to_string()))?
        .iter()
        .fold(0u64, |value, byte| (value << 8) | u64::from(*byte));
    let mut result = format!("S-{revision}-{authority}");
    for index in 0..count {
        let start = 8 + index * 4;
        let bytes: [u8; 4] = sid
            .get(start..start + 4)
            .and_then(|value| value.try_into().ok())
            .ok_or_else(|| InternalError("truncated SID sub-authority".to_string()))?;
        result.push('-');
        result.push_str(&u32::from_le_bytes(bytes).to_string());
    }
    Ok(result)
}

fn read_guest_allocation<V: VM + ?Sized>(vm: &V, address: i64) -> Result<Vec<u8>> {
    let length = vm
        .native_memory()
        .remaining_len(address)
        .ok_or_else(|| InternalError("invalid native address".to_string()))?;
    vm.native_memory()
        .try_read_bytes(address, length)
        .ok_or_else(|| InternalError("invalid native allocation".to_string()))
}

fn read_guest_acl<V: VM + ?Sized>(vm: &V, address: i64) -> Result<Vec<u8>> {
    let header = vm
        .native_memory()
        .try_read_bytes(address, 8)
        .ok_or_else(|| InternalError("invalid ACL address".to_string()))?;
    let size = usize::from(
        read_u16(&header, 2).ok_or_else(|| InternalError("truncated ACL".to_string()))?,
    );
    if size < 8 {
        return Err(InternalError("invalid ACL size".to_string()));
    }
    vm.native_memory()
        .try_read_bytes(address, size)
        .ok_or_else(|| InternalError("truncated ACL allocation".to_string()))
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_ne_bytes(
        bytes.get(offset..offset + 2)?.try_into().ok()?,
    ))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_ne_bytes(
        bytes.get(offset..offset + 4)?.try_into().ok()?,
    ))
}

bitflags! {
    /// Windows generic access rights for `CreateFile`.
    ///
    /// See [Generic Access Rights](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights).
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct DesiredAccess: u32 {
        const GENERIC_WRITE = 0x4000_0000;
        const FILE_WRITE_ATTRIBUTES = 0x0000_0100;
        const FILE_WRITE_DATA = 0x0000_0002;
        const FILE_APPEND_DATA = 0x0000_0004;
        const FILE_WRITE_EA = 0x0000_0010;
    }
}

/// Windows file attribute constants.
///
/// See [File Attribute Constants](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants).
mod file_attribute {
    pub const READONLY: i32 = 0x01;
    pub const HIDDEN: i32 = 0x02;
    pub const SYSTEM: i32 = 0x04;
    pub const DIRECTORY: i32 = 0x10;
    pub const ARCHIVE: i32 = 0x20;
    pub const NORMAL: i32 = 0x80;
}

/// Bit mask for extracting the low 32 bits of a 64-bit file size.
const FILE_SIZE_LOW_MASK: u64 = 0xFFFF_FFFF;

/// Preserves the bit pattern of a Win32 `DWORD` when storing it in Java/native signed memory.
fn file_size_dword(value: u64) -> i32 {
    u32::try_from(value & FILE_SIZE_LOW_MASK)
        .unwrap_or_default()
        .cast_signed()
}

#[cfg(target_family = "windows")]
fn metadata_file_attributes(metadata: &std::fs::Metadata) -> i32 {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes().cast_signed()
}

#[cfg(not(target_family = "windows"))]
fn metadata_file_attributes(metadata: &std::fs::Metadata) -> i32 {
    if metadata.is_dir() {
        let mut attributes = file_attribute::DIRECTORY;
        if metadata.permissions().readonly() {
            attributes |= file_attribute::READONLY;
        }
        attributes
    } else if metadata.permissions().readonly() {
        file_attribute::READONLY
    } else {
        file_attribute::NORMAL
    }
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn set_windows_file_attributes(path: &std::path::Path, attributes: i32) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::SetFileAttributesW;

    let path: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    if unsafe { SetFileAttributesW(path.as_ptr(), attributes.cast_unsigned()) } == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(target_family = "windows"))]
fn set_windows_file_attributes(path: &std::path::Path, attributes: i32) -> std::io::Result<()> {
    let metadata = std::fs::symlink_metadata(path)?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(attributes & file_attribute::READONLY != 0);
    std::fs::set_permissions(path, permissions)
}

/// Read a null-terminated UTF-16 string from native memory at the given address.
fn read_native_string<T: Thread + 'static>(
    thread: &Arc<T>,
    address: i64,
    context: &str,
) -> Result<String> {
    let mut path_chars = read_native_wide(thread, address, context)?;
    path_chars.pop();
    String::from_utf16(&path_chars)
        .map_err(|error| InternalError(format!("{context}: invalid UTF-16: {error}")))
}

/// Reads a zero-terminated UTF-16 string, retaining its terminator for Win32 calls.
fn read_native_wide<T: Thread + 'static>(
    thread: &Arc<T>,
    address: i64,
    context: &str,
) -> Result<Vec<u16>> {
    let vm = thread.vm()?;
    let native_memory = vm.native_memory();
    let mut path_chars = Vec::new();
    let mut offset = 0i64;
    loop {
        let word = native_memory
            .read_i16(address + offset)
            .ok_or_else(|| InternalError(format!("{context}: bad address")))?;
        path_chars.push(word.cast_unsigned());
        if word == 0 {
            return Ok(path_chars);
        }
        offset = offset
            .checked_add(2)
            .ok_or_else(|| InternalError(format!("{context}: address overflow")))?;
    }
}

fn wide_string(value: &[u16]) -> String {
    let length = value
        .iter()
        .position(|word| *word == 0)
        .unwrap_or(value.len());
    String::from_utf16_lossy(value.get(..length).unwrap_or(value))
}

#[expect(unsafe_code)]
fn write_native_structure<V: VM + ?Sized, S>(
    vm: &V,
    address: i64,
    structure: &S,
    context: &str,
) -> Result<()> {
    let bytes = unsafe {
        std::slice::from_raw_parts(std::ptr::from_ref(structure).cast::<u8>(), size_of::<S>())
    };
    if !vm.native_memory().try_write_bytes(address, bytes) {
        return Err(InternalError(format!("{context}: invalid output address")));
    }
    Ok(())
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CloseHandle(J)V", Any)]
#[async_method]
pub async fn close_handle<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    if !managed_files::close(vm.file_handles(), handle).await && !iocp::close_port(&*vm, handle)? {
        #[expect(unsafe_code)]
        unsafe {
            use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
            let _ = CloseHandle(handle as usize as HANDLE);
        }
    }
    iocp::mark_closed(&*vm, handle);
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateFile0(JIIJII)J", Any)]
#[async_method]
pub async fn create_file_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags_and_attrs = parameters.pop_int()?;
    let creation_disposition = parameters.pop_int()?;
    let security_descriptor = parameters.pop_long()?;
    let share_mode = parameters.pop_int()?;
    let desired_access = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "CreateFile0")?;
    let access = DesiredAccess::from_bits_truncate(desired_access.cast_unsigned());
    let writable = access.intersects(
        DesiredAccess::GENERIC_WRITE
            | DesiredAccess::FILE_WRITE_DATA
            | DesiredAccess::FILE_APPEND_DATA
            | DesiredAccess::FILE_WRITE_EA
            | DesiredAccess::FILE_WRITE_ATTRIBUTES,
    );
    let vm = thread.vm()?;
    let (raw_handle, error) = {
        let mut descriptor = host_security_descriptor(&*vm, security_descriptor)?;
        let security_attributes = descriptor.as_mut().map(|descriptor| {
            windows_sys::Win32::Security::SECURITY_ATTRIBUTES {
                nLength: u32::try_from(
                    size_of::<windows_sys::Win32::Security::SECURITY_ATTRIBUTES>(),
                )
                .unwrap_or(0),
                lpSecurityDescriptor: descriptor.pointer(),
                bInheritHandle: 0,
            }
        });
        let security_attributes_pointer = security_attributes
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref);
        #[expect(unsafe_code)]
        let raw_handle = unsafe {
            windows_sys::Win32::Storage::FileSystem::CreateFileW(
                path.as_ptr(),
                desired_access.cast_unsigned(),
                share_mode.cast_unsigned(),
                security_attributes_pointer,
                creation_disposition.cast_unsigned(),
                flags_and_attrs.cast_unsigned(),
                std::ptr::null_mut(),
            )
        };
        let error = if raw_handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            last_windows_error()
        } else {
            0
        };
        (raw_handle as usize, error)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let handle = managed_files::adopt_raw_handle(vm.file_handles(), raw_handle, writable)
        .await
        .map_err(|error| InternalError(format!("CreateFile0: {error}")))?;
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.FindClose(J)V", Any)]
#[async_method]
pub async fn find_close<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::FindClose(
            handle as usize as windows_sys::Win32::Foundation::HANDLE,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile0(JLsun/nio/fs/WindowsNativeDispatcher$FirstFile;)V",
    Any
)]
#[async_method]
pub async fn find_first_file_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let first_file_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("FindFirstFile0: null FirstFile".to_string()))?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "FindFirstFile0")?;
    let mut data = windows_sys::Win32::Storage::FileSystem::WIN32_FIND_DATAW::default();
    let (handle, error) = {
        #[expect(unsafe_code)]
        let handle = unsafe {
            windows_sys::Win32::Storage::FileSystem::FindFirstFileW(path.as_ptr(), &raw mut data)
        };
        let error = if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            last_windows_error()
        } else {
            0
        };
        (handle as usize, error)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let name_value = thread.intern_string(&wide_string(&data.cFileName)).await?;

    let mut guard = first_file_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError("FindFirstFile0: not an object".to_string()));
    };
    obj.set_value("handle", Value::Long(handle as i64))?;
    obj.set_value("name", name_value)?;
    obj.set_value(
        "attributes",
        Value::Int(data.dwFileAttributes.cast_signed()),
    )?;
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributesEx0(JJ)V", Any)]
#[async_method]
pub async fn get_file_attributes_ex_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "GetFileAttributesEx0")?;
    let mut attributes =
        windows_sys::Win32::Storage::FileSystem::WIN32_FILE_ATTRIBUTE_DATA::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetFileAttributesExW(
            path.as_ptr(),
            windows_sys::Win32::Storage::FileSystem::GetFileExInfoStandard,
            (&raw mut attributes).cast(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    write_native_structure(&*thread.vm()?, address, &attributes, "GetFileAttributesEx0")?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFullPathName0(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_full_path_name_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "GetFullPathName0")?;
    let mut buffer = vec![0u16; 260];
    loop {
        #[expect(unsafe_code)]
        let length = unsafe {
            windows_sys::Win32::Storage::FileSystem::GetFullPathNameW(
                path.as_ptr(),
                u32::try_from(buffer.len()).unwrap_or(u32::MAX),
                buffer.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if length == 0 {
            return Err(throw_windows_exception(&thread, last_windows_error()).await);
        }
        let length = usize::try_from(length).unwrap_or(usize::MAX);
        if length < buffer.len() {
            buffer.truncate(length);
            break;
        }
        buffer.resize(length.saturating_add(1), 0);
    }
    let full_path = String::from_utf16(&buffer)
        .map_err(|error| InternalError(format!("GetFullPathName0: {error}")))?;
    let string_value = thread.intern_string(&full_path).await?;
    Ok(Some(string_value))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z", Any)]
#[async_method]
pub async fn access_check<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let generic_all = parameters.pop_int()?;
    let generic_execute = parameters.pop_int()?;
    let generic_write = parameters.pop_int()?;
    let generic_read = parameters.pop_int()?;
    let access_mask = parameters.pop_int()?;
    let security_info = parameters.pop_long()?;
    let token = parameters.pop_long()?;
    let vm = thread.vm()?;
    let mut descriptor = read_guest_allocation(&*vm, security_info)?;
    let mapping = windows_sys::Win32::Security::GENERIC_MAPPING {
        GenericRead: generic_read.cast_unsigned(),
        GenericWrite: generic_write.cast_unsigned(),
        GenericExecute: generic_execute.cast_unsigned(),
        GenericAll: generic_all.cast_unsigned(),
    };
    let mut desired = access_mask.cast_unsigned();
    let mut privilege_buffer = vec![0u8; 1024];
    let mut privilege_length = u32::try_from(privilege_buffer.len()).unwrap_or(0);
    let mut granted = 0u32;
    let mut access_status = 0i32;
    #[expect(unsafe_code)]
    unsafe {
        windows_sys::Win32::Security::MapGenericMask(&raw mut desired, &raw const mapping);
    }
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::AccessCheck(
            descriptor.as_mut_ptr().cast(),
            token as usize as windows_sys::Win32::Foundation::HANDLE,
            desired,
            &raw const mapping,
            privilege_buffer.as_mut_ptr().cast(),
            &raw mut privilege_length,
            &raw mut granted,
            &raw mut access_status,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(Some(Value::Int(i32::from(access_status != 0))))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_allowed_ace_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sid_address = parameters.pop_long()?;
    let mask = parameters.pop_int()?;
    let flags = parameters.pop_int()?;
    let acl_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let sid = read_guest_sid(&*vm, sid_address)?;
    let mut acl = read_guest_acl(&*vm, acl_address)?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::AddAccessAllowedAceEx(
            acl.as_mut_ptr().cast(),
            windows_sys::Win32::Security::ACL_REVISION,
            flags.cast_unsigned(),
            mask.cast_unsigned(),
            sid.as_ptr().cast_mut().cast(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    if !vm.native_memory().try_write_bytes(acl_address, &acl) {
        return Err(InternalError(
            "AddAccessAllowedAceEx: invalid ACL address".to_string(),
        ));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_denied_ace_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sid_address = parameters.pop_long()?;
    let mask = parameters.pop_int()?;
    let flags = parameters.pop_int()?;
    let acl_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let sid = read_guest_sid(&*vm, sid_address)?;
    let mut acl = read_guest_acl(&*vm, acl_address)?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::AddAccessDeniedAceEx(
            acl.as_mut_ptr().cast(),
            windows_sys::Win32::Security::ACL_REVISION,
            flags.cast_unsigned(),
            mask.cast_unsigned(),
            sid.as_ptr().cast_mut().cast(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    if !vm.native_memory().try_write_bytes(acl_address, &acl) {
        return Err(InternalError(
            "AddAccessDeniedAceEx: invalid ACL address".to_string(),
        ));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V", Any)]
#[async_method]
pub async fn adjust_token_privileges<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop_int()?;
    let luid_address = parameters.pop_long()?;
    let token = parameters.pop_long()?;
    let vm = thread.vm()?;
    let luid = vm
        .native_memory()
        .read_i64(luid_address)
        .ok_or_else(|| InternalError("AdjustTokenPrivileges: invalid LUID address".to_string()))?;
    use windows_sys::Win32::Foundation::{HANDLE, LUID};
    use windows_sys::Win32::Security::{
        AdjustTokenPrivileges, LUID_AND_ATTRIBUTES, TOKEN_PRIVILEGES,
    };
    let privileges = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: LUID {
                LowPart: luid.cast_unsigned() as u32,
                HighPart: (luid >> 32) as i32,
            },
            Attributes: attributes.cast_unsigned(),
        }],
    };
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Foundation::SetLastError(0);
        AdjustTokenPrivileges(
            token as usize as HANDLE,
            0,
            &raw const privileges,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    let error = last_windows_error();
    if ok == 0 || error == 1300
    /* ERROR_NOT_ALL_ASSIGNED */
    {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn backup_read0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or_else(|| InternalError("BackupRead0: null result".to_string()))?;
    let context_handle = parameters.pop_long()?;
    let abort = parameters.pop_bool()?;
    let bytes_to_read = parameters.pop_int()?;
    let buffer_address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let file = vm
        .file_handles()
        .get(&handle)
        .await
        .ok_or_else(|| InternalError("BackupRead0: invalid file handle".to_string()))?;
    use std::os::windows::io::AsRawHandle;
    let raw_handle = file.file.as_raw_handle() as usize;
    drop(file);
    let contexts = backup_contexts(&*vm)?;
    let context = if context_handle == 0 {
        0
    } else {
        contexts
            .pointers
            .lock()
            .map_err(|_| InternalError("poisoned backup context map".to_string()))?
            .get(&context_handle)
            .copied()
            .ok_or_else(|| InternalError("BackupRead0: invalid backup context".to_string()))?
    };
    let length = usize::try_from(bytes_to_read.max(0)).unwrap_or(0);
    let mut buffer = vec![0u8; length];
    let (error, transferred, context) = {
        let mut context = context as *mut std::ffi::c_void;
        let mut transferred = 0u32;
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Storage::FileSystem::BackupRead(
                raw_handle as windows_sys::Win32::Foundation::HANDLE,
                buffer.as_mut_ptr(),
                u32::try_from(length).unwrap_or(u32::MAX),
                &raw mut transferred,
                abort.into(),
                0,
                &raw mut context,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        (error, transferred, context as usize)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let transferred_length = usize::try_from(transferred).unwrap_or(0).min(buffer.len());
    let transferred_bytes = buffer.get(..transferred_length).ok_or_else(|| {
        InternalError("BackupRead0: transferred length exceeds buffer".to_string())
    })?;
    if transferred_length != 0
        && !vm
            .native_memory()
            .try_write_bytes(buffer_address, transferred_bytes)
    {
        return Err(InternalError(
            "BackupRead0: invalid buffer address".to_string(),
        ));
    }
    let result_context = if abort || context == 0 {
        if context_handle != 0 {
            contexts
                .pointers
                .lock()
                .map_err(|_| InternalError("poisoned backup context map".to_string()))?
                .remove(&context_handle);
        }
        0
    } else if context_handle != 0 {
        contexts
            .pointers
            .lock()
            .map_err(|_| InternalError("poisoned backup context map".to_string()))?
            .insert(context_handle, context);
        context_handle
    } else {
        let new_handle = contexts.next.fetch_add(1, Ordering::Relaxed);
        contexts
            .pointers
            .lock()
            .map_err(|_| InternalError("poisoned backup context map".to_string()))?
            .insert(new_handle, context);
        new_handle
    };
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "BackupRead0: result is not an object".to_string(),
        ));
    };
    obj.set_value("bytesTransferred", Value::Int(transferred.cast_signed()))?;
    obj.set_value("context", Value::Long(result_context))?;
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V", Equal(JAVA_8))]
#[async_method]
pub async fn backup_seek<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let context_handle = parameters.pop_long()?;
    let bytes_to_skip = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let file = vm
        .file_handles()
        .get(&handle)
        .await
        .ok_or_else(|| InternalError("BackupSeek: invalid file handle".to_string()))?;
    use std::os::windows::io::AsRawHandle;
    let raw_handle = file.file.as_raw_handle() as usize;
    drop(file);
    let contexts = backup_contexts(&*vm)?;
    let pointer = contexts
        .pointers
        .lock()
        .map_err(|_| InternalError("poisoned backup context map".to_string()))?
        .get(&context_handle)
        .copied()
        .ok_or_else(|| InternalError("BackupSeek: invalid backup context".to_string()))?;
    let bytes = u64::from_ne_bytes(bytes_to_skip.to_ne_bytes());
    let (error, context) = {
        let mut context = pointer as *mut std::ffi::c_void;
        let mut low_seeked = 0u32;
        let mut high_seeked = 0u32;
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Storage::FileSystem::BackupSeek(
                raw_handle as windows_sys::Win32::Foundation::HANDLE,
                bytes as u32,
                (bytes >> 32) as u32,
                &raw mut low_seeked,
                &raw mut high_seeked,
                &raw mut context,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        (error, context as usize)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    contexts
        .pointers
        .lock()
        .map_err(|_| InternalError("poisoned backup context map".to_string()))?
        .insert(context_handle, context);
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V", Any)]
#[async_method]
pub async fn cancel_io<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let h_file = parameters.pop_long()?;
    let vm = thread.vm()?;
    let file = vm.file_handles().get(&h_file).await;
    let Some(file) = file else {
        return Err(throw_windows_exception(&thread, 6).await);
    };
    use std::os::windows::io::AsRawHandle;
    let raw_handle = file.file.as_raw_handle() as usize;
    drop(file);
    #[expect(unsafe_code)]
    let ok = unsafe {
        use windows_sys::Win32::Foundation::HANDLE;
        windows_sys::Win32::System::IO::CancelIoEx(raw_handle as HANDLE, std::ptr::null())
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn convert_sid_to_string_sid<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let sid = read_guest_sid(&*vm, address)?;
    let value = thread.intern_string(&sid_string(&sid)?).await?;
    Ok(Some(value))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J", Any)]
#[async_method]
pub async fn convert_string_sid_to_sid0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let sid_string = read_native_wide(&thread, address, "ConvertStringSidToSid0")?;
    let (error, sid) = {
        let mut native_sid = std::ptr::null_mut();
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Security::Authorization::ConvertStringSidToSidW(
                sid_string.as_ptr(),
                &raw mut native_sid,
            )
        };
        if ok == 0 {
            (last_windows_error(), Vec::new())
        } else {
            #[expect(unsafe_code)]
            let length = unsafe { windows_sys::Win32::Security::GetLengthSid(native_sid) };
            #[expect(unsafe_code)]
            let sid = unsafe {
                std::slice::from_raw_parts(native_sid.cast::<u8>(), length as usize).to_vec()
            };
            #[expect(unsafe_code)]
            unsafe {
                windows_sys::Win32::Foundation::LocalFree(native_sid.cast());
            }
            (0, sid)
        }
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let vm = thread.vm()?;
    let guest_address = vm.native_memory().allocate(sid.len());
    if !vm.native_memory().try_write_bytes(guest_address, &sid) {
        return Err(InternalError(
            "ConvertStringSidToSid0: allocation failed".to_string(),
        ));
    }
    Ok(Some(Value::Long(guest_address)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V", Any)]
#[async_method]
pub async fn copy_file_ex0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let cancel_address = parameters.pop_long()?;
    let flags = parameters.pop_int()?;
    let new_address = parameters.pop_long()?;
    let existing_address = parameters.pop_long()?;
    let existing = read_native_wide(&thread, existing_address, "CopyFileEx0")?;
    let new = read_native_wide(&thread, new_address, "CopyFileEx0")?;
    let vm = thread.vm()?;
    if cancel_address != 0 && vm.native_memory().read_i32(cancel_address).is_none() {
        return Err(throw_windows_exception(&thread, 998 /* ERROR_NOACCESS */).await);
    }
    let error = {
        let context = CopyCancelContext {
            memory: vm.native_memory(),
            address: cancel_address,
        };
        let (callback, context_pointer): (
            windows_sys::Win32::Storage::FileSystem::LPPROGRESS_ROUTINE,
            *const std::ffi::c_void,
        ) = if cancel_address == 0 {
            (None, std::ptr::null())
        } else {
            (Some(copy_progress), std::ptr::from_ref(&context).cast())
        };
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Storage::FileSystem::CopyFileExW(
                existing.as_ptr(),
                new.as_ptr(),
                callback,
                context_pointer,
                std::ptr::null_mut(),
                flags.cast_unsigned(),
            )
        };
        if ok == 0 { last_windows_error() } else { 0 }
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V", Any)]
#[async_method]
pub async fn create_directory0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sd_address = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let path = read_native_wide(&thread, address, "CreateDirectory0")?;
    let vm = thread.vm()?;
    let error = {
        let mut descriptor = host_security_descriptor(&*vm, sd_address)?;
        let security_attributes = descriptor.as_mut().map(|descriptor| {
            windows_sys::Win32::Security::SECURITY_ATTRIBUTES {
                nLength: u32::try_from(
                    size_of::<windows_sys::Win32::Security::SECURITY_ATTRIBUTES>(),
                )
                .unwrap_or(0),
                lpSecurityDescriptor: descriptor.pointer(),
                bInheritHandle: 0,
            }
        });
        let security_attributes_pointer = security_attributes
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref);
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Storage::FileSystem::CreateDirectoryW(
                path.as_ptr(),
                security_attributes_pointer,
            )
        };
        if ok == 0 { last_windows_error() } else { 0 }
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J", Any)]
#[async_method]
pub async fn create_event<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let initial_state = parameters.pop_bool()?;
    let manual_reset = parameters.pop_bool()?;
    let (handle, error) = {
        #[expect(unsafe_code)]
        let handle = unsafe {
            windows_sys::Win32::System::Threading::CreateEventW(
                std::ptr::null(),
                manual_reset.into(),
                initial_state.into(),
                std::ptr::null(),
            )
        };
        let error = if handle.is_null() {
            last_windows_error()
        } else {
            0
        };
        (handle as usize, error)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(Some(Value::Long(handle as i64)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V", Any)]
#[async_method]
pub async fn create_hard_link0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let existing_file_address = parameters.pop_long()?;
    let new_file_address = parameters.pop_long()?;
    let existing = read_native_wide(&thread, existing_file_address, "CreateHardLink0")?;
    let new = read_native_wide(&thread, new_file_address, "CreateHardLink0")?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::CreateHardLinkW(
            new.as_ptr(),
            existing.as_ptr(),
            std::ptr::null(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let completion_key = parameters.pop_long()?;
    let existing_port = parameters.pop_long()?;
    let file_handle = parameters.pop_long()?;
    let completion_key = i32::try_from(completion_key)
        .map_err(|_| InternalError("completion key is outside the Win32 range".to_string()))?;
    let vm = thread.vm()?;
    if file_handle != -1 && vm.file_handles().get(&file_handle).await.is_none() {
        return Err(throw_windows_exception(&thread, 6).await);
    }
    let port = iocp::create_or_associate(&*vm, file_handle, existing_port, completion_key)?;
    Ok(Some(Value::Long(port)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V", Any)]
#[async_method]
pub async fn create_symbolic_link0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let target_address = parameters.pop_long()?;
    let link_address = parameters.pop_long()?;
    let link = read_native_wide(&thread, link_address, "CreateSymbolicLink0")?;
    let target = read_native_wide(&thread, target_address, "CreateSymbolicLink0")?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::CreateSymbolicLinkW(
            link.as_ptr(),
            target.as_ptr(),
            flags.cast_unsigned(),
        )
    };
    if !ok {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V", Any)]
#[async_method]
pub async fn delete_file0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_wide(&thread, address, "DeleteFile0")?;
    let path_string = wide_string(&path);

    // Match Windows: a file with an active memory mapping cannot be deleted (sharing
    // violation). Compare via the canonicalized path stored when the mapping was created.
    let vm = thread.vm()?;
    if let Ok(regions) = vm
        .resource_manager()
        .get_or_init(crate::java::nio::mapped_regions::MappedRegions::new)
        && let Ok(canonical) = std::fs::canonicalize(&path_string)
        && let Some(canonical_str) = canonical.to_str()
        && regions.is_path_mapped(canonical_str)
    {
        const ERROR_SHARING_VIOLATION: i32 = 32;
        return Err(throw_windows_exception(&thread, ERROR_SHARING_VIOLATION).await);
    }

    #[expect(unsafe_code)]
    let ok = unsafe { windows_sys::Win32::Storage::FileSystem::DeleteFileW(path.as_ptr()) };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
    Any
)]
#[async_method]
pub async fn device_io_control_get_reparse_point<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let buffer_size = parameters.pop_int()?;
    let buffer_address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    match read_reparse_point(vm.file_handles(), handle, buffer_size).await {
        Ok(bytes) => {
            if !vm.native_memory().try_write_bytes(buffer_address, &bytes) {
                return Err(throw_windows_exception(&thread, 998 /* ERROR_NOACCESS */).await);
            }
            Ok(None)
        }
        Err(code) => Err(throw_windows_exception(&thread, code).await),
    }
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
async fn read_reparse_point(
    file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    fd: i64,
    buffer_size: i32,
) -> std::result::Result<Vec<u8>, i32> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::IO::DeviceIoControl;
    const FSCTL_GET_REPARSE_POINT: u32 = 0x0009_00A8;
    let file_handle = file_handles.get(&fd).await.ok_or(6_i32)?;
    let raw_handle = file_handle.file.as_raw_handle() as HANDLE;
    let size = usize::try_from(buffer_size.max(0)).unwrap_or(0);
    if size == 0 {
        return Err(122 /* ERROR_INSUFFICIENT_BUFFER */);
    }
    let mut buf = vec![0u8; size];
    let mut bytes_returned: u32 = 0;
    let ok = unsafe {
        DeviceIoControl(
            raw_handle,
            FSCTL_GET_REPARSE_POINT,
            std::ptr::null(),
            0,
            buf.as_mut_ptr().cast(),
            u32::try_from(size).unwrap_or(0),
            &raw mut bytes_returned,
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        let err = std::io::Error::last_os_error().raw_os_error().unwrap_or(6);
        return Err(err);
    }
    buf.truncate(usize::try_from(bytes_returned).unwrap_or(0));
    Ok(buf)
}

#[cfg(not(target_family = "windows"))]
async fn read_reparse_point(
    _file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    _fd: i64,
    _buffer_size: i32,
) -> std::result::Result<Vec<u8>, i32> {
    Err(4390 /* ERROR_NOT_A_REPARSE_POINT */)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V", Any)]
#[async_method]
pub async fn device_io_control_set_sparse<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let file = vm.file_handles().get(&handle).await.ok_or_else(|| {
        InternalError("DeviceIoControlSetSparse: invalid file handle".to_string())
    })?;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::IO::DeviceIoControl;
    let raw_handle = file.file.as_raw_handle() as usize;
    drop(file);
    let mut returned = 0u32;
    const FSCTL_SET_SPARSE: u32 = 0x0009_00C4;
    #[expect(unsafe_code)]
    let ok = unsafe {
        DeviceIoControl(
            raw_handle as HANDLE,
            FSCTL_SET_SPARSE,
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            0,
            &raw mut returned,
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J", Any)]
#[async_method]
pub async fn duplicate_token_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let desired_access = parameters.pop_int()?;
    let token = parameters.pop_long()?;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Security::{
        DuplicateTokenEx, SecurityImpersonation, TokenImpersonation,
    };
    let (result, error) = {
        let mut result: HANDLE = std::ptr::null_mut();
        #[expect(unsafe_code)]
        let ok = unsafe {
            DuplicateTokenEx(
                token as usize as HANDLE,
                desired_access.cast_unsigned(),
                std::ptr::null(),
                SecurityImpersonation,
                TokenImpersonation,
                &raw mut result,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        (result as usize, error)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(Some(Value::Long(result as i64)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J", Any)]
#[async_method]
pub async fn find_first_file1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data_address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "FindFirstFile1")?;
    let mut data = windows_sys::Win32::Storage::FileSystem::WIN32_FIND_DATAW::default();
    #[expect(unsafe_code)]
    let handle = unsafe {
        windows_sys::Win32::Storage::FileSystem::FindFirstFileW(path.as_ptr(), &raw mut data)
    };
    if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    if let Err(error) =
        write_native_structure(&*thread.vm()?, data_address, &data, "FindFirstFile1")
    {
        #[expect(unsafe_code)]
        unsafe {
            windows_sys::Win32::Storage::FileSystem::FindClose(handle);
        }
        return Err(error);
    }
    Ok(Some(Value::Long(handle as usize as i64)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V",
    Any
)]
#[async_method]
pub async fn find_first_stream0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or_else(|| InternalError("FindFirstStream0: null FirstStream".to_string()))?;
    let address = parameters.pop_long()?;
    let path = read_native_wide(&thread, address, "FindFirstStream0")?;
    let mut data = windows_sys::Win32::Storage::FileSystem::WIN32_FIND_STREAM_DATA::default();
    let (handle, error) = {
        #[expect(unsafe_code)]
        let handle = unsafe {
            windows_sys::Win32::Storage::FileSystem::FindFirstStreamW(
                path.as_ptr(),
                windows_sys::Win32::Storage::FileSystem::FindStreamInfoStandard,
                (&raw mut data).cast(),
                0,
            )
        };
        let error = if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            last_windows_error()
        } else {
            0
        };
        (handle as usize, error)
    };
    if error != 0 && error != 38
    /* ERROR_HANDLE_EOF */
    {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let name = if error != 0 {
        Value::Object(None)
    } else {
        thread
            .intern_string(&wide_string(&data.cStreamName))
            .await?
    };
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut object) = *guard else {
        return Err(InternalError("FindFirstStream0: not an object".to_string()));
    };
    object.set_value("handle", Value::Long(handle as i64))?;
    object.set_value("name", name)?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_file<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    find_next_file_impl(&thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_file0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    find_next_file_impl(&thread, parameters).await
}

async fn find_next_file_impl<T: Thread + 'static>(
    thread: &Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data_address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let mut data = windows_sys::Win32::Storage::FileSystem::WIN32_FIND_DATAW::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::FindNextFileW(
            handle as usize as windows_sys::Win32::Foundation::HANDLE,
            &raw mut data,
        )
    };
    if ok == 0 {
        let error = last_windows_error();
        if error == 18
        /* ERROR_NO_MORE_FILES */
        {
            return Ok(Some(Value::Object(None)));
        }
        return Err(throw_windows_exception(thread, error).await);
    }
    write_native_structure(&*thread.vm()?, data_address, &data, "FindNextFile")?;
    Ok(Some(
        thread.intern_string(&wide_string(&data.cFileName)).await?,
    ))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_stream<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    find_next_stream_impl(&thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_stream0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    find_next_stream_impl(&thread, parameters).await
}

async fn find_next_stream_impl<T: Thread + 'static>(
    thread: &Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let mut data = windows_sys::Win32::Storage::FileSystem::WIN32_FIND_STREAM_DATA::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::FindNextStreamW(
            handle as usize as windows_sys::Win32::Foundation::HANDLE,
            (&raw mut data).cast(),
        )
    };
    if ok == 0 {
        let error = last_windows_error();
        if error == 38 /* ERROR_HANDLE_EOF */ || error == 18
        /* ERROR_NO_MORE_FILES */
        {
            return Ok(Some(Value::Object(None)));
        }
        return Err(throw_windows_exception(thread, error).await);
    }
    Ok(Some(
        thread
            .intern_string(&wide_string(&data.cStreamName))
            .await?,
    ))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_message<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let error_code = parameters.pop_int()?;
    let message = format_win32_message(error_code)
        .unwrap_or_else(|| format!("Windows error code {error_code}"));
    let value = thread.intern_string(&message).await?;
    Ok(Some(value))
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn format_win32_message(code: i32) -> Option<String> {
    use windows_sys::Win32::System::Diagnostics::Debug::{
        FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
    };
    let mut buf = [0u16; 1024];
    let len = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(),
            u32::from_ne_bytes(code.to_ne_bytes()),
            0,
            buf.as_mut_ptr(),
            u32::try_from(buf.len()).unwrap_or(0),
            std::ptr::null_mut(),
        )
    };
    if len == 0 {
        return None;
    }
    let message = buf.get(..usize::try_from(len).unwrap_or(0))?;
    let s = String::from_utf16_lossy(message);
    Some(s.trim_end_matches(['\r', '\n']).to_string())
}

#[cfg(not(target_family = "windows"))]
fn format_win32_message(_code: i32) -> Option<String> {
    None
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J", Any)]
#[async_method]
pub async fn get_ace<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ace_index = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let mut acl = read_guest_acl(&*vm, address)?;
    let (error, offset) = {
        let mut ace = std::ptr::null_mut();
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::Security::GetAce(
                acl.as_mut_ptr().cast(),
                ace_index.cast_unsigned(),
                &raw mut ace,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        let offset = (ace as usize).checked_sub(acl.as_ptr() as usize);
        (error, offset)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let offset = offset.ok_or_else(|| InternalError("GetAce: invalid ACE pointer".to_string()))?;
    let guest_address = address
        .checked_add(
            i64::try_from(offset)
                .map_err(|_| InternalError("GetAce: ACE offset overflow".to_string()))?,
        )
        .ok_or_else(|| InternalError("GetAce: address overflow".to_string()))?;
    Ok(Some(Value::Long(guest_address)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V",
    Any
)]
#[async_method]
pub async fn get_acl_information0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetAclInformation0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let acl = read_guest_acl(&*vm, address)?;
    let mut information = windows_sys::Win32::Security::ACL_SIZE_INFORMATION::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::GetAclInformation(
            acl.as_ptr().cast(),
            (&raw mut information).cast(),
            u32::try_from(size_of_val(&information)).unwrap_or(0),
            windows_sys::Win32::Security::AclSizeInformation,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetAclInformation0: not an object".to_string(),
        ));
    };
    obj.set_value(
        "aceCount",
        Value::Int(i32::try_from(information.AceCount).unwrap_or(i32::MAX)),
    )?;
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J", Any)]
#[async_method]
pub async fn get_current_process<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Win32 returns a pseudo-handle (-1) for the current process.
    #[expect(unsafe_code)]
    let handle = unsafe { windows_sys::Win32::System::Threading::GetCurrentProcess() };
    Ok(Some(Value::Long(handle as usize as i64)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J", Any)]
#[async_method]
pub async fn get_current_thread<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Win32 returns a pseudo-handle (-2) for the current thread.
    #[expect(unsafe_code)]
    let handle = unsafe { windows_sys::Win32::System::Threading::GetCurrentThread() };
    Ok(Some(Value::Long(handle as usize as i64)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_disk_free_space0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetDiskFreeSpace0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetDiskFreeSpace0")?;
    let info = match disk_free_space(&path) {
        Ok(info) => info,
        Err(error) => return Err(throw_windows_exception(&thread, error).await),
    };
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetDiskFreeSpace0: not an object".to_string(),
        ));
    };
    obj.set_value(
        "bytesPerSector",
        Value::Long(i64::from(info.bytes_per_sector)),
    )?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    Any
)]
#[async_method]
pub async fn get_disk_free_space_ex0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetDiskFreeSpaceEx0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetDiskFreeSpaceEx0")?;
    let info = match disk_free_space_ex(&path) {
        Ok(info) => info,
        Err(error) => return Err(throw_windows_exception(&thread, error).await),
    };
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetDiskFreeSpaceEx0: not an object".to_string(),
        ));
    };
    obj.set_value(
        "freeBytesAvailable",
        Value::Long(i64::try_from(info.free_to_caller).unwrap_or(i64::MAX)),
    )?;
    obj.set_value(
        "totalNumberOfBytes",
        Value::Long(i64::try_from(info.total).unwrap_or(i64::MAX)),
    )?;
    obj.set_value(
        "totalNumberOfFreeBytes",
        Value::Long(i64::try_from(info.total_free).unwrap_or(i64::MAX)),
    )?;
    Ok(None)
}

struct DiskFreeSpaceInfo {
    bytes_per_sector: u32,
}

struct DiskFreeSpaceExInfo {
    free_to_caller: u64,
    total: u64,
    total_free: u64,
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn disk_free_space(path: &str) -> std::result::Result<DiskFreeSpaceInfo, i32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceW;
    let wide: Vec<u16> = std::ffi::OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut sectors_per_cluster: u32 = 0;
    let mut bytes_per_sector: u32 = 0;
    let mut free_clusters: u32 = 0;
    let mut total_clusters: u32 = 0;
    let ok = unsafe {
        GetDiskFreeSpaceW(
            wide.as_ptr(),
            &raw mut sectors_per_cluster,
            &raw mut bytes_per_sector,
            &raw mut free_clusters,
            &raw mut total_clusters,
        )
    };
    if ok == 0 {
        return Err(last_windows_error());
    }
    Ok(DiskFreeSpaceInfo { bytes_per_sector })
}

#[cfg(not(target_family = "windows"))]
fn disk_free_space(_path: &str) -> std::result::Result<DiskFreeSpaceInfo, i32> {
    Err(50 /* ERROR_NOT_SUPPORTED */)
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn disk_free_space_ex(path: &str) -> std::result::Result<DiskFreeSpaceExInfo, i32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
    let wide: Vec<u16> = std::ffi::OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut free_to_caller: u64 = 0;
    let mut total: u64 = 0;
    let mut total_free: u64 = 0;
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &raw mut free_to_caller,
            &raw mut total,
            &raw mut total_free,
        )
    };
    if ok == 0 {
        return Err(last_windows_error());
    }
    Ok(DiskFreeSpaceExInfo {
        free_to_caller,
        total,
        total_free,
    })
}

#[cfg(not(target_family = "windows"))]
fn disk_free_space_ex(_path: &str) -> std::result::Result<DiskFreeSpaceExInfo, i32> {
    Err(50 /* ERROR_NOT_SUPPORTED */)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I", Any)]
#[async_method]
pub async fn get_drive_type0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetDriveType0")?;
    Ok(Some(Value::Int(drive_type(&path))))
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn drive_type(path: &str) -> i32 {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetDriveTypeW;
    let wide: Vec<u16> = std::ffi::OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let t = unsafe { GetDriveTypeW(wide.as_ptr()) };
    i32::try_from(t).unwrap_or(0)
}

#[cfg(not(target_family = "windows"))]
fn drive_type(_path: &str) -> i32 {
    0
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I", Any)]
#[async_method]
pub async fn get_file_attributes0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "GetFileAttributes0")?;
    #[expect(unsafe_code)]
    let attributes =
        unsafe { windows_sys::Win32::Storage::FileSystem::GetFileAttributesW(path.as_ptr()) };
    if attributes == u32::MAX {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(Some(Value::Int(attributes.cast_signed())))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_information_by_handle<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_file_information_by_handle0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_file_information_by_handle0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let file = vm.file_handles().get(&handle).await.ok_or_else(|| {
        InternalError("GetFileInformationByHandle: invalid file handle".to_string())
    })?;
    use std::os::windows::io::AsRawHandle;
    let raw_handle = file.file.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;
    let mut information =
        windows_sys::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetFileInformationByHandle(
            raw_handle,
            &raw mut information,
        )
    };
    drop(file);
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    write_native_structure(&*vm, address, &information, "GetFileInformationByHandle")?;
    Ok(None)
}

/// Returns `(volume_serial_number, number_of_links, file_index)` for the given file handle.
#[expect(unsafe_code)]
pub(crate) async fn file_identity(
    file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    fd: i64,
) -> Option<(u32, u32, u64)> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };
    let file_handle = file_handles.get(&fd).await?;
    let raw_handle = file_handle.file.as_raw_handle() as HANDLE;
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    let ok = unsafe { GetFileInformationByHandle(raw_handle, &raw mut info) };
    if ok == 0 {
        return None;
    }
    let index = (u64::from(info.nFileIndexHigh) << 32) | u64::from(info.nFileIndexLow);
    Some((info.dwVolumeSerialNumber, info.nNumberOfLinks, index))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I", Any)]
#[async_method]
pub async fn get_file_security0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let n_length = parameters.pop_int()?;
    let desc_address = parameters.pop_long()?;
    let requested_information = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "GetFileSecurity0")?;
    let buffer_length = usize::try_from(n_length.max(0)).unwrap_or(0);
    let mut descriptor = vec![0u8; buffer_length];
    let mut required = 0u32;
    let descriptor_pointer = if descriptor.is_empty() {
        std::ptr::null_mut()
    } else {
        descriptor.as_mut_ptr().cast()
    };
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::GetFileSecurityW(
            path.as_ptr(),
            requested_information.cast_unsigned(),
            descriptor_pointer,
            u32::try_from(buffer_length).unwrap_or(u32::MAX),
            &raw mut required,
        )
    };
    if ok == 0 {
        let error = last_windows_error();
        if error != 122
        /* ERROR_INSUFFICIENT_BUFFER */
        {
            return Err(throw_windows_exception(&thread, error).await);
        }
        return Ok(Some(Value::Int(
            i32::try_from(required).unwrap_or(i32::MAX),
        )));
    }
    let bytes_written = usize::try_from(required)
        .unwrap_or(buffer_length)
        .min(buffer_length);
    let written_descriptor = descriptor
        .get(..bytes_written)
        .ok_or_else(|| InternalError("GetFileSecurity0: invalid result length".to_string()))?;
    if bytes_written != 0
        && !thread
            .vm()?
            .native_memory()
            .try_write_bytes(desc_address, written_descriptor)
    {
        return Err(InternalError(
            "GetFileSecurity0: invalid descriptor address".to_string(),
        ));
    }
    Ok(Some(Value::Int(n_length)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_size_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    match managed_files::metadata(vm.file_handles(), handle).await {
        Ok(meta) => Ok(Some(Value::Long(
            i64::try_from(meta.len()).unwrap_or(i64::MAX),
        ))),
        Err(e) => Err(throw_windows_exception(&thread, windows_error_code(&e)).await),
    }
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_final_path_name_by_handle<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    match final_path_name(vm.file_handles(), handle).await {
        Ok(path) => {
            let value = path.to_object(thread.as_ref()).await?;
            Ok(Some(value))
        }
        Err(code) => Err(throw_windows_exception(&thread, code).await),
    }
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
async fn final_path_name(
    file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    fd: i64,
) -> std::result::Result<String, i32> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::GetFinalPathNameByHandleW;
    let file_handle = file_handles.get(&fd).await.ok_or(6_i32)?;
    let raw_handle = file_handle.file.as_raw_handle() as HANDLE;
    let mut buf = vec![0u16; 1024];
    loop {
        let needed = unsafe {
            GetFinalPathNameByHandleW(
                raw_handle,
                buf.as_mut_ptr(),
                u32::try_from(buf.len()).unwrap_or(0),
                0, /* VOLUME_NAME_DOS | FILE_NAME_NORMALIZED */
            )
        };
        if needed == 0 {
            let err = std::io::Error::last_os_error().raw_os_error().unwrap_or(6);
            return Err(err);
        }
        let needed_usize = usize::try_from(needed).unwrap_or(0);
        if needed_usize >= buf.len() {
            buf.resize(needed_usize + 1, 0);
            continue;
        }
        let path = buf.get(..needed_usize).ok_or(87_i32)?;
        return Ok(String::from_utf16_lossy(path));
    }
}

#[cfg(not(target_family = "windows"))]
async fn final_path_name(
    file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    _fd: i64,
) -> std::result::Result<String, i32> {
    let _ = file_handles;
    Err(50 /* ERROR_NOT_SUPPORTED */)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I", Any)]
#[async_method]
pub async fn get_length_sid<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let length = read_guest_sid(&*thread.vm()?, address)?.len();
    Ok(Some(Value::Int(i32::try_from(length).unwrap_or(i32::MAX))))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I", Any)]
#[async_method]
pub async fn get_logical_drives<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[expect(unsafe_code)]
    let mask = unsafe { windows_sys::Win32::Storage::FileSystem::GetLogicalDrives() };
    if mask == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(Some(Value::Int(mask.cast_signed())))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I", Any)]
#[async_method]
pub async fn get_overlapped_result<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let lp_overlapped = parameters.pop_long()?;
    let h_file = parameters.pop_long()?;
    let vm = thread.vm()?;
    let operations = overlapped_operations(&*vm)?;
    let operation = operations
        .0
        .lock()
        .map_err(|_| InternalError("poisoned overlapped operation map".to_string()))?
        .get(&(h_file, lp_overlapped))
        .cloned()
        .ok_or_else(|| InternalError("GetOverlappedResult: unknown operation".to_string()))?;
    let (error, transferred) = loop {
        let notified = operation.notification.notified();
        if let Some(result) = *operation
            .result
            .lock()
            .map_err(|_| InternalError("poisoned overlapped result".to_string()))?
        {
            break result;
        }
        notified.await;
    };
    operations
        .0
        .lock()
        .map_err(|_| InternalError("poisoned overlapped operation map".to_string()))?
        .remove(&(h_file, lp_overlapped));
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(Some(Value::Int(transferred.cast_signed())))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
    Any
)]
#[async_method]
pub async fn get_queued_completion_status0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let status_gc = parameters.pop_reference()?.ok_or(InternalError(
        "GetQueuedCompletionStatus0: null status".to_string(),
    ))?;
    let completion_port = parameters.pop_long()?;
    let vm = thread.vm()?;
    let completion = iocp::receive(&*vm, completion_port).await?;
    let mut guard = status_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetQueuedCompletionStatus0: not an object".to_string(),
        ));
    };
    obj.set_value("error", Value::Int(completion.error))?;
    obj.set_value("bytesTransferred", Value::Int(completion.bytes_transferred))?;
    obj.set_value(
        "completionKey",
        Value::Long(i64::from(completion.completion_key)),
    )?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_dacl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    if let Some(descriptor) = security_descriptors(&*vm)?
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
        .get(&address)
        .cloned()
    {
        return Ok(Some(Value::Long(descriptor.dacl_address)));
    }
    let descriptor = vm
        .native_memory()
        .try_read_bytes(address, 20)
        .ok_or_else(|| {
            InternalError("GetSecurityDescriptorDacl: invalid descriptor".to_string())
        })?;
    let control = read_u16(&descriptor, 2).unwrap_or(0);
    if control & windows_sys::Win32::Security::SE_DACL_PRESENT == 0 {
        return Ok(Some(Value::Long(0)));
    }
    let offset = i64::from(read_u32(&descriptor, 16).unwrap_or(0));
    let dacl = if offset == 0 {
        0
    } else {
        address.saturating_add(offset)
    };
    Ok(Some(Value::Long(dacl)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_owner<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    if let Some(descriptor) = security_descriptors(&*vm)?
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
        .get(&address)
        .cloned()
    {
        return Ok(Some(Value::Long(descriptor.owner_address)));
    }
    let descriptor = vm
        .native_memory()
        .try_read_bytes(address, 20)
        .ok_or_else(|| {
            InternalError("GetSecurityDescriptorOwner: invalid descriptor".to_string())
        })?;
    let offset = i64::from(read_u32(&descriptor, 4).unwrap_or(0));
    let owner = if offset == 0 {
        0
    } else {
        address.saturating_add(offset)
    };
    Ok(Some(Value::Long(owner)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I", Any)]
#[async_method]
pub async fn get_token_information<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let token_info_length = parameters.pop_int()?;
    let token_info = parameters.pop_long()?;
    let token_info_class = parameters.pop_int()?;
    let token = parameters.pop_long()?;
    let length = usize::try_from(token_info_length.max(0)).unwrap_or(0);
    let mut buffer = vec![0u8; length];
    let mut required = 0u32;
    let pointer = if buffer.is_empty() {
        std::ptr::null_mut()
    } else {
        buffer.as_mut_ptr().cast()
    };
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::GetTokenInformation(
            token as usize as windows_sys::Win32::Foundation::HANDLE,
            token_info_class,
            pointer,
            u32::try_from(length).unwrap_or(u32::MAX),
            &raw mut required,
        )
    };
    if ok == 0 {
        let error = last_windows_error();
        if error != 122
        /* ERROR_INSUFFICIENT_BUFFER */
        {
            return Err(throw_windows_exception(&thread, error).await);
        }
        return Ok(Some(Value::Int(
            i32::try_from(required).unwrap_or(i32::MAX),
        )));
    }

    // TOKEN_USER begins with SID_AND_ATTRIBUTES. Convert its embedded host pointer into
    // the equivalent guest address before exposing the buffer to Java Unsafe.
    if token_info_class == windows_sys::Win32::Security::TokenUser && length >= size_of::<usize>() {
        let pointer_bytes = buffer
            .get(..size_of::<usize>())
            .ok_or_else(|| InternalError("GetTokenInformation: invalid TOKEN_USER".to_string()))?;
        let host_pointer =
            usize::from_ne_bytes(pointer_bytes.try_into().map_err(|_| {
                InternalError("GetTokenInformation: invalid TOKEN_USER".to_string())
            })?);
        let host_base = buffer.as_ptr() as usize;
        let offset = host_pointer.checked_sub(host_base).ok_or_else(|| {
            InternalError("GetTokenInformation: SID pointer outside result".to_string())
        })?;
        let guest_pointer = token_info
            .checked_add(i64::try_from(offset).map_err(|_| {
                InternalError("GetTokenInformation: SID offset overflow".to_string())
            })?)
            .ok_or_else(|| InternalError("GetTokenInformation: address overflow".to_string()))?;
        let output_pointer = buffer.get_mut(..size_of::<usize>()).ok_or_else(|| {
            InternalError("GetTokenInformation: truncated TOKEN_USER".to_string())
        })?;
        output_pointer.copy_from_slice(&(guest_pointer as usize).to_ne_bytes());
    }
    let written = usize::try_from(required).unwrap_or(length).min(length);
    let written_buffer = buffer
        .get(..written)
        .ok_or_else(|| InternalError("GetTokenInformation: invalid result length".to_string()))?;
    if written != 0
        && !thread
            .vm()?
            .native_memory()
            .try_write_bytes(token_info, written_buffer)
    {
        return Err(InternalError(
            "GetTokenInformation: invalid output address".to_string(),
        ));
    }
    Ok(Some(Value::Int(token_info_length)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
    Any
)]
#[async_method]
pub async fn get_volume_information0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetVolumeInformation0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetVolumeInformation0")?;
    let info = match volume_information(&path) {
        Ok(info) => info,
        Err(error) => return Err(throw_windows_exception(&thread, error).await),
    };
    let fs_name = info.0.to_object(thread.as_ref()).await?;
    let vol_name = info.1.to_object(thread.as_ref()).await?;
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetVolumeInformation0: not an object".to_string(),
        ));
    };
    obj.set_value("fileSystemName", fs_name)?;
    obj.set_value("volumeName", vol_name)?;
    obj.set_value("volumeSerialNumber", Value::Int(info.2.cast_signed()))?;
    obj.set_value("flags", Value::Int(info.3.cast_signed()))?;
    Ok(None)
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn volume_information(path: &str) -> std::result::Result<(String, String, u32, u32), i32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetVolumeInformationW;
    let wide: Vec<u16> = std::ffi::OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut vol_name = [0u16; 260];
    let mut fs_name = [0u16; 260];
    let mut serial: u32 = 0;
    let mut max_component: u32 = 0;
    let mut flags: u32 = 0;
    let ok = unsafe {
        GetVolumeInformationW(
            wide.as_ptr(),
            vol_name.as_mut_ptr(),
            u32::try_from(vol_name.len()).unwrap_or(0),
            &raw mut serial,
            &raw mut max_component,
            &raw mut flags,
            fs_name.as_mut_ptr(),
            u32::try_from(fs_name.len()).unwrap_or(0),
        )
    };
    if ok == 0 {
        return Err(last_windows_error());
    }
    let vol_len = vol_name
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(vol_name.len());
    let fs_len = fs_name
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(fs_name.len());
    let fs_name = fs_name.get(..fs_len).unwrap_or(&fs_name);
    let vol_name = vol_name.get(..vol_len).unwrap_or(&vol_name);
    Ok((
        String::from_utf16_lossy(fs_name),
        String::from_utf16_lossy(vol_name),
        serial,
        flags,
    ))
}

#[cfg(not(target_family = "windows"))]
fn volume_information(_path: &str) -> std::result::Result<(String, String, u32, u32), i32> {
    Err(50 /* ERROR_NOT_SUPPORTED */)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_volume_path_name0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetVolumePathName0")?;
    let volume = match volume_path_name(&path) {
        Ok(volume) => volume,
        Err(error) => return Err(throw_windows_exception(&thread, error).await),
    };
    let value = volume.to_object(thread.as_ref()).await?;
    Ok(Some(value))
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn volume_path_name(path: &str) -> std::result::Result<String, i32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::GetVolumePathNameW;
    let wide: Vec<u16> = std::ffi::OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut buf = [0u16; 260];
    let ok = unsafe {
        GetVolumePathNameW(
            wide.as_ptr(),
            buf.as_mut_ptr(),
            u32::try_from(buf.len()).unwrap_or(0),
        )
    };
    if ok == 0 {
        return Err(last_windows_error());
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Ok(String::from_utf16_lossy(buf.get(..len).unwrap_or(&buf)))
}

#[cfg(not(target_family = "windows"))]
fn volume_path_name(_path: &str) -> std::result::Result<String, i32> {
    Err(50 /* ERROR_NOT_SUPPORTED */)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V", Any)]
#[async_method]
pub async fn initialize_acl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let size = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let length = usize::try_from(size)
        .map_err(|_| InternalError("InitializeAcl: negative ACL size".to_string()))?;
    let mut acl = vec![0u8; length];
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::InitializeAcl(
            acl.as_mut_ptr().cast(),
            u32::try_from(length).unwrap_or(u32::MAX),
            windows_sys::Win32::Security::ACL_REVISION,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    if !thread.vm()?.native_memory().try_write_bytes(address, &acl) {
        return Err(InternalError(
            "InitializeAcl: invalid ACL address".to_string(),
        ));
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V",
    Any
)]
#[async_method]
pub async fn initialize_security_descriptor<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let descriptor = [1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if !vm.native_memory().try_write_bytes(address, &descriptor) {
        return Err(InternalError(
            "InitializeSecurityDescriptor: invalid descriptor address".to_string(),
        ));
    }
    security_descriptors(&*vm)?
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
        .insert(address, GuestSecurityDescriptor::default());
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V", Any)]
#[async_method]
pub async fn local_free<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    if address != 0 {
        let vm = thread.vm()?;
        vm.native_memory().free(address);
        security_descriptors(&*vm)?
            .0
            .lock()
            .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
            .remove(&address);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I", Any)]
#[async_method]
pub async fn lookup_account_name0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let cb_sid = parameters.pop_int()?;
    let sid_address = parameters.pop_long()?;
    let name_address = parameters.pop_long()?;
    let name = read_native_wide(&thread, name_address, "LookupAccountName0")?;
    let mut required_sid = 0u32;
    let mut required_domain = 0u32;
    let mut sid_use = 0i32;
    #[expect(unsafe_code)]
    unsafe {
        windows_sys::Win32::Security::LookupAccountNameW(
            std::ptr::null(),
            name.as_ptr(),
            std::ptr::null_mut(),
            &raw mut required_sid,
            std::ptr::null_mut(),
            &raw mut required_domain,
            &raw mut sid_use,
        );
    }
    let query_error = last_windows_error();
    if required_sid == 0 && query_error != 122
    /* ERROR_INSUFFICIENT_BUFFER */
    {
        return Err(throw_windows_exception(&thread, query_error).await);
    }
    if cb_sid <= 0 || sid_address == 0 {
        return Ok(Some(Value::Int(required_sid.cast_signed())));
    }
    let mut sid = vec![0u8; usize::try_from(cb_sid).unwrap_or(0)];
    let mut sid_size = u32::try_from(sid.len()).unwrap_or(0);
    let mut domain = vec![0u16; usize::try_from(required_domain).unwrap_or(0)];
    let mut domain_size = required_domain;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::LookupAccountNameW(
            std::ptr::null(),
            name.as_ptr(),
            sid.as_mut_ptr().cast(),
            &raw mut sid_size,
            domain.as_mut_ptr(),
            &raw mut domain_size,
            &raw mut sid_use,
        )
    };
    if ok == 0 {
        let error = last_windows_error();
        if error == 122 {
            return Ok(Some(Value::Int(sid_size.cast_signed())));
        }
        return Err(throw_windows_exception(&thread, error).await);
    }
    sid.truncate(usize::try_from(sid_size).unwrap_or(0));
    if !thread
        .vm()?
        .native_memory()
        .try_write_bytes(sid_address, &sid)
    {
        return Err(throw_windows_exception(&thread, 998).await);
    }
    Ok(Some(Value::Int(sid_size.cast_signed())))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V",
    Any
)]
#[async_method]
pub async fn lookup_account_sid0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("LookupAccountSid0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let mut sid = read_guest_sid(&*vm, address)?;
    let mut name_length = 0u32;
    let mut domain_length = 0u32;
    let mut sid_use = 0i32;
    #[expect(unsafe_code)]
    unsafe {
        windows_sys::Win32::Security::LookupAccountSidW(
            std::ptr::null(),
            sid.as_mut_ptr().cast(),
            std::ptr::null_mut(),
            &raw mut name_length,
            std::ptr::null_mut(),
            &raw mut domain_length,
            &raw mut sid_use,
        );
    }
    let error = last_windows_error();
    if error != 122 || name_length == 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    let mut name = vec![0u16; name_length as usize];
    let mut domain = vec![0u16; domain_length as usize];
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::LookupAccountSidW(
            std::ptr::null(),
            sid.as_mut_ptr().cast(),
            name.as_mut_ptr(),
            &raw mut name_length,
            domain.as_mut_ptr(),
            &raw mut domain_length,
            &raw mut sid_use,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    name.truncate(name_length as usize);
    domain.truncate(domain_length as usize);
    let name = String::from_utf16_lossy(&name)
        .to_object(thread.as_ref())
        .await?;
    let domain = String::from_utf16_lossy(&domain)
        .to_object(thread.as_ref())
        .await?;
    let mut guard = obj_gc.write();
    if let Reference::Object(ref mut obj) = *guard {
        obj.set_value("domain", domain)?;
        obj.set_value("name", name)?;
        obj.set_value("use", Value::Int(sid_use))?;
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J", Any)]
#[async_method]
pub async fn lookup_privilege_value0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name_address = parameters.pop_long()?;
    let name = read_native_wide(&thread, name_address, "LookupPrivilegeValue0")?;
    let mut luid = windows_sys::Win32::Foundation::LUID::default();
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Security::LookupPrivilegeValueW(
            std::ptr::null(),
            name.as_ptr(),
            &raw mut luid,
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    let value = (i64::from(luid.HighPart) << 32) | i64::from(luid.LowPart);
    let vm = thread.vm()?;
    let address = vm.native_memory().allocate(8);
    vm.native_memory().write_i64(address, value);
    Ok(Some(Value::Long(address)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V", Any)]
#[async_method]
pub async fn move_file_ex0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let new_address = parameters.pop_long()?;
    let existing_address = parameters.pop_long()?;
    let existing = read_native_wide(&thread, existing_address, "MoveFileEx0")?;
    let new = read_native_wide(&thread, new_address, "MoveFileEx0")?;
    #[expect(unsafe_code)]
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::MoveFileExW(
            existing.as_ptr(),
            new.as_ptr(),
            flags.cast_unsigned(),
        )
    };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J", Any)]
#[async_method]
pub async fn open_process_token<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let desired_access = parameters.pop_int()?;
    let process = parameters.pop_long()?;
    use windows_sys::Win32::Foundation::HANDLE;
    let (token, error) = {
        let mut token: HANDLE = std::ptr::null_mut();
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::System::Threading::OpenProcessToken(
                process as usize as HANDLE,
                desired_access.cast_unsigned(),
                &raw mut token,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        (token as usize, error)
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(Some(Value::Long(token as i64)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J", Any)]
#[async_method]
pub async fn open_thread_token<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let open_as_self = parameters.pop_bool()?;
    let desired_access = parameters.pop_int()?;
    let thread_handle = parameters.pop_long()?;
    use windows_sys::Win32::Foundation::HANDLE;
    let (token, error) = {
        let mut token: HANDLE = std::ptr::null_mut();
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::System::Threading::OpenThreadToken(
                thread_handle as usize as HANDLE,
                desired_access.cast_unsigned(),
                open_as_self.into(),
                &raw mut token,
            )
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        (token as usize, error)
    };
    if error != 0 {
        if error == 1008
        /* ERROR_NO_TOKEN */
        {
            return Ok(Some(Value::Long(0)));
        }
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(Some(Value::Long(token as i64)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V",
    Any
)]
#[async_method]
pub async fn post_queued_completion_status<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let completion_key = parameters.pop_long()?;
    let completion_port = parameters.pop_long()?;
    let vm = thread.vm()?;
    iocp::post_to_port(
        &*vm,
        completion_port,
        CompletionPacket {
            error: 0,
            bytes_transferred: 0,
            completion_key: i32::try_from(completion_key).unwrap_or(0),
            overlapped: 0,
        },
    )?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V",
    Any
)]
#[async_method]
#[expect(
    clippy::too_many_lines,
    reason = "issuing and completing one overlapped Win32 directory operation is one state machine"
)]
pub async fn read_directory_changes_w<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let p_overlapped = parameters.pop_long()?;
    let bytes_returned_address = parameters.pop_long()?;
    let filter = parameters.pop_int()?;
    let watch_sub_tree = parameters.pop_bool()?;
    let buffer_length = parameters.pop_int()?;
    let buffer_address = parameters.pop_long()?;
    let h_directory = parameters.pop_long()?;
    let size = usize::try_from(buffer_length)
        .map_err(|_| InternalError("ReadDirectoryChangesW: negative buffer length".to_string()))?;
    let vm = thread.vm()?;
    if vm
        .native_memory()
        .try_read_bytes(buffer_address, size)
        .is_none()
        || vm
            .native_memory()
            .try_read_bytes(bytes_returned_address, 4)
            .is_none()
    {
        return Err(throw_windows_exception(&thread, 998 /* ERROR_NOACCESS */).await);
    }
    let file = vm
        .file_handles()
        .get(&h_directory)
        .await
        .ok_or_else(|| InternalError("ReadDirectoryChangesW: invalid handle".to_string()))?;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::{ERROR_IO_PENDING, HANDLE};
    use windows_sys::Win32::System::IO::{GetOverlappedResult, OVERLAPPED};
    let raw_handle = file.file.as_raw_handle() as usize;
    drop(file);
    let target = iocp::begin_operation(&*vm, h_directory)?;
    let guest_operation = match begin_guest_overlapped(&*vm, h_directory, p_overlapped) {
        Ok(operation) => operation,
        Err(error) => {
            iocp::abandon_operation(&*vm, target)?;
            return Err(error);
        }
    };
    let mut buffer = vec![0u8; size];
    let mut overlapped = Box::<OVERLAPPED>::default();
    let mut initial_bytes = 0u32;
    #[expect(unsafe_code)]
    let issued = unsafe {
        windows_sys::Win32::Storage::FileSystem::ReadDirectoryChangesW(
            raw_handle as HANDLE,
            buffer.as_mut_ptr().cast(),
            u32::try_from(size).unwrap_or(u32::MAX),
            watch_sub_tree.into(),
            filter.cast_unsigned(),
            &raw mut initial_bytes,
            &raw mut *overlapped,
            None,
        )
    };
    if issued == 0 {
        let error = last_windows_error();
        if error.cast_unsigned() != ERROR_IO_PENDING {
            iocp::abandon_operation(&*vm, target)?;
            overlapped_operations(&*vm)?
                .0
                .lock()
                .map_err(|_| InternalError("poisoned overlapped operation map".to_string()))?
                .remove(&(h_directory, p_overlapped));
            drop(overlapped);
            return Err(throw_windows_exception(&thread, error).await);
        }
    }

    let overlapped_pointer = Box::into_raw(overlapped) as usize;
    tokio::task::spawn_blocking(move || {
        #[expect(unsafe_code)]
        let overlapped = unsafe { Box::from_raw(overlapped_pointer as *mut OVERLAPPED) };
        let mut transferred = initial_bytes;
        #[expect(unsafe_code)]
        let ok = unsafe {
            GetOverlappedResult(
                raw_handle as HANDLE,
                &raw const *overlapped,
                &raw mut transferred,
                1,
            )
        };
        let mut error = if ok == 0 { last_windows_error() } else { 0 };
        let mut transferred_usize = usize::try_from(transferred).unwrap_or(0).min(buffer.len());
        if error == 0 {
            let transferred_bytes = buffer.get(..transferred_usize);
            let transferred_value = transferred.to_ne_bytes();
            let wrote_result = transferred_bytes
                .is_some_and(|bytes| vm.native_memory().try_write_bytes(buffer_address, bytes));
            let wrote_length = vm
                .native_memory()
                .try_write_bytes(bytes_returned_address, &transferred_value);
            if !wrote_result || !wrote_length {
                error = 998; // ERROR_NOACCESS
                transferred = 0;
                transferred_usize = 0;
            }
        }
        finish_guest_overlapped(&guest_operation, error, transferred);
        let _ = iocp::post_operation(&*vm, target, error, transferred_usize, p_overlapped);
    });
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V", Any)]
#[async_method]
pub async fn remove_directory0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_wide(&thread, address, "RemoveDirectory0")?;
    #[expect(unsafe_code)]
    let ok = unsafe { windows_sys::Win32::Storage::FileSystem::RemoveDirectoryW(path.as_ptr()) };
    if ok == 0 {
        return Err(throw_windows_exception(&thread, last_windows_error()).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V", Any)]
#[async_method]
pub async fn set_end_of_file<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let pos =
        match managed_files::seek(vm.file_handles(), handle, std::io::SeekFrom::Current(0)).await {
            Ok(p) => p,
            Err(e) => return Err(throw_windows_exception(&thread, windows_error_code(&e)).await),
        };
    if let Err(e) = managed_files::set_len(vm.file_handles(), handle, pos).await {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V", Any)]
#[async_method]
pub async fn set_file_attributes0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "SetFileAttributes0")?;
    if let Err(e) = set_windows_file_attributes(std::path::Path::new(&path), value) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V", Any)]
#[async_method]
pub async fn set_file_security0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let desc_address = parameters.pop_long()?;
    let requested_information = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_wide(&thread, path_address, "SetFileSecurity0")?;
    let vm = thread.vm()?;
    let state = security_descriptors(&*vm)?
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?
        .get(&desc_address)
        .cloned();

    let (ok, error) = (|| -> Result<(i32, i32)> {
        let ok = if let Some(mut state) = state {
            let mut descriptor = windows_sys::Win32::Security::SECURITY_DESCRIPTOR::default();
            #[expect(unsafe_code)]
            let initialized = unsafe {
                windows_sys::Win32::Security::InitializeSecurityDescriptor(
                    (&raw mut descriptor).cast(),
                    1,
                )
            };
            if initialized == 0 {
                return Ok((0, last_windows_error()));
            }
            if let Some(owner) = state.owner.as_mut() {
                #[expect(unsafe_code)]
                let owner_ok = unsafe {
                    windows_sys::Win32::Security::SetSecurityDescriptorOwner(
                        (&raw mut descriptor).cast(),
                        owner.as_mut_ptr().cast(),
                        0,
                    )
                };
                if owner_ok == 0 {
                    return Ok((0, last_windows_error()));
                }
            }
            if let Some(dacl) = state.dacl.as_mut() {
                #[expect(unsafe_code)]
                let dacl_ok = unsafe {
                    windows_sys::Win32::Security::SetSecurityDescriptorDacl(
                        (&raw mut descriptor).cast(),
                        1,
                        dacl.as_mut_ptr().cast(),
                        0,
                    )
                };
                if dacl_ok == 0 {
                    return Ok((0, last_windows_error()));
                }
            }
            #[expect(unsafe_code)]
            unsafe {
                windows_sys::Win32::Security::SetFileSecurityW(
                    path.as_ptr(),
                    requested_information.cast_unsigned(),
                    (&raw mut descriptor).cast(),
                )
            }
        } else {
            let mut descriptor = read_guest_allocation(&*vm, desc_address)?;
            #[expect(unsafe_code)]
            unsafe {
                windows_sys::Win32::Security::SetFileSecurityW(
                    path.as_ptr(),
                    requested_information.cast_unsigned(),
                    descriptor.as_mut_ptr().cast(),
                )
            }
        };
        let error = if ok == 0 { last_windows_error() } else { 0 };
        Ok((ok, error))
    })()?;
    if ok == 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_file_time<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let last_write_time = parameters.pop_long()?;
    let last_access_time = parameters.pop_long()?;
    let create_time = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    if let Err(error) = set_file_time_via_handle(
        vm.file_handles(),
        handle,
        create_time,
        last_access_time,
        last_write_time,
    )
    .await
    {
        return Err(throw_windows_exception(&thread, windows_error_code(&error)).await);
    }
    Ok(None)
}

#[cfg(target_os = "windows")]
#[expect(unsafe_code)]
async fn set_file_time_via_handle(
    file_handles: &ristretto_types::handles::HandleManager<
        i64,
        ristretto_types::handles::FileHandle,
    >,
    fd: i64,
    create_time: i64,
    last_access_time: i64,
    last_write_time: i64,
) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::{FILETIME, HANDLE};
    use windows_sys::Win32::Storage::FileSystem::SetFileTime;
    let file_handle = file_handles.get(&fd).await.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "SetFileTime0: bad handle")
    })?;
    let raw_handle = file_handle.file.as_raw_handle() as HANDLE;
    let to_filetime = |value: i64| -> FILETIME {
        let v = u64::from_ne_bytes(value.to_ne_bytes());
        FILETIME {
            dwLowDateTime: u32::try_from(v & 0xFFFF_FFFF).unwrap_or(0),
            dwHighDateTime: u32::try_from(v >> 32).unwrap_or(0),
        }
    };
    let create_ft = to_filetime(create_time);
    let access_ft = to_filetime(last_access_time);
    let write_ft = to_filetime(last_write_time);
    let create_ptr = if create_time == -1 {
        std::ptr::null()
    } else {
        std::ptr::from_ref(&create_ft)
    };
    let access_ptr = if last_access_time == -1 {
        std::ptr::null()
    } else {
        std::ptr::from_ref(&access_ft)
    };
    let write_ptr = if last_write_time == -1 {
        std::ptr::null()
    } else {
        std::ptr::from_ref(&write_ft)
    };
    let ok = unsafe { SetFileTime(raw_handle, create_ptr, access_ptr, write_ptr) };
    if ok == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_file_time0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let last_write_time = parameters.pop_long()?;
    let last_access_time = parameters.pop_long()?;
    let create_time = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    if let Err(e) = set_file_time_via_handle(
        vm.file_handles(),
        handle,
        create_time,
        last_access_time,
        last_write_time,
    )
    .await
    {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_dacl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let acl_address = parameters.pop_long()?;
    let desc_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let acl = if acl_address == 0 {
        None
    } else {
        Some(read_guest_acl(&*vm, acl_address)?)
    };
    let descriptors = security_descriptors(&*vm)?;
    let mut descriptors = descriptors
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?;
    let descriptor = descriptors.get_mut(&desc_address).ok_or_else(|| {
        InternalError("SetSecurityDescriptorDacl: uninitialized descriptor".to_string())
    })?;
    descriptor.dacl = acl;
    descriptor.dacl_address = acl_address;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_owner<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let owner_address = parameters.pop_long()?;
    let desc_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let owner = if owner_address == 0 {
        None
    } else {
        Some(read_guest_sid(&*vm, owner_address)?)
    };
    let descriptors = security_descriptors(&*vm)?;
    let mut descriptors = descriptors
        .0
        .lock()
        .map_err(|_| InternalError("poisoned security descriptor map".to_string()))?;
    let descriptor = descriptors.get_mut(&desc_address).ok_or_else(|| {
        InternalError("SetSecurityDescriptorOwner: uninitialized descriptor".to_string())
    })?;
    descriptor.owner = owner;
    descriptor.owner_address = owner_address;
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V", Any)]
#[async_method]
pub async fn set_thread_token<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let token = parameters.pop_long()?;
    let thread_handle = parameters.pop_long()?;
    use windows_sys::Win32::Foundation::HANDLE;
    let error = {
        let thread_handle = thread_handle as usize as HANDLE;
        let thread_pointer = if thread_handle.is_null() {
            std::ptr::null()
        } else {
            &raw const thread_handle
        };
        #[expect(unsafe_code)]
        let ok = unsafe {
            windows_sys::Win32::System::Threading::SetThreadToken(
                thread_pointer,
                token as usize as HANDLE,
            )
        };
        if ok == 0 { last_windows_error() } else { 0 }
    };
    if error != 0 {
        return Err(throw_windows_exception(&thread, error).await);
    }
    Ok(None)
}

#[cfg(all(test, not(target_family = "windows")))]
mod tests {
    use super::*;

    #[test]
    fn test_file_size_dword_preserves_unsigned_bits() {
        assert_eq!(i32::MIN, file_size_dword(0x8000_0000));
        assert_eq!(-1, file_size_dword(0xFFFF_FFFF));
        assert_eq!(1, file_size_dword(0x1_0000_0001));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_dos_attributes_round_trip() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        let requested = file_attribute::HIDDEN | file_attribute::SYSTEM | file_attribute::ARCHIVE;
        set_windows_file_attributes(file.path(), requested).expect("set DOS attributes");
        let metadata = std::fs::symlink_metadata(file.path()).expect("metadata");
        let actual = metadata_file_attributes(&metadata);
        assert_eq!(requested, actual & requested);
        set_windows_file_attributes(file.path(), file_attribute::NORMAL)
            .expect("reset DOS attributes");
    }

    #[tokio::test]
    async fn test_close_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(999);
        let result = close_handle(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_file_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_close() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(1);
        let result = find_close(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_first_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_file_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_attributes_ex_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_attributes_ex_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_full_path_name_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_full_path_name_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_access_check() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = access_check(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(Some(Value::Int(1)), result);
    }

    #[tokio::test]
    async fn test_add_access_allowed_ace_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_access_allowed_ace_ex(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_add_access_denied_ace_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_access_denied_ace_ex(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_adjust_token_privileges() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = adjust_token_privileges(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_backup_read0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = backup_read0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::from(false),
                Value::Long(0),
                Value::Object(None),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_backup_seek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = backup_seek(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_cancel_io() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cancel_io(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.expect("cancel_io ok").is_none());
    }

    #[tokio::test]
    async fn test_convert_sid_to_string_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convert_sid_to_string_sid(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_convert_string_sid_to_sid0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convert_string_sid_to_sid0(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(1)), result);
    }

    #[tokio::test]
    async fn test_copy_file_ex0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_file_ex0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_directory0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_directory0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_event(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        let value = result.expect("create_event ok").expect("value");
        assert!(matches!(value, Value::Long(h) if h != 0));
    }

    #[tokio::test]
    async fn test_create_hard_link0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_hard_link0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_io_completion_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_io_completion_port(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        let value = result
            .expect("create_io_completion_port ok")
            .expect("value");
        assert!(matches!(value, Value::Long(h) if h != 0));
    }

    #[tokio::test]
    async fn test_create_symbolic_link0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_symbolic_link0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delete_file0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_device_io_control_get_reparse_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_io_control_get_reparse_point(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_device_io_control_set_sparse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_io_control_set_sparse(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_duplicate_token_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            duplicate_token_ex(thread, Parameters::new(vec![Value::Long(7), Value::Int(0)]))
                .await
                .expect("ok");
        assert_eq!(Some(Value::Long(7)), result);
    }

    #[tokio::test]
    async fn test_find_first_file1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_file1(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_first_stream0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_stream0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_next_file() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_next_file(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(-1)]),
        )
        .await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_next_file0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_next_file0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(-1)]),
        )
        .await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_next_stream() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Object(None)), result);
    }

    #[tokio::test]
    async fn test_find_next_stream0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream0(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Object(None)), result);
    }

    #[tokio::test]
    async fn test_format_message() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = format_message(thread, Parameters::new(vec![Value::Int(0)])).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_ace() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ace(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_get_acl_information0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_acl_information0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_current_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_process(thread, Parameters::default())
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(-1)), result);
    }

    #[tokio::test]
    async fn test_get_current_thread() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_thread(thread, Parameters::default())
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(-2)), result);
    }

    #[tokio::test]
    async fn test_get_disk_free_space0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_disk_free_space_ex0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space_ex0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_drive_type0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_drive_type0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Null pointer should fail with a WindowsException
        let result = get_file_attributes0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_information_by_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_information_by_handle0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_security0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_security0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_get_file_size_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_size_ex(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_final_path_name_by_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_final_path_name_by_handle(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_length_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_length_sid(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Int(12)), result);
    }

    #[tokio::test]
    async fn test_get_logical_drives() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_logical_drives(thread, Parameters::default()).await;
        let value = result.expect("ok").expect("some");
        let mask = value.as_i32().expect("i32");
        assert!(mask != 0, "expected at least one logical drive bit set");
    }

    #[tokio::test]
    async fn test_get_overlapped_result() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_overlapped_result(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            Some(Value::Int(0)),
            result.expect("get_overlapped_result ok")
        );
    }

    #[tokio::test]
    async fn test_get_queued_completion_status0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_queued_completion_status0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        // null status object surfaces as InternalError
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_security_descriptor_dacl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_security_descriptor_dacl(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_get_security_descriptor_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_security_descriptor_owner(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_get_token_information() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_token_information(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await
        .expect("ok");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_get_volume_information0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volume_information0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_volume_path_name0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volume_path_name0(thread, Parameters::new(vec![Value::Long(0)])).await;
        // Address 0 yields a bad-address read error.
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_initialize_acl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_acl(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
            .await
            .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_initialize_security_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_security_descriptor(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_local_free() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = local_free(thread, Parameters::new(vec![Value::Long(0)])).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_lookup_account_name0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_name0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await
        .expect("ok");
        assert_eq!(Some(Value::Int(0)), result);
    }

    #[tokio::test]
    async fn test_lookup_account_sid0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_sid0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lookup_privilege_value0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = lookup_privilege_value0(thread, Parameters::new(vec![Value::Long(0)])).await?;
        assert_eq!(Some(Value::Long(1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_move_file_ex0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_file_ex0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_open_process_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            open_process_token(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await
                .expect("ok");
        assert_eq!(Some(Value::Long(1)), result);
    }

    #[tokio::test]
    async fn test_open_thread_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_thread_token(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::from(false)]),
        )
        .await
        .expect("ok");
        assert_eq!(Some(Value::Long(0)), result);
    }

    #[tokio::test]
    async fn test_post_queued_completion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Posting to a non-existent port surfaces ERROR_INVALID_HANDLE.
        let result = post_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_directory_changes_w() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_directory_changes_w(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert!(result.expect("read_directory_changes_w ok").is_none());
    }

    #[tokio::test]
    async fn test_remove_directory0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_directory0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_end_of_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_end_of_file(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_file_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_file_attributes0(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_file_security0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_file_security0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_file_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_file_time(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_file_time0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Handle 0 is invalid; call should surface a Windows exception.
        let result = set_file_time0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(-1),
                Value::Long(-1),
                Value::Long(-1),
            ]),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_security_descriptor_dacl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_dacl(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_set_security_descriptor_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_owner(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await
        .expect("ok");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_set_thread_token() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_thread_token(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await?;
        assert_eq!(None, result);
        Ok(())
    }
}

#[cfg(all(test, target_family = "windows"))]
mod windows_tests {
    use super::*;

    #[test]
    fn test_file_size_dword_preserves_unsigned_bits() {
        assert_eq!(i32::MIN, file_size_dword(0x8000_0000));
        assert_eq!(-1, file_size_dword(0xFFFF_FFFF));
        assert_eq!(1, file_size_dword(0x1_0000_0001));
    }

    #[test]
    fn test_dos_attributes_round_trip() {
        let file = tempfile::NamedTempFile::new().expect("temp file");
        let requested = file_attribute::HIDDEN | file_attribute::SYSTEM | file_attribute::ARCHIVE;
        set_windows_file_attributes(file.path(), requested).expect("set DOS attributes");
        let metadata = std::fs::symlink_metadata(file.path()).expect("metadata");
        let actual = metadata_file_attributes(&metadata);
        assert_eq!(requested, actual & requested);
        set_windows_file_attributes(file.path(), file_attribute::NORMAL)
            .expect("reset DOS attributes");
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        assert_eq!(None, init_ids(thread, Parameters::default()).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_native_arguments_are_rejected() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let access_parameters = Parameters::new(vec![
            Value::Long(0),
            Value::Long(0),
            Value::Int(0),
            Value::Int(0),
            Value::Int(0),
            Value::Int(0),
            Value::Int(0),
        ]);
        assert!(
            access_check(thread.clone(), access_parameters)
                .await
                .is_err()
        );

        let ace_parameters = Parameters::new(vec![
            Value::Long(0),
            Value::Int(0),
            Value::Int(0),
            Value::Long(0),
        ]);
        assert!(
            add_access_allowed_ace_ex(thread.clone(), ace_parameters.clone())
                .await
                .is_err()
        );
        assert!(
            add_access_denied_ace_ex(thread.clone(), ace_parameters)
                .await
                .is_err()
        );

        assert!(
            adjust_token_privileges(
                thread.clone(),
                Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
            )
            .await
            .is_err()
        );
        assert!(
            backup_read0(
                thread.clone(),
                Parameters::new(vec![
                    Value::Long(0),
                    Value::Long(0),
                    Value::Int(0),
                    Value::from(false),
                    Value::Long(0),
                    Value::Object(None),
                ]),
            )
            .await
            .is_err()
        );
        assert!(
            backup_seek(
                thread.clone(),
                Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
            )
            .await
            .is_err()
        );
        assert!(
            cancel_io(thread.clone(), Parameters::new(vec![Value::Long(0)]),)
                .await
                .is_err()
        );
        assert!(
            convert_sid_to_string_sid(thread.clone(), Parameters::new(vec![Value::Long(0)]),)
                .await
                .is_err()
        );
        assert!(
            convert_string_sid_to_sid0(thread.clone(), Parameters::new(vec![Value::Long(0)]),)
                .await
                .is_err()
        );
        assert!(
            create_io_completion_port(
                thread.clone(),
                Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
            )
            .await
            .is_err()
        );
        assert!(
            device_io_control_set_sparse(thread, Parameters::new(vec![Value::Long(0)]),)
                .await
                .is_err()
        );
        Ok(())
    }
}
