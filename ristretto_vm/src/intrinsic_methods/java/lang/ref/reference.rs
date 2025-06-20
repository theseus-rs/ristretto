use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/ref/Reference.clear0()V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn clear_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
    object.set_value("referent", Value::Object(None))?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ref/Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_and_clear_reference_pending_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;")
}

#[intrinsic_method(
    "java/lang/ref/Reference.hasReferencePendingList()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn has_reference_pending_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.hasReferencePendingList()Z")
}

#[intrinsic_method(
    "java/lang/ref/Reference.refersTo0(Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn refers_to_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object_parameter = parameters.pop()?;
    let reference = parameters.pop_object()?;
    let object = reference.value("referent")?;
    let refers_to = object == object_parameter;
    Ok(Some(Value::from(refers_to)))
}

#[intrinsic_method(
    "java/lang/ref/Reference.waitForReferencePendingList()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn wait_for_reference_pending_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.waitForReferencePendingList()V")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_clear_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let value = "foo".to_object(&vm).await?;
        let weak_reference = vm
            .object(
                "java/lang/ref/WeakReference",
                "Ljava/lang/Object;",
                &[value],
            )
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(weak_reference.clone());

        let result = clear_0(thread, parameters).await?;
        assert_eq!(result, None);
        let weak_reference: Object = weak_reference.try_into()?;
        let referent = weak_reference.value("referent")?;
        assert_eq!(referent, Value::Object(None));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;"
    )]
    async fn test_get_and_clear_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_clear_reference_pending_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.hasReferencePendingList()Z"
    )]
    async fn test_has_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = has_reference_pending_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_refers_to_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let value = "foo".to_object(&vm).await?;
        let weak_reference = vm
            .object(
                "java/lang/ref/WeakReference",
                "Ljava/lang/Object;",
                &[value.clone()],
            )
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(weak_reference.clone());
        parameters.push(value);
        let value = refers_to_0(thread, parameters).await?.expect("refers to");
        let refers_to: bool = value.try_into()?;
        assert!(refers_to);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.waitForReferencePendingList()V"
    )]
    async fn test_wait_for_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_reference_pending_list(thread, Parameters::default()).await;
    }
}
