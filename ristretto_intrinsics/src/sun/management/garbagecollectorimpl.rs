use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionCount()J", Any)]
pub fn get_collection_count<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let statistics = thread.vm()?.garbage_collector().statistics()?;
    let count = i64::try_from(statistics.collections_completed)?;
    Ok(Some(Value::Long(count)))
}

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionTime()J", Any)]
pub fn get_collection_time<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let statistics = thread.vm()?.garbage_collector().statistics()?;
    let time = i64::try_from(statistics.total_collection_time.as_millis())?;
    Ok(Some(Value::Long(time)))
}

#[intrinsic_method(
    "sun/management/GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    LessThanOrEqual(JAVA_8)
)]
pub fn set_notification_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.management.GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_collection_count() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let before = collector.statistics().expect("statistics");
        let result = get_collection_count(thread, Parameters::default())
            .expect("result")
            .expect("count");
        let after = collector.statistics().expect("statistics");
        let count = usize::try_from(result.as_i64().expect("long")).expect("count");
        assert!(before.collections_completed <= count);
        assert!(count <= after.collections_completed);
    }

    #[tokio::test]
    async fn test_get_collection_time() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let collector = vm.garbage_collector();
        let before = collector.statistics().expect("statistics");
        let result = get_collection_time(thread, Parameters::default())
            .expect("result")
            .expect("time");
        let after = collector.statistics().expect("statistics");
        let time = u128::try_from(result.as_i64().expect("long")).expect("time");
        assert!(before.total_collection_time.as_millis() <= time);
        assert!(time <= after.total_collection_time.as_millis());
    }

    #[tokio::test]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_notification_enabled(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        );
        assert_eq!(
            "sun.management.GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
