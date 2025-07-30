use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/reflect/Reflection.getCallerClass()Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_caller_class_1(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let frames = thread.frames().await?;
    if frames.len() < 2 {
        return Ok(Some(Value::Object(None)));
    }
    // current frame = len - 1, caller = len - 2, etc.
    let caller_frame = &frames[frames.len() - 2];
    let class = caller_frame.class();
    let class_object = class.to_object(&thread).await?;
    Ok(Some(class_object))
}

#[intrinsic_method(
    "sun/reflect/Reflection.getCallerClass(I)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_caller_class_2(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let depth = usize::try_from(parameters.pop_int()?)?;
    let frames = thread.frames().await?;
    if frames.len() <= depth {
        return Ok(Some(Value::Object(None)));
    }
    // The frame at index (frames.len() - 1 - depth) is the target
    // current frame = len - 1, caller = len - 2, etc.
    let frame = &frames[frames.len() - 1 - depth];
    let class = frame.class();
    let class_object = class.to_object(&thread).await?;
    Ok(Some(class_object))
}

#[intrinsic_method(
    "sun/reflect/Reflection.getClassAccessFlags(Ljava/lang/Class;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_access_flags(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.value("name")?.as_string()?
    };
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
    async fn test_get_caller_class_2() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(42); // Depth parameter
        let result = get_caller_class_2(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_access_flags() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_class_access_flags(thread, parameters).await?;
        let access_flags = result.expect("access_flags").as_i32()?;
        assert_eq!(access_flags, 49);
        Ok(())
    }
}
