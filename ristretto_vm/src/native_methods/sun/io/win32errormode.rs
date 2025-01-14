use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.io.Win32ErrorMode`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/io/Win32ErrorMode";
    registry.register(class_name, "setErrorMode", "(J)J", set_error_mode);
}

#[async_recursion(?Send)]
async fn set_error_mode(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _error_mode = arguments.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/io/Win32ErrorMode";
        assert!(registry
            .method(class_name, "setErrorMode", "(J)J")
            .is_some());
    }

    #[tokio::test]
    async fn test_set_error_mode() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Long(0)]);
        let result = set_error_mode(thread, arguments).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
