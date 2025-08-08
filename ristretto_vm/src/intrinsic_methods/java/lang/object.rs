use crate::Error::InternalError;
use crate::JavaError::CloneNotSupportedException;
use crate::Result;
use crate::assignable::Assignable;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{ObjectArray, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Intrinsic methods for `java/lang/Object.clone().
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

    let reference = match &reference {
        Reference::ByteArray(array) => {
            let array = array.read();
            Reference::ByteArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::CharArray(array) => {
            let array = array.read();
            Reference::CharArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::ShortArray(array) => {
            let array = array.read();
            Reference::ShortArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::IntArray(array) => {
            let array = array.read();
            Reference::IntArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::LongArray(array) => {
            let array = array.read();
            Reference::LongArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::FloatArray(array) => {
            let array = array.read();
            Reference::FloatArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::DoubleArray(array) => {
            let array = array.read();
            Reference::DoubleArray(Gc::new(RwLock::new(array.clone())))
        }
        Reference::Array(object_array) => {
            let array = object_array.elements.read();
            let array = array.clone();
            let mut cloned_values = Vec::with_capacity(array.len());
            for value in array {
                match value {
                    Some(reference) => cloned_values.push(Some(reference.clone())),
                    None => cloned_values.push(value),
                }
            }
            let object_array = ObjectArray {
                class: object_array.class.clone(),
                elements: Gc::new(RwLock::new(cloned_values)),
            };
            Reference::Array(object_array)
        }
        Reference::Object(object) => {
            let object_class = {
                let object = object.read();
                object.class().clone()
            };
            if object_class.name() == "java/lang/Class" {
                reference.clone()
            } else {
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
                let object = {
                    let object = object.read();
                    object.clone()
                };
                Reference::Object(Gc::new(RwLock::new(object.clone())))
            }
        }
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

    let class_name = object.class_name()?;
    let class = thread.class(class_name).await?;
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
    let hash_code = reference.hash_code();
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
