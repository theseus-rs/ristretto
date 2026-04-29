#![cfg(not(target_family = "wasm"))]

use ristretto_classloader::DEFAULT_JAVA_VERSION;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Creates a VM, runs `HelloWorld`, drops the VM, and returns peak resident memory (RSS).
async fn run_hello_world_vm() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(&[classes_jar_path]);
    let stdout_buffer = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));

    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .java_version(DEFAULT_JAVA_VERSION)
        .stdout(stdout_buffer.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = vec!["world!"];

    let result = vm.invoke_main(&parameters).await?;
    assert!(result.is_none());

    let output = stdout_buffer.lock().await;
    let output_bytes = output.get_ref();
    let output_str = String::from_utf8(output_bytes.clone()).expect("Invalid UTF-8 output");
    assert_eq!(output_str.trim(), "Hello world!");

    drop(vm);
    Ok(())
}

/// Verifies that creating, running, and dropping VMs 10 times does not leak memory.
/// Uses process RSS to check that memory growth stays bounded.
#[tokio::test]
async fn test_vm_create_run_drop_no_memory_leak() -> Result<()> {
    // Warm up: create and drop VMs to establish baseline allocations
    // (JIT caches, class metadata caches, allocator pre-allocation, etc.)
    for _ in 0..3 {
        run_hello_world_vm().await?;
    }

    // Measure baseline memory after warm-up
    let baseline_rss = get_rss_bytes();

    // Create, run, and drop 10 VMs
    for _ in 0..10 {
        run_hello_world_vm().await?;
    }

    let final_rss = get_rss_bytes();
    let total_growth = final_rss.saturating_sub(baseline_rss);

    assert!(
        total_growth < 1024 * 1024 * 1024,
        "total RSS growth of {} MB after 10 iterations exceeds 1GB; possible memory leak",
        total_growth / (1024 * 1024)
    );

    Ok(())
}

/// Verifies that creating, running GC, and dropping a VM does not crash (SIGBUS regression).
#[tokio::test]
async fn test_vm_gc_no_crash() -> Result<()> {
    let vm = VM::default().await?;
    vm.gc();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let obj = vm.object("java.lang.Integer", "I", &[42]).await?;
    vm.gc();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    drop(obj);
    drop(vm);
    Ok(())
}

/// Verifies that multiple VMs can be created and dropped sequentially without deadlock or crash.
#[tokio::test]
async fn test_vm_sequential_create_drop() -> Result<()> {
    for _ in 0..10 {
        let vm = VM::default().await?;
        vm.gc();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        drop(vm);
    }
    Ok(())
}

/// Returns the current process resident set size in bytes (macOS/Linux/Windows).
fn get_rss_bytes() -> usize {
    #[cfg(target_os = "macos")]
    {
        // Use sysctl-style ps to get RSS for the current process
        let output = std::process::Command::new("ps")
            .args(["-o", "rss=", "-p", &std::process::id().to_string()])
            .output()
            .ok();
        output
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|s| s.trim().parse::<usize>().ok())
            .map_or(0, |kb| kb * 1024) // ps reports in KB
    }
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/self/statm")
            .ok()
            .and_then(|s| s.split_whitespace().nth(1)?.parse::<usize>().ok())
            .map_or(0, |pages| pages * 4096)
    }
    #[cfg(target_os = "windows")]
    {
        use std::mem::{size_of, zeroed};

        #[repr(C)]
        #[expect(non_snake_case)]
        struct PROCESS_MEMORY_COUNTERS {
            cb: u32,
            PageFaultCount: u32,
            PeakWorkingSetSize: usize,
            WorkingSetSize: usize,
            QuotaPeakPagedPoolUsage: usize,
            QuotaPagedPoolUsage: usize,
            QuotaPeakNonPagedPoolUsage: usize,
            QuotaNonPagedPoolUsage: usize,
            PagefileUsage: usize,
            PeakPagefileUsage: usize,
        }

        #[expect(unsafe_code)]
        unsafe extern "system" {
            fn GetCurrentProcess() -> *mut std::ffi::c_void;
            fn K32GetProcessMemoryInfo(
                process: *mut std::ffi::c_void,
                pmc: *mut PROCESS_MEMORY_COUNTERS,
                cb: u32,
            ) -> i32;
        }

        #[expect(unsafe_code)]
        unsafe {
            let mut pmc: PROCESS_MEMORY_COUNTERS = zeroed();
            pmc.cb = u32::try_from(size_of::<PROCESS_MEMORY_COUNTERS>()).unwrap_or(0);
            if K32GetProcessMemoryInfo(GetCurrentProcess(), &raw mut pmc, pmc.cb) != 0 {
                pmc.WorkingSetSize
            } else {
                0
            }
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        0
    }
}
