use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/Shutdown.beforeHalt()V", Any)]
#[async_method]
pub(crate) async fn before_halt(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Shutdown.halt0(I)V", Any)]
#[async_method]
pub(crate) async fn halt_0(
    _thread: Arc<Thread>,
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
