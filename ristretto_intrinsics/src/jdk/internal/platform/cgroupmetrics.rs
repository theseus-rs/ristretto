use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_os = "linux"))]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};
use std::sync::Arc;

const GET_TOTAL_MEMORY_SIZE: &str = "jdk/internal/platform/CgroupMetrics.getTotalMemorySize0()J";
const GET_TOTAL_SWAP_SIZE: &str = "jdk/internal/platform/CgroupMetrics.getTotalSwapSize0()J";
const IS_CONTAINERIZED: &str = "jdk/internal/platform/CgroupMetrics.isContainerized0()Z";
const IS_USE_CONTAINER_SUPPORT: &str =
    "jdk/internal/platform/CgroupMetrics.isUseContainerSupport()Z";
#[cfg(target_os = "linux")]
const PROC_MEMINFO: &str = "/proc/meminfo";
#[cfg(target_os = "linux")]
const PROC_SELF_CGROUP: &str = "/proc/self/cgroup";
#[cfg(target_os = "linux")]
const PROC_SELF_MOUNTINFO: &str = "/proc/self/mountinfo";

#[cfg(not(target_os = "linux"))]
fn unsupported_operation_error(method: &str) -> Result<Option<Value>> {
    Err(JavaError::UnsupportedOperationException(method.to_string()).into())
}

#[intrinsic_method("jdk/internal/platform/CgroupMetrics.getTotalMemorySize0()J", Any)]
#[async_method]
pub async fn get_total_memory_size0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        Ok(Some(Value::Long(total_memory_size().await)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        unsupported_operation_error(GET_TOTAL_MEMORY_SIZE)
    }
}
#[intrinsic_method(
    "jdk/internal/platform/CgroupMetrics.getTotalSwapSize0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_total_swap_size0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        Ok(Some(Value::Long(total_swap_size().await)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        unsupported_operation_error(GET_TOTAL_SWAP_SIZE)
    }
}
#[intrinsic_method(
    "jdk/internal/platform/CgroupMetrics.isContainerized0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_containerized0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        Ok(Some(Value::from(is_containerized().await)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        unsupported_operation_error(IS_CONTAINERIZED)
    }
}
#[intrinsic_method("jdk/internal/platform/CgroupMetrics.isUseContainerSupport()Z", Any)]
#[async_method]
pub async fn is_use_container_support<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        Ok(Some(Value::from(true)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        unsupported_operation_error(IS_USE_CONTAINER_SUPPORT)
    }
}

#[cfg(target_os = "linux")]
async fn total_memory_size() -> i64 {
    let memory_kib = meminfo_value_kib("MemTotal").await;
    memory_kib.map_or(0, kib_to_bytes)
}

#[cfg(target_os = "linux")]
async fn total_swap_size() -> i64 {
    let swap_kib = meminfo_value_kib("SwapTotal").await;
    swap_kib.map_or(0, kib_to_bytes)
}

#[cfg(target_os = "linux")]
async fn meminfo_value_kib(key: &str) -> Option<u64> {
    let contents = tokio::fs::read_to_string(PROC_MEMINFO).await.ok()?;
    parse_meminfo_value_kib(&contents, key)
}

#[cfg(target_os = "linux")]
fn parse_meminfo_value_kib(contents: &str, key: &str) -> Option<u64> {
    for line in contents.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name != key {
            continue;
        }
        let value = value.split_whitespace().next()?;
        return value.parse().ok();
    }
    None
}

#[cfg(target_os = "linux")]
fn kib_to_bytes(kib: u64) -> i64 {
    let bytes = kib.saturating_mul(1024);
    i64::try_from(bytes).unwrap_or(i64::MAX)
}

#[cfg(target_os = "linux")]
async fn is_containerized() -> bool {
    let Ok(cgroup_contents) = tokio::fs::read_to_string(PROC_SELF_CGROUP).await else {
        return false;
    };
    let Ok(mountinfo_contents) = tokio::fs::read_to_string(PROC_SELF_MOUNTINFO).await else {
        return false;
    };

    let controllers_read_only = controllers_are_read_only(&cgroup_contents, &mountinfo_contents);
    if controllers_read_only {
        return true;
    }

    let memory_limit_present = memory_limit_is_present(&cgroup_contents, &mountinfo_contents).await;
    if memory_limit_present {
        return true;
    }

    cpu_limit_is_present(&cgroup_contents, &mountinfo_contents).await
}

