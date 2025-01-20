use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/String";

/// Register all native methods for `java.lang.String`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "intern", "()Ljava/lang/String;", intern);
}

#[async_recursion(?Send)]
async fn intern(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    // TODO: implement string interning
    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;

    #[tokio::test]
    async fn test_intern() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let value = "foo".to_object(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(value.clone());
        let result = intern(thread, parameters).await?;
        assert_eq!(result, Some(value));
        Ok(())
    }
}
