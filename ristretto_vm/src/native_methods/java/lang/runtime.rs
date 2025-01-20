use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::cmp::min;
use std::sync::Arc;
use sysinfo::System;

const CLASS_NAME: &str = "java/lang/Runtime";

/// Register all native methods for `java.lang.Runtime`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "runFinalization0", "()V", run_finalization_0);
        registry.register(CLASS_NAME, "traceInstructions", "(Z)V", trace_instructions);
        registry.register(CLASS_NAME, "traceMethodCalls", "(Z)V", trace_method_calls);
    }

    registry.register(
        CLASS_NAME,
        "availableProcessors",
        "()I",
        available_processors,
    );
    registry.register(CLASS_NAME, "freeMemory", "()J", free_memory);
    registry.register(CLASS_NAME, "gc", "()V", gc);
    registry.register(CLASS_NAME, "maxMemory", "()J", max_memory);
    registry.register(CLASS_NAME, "totalMemory", "()J", total_memory);
}

#[async_recursion(?Send)]
async fn available_processors(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let sys = System::new_all();
    let cpus = sys.physical_core_count().unwrap_or(1);
    let cpus = i32::try_from(cpus)?;
    Ok(Some(Value::Int(cpus)))
}

#[async_recursion(?Send)]
async fn free_memory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
async fn gc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn max_memory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let sys = System::new_all();
    let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
    let total_memory = i64::try_from(total_memory)?;
    Ok(Some(Value::Long(total_memory)))
}

#[async_recursion(?Send)]
async fn run_finalization_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Runtime.runFinalization0()V")
}

#[async_recursion(?Send)]
async fn trace_instructions(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Runtime.traceInstructions(Z)V")
}

#[async_recursion(?Send)]
async fn trace_method_calls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Runtime.traceMethodCalls(Z)V")
}

#[async_recursion(?Send)]
async fn total_memory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_available_processors() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = available_processors(thread, Parameters::default()).await?;
        let available_processors = result.unwrap_or(Value::Int(0)).to_int()?;
        assert!(available_processors >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory(thread, Parameters::default()).await?;
        let free_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(free_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_gc() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = gc(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_max_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_memory(thread, Parameters::default()).await?;
        let max_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(max_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Runtime.runFinalization0()V")]
    async fn test_run_finalization_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = run_finalization_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Runtime.traceInstructions(Z)V")]
    async fn test_trace_instructions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = trace_instructions(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Runtime.traceMethodCalls(Z)V")]
    async fn test_trace_method_calls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = trace_method_calls(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_total_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = total_memory(thread, Parameters::default()).await?;
        let total_memory = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(total_memory >= 1);
        Ok(())
    }
}
