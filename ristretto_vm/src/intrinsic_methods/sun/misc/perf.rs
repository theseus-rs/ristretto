use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/Perf";

/// Register all intrinsic methods for `sun.misc.Perf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "attach",
        "(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
        attach,
    );
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
    todo!("sun.misc.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_byte_array(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_long(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn detach(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.detach(Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn high_res_counter(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.highResCounter()J")
}

#[async_recursion(?Send)]
async fn high_res_frequency(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.highResFrequency()J")
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
        expected = "not yet implemented: sun.misc.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;"
    )]
    async fn test_attach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = attach(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_byte_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_byte_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;"
    )]
    async fn test_create_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Perf.detach(Ljava/nio/ByteBuffer;)V")]
    async fn test_detach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = detach(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Perf.highResCounter()J")]
    async fn test_high_res_counter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = high_res_counter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Perf.highResFrequency()J")]
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
