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
use ristretto_types::JavaError;
use ristretto_types::JavaObject;
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
        const FILE_WRITE_ATTRIBUTES = 0x0000_0100;
        const FILE_WRITE_DATA = 0x0000_0002;
        const FILE_APPEND_DATA = 0x0000_0004;
        const FILE_WRITE_EA = 0x0000_0010;
        const DELETE = 0x0001_0000;
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
    let write = access.intersects(
        DesiredAccess::GENERIC_WRITE
            | DesiredAccess::FILE_WRITE_DATA
            | DesiredAccess::FILE_APPEND_DATA
            | DesiredAccess::FILE_WRITE_EA
            | DesiredAccess::FILE_WRITE_ATTRIBUTES,
    );
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
    let fd = match managed_files::open(
        vm.file_handles(),
        vm.resource_manager(),
        &path_str,
        flags,
        0,
    )
    .await
    {
        Ok(fd) => fd,
        Err(e) => {
            let code = windows_error_code(&e);
            return Err(throw_windows_exception(&thread, code).await);
        }
    };
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
            let creation = system_time_to_filetime(metadata.created().ok());
            let access = system_time_to_filetime(metadata.accessed().ok());
            let write = system_time_to_filetime(metadata.modified().ok());
            native_memory.write_i64(
                address + file_attribute_data_offset::CREATION_TIME,
                i64::from_ne_bytes(creation.to_ne_bytes()),
            );
            native_memory.write_i64(
                address + file_attribute_data_offset::LAST_ACCESS_TIME,
                i64::from_ne_bytes(access.to_ne_bytes()),
            );
            native_memory.write_i64(
                address + file_attribute_data_offset::LAST_WRITE_TIME,
                i64::from_ne_bytes(write.to_ne_bytes()),
            );
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
        Err(e) => Err(throw_windows_exception(&thread, windows_error_code(&e)).await),
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
    // We do not enforce ACLs; report the requested access as granted so JDK's
    // checkAccess returns success and Files.isWritable / isReadable / isExecutable
    // mirror the underlying file system semantics already enforced by open/read/write.
    Ok(Some(Value::Int(1)))
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CopyFileEx0(JJIJ)V", Any)]
#[async_method]
pub async fn copy_file_ex0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cancel_address = parameters.pop_long()?;
    let _flags = parameters.pop_int()?;
    let new_address = parameters.pop_long()?;
    let existing_address = parameters.pop_long()?;
    let existing = read_native_string(&thread, existing_address, "CopyFileEx0")?;
    let new = read_native_string(&thread, new_address, "CopyFileEx0")?;
    if let Err(e) = std::fs::copy(&existing, &new) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V", Any)]
#[async_method]
pub async fn create_directory0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sd_address = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "CreateDirectory0")?;
    if let Err(e) = std::fs::create_dir(&path) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V", Any)]
