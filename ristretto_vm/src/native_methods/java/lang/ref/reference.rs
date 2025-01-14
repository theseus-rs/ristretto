use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `java.lang.ref.Reference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/Reference";
    let java_version = registry.java_version().clone();

    if java_version == JAVA_17 || java_version >= JAVA_21 {
        registry.register(class_name, "clear0", "()V", clear_0);
        registry.register(
            class_name,
            "refersTo0",
            "(Ljava/lang/Object;)Z",
            refers_to_0,
        );
    }

    registry.register(
        class_name,
        "getAndClearReferencePendingList",
        "()Ljava/lang/ref/Reference;",
        get_and_clear_reference_pending_list,
    );
    registry.register(
        class_name,
        "hasReferencePendingList",
        "()Z",
        has_reference_pending_list,
    );
    registry.register(
        class_name,
        "waitForReferencePendingList",
        "()V",
        wait_for_reference_pending_list,
    );
}

#[async_recursion(?Send)]
async fn clear_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.clear0()V")
}

#[async_recursion(?Send)]
async fn get_and_clear_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;")
}

#[async_recursion(?Send)]
async fn has_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.hasReferencePendingList()Z")
}

#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object_argument = arguments.pop_reference()?;
    let object = arguments.pop_object()?;
    let refers_to = if let Some(Reference::Object(object_argument)) = object_argument {
        object == object_argument
    } else {
        // TODO: this should return true if object has been cleared
        false
    };
    Ok(Some(Value::from(refers_to)))
}

#[async_recursion(?Send)]
async fn wait_for_reference_pending_list(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.Reference.waitForReferencePendingList()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/ref/Reference";
        assert!(registry
            .method(
                class_name,
                "getAndClearReferencePendingList",
                "()Ljava/lang/ref/Reference;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "hasReferencePendingList", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "waitForReferencePendingList", "()V")
            .is_some());
    }

    #[test]
    fn test_register_java_21() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/ref/Reference";
        assert!(registry.method(class_name, "clear0", "()V").is_some());
        assert!(registry
            .method(class_name, "refersTo0", "(Ljava/lang/Object;)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ref.Reference.clear0()V")]
    async fn test_clear_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.getAndClearReferencePendingList()Ljava/lang/ref/Reference;"
    )]
    async fn test_get_and_clear_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_and_clear_reference_pending_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.hasReferencePendingList()Z"
    )]
    async fn test_has_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = has_reference_pending_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.Reference.waitForReferencePendingList()V"
    )]
    async fn test_wait_for_reference_pending_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_reference_pending_list(thread, Arguments::default()).await;
    }
}
