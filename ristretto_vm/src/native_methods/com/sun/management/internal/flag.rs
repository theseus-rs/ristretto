use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.management.internal.Flag`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/management/internal/Flag";
    registry.register(
        class_name,
        "getAllFlagNames",
        "()[Ljava/lang/String;",
        get_all_flag_names,
    );
    registry.register(
        class_name,
        "getFlags",
        "([Ljava/lang/String;[Lcom/sun/management/internal/Flag;I)I",
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
    todo!("com.sun.management.internal.Flag.getAllFlagNames()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_flags(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.getFlags([Ljava/lang/String;[Lcom/sun/management/internal/Flag;I)I")
}

#[async_recursion(?Send)]
async fn get_internal_flag_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.getInternalFlagCount()I")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.initialize()V")
}

#[async_recursion(?Send)]
async fn set_boolean_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.setBooleanValue(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn set_double_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.setDoubleValue(Ljava/lang/String;D)V")
}

#[async_recursion(?Send)]
async fn set_long_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.setLongValue(Ljava/lang/String;J)V")
}

#[async_recursion(?Send)]
async fn set_string_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V")
}
