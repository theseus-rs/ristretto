use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_collection_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
#[async_method]
pub async fn get_memory_managers_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.management.MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;".to_string()).into())
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_peak_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/MemoryPoolImpl.resetPeakUsage0()V", Any)]
#[async_method]
pub async fn reset_peak_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.resetPeakUsage0()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/MemoryPoolImpl.setCollectionThreshold0(JJ)V", Any)]
#[async_method]
pub async fn set_collection_threshold_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_threshold = parameters.pop_long()?;
    let _current = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.setCollectionThreshold0(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V",
    Any
)]
#[async_method]
pub async fn set_pool_collection_sensor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V",
    Any
)]
#[async_method]
pub async fn set_pool_usage_sensor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/MemoryPoolImpl.setUsageThreshold0(JJ)V", Any)]
#[async_method]
pub async fn set_usage_threshold_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_threshold = parameters.pop_long()?;
    let _current = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryPoolImpl.setUsageThreshold0(JJ)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_collection_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_collection_usage_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_managers_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_peak_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_peak_usage_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_usage_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_peak_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_peak_usage_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.resetPeakUsage0()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_collection_threshold_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_collection_threshold_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.setCollectionThreshold0(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_pool_collection_sensor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_pool_collection_sensor(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_pool_usage_sensor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_pool_usage_sensor(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_usage_threshold_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_usage_threshold_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.management.MemoryPoolImpl.setUsageThreshold0(JJ)V",
            result.unwrap_err().to_string()
        );
    }
}
