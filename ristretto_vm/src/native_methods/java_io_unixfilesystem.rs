use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classloader::{Reference, Value};
use std::path::PathBuf;
use std::sync::Arc;

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

/// Register all native methods for java.io.UnixFileSystem.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/UnixFileSystem";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "getBooleanAttributes0",
        "(Ljava/io/File;)I",
        get_boolean_attributes_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
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
