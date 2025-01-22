use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classloader::{Reference, Value};
use std::path::PathBuf;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/UnixFileSystem";

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

/// Register all native methods for `java.io.UnixFileSystem`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "getNameMax0",
            "(Ljava/lang/String;)J",
            get_name_max_0,
        );
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "checkAccess",
            "(Ljava/io/File;I)Z",
            check_access,
        );
        registry.register(
            CLASS_NAME,
            "createDirectory",
            "(Ljava/io/File;)Z",
            create_directory,
        );
        registry.register(
            CLASS_NAME,
            "createFileExclusively",
            "(Ljava/lang/String;)Z",
            create_file_exclusively,
        );
        registry.register(
            CLASS_NAME,
            "getLastModifiedTime",
            "(Ljava/io/File;)J",
            get_last_modified_time,
        );
        registry.register(CLASS_NAME, "getLength", "(Ljava/io/File;)J", get_length);
        registry.register(CLASS_NAME, "getSpace", "(Ljava/io/File;I)J", get_space);
        registry.register(
            CLASS_NAME,
            "list",
            "(Ljava/io/File;)[Ljava/lang/String;",
            list,
        );
        registry.register(
            CLASS_NAME,
            "setLastModifiedTime",
            "(Ljava/io/File;J)Z",
            set_last_modified_time,
        );
        registry.register(
            CLASS_NAME,
            "setPermission",
            "(Ljava/io/File;IZZ)Z",
            set_permission,
        );
        registry.register(
            CLASS_NAME,
            "setReadOnly",
            "(Ljava/io/File;)Z",
            set_read_only,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "checkAccess0",
            "(Ljava/io/File;I)Z",
            check_access_0,
        );
        registry.register(
            CLASS_NAME,
            "createDirectory0",
            "(Ljava/io/File;)Z",
            create_directory_0,
        );
        registry.register(
            CLASS_NAME,
            "createFileExclusively0",
            "(Ljava/lang/String;)Z",
            create_file_exclusively_0,
        );
        registry.register(
            CLASS_NAME,
            "getLastModifiedTime0",
            "(Ljava/io/File;)J",
            get_last_modified_time_0,
        );
        registry.register(CLASS_NAME, "getLength0", "(Ljava/io/File;)J", get_length_0);
        registry.register(CLASS_NAME, "getSpace0", "(Ljava/io/File;I)J", get_space_0);
        registry.register(
            CLASS_NAME,
            "list0",
            "(Ljava/io/File;)[Ljava/lang/String;",
            list_0,
        );
        registry.register(
            CLASS_NAME,
            "setLastModifiedTime0",
            "(Ljava/io/File;J)Z",
            set_last_modified_time_0,
        );
        registry.register(
            CLASS_NAME,
            "setPermission0",
            "(Ljava/io/File;IZZ)Z",
            set_permission_0,
        );
        registry.register(
            CLASS_NAME,
            "setReadOnly0",
            "(Ljava/io/File;)Z",
            set_read_only_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "canonicalize0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        canonicalize_0,
    );
    registry.register(CLASS_NAME, "delete0", "(Ljava/io/File;)Z", delete_0);
    registry.register(
        CLASS_NAME,
        "getBooleanAttributes0",
        "(Ljava/io/File;)I",
        get_boolean_attributes_0,
    );
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "rename0",
        "(Ljava/io/File;Ljava/io/File;)Z",
        rename_0,
    );
}

#[async_recursion(?Send)]
async fn canonicalize_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn check_access(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.checkAccess(Ljava/io/File;I)Z")
}

#[async_recursion(?Send)]
async fn check_access_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.checkAccess0(Ljava/io/File;I)Z")
}

#[async_recursion(?Send)]
async fn create_directory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.createDirectory(Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn create_directory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.createDirectory0(Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn create_file_exclusively(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.createFileExclusively(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn create_file_exclusively_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.createFileExclusively0(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn delete_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.delete0(Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn get_boolean_attributes_0(
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

#[async_recursion(?Send)]
async fn get_last_modified_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.getLastModifiedTime(Ljava/io/File;)J")
}

#[async_recursion(?Send)]
async fn get_last_modified_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.getLastModifiedTime0(Ljava/io/File;)J")
}

#[async_recursion(?Send)]
async fn get_length(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_length_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_length_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.getLength0(Ljava/io/File;)J")
}

#[async_recursion(?Send)]
async fn get_name_max_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.getNameMax0(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn get_space(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_space_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_space_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.getSpace0(Ljava/io/File;I)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn list(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    list_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn list_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn rename_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn set_last_modified_time(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_last_modified_time_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn set_last_modified_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z")
}

#[async_recursion(?Send)]
async fn set_permission(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    set_permission_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn set_permission_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.setPermission0(Ljava/io/File;IZZ)Z")
}

#[async_recursion(?Send)]
async fn set_read_only(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    set_read_only_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn set_read_only_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.UnixFileSystem.setReadOnly0(Ljava/io/File;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_canonicalize_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.checkAccess(Ljava/io/File;I)Z"
    )]
    async fn test_check_access() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_access(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.checkAccess0(Ljava/io/File;I)Z"
    )]
    async fn test_check_access_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_access_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.createDirectory(Ljava/io/File;)Z"
    )]
    async fn test_create_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_directory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.createDirectory0(Ljava/io/File;)Z"
    )]
    async fn test_create_directory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_directory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.createFileExclusively(Ljava/lang/String;)Z"
    )]
    async fn test_create_file_exclusively() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_file_exclusively(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.createFileExclusively0(Ljava/lang/String;)Z"
    )]
    async fn test_create_file_exclusively_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_file_exclusively_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.delete0(Ljava/io/File;)Z"
    )]
    async fn test_delete_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = delete_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getLastModifiedTime(Ljava/io/File;)J"
    )]
    async fn test_get_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_modified_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getLastModifiedTime0(Ljava/io/File;)J"
    )]
    async fn test_get_last_modified_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_modified_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getLength0(Ljava/io/File;)J"
    )]
    async fn test_get_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getLength0(Ljava/io/File;)J"
    )]
    async fn test_get_length_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getNameMax0(Ljava/lang/String;)J"
    )]
    async fn test_get_name_max_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_name_max_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getSpace0(Ljava/io/File;I)J"
    )]
    async fn test_get_space() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_space(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.getSpace0(Ljava/io/File;I)J"
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
        expected = "not yet implemented: java.io.UnixFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;"
    )]
    async fn test_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.list0(Ljava/io/File;)[Ljava/lang/String;"
    )]
    async fn test_list_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z"
    )]
    async fn test_rename_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rename_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z"
    )]
    async fn test_set_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_last_modified_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setLastModifiedTime0(Ljava/io/File;J)Z"
    )]
    async fn test_set_last_modified_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_last_modified_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setPermission0(Ljava/io/File;IZZ)Z"
    )]
    async fn test_set_permission() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_permission(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setPermission0(Ljava/io/File;IZZ)Z"
    )]
    async fn test_set_permission_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_permission_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setReadOnly0(Ljava/io/File;)Z"
    )]
    async fn test_set_read_only() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_read_only(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.UnixFileSystem.setReadOnly0(Ljava/io/File;)Z"
    )]
    async fn test_set_read_only_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_read_only_0(thread, Parameters::default()).await;
    }
}
