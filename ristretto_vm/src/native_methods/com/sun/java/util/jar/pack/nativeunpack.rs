use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/java/util/jar/pack/NativeUnpack";

/// Register all native methods for `com.sun.java.util.jar.pack.NativeUnpack`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "finish", "()J", finish);
        registry.register(
            CLASS_NAME,
            "getNextFile",
            "([Ljava/lang/Object;)Z",
            get_next_file,
        );
        registry.register(
            CLASS_NAME,
            "getOption",
            "(Ljava/lang/String;)Ljava/lang/String;",
            get_option,
        );
        registry.register(
            CLASS_NAME,
            "getUnusedInput",
            "()Ljava/nio/ByteBuffer;",
            get_unused_input,
        );
        registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
        registry.register(
            CLASS_NAME,
            "setOption",
            "(Ljava/lang/String;Ljava/lang/String;)Z",
            set_option,
        );
        registry.register(CLASS_NAME, "start", "(Ljava/nio/ByteBuffer;J)J", start);
    }
}

#[async_recursion(?Send)]
async fn finish(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.finish()J")
}

#[async_recursion(?Send)]
async fn get_next_file(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_unused_input(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z"
    )
}

#[async_recursion(?Send)]
async fn start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.start(Ljava/nio/ByteBuffer;J)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.finish()J"
    )]
    async fn test_finish() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = finish(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z"
    )]
    async fn test_get_next_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_next_file(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_option(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;"
    )]
    async fn test_get_unused_input() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unused_input(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z"
    )]
    async fn test_set_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_option(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.start(Ljava/nio/ByteBuffer;J)J"
    )]
    async fn test_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = start(thread, Arguments::default()).await;
    }
}
