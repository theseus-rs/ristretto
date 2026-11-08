use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryManagers0()[Ljava/lang/management/MemoryManagerMXBean;",
    Any
)]
pub async fn get_memory_managers_0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let name = "Ristretto GC".to_object(&thread).await?;
    let manager = thread
        .object(
            "sun.management.GarbageCollectorImpl",
            "Ljava/lang/String;",
            &[name],
        )
        .await?;
    let array_class = thread
        .class("[Ljava/lang/management/MemoryManagerMXBean;")
        .await?;
    let reference = Reference::try_from((array_class, vec![manager]))?;
    let managers = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(managers))
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
    Any
)]
pub fn get_memory_pools_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;",
    Any
)]
pub fn get_memory_usage_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _heap = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/MemoryImpl.setVerboseGC(Z)V", Any)]
pub fn set_verbose_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.management.MemoryImpl.setVerboseGC(Z)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_memory_managers_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_managers_0(thread, Parameters::default())
            .await
            .expect("get memory managers")
            .expect("memory managers");
        let reference = result.as_reference().expect("memory managers reference");
        let Reference::Array(managers) = &*reference else {
            panic!("expected an object array");
        };
        assert_eq!(
            "[Ljava/lang/management/MemoryManagerMXBean;",
            managers.class.name()
        );
        assert_eq!(1, managers.elements.len());
        let manager = managers
            .elements
            .first()
            .expect("manager element")
            .as_object_ref()
            .expect("manager object");
        assert_eq!(
            "sun/management/GarbageCollectorImpl",
            manager.class().name()
        );
    }

    #[tokio::test]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_pools_0(thread, Parameters::default());
        assert_eq!(
            "sun.management.MemoryImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_memory_usage_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_memory_usage_0(thread, Parameters::new(vec![Value::from(false)]));
        assert_eq!(
            "sun.management.MemoryImpl.getMemoryUsage0(Z)Ljava/lang/management/MemoryUsage;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_verbose_gc(thread, Parameters::new(vec![Value::from(false)]));
        assert_eq!(
            "sun.management.MemoryImpl.setVerboseGC(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
