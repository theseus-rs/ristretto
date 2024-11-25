use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::path::PathBuf;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

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
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/UnixFileSystem";
    let java_version = registry.java_version();

    if java_version >= &JAVA_11 {
        registry.register(
            class_name,
            "getNameMax0",
            "(Ljava/lang/String;)J",
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
        "getBooleanAttributes0",
        "(Ljava/io/File;)I",
        get_boolean_attributes_0,
    );
    registry.register(
        class_name,
        "getLastModifiedTime",
        "(Ljava/io/File;)J",
        get_last_modified_time,
    );
    registry.register(class_name, "getLength", "(Ljava/io/File;)J", get_length);
    registry.register(class_name, "getSpace", "(Ljava/io/File;I)J", get_space);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "list",
        "(Ljava/io/File;)[Ljava/lang/String;",
        list,
    );
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn canonicalize_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn check_access(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_directory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_file_exclusively(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn delete_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_boolean_attributes_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(file)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "getBooleanAttributes0: expected file argument".to_string(),
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_last_modified_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_length(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_name_max_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_space(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn rename_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_last_modified_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_permission(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_read_only(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