#[cfg(target_os = "linux")]
fn controllers_are_read_only(cgroup_contents: &str, mountinfo_contents: &str) -> bool {
    if cgroup_path(cgroup_contents, None).is_some() {
        let Some(mount_info) = cgroup2_mount(mountinfo_contents) else {
            return false;
        };
        return option_list_contains(mount_info.mount_options, "ro");
    }

    let required_controllers = ["memory", "cpu", "cpuacct", "cpuset"];
    required_controllers.iter().all(|controller| {
        let Some(mount_info) = cgroup1_mount(mountinfo_contents, controller) else {
            return false;
        };
        option_list_contains(mount_info.mount_options, "ro")
    })
}

#[cfg(target_os = "linux")]
async fn memory_limit_is_present(cgroup_contents: &str, mountinfo_contents: &str) -> bool {
    let host_memory = total_memory_size().await;
    let Ok(host_memory) = u64::try_from(host_memory) else {
        return false;
    };
    if host_memory == 0 {
        return false;
    }

    let cgroup2_path = cgroup_path(cgroup_contents, None);
    let cgroup2_mount_info = cgroup2_mount(mountinfo_contents);
    if let (Some(cgroup_path), Some(mount_info)) = (cgroup2_path, cgroup2_mount_info) {
        let memory_max_path = cgroup_file_path(mount_info, cgroup_path, "memory.max");
        let memory_max_limited = cgroup_limit_less_than(&memory_max_path, host_memory).await;
        if memory_max_limited {
            return true;
        }

        let memory_high_path = cgroup_file_path(mount_info, cgroup_path, "memory.high");
        let memory_high_limited = cgroup_limit_less_than(&memory_high_path, host_memory).await;
        if memory_high_limited {
            return true;
        }

        let memory_low_path = cgroup_file_path(mount_info, cgroup_path, "memory.low");
        return cgroup_limit_between(&memory_low_path, 1, host_memory).await;
    }

    let cgroup1_path = cgroup_path(cgroup_contents, Some("memory"));
    let cgroup1_mount_info = cgroup1_mount(mountinfo_contents, "memory");
    if let (Some(cgroup_path), Some(mount_info)) = (cgroup1_path, cgroup1_mount_info) {
        let memory_limit_path = cgroup_file_path(mount_info, cgroup_path, "memory.limit_in_bytes");
        let memory_limited = cgroup_limit_less_than(&memory_limit_path, host_memory).await;
        if memory_limited {
            return true;
        }

        let memory_soft_limit_path =
            cgroup_file_path(mount_info, cgroup_path, "memory.soft_limit_in_bytes");
        return cgroup_limit_between(&memory_soft_limit_path, 1, host_memory).await;
    }

    false
}

#[cfg(target_os = "linux")]
async fn cpu_limit_is_present(cgroup_contents: &str, mountinfo_contents: &str) -> bool {
    let cgroup2_path = cgroup_path(cgroup_contents, None);
    let cgroup2_mount_info = cgroup2_mount(mountinfo_contents);
    if let (Some(cgroup_path), Some(mount_info)) = (cgroup2_path, cgroup2_mount_info) {
        let cpu_max_path = cgroup_file_path(mount_info, cgroup_path, "cpu.max");
        let cpu_limit = cgroup_limit(&cpu_max_path).await;
        return cpu_limit.is_some();
    }

    let cgroup1_path = cgroup_path(cgroup_contents, Some("cpu"));
    let cgroup1_mount_info = cgroup1_mount(mountinfo_contents, "cpu");
    if let (Some(cgroup_path), Some(mount_info)) = (cgroup1_path, cgroup1_mount_info) {
        let cpu_quota_path = cgroup_file_path(mount_info, cgroup_path, "cpu.cfs_quota_us");
        return cgroup_positive_signed_limit_is_set(&cpu_quota_path).await;
    }

    false
}

