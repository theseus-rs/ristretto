use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/NativeThread.current()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn current<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let id_val: i64 = tokio::task::try_id().map_or(1, |id| format!("{id}").parse().unwrap_or(1));
    Ok(Some(Value::Long(id_val)))
}

#[intrinsic_method("sun/nio/ch/NativeThread.current0()J", GreaterThan(JAVA_17))]
#[async_method]
pub async fn current_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    current(thread, parameters).await
}

#[intrinsic_method("sun/nio/ch/NativeThread.init()V", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn signal<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal0(J)V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn signal_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    signal(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/ch/NativeThread.supportPendingSignals0()Z",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn support_pending_signals_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_current_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current_0(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_signal_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_support_pending_signals_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = support_pending_signals_0(thread, Parameters::default())
            .await?
            .expect("value");
        let value = value.as_bool().expect("bool");
        assert!(!value);
        Ok(())
    }
}
