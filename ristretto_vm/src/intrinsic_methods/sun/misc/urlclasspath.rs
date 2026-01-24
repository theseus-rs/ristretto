use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_lookup_cache_for_class_loader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.misc.URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I"
    )
}

#[intrinsic_method(
    "sun/misc/URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_lookup_cache_urls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;")
}

#[intrinsic_method(
    "sun/misc/URLClassPath.knownToNotExist0(Ljava/lang/ClassLoader;Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn known_to_not_exist_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.knownToNotExist0(Ljava/lang/ClassLoader;Ljava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I"
    )]
    async fn test_get_lookup_cache_for_class_loader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_lookup_cache_for_class_loader(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;"
    )]
    async fn test_get_lookup_cache_urls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_lookup_cache_urls(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.URLClassPath.knownToNotExist0(Ljava/lang/ClassLoader;Ljava/lang/String;)Z"
    )]
    async fn test_known_to_not_exist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = known_to_not_exist_0(thread, Parameters::default()).await;
    }
}
