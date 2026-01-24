use crate::Error::InternalError;
use crate::JavaError::CloneNotSupportedException;
use crate::Result;
use crate::assignable::Assignable;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{ObjectArray, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
use std::time::Duration;

/// Intrinsic methods for `java/lang/Object.clone()`.
///
/// # References
///
/// - [java.lang.Object.clone()](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/lang/Object.html#clone())
#[intrinsic_method("java/lang/Object.clone()Ljava/lang/Object;", Any)]
#[async_method]
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
        Reference::BooleanArray(array) => Reference::BooleanArray(array.clone()),
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
#[async_method]
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
#[async_method]
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
#[async_method]
pub(crate) async fn notify(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // For basic thread support, notify is a no-op since we don't have true monitor support yet.
    // In a full implementation, this would wake up one thread waiting on this object's monitor.
    Ok(None)
}

#[intrinsic_method("java/lang/Object.notifyAll()V", Any)]
#[async_method]
pub(crate) async fn notify_all(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.registerNatives()V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.wait(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn wait(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    wait_0(thread, parameters).await
}

#[intrinsic_method("java/lang/Object.wait0(J)V", GreaterThan(JAVA_17))]
#[async_method]
pub(crate) async fn wait_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let millis = parameters.pop_long()?;
    let object = parameters.pop_reference()?;

    if millis < 0 {
        return Err(crate::JavaError::IllegalArgumentException(
            "timeout value is negative".to_string(),
        )
        .into());
    }

    // Check if we're waiting on a Thread object for join()
    let is_thread_object = if let Some(ref obj_ref) = object {
        let guard = obj_ref.read();
        if let Ok(class_name) = guard.class_name() {
            class_name == "java/lang/Thread" || class_name.starts_with("java/lang/Thread$")
        } else {
            false
        }
    } else {
        false
    };

    if is_thread_object {
        // For Thread.join(), we need to poll until the thread is terminated
        // The thread's eetop field is set to 0 when the thread terminates
        let start = std::time::Instant::now();
        let timeout_duration = if millis == 0 {
            // Infinite wait
            Duration::from_secs(u64::MAX)
        } else {
            Duration::from_millis(u64::try_from(millis)?)
        };

        loop {
            // Yield to allow other tasks to run (important for spawned thread tasks)
            #[cfg(not(target_family = "wasm"))]
            tokio::task::yield_now().await;

            // Check if the thread has terminated (eetop == 0)
            // Use try_read to avoid blocking; this allows the spawned task to acquire write locks
            let is_terminated = if let Some(ref obj_ref) = object {
                if let Some(guard) = obj_ref.try_read() {
                    if let Reference::Object(obj) = &*guard {
                        if let Ok(eetop) = obj.value("eetop") {
                            eetop.as_i64().unwrap_or(0) == 0
                        } else {
                            // Can't read eetop, assume terminated
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    // Couldn't acquire lock, try again after sleep
                    false
                }
            } else {
                true
            };

            if is_terminated {
                return Ok(None);
            }

            if start.elapsed() >= timeout_duration {
                return Ok(None);
            }

            // Sleep briefly and check again
            #[cfg(target_family = "wasm")]
            std::thread::sleep(Duration::from_millis(10));
            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    } else if millis > 0 {
        let millis = u64::try_from(millis)?;
        let duration = Duration::from_millis(millis);
        #[cfg(target_family = "wasm")]
        std::thread::sleep(duration);
        #[cfg(not(target_family = "wasm"))]
        tokio::time::sleep(duration).await;
    }
    Ok(None)
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
    async fn test_notify() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
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
    async fn test_wait() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Create an object to wait on
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        let object_value = Value::from(object);
        // Pass object reference first (this), then timeout of 1ms (brief wait)
        let parameters = Parameters::new(vec![object_value, Value::Long(1)]);
        let result = wait(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Create an object to wait on
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        let object_value = Value::from(object);
        // Pass object reference first (this), then timeout of 1ms (brief wait)
        let parameters = Parameters::new(vec![object_value, Value::Long(1)]);
        let result = wait_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
