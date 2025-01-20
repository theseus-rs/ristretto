use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/URLClassPath";

/// Register all native methods for `sun.misc.URLClassPath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getLookupCacheForClassLoader",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;)[I",
        get_lookup_cache_for_class_loader,
    );
    registry.register(
        CLASS_NAME,
        "getLookupCacheURLs",
        "(Ljava/lang/ClassLoader;)[Ljava/net/URL;",
        get_lookup_cache_urls,
    );
    registry.register(
        CLASS_NAME,
        "knownToNotExist0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;)Z",
        known_to_not_exist_0,
    );
}

#[async_recursion(?Send)]
async fn get_lookup_cache_for_class_loader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I")
}

#[async_recursion(?Send)]
async fn get_lookup_cache_urls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;")
}

#[async_recursion(?Send)]
async fn known_to_not_exist_0(
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
