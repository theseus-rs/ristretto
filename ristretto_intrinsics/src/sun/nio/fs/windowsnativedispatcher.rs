use crate::sun::nio::fs::managed_files;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
#[cfg(target_os = "windows")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

bitflags! {
    /// Windows generic access rights for `CreateFile`.
    ///
    /// See [Generic Access Rights](https://learn.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights).
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct DesiredAccess: u32 {
        const GENERIC_READ = 0x8000_0000;
        const GENERIC_WRITE = 0x4000_0000;
    }
}

/// Windows file creation disposition values for `CreateFile`.
///
/// See [CreateFileW (dwCreationDisposition)](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew).
mod creation_disposition {
    pub const CREATE_NEW: i32 = 1;
    pub const CREATE_ALWAYS: i32 = 2;
    // OPEN_EXISTING = 3 (default / fallthrough)
    pub const OPEN_ALWAYS: i32 = 4;
    pub const TRUNCATE_EXISTING: i32 = 5;
}

/// POSIX-style open flags used by `managed_files::open`.
mod posix_open_flags {
    /// Read-only access.
    pub const O_RDONLY: i32 = 0;
    /// Write-only access.
    pub const O_WRONLY: i32 = 1;
    /// Read-write access.
    pub const O_RDWR: i32 = 2;
    /// Create file if it does not exist.
    pub const O_CREAT: i32 = 0x40;
    /// Exclusive creation (fail if file exists).
    pub const O_EXCL: i32 = 0x80;
    /// Truncate file to zero length.
    pub const O_TRUNC: i32 = 0x200;
}

/// Windows file attribute constants.
///
/// See [File Attribute Constants](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants).
mod file_attribute {
    pub const DIRECTORY: i32 = 0x10;
    pub const NORMAL: i32 = 0x80;
}

/// Field offsets within the `WIN32_FILE_ATTRIBUTE_DATA` structure written to native memory.
///
/// See [WIN32_FILE_ATTRIBUTE_DATA](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-win32_file_attribute_data).
mod file_attribute_data_offset {
    /// `dwFileAttributes` (DWORD, 4 bytes)
    pub const FILE_ATTRIBUTES: i64 = 0;
    /// `ftCreationTime` (FILETIME, 8 bytes)
    pub const CREATION_TIME: i64 = 4;
    /// `ftLastAccessTime` (FILETIME, 8 bytes)
    pub const LAST_ACCESS_TIME: i64 = 12;
    /// `ftLastWriteTime` (FILETIME, 8 bytes)
    pub const LAST_WRITE_TIME: i64 = 20;
    /// `nFileSizeHigh` (DWORD, 4 bytes)
    pub const FILE_SIZE_HIGH: i64 = 28;
    /// `nFileSizeLow` (DWORD, 4 bytes)
    pub const FILE_SIZE_LOW: i64 = 32;
}

/// Bit mask for extracting the low 32 bits of a 64-bit file size.
const FILE_SIZE_LOW_MASK: u64 = 0xFFFF_FFFF;

/// Number of bits in a DWORD, used to split a 64-bit file size into high and low 32-bit parts.
const DWORD_BITS: u64 = 32;

/// Placeholder handle value returned by `FindFirstFile` to indicate a valid (non-`INVALID_HANDLE_VALUE`) result.
const FIND_FILE_PLACEHOLDER_HANDLE: i64 = 1;

