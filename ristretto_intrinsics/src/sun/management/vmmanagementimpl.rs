//! Native methods for `sun.management.VMManagementImpl`.
//!
//! Counters that this VM does not track report conservative values (0 or false). Process identity, startup time and uptime come from the host; wasm uses a logical PID because it has no host process table.

use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::{Arc, LazyLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Management version reported by the VM.
///
/// `sun.management.VMManagementImpl` asserts this value during class initialization.
const MANAGEMENT_VERSION: &str = "1.2";

/// Returns the host process ID.
#[cfg(not(target_family = "wasm"))]
fn process_id() -> i32 {
    i32::try_from(std::process::id()).unwrap_or(i32::MAX)
}

/// Returns a stable logical PID for the wasm sandbox.
///
/// wasm has no host process table, and `std::process::id()` aborts on this target.
#[cfg(target_family = "wasm")]
fn process_id() -> i32 {
    1
}

/// Captures the wall-clock start time and the monotonic start instant together.
struct StartClock {
    startup_millis: i64,
    started: Instant,
}

static START_CLOCK: LazyLock<StartClock> = LazyLock::new(|| StartClock {
    startup_millis: now_millis(),
    started: Instant::now(),
});

/// Returns the current wall-clock time in milliseconds since the epoch.
fn now_millis() -> i64 {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH);
    let millis = elapsed.map_or(0, |duration| duration.as_millis());
    i64::try_from(millis).unwrap_or(i64::MAX)
}

/// Returns the wall-clock start time in milliseconds since the epoch.
fn startup_millis() -> i64 {
    START_CLOCK.startup_millis
}

/// Returns the elapsed time since the captured start instant in milliseconds.
fn uptime_millis() -> i64 {
    i64::try_from(START_CLOCK.started.elapsed().as_millis()).unwrap_or(i64::MAX)
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.endAOTRecording()Z",
    GreaterThanOrEqual(JAVA_25)
)]
pub fn end_aot_recording<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getAvailableProcessors()I", Any)]
pub fn get_available_processors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let processors = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let processors = i32::try_from(processors).unwrap_or(i32::MAX);
    Ok(Some(Value::Int(processors)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassInitializationTime()J", Any)]
pub fn get_class_initialization_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassLoadingTime()J", Any)]
pub fn get_class_loading_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassVerificationTime()J", Any)]
pub fn get_class_verification_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getDaemonThreadCount()I", Any)]
pub fn get_daemon_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getInitializedClassCount()J", Any)]
pub fn get_initialized_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getLiveThreadCount()I", Any)]
pub fn get_live_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getLoadedClassSize()J", Any)]
pub fn get_loaded_class_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getMethodDataSize()J", Any)]
pub fn get_method_data_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getPeakThreadCount()I", Any)]
pub fn get_peak_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getProcessId()I", Any)]
pub fn get_process_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let pid = process_id();
    Ok(Some(Value::Int(pid)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointCount()J", Any)]
pub fn get_safepoint_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointSyncTime()J", Any)]
pub fn get_safepoint_sync_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getStartupTime()J", Any)]
pub fn get_startup_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(startup_millis())))
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getTotalApplicationNonStoppedTime()J",
    Any
)]
pub fn get_total_application_non_stopped_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalClassCount()J", Any)]
pub fn get_total_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalCompileTime()J", Any)]
pub fn get_total_compile_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalSafepointTime()J", Any)]
pub fn get_total_safepoint_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalThreadCount()J", Any)]
pub fn get_total_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassCount()J", Any)]
pub fn get_unloaded_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassSize()J", Any)]
pub fn get_unloaded_class_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getUptime0()J", Any)]
pub fn get_uptime_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(uptime_millis())))
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseClass()Z", Any)]
pub fn get_verbose_class<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseGC()Z", Any)]
pub fn get_verbose_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/management/VMManagementImpl.getVersion0()Ljava/lang/String;", Any)]
pub async fn get_version_0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let version = MANAGEMENT_VERSION.to_object(&*thread).await?;
    Ok(Some(version))
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getVmArguments0()[Ljava/lang/String;",
    Any
)]
pub async fn get_vm_arguments_0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // The VM is not started from a command line, so it has no input arguments.
    let class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((class, Vec::<Value>::new()))?;
    let vm = thread.vm()?;
    Ok(Some(Value::new_object(vm.garbage_collector(), reference)))
}

