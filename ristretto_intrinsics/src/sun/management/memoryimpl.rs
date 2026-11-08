use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
pub fn get_memory_managers_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
    Any
)]
pub fn get_memory_pools_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;",
    Any
)]
pub async fn get_memory_usage_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let heap = parameters.pop_bool()?;
    let statistics = thread.vm()?.garbage_collector().statistics()?;
    let observed_bytes = i64::try_from(statistics.bytes_allocated)?;

    // The VM has no memory pools or heap cap. Report observed bytes as used and committed,
    // leaving `max` undefined; consumers treat `-1` as unset.
    let (init, used, committed, max) = if heap {
        (0, observed_bytes, observed_bytes, -1)
    } else {
        (-1, 0, 0, -1)
    };
    let values = [
        Value::Long(init),
        Value::Long(used),
        Value::Long(committed),
        Value::Long(max),
    ];
    let memory_usage = thread
        .object("java.lang.management.MemoryUsage", "JJJJ", &values)
        .await?;
    Ok(Some(memory_usage))
}

#[intrinsic_method("sun/management/MemoryImpl.setVerboseGC(Z)V", Any)]
pub fn set_verbose_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.management.MemoryImpl.setVerboseGC(Z)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_managers_0(thread, Parameters::default());
        assert_eq!(
            "sun.management.MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_pools_0(thread, Parameters::default());
        assert_eq!(
            "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_memory_usage_0_heap() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_memory_usage_0(thread, Parameters::new(vec![Value::from(true)]))
            .await?
            .expect("heap usage");
        let usage = result.as_object_ref()?;
        assert_eq!("java/lang/management/MemoryUsage", usage.class().name());

        let init = usage.value("init")?.as_i64()?;
        let used = usage.value("used")?.as_i64()?;
        let committed = usage.value("committed")?.as_i64()?;
        let max = usage.value("max")?.as_i64()?;
        assert_eq!(0, init);
        assert!(used >= 0);
        assert!(committed >= used);
        assert_eq!(-1, max);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_memory_usage_0_non_heap() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_memory_usage_0(thread, Parameters::new(vec![Value::from(false)]))
            .await?
            .expect("non-heap usage");
        let usage = result.as_object_ref()?;
        assert_eq!("java/lang/management/MemoryUsage", usage.class().name());

        let init = usage.value("init")?.as_i64()?;
        let used = usage.value("used")?.as_i64()?;
        let committed = usage.value("committed")?.as_i64()?;
        let max = usage.value("max")?.as_i64()?;
        assert_eq!((-1, 0, 0, -1), (init, used, committed, max));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_verbose_gc(thread, Parameters::new(vec![Value::from(false)]));
        assert_eq!(
            "sun.management.MemoryImpl.setVerboseGC(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
