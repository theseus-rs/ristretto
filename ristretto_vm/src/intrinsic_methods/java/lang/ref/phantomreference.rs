use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ref/PhantomReference";

/// Register all intrinsic methods for `java.lang.ref.PhantomReference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_24 {
        registry.register(CLASS_NAME, "clear0", "()V", clear_0);
    }

    registry.register(
        CLASS_NAME,
        "refersTo0",
        "(Ljava/lang/Object;)Z",
        refers_to_0,
    );
}

#[async_recursion(?Send)]
async fn clear_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.clear0()V")
}

#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ref.PhantomReference.clear0()V")]
    async fn test_clear_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z"
    )]
    async fn test_refers_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = refers_to_0(thread, Parameters::default()).await;
    }
}
