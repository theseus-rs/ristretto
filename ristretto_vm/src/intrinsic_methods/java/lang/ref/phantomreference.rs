use crate::Result;
use crate::intrinsic_methods::java::lang::r#ref::reference;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ref/PhantomReference.clear0()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn clear_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    reference::clear_0(thread, parameters).await
}

#[intrinsic_method("java/lang/ref/PhantomReference.refersTo0(Ljava/lang/Object;)Z", Any)]
#[async_method]
pub(crate) async fn refers_to_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    reference::refers_to_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;

    #[tokio::test]
    async fn test_clear_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let value = "foo".to_object(&thread).await?;
        let reference_queue = vm
            .object("java/lang/ref/ReferenceQueue", "", &[] as &[Value])
            .await?;
        let phantom_reference = vm
            .object(
                "java/lang/ref/PhantomReference",
                "Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;",
                &[value, reference_queue],
            )
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(phantom_reference.clone());

        let result = clear_0(thread, parameters).await?;
        assert_eq!(result, None);
        let phantom_reference = phantom_reference.as_object_ref()?;
        let referent = phantom_reference.value("referent")?;
        assert_eq!(referent, Value::Object(None));
        Ok(())
    }

    #[tokio::test]
    async fn test_refers_to_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let value = "foo".to_object(&thread).await?;
        let reference_queue = vm
            .object("java/lang/ref/ReferenceQueue", "", &[] as &[Value])
            .await?;
        let phantom_reference = vm
            .object(
                "java/lang/ref/PhantomReference",
                "Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;",
                &[value.clone(), reference_queue],
            )
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(phantom_reference.clone());
        parameters.push(value.clone());

        let value = refers_to_0(thread, parameters).await?.expect("refers to");
        let refers_to = value.as_bool()?;
        assert!(refers_to);
        Ok(())
    }
}
