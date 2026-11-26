use crate::java::lang::thread::{ThreadState, set_thread_status};
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::Assignable;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::{
    CloneNotSupportedException, IllegalArgumentException, InterruptedException,
};
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::time::Duration;

/// Register finalizable instances when Object's constructor successfully completes.
/// A subclass constructor may subsequently fail; that does not undo finalizability.
#[intrinsic_method("java/lang/Object.<init>()V", Any)]
pub async fn init<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    register_finalizer(&thread, &value).await?;
    Ok(None)
}

async fn register_finalizer<T: Thread + 'static>(thread: &Arc<T>, value: &Value) -> Result<()> {
    let class = value.as_object_ref()?.class().clone();
    if let Ok(method) = class.resolve_method("finalize", "()V")
        && method.declaring_class.name() != "java/lang/Object"
    {
        thread
            .invoke(
                "java/lang/ref/Finalizer",
                "register(Ljava/lang/Object;)V",
                std::slice::from_ref(value),
            )
            .await?;
    }
    Ok(())
}

/// Helper to get a stable identifier for an object to use as a monitor key.
#[must_use]
pub fn get_monitor_id(
    reference: &ristretto_gc::Gc<ristretto_gc::sync::RwLock<Reference>>,
) -> Option<i64> {
    let id = reference.as_handle_i64();
    (id != 0).then_some(id)
}

/// Intrinsic methods for `java/lang/Object.clone()`.
///
/// # References
///
/// - [java.lang.Object.clone()](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/lang/Object.html#clone())
#[intrinsic_method("java/lang/Object.clone()Ljava/lang/Object;", Any)]
pub async fn clone<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "clone() called on non-reference value".to_string(),
        ));
    };

    let object_class = match &*reference.read() {
        Reference::Object(object) => Some(object.class().clone()),
        _ => None,
    };
    if let Some(object_class) = &object_class {
        if object_class.name() == "java/lang/Class" {
            return Ok(Some(Value::from(reference.rooted_clone())));
        }
        let cloneable_class = thread.class("java.lang.Cloneable").await?;
        if !cloneable_class
            .is_assignable_from(&thread, object_class)
            .await?
        {
            return Err(CloneNotSupportedException(
                [
                    "class ",
                    object_class.name(),
                    " does not implement Cloneable",
                ]
                .concat(),
            )
            .into());
        }
    }
    let gc = thread.vm()?.garbage_collector().clone();
    let size = ristretto_gc::Gc::allocation_size(&*reference)?;
    gc.make_room_async(size).await?;
    // Release the source guard before publication can suspend for recovery.
    // The source handle and copied reference fields retain owning roots.
    let cloned = reference.read().try_clone()?;
    let value = Value::try_new_object_async(&gc, cloned).await?;
    if object_class.is_some() {
        register_finalizer(&thread, &value).await?;
    }
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Object.getClass()Ljava/lang/Class;", Any)]
pub async fn get_class<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(object) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };

    let object = Value::Object(Some(object));
    let class = ristretto_types::method_resolution::receiver_class(&*thread, &object).await?;
    let class = class.to_object(&thread).await?;
    Ok(Some(class))
}

#[intrinsic_method("java/lang/Object.hashCode()I", Any)]
pub fn hash_code<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };
    let hash_code = u64::from_ne_bytes(reference.as_handle_i64().to_ne_bytes());
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = (hash_code ^ (hash_code >> 32)) as u32;
    let hash_code: i32 = zerocopy::transmute!(hash_code);

    Ok(Some(Value::Int(hash_code)))
}

#[intrinsic_method("java/lang/Object.notify()V", Any)]
pub fn notify<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "notify() called on null reference".to_string(),
        ));
    };

    if let Some(id) = get_monitor_id(&reference) {
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.notify(thread.id())?;
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Object.notifyAll()V", Any)]
pub fn notify_all<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "notifyAll() called on null reference".to_string(),
        ));
    };

    if let Some(id) = get_monitor_id(&reference) {
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.notify_all(thread.id())?;
    }

    Ok(None)
}

#[intrinsic_method("java/lang/Object.registerNatives()V", LessThanOrEqual(JAVA_11))]
pub fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Object.wait(J)V", LessThanOrEqual(JAVA_17))]
pub async fn wait<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    wait_0(thread, parameters).await
}

#[intrinsic_method("java/lang/Object.wait0(J)V", GreaterThan(JAVA_17))]
pub async fn wait_0<T: Thread + 'static>(
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

    // Use standard monitor wait / notify for all objects including Thread objects. Thread
    // termination calls notifyAll() on the Thread object's monitor, so Thread.join() works
    // correctly via the monitor mechanism.
    let vm = thread.vm()?;
    let monitor_id = { get_monitor_id(&reference) };
    if let Some(id) = monitor_id {
        let monitor = vm.monitor_registry().monitor(id);
        drop(vm);
        let thread_clone = thread.clone();
        let is_interrupted = move || thread_clone.is_interrupted(false);

        let thread_object = thread.java_object().await;
        let interrupted = if millis == 0 {
            let _ = set_thread_status(&thread_object, ThreadState::WAITING);
            let result = monitor
                .wait_interruptibly(thread.id(), is_interrupted)
                .await;
            let _ = set_thread_status(&thread_object, ThreadState::RUNNABLE);
            result?
        } else {
            let duration = Duration::from_millis(u64::try_from(millis)?);
            let _ = set_thread_status(&thread_object, ThreadState::TIMED_WAITING);
            let result = monitor
                .wait_timeout_interruptibly(thread.id(), duration, is_interrupted)
                .await;
            let _ = set_thread_status(&thread_object, ThreadState::RUNNABLE);
            result?
        };

        if interrupted {
            // Clear the interrupt flag (consumed by InterruptedException)
            thread.is_interrupted(true);
            return Err(InterruptedException("Thread interrupted while waiting".into()).into());
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
            let monitor_id = get_monitor_id(reference).expect("monitor id");
            let vm = thread.vm()?;
            let monitor = vm.monitor_registry().monitor(monitor_id);
            monitor.acquire(thread.id()).await?;
        }

        let parameters = Parameters::new(vec![object_value]);
        let result = notify_all(thread, parameters)?;
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
        let (_vm, thread) = crate::test::java11_thread().await?;
        let result = register_natives(thread, Parameters::default())?;
        assert_eq!(result, None);
        Ok(())
    }
}
