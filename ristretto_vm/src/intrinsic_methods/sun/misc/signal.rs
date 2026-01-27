use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/Signal.findSignal(Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn find_signal(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/misc/Signal.handle0(IJ)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn handle_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/misc/Signal.raise0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn raise_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_signal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_signal(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = handle_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_raise_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = raise_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
