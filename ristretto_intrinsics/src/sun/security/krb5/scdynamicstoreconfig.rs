use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_kerberos_config_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/Hashtable;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.getKerberosConfig()Ljava/util/List;",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn get_kerberos_config_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.krb5.SCDynamicStoreConfig.getKerberosConfig()Ljava/util/List;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/krb5/SCDynamicStoreConfig.installNotificationCallback()V",
    Any
)]
#[async_method]
pub async fn install_notification_callback<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.krb5.SCDynamicStoreConfig.installNotificationCallback()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_kerberos_config_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_kerberos_config_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_kerberos_config_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_kerberos_config_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_install_notification_callback() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = install_notification_callback(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
