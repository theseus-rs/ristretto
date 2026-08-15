use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM as _};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HprofEventKind {
    Call,
    NewArray,
    ObjectInit,
    Return,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HprofEvent {
    kind: HprofEventKind,
    object: usize,
    related: usize,
    class_index: i32,
    trace_index: i32,
}

#[derive(Debug, Default)]
struct HprofTrackerState(Mutex<Vec<HprofEvent>>);

fn tracker_state<T: Thread + 'static>(thread: &T) -> Result<Arc<HprofTrackerState>> {
    thread
        .vm()?
        .resource_manager()
        .get_or_init(HprofTrackerState::default)
}

fn reference_identity(
    reference: Option<
        ristretto_gc::Gc<ristretto_gc::sync::RwLock<ristretto_classloader::Reference>>,
    >,
) -> usize {
    reference.map_or(0, |reference| reference.as_ptr() as usize)
}

fn record<T: Thread + 'static>(thread: &T, event: HprofEvent) -> Result<()> {
    tracker_state(thread)?
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(event);
    Ok(())
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeCallSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_call_site<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let trace_index = parameters.pop_int()?;
    let class_index = parameters.pop_int()?;
    let object = reference_identity(parameters.pop_reference()?);
    record(
        thread.as_ref(),
        HprofEvent {
            kind: HprofEventKind::Call,
            object,
            related: 0,
            class_index,
            trace_index,
        },
    )?;
    Ok(None)
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_new_array<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let related = reference_identity(parameters.pop_reference()?);
    let object = reference_identity(parameters.pop_reference()?);
    record(
        thread.as_ref(),
        HprofEvent {
            kind: HprofEventKind::NewArray,
            object,
            related,
            class_index: 0,
            trace_index: 0,
        },
    )?;
    Ok(None)
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_object_init<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let related = reference_identity(parameters.pop_reference()?);
    let object = reference_identity(parameters.pop_reference()?);
    record(
        thread.as_ref(),
        HprofEvent {
            kind: HprofEventKind::ObjectInit,
            object,
            related,
            class_index: 0,
            trace_index: 0,
        },
    )?;
    Ok(None)
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeReturnSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_return_site<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let trace_index = parameters.pop_int()?;
    let class_index = parameters.pop_int()?;
    let object = reference_identity(parameters.pop_reference()?);
    record(
        thread.as_ref(),
        HprofEvent {
            kind: HprofEventKind::Return,
            object,
            related: 0,
            class_index,
            trace_index,
        },
    )?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_call_site() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_call_site(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(None, result.expect("result"));
    }

    #[tokio::test]
    async fn test_native_new_array() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_new_array(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(None, result.expect("result"));
    }

    #[tokio::test]
    async fn test_native_object_init() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_object_init(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(None, result.expect("result"));
    }

    #[tokio::test]
    async fn test_native_return_site() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_return_site(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(None, result.expect("result"));
    }
}
