use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::JavaError::InterruptedException;
use ristretto_types::JavaError::{
    IllegalArgumentException, NullPointerException, RuntimeException, UnsupportedOperationException,
};
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::handles::ThreadHandle;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::time::Duration;
#[cfg(not(target_family = "wasm"))]
use thread_priority::{ThreadPriority, ThreadPriorityValue, set_current_thread_priority};
use tracing::error;

bitflags! {
    /// Thread state flags matching thread state values used by Java.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ThreadState: i32 {
        const TERMINATED = 0x02;
        const RUNNABLE = 0x04;
    }
}

/// Set the thread status in the thread object, handling both old and new (Java 19+) layouts.
fn set_thread_status(thread_object: &Value, status: ThreadState) -> Result<()> {
    let mut thread_object = thread_object.as_object_mut()?;
    // Try to set via holder (Java 19+), fall back to direct field (older versions)
    if let Ok(holder_value) = thread_object.value("holder")
        && let Ok(mut holder) = holder_value.as_object_mut()
    {
        holder.set_value("threadStatus", Value::Int(status.bits()))?;
        return Ok(());
    }
    // For older Java versions, set directly on Thread object
    thread_object.set_value("threadStatus", Value::Int(status.bits()))?;
    Ok(())
}

/// Get the thread from the thread ID in the `eetop` field of the thread object.
/// Returns an error if the thread is not found (which can happen if the thread has terminated).
async fn get_thread<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
    thread_object: &Value,
) -> Result<Arc<<<T as Thread>::Vm as VM>::ThreadType>> {
    let thread_id = {
        let thread_object = thread_object.as_object_ref()?;
        thread_object.value("eetop")?.as_i64()?
    };

    // eetop of 0 means the thread has terminated or not started
    if thread_id == 0 {
        return Err(RuntimeException("Thread has terminated or not started".to_string()).into());
    }

    let thread_id = u64::try_from(thread_id)?;
    let vm = thread.vm()?;
    let thread_handles = vm.thread_handles();
    let Some(thread_handle) = thread_handles.get(&thread_id).await else {
        return Err(RuntimeException(format!("Thread not found for id {thread_id}")).into());
    };
    Ok(thread_handle.thread.clone())
}

#[intrinsic_method("java/lang/Thread.clearInterruptEvent()V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn clear_interrupt_event<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    let instance_thread = get_thread(&thread, &thread_object).await?;
    let _ = instance_thread.is_interrupted(true);
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.countStackFrames()I", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn count_stack_frames<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
#[async_method]
pub async fn current_carrier_thread<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: correct this once threading is implemented
    current_thread(thread, parameters).await
}

#[intrinsic_method("java/lang/Thread.currentThread()Ljava/lang/Thread;", Any)]
#[async_method]
pub async fn current_thread<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let thread = thread.java_object().await;
    Ok(Some(thread))
}

#[intrinsic_method(
    "java/lang/Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;",
    Any
)]
#[async_method]
pub async fn dump_threads<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.dumpThreads([Ljava/lang/Thread;)[[Ljava/lang/StackTraceElement;")
}

#[intrinsic_method(
    "java/lang/Thread.ensureMaterializedForStackWalk(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ensure_materialized_for_stack_walk<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/Thread.findScopedValueBindings()Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_scoped_value_bindings<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.findScopedValueBindings()Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/Thread.getNextThreadIdOffset()J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_next_thread_id_offset<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
#[async_method]
pub async fn get_stack_trace_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.getStackTrace0()Ljava/lang/Object;")
}

#[intrinsic_method("java/lang/Thread.getThreads()[Ljava/lang/Thread;", Any)]
#[async_method]
pub async fn get_threads<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
    let reference = Reference::try_from((thread_class, threads))?;
    let threads = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(threads))
}

#[intrinsic_method("java/lang/Thread.holdsLock(Ljava/lang/Object;)Z", Any)]
#[async_method]
pub async fn holds_lock<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.holdsLock(Ljava/lang/Object;)Z")
}

#[intrinsic_method("java/lang/Thread.interrupt0()V", Any)]
#[async_method]
pub async fn interrupt_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    if let Ok(instance_thread) = get_thread(&thread, &thread_object).await {
        instance_thread.interrupt();
    }
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.isAlive()Z", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_alive<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;
    let is_alive = get_thread(&thread, &thread_object).await.is_ok();
    Ok(Some(Value::from(is_alive)))
}

