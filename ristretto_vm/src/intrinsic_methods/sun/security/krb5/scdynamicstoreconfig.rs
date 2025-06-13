use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_kerberos_config_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;")
}

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.getKerberosConfig()Ljava/util/List;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_kerberos_config_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/List;")
}

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.installNotificationCallback()V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn install_notification_callback(
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
    async fn test_get_kerberos_config_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_kerberos_config_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/List;"
    )]
    async fn test_get_kerberos_config_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_kerberos_config_1(thread, Parameters::default()).await;
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
