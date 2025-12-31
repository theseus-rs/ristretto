use crate::JavaError::{NullPointerException, RuntimeException, UnsupportedOperationException};
use crate::Result;
use crate::handles::ThreadHandle;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
use std::time::Duration;
#[cfg(not(target_family = "wasm"))]
use thread_priority::{ThreadPriority, ThreadPriorityValue, set_current_thread_priority};
use tokio::runtime::Builder;
use tracing::error;

/// Get the thread from the thread ID in the `eetop` field of the thread object.
async fn get_thread(thread: &Arc<Thread>, thread_object: &Value) -> Result<Arc<Thread>> {
    let thread_id = {
        let thread_object = thread_object.as_object_ref()?;
        thread_object.value("eetop")?.as_i64()?
    };
    let thread_id = u64::try_from(thread_id)?;
    let vm = thread.vm()?;
    let thread_handles = vm.thread_handles();
    let Some(thread_handle) = thread_handles.get(&thread_id).await else {
        return Err(RuntimeException(format!("Thread not found for id {thread_id}")).into());
    };
    Ok(thread_handle.thread.clone())
}

#[intrinsic_method("java/lang/Thread.clearInterruptEvent()V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn clear_interrupt_event(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    let instance_thread = get_thread(&thread, &thread_object).await?;
    let _ = instance_thread.is_interrupted(true);
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
    Ok(None)
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
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let thread_handles = vm.thread_handles().read().await;
    let mut threads = Vec::new();

    for (_id, vm_thread_handle) in thread_handles.iter() {
        let vm_thread = &vm_thread_handle.thread;
        let thread_object = vm_thread.java_object().await;
        threads.push(thread_object);
    }

    let thread_class = thread.class("java/lang/Thread").await?;
    let threads = (thread_class, threads).try_into()?;
    Ok(Some(threads))
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
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    let instance_thread = get_thread(&thread, &thread_object).await?;
    instance_thread.interrupt();
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.isAlive()Z", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_alive(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    let is_alive = get_thread(&thread, &thread_object).await.is_ok();
    Ok(Some(Value::from(is_alive)))
}

#[intrinsic_method("java/lang/Thread.isInterrupted(Z)Z", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_interrupted(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let clear_interrupt = parameters.pop_bool()?;
    let thread_object = parameters.pop()?;
    let instance_thread = get_thread(&thread, &thread_object).await?;
    let was_interrupted = instance_thread.is_interrupted(clear_interrupt);
    Ok(Some(Value::from(was_interrupted)))
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
    Err(
        UnsupportedOperationException("java/lang/Thread.resume0()V is not supported".to_string())
            .into(),
    )
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
    let name = parameters.pop()?;
    if name.is_null() {
        return Err(NullPointerException(Some("name cannot be null".to_string())).into());
    }
    let name = name.as_string()?;
    thread.set_name(name).await;
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.setPriority0(I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_priority_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_priority = parameters.pop_int()?;

    #[cfg(target_family = "wasm")]
    {
        let _ = new_priority;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let priority: ThreadPriority = match new_priority {
            ..=1 => ThreadPriority::Min,
            2..=9 => {
                let priority: u8;

                #[cfg(target_os = "macos")]
                {
                    priority = u8::try_from(new_priority)?
                        .saturating_mul(4)
                        .saturating_add(7);
                }

                #[cfg(target_os = "windows")]
                {
                    priority = match new_priority {
                        2 => 1,
                        3..=4 => 2,
                        5 => 3,
                        6 => 4,
                        7..=8 => 5,
                        _ => 6,
                    };
                }

                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    priority = u8::try_from(new_priority)?.saturating_mul(8);
                }

                let priority_value: ThreadPriorityValue = priority.try_into().map_err(|error| {
                    RuntimeException(format!("Unable to determine thread priority: {error}"))
                })?;
                ThreadPriority::Crossplatform(priority_value)
            }
            _ => ThreadPriority::Max,
        };
        set_current_thread_priority(priority)
            .map_err(|error| RuntimeException(format!("Unable to set thread priority: {error}")))?;
    }

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
    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);
    #[cfg(not(target_family = "wasm"))]
    tokio::time::sleep(duration).await;
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.sleep0(J)V", Equal(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn sleep_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    sleep(thread, parameters).await
}

