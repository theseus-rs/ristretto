use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
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
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_object = parameters.pop()?;
    let class = get_class(&thread, &class_object).await?;
    let has_initializer = class.method("<clinit>", "()V").is_some();
    Ok(Some(Value::from(has_initializer)))
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
    use crate::JavaObject;

    #[tokio::test]
    async fn test_has_static_initializer_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await?;
        let class_object = class.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(class_object);
        let has_initializer = has_static_initializer(thread, parameters).await?;
        assert_eq!(Some(Value::from(false)), has_initializer);
        Ok(())
    }

    #[tokio::test]
    async fn test_has_static_initializer_true() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/System").await?;
        let class_object = class.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(class_object);
        let has_initializer = has_static_initializer(thread, parameters).await?;
        assert_eq!(Some(Value::from(true)), has_initializer);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
