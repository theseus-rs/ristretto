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
    "sun/management/OperatingSystemImpl.getCommittedVirtualMemorySize0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_committed_virtual_memory_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getCommittedVirtualMemorySize0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getFreePhysicalMemorySize0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_free_physical_memory_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getFreePhysicalMemorySize0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getFreeSwapSpaceSize0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_free_swap_space_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getFreeSwapSpaceSize0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getHostConfiguredCpuCount0()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_host_configured_cpu_count_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostConfiguredCpuCount0()I")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getHostOnlineCpuCount0()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_host_online_cpu_count_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostOnlineCpuCount0()I")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getHostTotalCpuTicks0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_host_total_cpu_ticks_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostTotalCpuTicks0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getMaxFileDescriptorCount0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_max_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getMaxFileDescriptorCount0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getOpenFileDescriptorCount0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_open_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getOpenFileDescriptorCount0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getProcessCpuLoad0()D",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_process_cpu_load_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getProcessCpuLoad0()D")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getProcessCpuTime0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_process_cpu_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getProcessCpuTime0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getSingleCpuLoad0(I)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_single_cpu_load_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getSingleCpuLoad0(I)D")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getSystemCpuLoad0()D",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_cpu_load_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getSystemCpuLoad0()D")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getTotalPhysicalMemorySize0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_total_physical_memory_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getTotalPhysicalMemorySize0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.getTotalSwapSpaceSize0()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_total_swap_space_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getTotalSwapSpaceSize0()J")
}

#[intrinsic_method(
    "sun/management/OperatingSystemImpl.initialize0()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn initialize_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getCommittedVirtualMemorySize0()J"
    )]
    async fn test_get_committed_virtual_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_committed_virtual_memory_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getFreePhysicalMemorySize0()J"
    )]
    async fn test_get_free_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_physical_memory_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getFreeSwapSpaceSize0()J"
    )]
    async fn test_get_free_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_swap_space_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostConfiguredCpuCount0()I"
    )]
    async fn test_get_host_configured_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_configured_cpu_count_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostOnlineCpuCount0()I"
    )]
    async fn test_get_host_online_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_online_cpu_count_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostTotalCpuTicks0()J"
    )]
    async fn test_get_host_total_cpu_ticks_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_total_cpu_ticks_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getMaxFileDescriptorCount0()J"
    )]
    async fn test_get_max_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_file_descriptor_count_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getOpenFileDescriptorCount0()J"
    )]
    async fn test_get_open_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_open_file_descriptor_count_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getProcessCpuLoad0()D"
    )]
    async fn test_get_process_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getProcessCpuTime0()J"
    )]
    async fn test_get_process_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getSingleCpuLoad0(I)D"
    )]
    async fn test_get_single_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_single_cpu_load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getSystemCpuLoad0()D"
    )]
    async fn test_get_system_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_cpu_load_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getTotalPhysicalMemorySize0()J"
    )]
    async fn test_get_total_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_physical_memory_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getTotalSwapSpaceSize0()J"
    )]
    async fn test_get_total_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_swap_space_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_initialize_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
