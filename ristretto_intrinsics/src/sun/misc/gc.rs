use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/misc/GC.maxObjectInspectionAge()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn max_object_inspection_age<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_max_object_inspection_age() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = max_object_inspection_age(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
