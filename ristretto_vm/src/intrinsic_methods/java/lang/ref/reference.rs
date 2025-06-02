use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ref/Reference";

/// Register all intrinsic methods for `java.lang.ref.Reference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "clear0", "()V", clear_0);
        registry.register(
            CLASS_NAME,
            "refersTo0",
            "(Ljava/lang/Object;)Z",
            refers_to_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "getAndClearReferencePendingList",
        "()Ljava/lang/ref/Reference;",
        get_and_clear_reference_pending_list,
    );
    registry.register(
        CLASS_NAME,
        "hasReferencePendingList",
        "()Z",
        has_reference_pending_list,
    );
    registry.register(
        CLASS_NAME,
        "waitForReferencePendingList",
        "()V",
        wait_for_reference_pending_list,
    );
}

#[async_recursion(?Send)]
async fn clear_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.clear0()V")
}

#[async_recursion(?Send)]
async fn get_and_clear_reference_pending_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;")
}

#[async_recursion(?Send)]
async fn has_reference_pending_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.hasReferencePendingList()Z")
}

#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
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

#[async_recursion(?Send)]
async fn wait_for_reference_pending_list(
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