/// Read a null-terminated UTF-16 string from native memory at the given address.
fn read_native_string<T: Thread + 'static>(
    thread: &Arc<T>,
    address: i64,
    context: &str,
) -> Result<String> {
    let vm = thread.vm()?;
    let native_memory = vm.native_memory();
    let mut path_chars = Vec::new();
    let mut offset = 0i64;
    loop {
        let word = native_memory
            .read_i16(address + offset)
            .ok_or_else(|| InternalError(format!("{context}: bad address")))?;
        if word == 0 {
            break;
        }
        path_chars.push(word.cast_unsigned());
        offset += 2;
    }
    Ok(String::from_utf16_lossy(&path_chars))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CloseHandle(J)V", Any)]
#[async_method]
pub async fn close_handle<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    managed_files::close(vm.file_handles(), handle).await;
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
    let _security_attrs = parameters.pop_long()?;
    let _share_mode = parameters.pop_int()?;
    let desired_access = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let path_str = read_native_string(&thread, path_address, "CreateFile0")?;

    // Map Windows CreateFile parameters to POSIX-style flags for managed_files::open
    let access = DesiredAccess::from_bits_truncate(desired_access.cast_unsigned());
    let read = access.contains(DesiredAccess::GENERIC_READ);
    let write = access.contains(DesiredAccess::GENERIC_WRITE);
    let access_mode: i32 = match (read, write) {
        (true, true) => posix_open_flags::O_RDWR,
        (false, true) => posix_open_flags::O_WRONLY,
        _ => posix_open_flags::O_RDONLY,
    };

    let create_flag: i32 = match creation_disposition {
        creation_disposition::CREATE_NEW => posix_open_flags::O_CREAT | posix_open_flags::O_EXCL,
        creation_disposition::CREATE_ALWAYS => {
            posix_open_flags::O_CREAT | posix_open_flags::O_TRUNC
        }
        creation_disposition::OPEN_ALWAYS => posix_open_flags::O_CREAT,
        creation_disposition::TRUNCATE_EXISTING => posix_open_flags::O_TRUNC,
        _ => 0, // OPEN_EXISTING or default
    };

    let _ = flags_and_attrs;
    let flags = access_mode | create_flag;
    let vm = thread.vm()?;
    let fd = managed_files::open(
        vm.file_handles(),
        vm.resource_manager(),
        &path_str,
        flags,
        0,
    )
    .await
    .map_err(|e| InternalError(format!("CreateFile0: {e}")))?;
    Ok(Some(Value::Long(fd)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.FindClose(J)V", Any)]
#[async_method]
pub async fn find_close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
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
    let path_str = read_native_string(&thread, path_address, "FindFirstFile0")?;

    let metadata =
        std::fs::metadata(&path_str).map_err(|e| InternalError(format!("FindFirstFile0: {e}")))?;
    let file_attributes: i32 = if metadata.is_dir() {
        file_attribute::DIRECTORY
    } else {
        file_attribute::NORMAL
    };

    let filename = std::path::Path::new(&path_str)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&path_str)
        .to_string();
    let name_value = thread.intern_string(&filename).await?;

    let mut guard = first_file_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError("FindFirstFile0: not an object".to_string()));
    };
    obj.set_value("handle", Value::Long(FIND_FILE_PLACEHOLDER_HANDLE))?;
    obj.set_value("name", name_value)?;
    obj.set_value("attributes", Value::Int(file_attributes))?;
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
    let path_str = read_native_string(&thread, path_address, "GetFileAttributesEx0")?;
    let path = std::path::Path::new(&path_str);

    let vm = thread.vm()?;
    let native_memory = vm.native_memory();

    match std::fs::metadata(path) {
        Ok(metadata) => {
            let file_attributes: i32 = if metadata.is_dir() {
                file_attribute::DIRECTORY
            } else {
                file_attribute::NORMAL
            };
            // dwFileAttributes (4 bytes)
            native_memory.write_i32(
                address + file_attribute_data_offset::FILE_ATTRIBUTES,
                file_attributes,
            );
            // ftCreationTime (8 bytes) - set to 0
            native_memory.write_i64(address + file_attribute_data_offset::CREATION_TIME, 0);
            // ftLastAccessTime (8 bytes) - set to 0
            native_memory.write_i64(address + file_attribute_data_offset::LAST_ACCESS_TIME, 0);
            // ftLastWriteTime (8 bytes) - set to 0
            native_memory.write_i64(address + file_attribute_data_offset::LAST_WRITE_TIME, 0);
            // nFileSizeHigh (4 bytes) + nFileSizeLow (4 bytes)
            let size = metadata.len();
            let size_high = i32::try_from(size >> DWORD_BITS)
                .map_err(|_| InternalError("GetFileAttributesEx0: size overflow".to_string()))?;
            let size_low = i32::try_from(size & FILE_SIZE_LOW_MASK)
                .map_err(|_| InternalError("GetFileAttributesEx0: size overflow".to_string()))?;
            native_memory.write_i32(
                address + file_attribute_data_offset::FILE_SIZE_HIGH,
                size_high,
            );
            native_memory.write_i32(
                address + file_attribute_data_offset::FILE_SIZE_LOW,
                size_low,
            );
            Ok(None)
        }
        Err(e) => Err(InternalError(format!("GetFileAttributesEx0: {e}"))),
    }
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
    let path_str = read_native_string(&thread, path_address, "GetFullPathName0")?;
    let full_path = std::fs::canonicalize(&path_str)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(path_str);

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

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z", Any)]
#[async_method]
pub async fn access_check<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _generic_all = parameters.pop_int()?;
    let _generic_execute = parameters.pop_int()?;
    let _generic_write = parameters.pop_int()?;
    let _generic_read = parameters.pop_int()?;
    let _access_mask = parameters.pop_int()?;
    let _security_info = parameters.pop_long()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_allowed_ace_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sid_address = parameters.pop_long()?;
    let _mask = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _acl_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_denied_ace_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sid_address = parameters.pop_long()?;
    let _mask = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _acl_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V", Any)]
