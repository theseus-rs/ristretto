use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/reflect/Reflection";

/// Register all native methods for `sun.reflect.Reflection`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class_1,
    );
    registry.register(
        CLASS_NAME,
        "getCallerClass",
        "(I)Ljava/lang/Class;",
        get_caller_class_2,
    );
    registry.register(
        CLASS_NAME,
        "getClassAccessFlags",
        "(Ljava/lang/Class;)I",
        get_class_access_flags,
    );
}

#[async_recursion(?Send)]
async fn get_caller_class_1(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    let Some(frame) = frames.last() else {
        return Ok(Some(Value::Object(None)));
    };
    let class = frame.class();
    let class_name = class.name();
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn get_caller_class_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.reflect.Reflection.getCallerClass(I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_access_flags(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
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
    async fn test_get_caller_class_1_null() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::default();
        let result = get_caller_class_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.Reflection.getCallerClass(I)Ljava/lang/Class;"
    )]
    async fn test_get_caller_class_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::default();
        let _ = get_caller_class_2(thread, parameters).await;
    }

    #[tokio::test]
    async fn test_get_class_access_flags() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&vm).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_class_access_flags(thread, parameters).await?;
        let access_flags: i32 = result.expect("access_flags").try_into()?;
        assert_eq!(access_flags, 49);
        Ok(())
    }
}
