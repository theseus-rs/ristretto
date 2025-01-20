use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/io/Win32ErrorMode";

/// Register all native methods for `sun.io.Win32ErrorMode`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "setErrorMode", "(J)J", set_error_mode);
}

#[async_recursion(?Send)]
async fn set_error_mode(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
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