#[async_method]
pub async fn adjust_token_privileges<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attributes = parameters.pop_int()?;
    let _luid = parameters.pop_long()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn backup_read0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg5 = parameters.pop_reference()?;
    let _arg4 = parameters.pop_long()?;
    let _arg3 = parameters.pop_bool()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V", Equal(JAVA_8))]
#[async_method]
pub async fn backup_seek<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_long()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V", Any)]
#[async_method]
pub async fn cancel_io<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_file = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn convert_sid_to_string_sid<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J", Any)]
#[async_method]
pub async fn convert_string_sid_to_sid0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V", Any)]
#[async_method]
pub async fn copy_file_ex0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cancel_address = parameters.pop_long()?;
    let _flags = parameters.pop_int()?;
    let _new_address = parameters.pop_long()?;
    let _existing_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V", Any)]
#[async_method]
pub async fn create_directory0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sd_address = parameters.pop_long()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J", Any)]
#[async_method]
pub async fn create_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _b_initial_state = parameters.pop_bool()?;
    let _b_manual_reset = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V", Any)]
#[async_method]
pub async fn create_hard_link0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _existing_file_address = parameters.pop_long()?;
    let _new_file_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_long()?;
    let _existing_port = parameters.pop_long()?;
    let _file_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V", Any)]
#[async_method]
pub async fn create_symbolic_link0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _target_address = parameters.pop_long()?;
    let _link_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V", Any)]
#[async_method]
pub async fn delete_file0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
    Any
)]
#[async_method]
pub async fn device_io_control_get_reparse_point<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer_size = parameters.pop_int()?;
    let _buffer_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V", Any)]
#[async_method]
pub async fn device_io_control_set_sparse<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J", Any)]
#[async_method]
pub async fn duplicate_token_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J", Any)]
#[async_method]
pub async fn find_first_file1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V",
    Any
)]
#[async_method]
pub async fn find_first_stream0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_file0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_stream<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_stream0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _error_code = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J", Any)]
#[async_method]
pub async fn get_ace<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ace_index = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V",
    Any
)]
#[async_method]
pub async fn get_acl_information0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J", Any)]
#[async_method]
pub async fn get_current_process<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J", Any)]
#[async_method]
pub async fn get_current_thread<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_disk_free_space0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    Any
)]
#[async_method]
pub async fn get_disk_free_space_ex0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I", Any)]
#[async_method]
pub async fn get_drive_type0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I", Any)]
#[async_method]
pub async fn get_file_attributes0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_information_by_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_file_information_by_handle0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I", Any)]
#[async_method]
pub async fn get_file_security0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _n_length = parameters.pop_int()?;
    let _desc_address = parameters.pop_long()?;
    let _requested_information = parameters.pop_int()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_size_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_final_path_name_by_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I", Any)]
