use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.management.GarbageCollectorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/GarbageCollectorImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "setNotificationEnabled",
            "(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
            set_notification_enabled,
        );
    }

    registry.register(
        class_name,
        "getCollectionCount",
        "()J",
        get_collection_count,
    );
    registry.register(class_name, "getCollectionTime", "()J", get_collection_time);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_collection_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_collection_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_notification_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}
