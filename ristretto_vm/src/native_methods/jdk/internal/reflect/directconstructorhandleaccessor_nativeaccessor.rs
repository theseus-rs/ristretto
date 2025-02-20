use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor";

/// Register all native methods for `jdk.internal.reflect.DirectConstructorHandleAccessor$NativeAccessor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "newInstance0",
        "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        new_instance_0,
    );
}

#[async_recursion(?Send)]
async fn new_instance_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.reflect.DirectConstructorHandleAccessor$NativeAccessor.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.reflect.DirectConstructorHandleAccessor$NativeAccessor.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_new_instance_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = new_instance_0(thread, Parameters::default()).await;
    }
}
