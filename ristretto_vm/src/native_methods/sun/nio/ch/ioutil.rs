use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `sun.nio.ch.IOUtil`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/IOUtil";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(class_name, "drain1", "(I)I", drain_1);
        registry.register(class_name, "write1", "(IB)I", write_1);
    }

    if java_version >= JAVA_18 {
        registry.register(class_name, "writevMax", "()J", writev_max);
    }

    registry.register(
        class_name,
        "configureBlocking",
        "(Ljava/io/FileDescriptor;Z)V",
        configure_blocking,
    );
    registry.register(class_name, "drain", "(I)Z", drain);
    registry.register(class_name, "fdLimit", "()I", fd_limit);
    registry.register(class_name, "fdVal", "(Ljava/io/FileDescriptor;)I", fd_val);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "iovMax", "()I", iov_max);
    registry.register(class_name, "makePipe", "(Z)J", make_pipe);
    registry.register(class_name, "randomBytes", "([B)Z", random_bytes);
    registry.register(
        class_name,
        "setfdVal",
        "(Ljava/io/FileDescriptor;I)V",
        setfd_val,
    );
}

#[async_recursion(?Send)]
async fn configure_blocking(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V");
}

#[async_recursion(?Send)]
async fn drain(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.drain(I)Z");
}

#[async_recursion(?Send)]
async fn drain_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.drain1(I)I");
}

#[async_recursion(?Send)]
async fn fd_limit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.fdLimit()I");
}

#[async_recursion(?Send)]
async fn fd_val(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.fdVal(Ljava/io/FileDescriptor;)I");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn iov_max(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(16)))
}

#[async_recursion(?Send)]
async fn make_pipe(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.makePipe(Z)J");
}

#[async_recursion(?Send)]
async fn random_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.randomBytes([B)Z");
}

#[async_recursion(?Send)]
async fn setfd_val(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V");
}

#[async_recursion(?Send)]
async fn write_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.IOUtil.write1(IB)I");
}

#[async_recursion(?Send)]
async fn writev_max(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java18 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/nio/ch/IOUtil";
        assert!(registry
            .method(
                class_name,
                "configureBlocking",
                "(Ljava/io/FileDescriptor;Z)V"
            )
            .is_some());
        assert!(registry.method(class_name, "drain", "(I)Z").is_some());
        assert!(registry.method(class_name, "fdLimit", "()I").is_some());
        assert!(registry
            .method(class_name, "fdVal", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry.method(class_name, "iovMax", "()I").is_some());
        assert!(registry.method(class_name, "makePipe", "(Z)J").is_some());
        assert!(registry
            .method(class_name, "randomBytes", "([B)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setfdVal", "(Ljava/io/FileDescriptor;I)V")
            .is_some());
        assert!(registry.method(class_name, "drain1", "(I)I").is_some());
        assert!(registry.method(class_name, "write1", "(IB)I").is_some());
        assert!(registry.method(class_name, "writevMax", "()J").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.configureBlocking(Ljava/io/FileDescriptor;Z)V")]
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = configure_blocking(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.drain(I)Z")]
    async fn test_drain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.fdLimit()I")]
    async fn test_fd_limit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fd_limit(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.fdVal(Ljava/io/FileDescriptor;)I")]
    async fn test_fd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fd_val(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_iov_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = iov_max(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(16)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.makePipe(Z)J")]
    async fn test_make_pipe() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_pipe(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.randomBytes([B)Z")]
    async fn test_random_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = random_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V")]
    async fn test_setfd_val() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = setfd_val(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.drain1(I)I")]
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.IOUtil.write1(IB)I")]
    async fn test_write_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_writev_max() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writev_max(thread, Arguments::default()).await?;
        let expected = match OS {
            "windows" => i64::MAX,
            _ => i64::from(i32::MAX),
        };
        assert_eq!(result, Some(Value::Long(expected)));
        Ok(())
    }
}
