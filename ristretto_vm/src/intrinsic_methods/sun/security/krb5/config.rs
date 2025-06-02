use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/krb5/Config";

/// Register all intrinsic methods for `sun.security.krb5.Config`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getWindowsDirectory",
        "(Z)Ljava/lang/String;",
        get_windows_directory,
    );
}

#[async_recursion(?Send)]
async fn get_windows_directory(
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
