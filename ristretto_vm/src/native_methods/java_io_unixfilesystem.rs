use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::InternalError;
use crate::Result;
use bitflags::bitflags;
use ristretto_classloader::{Reference, Value};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

bitflags! {
    /// Boolean Attribute Flags.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BooleanAttributeFlags: i32 {
        /// If the path exists.
        const EXISTS = 0x01;
        /// If the path is a regular file.
        const REGULAR = 0x02;
        /// If the path is a directory.
        const DIRECTORY = 0x04;
        /// If the path is hidden.
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
fn init_ids(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}

#[expect(clippy::needless_pass_by_value)]
fn get_boolean_attributes_0(
    _call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(file)) = arguments.pop_object()? else {
            return Err(InternalError(
                "getBooleanAttributes0: expected file argument".to_string(),
            ));
        };
        let path = file.field("path")?.value()?.as_string()?;
        let path = PathBuf::from(path);
        let mut attributes = if path.exists() {
            BooleanAttributeFlags::EXISTS.bits()
        } else {
            0
        };
        if path.is_file() {
            attributes |= BooleanAttributeFlags::REGULAR.bits();
        }
        if path.is_dir() {
            attributes |= BooleanAttributeFlags::DIRECTORY.bits();
        }
        if path
            .file_name()
            .map(|name| name.to_string_lossy().starts_with('.'))
            == Some(true)
        {
            attributes |= BooleanAttributeFlags::HIDDEN.bits();
        }
        Ok(Some(Value::Int(attributes)))
    })
}
