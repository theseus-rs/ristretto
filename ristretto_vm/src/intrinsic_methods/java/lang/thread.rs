use crate::JavaError::NullPointerException;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_24};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
use std::time::Duration;

#[intrinsic_method("java/lang/Thread.clearInterruptEvent()V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn clear_interrupt_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.countStackFrames()I", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn count_stack_frames(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    let frames = i32::try_from(frames.len())?;
    Ok(Some(Value::Int(frames)))
}

#[intrinsic_method(
    "java/lang/Thread.currentCarrierThread()Ljava/lang/Thread;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn current_carrier_thread(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: correct this once threading is implemented
    current_thread(thread, parameters).await
}

#[intrinsic_method("java/lang/Thread.currentThread()Ljava/lang/Thread;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn current_thread(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let thread = thread.java_object().await;
    Ok(Some(thread))
}

#[intrinsic_method(
    "java/lang/Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn dump_threads(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;")
}

#[intrinsic_method(
    "java/lang/Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "java/lang/Thread.findScopedValueBindings()Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn find_scoped_value_bindings(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.findScopedValueBindings()Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/Thread.getNextThreadIdOffset()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_next_thread_id_offset(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let thread_id = vm.next_thread_id()?;
    let thread_id = i64::try_from(thread_id)?;
    Ok(Some(Value::from(thread_id)))
}

#[intrinsic_method(
    "java/lang/Thread.getStackTrace0()Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_stack_trace_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.getStackTrace0()Ljava/lang/Object;")
}

#[intrinsic_method("java/lang/Thread.getThreads()[Ljava/lang/Thread;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_threads(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.getThreads()[Ljava/lang/Thread;")
}

#[intrinsic_method("java/lang/Thread.holdsLock(Ljava/lang/Object;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn holds_lock(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.holdsLock(Ljava/lang/Object;)Z")
}

#[intrinsic_method("java/lang/Thread.interrupt0()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn interrupt_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.interrupt0()V")
}

#[intrinsic_method("java/lang/Thread.isAlive()Z", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_alive(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let object: Object = thread.java_object().await.try_into()?;
    let eetop = object.value("eetop")?.to_long()?;
    let is_alive = eetop != 0;
    Ok(Some(Value::from(is_alive)))
}

#[intrinsic_method("java/lang/Thread.isInterrupted(Z)Z", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_interrupted(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.isInterrupted(Z)Z")
}

#[intrinsic_method("java/lang/Thread.registerNatives()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.resume0()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn resume_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.resume0()V")
}

#[intrinsic_method(
    "java/lang/Thread.scopedValueCache()[Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn scoped_value_cache(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.scopedValueCache()[Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/Thread.setCurrentThread(Ljava/lang/Thread;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_current_thread(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setCurrentThread(Ljava/lang/Thread;)V")
}

#[intrinsic_method("java/lang/Thread.setNativeName(Ljava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_native_name(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(Reference::Object(name)) = parameters.pop_reference()? else {
        return Err(NullPointerException("name cannot be null".to_string()).into());
    };
    let name: String = name.try_into()?;
    thread.set_name(name).await;
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.setPriority0(I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_priority_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_priority = parameters.pop_int()?;
    // TODO: implement priority if/when tokio supports it
    Ok(None)
}

#[intrinsic_method(
    "java/lang/Thread.setScopedValueCache([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_scoped_value_cache(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setScopedValueCache([Ljava/lang/Object;)V")
}

#[intrinsic_method("java/lang/Thread.sleep(J)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sleep(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let millis = parameters.pop_long()?;
    let millis = u64::try_from(millis)?;
    let duration = Duration::from_millis(millis);
    #[cfg(not(target_family = "wasm"))]
    tokio::time::sleep(duration).await;
    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.sleep0(J)V", Equal(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn sleep_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    sleep(thread, parameters).await
}

