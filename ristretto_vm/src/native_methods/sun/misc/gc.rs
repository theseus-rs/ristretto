use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/GC";

/// Register all native methods for `sun.misc.GC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "maxObjectInspectionAge",
        "()J",
        max_object_inspection_age,
    );
}

#[async_recursion(?Send)]
async fn max_object_inspection_age(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_max_object_inspection_age() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_object_inspection_age(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
