use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_20};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/FileDescriptor";

/// Register all native methods for `java.io.FileDescriptor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(CLASS_NAME, "close0", "()V", close_0);
        registry.register(CLASS_NAME, "getAppend", "(I)Z", get_append);
        registry.register(CLASS_NAME, "getHandle", "(I)J", get_handle);
    }

    if registry.java_major_version() <= JAVA_20 {
        registry.register(CLASS_NAME, "sync", "()V", sync);
    } else {
        registry.register(CLASS_NAME, "sync0", "()V", sync_0);
    }

    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.close0()V")
}

#[expect(clippy::match_same_arms)]
#[async_recursion(?Send)]
async fn get_append(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
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

#[async_recursion(?Send)]
async fn get_handle(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn sync(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    sync_0(thread, arguments).await
}

#[async_recursion(?Send)]
async fn sync_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileDescriptor.sync0()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_get_append() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let handles = [0, 1, 2, 3];

        for handle in handles {
            let result =
                get_append(thread.clone(), Arguments::new(vec![Value::Int(handle)])).await?;
            assert_eq!(Some(Value::from(false)), result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_handle(thread, Arguments::new(vec![Value::Int(1)])).await?;
        assert_eq!(Some(Value::Long(1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.sync0()V")]
    async fn test_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileDescriptor.sync0()V")]
    async fn test_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync_0(thread, Arguments::default()).await;
    }
}
