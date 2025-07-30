use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::attributes::Attribute;
use ristretto_classloader::{Class, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn are_nest_mates(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let member_class_reference = parameters.pop_reference()?;
    let Some(Reference::Object(member_class_object)) = member_class_reference else {
        return Ok(Some(Value::from(false)));
    };
    let current_class_reference = parameters.pop_reference()?;
    let Some(Reference::Object(current_class_object)) = current_class_reference else {
        return Ok(Some(Value::from(false)));
    };

    let member_class = class::get_class(&thread, &member_class_object).await?;
    let current_class = class::get_class(&thread, &current_class_object).await?;
    if member_class == current_class {
        return Ok(Some(Value::from(true)));
    }

    let member_nest_host = get_nest_host(&thread, member_class).await?;
    let current_nest_host = get_nest_host(&thread, current_class).await?;
    let are_nest_mates = member_nest_host == current_nest_host;
    Ok(Some(Value::from(are_nest_mates)))
}

/// Returns the nest host class if it exists, otherwise returns the class.
async fn get_nest_host(thread: &Arc<Thread>, class: Arc<Class>) -> Result<Arc<Class>> {
    let class_file = class.class_file();
    for attribute in &class_file.attributes {
        let Attribute::NestHost {
            name_index: _name_index,
            host_class_index,
        } = attribute
        else {
            continue;
        };

        let constant_pool = &class_file.constant_pool;
        let host_class = constant_pool.try_get_class(*host_class_index)?;
        let host_class = thread.class(host_class).await?;
        return Ok(host_class);
    }
    Ok(class)
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
    // skip current frame
    for frame in frames.iter().rev().skip(1) {
        let class = frame.class();
        let class_name = class.name();
        // Skip known reflection/VM implementation frames
        if class_name.starts_with("jdk/internal/reflect/")
            || class_name.starts_with("sun/reflect/")
            || class_name == "java/lang/invoke/MethodHandles"
            || class_name == "java/lang/invoke/MethodHandleNatives"
            || class_name == "java/lang/invoke/MemberName"
        {
            continue;
        }
        let class_object = class.to_object(&thread).await?;
        return Ok(Some(class_object));
    }
    // No non-reflection caller found
    Ok(Some(Value::Object(None)))
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
    let class_name = object.value("name")?.as_string()?;
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
    async fn test_are_nest_mates_current_class_null_is_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = Value::Object(None);
        let member_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(!are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_member_class_null_is_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let member_class = Value::Object(None);
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(!are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_same_class_is_true() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let string_class = thread
            .class("java.lang.String")
            .await?
            .to_object(&thread)
            .await?;
        let current_class = string_class.clone();
        let member_class = string_class;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(are_nest_mates);
        Ok(())
    }

    #[tokio::test]
    async fn test_are_nest_mates_same_nest_host_is_true() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let current_class = thread
            .class("java.lang.Integer")
            .await?
            .to_object(&thread)
            .await?;
        let member_class = thread
            .class("java.lang.Integer$IntegerCache")
            .await?
            .to_object(&thread)
            .await?;
        let mut parameters = Parameters::default();
        parameters.push(current_class);
        parameters.push(member_class);
        let value = are_nest_mates(thread, parameters)
            .await?
            .expect("nest_mates");
        let are_nest_mates = value.as_bool()?;
        assert!(are_nest_mates);
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
