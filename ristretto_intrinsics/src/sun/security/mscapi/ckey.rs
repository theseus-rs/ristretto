use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/security/mscapi/CKey.cleanUp(JJ)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn clean_up<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    let _h_crypt_prov = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/security/mscapi/CKey.cleanUp(JJ)V".to_string()).into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKey.getContainerName(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_container_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_prov = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKey.getContainerName(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/security/mscapi/CKey.getKeyType(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_key_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_crypt_key = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/CKey.getKeyType(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_clean_up() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clean_up(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/CKey.cleanUp(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_container_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_container_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/security/mscapi/CKey.getContainerName(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_key_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_key_type(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/security/mscapi/CKey.getKeyType(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