#[cfg(target_os = "linux")]
async fn cgroup_limit_less_than(path: &Path, upper_bound: u64) -> bool {
    let Some(limit) = cgroup_limit(path).await else {
        return false;
    };
    limit < upper_bound
}

#[cfg(target_os = "linux")]
async fn cgroup_limit_between(path: &Path, lower_bound: u64, upper_bound: u64) -> bool {
    let Some(limit) = cgroup_limit(path).await else {
        return false;
    };
    let limit_range = lower_bound..upper_bound;
    limit_range.contains(&limit)
}

#[cfg(target_os = "linux")]
async fn cgroup_positive_signed_limit_is_set(path: &Path) -> bool {
    let Ok(contents) = tokio::fs::read_to_string(path).await else {
        return false;
    };
    let Some(token) = contents.split_whitespace().next() else {
        return false;
    };
    let parsed_limit = token.parse::<i64>();
    parsed_limit.is_ok_and(|value| value > 0)
}

#[cfg(target_os = "linux")]
async fn cgroup_limit(path: &Path) -> Option<u64> {
    let contents = tokio::fs::read_to_string(path).await.ok()?;
    parse_limit(&contents)
}

#[cfg(target_os = "linux")]
fn parse_limit(contents: &str) -> Option<u64> {
    let token = contents.split_whitespace().next()?;
    if token == "max" {
        None
    } else {
        token.parse().ok()
    }
}

#[cfg(target_os = "linux")]
fn cgroup_path<'a>(contents: &'a str, controller: Option<&str>) -> Option<&'a str> {
    for line in contents.lines() {
        let mut parts = line.splitn(3, ':');
        let Some(_hierarchy_id) = parts.next() else {
            continue;
        };
        let Some(controllers) = parts.next() else {
            continue;
        };
        let Some(path) = parts.next() else {
            continue;
        };

        if controller.is_none() && controllers.is_empty() {
            return Some(path);
        }

        let Some(controller) = controller else {
            continue;
        };
        let controller_found = controllers.split(',').any(|value| value == controller);
        if controller_found {
            return Some(path);
        }
    }
    None
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy)]
struct MountInfo<'a> {
    root: &'a str,
    mount_point: &'a str,
    mount_options: &'a str,
    fs_type: &'a str,
    super_options: &'a str,
}

