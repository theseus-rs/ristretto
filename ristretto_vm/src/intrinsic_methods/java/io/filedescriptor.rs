use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/io/FileDescriptor.close0()V", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.close0()V")
}

#[intrinsic_method("java/io/FileDescriptor.getAppend(I)Z", GreaterThanOrEqual(JAVA_11))]
#[expect(clippy::match_same_arms)]
#[async_recursion(?Send)]
pub(crate) async fn get_append(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let append = match handle {
        0 => {
            // true if stdin is in append mode
            false
        }
        1 => {
            // true if stdout is in append mode
            false
        }
        2 => {
            // true if stderr is in append mode
            false
        }
        _ => false,
    };
    Ok(Some(Value::from(append)))
}

#[intrinsic_method("java/io/FileDescriptor.getHandle(I)J", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn get_handle(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/io/FileDescriptor.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileDescriptor.sync()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sync(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    sync_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileDescriptor.sync0()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sync_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.sync0()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_append() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let handles = [0, 1, 2, 3];

        for handle in handles {
            let result =
                get_append(thread.clone(), Parameters::new(vec![Value::Int(handle)])).await?;
            assert_eq!(Some(Value::from(false)), result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_handle(thread, Parameters::new(vec![Value::Int(1)])).await?;
        assert_eq!(Some(Value::Long(1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.sync0()V")]
    async fn test_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.sync0()V")]
    async fn test_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync_0(thread, Parameters::default()).await;
    }
}
