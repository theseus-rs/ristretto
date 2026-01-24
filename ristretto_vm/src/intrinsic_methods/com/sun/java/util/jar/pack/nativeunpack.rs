use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.finish()J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn finish(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.finish()J")
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getNextFile([Ljava/lang/Object;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_next_file(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z")
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_option(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;")
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_unused_input(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;")
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.initIDs()V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_option(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z"
    )
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.start(Ljava/nio/ByteBuffer;J)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn start(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
        let _ = finish(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z"
    )]
    async fn test_get_next_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_next_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;"
    )]
    async fn test_get_unused_input() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unused_input(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z"
    )]
    async fn test_set_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.util.jar.pack.NativeUnpack.start(Ljava/nio/ByteBuffer;J)J"
    )]
    async fn test_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = start(thread, Parameters::default()).await;
    }
}
