use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::cmp::min;
use std::sync::Arc;
use sysinfo::System;

#[intrinsic_method("java/lang/Runtime.availableProcessors()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn available_processors(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let cpus = System::physical_core_count().unwrap_or(1);
    let cpus = i32::try_from(cpus)?;
    Ok(Some(Value::Int(cpus)))
}

#[intrinsic_method("java/lang/Runtime.freeMemory()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn free_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let sys = System::new_all();
    let free_memory = sys.total_memory() - sys.used_memory();
    let free_memory = if free_memory > u64::try_from(i64::MAX)? {
        i64::MAX
    } else {
        i64::try_from(free_memory)?
    };
    Ok(Some(Value::Long(free_memory)))
}

#[intrinsic_method("java/lang/Runtime.gc()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn gc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Runtime.maxMemory()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn max_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let sys = System::new_all();
    let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
    let total_memory = i64::try_from(total_memory)?;
    Ok(Some(Value::Long(total_memory)))
}

#[intrinsic_method("java/lang/Runtime.runFinalization0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn run_finalization_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Runtime.traceInstructions(Z)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn trace_instructions(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Runtime.traceInstructions(Z)V")
}

#[intrinsic_method("java/lang/Runtime.traceMethodCalls(Z)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn trace_method_calls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Runtime.traceMethodCalls(Z)V")
}

#[intrinsic_method("java/lang/Runtime.totalMemory()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn total_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    async fn test_run_finalization_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = run_finalization_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
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
