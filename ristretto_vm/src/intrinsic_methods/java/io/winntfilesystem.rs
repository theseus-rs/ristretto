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
    let path = parameters.pop()?.as_string()?;
    let prefix = parameters.pop()?.as_string()?;
    let canonical_path: String;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
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

    let canonical = canonical_path.to_object(&thread).await?;
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
        let letter_code = base.saturating_add(u8::try_from(drive_number)?);
        let letter_code = if letter_code > max {
            return Ok(Some(Value::Object(None)));
        } else {
            letter_code
        };
        letter_code as char
    };
    let directory = format!("{drive_letter}:\\");
    let directory_value = directory.to_object(&thread).await?;
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
        Err(crate::JavaError::RuntimeException("Only supported on Windows".to_string()).into())
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
    use crate::JavaObject;
    use crate::intrinsic_methods::java::io::unixfilesystem::FileAccessMode;
    use std::path::Path;
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
        let canonical_path = value.as_string()?;
        assert_ne!(original_path, canonical_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_canonicalize_with_prefix_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let original_path_prefix = "..";
        let original_path = ".";
        let path_prefix = original_path_prefix.to_object(&thread).await?;
        let path = original_path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_prefix);
        parameters.push(path);
        let value = canonicalize_with_prefix_0(thread, parameters)
            .await?
            .expect("value");
        let canonical_path = value.as_string()?;
        assert!(!canonical_path.contains(original_path_prefix));
        assert!(!canonical_path.contains(original_path));
        Ok(())
    }

    #[tokio::test]
    async fn test_check_access() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "check_access").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_int(FileAccessMode::READ.bits());
        let value = check_access(thread, parameters).await?.expect("name max");
        let has_access = value.as_bool()?;
        assert!(has_access);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_directory() -> Result<()> {
        let path_name = "test_create_directory";
        if Path::new(path_name).exists() {
            tokio::fs::remove_dir_all(path_name).await?;
        }
        let (vm, thread) = crate::test::thread().await?;
        let path_object: Value = path_name.to_object(&thread).await?;
        let file_object = vm
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = create_directory(thread, parameters)
            .await?
            .expect("created");
        let created = value.as_bool()?;
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
        let created = value.as_bool()?;
        assert!(created);
        tokio::fs::remove_file(path_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (file, file_object) = create_file(&thread, "delete").await?;
        let path = file.path().to_string_lossy().to_string();
        assert!(Path::new(&path).exists());

        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = unixfilesystem::delete_0(thread, parameters)
            .await?
            .expect("deleted");
        let deleted = value.as_bool()?;
        assert!(deleted);
        assert!(!Path::new(&path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_attributes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let (_file, file_object) = create_file(&thread, "get_boolean_attributes_0").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_boolean_attributes(thread, parameters)
            .await?
            .expect("attributes");
        let attributes = value.as_i32()?;
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
        let drive = value.as_string()?;
        assert_eq!(drive, "C:\\");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_last_modified_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let start_time = i64::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| RuntimeException(e.to_string()))?
                .as_millis(),
        )?;
        let (_file, file_object) = create_file(&thread, "get_last_modified_time").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_last_modified_time(thread, parameters)
            .await?
            .expect("last modified time");
        let last_modified_time = value.as_i64()?;
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
        let length = value.as_i64()?;
        assert_eq!(0, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_max_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path = "get_name_max_0.txt".to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path);
        let value = unixfilesystem::get_name_max_0(thread, parameters)
            .await?
            .expect("name max");
        let length = value.as_i64()?;
        assert_eq!(255, length);
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
            let space = value.as_i64()?;

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
        let (vm, thread) = crate::test::thread().await?;
        let path_object: Value = ".".to_object(&thread).await?;
        let file_object = vm
            .object("java.io.File", "Ljava/lang/String;", &[path_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = list(thread, parameters).await?.expect("paths");
        let class_name = value.as_reference()?.class_name()?;
        let elements: Vec<Value> = value.try_into()?;
        assert_eq!(class_name, "java/lang/String");
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
            let count = value.as_i32().expect("count");
            assert!(count > 0);
        }

        #[cfg(not(target_os = "windows"))]
        {
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_rename_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let (source_file, source_object) = create_file(&thread, "rename_source").await?;
        let source_path = source_file.path().to_string_lossy().to_string();
        let destination_path = format!("{source_path}_destination");
        let destination_object: Value = destination_path.to_object(&thread).await?;
        let destination_object = vm
            .object("java.io.File", "Ljava/lang/String;", &[destination_object])
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(source_object);
        parameters.push(destination_object);
        let value = rename_0(thread, parameters).await?.expect("renamed");
        let renamed = value.as_bool()?;
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
        let success = value.as_bool()?;
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
        let success = value.as_bool()?;
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
        let success = value.as_bool()?;
        assert!(success);
        Ok(())
    }
}
