use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.Flag`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/Flag";
    registry.register(
        class_name,
        "getAllFlagNames",
        "()[Ljava/lang/String;",
        get_all_flag_names,
    );
    registry.register(
        class_name,
        "getFlags",
        "([Ljava/lang/String;[Lsun/management/Flag;I)I",
        get_flags,
    );
    registry.register(
        class_name,
        "getInternalFlagCount",
        "()I",
        get_internal_flag_count,
    );
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(
        class_name,
        "setBooleanValue",
        "(Ljava/lang/String;Z)V",
        set_boolean_value,
    );
    registry.register(
        class_name,
        "setDoubleValue",
        "(Ljava/lang/String;D)V",
        set_double_value,
    );
    registry.register(
        class_name,
        "setLongValue",
        "(Ljava/lang/String;J)V",
        set_long_value,
    );
    registry.register(
        class_name,
        "setStringValue",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        set_string_value,
    );
}

#[async_recursion(?Send)]
async fn get_all_flag_names(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_flags(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_internal_flag_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_boolean_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_double_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_long_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_string_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
