use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for jdk.internal.reflect.Reflection.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/reflect/Reflection";
    registry.register(
        class_name,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_caller_class(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    for frame in frames.iter().rev() {
        let class = frame.class();
        let class_name = class.name();

        if class_name == "java/lang/MethodHandles" {
            continue;
        }

        let vm = thread.vm()?;
        let class = vm.to_class_value(&thread, class_name).await?;
        return Ok(Some(class));
    }
    Ok(None)
}
