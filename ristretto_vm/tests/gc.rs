#![expect(
    clippy::panic_in_result_fn,
    reason = "integration tests use assertions in Result-returning async tests"
)]

use ristretto_vm::{Result, VM};
use std::time::Duration;

#[tokio::test]
async fn test_basic_gc() -> Result<()> {
    let vm = VM::default().await?;

    // Wait for explicit completion; background pacing and host scheduling do not
    // promise that an asynchronous request finishes within an arbitrary sleep.
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    // Pre-warm the VM to ensure the class is loaded and static initializers are run
    let _ = vm.object("java.lang.Integer", "I", &[42]).await?;
    // Wait for explicit completion; background pacing and host scheduling do not
    // promise that an asynchronous request finishes within an arbitrary sleep.
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    let vm_statistics = vm.statistics();

    let object_allocated_bytes;
    {
        let _object = vm.object("java.lang.Integer", "I", &[42]).await?;
        object_allocated_bytes = vm.statistics().bytes_allocated;

        assert!(object_allocated_bytes > vm_statistics.bytes_allocated);
    }

    // Wait for explicit completion; background pacing and host scheduling do not
    // promise that an asynchronous request finishes within an arbitrary sleep.
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    let collector_statistics = vm.statistics();

    let diff = collector_statistics
        .bytes_allocated
        .saturating_sub(vm_statistics.bytes_allocated);
    assert!(diff <= 64, "Leaked {diff} bytes");

    if object_allocated_bytes - vm_statistics.bytes_allocated > 64 {
        assert!(object_allocated_bytes > collector_statistics.bytes_allocated);
    } else {
        assert!(object_allocated_bytes >= collector_statistics.bytes_allocated);
    }

    Ok(())
}

#[tokio::test]
async fn weak_referents_are_cleared_without_retaining_the_heap() -> Result<()> {
    use ristretto_classloader::{Reference, Value};
    let vm = VM::default().await?;
    let target = Value::new_object(
        vm.garbage_collector(),
        Reference::ByteArray(vec![0; 1024 * 1024].into()),
    );
    let weak = vm
        .object(
            "java.lang.ref.WeakReference",
            "Ljava/lang/Object;",
            std::slice::from_ref(&target),
        )
        .await?;
    vm.garbage_collector().collect_full()?;
    assert!(matches!(
        weak.as_object_ref()?.value("referent")?,
        Value::Object(Some(_))
    ));
    drop(target);
    vm.garbage_collector().collect_full()?;
    vm.garbage_collector().collect_full()?;
    assert!(matches!(
        weak.as_object_ref()?.value("referent")?,
        Value::Object(None)
    ));
    Ok(())
}

#[tokio::test]
async fn shutdown_releases_daemons_and_preserves_escaped_values() -> Result<()> {
    use ristretto_classloader::{Reference, Value};
    let vm = VM::default().await?;
    let weak_vm = std::sync::Arc::downgrade(&vm);
    let weak_gc = std::sync::Arc::downgrade(vm.garbage_collector());
    let escaped =
        Value::try_new_object(vm.garbage_collector(), Reference::IntArray(vec![42].into()))?;
    drop(vm);
    assert!(weak_vm.upgrade().is_none());
    assert_eq!(escaped.as_reference()?.as_int_vec_ref()?, &[42]);
    drop(escaped);
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while weak_gc.upgrade().is_some() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "shutdown leaked collector roots"
        );
        tokio::task::yield_now().await;
    }
    Ok(())
}
