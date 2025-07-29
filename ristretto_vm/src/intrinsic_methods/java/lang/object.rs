use crate::Error::InternalError;
use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/Object.clone()Ljava/lang/Object;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn clone(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let cloned_value = value.deep_clone()?;
    Ok(Some(cloned_value))
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

    let class_name = object.class_name();
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
