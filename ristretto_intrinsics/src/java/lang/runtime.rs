use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
#[cfg(not(target_family = "wasm"))]
use std::cmp::min;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use sysinfo::System;

#[intrinsic_method("java/lang/Runtime.availableProcessors()I", Any)]
#[async_method]
pub async fn available_processors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let cpus = System::physical_core_count().unwrap_or(1);
        let cpus = i32::try_from(cpus)?;
        Ok(Some(Value::Int(cpus)))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::Int(1)))
    }
}

#[intrinsic_method("java/lang/Runtime.freeMemory()J", Any)]
#[async_method]
pub async fn free_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let sys = System::new_all();
        let free_memory = sys.total_memory() - sys.used_memory();
        let free_memory = if free_memory > u64::try_from(i64::MAX)? {
            i64::MAX
        } else {
            i64::try_from(free_memory)?
        };
        Ok(Some(Value::Long(free_memory)))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::Long(0)))
    }
}

#[intrinsic_method("java/lang/Runtime.gc()V", Any)]
#[async_method]
pub async fn gc<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    thread.vm()?.garbage_collector().collect();
    Ok(None)
}

#[intrinsic_method("java/lang/Runtime.maxMemory()J", Any)]
#[async_method]
pub async fn max_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let sys = System::new_all();
        let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
        let total_memory = i64::try_from(total_memory)?;
        Ok(Some(Value::Long(total_memory)))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::Long(i64::MAX)))
    }
}

#[intrinsic_method("java/lang/Runtime.runFinalization0()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn run_finalization_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Runtime.traceInstructions(Z)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn trace_instructions<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("java.lang.Runtime.traceInstructions(Z)V".to_string())
            .into(),
    )
}

#[intrinsic_method("java/lang/Runtime.traceMethodCalls(Z)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn trace_method_calls<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("java.lang.Runtime.traceMethodCalls(Z)V".to_string())
            .into(),
    )
}

#[intrinsic_method("java/lang/Runtime.totalMemory()J", Any)]
#[async_method]
pub async fn total_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
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
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::Long(0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_available_processors() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = available_processors(thread, Parameters::default()).await?;
        let available_processors = result.unwrap_or(Value::Int(0)).as_i32()?;
        assert!(available_processors >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory(thread, Parameters::default()).await?;
        let free_memory = result.unwrap_or(Value::Long(0)).as_i64()?;
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
        let max_memory = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert!(max_memory >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_run_finalization_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = run_finalization_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_trace_instructions() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = trace_instructions(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java.lang.Runtime.traceInstructions(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_trace_method_calls() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = trace_method_calls(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java.lang.Runtime.traceMethodCalls(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_total_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = total_memory(thread, Parameters::default()).await?;
        let total_memory = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert!(total_memory >= 1);
        Ok(())
    }
}
