//! Shared file system utilities used by `java.io.UnixFileSystem` and
//! `java.io.WinNTFileSystem` intrinsic implementations.
//!
//! Each public helper performs the common parameter parsing, file system access, and result
//! conversion so the per-OS intrinsic modules can simply delegate to the matching helper.

#![allow(clippy::unused_async)]

use bitflags::bitflags;
use filetime::{FileTime, set_file_mtime};
use ristretto_classloader::{Reference, Value};
use ristretto_types::JavaError::RuntimeException;
use ristretto_types::{JavaObject, Parameters, Result, Thread, VM};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sysinfo::Disks;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

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

bitflags! {
    /// File access mode flags per `java.io.FileSystem`.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FileAccessMode: i32 {
        const EXECUTE = 0x01;
        const WRITE = 0x02;
        const READ = 0x04;
    }
}

/// Canonicalize a path, even when it (or some of its ancestors) do not exist.
///
/// This mirrors the behavior of `java.io.File#getCanonicalPath()`, which resolves symlinks for
/// the longest existing prefix and then normalizes the remaining components without requiring
/// them to exist on disk.
pub(crate) fn canonicalize_best_effort(path: &Path) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map_or_else(|_| path.to_path_buf(), |cwd| cwd.join(path))
    };

    let mut existing = absolute.clone();
    let mut trailing: Vec<std::ffi::OsString> = Vec::new();
    while !existing.as_os_str().is_empty() && !existing.exists() {
        let file_name = match existing.file_name() {
            Some(name) => name.to_os_string(),
            None => break,
        };
        trailing.push(file_name);
        if !existing.pop() {
            break;
        }
    }

    let base = existing.canonicalize().unwrap_or_else(|_| existing.clone());
    let mut result = base;
    for component in trailing.into_iter().rev() {
        if component == std::ffi::OsStr::new("..") {
            result.pop();
        } else if component != std::ffi::OsStr::new(".") {
            result.push(component);
        }
    }

    result.to_string_lossy().to_string()
}

pub async fn canonicalize<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?.as_string()?;
    let path_buf = PathBuf::from(&path);
    let canonical_path = canonicalize_best_effort(&path_buf);
    let canonical = canonical_path.to_object(&thread).await?;
    Ok(Some(canonical))
}

pub async fn canonicalize_with_prefix<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?.as_string()?;
    let prefix = parameters.pop()?.as_string()?;
    let joined = Path::new(&prefix).join(&path);
    let canonical_path = canonicalize_best_effort(&joined);
    let canonical = canonical_path.to_object(&thread).await?;
    Ok(Some(canonical))
}

pub async fn check_access<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let access_mode = FileAccessMode::from_bits_truncate(parameters.pop_int()?);
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = Path::new(&path);

    let Ok(metadata) = path.metadata() else {
        return Ok(Some(Value::from(false)));
    };

    #[cfg(target_family = "unix")]
    let (can_read, can_write, can_execute) = {
        let mode = metadata.permissions().mode();
        (mode & 0o444 != 0, mode & 0o222 != 0, mode & 0o111 != 0)
    };
    #[cfg(not(target_family = "unix"))]
    let (can_read, can_write, can_execute) = {
        let readonly = metadata.permissions().readonly();
        (true, !readonly, true)
    };

    let allowed = (!access_mode.contains(FileAccessMode::READ) || can_read)
        && (!access_mode.contains(FileAccessMode::WRITE) || can_write)
        && (!access_mode.contains(FileAccessMode::EXECUTE) || can_execute);

    Ok(Some(Value::from(allowed)))
}

pub async fn create_directory<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(&path);
    let created = tokio::fs::create_dir(&path).await.is_ok();
    Ok(Some(Value::from(created)))
}

pub async fn create_file_exclusively<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?.as_string()?;
    let created = tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .is_ok();
    Ok(Some(Value::from(created)))
}

pub async fn delete<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(&path);

    // Match Windows: a file with an active memory mapping cannot be deleted.
    let vm = thread.vm()?;
    if let Ok(regions) = vm
        .resource_manager()
        .get_or_init(crate::java::nio::mapped_regions::MappedRegions::new)
        && let Ok(canonical) = std::fs::canonicalize(&path)
        && let Some(canonical_str) = canonical.to_str()
        && regions.is_path_mapped(canonical_str)
    {
        return Ok(Some(Value::from(false)));
    }

    let deleted = tokio::fs::remove_file(&path)
        .await
        .or(tokio::fs::remove_dir(&path).await)
        .is_ok();
    Ok(Some(Value::from(deleted)))
}

pub async fn get_boolean_attributes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
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
        .is_some_and(|name| name.to_string_lossy().starts_with('.'))
    {
        attributes |= BooleanAttributeFlags::HIDDEN;
    }
    Ok(Some(Value::Int(attributes.bits())))
}

