use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.accept0(JJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.accept0(JJ)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.attach0(Ljava/lang/String;J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn attach0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    let _address = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.attach0(Ljava/lang/String;J)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.initialize()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.initialize()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.name(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.name(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.startListening0(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn start_listening0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.startListening0(Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "com/sun/tools/jdi/SharedMemoryTransportService.stopListening0(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn stop_listening0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/tools/jdi/SharedMemoryTransportService.stopListening0(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.accept0(JJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_attach0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = attach0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.attach0(Ljava/lang/String;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.initialize()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.name(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start_listening0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_listening0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.startListening0(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_stop_listening0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = stop_listening0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com/sun/tools/jdi/SharedMemoryTransportService.stopListening0(J)V",
            result.unwrap_err().to_string()
        );
    }
}
