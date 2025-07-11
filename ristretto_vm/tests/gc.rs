use ristretto_vm::{Result, VM};
use std::time::Duration;

#[tokio::test]
async fn test_basic_gc() -> Result<()> {
    let vm = VM::default().await?;

    vm.gc();
    tokio::time::sleep(Duration::from_millis(100)).await; // Allow time for the collector to run
    let vm_statistics = vm.statistics();

    {
        let _object = vm.object("java.lang.Integer", "I", &[42]).await?;
        let object_statistics = vm.statistics();

        assert!(object_statistics.bytes_allocated > vm_statistics.bytes_allocated);
    }

    vm.gc();
    tokio::time::sleep(Duration::from_millis(100)).await; // Allow time for the collector to run
    let collector_statistics = vm.statistics();

    assert_eq!(
        collector_statistics.bytes_allocated,
        vm_statistics.bytes_allocated
    );

    Ok(())
}
