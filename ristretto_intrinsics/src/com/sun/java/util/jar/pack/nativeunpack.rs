use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.finish()J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn finish<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.finish()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getNextFile([Ljava/lang/Object;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_next_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_parts = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_prop = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_unused_input<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.initIDs()V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_value = parameters.pop_reference()?;
    let _p_prop = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/util/jar/pack/NativeUnpack.start(Ljava/nio/ByteBuffer;J)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn start<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _offset = parameters.pop_long()?;
    let _buf = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.util.jar.pack.NativeUnpack.start(Ljava/nio/ByteBuffer;J)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_finish() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = finish(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.finish()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_next_file() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_next_file(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.getNextFile([Ljava/lang/Object;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_option() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_option(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.getOption(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_unused_input() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_unused_input(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.getUnusedInput()Ljava/nio/ByteBuffer;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_option() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = set_option(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.setOption(Ljava/lang/String;Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_start() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = start(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.java.util.jar.pack.NativeUnpack.start(Ljava/nio/ByteBuffer;J)J",
            result.unwrap_err().to_string()
        );
    }
}
