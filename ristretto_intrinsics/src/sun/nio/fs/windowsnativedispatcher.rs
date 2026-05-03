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
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock};

/// Directory enumeration state retained across `FindFirstFile`/`FindNextFile` invocations.
struct FindFileState {
    parent: std::path::PathBuf,
    remaining: VecDeque<String>,
}

fn find_file_handles() -> &'static Mutex<HashMap<i64, FindFileState>> {
    static HANDLES: OnceLock<Mutex<HashMap<i64, FindFileState>>> = OnceLock::new();
    HANDLES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_find_file_handle() -> i64 {
    use std::sync::atomic::{AtomicI64, Ordering};
    static COUNTER: AtomicI64 = AtomicI64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// I/O completion port state used to back JDK's `WindowsWatchService` IOCP loop.
///
/// JDK calls `CreateIoCompletionPort`, posts wakeups via `PostQueuedCompletionStatus`, and the
/// poller thread blocks in `GetQueuedCompletionStatus0`. We implement a minimal in-process queue
/// that supports the wakeup/close handshake; no real directory-change events are produced.
struct IocpPort {
    tx: tokio::sync::mpsc::UnboundedSender<IocpCompletion>,
    rx: tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<IocpCompletion>>,
}

#[derive(Clone, Copy)]
struct IocpCompletion {
    error: i32,
    bytes_transferred: i32,
    completion_key: i32,
    overlapped: i64,
}

fn iocp_ports() -> &'static Mutex<HashMap<i64, Arc<IocpPort>>> {
    static PORTS: OnceLock<Mutex<HashMap<i64, Arc<IocpPort>>>> = OnceLock::new();
    PORTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_iocp_port_handle() -> i64 {
    use std::sync::atomic::{AtomicI64, Ordering};
    static COUNTER: AtomicI64 = AtomicI64::new(0x1000_0000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn next_iocp_event_handle() -> i64 {
    use std::sync::atomic::{AtomicI64, Ordering};
    static COUNTER: AtomicI64 = AtomicI64::new(0x2000_0000);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Splits a path of the form `dir\*` (or similar wildcard) into the directory portion.
fn split_search_pattern(path: &str) -> (String, String) {
    // Strip trailing wildcards (`*`, `*.*`) and the separator that precedes them.
    let trimmed = path.trim_end_matches(['*', '?']);
    let trimmed = trimmed.trim_end_matches(['\\', '/']);
    let dir = if trimmed.is_empty() {
        path.to_string()
    } else {
        trimmed.to_string()
    };
    let pattern = path[dir.len()..]
        .trim_start_matches(['\\', '/'])
        .to_string();
    (dir, pattern)
}

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
    pub const READONLY: i32 = 0x01;
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
    let handle = parameters.pop_long()?;
    if let Ok(mut handles) = find_file_handles().lock() {
        handles.remove(&handle);
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
    let path_str = read_native_string(&thread, path_address, "FindFirstFile0")?;

    let (dir, pattern) = split_search_pattern(&path_str);
    let dir_path = std::path::Path::new(&dir);
    let is_enumeration = !pattern.is_empty();

    // Mirror the Win32 `FindFirstFile` contract: when called with a wildcard pattern (e.g.
    // `dir\*`) we enumerate directory contents; when called with a concrete path we return
    // a single entry describing that path itself (used by `WindowsLinkSupport.getRealPath`
    // and similar callers).
    let mut entries: VecDeque<String> = VecDeque::new();
    if is_enumeration && dir_path.is_dir() {
        // Win32 `FindFirstFile` always returns `.` and `..` as the first two entries; mirror that
        // so JDK's `WindowsDirectoryStream` can correctly filter them. Without these synthetic
        // entries, an empty directory returns an empty name that JDK interprets as a real child
        // and the walker recurses into the same directory forever.
        entries.push_back(".".to_string());
        entries.push_back("..".to_string());
        match std::fs::read_dir(dir_path) {
            Ok(read_dir) => {
                for entry in read_dir.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push_back(name.to_string());
                    }
                }
            }
            Err(error) => {
                let code = windows_error_code(&error);
                return Err(throw_windows_exception(&thread, code).await);
            }
        }
    } else {
        match std::fs::metadata(dir_path) {
            Ok(_) => {
                if let Some(name) = dir_path.file_name().and_then(|n| n.to_str()) {
                    entries.push_back(name.to_string());
                } else {
                    entries.push_back(dir.clone());
                }
            }
            Err(error) => {
                let code = windows_error_code(&error);
                return Err(throw_windows_exception(&thread, code).await);
            }
        }
    }

    let first = entries.pop_front().unwrap_or_default();
    let first_path = if !is_enumeration || first.is_empty() {
        dir_path.to_path_buf()
    } else {
        dir_path.join(&first)
    };
    let attributes = match std::fs::symlink_metadata(&first_path) {
        Ok(meta) if meta.is_dir() => file_attribute::DIRECTORY,
        _ => file_attribute::NORMAL,
    };

    let handle = next_find_file_handle();
    if let Ok(mut handles) = find_file_handles().lock() {
        handles.insert(
            handle,
            FindFileState {
                parent: dir_path.to_path_buf(),
                remaining: entries,
            },
        );
    }

    let name_value = thread.intern_string(&first).await?;

    let mut guard = first_file_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError("FindFirstFile0: not an object".to_string()));
    };
    obj.set_value("handle", Value::Long(handle))?;
    obj.set_value("name", name_value)?;
    obj.set_value("attributes", Value::Int(attributes))?;
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
    Ok(None)
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
    Ok(None)
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
    // Privilege adjustment is a no-op: we do not impersonate Windows token state.
    Ok(None)
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
    let obj_gc = parameters.pop_reference()?;
    let _context_address = parameters.pop_long()?;
    let _abort = parameters.pop_bool()?;
    let _bytes_to_read = parameters.pop_int()?;
    let _buffer_address = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    // Report end-of-stream so callers do not loop indefinitely.
    if let Some(obj_gc) = obj_gc {
        let mut guard = obj_gc.write();
        if let Reference::Object(ref mut obj) = *guard {
            obj.set_value("bytesTransferred", Value::Int(0))?;
            obj.set_value("context", Value::Long(0))?;
        }
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.BackupSeek(JJJ)V", Equal(JAVA_8))]
#[async_method]
pub async fn backup_seek<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context_address = parameters.pop_long()?;
    let _bytes_to_skip = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CancelIo(J)V", Any)]
#[async_method]
pub async fn cancel_io<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_file = parameters.pop_long()?;
    // No outstanding asynchronous I/O is tracked in this stub.
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
    let _address = parameters.pop_long()?;
    // We don't track real SIDs; surface the well-known null SID.
    let value = thread.intern_string("S-1-0-0").await?;
    Ok(Some(value))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.ConvertStringSidToSid0(J)J", Any)]
#[async_method]
pub async fn convert_string_sid_to_sid0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Ok(Some(Value::Long(1)))
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
    // We don't implement real Win32 events; return a unique non-zero pseudo-handle so the JDK's
    // overlapped-I/O wrapper has something to store. `WatchService` only uses the value to detect
    // `INVALID_HANDLE_VALUE` and to call `CloseHandle` later.
    Ok(Some(Value::Long(next_iocp_event_handle())))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateHardLink0(JJ)V", Any)]
#[async_method]
pub async fn create_hard_link0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let existing_file_address = parameters.pop_long()?;
    let new_file_address = parameters.pop_long()?;
    let existing = read_native_string(&thread, existing_file_address, "CreateHardLink0")?;
    let new = read_native_string(&thread, new_file_address, "CreateHardLink0")?;
    if let Err(e) = std::fs::hard_link(&existing, &new) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateIoCompletionPort(JJJ)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_long()?;
    let existing_port = parameters.pop_long()?;
    let _file_handle = parameters.pop_long()?;
    if existing_port == 0 {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let port = Arc::new(IocpPort {
            tx,
            rx: tokio::sync::Mutex::new(rx),
        });
        let handle = next_iocp_port_handle();
        if let Ok(mut ports) = iocp_ports().lock() {
            ports.insert(handle, port);
        }
        Ok(Some(Value::Long(handle)))
    } else {
        // Associating a handle with an existing port - we don't track per-handle state.
        Ok(Some(Value::Long(existing_port)))
    }
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
    let link = read_native_string(&thread, link_address, "CreateSymbolicLink0")?;
    let target = read_native_string(&thread, target_address, "CreateSymbolicLink0")?;
    let is_dir = (flags & 0x1) != 0;
    let result = create_symlink(&link, &target, is_dir);
    if let Err(e) = result {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
}

#[cfg(target_family = "windows")]
fn create_symlink(link: &str, target: &str, is_dir: bool) -> std::io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};
    if is_dir {
        symlink_dir(target, link)
    } else {
        symlink_file(target, link)
    }
}

