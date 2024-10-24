use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::NullPointer;
use crate::Result;
use ristretto_classloader::{Object, Reference, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

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
fn clear_interrupt_event(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}

#[expect(clippy::needless_pass_by_value)]
fn count_stack_frames(
    thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let frames = thread.frames().await?;
        let frames = i32::try_from(frames.len())?;
        Ok(Some(Value::Int(frames)))
    })
}

fn current_carrier_thread(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        // TODO: correct this once threading is implemented
        current_thread(thread, arguments).await
    })
}

#[expect(clippy::needless_pass_by_value)]
fn current_thread(
    thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        // TODO: correct this once threading is implemented
        let vm = thread.vm()?;
        let thread_class = vm.load_class(&thread, "java/lang/Thread").await?;
        let object = Object::new(thread_class)?;
        let reference = Reference::Object(object);
        let thread = Value::Object(Some(reference));
        Ok(Some(thread))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn register_natives(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}

fn set_native_name(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(name)) = arguments.pop_object()? else {
            return Err(NullPointer("name cannot be null".to_string()));
        };
        let name = name.as_string()?;
        thread.set_name(name).await;
        Ok(None)
    })
}

#[expect(clippy::needless_pass_by_value)]
fn set_priority_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _new_priority = arguments.pop_int()?;
        // TODO: implement priority if/when tokio supports it
        Ok(None)
    })
}

#[expect(clippy::needless_pass_by_value)]
fn sleep(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let millis = arguments.pop_long()?;
        let millis = u64::try_from(millis)?;
        let duration = Duration::from_millis(millis);
        #[cfg(not(target_arch = "wasm32"))]
        tokio::time::sleep(duration).await;
        #[cfg(target_arch = "wasm32")]
        std::thread::sleep(duration);
        Ok(None)
    })
}

#[expect(clippy::needless_pass_by_value)]
fn r#yield(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        #[cfg(not(target_arch = "wasm32"))]
        tokio::task::yield_now().await;
        #[cfg(target_arch = "wasm32")]
        std::thread::yield_now();
        Ok(None)
    })
}
