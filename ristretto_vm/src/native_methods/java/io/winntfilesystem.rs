use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `java.io.WinNTFileSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/WinNTFileSystem";
    let java_version = registry.java_version();

    if java_version >= &JAVA_11 {
        registry.register(
            class_name,
            "getNameMax0",
            "(Ljava/lang/String;)I",
            get_name_max_0,
        );
    }

    registry.register(
        class_name,
        "canonicalize0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        canonicalize_0,
    );
    registry.register(
        class_name,
        "canonicalizeWithPrefix0",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        canonicalize_with_prefix_0,
    );
    registry.register(
        class_name,
        "checkAccess",
        "(Ljava/io/File;I)Z",
        check_access,
    );
    registry.register(
        class_name,
        "createDirectory",
        "(Ljava/io/File;)Z",
        create_directory,
    );
    registry.register(
        class_name,
        "createFileExclusively",
        "(Ljava/lang/String;)Z",
        create_file_exclusively,
    );
    registry.register(class_name, "delete0", "(Ljava/io/File;)Z", delete_0);
    registry.register(
        class_name,
        "getBooleanAttributes",
        "(Ljava/io/File;)I",
        get_boolean_attributes,
    );
    registry.register(
        class_name,
        "getDriveDirectory",
        "(I)Ljava/lang/String;",
        get_drive_directory,
    );
    registry.register(
        class_name,
        "getLastModifiedTime",
        "(Ljava/io/File;)J",
        get_last_modified_time,
    );
    registry.register(class_name, "getLength", "(Ljava/io/File;)J", get_length);
    registry.register(class_name, "getSpace0", "(Ljava/io/File;I)J", get_space_0);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "list",
        "(Ljava/io/File;)[Ljava/lang/String;",
        list,
    );
    registry.register(class_name, "listRoots0", "()I", list_roots_0);
    registry.register(
        class_name,
        "rename0",
        "(Ljava/io/File;Ljava/io/File;)Z",
        rename_0,
    );
    registry.register(
        class_name,
        "setLastModifiedTime",
        "(Ljava/io/File;J)Z",
        set_last_modified_time,
    );
    registry.register(
        class_name,
        "setPermission",
        "(Ljava/io/File;IZZ)Z",
        set_permission,
    );
    registry.register(
        class_name,
        "setReadOnly",
        "(Ljava/io/File;)Z",
        set_read_only,
    );
}

#[async_recursion(?Send)]
async fn canonicalize_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn canonicalize_with_prefix_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn check_access(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.checkAccess(Ljava/io/File;I)Z")
}

#[async_recursion(?Send)]
async fn create_directory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.createDirectory(Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn create_file_exclusively(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn delete_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn get_boolean_attributes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I")
}

#[async_recursion(?Send)]
async fn get_drive_directory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_last_modified_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J")
}

#[async_recursion(?Send)]
async fn get_length(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getLength(Ljava/io/File;)J")
}

#[async_recursion(?Send)]
async fn get_name_max_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getNameMax0(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn get_space_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.getSpace0(Ljava/io/File;I)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn list_roots_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.listRoots0()I")
}

#[async_recursion(?Send)]
async fn rename_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z")
}

#[async_recursion(?Send)]
async fn set_last_modified_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z")
}

#[async_recursion(?Send)]
async fn set_permission(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z")
}

#[async_recursion(?Send)]
async fn set_read_only(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.WinNTFileSystem.setReadOnly(Ljava/io/File;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/io/WinNTFileSystem";
        assert!(registry
            .method(
                class_name,
                "canonicalize0",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "canonicalizeWithPrefix0",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "checkAccess", "(Ljava/io/File;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "createDirectory", "(Ljava/io/File;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "createFileExclusively", "(Ljava/lang/String;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "delete0", "(Ljava/io/File;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "getBooleanAttributes", "(Ljava/io/File;)I")
            .is_some());
        assert!(registry
            .method(class_name, "getDriveDirectory", "(I)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "getLastModifiedTime", "(Ljava/io/File;)J")
            .is_some());
        assert!(registry
            .method(class_name, "getLength", "(Ljava/io/File;)J")
            .is_some());
        assert!(registry
            .method(class_name, "getSpace0", "(Ljava/io/File;I)J")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "list", "(Ljava/io/File;)[Ljava/lang/String;")
            .is_some());
        assert!(registry.method(class_name, "listRoots0", "()I").is_some());
        assert!(registry
            .method(class_name, "rename0", "(Ljava/io/File;Ljava/io/File;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setLastModifiedTime", "(Ljava/io/File;J)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setPermission", "(Ljava/io/File;IZZ)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setReadOnly", "(Ljava/io/File;)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.canonicalize0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_canonicalize_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.canonicalizeWithPrefix0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_canonicalize_with_prefix_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = canonicalize_with_prefix_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.checkAccess(Ljava/io/File;I)Z"
    )]
    async fn test_check_access() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_access(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.createDirectory(Ljava/io/File;)Z"
    )]
    async fn test_create_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_directory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.createFileExclusively(Ljava/lang/String;)Z"
    )]
    async fn test_create_file_exclusively() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_file_exclusively(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.delete0(Ljava/io/File;)Z"
    )]
    async fn test_delete_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = delete_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getBooleanAttributes(Ljava/io/File;)I"
    )]
    async fn test_get_boolean_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_attributes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getDriveDirectory(I)Ljava/lang/String;"
    )]
    async fn test_get_drive_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_drive_directory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getLastModifiedTime(Ljava/io/File;)J"
    )]
    async fn test_get_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_modified_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getLength(Ljava/io/File;)J"
    )]
    async fn test_get_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getNameMax0(Ljava/lang/String;)I"
    )]
    async fn test_get_name_max_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_name_max_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.getSpace0(Ljava/io/File;I)J"
    )]
    async fn test_get_space_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_space_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.list(Ljava/io/File;)[Ljava/lang/String;"
    )]
    async fn test_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.WinNTFileSystem.listRoots0()I")]
    async fn test_list_roots_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = list_roots_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.rename0(Ljava/io/File;Ljava/io/File;)Z"
    )]
    async fn test_rename_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rename_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setLastModifiedTime(Ljava/io/File;J)Z"
    )]
    async fn test_set_last_modified_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_last_modified_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setPermission(Ljava/io/File;IZZ)Z"
    )]
    async fn test_set_permission() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_permission(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.WinNTFileSystem.setReadOnly(Ljava/io/File;)Z"
    )]
    async fn test_set_read_only() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_read_only(thread, Arguments::default()).await;
    }
}
