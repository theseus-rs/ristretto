use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/reflect/Executable.getParameters0()[Ljava/lang/reflect/Parameter;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_parameters_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Executable.getParameters0()[Ljava/lang/reflect/Parameter;")
}

#[intrinsic_method("java/lang/reflect/Executable.getTypeAnnotationBytes0()[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_type_annotation_bytes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Executable.getTypeAnnotationBytes0()[B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Executable.getParameters0()[Ljava/lang/reflect/Parameter;"
    )]
    async fn test_get_parameters_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_parameters_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Executable.getTypeAnnotationBytes0()[B"
    )]
    async fn test_get_type_annotation_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_type_annotation_bytes_0(thread, Parameters::default()).await;
    }
}
