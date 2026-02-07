use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{ObjectArray, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Assignable;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::{CloneNotSupportedException, IllegalArgumentException};
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::time::Duration;

/// Helper to get a stable identifier for an object to use as a monitor key.
#[must_use]
pub fn get_monitor_id(reference: &Reference) -> Option<usize> {
    match reference {
        Reference::Object(obj) => Some(std::ptr::from_ref(obj) as usize),
        Reference::Array(arr) => Some(std::ptr::from_ref(arr) as usize),
        _ => None,
    }
}

/// Intrinsic methods for `java/lang/Object.clone()`.
///
/// # References
///
/// - [java.lang.Object.clone()](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/lang/Object.html#clone())
#[intrinsic_method("java/lang/Object.clone()Ljava/lang/Object;", Any)]
#[async_method]
pub async fn clone<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
        let value = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::Object(object.clone()),
        );
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
    let value = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Object.getClass()Ljava/lang/Class;", Any)]
#[async_method]
pub async fn get_class<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
pub async fn hash_code<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };
    let guard = reference.read();
    let hash_code = guard.hash_code();
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = (hash_code ^ (hash_code >> 32)) as u32;
    let hash_code: i32 = zerocopy::transmute!(hash_code);

    Ok(Some(Value::Int(hash_code)))
}

#[intrinsic_method("java/lang/Object.notify()V", Any)]
#[async_method]
pub async fn notify<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "notify() called on null reference".to_string(),
        ));
    };

    if let Some(id) = get_monitor_id(&reference.read()) {
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.notify(thread.id())?;
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Object.notifyAll()V", Any)]
#[async_method]
pub async fn notify_all<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "notifyAll() called on null reference".to_string(),
        ));
    };

    if let Some(id) = get_monitor_id(&reference.read()) {
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.notify_all(thread.id())?;
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Object.registerNatives()V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.wait(J)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn wait<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    wait_0(thread, parameters).await
}

#[intrinsic_method("java/lang/Object.wait0(J)V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn wait_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let millis = parameters.pop_long()?;
    let object = parameters.pop_reference()?;
    let Some(reference) = object else {
        return Err(InternalError("wait() called on null reference".to_string()));
    };

    if millis < 0 {
        return Err(IllegalArgumentException("timeout value is negative".to_string()).into());
    }

    // Check if we're waiting on a Thread object for join()
    // This logic is preserved for "Thread.join()" behavior compatibility
    // until we unify Thread.exit() with monitor notification.
    let is_thread_object = {
        let guard = reference.read();
        if let Ok(class_name) = guard.class_name() {
            class_name == "java/lang/Thread" || class_name.starts_with("java/lang/Thread$")
        } else {
            false
        }
    };

    if is_thread_object {
        // For Thread.join(), we need to poll until the thread is terminated
        // The thread's eetop field is set to 0 when the thread terminates
        // TODO: Refactor this to use monitor notification from the dying thread
        let start = std::time::Instant::now();
        let timeout_duration = if millis == 0 {
            // Infinite wait
            Duration::from_secs(u64::MAX)
        } else {
            Duration::from_millis(u64::try_from(millis)?)
        };

        loop {
            // Check for interruption
            if thread.is_interrupted(true) {
                return Err(ristretto_types::JavaError::InterruptedException(
                    "Thread interrupted while waiting".into(),
                )
                .into());
            }

            // Check if the thread has terminated (eetop == 0)
            let is_terminated = {
                if let Some(guard) = reference.try_read() {
                    if let Reference::Object(obj) = &*guard {
                        if let Ok(eetop) = obj.value("eetop") {
                            eetop.as_i64().unwrap_or(0) == 0
                        } else {
                            // Field not found?
                            true
                        }
                    } else {
                        // Not an object (e.g. array), assume generic wait/terminated
                        true
                    }
                } else {
                    false
                }
            };

            if is_terminated {
                return Ok(None);
            }

            if start.elapsed() >= timeout_duration {
                return Ok(None);
            }

            // Sleep briefly and check again
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    } else {
        // Normal Object.wait() using Monitors
        let vm = thread.vm()?;
        let monitor_id = { get_monitor_id(&reference.read()) };
        if let Some(id) = monitor_id {
            let monitor = vm.monitor_registry().monitor(id);

            // Perform the wait
            if millis == 0 {
                monitor.wait(thread.id()).await?;
            } else {
                let duration = Duration::from_millis(u64::try_from(millis)?);
                monitor.wait_timeout(thread.id(), duration).await?;
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_notify_all() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        let object_value =
            Value::new_object(thread.vm()?.garbage_collector(), Reference::Object(object));

        // Acquire monitor
        if let Value::Object(Some(ref reference)) = object_value {
            let monitor_id = get_monitor_id(&reference.read()).expect("monitor id");
            let vm = thread.vm()?;
            let monitor = vm.monitor_registry().monitor(monitor_id);
            monitor.acquire(thread.id()).await?;
        }

        let parameters = Parameters::new(vec![object_value]);
        let result = notify_all(thread, parameters).await?;
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
        let object_value =
            Value::new_object(thread.vm()?.garbage_collector(), Reference::Object(object));
        let parameters = Parameters::new(vec![object_value]);

        // This should fail because Object doesn't implement Cloneable
        let result = clone(thread, parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