#[async_method]
pub async fn get_length_sid<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I", Any)]
#[async_method]
pub async fn get_logical_drives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I", Any)]
#[async_method]
pub async fn get_overlapped_result<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_overlapped = parameters.pop_long()?;
    let _h_file = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
    Any
)]
#[async_method]
pub async fn get_queued_completion_status0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_dacl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I", Any)]
#[async_method]
pub async fn get_token_information<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token_info_length = parameters.pop_int()?;
    let _token_info = parameters.pop_long()?;
    let _token_info_class = parameters.pop_int()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
    Any
)]
#[async_method]
pub async fn get_volume_information0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_volume_path_name0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V", Any)]
#[async_method]
pub async fn initialize_acl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V",
    Any
)]
#[async_method]
pub async fn initialize_security_descriptor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V", Any)]
#[async_method]
pub async fn local_free<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I", Any)]
#[async_method]
pub async fn lookup_account_name0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cb_sid = parameters.pop_int()?;
    let _sid_address = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V",
    Any
)]
#[async_method]
pub async fn lookup_account_sid0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J", Any)]
#[async_method]
pub async fn lookup_privilege_value0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V", Any)]
#[async_method]
pub async fn move_file_ex0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _new_address = parameters.pop_long()?;
    let _existing_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J", Any)]
#[async_method]
pub async fn open_process_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let _process = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J", Any)]
#[async_method]
pub async fn open_thread_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _open_as_self = parameters.pop_bool()?;
    let _desired_access = parameters.pop_int()?;
    let _thread = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V",
    Any
)]
#[async_method]
pub async fn post_queued_completion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_long()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V",
    Any
)]
#[async_method]
pub async fn read_directory_changes_w<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_overlapped = parameters.pop_long()?;
    let _bytes_returned_address = parameters.pop_long()?;
    let _filter = parameters.pop_int()?;
    let _watch_sub_tree = parameters.pop_bool()?;
    let _buffer_length = parameters.pop_int()?;
    let _buffer_address = parameters.pop_long()?;
    let _h_directory = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V", Any)]
#[async_method]
pub async fn remove_directory0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V", Any)]
#[async_method]
pub async fn set_end_of_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V", Any)]
#[async_method]
pub async fn set_file_attributes0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V", Any)]
#[async_method]
pub async fn set_file_security0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desc_address = parameters.pop_long()?;
    let _requested_information = parameters.pop_int()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_file_time<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _last_write_time = parameters.pop_long()?;
    let _last_access_time = parameters.pop_long()?;
    let _create_time = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_file_time0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _last_write_time = parameters.pop_long()?;
    let _last_access_time = parameters.pop_long()?;
    let _create_time = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_dacl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _acl_address = parameters.pop_long()?;
    let _desc_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _owner_address = parameters.pop_long()?;
    let _desc_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V", Any)]
#[async_method]
pub async fn set_thread_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token = parameters.pop_long()?;
    let _thread = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z", Any)]
#[async_method]
pub async fn access_check_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _generic_all = parameters.pop_int()?;
    let _generic_execute = parameters.pop_int()?;
    let _generic_write = parameters.pop_int()?;
    let _generic_read = parameters.pop_int()?;
    let _access_mask = parameters.pop_int()?;
    let _security_info = parameters.pop_long()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_allowed_ace_ex_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sid_address = parameters.pop_long()?;
    let _mask = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _acl_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V", Any)]
#[async_method]
pub async fn add_access_denied_ace_ex_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sid_address = parameters.pop_long()?;
    let _mask = parameters.pop_int()?;
    let _flags = parameters.pop_int()?;
    let _acl_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V", Any)]
#[async_method]
pub async fn adjust_token_privileges_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _attributes = parameters.pop_int()?;
    let _luid = parameters.pop_long()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn backup_read0_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg5 = parameters.pop_reference()?;
    let _arg4 = parameters.pop_long()?;
    let _arg3 = parameters.pop_bool()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V", Equal(JAVA_8))]
