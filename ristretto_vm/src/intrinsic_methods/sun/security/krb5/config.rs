use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/Config.getWindowsDirectory(Z)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_windows_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.Config.getWindowsDirectory(Z)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.krb5.Config.getWindowsDirectory(Z)Ljava/lang/String;"
    )]
    async fn test_get_windows_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_windows_directory(thread, Parameters::default()).await;
    }
}
