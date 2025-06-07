use crate::Error::InternalError;
use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn are_nest_mates(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.reflect.Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z")
}

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.getCallerClass()Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_caller_class(
    thread: Arc<Thread>,
    _parameters: Parameters,
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

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.getClassAccessFlags(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_access_flags(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = parameters.pop_reference()? else {
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
        let _ = are_nest_mates(thread, Parameters::default()).await;
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
