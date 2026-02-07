use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/String.intern()Ljava/lang/String;", Any)]
#[async_method]
pub async fn intern<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let value = value.as_string()?;

    let value = thread.intern_string(&value).await?;
    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_intern() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = "foo".to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(value.clone());
        let result = intern(thread, parameters).await?;
        assert_eq!(result, Some(value));
        Ok(())
    }
}
