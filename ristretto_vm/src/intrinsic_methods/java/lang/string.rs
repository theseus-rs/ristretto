use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/String.intern()Ljava/lang/String;", Any)]
#[async_method]
pub(crate) async fn intern(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let value = value.as_string()?;

    let vm = thread.vm()?;
    let string_pool = vm.string_pool();
    let value = string_pool.intern(&thread, &value).await?;
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
