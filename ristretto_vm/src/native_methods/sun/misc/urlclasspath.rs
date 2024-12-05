use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.URLClassPath`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/URLClassPath";
    registry.register(
        class_name,
        "getLookupCacheForClassLoader",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;)[I",
        get_lookup_cache_for_class_loader,
    );
    registry.register(
        class_name,
        "getLookupCacheURLs",
        "(Ljava/lang/ClassLoader;)[Ljava/net/URL;",
        get_lookup_cache_urls,
    );
    registry.register(
        class_name,
        "knownToNotExist0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;)Z",
        known_to_not_exist_0,
    );
}

#[async_recursion(?Send)]
async fn get_lookup_cache_for_class_loader(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I")
}

#[async_recursion(?Send)]
async fn get_lookup_cache_urls(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;")
}

#[async_recursion(?Send)]
async fn known_to_not_exist_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.URLClassPath.knownToNotExist0(Ljava/lang/ClassLoader;Ljava/lang/String;)Z")
}
