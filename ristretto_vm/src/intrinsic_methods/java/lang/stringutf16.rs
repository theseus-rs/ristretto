use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/StringUTF16.isBigEndian()Z", Between(JAVA_11, JAVA_21))]
#[async_method]
pub(crate) async fn is_big_endian(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let big_endian = cfg!(target_endian = "big");
    Ok(Some(Value::from(big_endian)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_big_endian() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = is_big_endian(thread, Parameters::default()).await?;
        let big_endian = cfg!(target_endian = "big");
        assert_eq!(value, Some(Value::from(big_endian)));
        Ok(())
    }
}
