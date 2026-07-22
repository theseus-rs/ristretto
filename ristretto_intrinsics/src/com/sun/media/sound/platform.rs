use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetExtraLibraries()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn n_get_extra_libraries<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetLibraryForFeature(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn n_get_library_for_feature<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsBigEndian()Z", Any)]
#[async_method]
pub async fn n_is_big_endian<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let big_endian = cfg!(target_endian = "big");
    Ok(Some(Value::from(big_endian)))
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsSigned8()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn n_is_signed_8<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_n_get_extra_libraries() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_extra_libraries(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_n_get_library_for_feature() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_get_library_for_feature(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
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
    async fn test_n_is_signed_8() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = n_is_signed_8(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }
}
