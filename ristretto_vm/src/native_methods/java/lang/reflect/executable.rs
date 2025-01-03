use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.reflect.Executable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Executable";
    registry.register(
        class_name,
        "getParameters0",
        "()[Ljava/lang/reflect/Parameter;",
        get_parameters_0,
    );
    registry.register(
        class_name,
        "getTypeAnnotationBytes0",
        "()[B",
        get_type_annotation_bytes_0,
    );
}

#[async_recursion(?Send)]
async fn get_parameters_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Executable.getParameters0()[Ljava/lang/reflect/Parameter;")
}

#[async_recursion(?Send)]
async fn get_type_annotation_bytes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Executable.getTypeAnnotationBytes0()[B")
}