#[cfg(target_os = "linux")]
fn cgroup2_mount(contents: &str) -> Option<MountInfo<'_>> {
    for line in contents.lines() {
        let Some(mount_info) = parse_mountinfo_line(line) else {
            continue;
        };
        if mount_info.fs_type == "cgroup2" {
            return Some(mount_info);
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn cgroup1_mount<'a>(contents: &'a str, controller: &str) -> Option<MountInfo<'a>> {
    for line in contents.lines() {
        let Some(mount_info) = parse_mountinfo_line(line) else {
            continue;
        };
        let cgroup_filesystem = mount_info.fs_type == "cgroup";
        let controller_mounted = option_list_contains(mount_info.super_options, controller);
        if cgroup_filesystem && controller_mounted {
            return Some(mount_info);
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn parse_mountinfo_line(line: &str) -> Option<MountInfo<'_>> {
    let (mount_fields, filesystem_fields) = line.split_once(" - ")?;
    let mut mount_field_values = mount_fields.split_whitespace();
    mount_field_values.next()?;
    mount_field_values.next()?;
    mount_field_values.next()?;
    let root = mount_field_values.next()?;
    let mount_point = mount_field_values.next()?;
    let mount_options = mount_field_values.next()?;

    let mut filesystem_field_values = filesystem_fields.split_whitespace();
    let fs_type = filesystem_field_values.next()?;
    filesystem_field_values.next()?;
    let super_options = filesystem_field_values.next().unwrap_or("");

    Some(MountInfo {
        root,
        mount_point,
        mount_options,
        fs_type,
        super_options,
    })
}

#[cfg(target_os = "linux")]
fn option_list_contains(list: &str, option: &str) -> bool {
    list.split(',').any(|value| value == option)
}

#[cfg(target_os = "linux")]
fn cgroup_file_path(mount_info: MountInfo<'_>, cgroup_path: &str, file_name: &str) -> PathBuf {
    let mut file_path = PathBuf::from(mount_info.mount_point);
    let root_is_host = mount_info.root == "/";
    let relative_path = if root_is_host {
        cgroup_path
    } else {
        let stripped_path = cgroup_path.strip_prefix(mount_info.root);
        stripped_path.unwrap_or(cgroup_path)
    };
    push_cgroup_relative_path(&mut file_path, relative_path);
    file_path.push(file_name);
    file_path
}

#[cfg(target_os = "linux")]
fn push_cgroup_relative_path(file_path: &mut PathBuf, cgroup_path: &str) {
    let trimmed_path = cgroup_path.trim_matches('/');
    let components = trimmed_path
        .split('/')
        .filter(|component| !component.is_empty());
    for component in components {
        file_path.push(component);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_total_memory_size0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_total_memory_size0(thread, Parameters::default()).await;
        #[cfg(target_os = "linux")]
        assert!(matches!(result?, Some(Value::Long(value)) if value > 0));
        #[cfg(not(target_os = "linux"))]
        assert_unsupported_operation_error(result, GET_TOTAL_MEMORY_SIZE);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_total_swap_size0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_total_swap_size0(thread, Parameters::default()).await;
        #[cfg(target_os = "linux")]
        assert!(matches!(result?, Some(Value::Long(value)) if value >= 0));
        #[cfg(not(target_os = "linux"))]
        assert_unsupported_operation_error(result, GET_TOTAL_SWAP_SIZE);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_containerized0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_containerized0(thread, Parameters::default()).await;
        #[cfg(target_os = "linux")]
        assert!(matches!(result?, Some(value) if value.as_bool().is_ok()));
        #[cfg(not(target_os = "linux"))]
        assert_unsupported_operation_error(result, IS_CONTAINERIZED);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_use_container_support() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_use_container_support(thread, Parameters::default()).await;
        #[cfg(target_os = "linux")]
        assert_eq!(result?, Some(Value::from(true)));
        #[cfg(not(target_os = "linux"))]
        assert_unsupported_operation_error(result, IS_USE_CONTAINER_SUPPORT);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_meminfo_value_kib() {
        let meminfo = "MemTotal:       16384256 kB\nSwapTotal:       2097148 kB\n";
        let total_memory = parse_meminfo_value_kib(meminfo, "MemTotal");
        let total_swap = parse_meminfo_value_kib(meminfo, "SwapTotal");
        let missing_value = parse_meminfo_value_kib(meminfo, "Missing");

        assert_eq!(total_memory, Some(16_384_256));
        assert_eq!(total_swap, Some(2_097_148));
        assert_eq!(missing_value, None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cgroup_path() {
        let cgroup = "0::/user.slice/session.scope\n5:cpu,cpuacct:/docker/id\n";
        let unified_path = cgroup_path(cgroup, None);
        let cpu_path = cgroup_path(cgroup, Some("cpu"));
        let cpu_accounting_path = cgroup_path(cgroup, Some("cpuacct"));
        let memory_path = cgroup_path(cgroup, Some("memory"));

        assert_eq!(unified_path, Some("/user.slice/session.scope"));
        assert_eq!(cpu_path, Some("/docker/id"));
        assert_eq!(cpu_accounting_path, Some("/docker/id"));
        assert_eq!(memory_path, None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_mountinfo_line() {
        let line =
            "1043 1034 0:27 / /sys/fs/cgroup ro,nosuid,nodev,noexec,relatime - cgroup2 cgroup2 rw";
        let mount_info = parse_mountinfo_line(line);
        assert!(matches!(
            mount_info,
            Some(MountInfo {
                root: "/",
                mount_point: "/sys/fs/cgroup",
                mount_options: "ro,nosuid,nodev,noexec,relatime",
                fs_type: "cgroup2",
                super_options: "rw",
            })
        ));
    }

    #[cfg(not(target_os = "linux"))]
    fn assert_unsupported_operation_error(result: Result<Option<Value>>, expected_message: &str) {
        match result {
            Ok(value) => panic!("expected UnsupportedOperationException, got {value:?}"),
            Err(error) => assert_eq!(expected_message, error.to_string()),
        }
    }
}
