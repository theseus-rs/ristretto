use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/security/auth/module/UnixSystem";

/// Register all intrinsic methods for `com.sun.security.auth.module.UnixSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "getUnixInfo", "()V", get_unix_info);
}

#[async_recursion(?Send)]
async fn get_unix_info(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
