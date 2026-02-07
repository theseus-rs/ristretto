use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_collection_usage_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;")
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
#[async_method]
pub async fn get_memory_managers_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_peak_usage_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;")
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;",
    Any
)]
#[async_method]
pub async fn get_usage_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;")
}

#[intrinsic_method("sun/management/MemoryPoolImpl.resetPeakUsage0()V", Any)]
#[async_method]
pub async fn reset_peak_usage_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.resetPeakUsage0()V")
}

#[intrinsic_method("sun/management/MemoryPoolImpl.setCollectionThreshold0(JJ)V", Any)]
#[async_method]
pub async fn set_collection_threshold_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setCollectionThreshold0(JJ)V")
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V",
    Any
)]
#[async_method]
pub async fn set_pool_collection_sensor<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V")
}

#[intrinsic_method(
    "sun/management/MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V",
    Any
)]
#[async_method]
pub async fn set_pool_usage_sensor<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V")
}

#[intrinsic_method("sun/management/MemoryPoolImpl.setUsageThreshold0(JJ)V", Any)]
#[async_method]
pub async fn set_usage_threshold_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setUsageThreshold0(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;"
    )]
    async fn test_get_collection_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_collection_usage_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_managers_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;"
    )]
    async fn test_get_peak_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_peak_usage_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;"
    )]
    async fn test_get_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_usage_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.resetPeakUsage0()V"
    )]
    async fn test_reset_peak_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_peak_usage_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.setCollectionThreshold0(JJ)V"
    )]
    async fn test_set_collection_threshold_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_collection_threshold_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V"
    )]
    async fn test_set_pool_collection_sensor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_pool_collection_sensor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V"
    )]
    async fn test_set_pool_usage_sensor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_pool_usage_sensor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryPoolImpl.setUsageThreshold0(JJ)V"
    )]
    async fn test_set_usage_threshold_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_usage_threshold_0(thread, Parameters::default()).await;
    }
}
