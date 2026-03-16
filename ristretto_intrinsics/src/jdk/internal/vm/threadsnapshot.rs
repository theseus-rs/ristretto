use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/vm/ThreadSnapshot.create(Ljava/lang/Thread;)Ljdk/internal/vm/ThreadSnapshot;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _thread_param = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.ThreadSnapshot.create(Ljava/lang/Thread;)Ljdk/internal/vm/ThreadSnapshot;"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::new(vec![Value::Object(None)]);
        let result = create(thread, parameters).await;
        assert!(result.is_err());
    }
}
