use crate::Result;
use crate::native_methods::registry::{JAVA_11, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/IOUtil";

/// Register all native methods for `sun.nio.ch.IOUtil`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(CLASS_NAME, "drain1", "(I)I", drain_1);
        registry.register(CLASS_NAME, "write1", "(IB)I", write_1);
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "writevMax", "()J", writev_max);
    }

    registry.register(
        CLASS_NAME,
        "configureBlocking",
        "(Ljava/io/FileDescriptor;Z)V",
        configure_blocking,
    );
    registry.register(CLASS_NAME, "drain", "(I)Z", drain);
    registry.register(CLASS_NAME, "fdLimit", "()I", fd_limit);
    registry.register(CLASS_NAME, "fdVal", "(Ljava/io/FileDescriptor;)I", fd_val);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "iovMax", "()I", iov_max);
    registry.register(CLASS_NAME, "makePipe", "(Z)J", make_pipe);
    registry.register(CLASS_NAME, "randomBytes", "([B)Z", random_bytes);
    registry.register(
        CLASS_NAME,
        "setfdVal",
        "(Ljava/io/FileDescriptor;I)V",
        setfd_val,
    );
}

#[async_recursion(?Send)]
async fn configure_blocking(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V");
}

#[async_recursion(?Send)]
async fn drain(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.drain(I)Z");
}

#[async_recursion(?Send)]
async fn drain_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.drain1(I)I");
}

#[async_recursion(?Send)]
async fn fd_limit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.fdLimit()I");
}

#[async_recursion(?Send)]
async fn fd_val(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.fdVal(Ljava/io/FileDescriptor;)I");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn iov_max(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Int(16)))
}

#[async_recursion(?Send)]
async fn make_pipe(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.makePipe(Z)J");
}

#[async_recursion(?Send)]
async fn random_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.randomBytes([B)Z");
}

#[async_recursion(?Send)]
async fn setfd_val(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V");
}

#[async_recursion(?Send)]
async fn write_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.write1(IB)I");
}

#[async_recursion(?Send)]
async fn writev_max(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Some(Value::Long(i64::MAX)))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Some(Value::Long(i64::from(i32::MAX))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::consts::OS;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V"
    )]
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = configure_blocking(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.drain(I)Z")]
    async fn test_drain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.fdLimit()I")]
    async fn test_fd_limit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fd_limit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.IOUtil.fdVal(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_fd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fd_val(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_iov_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = iov_max(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(16)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.makePipe(Z)J")]
    async fn test_make_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_pipe(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.randomBytes([B)Z")]
    async fn test_random_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = random_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_setfd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = setfd_val(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.drain1(I)I")]
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.IOUtil.write1(IB)I")]
    async fn test_write_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_writev_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writev_max(thread, Parameters::default()).await?;
        let expected = match OS {
            "windows" => i64::MAX,
            _ => i64::from(i32::MAX),
        };
        assert_eq!(result, Some(Value::Long(expected)));
        Ok(())
    }
}
