use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/security/auth/module/UnixSystem.getUnixInfo()V", Any)]
#[async_method]
pub async fn get_unix_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.UnixSystem.getUnixInfo()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.UnixSystem.getUnixInfo()V"
    )]
    async fn test_get_unix_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unix_info(thread, Parameters::default()).await;
    }
}
