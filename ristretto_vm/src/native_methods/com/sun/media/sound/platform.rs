use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/media/sound/Platform";

/// Register all native methods for `com.sun.media.sound.Platform`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "nGetExtraLibraries",
            "()Ljava/lang/String;",
            n_get_extra_libraries,
        );
        registry.register(
            CLASS_NAME,
            "nGetLibraryForFeature",
            "(I)I",
            n_get_library_for_feature,
        );
        registry.register(CLASS_NAME, "nIsSigned8", "()Z", n_is_signed_8);
    }

    registry.register(CLASS_NAME, "nIsBigEndian", "()Z", n_is_big_endian);
}

#[async_recursion(?Send)]
async fn n_get_extra_libraries(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_library_for_feature(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetLibraryForFeature(I)I")
}

#[async_recursion(?Send)]
async fn n_is_big_endian(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nIsBigEndian()Z")
}

#[async_recursion(?Send)]
async fn n_is_signed_8(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
