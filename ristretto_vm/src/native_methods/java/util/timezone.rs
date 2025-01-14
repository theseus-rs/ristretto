use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.TimeZone`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/TimeZone";
    registry.register(
        class_name,
        "getSystemGMTOffsetID",
        "()Ljava/lang/String;",
        get_system_gmt_offset_id,
    );
    registry.register(
        class_name,
        "getSystemTimeZoneID",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_time_zone_id,
    );
}

#[async_recursion(?Send)]
async fn get_system_gmt_offset_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.TimeZone.getSystemGMTOffsetID()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_system_time_zone_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/util/TimeZone";
        assert!(registry
            .method(class_name, "getSystemGMTOffsetID", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getSystemTimeZoneID",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.TimeZone.getSystemGMTOffsetID()Ljava/lang/String;"
    )]
    async fn test_get_system_gmt_offset_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_gmt_offset_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_time_zone_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_time_zone_id(thread, Arguments::default()).await;
    }
}