/// Optional support fields are initialized from Java threads, so there is nothing to do here.
#[intrinsic_method("sun/management/VMManagementImpl.initOptionalSupportFields()V", Any)]
pub fn init_optional_support_fields<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadAllocatedMemoryEnabled()Z",
    Any
)]
pub fn is_thread_allocated_memory_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadContentionMonitoringEnabled()Z",
    Any
)]
pub fn is_thread_contention_monitoring_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("sun/management/VMManagementImpl.isThreadCpuTimeEnabled()Z", Any)]
pub fn is_thread_cpu_time_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_end_aot_recording() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = end_aot_recording(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }

    #[tokio::test]
    async fn test_get_available_processors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_available_processors(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(value.as_i32().expect("int") > 0);
    }

    #[tokio::test]
    async fn test_get_class_initialization_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_class_initialization_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_class_loading_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_class_loading_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_class_verification_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_class_verification_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_daemon_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_daemon_thread_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i32().expect("int"));
    }

    #[tokio::test]
    async fn test_get_initialized_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_initialized_class_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_live_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_live_thread_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i32().expect("int"));
    }

    #[tokio::test]
    async fn test_get_loaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_loaded_class_size(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_method_data_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_method_data_size(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_peak_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_peak_thread_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i32().expect("int"));
    }

    #[cfg(target_family = "wasm")]
    #[test]
    fn test_process_id_on_wasm() {
        assert_eq!(1, process_id());
    }

    #[tokio::test]
    async fn test_get_process_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_process_id(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(process_id(), value.as_i32().expect("int"));
    }

    #[tokio::test]
    async fn test_get_safepoint_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_safepoint_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_safepoint_sync_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_safepoint_sync_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_startup_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_startup_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(value.as_i64().expect("long") > 0);
    }

    #[tokio::test]
    async fn test_get_total_application_non_stopped_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_total_application_non_stopped_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_total_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_total_class_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_total_compile_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_total_compile_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_total_safepoint_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_total_safepoint_time(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_total_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_total_thread_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_unloaded_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_unloaded_class_count(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_unloaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_unloaded_class_size(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert_eq!(0, value.as_i64().expect("long"));
    }

    #[tokio::test]
    async fn test_get_uptime_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_uptime_0(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(value.as_i64().expect("long") >= 0);
    }

    #[tokio::test]
    async fn test_get_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_verbose_class(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }

    #[tokio::test]
    async fn test_get_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_verbose_gc(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }

    #[tokio::test]
    async fn test_get_version_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_version_0(thread, Parameters::default())
            .await
            .expect("value")
            .expect("value");
        assert_eq!("1.2", value.as_string().expect("string"));
    }

    #[tokio::test]
    async fn test_get_vm_arguments_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = get_vm_arguments_0(thread, Parameters::default())
            .await
            .expect("value")
            .expect("value");
        let (class, elements) = value.as_class_vec_ref().expect("array");
        assert_eq!("[Ljava/lang/String;", class.name());
        assert!(elements.is_empty());
    }

    #[tokio::test]
    async fn test_init_optional_support_fields() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = init_optional_support_fields(thread, Parameters::default()).expect("value");
        assert!(value.is_none());
    }

    #[tokio::test]
    async fn test_is_thread_allocated_memory_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = is_thread_allocated_memory_enabled(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }

    #[tokio::test]
    async fn test_is_thread_contention_monitoring_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = is_thread_contention_monitoring_enabled(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }

    #[tokio::test]
    async fn test_is_thread_cpu_time_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = is_thread_cpu_time_enabled(thread, Parameters::default())
            .expect("value")
            .expect("value");
        assert!(!value.as_bool().expect("bool"));
    }
}
