use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/Flag.getAllFlagNames()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_all_flag_names(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getAllFlagNames()[Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/management/Flag.getFlags([Ljava/lang/String;[Lsun/management/Flag;I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_flags(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getFlags([Ljava/lang/String;[Lsun/management/Flag;I)I")
}

#[intrinsic_method("sun/management/Flag.getInternalFlagCount()I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_internal_flag_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getInternalFlagCount()I")
}

#[intrinsic_method("sun/management/Flag.initialize()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn initialize(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/management/Flag.setBooleanValue(Ljava/lang/String;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_boolean_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setBooleanValue(Ljava/lang/String;Z)V")
}

#[intrinsic_method(
    "sun/management/Flag.setDoubleValue(Ljava/lang/String;D)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_double_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setDoubleValue(Ljava/lang/String;D)V")
}

#[intrinsic_method(
    "sun/management/Flag.setLongValue(Ljava/lang/String;J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_long_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setLongValue(Ljava/lang/String;J)V")
}

#[intrinsic_method(
    "sun/management/Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_string_value(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.getAllFlagNames()[Ljava/lang/String;"
    )]
    async fn test_get_all_flag_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all_flag_names(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.getFlags([Ljava/lang/String;[Lsun/management/Flag;I)I"
    )]
    async fn test_get_flags() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_flags(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.management.Flag.getInternalFlagCount()I")]
    async fn test_get_internal_flag_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_internal_flag_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.setBooleanValue(Ljava/lang/String;Z)V"
    )]
    async fn test_set_boolean_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_boolean_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.setDoubleValue(Ljava/lang/String;D)V"
    )]
    async fn test_set_double_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_double_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.setLongValue(Ljava/lang/String;J)V"
    )]
    async fn test_set_long_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_long_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V"
    )]
    async fn test_set_string_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_string_value(thread, Parameters::default()).await;
    }
}
