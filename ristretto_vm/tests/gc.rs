use ristretto_vm::{Result, VM};
use std::time::Duration;

#[tokio::test]
async fn test_basic_gc() -> Result<()> {
    let vm = VM::default().await?;

    vm.gc();
    // Allow time for the collector to run
    tokio::time::sleep(Duration::from_millis(100)).await;
    // Pre-warm the VM to ensure the class is loaded and static initializers are run
    let _ = vm.object("java.lang.Integer", "I", &[42]).await?;
    vm.gc();
    // Allow time for the collector to run
    tokio::time::sleep(Duration::from_millis(100)).await;
    let vm_statistics = vm.statistics();

    let object_allocated_bytes;
    {
        let _object = vm.object("java.lang.Integer", "I", &[42]).await?;
        object_allocated_bytes = vm.statistics().bytes_allocated;

        assert!(object_allocated_bytes > vm_statistics.bytes_allocated);
    }

    vm.gc();
    // Allow time for the collector to run
    tokio::time::sleep(Duration::from_millis(100)).await;
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
