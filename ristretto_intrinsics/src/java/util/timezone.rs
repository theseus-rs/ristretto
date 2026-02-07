use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/util/TimeZone.getSystemGMTOffsetID()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_system_gmt_offset_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // This method returns null to indicate that the system doesn't have a fixed GMT offset ID. The
    // Java TimeZone class will then use the IANA timezone ID to determine the offset dynamically.
    // Returning null is the correct behavior for most systems.
    Ok(None)
}

#[intrinsic_method(
    "java/util/TimeZone.getSystemTimeZoneID(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_system_time_zone_id<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Get the system timezone ID (e.g., "America/New_York", "Europe/London")
    let timezone_id = jiff::tz::TimeZone::system()
        .iana_name()
        .map_or_else(|| "UTC".to_string(), ToString::to_string);
    let result = timezone_id.to_object(&thread).await?;
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_system_gmt_offset_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_gmt_offset_id(thread, Parameters::default())
            .await
            .expect("result");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_system_time_zone_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_time_zone_id(thread, Parameters::default())
            .await
            .expect("result");
        assert!(result.is_some());
    }
}
