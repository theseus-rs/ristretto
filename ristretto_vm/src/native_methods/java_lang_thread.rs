use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::NullPointer;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Object, Reference, Value};
use std::sync::Arc;
use std::time::Duration;

const JAVA_19: Version = Version::Java19 { minor: 0 };

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
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setNativeName",
        "(Ljava/lang/String;)V",
        set_native_name,
    );
    registry.register(class_name, "setPriority0", "(I)V", set_priority_0);
    registry.register(class_name, "sleep", "(J)V", sleep);
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
    // TODO: correct this once threading is implemented
    let vm = thread.vm()?;
    let java_version = vm.java_class_file_version();

    let thread_group_class = vm.load_class(&thread, "java/lang/ThreadGroup").await?;
    let thread_group = Object::new(thread_group_class)?;
    thread_group.set_value("maxPriority", Value::Int(10))?;
    thread_group.set_value("name", vm.string("main").await?)?;
    thread_group.set_value("parent", Value::Object(None))?;
    let reference = Reference::from(thread_group);
    let thread_group = Value::Object(Some(reference));

    // The internal structure of Thread changed in Java 19
    let new_thread = if java_version < &JAVA_19 {
        let thread_class = vm.load_class(&thread, "java/lang/Thread").await?;
        let new_thread = Object::new(thread_class)?;
        new_thread.set_value("group", thread_group)?;
        new_thread.set_value("priority", Value::Int(1))?;
        new_thread.set_value("threadStatus", Value::Int(4))?; // Runnable
        new_thread
    } else {
        let field_holder_class = vm
            .load_class(&thread, "java/lang/Thread$FieldHolder")
            .await?;
        let field_holder = Object::new(field_holder_class)?;
        field_holder.set_value("group", thread_group)?;
        field_holder.set_value("priority", Value::Int(1))?;
        field_holder.set_value("threadStatus", Value::Int(4))?; // Runnable
        let reference = Reference::from(field_holder);
        let field_holder = Value::Object(Some(reference));

        let thread_class = vm.load_class(&thread, "java/lang/Thread").await?;
        let new_thread = Object::new(thread_class)?;
        new_thread.set_value("holder", field_holder)?;
        new_thread
    };

    let reference = Reference::from(new_thread);
    let new_thread = Value::Object(Some(reference));
    Ok(Some(new_thread))
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
async fn r#yield(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::task::yield_now().await;
    #[cfg(target_arch = "wasm32")]
    std::thread::yield_now();
    Ok(None)
}