#[intrinsic_method("java/lang/Thread.sleepNanos0(J)V", GreaterThanOrEqual(JAVA_25))]
#[async_recursion(?Send)]
pub(crate) async fn sleep_nanos_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nanos = parameters.pop_long()?;
    let nanos = u64::try_from(nanos)?;
    let duration = Duration::from_nanos(nanos);
    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);
    #[cfg(not(target_family = "wasm"))]
    tokio::time::sleep(duration).await;
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.start0()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn start_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    if !matches!(
        std::env::var("RISTRETTO_THREADING"),
        Ok(v) if matches!(&*v.to_ascii_lowercase(), "1" | "true" | "yes" | "on")
    ) {
        return Ok(None);
    }

    let thread_object = parameters.pop()?;
    let (thread_class, thread_id) = {
        let mut thread_object = thread_object.as_object_mut()?;
        let thread_class = thread_object.class().clone();
        let thread_id = thread_object.value("tid")?.as_u64()?;
        // Associate the Java Thread object with the new internal thread and set its ID in `eetop`
        thread_object.set_value("eetop", Value::from(thread_id))?;
        (thread_class, thread_id)
    };
    let run_method = thread_class.try_get_method("run", "()V")?;
    let thread_value = thread_object.clone();

    // Create a new internal thread (Arc<Thread>) and registered with the VM
    let vm = thread.vm()?;
    let weak_vm = Arc::downgrade(&vm);
    let new_thread = Thread::new(&weak_vm, thread_id);
    new_thread.set_java_object(thread_object.clone()).await;

    // Spawn a new OS thread to run the target's run() method in a Tokio runtime
    let spawn_vm = vm.clone();
    let spawn_thread = new_thread.clone();
    let join_handle = std::thread::spawn({
        move || {
            let runtime = Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime for new thread");
            // Execute the Runnable's run() method within the new thread's context
            runtime.block_on(async move {
                let _ = spawn_thread
                    .execute(&thread_class, &run_method, &[thread_value])
                    .await;
                // Set `eetop` to 0 after execution to indicate the thread has finished
                {
                    let mut thread_object = match thread_object.as_object_mut() {
                        Ok(thread_object) => thread_object,
                        Err(error) => {
                            error!("Failed to get thread object: {error:?}");
                            return;
                        }
                    };
                    if let Err(error) = thread_object.set_value("eetop", Value::Long(0)) {
                        error!("Failed to set eetop to 0: {error}");
                    }
                }
                // Remove the thread from the VM's thread handles; this drops the JoinHandle which
                // in turn will drop the OS thread.
                let thread_handles = spawn_vm.thread_handles();
                thread_handles.remove(&thread_id).await;
            });
        }
    });

    let thread_handle = ThreadHandle::from((new_thread, join_handle));
    let thread_handles = vm.thread_handles();
    thread_handles.insert(thread_id, thread_handle).await?;

    Ok(None)
}

#[intrinsic_method(
    "java/lang/Thread.stop0(Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn stop_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Err(UnsupportedOperationException(
        "java/lang/Thread.stop0(Ljava/lang/Object;)V is not supported".to_string(),
    )
    .into())
}

#[intrinsic_method("java/lang/Thread.suspend0()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn suspend_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        UnsupportedOperationException("java/lang/Thread.suspend0()V is not supported".to_string())
            .into(),
    )
}

#[intrinsic_method("java/lang/Thread.yield()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn r#yield(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        tokio::task::yield_now().await;
    }

    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        std::thread::yield_now();
    }

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
    use crate::VM;
    use crate::handles::ThreadHandle;

    /// Helper function to create a thread object for testing
    async fn create_thread(vm: &Arc<VM>) -> Result<Value> {
        let thread_value = vm.object("java/lang/Thread", "", &[] as &[Value]).await?;
        let thread_id = vm.next_thread_id()?;
        let weak_vm = Arc::downgrade(vm);
        let thread = Thread::new(&weak_vm, thread_id);
        let thread_handle = ThreadHandle::from(thread);
        let mut thread_handles = vm.thread_handles().write().await;
        thread_handles.insert(thread_id, thread_handle);
        {
            let mut thread_object = thread_value.as_object_mut()?;
            thread_object.set_value("eetop", Value::from(thread_id))?;
            thread_object.set_value("tid", Value::from(thread_id))?;
        }
        Ok(thread_value)
    }

    #[tokio::test]
    async fn test_clear_interrupt_event() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let thread_instance = create_thread(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(thread_instance);
        let result = clear_interrupt_event(thread, parameters).await?;
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
    async fn test_ensure_materialized_for_stack_walk() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::default();
        let value = ensure_materialized_for_stack_walk(thread, parameters)
            .await
            .expect("value");
        assert!(value.is_none());
        Ok(())
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
        let thread_id = result.unwrap_or(Value::Long(0)).as_i64()?;
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
    async fn test_get_threads() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_threads(thread, Parameters::default())
            .await?
            .expect("threads");
        let threads: Vec<Value> = value.try_into()?;
        assert_eq!(threads.len(), 1);
        Ok(())
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
    async fn test_interrupt_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let thread_instance = create_thread(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(thread_instance);
        let result = interrupt_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_interrupted() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let thread_instance = create_thread(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(thread_instance.clone());
        parameters.push(Value::from(false)); // clear interrupt
        let result = is_interrupted(thread.clone(), parameters)
            .await?
            .expect("was_cleared");
        let was_interrupted = result.as_bool()?;
        assert!(!was_interrupted);

        let mut parameters = Parameters::default();
        parameters.push(thread_instance.clone());
        let result = interrupt_0(thread.clone(), parameters).await?;
        assert_eq!(result, None);

        let mut parameters = Parameters::default();
        parameters.push(thread_instance);
        parameters.push(Value::from(true)); // clear interrupt
        let result = is_interrupted(thread, parameters)
            .await?
            .expect("was_cleared");
        let was_interrupted = result.as_bool()?;
        assert!(was_interrupted);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_resume_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = resume_0(thread, Parameters::default()).await;
        assert!(result.is_err());
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
        let (vm, thread) = crate::test::thread().await?;
        let thread_instance = create_thread(&vm).await?;
        let mut parameters = Parameters::default();
        parameters.push(thread_instance);
        parameters.push(Value::Int(7)); // newPriority
        let result = set_priority_0(thread, parameters).await?;
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
    async fn test_stop_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = stop_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_suspend_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = suspend_0(thread, Parameters::default()).await;
        assert!(result.is_err());
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
