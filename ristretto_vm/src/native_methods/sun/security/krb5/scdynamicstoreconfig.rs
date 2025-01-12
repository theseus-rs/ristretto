use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.security.krb5.SCDynamicStoreConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/krb5/SCDynamicStoreConfig";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "getKerberosConfig",
            "()Ljava/util/Hashtable;",
            get_kerberos_config,
        );
    } else {
        registry.register(
            class_name,
            "getKerberosConfig",
            "()Ljava/util/List;",
            get_kerberos_config,
        );
    }

    registry.register(
        class_name,
        "installNotificationCallback",
        "()V",
        install_notification_callback,
    );
}

#[async_recursion(?Send)]
async fn get_kerberos_config(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;")
}

#[async_recursion(?Send)]
async fn install_notification_callback(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.installNotificationCallback()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/security/krb5/SCDynamicStoreConfig";
        assert!(registry
            .method(class_name, "getKerberosConfig", "()Ljava/util/Hashtable;")
            .is_some());
        assert!(registry
            .method(class_name, "installNotificationCallback", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;"
    )]
    async fn test_get_kerberos_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_kerberos_config(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.krb5.SCDynamicStoreConfig.installNotificationCallback()V"
    )]
    async fn test_install_notification_callback() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = install_notification_callback(thread, Arguments::default()).await;
    }
}
