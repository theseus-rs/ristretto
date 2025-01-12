use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Object, Value};
use std::io::Write;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.io.FileOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileOutputStream";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "close0", "()V", close_0);
    }

    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "open0", "(Ljava/lang/String;Z)V", open_0);
    registry.register(class_name, "write", "(IZ)V", write);
    registry.register(class_name, "writeBytes", "([BIIZ)V", write_bytes);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.close0()V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.open0(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.write(IZ)V")
}

#[async_recursion(?Send)]
async fn write_bytes(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _append = arguments.pop_int()? != 0;
    let length = usize::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_int()?)?;
    let bytes: Vec<u8> = arguments.pop()?.try_into()?;
    let file_output_stream = arguments.pop_object()?;
    let file_descriptor: Object = file_output_stream.value("fd")?.try_into()?;
    let handle = file_descriptor.value("handle")?.to_long()?;

    match handle {
        1 => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            stdout
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stdout
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        2 => {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            stderr
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stderr
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        _ => {
            return Err(InternalError(format!("Invalid file handle: {handle}")));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/io/FileOutputStream";
        assert!(registry.method(class_name, "close0", "()V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "open0", "(Ljava/lang/String;Z)V")
            .is_some());
        assert!(registry.method(class_name, "write", "(IZ)V").is_some());
        assert!(registry
            .method(class_name, "writeBytes", "([BIIZ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileOutputStream.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.FileOutputStream.open0(Ljava/lang/String;Z)V"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileOutputStream.write(IZ)V")]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write(thread, Arguments::default()).await;
    }
}