#[cfg(not(target_family = "windows"))]
fn create_symlink(link: &str, target: &str, _is_dir: bool) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link)
    }
    #[cfg(not(unix))]
    {
        let _ = (link, target);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "symlinks not supported",
        ))
    }
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let buffer_size = parameters.pop_int()?;
    let buffer_address = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    match read_reparse_point(vm.file_handles(), handle, buffer_size).await {
        Ok(bytes) => {
            let native_memory = vm.native_memory();
            native_memory.write_bytes(buffer_address, &bytes);
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    // Best-effort no-op: we don't mark files sparse, but allow the call to succeed so
    // higher-level code (Files.createFile with SparseFileOption) continues without error.
    Ok(None)
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data_address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let path = read_native_string(&thread, path_address, "FindFirstFile1")?;
    let entry_path = std::path::PathBuf::from(&path);
    if std::fs::symlink_metadata(&entry_path).is_err() {
        return Err(throw_windows_exception(&thread, 2 /* ERROR_FILE_NOT_FOUND */).await);
    }
    write_find_data(&thread, data_address, &entry_path)?;
    Ok(Some(Value::Long(next_find_file_handle())))
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
    let _obj = parameters.pop_reference()?;
    let _address = parameters.pop_long()?;
    // We don't enumerate NTFS alternate data streams; report end-of-stream so JDK
    // callers cleanly observe an empty stream list.
    Err(throw_windows_exception(&thread, 38 /* ERROR_HANDLE_EOF */).await)
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
    let next = {
        let mut handles = find_file_handles().lock().map_err(|_| {
            InternalError("FindNextFile: poisoned find-file handle map".to_string())
        })?;
        handles.get_mut(&handle).and_then(|state| {
            state
                .remaining
                .pop_front()
                .map(|name| (state.parent.clone(), name))
        })
    };
    match next {
        Some((parent, name)) => {
            let entry_path = parent.join(&name);
            write_find_data(thread, data_address, &entry_path)?;
            let value = thread.intern_string(&name).await?;
            Ok(Some(value))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

/// Write a `WIN32_FIND_DATAW` structure to the supplied native buffer based on the metadata
/// for `entry_path`. The JDK's `WindowsDirectoryStream` reads at least `dwFileAttributes`,
/// timestamps, and file size from this buffer to construct cached `BasicFileAttributes`
/// instances for the entries it returns.
fn write_find_data<T: Thread + 'static>(
    thread: &Arc<T>,
    data_address: i64,
    entry_path: &std::path::Path,
) -> Result<()> {
    if data_address == 0 {
        return Ok(());
    }
    let vm = thread.vm()?;
    let native_memory = vm.native_memory();
    let (attrs, creation, access, write, size) = match std::fs::symlink_metadata(entry_path) {
        Ok(meta) => {
            let attrs = if meta.is_dir() {
                file_attribute::DIRECTORY
            } else {
                file_attribute::NORMAL
            };
            (
                attrs,
                system_time_to_filetime(meta.created().ok()),
                system_time_to_filetime(meta.accessed().ok()),
                system_time_to_filetime(meta.modified().ok()),
                meta.len(),
            )
        }
        Err(_) => (file_attribute::NORMAL, 0, 0, 0, 0),
    };
    native_memory.write_i32(
        data_address + file_attribute_data_offset::FILE_ATTRIBUTES,
        attrs,
    );
    native_memory.write_i64(
        data_address + file_attribute_data_offset::CREATION_TIME,
        i64::from_ne_bytes(creation.to_ne_bytes()),
    );
    native_memory.write_i64(
        data_address + file_attribute_data_offset::LAST_ACCESS_TIME,
        i64::from_ne_bytes(access.to_ne_bytes()),
    );
    native_memory.write_i64(
        data_address + file_attribute_data_offset::LAST_WRITE_TIME,
        i64::from_ne_bytes(write.to_ne_bytes()),
    );
    let size_high = i32::try_from(size >> DWORD_BITS)
        .map_err(|_| InternalError("FindNextFile: size overflow".to_string()))?;
    let size_low = i32::try_from(size & FILE_SIZE_LOW_MASK)
        .map_err(|_| InternalError("FindNextFile: size overflow".to_string()))?;
    native_memory.write_i32(
        data_address + file_attribute_data_offset::FILE_SIZE_HIGH,
        size_high,
    );
    native_memory.write_i32(
        data_address + file_attribute_data_offset::FILE_SIZE_LOW,
        size_low,
    );
    Ok(())
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
    Ok(Some(Value::Object(None)))
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
    Ok(Some(Value::Object(None)))
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
    let s = String::from_utf16_lossy(&buf[..usize::try_from(len).unwrap_or(0)]);
    Some(s.trim_end_matches(['\r', '\n']).to_string())
}

#[cfg(not(target_family = "windows"))]
fn format_win32_message(_code: i32) -> Option<String> {
    None
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetAce(JI)J", Any)]
#[async_method]
pub async fn get_ace<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ace_index = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
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
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetAclInformation0: null obj".to_string()))?;
    let _address = parameters.pop_long()?;
    let mut guard = obj_gc.write();
    if let Reference::Object(ref mut obj) = *guard {
        // Empty DACL: no entries, just the 8-byte ACL header.
        obj.set_value("aceCount", Value::Int(0))?;
        obj.set_value("aclBytesInUse", Value::Int(8))?;
    }
    Ok(None)
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let obj_gc = parameters
        .pop_reference()?
        .ok_or(InternalError("GetDiskFreeSpace0: null obj".to_string()))?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "GetDiskFreeSpace0")?;
    let Some(info) = disk_free_space(&path) else {
        return Err(throw_windows_exception(&thread, 3 /* ERROR_PATH_NOT_FOUND */).await);
    };
    let mut guard = obj_gc.write();
    let Reference::Object(ref mut obj) = *guard else {
        return Err(InternalError(
            "GetDiskFreeSpace0: not an object".to_string(),
        ));
    };
    // DiskFreeSpace fields (Java<11 layout): sectorsPerAllocationUnit, bytesPerSector,
    // totalNumberOfClusters, numberOfFreeClusters (all long).
    obj.set_value(
        "sectorsPerAllocationUnit",
        Value::Long(i64::from(info.sectors_per_cluster)),
    )?;
    obj.set_value(
        "bytesPerSector",
        Value::Long(i64::from(info.bytes_per_sector)),
    )?;
    obj.set_value(
        "totalNumberOfClusters",
        Value::Long(i64::from(info.total_clusters)),
    )?;
    obj.set_value(
        "numberOfFreeClusters",
        Value::Long(i64::from(info.free_clusters)),
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
    let Some(info) = disk_free_space_ex(&path) else {
        return Err(throw_windows_exception(&thread, 3 /* ERROR_PATH_NOT_FOUND */).await);
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
    sectors_per_cluster: u32,
    bytes_per_sector: u32,
    free_clusters: u32,
    total_clusters: u32,
}

struct DiskFreeSpaceExInfo {
    free_to_caller: u64,
    total: u64,
    total_free: u64,
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn disk_free_space(path: &str) -> Option<DiskFreeSpaceInfo> {
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
        return None;
    }
    Some(DiskFreeSpaceInfo {
        sectors_per_cluster,
        bytes_per_sector,
        free_clusters,
        total_clusters,
    })
}

#[cfg(not(target_family = "windows"))]
fn disk_free_space(_path: &str) -> Option<DiskFreeSpaceInfo> {
    None
}

#[cfg(target_family = "windows")]
#[expect(unsafe_code)]
fn disk_free_space_ex(path: &str) -> Option<DiskFreeSpaceExInfo> {
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
        return None;
    }
    Some(DiskFreeSpaceExInfo {
        free_to_caller,
        total,
        total_free,
    })
}

#[cfg(not(target_family = "windows"))]
fn disk_free_space_ex(_path: &str) -> Option<DiskFreeSpaceExInfo> {
    None
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
    let path_str = read_native_string(&thread, path_address, "GetFileAttributes0")?;
    let path = std::path::Path::new(&path_str);
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            let attrs = if metadata.is_dir() {
                file_attribute::DIRECTORY
            } else {
                file_attribute::NORMAL
            };
            Ok(Some(Value::Int(attrs)))
        }
        Err(e) => Err(throw_windows_exception(&thread, windows_error_code(&e)).await),
    }
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
        return Ok(String::from_utf16_lossy(&buf[..needed_usize]));
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Ok(Some(Value::Int(12)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I", Any)]
#[async_method]
pub async fn get_logical_drives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    {
        // Probe drive letters A-Z by attempting to read metadata on the root.
        let mut mask: u32 = 0;
        for i in 0u8..26 {
            let letter = (b'A' + i) as char;
            let root = format!("{letter}:\\");
            if std::fs::metadata(&root).is_ok() {
                mask |= 1u32 << u32::from(i);
            }
        }
        return Ok(Some(Value::Int(mask.cast_signed())));
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Some(Value::Int(1 << 2)))
    }
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetOverlappedResult(JJ)I", Any)]
#[async_method]
pub async fn get_overlapped_result<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_overlapped = parameters.pop_long()?;
    let _h_file = parameters.pop_long()?;
    // No outstanding overlapped I/O in our stub: report zero bytes transferred.
    Ok(Some(Value::Int(0)))
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
    let port = {
        let Ok(ports) = iocp_ports().lock() else {
            return Err(InternalError(
                "GetQueuedCompletionStatus0: iocp_ports poisoned".to_string(),
            ));
        };
        ports.get(&completion_port).cloned()
    };
    let Some(port) = port else {
        return Err(throw_windows_exception(&thread, 6 /* ERROR_INVALID_HANDLE */).await);
    };
    let completion = {
        let mut rx = port.rx.lock().await;
        rx.recv().await.unwrap_or(IocpCompletion {
            error: 6,
            bytes_transferred: 0,
            completion_key: 0,
            overlapped: 0,
        })
    };
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
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
    Ok(Some(Value::Long(0)))
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
    Ok(Some(Value::Int(0)))
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
    Ok(None)
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
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V", Any)]
#[async_method]
pub async fn local_free<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    // No native allocations are tracked here; the call is a safe no-op.
    Ok(None)
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
    Ok(Some(Value::Int(0)))
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
    let _address = parameters.pop_long()?;
    let name = "Unknown".to_object(thread.as_ref()).await?;
    let domain = String::new().to_object(thread.as_ref()).await?;
    let mut guard = obj_gc.write();
    if let Reference::Object(ref mut obj) = *guard {
        obj.set_value("domain", domain)?;
        obj.set_value("name", name)?;
        // SID_NAME_USE::SidTypeUnknown = 8
        obj.set_value("use", Value::Int(8))?;
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LookupPrivilegeValue0(J)J", Any)]
#[async_method]
pub async fn lookup_privilege_value0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_long()?;
    // Return a sentinel LUID. The JDK treats `LookupPrivilegeValue0` failure as
    // an `AssertionError`, so we always succeed; subsequent privilege adjustments
    // are no-ops handled by `adjust_token_privileges`.
    Ok(Some(Value::Long(1)))
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let completion_key = parameters.pop_long()?;
    let completion_port = parameters.pop_long()?;
    let port = {
        let Ok(ports) = iocp_ports().lock() else {
            return Err(InternalError(
                "PostQueuedCompletionStatus: iocp_ports poisoned".to_string(),
            ));
        };
        ports.get(&completion_port).cloned()
    };
    let Some(port) = port else {
        return Err(throw_windows_exception(&thread, 6 /* ERROR_INVALID_HANDLE */).await);
    };
    let _ = port.tx.send(IocpCompletion {
        error: 0,
        bytes_transferred: 0,
        completion_key: i32::try_from(completion_key).unwrap_or(0),
        overlapped: 0,
    });
    Ok(None)
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
    // We don't generate real file-change notifications; the call is treated as a successful
    // arming of the overlapped read. The watcher's `poll` will simply time out.
    Ok(None)
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let path = read_native_string(&thread, address, "SetFileAttributes0")?;
    let metadata = match std::fs::metadata(&path) {
        Ok(meta) => meta,
        Err(e) => return Err(throw_windows_exception(&thread, windows_error_code(&e)).await),
    };
    let mut permissions = metadata.permissions();
    let readonly = (value & file_attribute::READONLY) != 0;
    permissions.set_readonly(readonly);
    if let Err(e) = std::fs::set_permissions(&path, permissions) {
        return Err(throw_windows_exception(&thread, windows_error_code(&e)).await);
    }
    Ok(None)
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
    // Best-effort no-op: we don't apply NT security descriptors. This keeps
    // `Files.copy` with `COPY_ATTRIBUTES` working without surfacing an error.
    Ok(None)
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
    // Best-effort no-op: setting NT file times via raw FILETIME values is unsupported.
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _acl_address = parameters.pop_long()?;
    let _desc_address = parameters.pop_long()?;
    Ok(None)
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
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetThreadToken(JJ)V", Any)]
#[async_method]
pub async fn set_thread_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _token = parameters.pop_long()?;
    let _thread = parameters.pop_long()?;
    // We do not impersonate Windows tokens; ignore the request so callers proceed.
    Ok(None)
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
