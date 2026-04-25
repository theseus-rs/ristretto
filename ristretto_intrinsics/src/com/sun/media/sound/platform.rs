use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetExtraLibraries()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn n_get_extra_libraries<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetLibraryForFeature(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn n_get_library_for_feature<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _feature = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.media.sound.Platform.nGetLibraryForFeature(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsBigEndian()Z", Any)]
#[async_method]
pub async fn n_is_big_endian<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let big_endian = cfg!(target_endian = "big");
    Ok(Some(Value::from(big_endian)))
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsSigned8()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn n_is_signed_8<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("com.sun.media.sound.Platform.nIsSigned8()Z".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_extra_libraries() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = n_get_extra_libraries(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_get_library_for_feature() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = n_get_library_for_feature(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.media.sound.Platform.nGetLibraryForFeature(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_n_is_big_endian() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_is_big_endian(thread, Parameters::default()).await?;
        let big_endian = cfg!(target_endian = "big");
        assert_eq!(result, Some(Value::from(big_endian)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_is_signed_8() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = n_is_signed_8(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.media.sound.Platform.nIsSigned8()Z",
            result.unwrap_err().to_string()
        );
    }
}
