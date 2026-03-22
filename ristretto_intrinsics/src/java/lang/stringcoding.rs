use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Writes an error message to stderr.
///
/// This implements the native method `java.lang.StringCoding.err(Ljava/lang/String;)V`
/// which is used internally by `StringCoding` to report encoding/decoding errors.
#[intrinsic_method("java/lang/StringCoding.err(Ljava/lang/String;)V", Equal(JAVA_11))]
#[async_method]
pub async fn err<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let message = parameters.pop()?.as_string()?;
    let vm = thread.vm()?;
    let stderr = vm.stderr();
    let mut stderr = stderr.lock().await;
    write!(stderr, "{message}")?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_err() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let message = "test error message".to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(message);
        let result = err(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
