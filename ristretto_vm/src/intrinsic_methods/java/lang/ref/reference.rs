use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/ref/Reference.clear0()V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn clear_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.clear0()V")
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
    let object_parameter = parameters.pop_reference()?;
    let object = parameters.pop_object()?;
    let refers_to = if let Some(Reference::Object(object_parameter)) = object_parameter {
        object == object_parameter
    } else {
        // TODO: this should return true if object has been cleared
        false
    };
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

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ref.Reference.clear0()V")]
    async fn test_clear_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_0(thread, Parameters::default()).await;
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
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.waitForReferencePendingList()V"
    )]
    async fn test_wait_for_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_reference_pending_list(thread, Parameters::default()).await;
    }
}