#[intrinsic_method("java/lang/Thread.isInterrupted(Z)Z", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_interrupted<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let clear_interrupt = parameters.pop_bool()?;
    let thread_object = parameters.pop()?;

    if let Ok(instance_thread) = get_thread(&thread, &thread_object).await {
        let was_interrupted = instance_thread.is_interrupted(clear_interrupt);
        Ok(Some(Value::from(was_interrupted)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Thread.registerNatives()V", Any)]
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.resume0()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn resume_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn scoped_value_cache<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.scopedValueCache()[Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/Thread.setCurrentThread(Ljava/lang/Thread;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_current_thread<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setCurrentThread(Ljava/lang/Thread;)V")
}

#[intrinsic_method("java/lang/Thread.setNativeName(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_native_name<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop()?;
    if name.is_null() {
        return Err(NullPointerException(Some("name cannot be null".to_string())).into());
    }
    let name = name.as_string()?;
    thread.set_name(&name).await;
    Ok(None)
}

#[intrinsic_method("java/lang/Thread.setPriority0(I)V", Any)]
#[async_method]
pub async fn set_priority_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn set_scoped_value_cache<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Thread.setScopedValueCache([Ljava/lang/Object;)V")
}

#[intrinsic_method("java/lang/Thread.sleep(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn sleep<T: ristretto_types::Thread + 'static>(
    #[cfg_attr(target_family = "wasm", allow(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use ristretto_types::JavaError;
    let millis = parameters.pop_long()?;
    if millis < 0 {
        return Err(
            JavaError::IllegalArgumentException("timeout value is negative".to_string()).into(),
        );
    }
    let millis = u64::try_from(millis)?;
    let duration = Duration::from_millis(millis);

    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);

    #[cfg(not(target_family = "wasm"))]
    {
        let interrupted = thread.sleep(duration).await;
        if interrupted {
            return Err(JavaError::InterruptedException("sleep interrupted".to_string()).into());
        }
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Thread.sleep0(J)V", Equal(JAVA_21))]
#[async_method]
pub async fn sleep_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    sleep(thread, parameters).await
}

#[intrinsic_method("java/lang/Thread.sleepNanos0(J)V", GreaterThanOrEqual(JAVA_25))]
#[async_method]
pub async fn sleep_nanos_0<T: ristretto_types::Thread + 'static>(
    #[cfg_attr(target_family = "wasm", allow(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nanos = parameters.pop_long()?;
    if nanos < 0 {
        return Err(IllegalArgumentException("timeout value is negative".to_string()).into());
    }
    let nanos = u64::try_from(nanos)?;
    let duration = Duration::from_nanos(nanos);

    #[cfg(target_family = "wasm")]
    std::thread::sleep(duration);

    #[cfg(not(target_family = "wasm"))]
    {
        let interrupted = thread.sleep(duration).await;
        if interrupted {
            return Err(InterruptedException("sleep interrupted".to_string()).into());
        }
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Thread.start0()V", Any)]
#[expect(clippy::too_many_lines)]
#[async_method]
pub async fn start_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let thread_object = parameters.pop()?;

    // First, get the VM to generate a unique internal thread ID
    let vm = thread.vm()?;

    #[cfg_attr(target_family = "wasm", allow(unused_variables))]
    let (thread_class, internal_thread_id, is_daemon) = {
        let mut thread_object = thread_object.as_object_mut()?;
        let thread_class = thread_object.class().clone();

        // Check if thread has already been started (eetop != 0)
        let eetop = thread_object
            .value("eetop")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        if eetop != 0 {
            return Ok(None);
        }

        // Check if this is a daemon thread
        // In Java 19+, daemon is in holder.daemon; in older versions, it's directly on Thread
        let is_daemon = if let Ok(holder_value) = thread_object.value("holder")
            && let Ok(holder) = holder_value.as_object_ref()
        {
            holder
                .value("daemon")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            thread_object
                .value("daemon")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        };

        // Generate a unique internal thread ID for this execution
        let internal_thread_id = vm.next_thread_id()?;

        // Associate the Java Thread object with the new internal thread and set its ID in `eetop`
        thread_object.set_value("eetop", Value::from(internal_thread_id))?;
        (thread_class, internal_thread_id, is_daemon)
    };

    // Set thread status to RUNNABLE
    if let Err(error) = set_thread_status(&thread_object, ThreadState::RUNNABLE) {
        error!("Failed to set thread status to RUNNABLE: {error}");
    }

    let run_method = thread_class.try_get_method("run", "()V")?;
    let thread_value = thread_object.clone();

    // Create a new internal thread (Arc<Thread>) and register with the VM
    let new_thread = vm.create_thread(internal_thread_id)?;
    new_thread.set_java_object(thread_object.clone()).await;

    // Spawn a task to run the thread's run() method
    let spawn_thread = new_thread.clone();

    #[cfg(not(target_family = "wasm"))]
    {
        let join_handle = tokio::spawn(async move {
            // Use a RAII guard to ensure cleanup happens even if the thread panics
            struct ThreadCleanup {
                thread_object: Value,
            }

            impl Drop for ThreadCleanup {
                fn drop(&mut self) {
                    // Set thread status to TERMINATED and eetop to 0 after execution
                    if let Err(error) =
                        set_thread_status(&self.thread_object, ThreadState::TERMINATED)
                    {
                        error!("Failed to set thread status to TERMINATED: {error}");
                    }
                    if let Ok(mut thread_obj) = self.thread_object.as_object_mut() {
                        let result = thread_obj.set_value("eetop", Value::Long(0));
                        if let Err(error) = result {
                            error!("Failed to set eetop to 0: {error}");
                        }
                    } else {
                        error!("Failed to get mutable reference for thread object cleanup");
                    }
                }
            }

            let _cleanup = ThreadCleanup {
                thread_object: thread_object.clone(),
            };

            let _ = spawn_thread
                .execute(&thread_class, &run_method, &[thread_value])
                .await;

            // Note: We intentionally do NOT remove the thread handle here.
            // The VM's wait_for_non_daemon_threads() will await the JoinHandle
            // and clean up the handle. Removing it here would prevent proper
            // thread synchronization when calling Thread.join() or VM shutdown.
        });

        // Register the thread handle with the tokio JoinHandle and daemon flag
        let thread_handle = ThreadHandle::from((new_thread, join_handle, is_daemon));
        let thread_handles = vm.thread_handles();
        thread_handles
            .insert(internal_thread_id, thread_handle)
            .await?;

        // Yield to give the spawned task a chance to start running.
        // This helps ensure the new thread enters its run() method before
        // the calling thread continues, which is important for timing-sensitive
        // operations like checking thread state immediately after start().
        tokio::task::yield_now().await;
    }

    #[cfg(target_family = "wasm")]
    {
        let spawn_vm = vm.clone();
        let spawn_thread_id = internal_thread_id;
        wasm_bindgen_futures::spawn_local(async move {
            let _ = spawn_thread
                .execute(&thread_class, &run_method, &[thread_value])
                .await;

            // Set thread status to TERMINATED and eetop to 0 after execution
            if let Err(error) = set_thread_status(&thread_object, ThreadState::TERMINATED) {
                error!("Failed to set thread status to TERMINATED: {error}");
            }
            if let Ok(mut thread_obj) = thread_object.as_object_mut()
                && let Err(error) = thread_obj.set_value("eetop", Value::Long(0))
            {
                error!("Failed to set eetop to 0: {error}");
            }

            // Remove the thread from the VM's thread handles
            // (WASM doesn't support JoinHandles, so we clean up here)
            let thread_handles = spawn_vm.thread_handles();
            thread_handles.remove(&spawn_thread_id).await;
        });

        // Register the thread handle without a JoinHandle for WASM
        let thread_handle = ThreadHandle::from(new_thread);
        let thread_handles = vm.thread_handles();
        thread_handles
            .insert(internal_thread_id, thread_handle)
            .await?;
    }

    Ok(None)
}

#[intrinsic_method(
    "java/lang/Thread.stop0(Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn stop_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(UnsupportedOperationException(
        "java/lang/Thread.stop0(Ljava/lang/Object;)V is not supported".to_string(),
    )
    .into())
}

#[intrinsic_method("java/lang/Thread.suspend0()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn suspend_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        UnsupportedOperationException("java/lang/Thread.suspend0()V is not supported".to_string())
            .into(),
    )
}

#[intrinsic_method("java/lang/Thread.yield()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn r#yield<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn yield_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    r#yield(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::VM;

    /// Helper function to create a thread object for testing
    async fn create_thread<V: VM + 'static>(vm: &Arc<V>) -> Result<Value> {
        let thread_value = vm.object("java/lang/Thread", "", &[] as &[Value]).await?;
        let thread_id = vm.next_thread_id()?;
        let thread = vm.create_thread(thread_id)?;
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
        // At least one thread should exist (the main thread)
        assert!(!threads.is_empty());
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
