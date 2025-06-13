use crate::Result;
use crate::intrinsic_methods::jdk;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn new_instance_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::reflect::nativeconstructoraccessorimpl::new_instance_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intrinsic_methods::jdk;

    #[tokio::test]
    async fn test_new_instance_0() -> Result<()> {
        jdk::internal::reflect::nativeconstructoraccessorimpl::tests::new_instance_test(
            new_instance_0,
        )
        .await
    }
}
