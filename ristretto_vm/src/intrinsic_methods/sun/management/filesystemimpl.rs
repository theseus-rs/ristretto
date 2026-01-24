use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn is_access_user_only_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z"
    )]
    async fn test_is_access_user_only_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_access_user_only_0(thread, Parameters::default()).await;
    }
}
