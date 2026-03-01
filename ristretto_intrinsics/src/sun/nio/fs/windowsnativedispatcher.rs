#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_possible_wrap)]

use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Read a null-terminated UTF-16LE string from native memory at the given address.
fn read_wstring<V: VM>(vm: &V, address: i64) -> String {
    let mut chars = Vec::new();
    let mut offset = address;
    loop {
        let bytes = vm.native_memory().read_bytes(offset, 2);
        let wchar = u16::from_le_bytes([bytes[0], bytes[1]]);
        if wchar == 0 {
            break;
        }
        chars.push(wchar);
        offset += 2;
    }
    String::from_utf16_lossy(&chars)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributesEx0(JJ)V", Any)]
#[async_method]
pub async fn get_file_attributes_ex_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attr_buf_address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);

    let metadata = std::fs::metadata(&path).map_err(|_| {
        ristretto_types::Error::InternalError(format!("GetFileAttributesEx0 failed for: {path}"))
    })?;

    // WIN32_FILE_ATTRIBUTE_DATA layout (36 bytes):
    // offset 0:  DWORD dwFileAttributes (4 bytes)
    // offset 4:  FILETIME ftCreationTime (8 bytes)
    // offset 12: FILETIME ftLastAccessTime (8 bytes)
    // offset 20: FILETIME ftLastWriteTime (8 bytes)
    // offset 28: DWORD nFileSizeHigh (4 bytes)
    // offset 32: DWORD nFileSizeLow (4 bytes)
    let mut buf = [0u8; 36];

    let mut attrs: u32 = 0;
    if metadata.is_dir() {
        attrs |= 0x10; // FILE_ATTRIBUTE_DIRECTORY
    }
    if metadata.is_symlink() {
        attrs |= 0x400; // FILE_ATTRIBUTE_REPARSE_POINT
    }
    if attrs == 0 {
        attrs = 0x80; // FILE_ATTRIBUTE_NORMAL
    }
    buf[0..4].copy_from_slice(&attrs.to_le_bytes());

    let file_size = metadata.len();
    let size_high = (file_size >> 32) as u32;
    let size_low = file_size as u32;
    buf[28..32].copy_from_slice(&size_high.to_le_bytes());
    buf[32..36].copy_from_slice(&size_low.to_le_bytes());

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        let creation = metadata.creation_time();
        buf[4..12].copy_from_slice(&creation.to_le_bytes());
        let last_access = metadata.last_access_time();
        buf[12..20].copy_from_slice(&last_access.to_le_bytes());
        let last_write = metadata.last_write_time();
        buf[20..28].copy_from_slice(&last_write.to_le_bytes());
    }

    vm.native_memory().write_bytes(attr_buf_address, &buf);
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileAttributes0(J)I", Any)]
#[async_method]
pub async fn get_file_attributes_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);

    let Ok(metadata) = std::fs::metadata(&path) else {
        // INVALID_FILE_ATTRIBUTES
        return Ok(Some(Value::Int(-1)));
    };

    let mut attrs: u32 = 0;
    if metadata.is_dir() {
        attrs |= 0x10; // FILE_ATTRIBUTE_DIRECTORY
    }
    if attrs == 0 {
        attrs = 0x80; // FILE_ATTRIBUTE_NORMAL
    }
    Ok(Some(Value::Int(attrs as i32)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateFile0(JIIJII)J", Any)]
#[async_method]
pub async fn create_file_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags_and_attrs = parameters.pop_int()?;
    let _create_disposition = parameters.pop_int()?;
    let _security_attrs = parameters.pop_long()?;
    let _share_mode = parameters.pop_int()?;
    let _desired_access = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);

    // Use managed_files to handle the file
    let handle = super::managed_files::open_windows(vm.nio_file_handles(), &path).await;

    match handle {
        Ok(fd) => Ok(Some(Value::Long(i64::from(fd)))),
        Err(_) => Err(ristretto_types::Error::InternalError(format!(
            "CreateFile0 failed for: {path}"
        ))),
    }
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CloseHandle(J)V", Any)]
#[async_method]
pub async fn close_handle<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    super::managed_files::close(vm.nio_file_handles(), handle as i32).await;
    Ok(None)
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
    use ristretto_types::JavaObject;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);
    let full_path = std::fs::canonicalize(&path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(path);
    // Remove \\?\ prefix on Windows
    let clean_path = full_path
        .strip_prefix(r"\\?\")
        .unwrap_or(&full_path)
        .to_string();
    let result = clean_path.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.FormatMessage(I)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_message<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use ristretto_types::JavaObject;
    let error_code = parameters.pop_int()?;
    let msg = format!("Windows error {error_code}");
    let result = msg.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.LocalFree(J)V", Any)]
#[async_method]
pub async fn local_free<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetLogicalDrives()I", Any)]
#[async_method]
pub async fn get_logical_drives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Bit mask of available drives. Bit 0 = A:, Bit 2 = C:, etc.
    // Return C: drive as available by default
    Ok(Some(Value::Int(4))) // bit 2 = C:
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetFileSizeEx(J)J", Any)]
#[async_method]
pub async fn get_file_size_ex<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let size = super::managed_files::file_size(vm.nio_file_handles(), handle as i32)
        .await
        .unwrap_or(0);
    Ok(Some(Value::Long(size)))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.SetFileTime0(JJJJ)V", Any)]
#[async_method]
pub async fn set_file_time_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetVolumePathName0(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_volume_path_name_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use ristretto_types::JavaObject;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);
    // Return the drive root
    let root = if path.len() >= 3 && path.as_bytes()[1] == b':' {
        format!("{}\\", &path[..2])
    } else {
        "C:\\".to_string()
    };
    let result = root.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.GetDriveType0(J)I", Any)]
#[async_method]
pub async fn get_drive_type_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // DRIVE_FIXED = 3
    Ok(Some(Value::Int(3)))
}

#[intrinsic_method(
    "sun/nio/fs/WindowsNativeDispatcher.GetFinalPathNameByHandle(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_final_path_name_by_handle<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use ristretto_types::JavaObject;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = super::managed_files::get_path(vm.nio_file_handles(), handle as i32);
    let result = path.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.DeleteFile0(J)V", Any)]
#[async_method]
pub async fn delete_file_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);
    let _ = std::fs::remove_file(&path);
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.CreateDirectory0(JJ)V", Any)]
#[async_method]
pub async fn create_directory_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _security_attrs = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);
    let _ = std::fs::create_dir(&path);
    Ok(None)
}

#[intrinsic_method("sun/nio/fs/WindowsNativeDispatcher.RemoveDirectory0(J)V", Any)]
#[async_method]
pub async fn remove_directory_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path = read_wstring(&*vm, path_address);
    let _ = std::fs::remove_dir(&path);
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
