#![allow(unused_imports)]
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_8;
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[cfg(not(target_os = "windows"))]
#[intrinsic_method("sun/net/PortConfig.getLower0()I", Any)]
#[async_method]
pub async fn get_lower_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(49152)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/PortConfig.getLower0()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_lower_0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(49152)))
}

#[cfg(not(target_os = "windows"))]
#[intrinsic_method("sun/net/PortConfig.getUpper0()I", Any)]
#[async_method]
pub async fn get_upper_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(65535)))
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/PortConfig.getUpper0()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_upper_0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(65535)))
}

#[cfg(all(test, not(target_os = "windows"), not(target_family = "wasm")))]
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
