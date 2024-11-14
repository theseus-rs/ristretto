use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, NullPointer};
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;
use std::time::Duration;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for java.lang.Thread.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Thread";
    registry.register(
        class_name,
        "clearInterruptEvent",
        "()V",
        clear_interrupt_event,
    );
    registry.register(class_name, "countStackFrames", "()I", count_stack_frames);
    registry.register(
        class_name,
        "currentCarrierThread",
        "()Ljava/lang/Thread;",
        current_carrier_thread,
    );
    registry.register(
        class_name,
        "currentThread",
        "()Ljava/lang/Thread;",
        current_thread,
    );
    registry.register(
        class_name,
        "getNextThreadIdOffset",
        "()J",
        get_next_thread_id_offset,
    );
    // TODO: only register the isAlive()Z method if the version for Java 18
    registry.register(class_name, "isAlive", "()Z", is_alive);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setNativeName",
        "(Ljava/lang/String;)V",
        set_native_name,
    );
    registry.register(class_name, "setPriority0", "(I)V", set_priority_0);
    registry.register(class_name, "sleep", "(J)V", sleep);
    registry.register(class_name, "start0", "()V", start_0);
    registry.register(class_name, "yield", "()V", r#yield);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn clear_interrupt_event(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn count_stack_frames(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    let frames = i32::try_from(frames.len())?;
    Ok(Some(Value::Int(frames)))
}

#[async_recursion(?Send)]
async fn current_carrier_thread(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    // TODO: correct this once threading is implemented
    current_thread(thread, arguments).await
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn current_thread(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let thread = thread.java_object().await;
    Ok(Some(thread))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_next_thread_id_offset(
    thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let thread_id = vm.next_thread_id()?;
    let thread_id = i64::try_from(thread_id)?;
    Ok(Some(Value::from(thread_id)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_alive(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let java_object = thread.java_object().await;
    let Some(Reference::Object(object)) = java_object.to_object()? else {
        return Err(InternalError("thread is not a java object".to_string()));
    };
    let eetop = object.value("eetop")?.to_long()?;
    let is_alive = eetop != 0;
    Ok(Some(Value::from(is_alive)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_native_name(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(name)) = arguments.pop_object()? else {
        return Err(NullPointer("name cannot be null".to_string()));
    };
    let name: String = name.try_into()?;
    thread.set_name(name).await;
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_priority_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _new_priority = arguments.pop_int()?;
    // TODO: implement priority if/when tokio supports it
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn sleep(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let millis = arguments.pop_long()?;
    let millis = u64::try_from(millis)?;
    let duration = Duration::from_millis(millis);
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(duration).await;
    #[cfg(target_arch = "wasm32")]
    std::thread::sleep(duration);
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn start_0(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = thread.java_object().await.to_object()? else {
        return Err(InternalError("thread is not a java object".to_string()));
    };

    let thread_id = i64::try_from(thread.id())?;
    object.set_value("eetop", Value::from(thread_id))?;
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn r#yield(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::task::yield_now().await;
    #[cfg(target_arch = "wasm32")]
    std::thread::yield_now();
    Ok(None)
}
