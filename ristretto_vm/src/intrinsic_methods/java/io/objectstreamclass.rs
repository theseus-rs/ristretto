use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/io/ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn has_static_initializer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z")
}

#[intrinsic_method("java/io/ObjectStreamClass.initNative()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z"
    )]
    async fn test_has_static_initializer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = has_static_initializer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
