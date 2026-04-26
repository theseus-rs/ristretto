use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getCredentialsHandle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn get_credentials_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _password = parameters.pop_reference()?;
    let _domain = parameters.pop_reference()?;
    let _user = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getCredentialsHandle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)J".to_string()).into())
}
#[intrinsic_method(
    "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getNextToken(J[BLsun/net/www/protocol/http/ntlm/NTLMAuthSequence$Status;)[B",
    Any
)]
#[async_method]
pub async fn get_next_token<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _status = parameters.pop_reference()?;
    let _last_token = parameters.pop_reference()?;
    let _crd_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getNextToken(J[BLsun/net/www/protocol/http/ntlm/NTLMAuthSequence$Status;)[B".to_string()).into())
}
#[intrinsic_method(
    "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.initFirst(Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_first<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _status_clazz = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.initFirst(Ljava/lang/Class;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_credentials_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_credentials_handle(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getCredentialsHandle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_next_token() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_next_token(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.getNextToken(J[BLsun/net/www/protocol/http/ntlm/NTLMAuthSequence$Status;)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_first() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_first(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/net/www/protocol/http/ntlm/NTLMAuthSequence.initFirst(Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }
}
