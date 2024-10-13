use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::{Object, Reference, Value};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Register all native methods for java.lang.Thread.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Thread";
    registry.register(class_name, "countStackFrames", "()I", count_stack_frames);
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
fn count_stack_frames(call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    let frames = call_stack.frames()?;
    let frames = i32::try_from(frames.len())?;
    Ok(Some(Value::Int(frames)))
}

#[expect(clippy::needless_pass_by_value)]
fn current_thread(call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: correct this once threading is implemented
    let vm = call_stack.vm()?;
    let thread_class = vm.class(call_stack, "java/lang/Thread")?;
    let thread = Value::Object(Some(Reference::Object(Object::new(thread_class)?)));
    Ok(Some(thread))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn register_natives(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

fn sleep(_call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let millis = arguments.pop_long()?;
    let millis = u64::try_from(millis)?;
    let duration = Duration::from_millis(millis);
    thread::sleep(duration);
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn r#yield(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    thread::yield_now();
    Ok(None)
}
