use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/Flag";

/// Register all intrinsic methods for `sun.management.Flag`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getAllFlagNames",
        "()[Ljava/lang/String;",
        get_all_flag_names,
    );
    registry.register(
        CLASS_NAME,
        "getFlags",
        "([Ljava/lang/String;[Lsun/management/Flag;I)I",
        get_flags,
    );
    registry.register(
        CLASS_NAME,
        "getInternalFlagCount",
        "()I",
        get_internal_flag_count,
    );
    registry.register(CLASS_NAME, "initialize", "()V", initialize);
    registry.register(
        CLASS_NAME,
        "setBooleanValue",
        "(Ljava/lang/String;Z)V",
        set_boolean_value,
    );
    registry.register(
        CLASS_NAME,
        "setDoubleValue",
        "(Ljava/lang/String;D)V",
        set_double_value,
    );
    registry.register(
        CLASS_NAME,
        "setLongValue",
        "(Ljava/lang/String;J)V",
        set_long_value,
    );
    registry.register(
        CLASS_NAME,
        "setStringValue",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        set_string_value,
    );
}

#[async_recursion(?Send)]
async fn get_all_flag_names(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getAllFlagNames()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_flags(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getFlags([Ljava/lang/String;[Lsun/management/Flag;I)I")
}

#[async_recursion(?Send)]
async fn get_internal_flag_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.Flag.getInternalFlagCount()I")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.Flag.initialize()V")
}

#[async_recursion(?Send)]
async fn set_boolean_value(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setBooleanValue(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn set_double_value(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setDoubleValue(Ljava/lang/String;D)V")
}

#[async_recursion(?Send)]
async fn set_long_value(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.Flag.setLongValue(Ljava/lang/String;J)V")
}

#[async_recursion(?Send)]
async fn set_string_value(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
    #[should_panic(expected = "not yet implemented: sun.management.Flag.initialize()V")]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize(thread, Parameters::default()).await;
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
