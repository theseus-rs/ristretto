use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn attach<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/misc/Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_byte_array<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/misc/Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_long<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.misc.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/misc/Perf.detach(Ljava/nio/ByteBuffer;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn detach<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.misc.Perf.detach(Ljava/nio/ByteBuffer;)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/misc/Perf.highResCounter()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn high_res_counter<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.misc.Perf.highResCounter()J".to_string()).into())
}

#[intrinsic_method("sun/misc/Perf.highResFrequency()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn high_res_frequency<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.misc.Perf.highResFrequency()J".to_string()).into())
}

#[intrinsic_method("sun/misc/Perf.registerNatives()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_attach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = attach(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_byte_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_byte_array(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_long(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detach() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = detach(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_high_res_counter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = high_res_counter(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_high_res_frequency() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = high_res_frequency(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
