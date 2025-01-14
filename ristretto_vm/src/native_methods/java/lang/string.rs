use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.String`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/String";
    registry.register(class_name, "intern", "()Ljava/lang/String;", intern);
}

#[async_recursion(?Send)]
async fn intern(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let value = arguments.pop()?;
    // TODO: implement string interning
    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/String";
        assert!(registry
            .method(class_name, "intern", "()Ljava/lang/String;")
            .is_some());
    }

    #[tokio::test]
    async fn test_intern() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let value = "foo".to_object(&vm).await?;
        let mut arguments = Arguments::default();
        arguments.push(value.clone());
        let result = intern(thread, arguments).await?;
        assert_eq!(result, Some(value));
        Ok(())
    }
}
