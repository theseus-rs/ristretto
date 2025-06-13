use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/StringUTF16.isBigEndian()Z", Between(JAVA_11, JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_big_endian(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_big_endian() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = is_big_endian(thread, Parameters::default()).await?;
        assert_eq!(value, Some(Value::from(true)));
        Ok(())
    }
}
