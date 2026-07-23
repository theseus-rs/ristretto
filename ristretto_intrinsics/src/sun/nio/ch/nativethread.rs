use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Error::InternalError, Parameters, Result, VM};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
struct NativeSignals {
    pending: Mutex<HashMap<i64, Arc<AtomicBool>>>,
}

impl NativeSignals {
    fn state(&self, id: i64) -> Result<Arc<AtomicBool>> {
        let mut pending = self
            .pending
            .lock()
            .map_err(|error| InternalError(error.to_string()))?;
        Ok(pending
            .entry(id)
            .or_insert_with(|| Arc::new(AtomicBool::new(false)))
            .clone())
    }
}

fn thread_id<T: Thread + ?Sized>(thread: &T) -> Result<i64> {
    i64::try_from(thread.id()).map_err(|error| InternalError(error.to_string()))
}

fn signals<V: VM + ?Sized>(vm: &V) -> Result<Arc<NativeSignals>> {
    vm.resource_manager().get_or_init(NativeSignals::default)
}

/// Consumes a signal sent to the supplied Java thread.
pub(crate) fn take_signal<T: Thread + ?Sized>(thread: &T) -> Result<bool> {
    let vm = thread.vm()?;
    let state = signals(&*vm)?.state(thread_id(thread)?)?;
    Ok(state.swap(false, Ordering::AcqRel))
}

#[intrinsic_method("sun/nio/ch/NativeThread.current()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn current<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let id = thread_id(&*thread)?;
    let vm = thread.vm()?;
    let _ = signals(&*vm)?.state(id)?;
    Ok(Some(Value::Long(id)))
}

#[intrinsic_method("sun/nio/ch/NativeThread.current0()J", GreaterThan(JAVA_17))]
#[async_method]
pub async fn current_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    current(thread, parameters).await
}

#[intrinsic_method("sun/nio/ch/NativeThread.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn signal<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let id = parameters.pop_long()?;
    if id != 0 {
        let vm = thread.vm()?;
        signals(&*vm)?.state(id)?.store(true, Ordering::Release);
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/NativeThread.signal0(J)V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn signal_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    signal(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/ch/NativeThread.supportPendingSignals0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn support_pending_signals_0<T: Thread + 'static>(
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
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let id = current(thread.clone(), Parameters::default())
            .await
            .expect("current")
            .expect("id")
            .as_i64()
            .expect("long");
        signal(thread.clone(), Parameters::new(vec![Value::Long(id)]))
            .await
            .expect("signal");
        assert!(take_signal(&*thread).expect("take signal"));
    }

    #[tokio::test]
    async fn test_signal_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = signal_0(thread, Parameters::default()).await;
        assert!(result.is_err());
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