#[async_method]
pub async fn backup_seek_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_long()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V", Any)]
#[async_method]
pub async fn cancel_io_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_file = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn convert_sid_to_string_sid_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J", Any)]
#[async_method]
pub async fn convert_string_sid_to_sid0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V", Any)]
#[async_method]
pub async fn copy_file_ex0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cancel_address = parameters.pop_long()?;
    let _flags = parameters.pop_int()?;
    let _new_address = parameters.pop_long()?;
    let _existing_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V", Any)]
#[async_method]
pub async fn create_directory0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sd_address = parameters.pop_long()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J", Any)]
#[async_method]
pub async fn create_event_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _b_initial_state = parameters.pop_bool()?;
    let _b_manual_reset = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V", Any)]
#[async_method]
pub async fn create_hard_link0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _existing_file_address = parameters.pop_long()?;
    let _new_file_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J", Any)]
#[async_method]
pub async fn create_io_completion_port_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_long()?;
    let _existing_port = parameters.pop_long()?;
    let _file_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V", Any)]
#[async_method]
pub async fn create_symbolic_link0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _target_address = parameters.pop_long()?;
    let _link_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V", Any)]
#[async_method]
pub async fn delete_file0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
    Any
)]
#[async_method]
pub async fn device_io_control_get_reparse_point_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer_size = parameters.pop_int()?;
    let _buffer_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V", Any)]
#[async_method]
pub async fn device_io_control_set_sparse_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J", Any)]
#[async_method]
pub async fn duplicate_token_ex_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J", Any)]
#[async_method]
pub async fn find_first_file1_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V",
    Any
)]
#[async_method]
pub async fn find_first_stream0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_file_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_file0_windows_ge_v21<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_next_stream_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_next_stream0_windows_ge_v21<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_message_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _error_code = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J", Any)]
#[async_method]
pub async fn get_ace_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ace_index = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V",
    Any
)]
#[async_method]
pub async fn get_acl_information0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J", Any)]
#[async_method]
pub async fn get_current_process_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J", Any)]
#[async_method]
pub async fn get_current_thread_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_disk_free_space0_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
    Any
)]
#[async_method]
pub async fn get_disk_free_space_ex0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I", Any)]
#[async_method]
pub async fn get_drive_type0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I", Any)]
#[async_method]
pub async fn get_file_attributes0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_information_by_handle_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_file_information_by_handle0_windows_ge_v21<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I", Any)]
#[async_method]
pub async fn get_file_security0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _n_length = parameters.pop_int()?;
    let _desc_address = parameters.pop_long()?;
    let _requested_information = parameters.pop_int()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_file_size_ex_windows_ge_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_final_path_name_by_handle_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I", Any)]
#[async_method]
pub async fn get_length_sid_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I", Any)]
#[async_method]
pub async fn get_logical_drives_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I", Any)]
#[async_method]
pub async fn get_overlapped_result_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_overlapped = parameters.pop_long()?;
    let _h_file = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
    Any
)]
#[async_method]
pub async fn get_queued_completion_status0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_dacl_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J",
    Any
)]
#[async_method]
pub async fn get_security_descriptor_owner_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I", Any)]
#[async_method]
pub async fn get_token_information_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token_info_length = parameters.pop_int()?;
    let _token_info = parameters.pop_long()?;
    let _token_info_class = parameters.pop_int()?;
    let _token = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
    Any
)]
#[async_method]
pub async fn get_volume_information0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_volume_path_name0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V", Any)]
#[async_method]
pub async fn initialize_acl_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V",
    Any
)]
#[async_method]
pub async fn initialize_security_descriptor_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V", Any)]
#[async_method]
pub async fn local_free_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I", Any)]
#[async_method]
pub async fn lookup_account_name0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cb_sid = parameters.pop_int()?;
    let _sid_address = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V",
    Any
)]
#[async_method]
pub async fn lookup_account_sid0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V".to_string()).into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J", Any)]
#[async_method]
pub async fn lookup_privilege_value0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V", Any)]
#[async_method]
pub async fn move_file_ex0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _new_address = parameters.pop_long()?;
    let _existing_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J", Any)]
