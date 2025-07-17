use crate::Error::InternalError;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use crate::JavaError::RuntimeException;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use async_recursion::async_recursion;
use bitflags::bitflags;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use filetime::{FileTime, set_file_mtime};
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use sysinfo::Disks;

bitflags! {
    /// Boolean Attribute Flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BooleanAttributeFlags: i32 {
        const EXISTS = 0x01;
        const REGULAR = 0x02;
        const DIRECTORY = 0x04;
        const HIDDEN = 0x08;
    }
}

#[intrinsic_method(
    "java/io/UnixFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn canonicalize_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path: String = parameters.pop_object()?.try_into()?;
    let canonical_path: String;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        // In WebAssembly, we cannot access the filesystem directly, so we return the path as is.
        canonical_path = path;
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let path = PathBuf::from(&path);
        let canonicalized_path = path.canonicalize()?;
        canonical_path = canonicalized_path.to_string_lossy().to_string();
    }

    let canonical = canonical_path.to_object(&thread).await?;
    Ok(Some(canonical))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.checkAccess(Ljava/io/File;I)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_access(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    check_access_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.checkAccess0(Ljava/io/File;I)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_access_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let access_mode: i32 = parameters.pop_int()?;
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = Path::new(&path);

    let Ok(metadata) = path.metadata() else {
        return Ok(Some(Value::from(false)));
    };

    let readonly = metadata.permissions().readonly();
    // Access mode bits: 1=read, 2=write, 4=execute
    let can_read = access_mode & 1 == 0 || !readonly;
    let can_write = access_mode & 2 == 0 || !readonly;
    #[cfg(unix)]
    let can_execute = access_mode & 4 == 0 || { metadata.permissions().mode() & 0o111 != 0 };
    #[cfg(not(unix))]
    let can_execute = true; // Not supported on non-Unix

    let allowed = (access_mode & 1 == 0 || can_read)
        && (access_mode & 2 == 0 || can_write)
        && (access_mode & 4 == 0 || can_execute);

    Ok(Some(Value::from(allowed)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.createDirectory(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_directory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    create_directory_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.createDirectory0(Ljava/io/File;)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_directory_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let created: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        created = false;
    }

    #[cfg(target_os = "wasi")]
    {
        let path = PathBuf::from(&path);
        created = std::fs::create_dir(&path).is_ok();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let path = PathBuf::from(&path);
        created = tokio::fs::create_dir(&path).await.is_ok();
    }

    Ok(Some(Value::from(created)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.createFileExclusively(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_file_exclusively(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    create_file_exclusively_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.createFileExclusively0(Ljava/lang/String;)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_file_exclusively_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path: String = parameters.pop_object()?.try_into()?;
    let created: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        created = false;
    }

    #[cfg(target_os = "wasi")]
    {
        created = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .is_ok();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        created = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .is_ok();
    }

    Ok(Some(Value::from(created)))
}

#[intrinsic_method("java/io/UnixFileSystem.delete0(Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn delete_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let deleted: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        deleted = false;
    }

    #[cfg(target_os = "wasi")]
    {
        let path = PathBuf::from(&path);
        deleted = std::fs::remove_file(&path).is_ok();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let path = PathBuf::from(&path);
        deleted = tokio::fs::remove_file(&path).await.is_ok();
    }

    Ok(Some(Value::from(deleted)))
}

#[intrinsic_method("java/io/UnixFileSystem.getBooleanAttributes0(Ljava/io/File;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean_attributes_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(Reference::Object(file)) = parameters.pop_reference()? else {
        return Err(InternalError(
            "getBooleanAttributes0: expected file parameter".to_string(),
        ));
    };
    let path: String = file.value("path")?.try_into()?;
    let path = PathBuf::from(path);
    let mut attributes = if path.exists() {
        BooleanAttributeFlags::EXISTS
    } else {
        BooleanAttributeFlags::empty()
    };
    if path.is_file() {
        attributes |= BooleanAttributeFlags::REGULAR;
    }
    if path.is_dir() {
        attributes |= BooleanAttributeFlags::DIRECTORY;
    }
    if path
        .file_name()
        .map(|name| name.to_string_lossy().starts_with('.'))
        == Some(true)
    {
        attributes |= BooleanAttributeFlags::HIDDEN;
    }
    Ok(Some(Value::Int(attributes.bits())))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getLastModifiedTime(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_last_modified_time(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_last_modified_time_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getLastModifiedTime0(Ljava/io/File;)J",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_last_modified_time_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let last_modified: i64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        last_modified = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        let path = PathBuf::from(&path);
        let metadata = std::fs::metadata(&path)?;
        last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| RuntimeException(error.to_string()))?
            .as_millis() as i64;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let path = PathBuf::from(&path);
        let metadata = tokio::fs::metadata(&path).await?;
        last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| RuntimeException(error.to_string()))?
            .as_millis() as i64;
    }

    Ok(Some(Value::Long(last_modified)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getLength(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_length(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_length_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getLength0(Ljava/io/File;)J",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_length_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let length: i64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        length = 0;
    }

    #[cfg(target_os = "wasi")]
    {
        let path = PathBuf::from(&path);
        let metadata = std::fs::metadata(&path)?;
        length = metadata.len() as i64;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let path = PathBuf::from(&path);
        let metadata = tokio::fs::metadata(&path).await?;
        length = metadata.len() as i64;
    }

    Ok(Some(Value::Long(length)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getNameMax0(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_max_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path: String = parameters.pop_object()?.try_into()?;

    // The default on windows is 255 characters for the maximum filename length, but this can be
    // extended to 32,767 characters when long paths are enabled.
    // https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation
    let maximum_name_length = 255;

    Ok(Some(Value::Long(maximum_name_length)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getSpace(Ljava/io/File;I)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_space(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_space_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.getSpace0(Ljava/io/File;I)J",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_space_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let space_type: i32 = parameters.pop_int()?;
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = PathBuf::from(path);
    let result: i64;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = space_type;
        let _ = path;
        result = 0;
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let disks = Disks::new_with_refreshed_list();
        let disk = disks
            .iter()
            .find(|d| path.starts_with(d.mount_point()))
            .or_else(|| disks.iter().find(|d| d.mount_point() == PathBuf::from("/")));

        result = if let Some(disk) = disk {
            match space_type {
                0 => disk.total_space() as i64,     // total
                1 => disk.available_space() as i64, // free
                2 => disk.available_space() as i64, // usable
                _ => 0,
            }
        } else {
            0
        };
    }

    Ok(Some(Value::Long(result)))
}

#[intrinsic_method("java/io/UnixFileSystem.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/io/UnixFileSystem.list(Ljava/io/File;)[Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn list(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    list_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn list_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = std::path::PathBuf::from(path);
    let entries;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        entries = Vec::<Value>::new();
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let mut path_entries = Vec::new();

        #[cfg(target_os = "wasi")]
        {
            let read_directory = match std::fs::read_dir(&path) {
                Ok(directory) => directory,
                Err(_) => return Ok(None),
            };

            for entry in read_directory {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        let entry_name = name.to_string().to_object(&thread).await?;
                        path_entries.push(entry_name);
                    }
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let read_directory = match tokio::fs::read_dir(&path).await {
                Ok(directory) => directory,
                Err(_) => return Ok(None),
            };

            let mut directory = read_directory;
            while let Ok(Some(entry)) = directory.next_entry().await {
                if let Some(name) = entry.file_name().to_str() {
                    let entry_name = name.to_string().to_object(&thread).await?;
                    path_entries.push(entry_name);
                }
            }
        }

        entries = path_entries;
    }

    let class = thread.class("java.lang.String").await?;
    let paths = Value::try_from((class, entries))?;
    Ok(Some(paths))
}

#[intrinsic_method("java/io/UnixFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rename_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let destination_file = parameters.pop_object()?;
    let source_file = parameters.pop_object()?;
    let source_path: String = source_file.value("path")?.try_into()?;
    let destination_path: String = destination_file.value("path")?.try_into()?;
    let source = std::path::PathBuf::from(source_path);
    let destination = std::path::PathBuf::from(destination_path);
    let success: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = source;
        let _ = destination;
        success = false;
    }

    #[cfg(target_os = "wasi")]
    {
        success = std::fs::rename(&source, &destination).is_ok();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        success = tokio::fs::rename(&source, &destination).await.is_ok();
    }

    Ok(Some(Value::from(success)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setLastModifiedTime(Ljava/io/File;J)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_last_modified_time(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_last_modified_time_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_last_modified_time_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let time = parameters.pop_long()?;
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = PathBuf::from(path);
    let modified: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = time;
        let _ = path;
        modified = false;
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let seconds = time.saturating_div(1000);
        let nanoseconds = (time % 1000).saturating_mul(1_000_000) as u32;
        let mtime = FileTime::from_unix_time(seconds, nanoseconds);
        modified = set_file_mtime(&path, mtime).is_ok();
    }

    Ok(Some(Value::from(modified)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setPermission(Ljava/io/File;IZZ)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_permission(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_permission_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setPermission0(Ljava/io/File;IZZ)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_permission_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let owner_only: bool = parameters.pop_bool()?;
    let enable: bool = parameters.pop_bool()?;
    let access: i32 = parameters.pop_int()?;
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = std::path::PathBuf::from(path);
    let modified: bool;

    #[cfg(not(unix))]
    {
        let _ = owner_only;
        let _ = enable;
        let _ = access;
        let _ = path;
        modified = false;
    }

    #[cfg(unix)]
    {
        let metadata = tokio::fs::metadata(&path).await?;
        let mut permissions = metadata.permissions();
        let mut mode = permissions.mode();

        let (read_bit, write_bit, execute_bit) = if owner_only {
            (0o400, 0o200, 0o100)
        } else {
            (0o444, 0o222, 0o111)
        };

        match access {
            0 => {
                // read
                if enable {
                    mode |= read_bit;
                } else {
                    mode &= !read_bit;
                }
            }
            1 => {
                // write
                if enable {
                    mode |= write_bit;
                } else {
                    mode &= !write_bit;
                }
            }
            2 => {
                // execute
                if enable {
                    mode |= execute_bit;
                } else {
                    mode &= !execute_bit;
                }
            }
            _ => return Ok(Some(Value::from(false))),
        }

        permissions.set_mode(mode);
        modified = tokio::fs::set_permissions(&path, permissions).await.is_ok();
    }

    Ok(Some(Value::from(modified)))
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setReadOnly(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_read_only(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_read_only_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/UnixFileSystem.setReadOnly0(Ljava/io/File;)Z",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_read_only_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop_object()?;
    let path: String = file.value("path")?.try_into()?;
    let path = std::path::PathBuf::from(path);
    let modified: bool;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = path;
        modified = false;
    }

    #[cfg(target_os = "wasi")]
    {
        let metadata = std::fs::metadata(&path)?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        modified = std::fs::set_permissions(&path, permissions).is_ok();
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let metadata = tokio::fs::metadata(&path).await?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        modified = tokio::fs::set_permissions(&path, permissions).await.is_ok();
    }

    Ok(Some(Value::from(modified)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    async fn create_file(thread: &Thread, prefix: &str) -> Result<(NamedTempFile, Value)> {
        let file = NamedTempFile::with_prefix(prefix)?;
        let path_name = file.path().to_string_lossy().to_string();
        let path_name: Value = path_name.to_object(thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_name])
            .await?;
        Ok((file, file_object))
    }

    #[tokio::test]
    async fn test_canonicalize_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let original_path = ".";
        let path = original_path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path);
        let value = canonicalize_0(thread, parameters).await?.expect("value");
        let canonical_path: String = value.try_into()?;
        assert_ne!(original_path, canonical_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_access() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "check_access").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_int(1); // Read access
        let value = check_access(thread, parameters).await?.expect("access");
        let has_access: bool = value.try_into()?;
        assert!(has_access);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_access_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "check_access_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_int(1); // Read access
        let value = check_access_0(thread, parameters).await?.expect("access");
        let has_access: bool = value.try_into()?;
        assert!(has_access);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_directory() -> Result<()> {
        let path_name = "test_create_directory";
        if Path::new(path_name).exists() {
            tokio::fs::remove_dir_all(path_name).await?;
        }
        let (_vm, thread) = crate::test::thread().await?;
        let path_object: Value = path_name.to_object(&thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = create_directory(thread, parameters)
            .await?
            .expect("created");
        let created: bool = value.try_into()?;
        assert!(created);
        tokio::fs::remove_dir_all(path_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_directory_0() -> Result<()> {
        let path_name = "test_create_directory_0";
        if Path::new(path_name).exists() {
            tokio::fs::remove_dir_all(path_name).await?;
        }
        let (_vm, thread) = crate::test::thread().await?;
        let path_object: Value = path_name.to_object(&thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = create_directory_0(thread, parameters)
            .await?
            .expect("created");
        let created: bool = value.try_into()?;
        assert!(created);
        tokio::fs::remove_dir_all(path_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_file_exclusively() -> Result<()> {
        let path_name = "test_create_file_exclusively";
        if Path::new(path_name).exists() {
            tokio::fs::remove_file(path_name).await?;
        }

        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path_object: Value = path_name.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_object);
        let value = create_file_exclusively(thread, parameters)
            .await?
            .expect("created");
        let created: bool = value.try_into()?;
        assert!(created);
        tokio::fs::remove_file(path_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_file_exclusively_0() -> Result<()> {
        let path_name = "test_create_file_exclusively_0";
        if Path::new(path_name).exists() {
            tokio::fs::remove_file(path_name).await?;
        }

        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path_object: Value = path_name.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_object);
        let value = create_file_exclusively_0(thread, parameters)
            .await?
            .expect("created");
        let created: bool = value.try_into()?;
        assert!(created);
        tokio::fs::remove_file(path_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (file, file_object) = create_file(&thread, "delete").await?;
        let path: String = file.path().to_string_lossy().to_string();
        assert!(Path::new(&path).exists());

        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = delete_0(thread, parameters).await?.expect("deleted");
        let deleted: bool = value.try_into()?;
        assert!(deleted);
        assert!(!Path::new(&path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_attributes_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "get_boolean_attributes_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_boolean_attributes_0(thread, parameters)
            .await?
            .expect("attributes");
        let attributes: i32 = value.try_into()?;
        assert!(attributes > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_last_modified_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| RuntimeException(e.to_string()))?
            .as_millis() as i64;
        let (_file, file_object) = create_file(&thread, "get_last_modified_time").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_last_modified_time(thread, parameters)
            .await?
            .expect("last modified time");
        let last_modified_time: i64 = value.try_into()?;
        assert!(last_modified_time >= start_time);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_last_modified_time_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| RuntimeException(e.to_string()))?
            .as_millis() as i64;
        let (_file, file_object) = create_file(&thread, "get_last_modified_time_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_last_modified_time_0(thread, parameters)
            .await?
            .expect("last modified time");
        let last_modified_time: i64 = value.try_into()?;
        assert!(last_modified_time >= start_time);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "get_length").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_length(thread, parameters).await?.expect("length");
        let length: i64 = value.try_into()?;
        assert_eq!(0, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "get_length_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_length_0(thread, parameters).await?.expect("length");
        let length: i64 = value.try_into()?;
        assert_eq!(0, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_max_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path = "get_name_max_0.txt".to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path);
        let value = get_name_max_0(thread, parameters).await?.expect("name max");
        let length: i64 = value.try_into()?;
        assert_eq!(255, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_space() -> Result<()> {
        for space_type in [0, 1, 2, 3] {
            let (_vm, thread) = crate::test::thread().await?;
            let (_file, file_object) = create_file(&thread, "get_space").await?;
            let mut parameters = Parameters::default();
            parameters.push(file_object);
            parameters.push_int(space_type);
            let value = get_space(thread, parameters).await?.expect("space");
            let space: i64 = value.try_into()?;

            if space_type > 2 {
                assert_eq!(space, 0);
            } else {
                assert!(space > 0);
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_space_0() -> Result<()> {
        for space_type in [0, 1, 2, 3] {
            let (_vm, thread) = crate::test::thread().await?;
            let (_file, file_object) = create_file(&thread, "get_space_0").await?;
            let mut parameters = Parameters::default();
            parameters.push(file_object);
            parameters.push_int(space_type);
            let value = get_space_0(thread, parameters).await?.expect("space");
            let space: i64 = value.try_into()?;

            if space_type > 2 {
                assert_eq!(space, 0);
            } else {
                assert!(space > 0);
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let path_object: Value = ".".to_object(&thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = list(thread, parameters).await?.expect("paths");
        let reference = value.to_reference()?.expect("paths");
        let class_name = reference.class_name().to_string();
        let elements: Vec<Value> = reference.try_into()?;
        assert_eq!(class_name, "java/lang/String");
        assert!(!elements.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_list_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let path_object: Value = ".".to_object(&thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = list_0(thread, parameters).await?.expect("paths");
        let reference = value.to_reference()?.expect("paths");
        let class_name = reference.class_name().to_string();
        let elements: Vec<Value> = value.try_into()?;
        assert_eq!(class_name, "java/lang/String");
        assert!(!elements.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_rename_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (source_file, source_object) = create_file(&thread, "rename_source").await?;
        let source_path: String = source_file.path().to_string_lossy().to_string();
        let destination_path = format!("{source_path}_destination");
        let destination_object: Value = destination_path.to_object(&thread).await?;
        let destination_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[destination_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(source_object);
        parameters.push(destination_object);
        let value = rename_0(thread, parameters).await?.expect("renamed");
        let renamed: bool = value.try_into()?;
        assert!(renamed);
        assert!(!Path::new(&source_path).exists());
        assert!(Path::new(&destination_path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_last_modified_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_last_modified_time").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_long(0);
        let value = set_last_modified_time(thread, parameters)
            .await?
            .expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_last_modified_time_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_last_modified_time").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_long(0);
        let value = set_last_modified_time_0(thread, parameters)
            .await?
            .expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_permission() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_permission").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_bool(false); // owner_only
        parameters.push_bool(true); // enable
        parameters.push_int(1); // access (write)
        let value = set_permission(thread, parameters).await?.expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_permission_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_permission_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_bool(false); // owner_only
        parameters.push_bool(true); // enable
        parameters.push_int(1); // access (write)
        let value = set_permission_0(thread, parameters)
            .await?
            .expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_read_only() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_read_only").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = set_read_only(thread, parameters).await?.expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_read_only_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "set_read_only_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = set_read_only_0(thread, parameters).await?.expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }
}
