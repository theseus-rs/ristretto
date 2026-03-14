use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/AbstractPlainDatagramSocketImpl.isReusePortAvailable0()Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn is_reuse_port_available_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_reuse_port_available_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_reuse_port_available_0(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }
}