#[async_method]
pub async fn delete_file0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "DeleteFile0")?;

    // Match Windows: a file with an active memory mapping cannot be deleted (sharing
    // violation). Compare via the canonicalized path stored when the mapping was created.
    let vm = thread.vm()?;
    if let Ok(regions) = vm
        .resource_manager()
        .get_or_init(crate::java::nio::mapped_regions::MappedRegions::new)
        && let Ok(canonical) = std::fs::canonicalize(&path)
        && let Some(canonical_str) = canonical.to_str()
        && regions.is_path_mapped(canonical_str)
    {
        const ERROR_SHARING_VIOLATION: i32 = 32;
        return Err(throw_windows_exception(&thread, ERROR_SHARING_VIOLATION).await);
    }

    let metadata = std::fs::symlink_metadata(&path);
    let result = match metadata {
        Ok(meta) if meta.is_dir() => std::fs::remove_dir(&path),
        _ => std::fs::remove_file(&path),
    };
    if let Err(e) = result {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DuplicateTokenEx(JI)J", Any)]
#[async_method]
pub async fn duplicate_token_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let token = parameters.pop_long()?;
    // We don't impersonate the user; return the same pseudo-token.
    Ok(Some(Value::Long(token)))
}

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
    let message = format!("Windows error code {error_code}");
    let value = thread.intern_string(&message).await?;
    Ok(Some(value))
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentProcess()J", Any)]
#[async_method]
pub async fn get_current_process<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Win32 returns a pseudo-handle (-1) for the current process.
    Ok(Some(Value::Long(-1)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetCurrentThread()J", Any)]
#[async_method]
pub async fn get_current_thread<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Win32 returns a pseudo-handle (-2) for the current thread.
    Ok(Some(Value::Long(-2)))
}

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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I".to_string(),
    )
    .into())
}

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
    let Ok(metadata) = managed_files::metadata(vm.file_handles(), handle).await else {
        return Err(throw_windows_exception(&thread, 2 /* ERROR_FILE_NOT_FOUND */).await);
    };
    let native_memory = vm.native_memory();
    let attributes = if metadata.is_dir() { 0x10u32 } else { 0x80u32 };
    let size = metadata.len();
    let size_high = u32::try_from(size >> 32).unwrap_or(0);
    let size_low = u32::try_from(size & 0xFFFF_FFFF).unwrap_or(0);
    let modified_ft = system_time_to_filetime(metadata.modified().ok());
    let created_ft = system_time_to_filetime(metadata.created().ok());
    let accessed_ft = system_time_to_filetime(metadata.accessed().ok());
    let (vol_serial, n_links, file_index) = file_identity(vm.file_handles(), handle)
        .await
        .unwrap_or((0, 1, 0));
    let index_high = u32::try_from(file_index >> 32).unwrap_or(0);
    let index_low = u32::try_from(file_index & 0xFFFF_FFFF).unwrap_or(0);
    let mut buffer = [0u8; 52];
    buffer[0..4].copy_from_slice(&attributes.to_le_bytes());
    buffer[4..12].copy_from_slice(&created_ft.to_le_bytes());
    buffer[12..20].copy_from_slice(&accessed_ft.to_le_bytes());
    buffer[20..28].copy_from_slice(&modified_ft.to_le_bytes());
    buffer[28..32].copy_from_slice(&vol_serial.to_le_bytes());
    buffer[32..36].copy_from_slice(&size_high.to_le_bytes());
    buffer[36..40].copy_from_slice(&size_low.to_le_bytes());
    buffer[40..44].copy_from_slice(&n_links.to_le_bytes());
    buffer[44..48].copy_from_slice(&index_high.to_le_bytes());
    buffer[48..52].copy_from_slice(&index_low.to_le_bytes());
    native_memory.write_bytes(address, &buffer);
    Ok(None)
}

/// Returns `(volume_serial_number, number_of_links, file_index)` for the given file handle.
#[expect(unsafe_code)]
async fn file_identity(
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

fn system_time_to_filetime(time: Option<std::time::SystemTime>) -> u64 {
    match time {
        Some(t) => match t.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => {
                // Convert Unix time to Windows FILETIME (100-ns intervals since 1601-01-01)
                d.as_secs()
                    .saturating_mul(10_000_000)
                    .saturating_add(u64::from(d.subsec_nanos() / 100))
                    .saturating_add(116_444_736_000_000_000)
            }
            Err(_) => 0,
        },
        None => 0,
    }
}

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
    // We don't surface real security descriptors. Report zero bytes were written
    // (the JDK treats this as success because the buffer was sufficient).
    Ok(Some(Value::Int(0)))
}

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
    let info = volume_information(&path)
        .ok_or_else(|| InternalError("GetVolumeInformation0: failed".to_string()))?;
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
fn volume_information(path: &str) -> Option<(String, String, u32, u32)> {
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
        return None;
    }
    let vol_len = vol_name
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(vol_name.len());
    let fs_len = fs_name
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(fs_name.len());
    Some((
        String::from_utf16_lossy(&fs_name[..fs_len]),
        String::from_utf16_lossy(&vol_name[..vol_len]),
        serial,
        flags,
    ))
}

