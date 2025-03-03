use crate::Result;
use crate::native_methods::jdk;
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
async fn new_instance_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::nativeconstructoraccessorimpl::new_instance_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_methods::jdk;

    #[tokio::test]
    async fn test_new_instance_0() -> Result<()> {
        jdk::internal::reflect::nativeconstructoraccessorimpl::tests::new_instance_test(
            new_instance_0,
        )
        .await
    }
}