#[intrinsic_method("java/lang/Thread.sleepNanos0(J)V", GreaterThanOrEqual(JAVA_24))]
#[async_recursion(?Send)]
pub(crate) async fn sleep_nanos_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nanos = parameters.pop_long()?;
    let nanos = u64::try_from(nanos)?;
    let duration = Duration::from_nanos(nanos);
    #[cfg(not(target_family = "wasm"))]
    tokio::time::sleep(duration).await;
    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.start0()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn start_0(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let thread_id = i64::try_from(thread.id())?;
    let object: Object = thread.java_object().await.try_into()?;
    object.set_value("eetop", Value::from(thread_id))?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/Thread.stop0(Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn stop_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Thread.stop0(Ljava/lang/Object;)V")
}

#[intrinsic_method("java/lang/Thread.suspend0()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn suspend_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.suspend0()V")
}

#[intrinsic_method("java/lang/Thread.yield()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn r#yield(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    tokio::task::yield_now().await;
    #[cfg(target_family = "wasm")]
    std::thread::yield_now();
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.yield0()V", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn yield_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    r#yield(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear_interrupt_event() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = clear_interrupt_event(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_count_stack_frames() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = count_stack_frames(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_current_carrier_thread() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = current_carrier_thread(thread, Parameters::default()).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_current_thread() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = current_thread(thread, Parameters::default()).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;"
    )]
    async fn test_dump_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_threads(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V"
    )]
    async fn test_ensure_materialized_for_stack_walk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ensure_materialized_for_stack_walk(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.findScopedValueBindings()Ljava/lang/Object;"
    )]
    async fn test_find_scoped_value_bindings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_scoped_value_bindings(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_next_thread_id_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_next_thread_id_offset(thread, Parameters::default()).await?;
        let thread_id = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(thread_id > 0);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.getStackTrace0()Ljava/lang/Object;"
    )]
    async fn test_get_stack_trace_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.getThreads()[Ljava/lang/Thread;"
    )]
    async fn test_get_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_threads(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.holdsLock(Ljava/lang/Object;)Z"
    )]
    async fn test_holds_lock() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = holds_lock(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Thread.interrupt0()V")]
    async fn test_interrupt_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Thread.isInterrupted(Z)Z")]
    async fn test_is_interrupted() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_interrupted(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Thread.resume0()V")]
    async fn test_resume_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = resume_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.scopedValueCache()[Ljava/lang/Object;"
    )]
    async fn test_scoped_value_cache() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = scoped_value_cache(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.setCurrentThread(Ljava/lang/Thread;)V"
    )]
    async fn test_set_current_thread() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_current_thread(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_set_priority_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let priority = Value::Int(0);
        let result = set_priority_0(thread, Parameters::new(vec![priority])).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Thread.setScopedValueCache([Ljava/lang/Object;)V"
    )]
    async fn test_set_scoped_value_cache() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_scoped_value_cache(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_sleep() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let millis = Value::Long(250);
        let start = std::time::Instant::now();
        let result = sleep(thread, Parameters::new(vec![millis])).await?;
        let elapsed = start.elapsed();
        assert_eq!(result, None);
        assert!(elapsed.as_millis() > 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_sleep_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let millis = Value::Long(250);
        let start = std::time::Instant::now();
        let result = sleep_0(thread, Parameters::new(vec![millis])).await?;
        let elapsed = start.elapsed();
        assert_eq!(result, None);
        assert!(elapsed.as_millis() > 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_sleep_nanos_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let millis = Value::Long(1000);
        let start = std::time::Instant::now();
        let result = sleep_nanos_0(thread, Parameters::new(vec![millis])).await?;
        let elapsed = start.elapsed();
        assert_eq!(result, None);
        assert!(elapsed.as_nanos() > 500);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Thread.stop0(Ljava/lang/Object;)V")]
    async fn test_stop_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = stop_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Thread.suspend0()V")]
    async fn test_suspend_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = suspend_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_yield() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = r#yield(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_yield_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = yield_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
