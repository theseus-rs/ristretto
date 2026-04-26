use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getCommittedVirtualMemorySize0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_committed_virtual_memory_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getCommittedVirtualMemorySize0()J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getCpuLoad0()D",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_cpu_load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getCpuLoad0()D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getFreePhysicalMemorySize0()J",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_free_physical_memory_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getFreePhysicalMemorySize0()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getFreeMemorySize0()J",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_free_memory_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getFreeMemorySize0()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getFreeSwapSpaceSize0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_free_swap_space_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getFreeSwapSpaceSize0()J".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getHostConfiguredCpuCount0()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_host_configured_cpu_count_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getHostConfiguredCpuCount0()I".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getHostOnlineCpuCount0()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_host_online_cpu_count_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getHostOnlineCpuCount0()I".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getHostTotalCpuTicks0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_host_total_cpu_ticks_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getHostTotalCpuTicks0()J".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getMaxFileDescriptorCount0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_max_file_descriptor_count_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getMaxFileDescriptorCount0()J".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getOpenFileDescriptorCount0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_open_file_descriptor_count_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getOpenFileDescriptorCount0()J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getProcessCpuLoad0()D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_process_cpu_load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getProcessCpuLoad0()D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getProcessCpuTime0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_process_cpu_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getProcessCpuTime0()J".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getSingleCpuLoad0(I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_single_cpu_load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cpu_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getSingleCpuLoad0(I)D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getSystemCpuLoad0()D",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_system_cpu_load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getSystemCpuLoad0()D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getTotalMemorySize0()J",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_total_memory_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getTotalMemorySize0()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getTotalPhysicalMemorySize0()J",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_total_physical_memory_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getTotalPhysicalMemorySize0()J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.getTotalSwapSpaceSize0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_total_swap_space_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.OperatingSystemImpl.getTotalSwapSpaceSize0()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/OperatingSystemImpl.initialize0()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(all(test, target_family = "unix"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_committed_virtual_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_committed_virtual_memory_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getCommittedVirtualMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cpu_load_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getCpuLoad0()D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_free_physical_memory_size_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_free_physical_memory_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getFreePhysicalMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_free_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_free_memory_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getFreeMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_free_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_free_swap_space_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getFreeSwapSpaceSize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_host_configured_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_host_configured_cpu_count_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getHostConfiguredCpuCount0()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_host_online_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_host_online_cpu_count_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getHostOnlineCpuCount0()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_host_total_cpu_ticks_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_host_total_cpu_ticks_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getHostTotalCpuTicks0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_max_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_max_file_descriptor_count_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getMaxFileDescriptorCount0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_open_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_open_file_descriptor_count_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getOpenFileDescriptorCount0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_process_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_process_cpu_load_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getProcessCpuLoad0()D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_process_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_process_cpu_time_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getProcessCpuTime0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_single_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_single_cpu_load_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getSingleCpuLoad0(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_system_cpu_load_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_system_cpu_load_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getSystemCpuLoad0()D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_total_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_memory_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getTotalMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_total_physical_memory_size_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_total_physical_memory_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getTotalPhysicalMemorySize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_total_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_swap_space_size_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.OperatingSystemImpl.getTotalSwapSpaceSize0()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_initialize_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_0(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
