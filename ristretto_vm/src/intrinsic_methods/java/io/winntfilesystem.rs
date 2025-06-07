use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
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
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn canonicalize_with_prefix_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.io.WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
    )
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.checkAccess(Ljava/io/File;I)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_access(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.checkAccess(Ljava/io/File;I)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createDirectory(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.createDirectory(Ljava/io/File;)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_file_exclusively(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z")
}

#[intrinsic_method("java/io/WinNTFileSystem.delete0(Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn delete_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean_attributes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I")
}

#[intrinsic_method("java/io/WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_drive_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_last_modified_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getLength(Ljava/io/File;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_length(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getLength(Ljava/io/File;)J")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.getNameMax0(Ljava/lang/String;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_max_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getNameMax0(Ljava/lang/String;)I")
}

#[intrinsic_method("java/io/WinNTFileSystem.getSpace0(Ljava/io/File;I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_space_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getSpace0(Ljava/io/File;I)J")
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
pub(crate) async fn list(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;")
}

#[intrinsic_method("java/io/WinNTFileSystem.listRoots0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn list_roots_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.listRoots0()I")
}

#[intrinsic_method("java/io/WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rename_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_last_modified_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_permission(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z")
}

#[intrinsic_method(
    "java/io/WinNTFileSystem.setReadOnly(Ljava/io/File;)Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_read_only(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setReadOnly(Ljava/io/File;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_canonicalize_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_canonicalize_with_prefix_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_with_prefix_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.checkAccess(Ljava/io/File;I)Z"
    )]
    async fn test_check_access() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_access(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.createDirectory(Ljava/io/File;)Z"
    )]
    async fn test_create_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_directory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z"
    )]
    async fn test_create_file_exclusively() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_file_exclusively(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z"
    )]
    async fn test_delete_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = delete_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I"
    )]
    async fn test_get_boolean_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_attributes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;"
    )]
    async fn test_get_drive_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_drive_directory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J"
    )]
    async fn test_get_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_modified_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getLength(Ljava/io/File;)J"
    )]
    async fn test_get_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getNameMax0(Ljava/lang/String;)I"
    )]
    async fn test_get_name_max_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_name_max_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getSpace0(Ljava/io/File;I)J"
    )]
    async fn test_get_space_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_space_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;"
    )]
    async fn test_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.WinNTFileSystem.listRoots0()I")]
    async fn test_list_roots_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list_roots_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z"
    )]
    async fn test_rename_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rename_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z"
    )]
    async fn test_set_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_last_modified_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z"
    )]
    async fn test_set_permission() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_permission(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setReadOnly(Ljava/io/File;)Z"
    )]
    async fn test_set_read_only() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_read_only(thread, Parameters::default()).await;
    }
}
