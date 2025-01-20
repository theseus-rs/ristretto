use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ref/PhantomReference";

/// Register all native methods for `java.lang.ref.PhantomReference`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "refersTo0",
        "(Ljava/lang/Object;)Z",
        refers_to_0,
    );
}

#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z"
    )]
    async fn test_refers_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = refers_to_0(thread, Parameters::default()).await;
    }
}
