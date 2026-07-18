use crate::java::io::filesystem;
pub use crate::java::io::filesystem::{BooleanAttributeFlags, FileAccessMode};
use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

#[intrinsic_method(
    "java/io/WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn canonicalize_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::canonicalize(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn canonicalize_with_prefix_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::canonicalize_with_prefix(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.checkAccess(Ljava/io/File;I)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn check_access<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::check_access(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.checkAccess0(Ljava/io/File;I)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn check_access_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::check_access(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createDirectory(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn create_directory<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::create_directory(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createDirectory0(Ljava/io/File;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn create_directory_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::create_directory(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn create_file_exclusively<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::create_file_exclusively(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createFileExclusively0(Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn create_file_exclusively_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::create_file_exclusively(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.delete0(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn delete_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::delete(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.delete0(Ljava/io/File;Z)Z", Equal(JAVA_25))]
#[async_method]
pub async fn delete_0_with_flag<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_directory = parameters.pop_bool()?;
    filesystem::delete(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getFinalPath0(Ljava/lang/String;)Ljava/lang/String;",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn get_final_path_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::canonicalize(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_boolean_attributes<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_boolean_attributes(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getBooleanAttributes0(Ljava/io/File;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_boolean_attributes_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_boolean_attributes(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_drive_directory<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_drive_directory(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_last_modified_time<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_last_modified_time(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLastModifiedTime0(Ljava/io/File;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_last_modified_time_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_last_modified_time(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLength(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_length<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_length(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLength0(Ljava/io/File;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_length_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_length(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getNameMax0(Ljava/lang/String;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_name_max_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_name_max(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.getSpace0(Ljava/io/File;I)J", Any)]
#[async_method]
pub async fn get_space_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::get_space(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn list<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::list(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn list_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::list(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.listRoots0()I", Any)]
#[async_method]
pub async fn list_roots_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::list_roots(thread, parameters).await
}

#[intrinsic_method("java/io/WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z", Any)]
#[async_method]
pub async fn rename_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::rename(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_last_modified_time<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_last_modified_time(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_last_modified_time_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_last_modified_time(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_permission<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_permission(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setPermission0(Ljava/io/File;IZZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_permission_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_permission(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setReadOnly(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_read_only<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_read_only(thread, parameters).await
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setReadOnly0(Ljava/io/File;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_read_only_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    filesystem::set_read_only(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaError::RuntimeException;
    use ristretto_types::JavaObject;
    use std::path::Path;
    use tempfile::{NamedTempFile, tempdir};

    async fn create_file<T: Thread + 'static>(
        thread: &T,
        prefix: &str,
    ) -> Result<(NamedTempFile, Value)> {
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
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::java17_thread().await?;
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
        let (vm, thread) = crate::test::java17_thread().await?;
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

        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
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
        let value = delete_0(thread, parameters).await?.expect("deleted");
        let deleted = value.as_bool()?;
        assert!(deleted);
        assert!(!Path::new(&path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_attributes() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
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
        let (_vm, thread) = crate::test::java17_thread().await?;
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
        assert!(last_modified_time >= start_time - 1000);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let (_file, file_object) = create_file(&thread, "get_length").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = get_length(thread, parameters).await?.expect("length");
        let length = value.as_i64()?;
        assert_eq!(0, length);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_missing_file() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let temp_dir = tempdir()?;
        let path_name = temp_dir
            .path()
            .join("get_length_missing_file")
            .to_string_lossy()
            .to_string();
        let path_name: Value = path_name.to_object(&thread).await?;
        let file_object = thread
            .object("java.io.File", "Ljava/lang/String;", &[path_name])
            .await?;

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
        let value = get_name_max_0(thread, parameters).await?.expect("name max");
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
        let (vm, thread) = crate::test::java17_thread().await?;
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
    async fn test_list_roots_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = list_roots_0(thread, Parameters::default())
            .await?
            .expect("roots");
        let count = value.as_i32()?;
        assert!(count > 0);
        Ok(())
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
        let (_vm, thread) = crate::test::java17_thread().await?;
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
        let (_vm, thread) = crate::test::java17_thread().await?;
        let (_file, file_object) = create_file(&thread, "set_permission").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        parameters.push_int(2); // access (write)
        parameters.push_bool(true); // enable
        parameters.push_bool(false); // owner_only
        let value = set_permission(thread, parameters).await?.expect("success");
        let success = value.as_bool()?;
        assert!(success);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_read_only() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let (_file, file_object) = create_file(&thread, "set_read_only").await?;
        let mut parameters = Parameters::default();
        parameters.push(file_object);
        let value = set_read_only(thread, parameters).await?.expect("success");
        let success = value.as_bool()?;
        assert!(success);
        Ok(())
    }
}
