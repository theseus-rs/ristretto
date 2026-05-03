//! Shared file system utilities used by `java.io.UnixFileSystem` and
//! `java.io.WinNTFileSystem` intrinsic implementations.
//!
//! Each public helper performs the common parameter parsing, file system access, and result
//! conversion so the per-OS intrinsic modules can simply delegate to the matching helper.

#![allow(clippy::unused_async)]

use crate::async_fs;
use bitflags::bitflags;
use ristretto_classloader::{Reference, Value};
use ristretto_types::JavaError::RuntimeException;
use ristretto_types::{JavaObject, Parameters, Result, Thread, VM};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg(not(target_family = "wasm"))]
use filetime::{FileTime, set_file_mtime};
#[cfg(not(target_family = "wasm"))]
use sysinfo::Disks;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_family = "unix")]
#[expect(unsafe_code)]
fn check_effective_access(path: &Path, access_mode: FileAccessMode) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let Ok(path) = CString::new(path.as_os_str().as_bytes()) else {
        return false;
    };
    let mut mode = libc::F_OK;
    if access_mode.contains(FileAccessMode::READ) {
        mode |= libc::R_OK;
    }
    if access_mode.contains(FileAccessMode::WRITE) {
        mode |= libc::W_OK;
    }
    if access_mode.contains(FileAccessMode::EXECUTE) {
        mode |= libc::X_OK;
    }

    // AT_EACCESS makes faccessat evaluate the effective uid/gid and supplementary groups,
    // matching the identity the process will use for the subsequent operation.
    unsafe { libc::faccessat(libc::AT_FDCWD, path.as_ptr(), mode, libc::AT_EACCESS) == 0 }
}

fn resolve_path<T: Thread + 'static>(thread: &Arc<T>, path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let vm = thread.vm()?;
    if let Some(user_dir) = vm.system_properties().get("user.dir") {
        Ok(PathBuf::from(user_dir).join(path))
    } else {
        Ok(path.to_path_buf())
    }
}

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

    let result_string = result.to_string_lossy().to_string();
    strip_windows_extended_prefix(&result_string)
}

/// On Windows, [`std::fs::canonicalize`] returns paths prefixed with the extended-length
/// `\\?\` form (or `\\?\UNC\` for UNC paths). The `java.io.File#getCanonicalPath` contract
/// does not include that prefix, so strip it to produce a result that matches the JVM.
fn strip_windows_extended_prefix(path: &str) -> String {
    #[cfg(windows)]
    {
        if let Some(rest) = path.strip_prefix(r"\\?\UNC\") {
            return format!(r"\\{rest}");
        }
        if let Some(rest) = path.strip_prefix(r"\\?\") {
            return rest.to_string();
        }
    }
    path.to_string()
}

/// Maps an [`std::io::Error`] that occurred while opening a file to the message text used by
/// the JDK's native `open0` implementations.
///
/// The JVM's `FileInputStream`, `FileOutputStream`, and `RandomAccessFile` constructors all
/// throw `FileNotFoundException` for any failure to open the underlying file. The exception
/// message uses the platform's native description for the error code so that callers see the
/// same text as a real JVM running on the host operating system.
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
pub(crate) fn open_error_message(error: &std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::NotFound => not_found_message().to_string(),
        std::io::ErrorKind::PermissionDenied => access_denied_message().to_string(),
        _ => os_error_description(error).unwrap_or_else(|| error.to_string()),
    }
}

/// Returns the platform-specific description for the "file not found" error.
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
pub(crate) fn not_found_message() -> &'static str {
    #[cfg(windows)]
    {
        "The system cannot find the file specified"
    }
    #[cfg(not(windows))]
    {
        "No such file or directory"
    }
}

/// Returns the platform-specific description for the "access denied" error.
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
pub(crate) fn access_denied_message() -> &'static str {
    #[cfg(windows)]
    {
        "Access is denied"
    }
    #[cfg(not(windows))]
    {
        "Permission denied"
    }
}

#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
fn os_error_description(error: &std::io::Error) -> Option<String> {
    let raw = error.raw_os_error()?;
    let rendered = std::io::Error::from_raw_os_error(raw).to_string();
    let trimmed = rendered
        .rsplit_once(" (os error")
        .map_or(rendered.as_str(), |(prefix, _)| prefix);
    let trimmed = trimmed.trim_end_matches('.').trim_end();
    Some(trimmed.to_string())
}

