use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/MemoryPoolImpl";

/// Register all intrinsic methods for `sun.management.MemoryPoolImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getCollectionUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_collection_usage_0,
    );
    registry.register(
        CLASS_NAME,
        "getMemoryManagers0",
        "()[Ljava/lang/management/MemoryManagerMXBean;",
        get_memory_managers_0,
    );
    registry.register(
        CLASS_NAME,
        "getPeakUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_peak_usage_0,
    );
    registry.register(
        CLASS_NAME,
        "getUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_usage_0,
    );
    registry.register(CLASS_NAME, "resetPeakUsage0", "()V", reset_peak_usage_0);
    registry.register(
        CLASS_NAME,
        "setCollectionThreshold0",
        "(JJ)V",
        set_collection_threshold_0,
    );
    registry.register(
        CLASS_NAME,
        "setPoolCollectionSensor",
        "(Lsun/management/Sensor;)V",
        set_pool_collection_sensor,
    );
    registry.register(
        CLASS_NAME,
        "setPoolUsageSensor",
        "(Lsun/management/Sensor;)V",
        set_pool_usage_sensor,
    );
    registry.register(
        CLASS_NAME,
        "setUsageThreshold0",
        "(JJ)V",
        set_usage_threshold_0,
    );
}

#[async_recursion(?Send)]
async fn get_collection_usage_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getCollectionUsage0()Ljava/lang/management/MemoryUsage;")
}

#[async_recursion(?Send)]
async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryPoolImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;"
    )
}

#[async_recursion(?Send)]
async fn get_peak_usage_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getPeakUsage0()Ljava/lang/management/MemoryUsage;")
}

#[async_recursion(?Send)]
async fn get_usage_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.getUsage0()Ljava/lang/management/MemoryUsage;")
}

#[async_recursion(?Send)]
async fn reset_peak_usage_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.resetPeakUsage0()V")
}

#[async_recursion(?Send)]
async fn set_collection_threshold_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setCollectionThreshold0(JJ)V")
}

#[async_recursion(?Send)]
async fn set_pool_collection_sensor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setPoolCollectionSensor(Lsun/management/Sensor;)V")
}

#[async_recursion(?Send)]
async fn set_pool_usage_sensor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.MemoryPoolImpl.setPoolUsageSensor(Lsun/management/Sensor;)V")
}

#[async_recursion(?Send)]
async fn set_usage_threshold_0(
    _thread: Arc<Thread>,
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
