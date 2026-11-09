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
pub async fn get_memory_usage_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let heap = parameters.pop_bool()?;

    // The VM has no memory pools or heap cap. Use the collector's live allocation estimate
    // for used and committed; the scheduling counter resets even when objects survive GC.
    // Leave `max` undefined; consumers treat `-1` as unset.
    let (init, used, committed, max) = if heap {
        let statistics = thread.vm()?.garbage_collector().statistics()?;
        let live_bytes = i64::try_from(statistics.bytes_live)?;
        (0, live_bytes, live_bytes, -1)
    } else {
        (-1, 0, 0, -1)
    };
    let values = [
        Value::Long(init),
        Value::Long(used),
        Value::Long(committed),
        Value::Long(max),
    ];
    let memory_usage = thread
        .object("java.lang.management.MemoryUsage", "JJJJ", &values)
        .await?;
    Ok(Some(memory_usage))
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
    #[cfg(not(target_family = "wasm"))]
    use ristretto_classloader::Reference;
    #[cfg(not(target_family = "wasm"))]
    use std::time::Duration;

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
    async fn test_get_memory_usage_0_heap() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_memory_usage_0(thread, Parameters::new(vec![Value::from(true)]))
            .await?
            .expect("heap usage");
        let usage = result.as_object_ref()?;
        assert_eq!("java/lang/management/MemoryUsage", usage.class().name());

        let init = usage.value("init")?.as_i64()?;
        let used = usage.value("used")?.as_i64()?;
        let committed = usage.value("committed")?.as_i64()?;
        let max = usage.value("max")?.as_i64()?;
        assert_eq!(0, init);
        assert!(used >= 0);
        assert!(committed >= used);
        assert_eq!(-1, max);
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn test_get_memory_usage_0_heap_after_collection() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let retained = Value::new_object(
            vm.garbage_collector(),
            Reference::ByteArray(vec![42; 1024 * 1024].into_boxed_slice()),
        );
        let before = get_memory_usage_0(thread.clone(), Parameters::new(vec![Value::from(true)]))
            .await?
            .expect("heap usage");
        let before_used = before.as_object_ref()?.value("used")?.as_i64()?;
        assert!(before_used > 0);

        for _ in 0..2 {
            let completed = vm.garbage_collector().statistics()?.collections_completed;
            vm.gc();
            tokio::time::timeout(Duration::from_secs(5), async {
                while vm.garbage_collector().statistics()?.collections_completed == completed {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
                Ok::<_, ristretto_types::Error>(())
            })
            .await
            .expect("collection completed")?;

            let after =
                get_memory_usage_0(thread.clone(), Parameters::new(vec![Value::from(true)]))
                    .await?
                    .expect("heap usage");
            let usage = after.as_object_ref()?;
            let used = usage.value("used")?.as_i64()?;
            let committed = usage.value("committed")?.as_i64()?;
            assert!(used >= before_used, "live heap usage was lost during GC");
            assert!(committed >= used);
            let array = retained.as_reference()?;
            let bytes = array.as_byte_vec_ref()?;
            assert_eq!(bytes.len(), 1024 * 1024);
            assert!(bytes.iter().all(|&byte| byte == 42));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_memory_usage_0_non_heap() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_memory_usage_0(thread, Parameters::new(vec![Value::from(false)]))
            .await?
            .expect("non-heap usage");
        let usage = result.as_object_ref()?;
        assert_eq!("java/lang/management/MemoryUsage", usage.class().name());

        let init = usage.value("init")?.as_i64()?;
        let used = usage.value("used")?.as_i64()?;
        let committed = usage.value("committed")?.as_i64()?;
        let max = usage.value("max")?.as_i64()?;
        assert_eq!((-1, 0, 0, -1), (init, used, committed, max));
        Ok(())
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
