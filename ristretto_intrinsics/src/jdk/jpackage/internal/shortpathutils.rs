use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jpackage/internal/ShortPathUtils.getShortPath(Ljava/lang/String;)Ljava/lang/String;",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn get_short_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_long_path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ShortPathUtils.getShortPath(Ljava/lang/String;)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_short_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_short_path(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk/jpackage/internal/ShortPathUtils.getShortPath(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
