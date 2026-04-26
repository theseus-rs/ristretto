use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/WEPoll.close(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.close(J)V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.create()J", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.create()J".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.ctl(JIJI)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn ctl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _events = parameters.pop_int()?;
    let _s = parameters.pop_long()?;
    let _opcode = parameters.pop_int()?;
    let _h = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.ctl(JIJI)I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.dataOffset()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn data_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.dataOffset()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.eventSize()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.eventSize()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.eventsOffset()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn events_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.eventsOffset()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/WEPoll.wait(JJII)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn wait<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _numfds = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _h = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/WEPoll.wait(JJII)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/WEPoll.close(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WEPoll.create()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ctl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ctl(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WEPoll.ctl(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_data_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WEPoll.dataOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_event_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WEPoll.eventSize()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_events_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = events_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WEPoll.eventsOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WEPoll.wait(JJII)I",
            result.unwrap_err().to_string()
        );
    }
}
