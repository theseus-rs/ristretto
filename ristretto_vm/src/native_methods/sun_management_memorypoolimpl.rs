use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.MemoryPoolImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/MemoryPoolImpl";
    registry.register(
        class_name,
        "getCollectionUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_collection_usage_0,
    );
    registry.register(
        class_name,
        "getMemoryManagers0",
        "()[Ljava/lang/management/MemoryManagerMXBean;",
        get_memory_managers_0,
    );
    registry.register(
        class_name,
        "getPeakUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_peak_usage_0,
    );
    registry.register(
        class_name,
        "getUsage0",
        "()Ljava/lang/management/MemoryUsage;",
        get_usage_0,
    );
    registry.register(class_name, "resetPeakUsage0", "()V", reset_peak_usage_0);
    registry.register(
        class_name,
        "setCollectionThreshold0",
        "(JJ)V",
        set_collection_threshold_0,
    );
    registry.register(
        class_name,
        "setPoolCollectionSensor",
        "(Lsun/management/Sensor;)V",
        set_pool_collection_sensor,
    );
    registry.register(
        class_name,
        "setPoolUsageSensor",
        "(Lsun/management/Sensor;)V",
        set_pool_usage_sensor,
    );
    registry.register(
        class_name,
        "setUsageThreshold0",
        "(JJ)V",
        set_usage_threshold_0,
    );
}

#[async_recursion(?Send)]
async fn get_collection_usage_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_memory_managers_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_peak_usage_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_usage_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn reset_peak_usage_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_collection_threshold_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_pool_collection_sensor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_pool_usage_sensor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_usage_threshold_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
