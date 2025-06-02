use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/krb5/SCDynamicStoreConfig";

/// Register all intrinsic methods for `sun.security.krb5.SCDynamicStoreConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "getKerberosConfig",
            "()Ljava/util/Hashtable;",
            get_kerberos_config,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "getKerberosConfig",
            "()Ljava/util/List;",
            get_kerberos_config,
        );
    }

    registry.register(
        CLASS_NAME,
        "installNotificationCallback",
        "()V",
        install_notification_callback,
    );
}

#[async_recursion(?Send)]
async fn get_kerberos_config(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;")
}

#[async_recursion(?Send)]
async fn install_notification_callback(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.installNotificationCallback()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;"
    )]
    async fn test_get_kerberos_config() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_kerberos_config(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.krb5.SCDynamicStoreConfig.installNotificationCallback()V"
    )]
    async fn test_install_notification_callback() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = install_notification_callback(thread, Parameters::default()).await;
    }
}
