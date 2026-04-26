use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fill_gc_attribute_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _descriptions = parameters.pop_reference()?;
    let _types = parameters.pop_reference()?;
    let _attribute_names = parameters.pop_reference()?;
    let _num_attributes = parameters.pop_int()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_last_gc_info_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _usage_after_gc = parameters.pop_reference()?;
    let _usage_before_gc = parameters.pop_reference()?;
    let _ext_att_types = parameters.pop_reference()?;
    let _ext_att_values = parameters.pop_reference()?;
    let _ext_att_count = parameters.pop_int()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_num_gc_ext_attributes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fill_gc_attribute_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_gc_attribute_info(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_last_gc_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_last_gc_info_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_num_gc_ext_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_num_gc_ext_attributes(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I",
            result.unwrap_err().to_string()
        );
    }
}
