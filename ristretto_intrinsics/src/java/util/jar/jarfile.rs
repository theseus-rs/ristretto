use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/util/jar/JarFile.getMetaInfEntryNames()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_meta_inf_entry_names<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;"
    )]
    async fn test_get_meta_inf_entry_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_meta_inf_entry_names(thread, Parameters::default()).await;
    }
}
