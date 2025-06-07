use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/perf/Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
    Between(JAVA_11, JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn attach(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.attach0(I)Ljava/nio/ByteBuffer;",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn attach_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.attach0(I)Ljava/nio/ByteBuffer;")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_byte_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.detach(Ljava/nio/ByteBuffer;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn detach(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.detach(Ljava/nio/ByteBuffer;)V")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.highResCounter()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn high_res_counter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResCounter()J")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.highResFrequency()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn high_res_frequency(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.perf.Perf.highResFrequency()J")
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.registerNatives()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
