use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::{Object, Reference, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

/// Register all native methods for java.lang.Thread.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Thread";
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
    registry.register(class_name, "sleep", "(J)V", sleep);
    registry.register(class_name, "yield", "()V", r#yield);
}

#[expect(clippy::needless_pass_by_value)]
fn count_stack_frames(
    call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let frames = call_stack.frames()?;
        let frames = i32::try_from(frames.len())?;
        Ok(Some(Value::Int(frames)))
    })
}

fn current_carrier_thread(
    call_stack: Arc<CallStack>,
    arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        // TODO: correct this once threading is implemented
        current_thread(call_stack, arguments).await
    })
}

#[expect(clippy::needless_pass_by_value)]
fn current_thread(
    call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        // TODO: correct this once threading is implemented
        let vm = call_stack.vm()?;
        let thread_class = vm.load_class(&call_stack, "java/lang/Thread").await?;
        let object = Object::new(thread_class)?;
        let reference = Reference::Object(object);
        let thread = Value::Object(Some(reference));
        Ok(Some(thread))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn register_natives(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}

#[expect(clippy::needless_pass_by_value)]
fn sleep(
    _call_stack: Arc<CallStack>,
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
    _call_stack: Arc<CallStack>,
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