#[async_method]
pub async fn open_process_token_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let _process = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J", Any)]
#[async_method]
pub async fn open_thread_token_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _open_as_self = parameters.pop_bool()?;
    let _desired_access = parameters.pop_int()?;
    let _thread = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V",
    Any
)]
#[async_method]
pub async fn post_queued_completion_status_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_long()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V",
    Any
)]
#[async_method]
pub async fn read_directory_changes_w_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_overlapped = parameters.pop_long()?;
    let _bytes_returned_address = parameters.pop_long()?;
    let _filter = parameters.pop_int()?;
    let _watch_sub_tree = parameters.pop_bool()?;
    let _buffer_length = parameters.pop_int()?;
    let _buffer_address = parameters.pop_long()?;
    let _h_directory = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V", Any)]
#[async_method]
pub async fn remove_directory0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V", Any)]
#[async_method]
pub async fn set_end_of_file_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V", Any)]
#[async_method]
pub async fn set_file_attributes0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V", Any)]
#[async_method]
pub async fn set_file_security0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desc_address = parameters.pop_long()?;
    let _requested_information = parameters.pop_int()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_file_time_windows_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _last_write_time = parameters.pop_long()?;
    let _last_access_time = parameters.pop_long()?;
    let _create_time = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_file_time0_windows_ge_v21<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _last_write_time = parameters.pop_long()?;
    let _last_access_time = parameters.pop_long()?;
    let _create_time = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_dacl_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _acl_address = parameters.pop_long()?;
    let _desc_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V",
    Any
)]
#[async_method]
pub async fn set_security_descriptor_owner_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _owner_address = parameters.pop_long()?;
    let _desc_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V", Any)]
