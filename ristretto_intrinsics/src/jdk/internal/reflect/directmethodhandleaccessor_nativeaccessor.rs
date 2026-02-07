use crate::jdk;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn invoke_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
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
