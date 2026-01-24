use crate::Result;
use crate::intrinsic_methods::jdk;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/reflect/NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn invoke_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
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
