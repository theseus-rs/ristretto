use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/String.intern()Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn intern(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    // TODO: implement string interning
    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;

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
