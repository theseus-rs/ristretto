use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/AbstractPlainSocketImpl.isReusePortAvailable0()Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn is_reuse_port_available_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    let available = super::socket_ops::reuse_port_available(true);
    #[cfg(target_family = "wasm")]
    let available = false;
    Ok(Some(Value::from(available)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_reuse_port_available_0() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = is_reuse_port_available_0(thread, Parameters::default()).await?;
        #[cfg(not(target_family = "wasm"))]
        let expected = super::super::socket_ops::reuse_port_available(true);
        #[cfg(target_family = "wasm")]
        let expected = false;
        assert_eq!(result, Some(Value::from(expected)));
        Ok(())
    }
}
