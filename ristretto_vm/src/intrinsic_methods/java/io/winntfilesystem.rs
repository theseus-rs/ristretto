use crate::JavaError::RuntimeException;
use crate::intrinsic_methods::java::io::unixfilesystem;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/io/WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn canonicalize_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::canonicalize_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn canonicalize_with_prefix_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let path: String = parameters.pop_object()?.try_into()?;
    let prefix: String = parameters.pop_object()?.try_into()?;
    let canonical_path: String;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = vm;
        let _ = prefix;
        // In WebAssembly, we cannot access the filesystem directly, so we return the path as is.
        canonical_path = path;
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let path = std::path::Path::new(&prefix).join(&path);
        let canonicalized_path = path.canonicalize()?;
        canonical_path = canonicalized_path.to_string_lossy().to_string();
    }

    let canonical = canonical_path.to_object(&vm).await?;
    Ok(Some(canonical))
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.checkAccess(Ljava/io/File;I)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_access(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::check_access_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createDirectory(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_directory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::create_directory_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_file_exclusively(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::create_file_exclusively_0(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.delete0(Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn delete_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    unixfilesystem::delete_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean_attributes(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::get_boolean_attributes_0(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_drive_directory(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let drive_number = parameters.pop_int()?;
    let drive_letter = {
        let base = b'A';
        let max = b'Z';
        let letter_code = base.saturating_add(drive_number as u8);
        let letter_code = if letter_code > max {
            return Ok(Some(Value::Object(None)));
        } else {
            letter_code
        };
        letter_code as char
    };
    let directory = format!("{drive_letter}:\\");
    let vm = thread.vm()?;
    let directory_value = directory.to_object(&vm).await?;
    Ok(Some(directory_value))
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_last_modified_time(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::get_last_modified_time_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLength(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_length(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::get_length_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getNameMax0(Ljava/lang/String;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_max_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::get_name_max_0(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.getSpace0(Ljava/io/File;I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_space_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::get_space_0(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn list(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    unixfilesystem::list_0(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.listRoots0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn list_roots_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    {
        use std::io::{Error, ErrorKind};
        let mut count = 0i32;
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            if std::path::Path::new(&drive).exists() {
                count += 1;
            }
        }
        Ok(Some(Value::Int(count)))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err(RuntimeException("Only supported on Windows".to_string()).into())
    }
}

#[intrinsic_method("java/io/WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rename_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    unixfilesystem::rename_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_last_modified_time(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::set_last_modified_time_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_permission(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::set_permission_0(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setReadOnly(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_read_only(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    unixfilesystem::set_read_only_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaError::RuntimeException;
    use crate::{JavaObject, VM};
    use std::path::Path;
    use tempfile::NamedTempFile;

    async fn create_file(vm: &VM, prefix: &str) -> Result<(NamedTempFile, Value)> {
        let file = NamedTempFile::with_prefix(prefix)?;
        let path_name = file.path().to_string_lossy().to_string();
        let path_name: Value = path_name.to_object(vm).await?;
        let file_object = vm
            .object("java.io.File", "Ljava/lang/String;", &[path_name])
            .await?;
        Ok((file, file_object))
    }

    #[tokio::test]
    async fn test_canonicalize_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let original_path = ".";
        let path = original_path.to_object(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(path);
        let value = canonicalize_0(thread, parameters).await?.expect("value");
        let canonical_path: String = value.try_into()?;
        assert_ne!(original_path, canonical_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_canonicalize_with_prefix_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let original_path_prefix = "..";
        let original_path = ".";
        let path_prefix = original_path_prefix.to_object(&vm).await?;
        let path = original_path.to_object(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_prefix);
        parameters.push(path);
        let value = canonicalize_with_prefix_0(thread, parameters)
            .await?
            .expect("value");
        let canonical_path: String = value.try_into()?;
        assert!(!canonical_path.contains(original_path_prefix));
        assert!(!canonical_path.contains(original_path));
        Ok(())
    }

    #[tokio::test]
    async fn test_check_access() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "check_access").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_int(1); // Read access
        let value = check_access(thread, parameters).await?.expect("name max");
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
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_object: Value = path_name.to_object(&vm).await?;
        let file_object = vm
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
    async fn test_create_file_exclusively() -> Result<()> {
        let path_name = "test_create_file_exclusively";
        if Path::new(path_name).exists() {
            tokio::fs::remove_file(path_name).await?;
        }

        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_object: Value = path_name.to_object(&vm).await?;
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
    async fn test_delete_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (file, file_object) = create_file(&vm, "delete").await?;
        let path: String = file.path().to_string_lossy().to_string();
        assert!(Path::new(&path).exists());

        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = unixfilesystem::delete_0(thread, parameters)
            .await?
            .expect("deleted");
        let deleted: bool = value.try_into()?;
        assert!(deleted);
        assert!(!Path::new(&path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_attributes() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "get_boolean_attributes_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_boolean_attributes(thread, parameters)
            .await?
            .expect("attributes");
        let attributes: i32 = value.try_into()?;
        assert!(attributes > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_drive_directory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(2); // Drive number 0 corresponds to A:
        let value = get_drive_directory(thread, parameters)
            .await?
            .expect("drive");
        let drive: String = value.try_into()?;
        assert_eq!(drive, "C:\\");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_last_modified_time() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| RuntimeException(e.to_string()))?
            .as_millis() as i64;
        let (_file, file_object) = create_file(&vm, "get_last_modified_time").await?;
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
    async fn test_get_length() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "get_length").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_length(thread, parameters).await?.expect("length");
        let length: i64 = value.try_into()?;
        assert_eq!(0, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_max_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path = "get_name_max_0.txt".to_object(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(path);
        let value = unixfilesystem::get_name_max_0(thread, parameters)
            .await?
            .expect("name max");
        let length: i64 = value.try_into()?;
        assert_eq!(255, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_space_0() -> Result<()> {
        for space_type in [0, 1, 2, 3] {
            let (vm, thread) = crate::test::thread().await.expect("thread");
            let (_file, file_object) = create_file(&vm, "get_space_0").await?;
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
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_object: Value = ".".to_object(&vm).await?;
        let file_object = vm
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = list(thread, parameters).await?.expect("paths");
        let (class, elements) = value.try_into()?;
        assert_eq!(class.name(), "java/lang/String");
        assert!(!elements.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_list_roots_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = list_roots_0(thread, Parameters::default()).await;

        #[cfg(target_os = "windows")]
        {
            let value = result.expect("roots");
            let count: i32 = value.try_into().expect("count");
            assert!(count > 0);
        }

        #[cfg(not(target_os = "windows"))]
        {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_rename_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (source_file, source_object) = create_file(&vm, "rename_source").await?;
        let source_path: String = source_file.path().to_string_lossy().to_string();
        let destination_path = format!("{source_path}_destination");
        let destination_object: Value = destination_path.to_object(&vm).await?;
        let destination_object = vm
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
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "set_last_modified_time").await?;
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
    async fn test_set_permission() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "set_permission").await?;
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
    async fn test_set_read_only() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let (_file, file_object) = create_file(&vm, "set_read_only").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = set_read_only(thread, parameters).await?.expect("success");
        let success: bool = value.try_into()?;
        assert!(success);
        Ok(())
    }
}
