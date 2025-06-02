use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/TimeZone";

/// Register all intrinsic methods for `java.util.TimeZone`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getSystemGMTOffsetID",
        "()Ljava/lang/String;",
        get_system_gmt_offset_id,
    );
    registry.register(
        CLASS_NAME,
        "getSystemTimeZoneID",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_system_time_zone_id,
    );
}

#[async_recursion(?Send)]
async fn get_system_gmt_offset_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.TimeZone.getSystemGMTOffsetID()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_system_time_zone_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.TimeZone.getSystemGMTOffsetID()Ljava/lang/String;"
    )]
    async fn test_get_system_gmt_offset_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_gmt_offset_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_time_zone_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_time_zone_id(thread, Parameters::default()).await;
    }
}
