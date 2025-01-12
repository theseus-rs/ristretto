use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.StringUTF16`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StringUTF16";
    registry.register(class_name, "isBigEndian", "()Z", is_big_endian);
}

#[async_recursion(?Send)]
async fn is_big_endian(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/StringUTF16";
        assert!(registry.method(class_name, "isBigEndian", "()Z").is_some());
    }

    #[tokio::test]
    async fn test_is_big_endian() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = is_big_endian(thread, Arguments::default()).await?;
        assert_eq!(value, Some(Value::from(true)));
        Ok(())
    }
}
