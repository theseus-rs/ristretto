use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Version`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Version";
    registry.register(
        class_name,
        "getJdkSpecialVersion",
        "()Ljava/lang/String;",
        get_jdk_special_version,
    );
    registry.register(class_name, "getJdkVersionInfo", "()V", get_jdk_version_info);
    registry.register(
        class_name,
        "getJvmSpecialVersion",
        "()Ljava/lang/String;",
        get_jvm_special_version,
    );
    registry.register(class_name, "getJvmVersionInfo", "()Z", get_jvm_version_info);
}

#[async_recursion(?Send)]
async fn get_jdk_special_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_jdk_version_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJdkVersionInfo()V")
}

#[async_recursion(?Send)]
async fn get_jvm_special_version(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_jvm_version_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.Version.getJvmVersionInfo()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/Version";
        assert!(registry
            .method(class_name, "getJdkSpecialVersion", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "getJdkVersionInfo", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "getJvmSpecialVersion", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "getJvmVersionInfo", "()Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Version.getJdkSpecialVersion()Ljava/lang/String;")]
    async fn test_get_jdk_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jdk_special_version(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Version.getJdkVersionInfo()V")]
    async fn test_get_jdk_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jdk_version_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Version.getJvmSpecialVersion()Ljava/lang/String;")]
    async fn test_get_jvm_special_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jvm_special_version(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Version.getJvmVersionInfo()Z")]
    async fn test_get_jvm_version_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_jvm_version_info(thread, Arguments::default()).await;
    }
}
