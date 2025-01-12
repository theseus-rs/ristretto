use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `com.sun.media.sound.Platform`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/media/sound/Platform";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "nGetExtraLibraries",
            "()Ljava/lang/String;",
            n_get_extra_libraries,
        );
        registry.register(
            class_name,
            "nGetLibraryForFeature",
            "(I)I",
            n_get_library_for_feature,
        );
        registry.register(class_name, "nIsSigned8", "()Z", n_is_signed_8);
    }

    registry.register(class_name, "nIsBigEndian", "()Z", n_is_big_endian);
}

#[async_recursion(?Send)]
async fn n_get_extra_libraries(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn n_get_library_for_feature(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nGetLibraryForFeature(I)I")
}

#[async_recursion(?Send)]
async fn n_is_big_endian(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nIsBigEndian()Z")
}

#[async_recursion(?Send)]
async fn n_is_signed_8(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.media.sound.Platform.nIsSigned8()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/media/sound/Platform";
        assert!(registry
            .method(class_name, "nGetExtraLibraries", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "nGetLibraryForFeature", "(I)I")
            .is_some());
        assert!(registry.method(class_name, "nIsBigEndian", "()Z").is_some());
        assert!(registry.method(class_name, "nIsSigned8", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.Platform.nGetExtraLibraries()Ljava/lang/String;"
    )]
    async fn test_n_get_extra_libraries() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_extra_libraries(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.media.sound.Platform.nGetLibraryForFeature(I)I"
    )]
    async fn test_n_get_library_for_feature() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_get_library_for_feature(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.Platform.nIsBigEndian()Z")]
    async fn test_n_is_big_endian() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_big_endian(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: com.sun.media.sound.Platform.nIsSigned8()Z")]
    async fn test_n_is_signed_8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = n_is_signed_8(thread, Arguments::default()).await;
    }
}
