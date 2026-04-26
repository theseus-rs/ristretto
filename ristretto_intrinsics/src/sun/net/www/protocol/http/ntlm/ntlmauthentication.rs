use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSite0(Ljava/lang/String;)Z",
    Any
)]
#[async_method]
pub async fn is_trusted_site0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _url = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSite0(Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSiteAvailable()Z",
    Any
)]
#[async_method]
pub async fn is_trusted_site_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSiteAvailable()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_trusted_site0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_trusted_site0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSite0(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_trusted_site_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_trusted_site_available(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/www/protocol/http/ntlm/NTLMAuthentication.isTrustedSiteAvailable()Z",
            result.unwrap_err().to_string()
        );
    }
}
