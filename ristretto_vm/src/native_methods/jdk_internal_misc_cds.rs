use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for jdk.internal.misc.CDS.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/CDS";
    registry.register(
        class_name,
        "initializeFromArchive",
        "(Ljava/lang/Class;)V",
        initialize_from_archive,
    );
    registry.register(class_name, "isDumpingArchive0", "()Z", is_dumping_archive_0);
    registry.register(
        class_name,
        "isDumpingClassList0",
        "()Z",
        is_dumping_class_list_0,
    );
    registry.register(class_name, "isSharingEnabled0", "()Z", is_sharing_enabled_0);
}

fn initialize_from_archive(
    _vm: &VM,
    _call_stack: &mut CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _class = arguments.pop_object()?;
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn is_dumping_archive_0(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn is_dumping_class_list_0(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn is_sharing_enabled_0(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}
