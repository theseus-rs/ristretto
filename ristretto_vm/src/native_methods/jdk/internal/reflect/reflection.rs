use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/reflect/Reflection";

/// Register all native methods for `jdk.internal.reflect.Reflection`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "areNestMates",
        "(Ljava/lang/Class;Ljava/lang/Class;)Z",
        are_nest_mates,
    );
    registry.register(
        CLASS_NAME,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class,
    );
    registry.register(
        CLASS_NAME,
        "getClassAccessFlags",
        "(Ljava/lang/Class;)I",
        get_class_access_flags,
    );
}

#[async_recursion(?Send)]
async fn are_nest_mates(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.reflect.Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
pub(crate) async fn get_caller_class(
    thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    for frame in frames.iter().rev() {
        let class = frame.class();
        let class_name = class.name();

        if class_name == "java/lang/MethodHandles" {
            continue;
        }

        let vm = thread.vm()?;
        let class = thread.class(class_name).await?;
        let class = class.to_object(&vm).await?;
        return Ok(Some(class));
    }
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_class_access_flags(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "getClassAccessFlags: no arguments".to_string(),
        ));
    };
    let class_name: String = object.value("name")?.try_into()?;
    let class = thread.class(&class_name).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags;
    #[expect(clippy::cast_lossless)]
    let class_access_flags = access_flags.bits() as i32;
    Ok(Some(Value::Int(class_access_flags)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.reflect.Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z"
    )]
    async fn test_are_nest_mates() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = are_nest_mates(thread, Arguments::default()).await;
    }
}
