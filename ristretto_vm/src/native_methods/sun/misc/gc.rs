use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.GC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/GC";
    registry.register(
        class_name,
        "maxObjectInspectionAge",
        "()J",
        max_object_inspection_age,
    );
}

#[async_recursion(?Send)]
async fn max_object_inspection_age(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/GC";
        assert!(registry
            .method(class_name, "maxObjectInspectionAge", "()J")
            .is_some());
    }

    #[tokio::test]
    async fn test_max_object_inspection_age() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_object_inspection_age(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
