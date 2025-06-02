use crate::Result;
use crate::intrinsic_methods::jdk;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor";

/// Register all intrinsic methods for `jdk.internal.reflect.DirectMethodHandleAccessor$NativeAccessor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "invoke0",
        "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_0,
    );
}

#[async_recursion(?Send)]
async fn invoke_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::nativemethodaccessorimpl::invoke_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invoke_0() -> Result<()> {
        jdk::internal::reflect::nativemethodaccessorimpl::tests::invoke_test(invoke_0).await
    }
}
