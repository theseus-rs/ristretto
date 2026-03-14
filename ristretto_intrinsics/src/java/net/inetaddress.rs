use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/net/InetAddress.init()V", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/net/InetAddress.isIPv4Available()Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn is_ipv_4_available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method("java/net/InetAddress.isIPv6Supported()Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn is_ipv_6_supported<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_ipv_4_available() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_ipv_4_available(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_ipv_6_supported() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_ipv_6_supported(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }
}
