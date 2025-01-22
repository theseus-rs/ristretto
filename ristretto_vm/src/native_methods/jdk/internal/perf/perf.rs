use crate::native_methods::registry::{MethodRegistry, JAVA_17};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/perf/Perf";

/// Register all native methods for `jdk.internal.perf.Perf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "attach",
            "(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
            attach,
        );
    } else {
        registry.register(CLASS_NAME, "attach0", "(I)Ljava/nio/ByteBuffer;", attach_0);
    }

    registry.register(
        CLASS_NAME,
        "createByteArray",
        "(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
        create_byte_array,
    );
    registry.register(
        CLASS_NAME,
        "createLong",
        "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
        create_long,
    );
    registry.register(CLASS_NAME, "detach", "(Ljava/nio/ByteBuffer;)V", detach);
    registry.register(CLASS_NAME, "highResCounter", "()J", high_res_counter);
    registry.register(CLASS_NAME, "highResFrequency", "()J", high_res_frequency);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn attach(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn attach_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach0(I)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_byte_array(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_long(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn detach(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.detach(Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn high_res_counter(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResCounter()J")
}

#[async_recursion(?Send)]
async fn high_res_frequency(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResFrequency()J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.perf.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;"
    )]
    async fn test_attach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = attach(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.perf.Perf.attach0(I)Ljava/nio/ByteBuffer;"
    )]
    async fn test_attach_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = attach_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_byte_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_byte_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.perf.Perf.detach(Ljava/nio/ByteBuffer;)V"
    )]
    async fn test_detach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = detach(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.perf.Perf.highResCounter()J")]
    async fn test_high_res_counter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = high_res_counter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.perf.Perf.highResFrequency()J")]
    async fn test_high_res_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = high_res_frequency(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
