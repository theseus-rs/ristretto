use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/io/Win32ErrorMode.setErrorMode(J)J", Any)]
#[async_method]
pub(crate) async fn set_error_mode(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _error_mode = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_error_mode() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Long(0)]);
        let result = set_error_mode(thread, parameters).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
