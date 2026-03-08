use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/net/PortConfig.getLower0()I", Any)]
#[async_method]
pub async fn get_lower_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(49152)))
}

#[intrinsic_method("sun/net/PortConfig.getUpper0()I", Any)]
#[async_method]
pub async fn get_upper_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(65535)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_lower_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_lower_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(49152)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_upper_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_upper_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(65535)));
        Ok(())
    }
}
