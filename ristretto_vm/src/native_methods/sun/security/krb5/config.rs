use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.krb5.Config`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/krb5/Config";
    registry.register(
        class_name,
        "getWindowsDirectory",
        "(Z)Ljava/lang/String;",
        get_windows_directory,
    );
}

#[async_recursion(?Send)]
async fn get_windows_directory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.Config.getWindowsDirectory(Z)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/krb5/Config";
        assert!(registry
            .method(class_name, "getWindowsDirectory", "(Z)Ljava/lang/String;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.krb5.Config.getWindowsDirectory(Z)Ljava/lang/String;")]
    async fn test_get_windows_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_windows_directory(thread, Arguments::default()).await;
    }
}
