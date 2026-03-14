use crate::sun::nio::fs::managed_files;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
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
fn read_native_string<T: ristretto_types::Thread + 'static>(
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
pub async fn close_handle<T: ristretto_types::Thread + 'static>(
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
pub async fn create_file_0<T: ristretto_types::Thread + 'static>(
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
pub async fn find_close<T: ristretto_types::Thread + 'static>(
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
pub async fn find_first_file_0<T: ristretto_types::Thread + 'static>(
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
pub async fn get_file_attributes_ex_0<T: ristretto_types::Thread + 'static>(
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
pub async fn get_full_path_name_0<T: ristretto_types::Thread + 'static>(
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
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
}
