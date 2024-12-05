use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.reflect.Field`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Field";
    registry.register(
        class_name,
        "getTypeAnnotationBytes0",
        "()[B",
        get_type_annotation_bytes_0,
    );
}

#[async_recursion(?Send)]
async fn get_type_annotation_bytes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Field.getTypeAnnotationBytes0()[B")
}
