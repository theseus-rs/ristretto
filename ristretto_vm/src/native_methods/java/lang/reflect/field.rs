use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/reflect/Field";

/// Register all native methods for `java.lang.reflect.Field`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Field.getTypeAnnotationBytes0()[B"
    )]
    async fn test_get_type_annotation_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_type_annotation_bytes_0(thread, Arguments::default()).await;
    }
}
