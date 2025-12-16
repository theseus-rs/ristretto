use crate::Error::InternalError;
use crate::JavaError::CloneNotSupportedException;
use crate::Result;
use crate::assignable::Assignable;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{ObjectArray, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Intrinsic methods for `java/lang/Object.clone()`.
///
/// # References
///
/// - [java.lang.Object.clone()](https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/lang/Object.html#clone())
#[intrinsic_method("java/lang/Object.clone()Ljava/lang/Object;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn clone(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "clone() called on non-reference value".to_string(),
        ));
    };

    let object_to_clone = {
        let guard = reference.read();
        if let Reference::Object(object) = &*guard {
            Some(object.clone())
        } else {
            None
        }
    };

    if let Some(object) = object_to_clone {
        let object_class = object.class().clone();
        if object_class.name() == "java/lang/Class" {
            return Ok(Some(Value::from(reference.clone())));
        }
        let cloneable_class = thread.class("java.lang.Cloneable").await?;
        let implements_cloneable = cloneable_class
            .is_assignable_from(&thread, &object_class)
            .await?;
        if !implements_cloneable {
            let class_name = object_class.name();
            return Err(CloneNotSupportedException(format!(
                "class {class_name} does not implement Cloneable"
            ))
            .into());
        }
        let value = Value::from(Reference::Object(object.clone()));
        return Ok(Some(value));
    }

    let guard = reference.read();
    let reference = match &*guard {
        Reference::ByteArray(array) => Reference::ByteArray(array.clone()),
        Reference::CharArray(array) => Reference::CharArray(array.clone()),
        Reference::ShortArray(array) => Reference::ShortArray(array.clone()),
        Reference::IntArray(array) => Reference::IntArray(array.clone()),
        Reference::LongArray(array) => Reference::LongArray(array.clone()),
        Reference::FloatArray(array) => Reference::FloatArray(array.clone()),
        Reference::DoubleArray(array) => Reference::DoubleArray(array.clone()),
        Reference::Array(object_array) => {
            let elements = object_array.elements.clone();
            let mut cloned_values = Vec::with_capacity(elements.len());
            for value in elements {
                if let Value::Object(Some(ref reference)) = value {
                    cloned_values.push(Value::Object(Some(reference.clone())));
                } else {
                    cloned_values.push(value);
                }
            }
            let object_array = ObjectArray {
                class: object_array.class.clone(),
                elements: cloned_values.into(),
            };
            Reference::Array(object_array)
        }
        Reference::Object(_) => unreachable!("Handled above"),
    };
    let value = Value::from(reference);
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Object.getClass()Ljava/lang/Class;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_class(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(object) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };

    let class_name = {
        let guard = object.read();
        guard.class_name()?.clone()
    };
    let class = thread.class(&class_name).await?;
    let class = class.to_object(&thread).await?;
    Ok(Some(class))
}

#[intrinsic_method("java/lang/Object.hashCode()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn hash_code(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };
    let guard = reference.read();
    let hash_code = guard.hash_code();
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = (hash_code ^ (hash_code >> 32)) as u32;
    #[expect(clippy::cast_possible_wrap)]
    let hash_code = hash_code as i32;

    Ok(Some(Value::Int(hash_code)))
}

#[intrinsic_method("java/lang/Object.notify()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn notify(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.notify()V")
}

#[intrinsic_method("java/lang/Object.notifyAll()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn notify_all(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.registerNatives()V", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.wait(J)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn wait(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait(J)V")
}

#[intrinsic_method("java/lang/Object.wait0(J)V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn wait_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait0(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_all(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    // TODO: Add test for clone of Value::Object
    #[tokio::test]
    async fn test_clone() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Create a proper object for cloning test
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        let object_value = Value::from(object);
        let parameters = Parameters::new(vec![object_value]);

        // This should fail because Object doesn't implement Cloneable
        let result = clone(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Object.notify()V")]
    async fn test_notify() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_notify_all() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_all(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Object.wait(J)V")]
    async fn test_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Object.wait0(J)V")]
    async fn test_wait_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_0(thread, Parameters::default()).await;
    }
}
