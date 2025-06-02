use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, MethodRegistry};
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Object";

/// Register all intrinsic methods for `java.lang.Object`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "wait", "(J)V", wait);
    } else {
        registry.register(CLASS_NAME, "wait0", "(J)V", wait_0);
    }

    registry.register(CLASS_NAME, "clone", "()Ljava/lang/Object;", clone);
    registry.register(CLASS_NAME, "getClass", "()Ljava/lang/Class;", get_class);
    registry.register(CLASS_NAME, "hashCode", "()I", hash_code);
    registry.register(CLASS_NAME, "notify", "()V", notify);
    registry.register(CLASS_NAME, "notifyAll", "()V", notify_all);
}

#[async_recursion(?Send)]
async fn clone(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let cloned_value = value.deep_clone()?;
    Ok(Some(cloned_value))
}

#[async_recursion(?Send)]
async fn get_class(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let Some(object) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };

    let class_name = object.class_name();
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn hash_code(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let Some(object) = parameters.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };
    let hash_code = object_hash_code(&object);
    Ok(Some(Value::Int(hash_code)))
}

pub(crate) fn object_hash_code(object: &Reference) -> i32 {
    let value = format!("{object}");
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash_code = hasher.finish();
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = hash_code as i32;
    hash_code
}

#[async_recursion(?Send)]
async fn notify(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.notify()V")
}

#[async_recursion(?Send)]
async fn notify_all(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn wait(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait(J)V")
}

#[async_recursion(?Send)]
async fn wait_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait0(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_all(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    // TODO: Add test for deep clone of Value::Object
    #[tokio::test]
    async fn test_clone() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = Value::Int(42);
        let parameters = Parameters::new(vec![object.clone()]);
        let result = clone(thread, parameters).await?;
        assert_eq!(result, Some(object));
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
