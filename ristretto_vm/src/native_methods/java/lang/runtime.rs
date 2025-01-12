use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::cmp::min;
use std::sync::Arc;
use sysinfo::System;

/// Register all native methods for `java.lang.Runtime`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Runtime";
    registry.register(
        class_name,
        "availableProcessors",
        "()I",
        available_processors,
    );
    registry.register(class_name, "freeMemory", "()J", free_memory);
    registry.register(class_name, "gc", "()V", gc);
    registry.register(class_name, "maxMemory", "()J", max_memory);
    registry.register(class_name, "totalMemory", "()J", total_memory);
}

#[async_recursion(?Send)]
async fn available_processors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let sys = System::new_all();
    let cpus = sys.physical_core_count().unwrap_or(1);
    let cpus = i32::try_from(cpus)?;
    Ok(Some(Value::Int(cpus)))
}

#[async_recursion(?Send)]
async fn free_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let sys = System::new_all();
    let free_memory = sys.total_memory() - sys.used_memory();
    let free_memory = if free_memory > u64::try_from(i64::MAX)? {
        i64::MAX
    } else {
        i64::try_from(free_memory)?
    };
    Ok(Some(Value::Long(free_memory)))
}

#[async_recursion(?Send)]
async fn total_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: This is not the correct implementation; should be the total memory of the JVM
    let sys = System::new_all();
    let used_memory = sys.used_memory();
    let free_memory = if used_memory > u64::try_from(i64::MAX)? {
        i64::MAX
    } else {
        i64::try_from(used_memory)?
    };
    Ok(Some(Value::Long(free_memory)))
}

#[async_recursion(?Send)]
async fn max_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let sys = System::new_all();
    let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
    let total_memory = i64::try_from(total_memory)?;
    Ok(Some(Value::Long(total_memory)))
}

#[async_recursion(?Send)]
async fn gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Runtime";
        assert!(registry
            .method(class_name, "availableProcessors", "()I")
            .is_some());
        assert!(registry.method(class_name, "freeMemory", "()J").is_some());
        assert!(registry.method(class_name, "gc", "()V").is_some());
        assert!(registry.method(class_name, "maxMemory", "()J").is_some());
        assert!(registry.method(class_name, "totalMemory", "()J").is_some());
    }

    #[tokio::test]
    async fn test_available_processors() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = available_processors(thread, Arguments::default()).await?;
        let available_processors = result.unwrap_or(Value::Int(0)).to_int()?;
        assert!(available_processors >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory(thread, Arguments::default()).await?;
        let free_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(free_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_total_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = total_memory(thread, Arguments::default()).await?;
        let total_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(total_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_max_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_memory(thread, Arguments::default()).await?;
        let max_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(max_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_gc() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = gc(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
