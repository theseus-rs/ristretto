use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/Shutdown.beforeHalt()V", Any)]
#[async_method]
pub async fn before_halt<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Shutdown.halt0(I)V", Any)]
#[async_method]
pub async fn halt_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let code = parameters.pop_int()?;
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_before_halt() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = before_halt(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
