use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/util/TimeZone.getSystemGMTOffsetID()Ljava/lang/String;", Any)]
#[async_method]
pub(crate) async fn get_system_gmt_offset_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.TimeZone.getSystemGMTOffsetID()Ljava/lang/String;")
}

#[intrinsic_method(
    "java/util/TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub(crate) async fn get_system_time_zone_id(
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