#[async_method]
pub async fn set_thread_token_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token = parameters.pop_long()?;
    let _thread = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

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

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_adjust_token_privileges() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = adjust_token_privileges(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_backup_seek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = backup_seek(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_cancel_io() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cancel_io(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_convert_sid_to_string_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convert_sid_to_string_sid(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_convert_string_sid_to_sid0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            convert_string_sid_to_sid0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_directory0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_directory0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_event(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_hard_link0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_hard_link0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_io_completion_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_io_completion_port(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_symbolic_link0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_symbolic_link0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_delete_file0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delete_file0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_io_control_get_reparse_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_io_control_get_reparse_point(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_io_control_set_sparse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            device_io_control_set_sparse(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_duplicate_token_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            duplicate_token_ex(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_first_file1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_file1(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_first_stream0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_stream0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_file(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_file0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_file0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_stream() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_stream0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_format_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = format_message(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_ace() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ace(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_acl_information0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_acl_information0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_process(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_thread() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_thread(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_disk_free_space0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_disk_free_space_ex0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space_ex0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_drive_type0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_drive_type0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_attributes0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_information_by_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_information_by_handle0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_size_ex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_size_ex(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_final_path_name_by_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_final_path_name_by_handle(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_length_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_length_sid(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_logical_drives() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_logical_drives(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_overlapped_result() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_overlapped_result(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_queued_completion_status0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_queued_completion_status0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_security_descriptor_dacl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_security_descriptor_dacl(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_security_descriptor_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_security_descriptor_owner(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_volume_information0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volume_information0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_volume_path_name0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volume_path_name0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize_acl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            initialize_acl(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize_security_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            initialize_security_descriptor(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_free() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_free(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_account_name0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_name0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_account_sid0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_sid0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_privilege_value0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_privilege_value0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_move_file_ex0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_file_ex0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_process_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            open_process_token(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_thread_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_thread_token(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_post_queued_completion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = post_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_directory0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_directory0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_end_of_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_end_of_file(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_file_attributes0(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_security0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_security0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_time(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_time0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_time0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_security_descriptor_dacl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_dacl(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_security_descriptor_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_owner(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_thread_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_token(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_access_check_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = access_check_windows(
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AccessCheck(JJIIIII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_access_allowed_ace_ex_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_access_allowed_ace_ex_windows(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_access_denied_ace_ex_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_access_denied_ace_ex_windows(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_adjust_token_privileges_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = adjust_token_privileges_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AdjustTokenPrivileges(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_backup_read0_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = backup_read0_windows_v8(
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.BackupRead0(JJIZJLsun/nio/fs/WindowsNativeDispatcher$BackupResult;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_backup_seek_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = backup_seek_windows_v8(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_cancel_io_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cancel_io_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_convert_sid_to_string_sid_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            convert_sid_to_string_sid_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_convert_string_sid_to_sid0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            convert_string_sid_to_sid0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_copy_file_ex0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_file_ex0_windows(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_directory0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_directory0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_event_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_event_windows(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_hard_link0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_hard_link0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_io_completion_port_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_io_completion_port_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_symbolic_link0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_symbolic_link0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateSymbolicLink0(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_delete_file0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delete_file0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_io_control_get_reparse_point_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = device_io_control_get_reparse_point_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_device_io_control_set_sparse_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            device_io_control_set_sparse_windows(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlSetSparse(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_duplicate_token_ex_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = duplicate_token_ex_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_first_file1_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_file1_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_first_stream0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_first_stream0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindFirstStream0(JLsun/nio/fs/WindowsNativeDispatcher$FirstStream;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_file_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_file_windows_le_v17(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextFile(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_file0_windows_ge_v21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_file0_windows_ge_v21(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextFile0(JJ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_stream_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            find_next_stream_windows_le_v17(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_next_stream0_windows_ge_v21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            find_next_stream0_windows_ge_v21(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_format_message_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = format_message_windows(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_ace_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_ace_windows(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_acl_information0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_acl_information0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetAclInformation0(JLsun/nio/fs/WindowsNativeDispatcher$AclInformation;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_process_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_process_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_thread_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_thread_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_disk_free_space0_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space0_windows_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_disk_free_space_ex0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_disk_free_space_ex0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpaceEx0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_drive_type0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_drive_type0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_attributes0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_file_attributes0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_information_by_handle_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle_windows_le_v17(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_information_by_handle0_windows_ge_v21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_information_by_handle0_windows_ge_v21(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileInformationByHandle0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_security0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_security0_windows(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileSecurity0(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_file_size_ex_windows_ge_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_file_size_ex_windows_ge_v17(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_final_path_name_by_handle_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_final_path_name_by_handle_windows(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_length_sid_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_length_sid_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_logical_drives_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_logical_drives_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_overlapped_result_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_overlapped_result_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_queued_completion_status0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_queued_completion_status0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_security_descriptor_dacl_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_security_descriptor_dacl_windows(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorDacl(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_security_descriptor_owner_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_security_descriptor_owner_windows(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetSecurityDescriptorOwner(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_token_information_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_token_information_windows(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetTokenInformation(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_volume_information0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volume_information0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetVolumeInformation0(JLsun/nio/fs/WindowsNativeDispatcher$VolumeInformation;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_volume_path_name0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_volume_path_name0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize_acl_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            initialize_acl_windows(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize_security_descriptor_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            initialize_security_descriptor_windows(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.InitializeSecurityDescriptor(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_local_free_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_free_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_account_name0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_name0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupAccountName0(JJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_account_sid0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_account_sid0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupAccountSid0(JLsun/nio/fs/WindowsNativeDispatcher$Account;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lookup_privilege_value0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            lookup_privilege_value0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_move_file_ex0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_file_ex0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_process_token_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_process_token_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_thread_token_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_thread_token_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_post_queued_completion_status_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = post_queued_completion_status_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.PostQueuedCompletionStatus(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_directory_changes_w_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_directory_changes_w_windows(
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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ReadDirectoryChangesW(JJIZIJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_directory0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_directory0_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_end_of_file_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_end_of_file_windows(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetEndOfFile(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_attributes0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_attributes0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_security0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_security0_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileSecurity0(JIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_time_windows_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_time_windows_le_v17(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileTime(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_file_time0_windows_ge_v21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_time0_windows_ge_v21(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_security_descriptor_dacl_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_dacl_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_security_descriptor_owner_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_security_descriptor_owner_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorOwner(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_thread_token_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_token_windows(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V",
            result.unwrap_err().to_string()
        );
    }
}