pub async fn canonicalize<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?.as_string()?;
    let path_buf = resolve_path(&thread, &path)?;
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
    let prefix = resolve_path(&thread, &prefix)?;
    let joined = prefix.join(&path);
    let canonical_path = canonicalize_best_effort(&joined);
    let canonical = canonical_path.to_object(&thread).await?;
    Ok(Some(canonical))
}

pub async fn check_access<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let access_mode = FileAccessMode::from_bits_truncate(parameters.pop_int()?);
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = resolve_path(&thread, &path)?;

    #[cfg(target_family = "unix")]
    let allowed = check_effective_access(&path, access_mode);
    #[cfg(not(target_family = "unix"))]
    let allowed = {
        let Ok(metadata) = path.metadata() else {
            return Ok(Some(Value::from(false)));
        };
        let can_read = true;
        let can_write = !metadata.permissions().readonly();
        let can_execute = true;
        (!access_mode.contains(FileAccessMode::READ) || can_read)
            && (!access_mode.contains(FileAccessMode::WRITE) || can_write)
            && (!access_mode.contains(FileAccessMode::EXECUTE) || can_execute)
    };

    Ok(Some(Value::from(allowed)))
}

pub async fn create_directory<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = resolve_path(&thread, &path)?;
    let created = async_fs::create_dir(&path).await.is_ok();
    Ok(Some(Value::from(created)))
}

pub async fn create_file_exclusively<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path = parameters.pop()?.as_string()?;
    let resolved_path = resolve_path(&thread, &path)?;
    let created = async_fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&resolved_path)
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
    let path = resolve_path(&thread, &path)?;
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

    let deleted = match async_fs::remove_file(&path).await {
        Ok(()) => true,
        Err(_) => async_fs::remove_dir(&path).await.is_ok(),
    };
    Ok(Some(Value::from(deleted)))
}