pub async fn get_drive_directory<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let drive_number = parameters.pop_int()?;
    let base = b'A';
    let max = b'Z';
    let letter_code = base.saturating_add(u8::try_from(drive_number)?);
    if letter_code > max {
        return Ok(Some(Value::Object(None)));
    }
    let drive_letter = letter_code as char;
    let directory = format!("{drive_letter}:\\");
    let directory_value = directory.to_object(&thread).await?;
    Ok(Some(directory_value))
}

pub async fn get_last_modified_time<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(&path);
    let last_modified = match tokio::fs::metadata(&path).await {
        Ok(metadata) => i64::try_from(
            metadata
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|error| RuntimeException(error.to_string()))?
                .as_millis(),
        )?,
        Err(_) => 0,
    };
    Ok(Some(Value::Long(last_modified)))
}

pub async fn get_length<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(&path);
    let metadata = tokio::fs::metadata(&path).await?;
    let length = i64::try_from(metadata.len())?;
    Ok(Some(Value::Long(length)))
}

pub async fn get_name_max<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path = parameters.pop()?.as_string()?;
    // The default on windows is 255 characters for the maximum filename length, but this can be
    // extended to 32,767 characters when long paths are enabled.
    // https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation
    Ok(Some(Value::Long(255)))
}

pub async fn get_space<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let space_type = parameters.pop_int()?;
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = PathBuf::from(path);

    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .iter()
        .find(|d| path.starts_with(d.mount_point()))
        .or_else(|| disks.iter().find(|d| d.mount_point() == Path::new("/")));

    let result = if let Some(disk) = disk {
        match space_type {
            0 => i64::try_from(disk.total_space()).unwrap_or(i64::MAX), // 0: total
            1 | 2 => i64::try_from(disk.available_space()).unwrap_or(i64::MAX), // 1: free | 2: usable
            _ => 0,
        }
    } else {
        0
    };

    Ok(Some(Value::Long(result)))
}

pub async fn list<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(path);

    let mut entries: Vec<Value> = Vec::new();
    let Ok(read_directory) = tokio::fs::read_dir(&path).await else {
        return Ok(Some(Value::Object(None)));
    };
    let mut directory = read_directory;
    while let Ok(Some(entry)) = directory.next_entry().await {
        if let Some(name) = entry.file_name().to_str() {
            let entry_name = name.to_string().to_object(&thread).await?;
            entries.push(entry_name);
        }
    }

    let class = thread.class("java.lang.String").await?;
    let reference = Reference::try_from((class, entries))?;
    let paths = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(paths))
}

pub async fn list_roots<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let mut count = 0i32;
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        if Path::new(&drive).exists() {
            count += 1;
        }
    }
    Ok(Some(Value::Int(count)))
}

pub async fn rename<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let destination_file = parameters.pop()?;
    let destination_path = {
        let destination_file = destination_file.as_object_ref()?;
        destination_file.value("path")?.as_string()?
    };
    let destination = PathBuf::from(destination_path);
    let source_file = parameters.pop()?;
    let source_path = {
        let source_file = source_file.as_object_ref()?;
        source_file.value("path")?.as_string()?
    };
    let source = PathBuf::from(source_path);
    let success = tokio::fs::rename(&source, &destination).await.is_ok();
    Ok(Some(Value::from(success)))
}

pub async fn set_last_modified_time<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let time = parameters.pop_long()?;
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = PathBuf::from(path);

    let seconds = time.saturating_div(1000);
    let nanoseconds = u32::try_from(time % 1000)?.saturating_mul(1_000_000);
    let mtime = FileTime::from_unix_time(seconds, nanoseconds);
    let modified = set_file_mtime(&path, mtime).is_ok();
    Ok(Some(Value::from(modified)))
}

pub async fn set_permission<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let owner_only = parameters.pop_bool()?;
    let enable = parameters.pop_bool()?;
    let access = parameters.pop_int()?;
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(path);
    let modified: bool;

    #[cfg(target_family = "unix")]
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
                if enable {
                    mode |= read_bit;
                } else {
                    mode &= !read_bit;
                }
            }
            1 => {
                if enable {
                    mode |= write_bit;
                } else {
                    mode &= !write_bit;
                }
            }
            2 => {
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

    #[cfg(not(target_family = "unix"))]
    {
        let _ = owner_only;
        let metadata = tokio::fs::metadata(&path).await?;
        let mut permissions = metadata.permissions();
        modified = match access {
            1 => {
                // write
                permissions.set_readonly(!enable);
                tokio::fs::set_permissions(&path, permissions).await.is_ok()
            }
            0 | 2 => {
                // read or execute - not separately controllable on windows
                true
            }
            _ => false,
        };
    }

    Ok(Some(Value::from(modified)))
}

pub async fn set_read_only<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = PathBuf::from(path);
    let metadata = tokio::fs::metadata(&path).await?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    let modified = tokio::fs::set_permissions(&path, permissions).await.is_ok();
    Ok(Some(Value::from(modified)))
}
