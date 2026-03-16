use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Returns an array of indices into the lookup cache URLs that may contain the specified resource.
/// This is part of the CDS (Class Data Sharing) lookup cache optimization. Since Ristretto does
/// not implement CDS, this returns null to indicate no cache is available, causing the caller to
/// fall back to a full classpath search.
#[intrinsic_method(
    "sun/misc/URLClassPath.getLookupCacheForClassLoader(Ljava/lang/ClassLoader;Ljava/lang/String;)[I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_lookup_cache_for_class_loader<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

/// Returns the lookup cache URLs for the specified class loader. This is part of the CDS (Class
/// Data Sharing) shared lookup cache. Since Ristretto does not implement CDS, this returns null,
/// causing `initLookupCache()` to call `disableAllLookupCaches()`.
#[intrinsic_method(
    "sun/misc/URLClassPath.getLookupCacheURLs(Ljava/lang/ClassLoader;)[Ljava/net/URL;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_lookup_cache_urls<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

/// Returns whether a class is known to not exist in the class loader's classpath. This is a
/// negative lookup optimization used with the CDS lookup cache. Since Ristretto does not implement
/// CDS, this returns false to indicate we do not know whether the class exists, forcing a full
/// classpath search.
#[intrinsic_method(
    "sun/misc/URLClassPath.knownToNotExist0(Ljava/lang/ClassLoader;Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn known_to_not_exist_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_lookup_cache_for_class_loader() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_lookup_cache_for_class_loader(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_lookup_cache_urls() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_lookup_cache_urls(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_known_to_not_exist_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = known_to_not_exist_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }
}