pub async fn get_boolean_attributes<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = resolve_path(&thread, path)?;
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = resolve_path(&thread, &path)?;
    let last_modified = match async_fs::metadata(&path).await {
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = resolve_path(&thread, &path)?;
    let length = match async_fs::metadata(&path).await {
        Ok(metadata) => i64::try_from(metadata.len())?,
        Err(_) => 0,
    };
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let space_type = parameters.pop_int()?;
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = resolve_path(&thread, path)?;

    #[cfg(not(target_family = "wasm"))]
    let result = {
        let disks = Disks::new_with_refreshed_list();
        let disk = disks
            .iter()
            .find(|d| path.starts_with(d.mount_point()))
            .or_else(|| disks.iter().find(|d| d.mount_point() == Path::new("/")));

        if let Some(disk) = disk {
            match space_type {
                0 => i64::try_from(disk.total_space()).unwrap_or(i64::MAX),
                1 | 2 => i64::try_from(disk.available_space()).unwrap_or(i64::MAX),
                _ => 0,
            }
        } else {
            0
        }
    };
    #[cfg(target_family = "wasm")]
    let result = {
        let _ = (space_type, &path);
        0i64
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
    let path = resolve_path(&thread, path)?;

    let mut entries: Vec<Value> = Vec::new();
    let Ok(names) = async_fs::read_dir_names(&path).await else {
        return Ok(Some(Value::Object(None)));
    };
    for file_name in names {
        if let Some(name) = file_name.to_str() {
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
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let destination_file = parameters.pop()?;
    let destination_path = {
        let destination_file = destination_file.as_object_ref()?;
        destination_file.value("path")?.as_string()?
    };
    let destination = resolve_path(&thread, destination_path)?;
    let source_file = parameters.pop()?;
    let source_path = {
        let source_file = source_file.as_object_ref()?;
        source_file.value("path")?.as_string()?
    };
    let source = resolve_path(&thread, source_path)?;
    let success = async_fs::rename(&source, &destination).await.is_ok();
    Ok(Some(Value::from(success)))
}

pub async fn set_last_modified_time<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let time = parameters.pop_long()?;
    let file = parameters.pop()?;
    let file = file.as_object_ref()?;
    let path = file.value("path")?.as_string()?;
    let path = resolve_path(&thread, path)?;

    let seconds = time.saturating_div(1000);
    let nanoseconds = u32::try_from(time % 1000)?.saturating_mul(1_000_000);
    #[cfg(not(target_family = "wasm"))]
    let modified = {
        let mtime = FileTime::from_unix_time(seconds, nanoseconds);
        set_file_mtime(&path, mtime).is_ok()
    };
    #[cfg(target_family = "wasm")]
    let modified = {
        let _ = (seconds, nanoseconds, &path);
        false
    };
    Ok(Some(Value::from(modified)))
}

pub async fn set_permission<T: Thread + 'static>(
    thread: Arc<T>,
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
    let path = resolve_path(&thread, path)?;
    let modified: bool;

    #[cfg(target_family = "unix")]
    {
        let metadata = async_fs::metadata(&path).await?;
        let mut permissions = metadata.permissions();
        let mut mode = permissions.mode();

        match access {
            access if access == FileAccessMode::READ.bits() => {
                let read_bit = if owner_only { 0o400 } else { 0o444 };
                if enable {
                    mode |= read_bit;
                } else {
                    mode &= !read_bit;
                }
            }
            access if access == FileAccessMode::WRITE.bits() => {
                let write_bit = if owner_only { 0o200 } else { 0o222 };
                if enable {
                    mode |= write_bit;
                } else {
                    mode &= !write_bit;
                }
            }
            access if access == FileAccessMode::EXECUTE.bits() => {
                let execute_bit = if owner_only { 0o100 } else { 0o111 };
                if enable {
                    mode |= execute_bit;
                } else {
                    mode &= !execute_bit;
                }
            }
            _ => return Ok(Some(Value::from(false))),
        }

        permissions.set_mode(mode);
        modified = async_fs::set_permissions(&path, permissions).await.is_ok();
    }

    #[cfg(not(target_family = "unix"))]
    {
        let _ = owner_only;
        let metadata = async_fs::metadata(&path).await?;
        let mut permissions = metadata.permissions();
        modified = match access {
            access if access == FileAccessMode::WRITE.bits() => {
                permissions.set_readonly(!enable);
                async_fs::set_permissions(&path, permissions).await.is_ok()
            }
            // Windows has no notion of separate READ or EXECUTE permission bits. The JDK's
            // `WinNTFileSystem` returns `true` when enabling these (which is always already
            // allowed) and `false` when attempting to disable them.
            access
                if access == FileAccessMode::READ.bits()
                    || access == FileAccessMode::EXECUTE.bits() =>
            {
                enable
            }
            _ => false,
        };
    }

    Ok(Some(Value::from(modified)))
}

pub async fn set_read_only<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file = parameters.pop()?;
    let path = {
        let file = file.as_object_ref()?;
        file.value("path")?.as_string()?
    };
    let path = resolve_path(&thread, path)?;
    let metadata = async_fs::metadata(&path).await?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    let modified = async_fs::set_permissions(&path, permissions).await.is_ok();
    Ok(Some(Value::from(modified)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_windows_extended_prefix() {
        #[cfg(windows)]
        {
            assert_eq!(r"C:\test", strip_windows_extended_prefix(r"\\?\C:\test"));
            assert_eq!(
                r"\\server\share\test",
                strip_windows_extended_prefix(r"\\?\UNC\server\share\test")
            );
        }
        assert_eq!(
            "relative/path",
            strip_windows_extended_prefix("relative/path")
        );
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    #[test]
    fn test_open_error_message() {
        let not_found = std::io::Error::new(std::io::ErrorKind::NotFound, "ignored");
        assert_eq!(not_found_message(), open_error_message(&not_found));

        let permission_denied =
            std::io::Error::new(std::io::ErrorKind::PermissionDenied, "ignored");
        assert_eq!(
            access_denied_message(),
            open_error_message(&permission_denied)
        );

        let raw_error = std::io::Error::from_raw_os_error(22);
        let message = open_error_message(&raw_error);
        assert!(!message.is_empty());
        assert!(!message.contains("(os error"));
        assert!(!message.ends_with('.'));

        let custom_error = std::io::Error::other("custom failure");
        assert_eq!("custom failure", open_error_message(&custom_error));
    }
}
