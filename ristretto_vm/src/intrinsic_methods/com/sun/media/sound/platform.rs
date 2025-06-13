use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetExtraLibraries()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_extra_libraries(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;")
}

#[intrinsic_method(
    "com/sun/media/sound/Platform.nGetLibraryForFeature(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn n_get_library_for_feature(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetLibraryForFeature(I)I")
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsBigEndian()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn n_is_big_endian(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nIsBigEndian()Z")
}

#[intrinsic_method("com/sun/media/sound/Platform.nIsSigned8()Z", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn n_is_signed_8(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nIsSigned8()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;"
    )]
    async fn test_n_get_extra_libraries() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_extra_libraries(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.Platform.nGetLibraryForFeature(I)I"
    )]
    async fn test_n_get_library_for_feature() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_library_for_feature(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.Platform.nIsBigEndian()Z")]
    async fn test_n_is_big_endian() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_big_endian(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.Platform.nIsSigned8()Z")]
    async fn test_n_is_signed_8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_signed_8(thread, Parameters::default()).await;
    }
}
