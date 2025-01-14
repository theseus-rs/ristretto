use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.security.auth.module.UnixSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/security/auth/module/UnixSystem";
    registry.register(class_name, "getUnixInfo", "()V", get_unix_info);
}

#[async_recursion(?Send)]
async fn get_unix_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.UnixSystem.getUnixInfo()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/security/auth/module/UnixSystem";
        assert!(registry.method(class_name, "getUnixInfo", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.UnixSystem.getUnixInfo()V"
    )]
    async fn test_get_unix_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unix_info(thread, Arguments::default()).await;
    }
}