#[cfg(not(target_family = "windows"))]
fn volume_information(_path: &str) -> Option<(String, String, u32, u32)> {
    None
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
    let volume = volume_path_name(&path);
    let value = volume.to_object(thread.as_ref()).await?;
    Ok(Some(value))
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn volume_path_name(path: &str) -> String {
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
        return String::new();
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

#[cfg(not(target_family = "windows"))]
fn volume_path_name(_path: &str) -> String {
    String::new()
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.MoveFileEx0(JJI)V", Any)]
#[async_method]
pub async fn move_file_ex0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let new_address = parameters.pop_long()?;
    let existing_address = parameters.pop_long()?;
    let existing = read_native_string(&thread, existing_address, "MoveFileEx0")?;
    let new = read_native_string(&thread, new_address, "MoveFileEx0")?;
    if let Err(e) = std::fs::rename(&existing, &new) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenProcessToken(JI)J", Any)]
#[async_method]
pub async fn open_process_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _desired_access = parameters.pop_int()?;
    let _process = parameters.pop_long()?;
    // Return a sentinel pseudo-token. AccessCheck below ignores it.
    Ok(Some(Value::Long(1)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.OpenThreadToken(JIZ)J", Any)]
#[async_method]
pub async fn open_thread_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _open_as_self = parameters.pop_bool()?;
    let _desired_access = parameters.pop_int()?;
    let _thread = parameters.pop_long()?;
    // No thread-impersonation token. JDK falls back to the process token.
    Ok(Some(Value::Long(0)))
}

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

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V", Any)]
#[async_method]
pub async fn remove_directory0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "RemoveDirectory0")?;
    if let Err(e) = std::fs::remove_dir(&path) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessAllowedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.AddAccessDeniedAceEx(JIIJ)V",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_cancel_io() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cancel_io(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_convert_sid_to_string_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convert_sid_to_string_sid(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.ConvertSidToStringSid(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.CreateEvent(ZZ)J",
            result.unwrap_err().to_string()
        );
    }

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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.DeviceIoControlGetReparsePoint(JJI)V",
            result.unwrap_err().to_string()
        );
    }

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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindFirstFile1(JJ)J",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_find_next_stream() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_find_next_stream0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_next_stream0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.FindNextStream0(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
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
        let result = get_ace(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J",
            result.unwrap_err().to_string()
        );
    }

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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetDiskFreeSpace0(JLsun/nio/fs/WindowsNativeDispatcher$DiskFreeSpace;)V",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_get_drive_type0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_drive_type0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_file_attributes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_attributes0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I",
            result.unwrap_err().to_string()
        );
    }

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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_get_length_sid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_length_sid(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLengthSid(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_logical_drives() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_logical_drives(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I",
            result.unwrap_err().to_string()
        );
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
            "sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I",
            result.unwrap_err().to_string()
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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.GetQueuedCompletionStatus0(JLsun/nio/fs/WindowsNativeDispatcher$CompletionStatus;)V",
            result.unwrap_err().to_string()
        );
    }

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
        let result =
            initialize_acl(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.InitializeAcl(JI)V",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_local_free() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = local_free(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V",
            result.unwrap_err().to_string()
        );
    }

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

    #[tokio::test]
    async fn test_lookup_privilege_value0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_privilege_value0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J",
            result.unwrap_err().to_string()
        );
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
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetFileAttributes0(JI)V",
            result.unwrap_err().to_string()
        );
    }

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
        .await;
        assert_eq!(
            "sun/nio/fs/WindowsNativeDispatcher.SetSecurityDescriptorDacl(JJ)V",
            result.unwrap_err().to_string()
        );
    }

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
}
