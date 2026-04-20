use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(target_family = "wasm")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[cfg(target_os = "windows")]
mod win32 {
    use std::ffi::c_void;

    pub type Handle = *mut c_void;
    pub const INFINITE: u32 = 0xFFFF_FFFF;
    pub const STILL_ACTIVE: u32 = 259;

    #[expect(unsafe_code)]
    unsafe extern "system" {
        pub safe fn WaitForSingleObject(hHandle: Handle, dwMilliseconds: u32) -> u32;
        pub safe fn GetExitCodeProcess(hProcess: Handle, lpExitCode: *mut u32) -> i32;
        pub safe fn TerminateProcess(hProcess: Handle, uExitCode: u32) -> i32;
        pub safe fn CloseHandle(hObject: Handle) -> i32;
        pub safe fn GetProcessId(hProcess: Handle) -> u32;
    }
}

/// Spawn a child process using the JDK `forkAndExec` parameter convention.
///
/// In Java 9+, `ProcessImpl` replaced `UNIXProcess` on Unix platforms. Both share the same
/// parameter layout, so `UNIXProcess.forkAndExec` (≤ Java 8) delegates here as well.
#[intrinsic_method(
    "java/lang/ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fork_and_exec<T: Thread + 'static>(
    thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let redirect_error_stream = parameters.pop_bool()?;
        let fds_ref = parameters.pop_reference()?;
        let dir_ref = parameters.pop_reference()?;
        let _envc = parameters.pop_int()?;
        let env_block_ref = parameters.pop_reference()?;
        let _argc = parameters.pop_int()?;
        let arg_block_ref = parameters.pop_reference()?;
        let file_ref = parameters.pop_reference()?;
        let _helper_method_ref = parameters.pop_reference()?;
        let _mode = parameters.pop_int()?;

        // Extract the program/file name from the byte array
        let program = extract_null_terminated_string(file_ref.as_ref());

        // Extract arguments from the arg block (null-separated byte sequences)
        let args = extract_null_separated_strings(arg_block_ref.as_ref());

        // Build the command using tokio's async Command
        let mut command = tokio::process::Command::new(&program);

        // Skip the first argument if it matches the program name (it's the argv[0])
        let skip_first = args.first().is_some_and(|arg| *arg == program);
        if skip_first {
            command.args(&args[1..]);
        } else {
            command.args(&args);
        }

        // Set environment variables from envBlock (null-separated KEY=VALUE pairs)
        if let Some(env_ref) = &env_block_ref {
            let env_guard = env_ref.read();
            if let Reference::ByteArray(env_bytes) = &*env_guard
                && !env_bytes.is_empty()
            {
                command.env_clear();
                let env_strings = split_null_terminated(env_bytes);
                for env_str in env_strings {
                    if let Some((key, value)) = env_str.split_once('=') {
                        command.env(key, value);
                    }
                }
            }
        }

        // Set working directory
        if let Some(dir_r) = &dir_ref {
            let dir_guard = dir_r.read();
            if let Reference::ByteArray(dir_bytes) = &*dir_guard
                && !dir_bytes.is_empty()
            {
                let dir_str = bytes_to_string(dir_bytes);
                command.current_dir(&dir_str);
            }
        }

        // Configure stdio; always pipe all three streams
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
        let _ = redirect_error_stream;

        // Spawn the process
        let mut child = command
            .spawn()
            .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;

        let child_pid = child.id().unwrap_or(0);

        // Take ownership of child pipes, store fd numbers in the Java fds array,
        // and register them with the VM's file handle system.
        register_child_pipes(&thread, fds_ref.as_ref(), &mut child).await?;

        // We intentionally leak the child handle here (sans pipes). The process will be
        // managed by destroyProcess/waitForProcessExit using the pid.
        std::mem::forget(child);

        let pid = i32::try_from(child_pid)?;
        Ok(Some(Value::Int(pid)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = (thread, parameters);
        Err(JavaError::UnsupportedOperationException(
            "java.lang.ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I".to_string(),
        )
        .into())
    }
}

/// Take ownership of child process pipes, store their fd numbers in the Java fds array,
/// and register them with the VM's file handle system so Java I/O can access them.
#[cfg(all(unix, not(target_family = "wasm")))]
async fn register_child_pipes<T: Thread + 'static>(
    thread: &Arc<T>,
    fds_ref: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
    child: &mut tokio::process::Child,
) -> Result<()> {
    use std::os::fd::{AsFd, AsRawFd};

    use ristretto_types::VM;
    use ristretto_types::handles::FileHandle;

    let vm = thread.vm()?;
    let file_handles = vm.file_handles();

    // Take pipes and clone their fds using dup (safe)
    let mut pipe_data = Vec::new();

    if let Some(stdin) = child.stdin.take() {
        let owned_fd = stdin.as_fd().try_clone_to_owned()?;
        drop(stdin);
        pipe_data.push((0usize, owned_fd));
    }
    if let Some(stdout) = child.stdout.take() {
        let owned_fd = stdout.as_fd().try_clone_to_owned()?;
        drop(stdout);
        pipe_data.push((1usize, owned_fd));
    }
    if let Some(stderr) = child.stderr.take() {
        let owned_fd = stderr.as_fd().try_clone_to_owned()?;
        drop(stderr);
        pipe_data.push((2usize, owned_fd));
    }

    // Store fd numbers in the Java fds array (under parking_lot lock, no await)
    if let Some(fds_r) = fds_ref {
        let mut fds_guard = fds_r.write();
        if let Reference::IntArray(fds) = &mut *fds_guard {
            for (idx, owned_fd) in &pipe_data {
                if let Some(slot) = fds.get_mut(*idx) {
                    *slot = owned_fd.as_raw_fd();
                }
            }
        }
    }

    // Register with VM file handles (async, lock released)
    for (_idx, owned_fd) in pipe_data {
        let raw_fd = owned_fd.as_raw_fd();
        let std_file = std::fs::File::from(owned_fd);
        let tokio_file = tokio::fs::File::from_std(std_file);
        let file_handle = FileHandle::from((tokio_file, false));
        file_handles.insert(i64::from(raw_fd), file_handle).await?;
    }

    Ok(())
}

/// No-op on non-Unix platforms; stores -1 for all fds.
#[cfg(not(any(unix, target_family = "wasm")))]
#[expect(clippy::unused_async)]
async fn register_child_pipes<T: Thread + 'static>(
    _thread: &Arc<T>,
    fds_ref: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
    _child: &mut tokio::process::Child,
) -> Result<()> {
    if let Some(fds_r) = fds_ref {
        let mut fds_guard = fds_r.write();
        if let Reference::IntArray(fds) = &mut *fds_guard {
            if let Some(slot) = fds.first_mut() {
                *slot = -1;
            }
            if let Some(slot) = fds.get_mut(1) {
                *slot = -1;
            }
            if let Some(slot) = fds.get_mut(2) {
                *slot = -1;
            }
        }
    }
    Ok(())
}

/// Extract a null-terminated string from an optional byte array reference.
#[cfg(not(target_family = "wasm"))]
fn extract_null_terminated_string(
    reference: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
) -> String {
    let Some(r) = reference else {
        return String::new();
    };
    let guard = r.read();
    let Reference::ByteArray(bytes) = &*guard else {
        return String::new();
    };
    bytes_to_string_null_terminated(bytes)
}

/// Extract null-separated strings from an optional byte array reference.
#[cfg(not(target_family = "wasm"))]
fn extract_null_separated_strings(
    reference: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
) -> Vec<String> {
    let Some(r) = reference else {
        return Vec::new();
    };
    let guard = r.read();
    let Reference::ByteArray(bytes) = &*guard else {
        return Vec::new();
    };
    split_null_terminated(bytes)
}

/// Convert a byte array (i8 values) to a string, stopping at the first null byte.
#[cfg(not(target_family = "wasm"))]
#[expect(clippy::cast_sign_loss)]
fn bytes_to_string_null_terminated(bytes: &[i8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let u8_bytes: Vec<u8> = bytes[..end].iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&u8_bytes).into_owned()
}

/// Convert a byte array (i8 values) to a string.
#[cfg(not(target_family = "wasm"))]
#[expect(clippy::cast_sign_loss)]
fn bytes_to_string(bytes: &[i8]) -> String {
    let u8_bytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&u8_bytes).into_owned()
}

/// Split a byte array (i8 values) at null terminators, returning a vector of strings.
#[cfg(not(target_family = "wasm"))]
#[expect(clippy::cast_sign_loss)]
fn split_null_terminated(bytes: &[i8]) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == 0 {
            if i > start {
                let u8_bytes: Vec<u8> = bytes[start..i].iter().map(|&b| b as u8).collect();
                result.push(String::from_utf8_lossy(&u8_bytes).into_owned());
            }
            start = i + 1;
        }
    }
    if start < bytes.len() {
        let u8_bytes: Vec<u8> = bytes[start..].iter().map(|&b| b as u8).collect();
        result.push(String::from_utf8_lossy(&u8_bytes).into_owned());
    }
    result
}

#[intrinsic_method("java/lang/ProcessImpl.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

/// Returns the Windows `STILL_ACTIVE` constant (259).
#[cfg(target_os = "windows")]
#[intrinsic_method("java/lang/ProcessImpl.getStillActive()I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_still_active<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(
        i32::try_from(win32::STILL_ACTIVE).unwrap_or(259),
    )))
}

/// Creates a new process on Windows and returns its handle.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.create(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[JZ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use std::os::windows::io::IntoRawHandle;
    use std::os::windows::process::CommandExt;

    let _redirect_error_stream = parameters.pop_bool()?;
    let std_handles_ref = parameters.pop_reference()?;
    let dir_value = parameters.pop()?;
    let envblock_value = parameters.pop()?;
    let cmdstr = parameters.pop()?.as_string()?;

    // Parse the command line to extract program and arguments
    let (program, raw_args) = parse_windows_command_line(&cmdstr);

    let mut command = std::process::Command::new(&program);
    if !raw_args.is_empty() {
        command.raw_arg(&raw_args);
    }

    // Set environment if provided
    if let Ok(envblock) = envblock_value.as_string()
        && !envblock.is_empty()
    {
        command.env_clear();
        for pair in envblock.split('\0') {
            if pair.is_empty() {
                continue;
            }
            if let Some((key, value)) = pair.split_once('=') {
                command.env(key, value);
            }
        }
    }

    // Set working directory if provided
    if let Ok(dir) = dir_value.as_string()
        && !dir.is_empty()
    {
        command.current_dir(&dir);
    }

    // Use null stdio - pipe handles are set to -1 in stdHandles
    command.stdin(std::process::Stdio::null());
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());

    let child = command
        .spawn()
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;

    // Extract the raw handle, transferring ownership away from Child.
    // The handle must be closed later via closeHandle.
    let raw_handle = child.into_raw_handle();
    let handle = raw_handle as i64;

    // Set stdHandles to -1 (null streams)
    if let Some(ref handles_gc) = std_handles_ref {
        let mut guard = handles_gc.write();
        if let Reference::LongArray(handles) = &mut *guard {
            for slot in handles.iter_mut().take(3) {
                *slot = -1;
            }
        }
    }

    Ok(Some(Value::Long(handle)))
}

/// Gets the process ID from a Windows process handle.
#[cfg(target_os = "windows")]
#[intrinsic_method("java/lang/ProcessImpl.getProcessId0(J)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_process_id_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let pid = win32::GetProcessId(handle as win32::Handle);
    Ok(Some(Value::Int(i32::try_from(pid)?)))
}

/// Waits for a process to complete (blocking).
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.waitForInterruptibly(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn wait_for_interruptibly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    tokio::task::spawn_blocking(move || {
        win32::WaitForSingleObject(handle as win32::Handle, win32::INFINITE);
    })
    .await
    .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;
    Ok(None)
}

/// Waits for a process to complete with a timeout (blocking).
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.waitForTimeoutInterruptibly(JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn wait_for_timeout_interruptibly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout_millis = parameters.pop_long()?;
    let handle = parameters.pop_long()?;
    let timeout = u32::try_from(timeout_millis).unwrap_or(win32::INFINITE);
    tokio::task::spawn_blocking(move || {
        win32::WaitForSingleObject(handle as win32::Handle, timeout);
    })
    .await
    .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;
    Ok(None)
}

/// Gets the exit code of a process.
#[cfg(target_os = "windows")]
#[expect(clippy::cast_possible_wrap)]
#[intrinsic_method(
    "java/lang/ProcessImpl.getExitCodeProcess(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_exit_code_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let mut exit_code: u32 = 0;
    win32::GetExitCodeProcess(handle as win32::Handle, &raw mut exit_code);
    Ok(Some(Value::Int(exit_code as i32)))
}

/// Checks if a process is still alive.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.isProcessAlive(J)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_process_alive<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let mut exit_code: u32 = 0;
    win32::GetExitCodeProcess(handle as win32::Handle, &raw mut exit_code);
    Ok(Some(Value::from(exit_code == win32::STILL_ACTIVE)))
}

/// Terminates a process.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.terminateProcess(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn terminate_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    win32::TerminateProcess(handle as win32::Handle, 1);
    Ok(None)
}

/// Closes a Windows handle.
#[cfg(target_os = "windows")]
#[intrinsic_method("java/lang/ProcessImpl.closeHandle(J)Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn close_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let result = win32::CloseHandle(handle as win32::Handle);
    Ok(Some(Value::from(result != 0)))
}

/// Opens a file for atomic append and returns a handle.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessImpl.openForAtomicAppend(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn open_for_atomic_append<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use std::fs::OpenOptions;
    use std::os::windows::io::IntoRawHandle;

    let path = parameters.pop()?.as_string()?;
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;
    let handle = file.into_raw_handle() as i64;
    Ok(Some(Value::Long(handle)))
}

/// Parse a Windows command line string into the program and remaining arguments.
#[cfg(target_os = "windows")]
fn parse_windows_command_line(cmdstr: &str) -> (String, String) {
    let trimmed = cmdstr.trim();
    if let Some(rest) = trimmed.strip_prefix('"') {
        // Quoted program name
        if let Some(end) = rest.find('"') {
            let program = &rest[..end];
            let args = rest[end + 1..].trim_start();
            (program.to_string(), args.to_string())
        } else {
            (trimmed.to_string(), String::new())
        }
    } else if let Some(space) = trimmed.find(' ') {
        let program = &trimmed[..space];
        let args = trimmed[space + 1..].trim_start();
        (program.to_string(), args.to_string())
    } else {
        (trimmed.to_string(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_gc::GarbageCollector;

    /// Returns the platform-appropriate command for echoing text.
    fn echo_command() -> (&'static str, &'static str) {
        if cfg!(target_os = "windows") {
            ("cmd.exe", "cmd.exe")
        } else {
            ("/bin/echo", "/bin/echo")
        }
    }

    /// Returns the echo arg block bytes for a given message, suitable for `forkAndExec`.
    #[expect(clippy::cast_possible_wrap)]
    fn echo_arg_block(msg: &str) -> Vec<i8> {
        if cfg!(target_os = "windows") {
            let s = format!("cmd.exe\0/C\0echo\0{msg}\0");
            s.into_bytes().iter().map(|&b| b as i8).collect()
        } else {
            let s = format!("/bin/echo\0{msg}\0");
            s.into_bytes().iter().map(|&b| b as i8).collect()
        }
    }

    /// Returns the argc for echo commands (program name + args).
    fn echo_argc(extra_args: usize) -> i32 {
        if cfg!(target_os = "windows") {
            i32::try_from(3 + extra_args).unwrap_or(3)
        } else {
            i32::try_from(1 + extra_args).unwrap_or(1)
        }
    }

    /// Returns the platform-appropriate file bytes (program path with null terminator).
    #[expect(clippy::cast_possible_wrap)]
    fn echo_file_bytes() -> Vec<i8> {
        let (path, _) = echo_command();
        let mut s = path.to_string();
        s.push('\0');
        s.into_bytes().iter().map(|&b| b as i8).collect()
    }

    /// Build standard `fork_and_exec` parameters from file bytes, arg bytes, argc,
    /// and optional env/dir/fds references.
    struct ForkParamsBuilder {
        file_bytes: Vec<i8>,
        arg_bytes: Vec<i8>,
        argc: i32,
        env_ref: Option<Value>,
        envc: i32,
        dir_ref: Option<Value>,
        fds_ref: Option<Value>,
        redirect_error_stream: bool,
    }

    impl ForkParamsBuilder {
        fn new(file_bytes: Vec<i8>, arg_bytes: Vec<i8>, argc: i32) -> Self {
            Self {
                file_bytes,
                arg_bytes,
                argc,
                env_ref: None,
                envc: 0,
                dir_ref: None,
                fds_ref: None,
                redirect_error_stream: false,
            }
        }

        fn build(self, gc: &GarbageCollector) -> Parameters {
            let file_ref = Value::new_object(gc, Reference::from(self.file_bytes));
            let arg_ref = Value::new_object(gc, Reference::from(self.arg_bytes));
            let env = self.env_ref.unwrap_or(Value::Object(None));
            let dir = self.dir_ref.unwrap_or(Value::Object(None));
            let fds = self
                .fds_ref
                .unwrap_or_else(|| Value::new_object(gc, Reference::from(vec![0i32; 3])));
            let helper_ref = Value::Object(None);

            let mut parameters = Parameters::default();
            parameters.push_int(0); // mode
            parameters.push(helper_ref); // helperMethod
            parameters.push(file_ref); // file
            parameters.push(arg_ref); // argBlock
            parameters.push_int(self.argc); // argc
            parameters.push(env); // envBlock
            parameters.push_int(self.envc); // envc
            parameters.push(dir); // dir
            parameters.push(fds); // fds
            parameters.push_bool(self.redirect_error_stream);
            parameters
        }
    }

    #[tokio::test]
    async fn test_fork_and_exec_empty_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fork_and_exec(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fork_and_exec() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let gc = GarbageCollector::new();

        let parameters =
            ForkParamsBuilder::new(echo_file_bytes(), echo_arg_block("hello"), echo_argc(1))
                .build(&gc);

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec_with_env() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let gc = GarbageCollector::new();

        #[expect(clippy::cast_possible_wrap)]
        let env_bytes: Vec<i8> = b"FOO=bar\0BAZ=qux\0".iter().map(|&b| b as i8).collect();
        let env_ref = Value::new_object(&gc, Reference::from(env_bytes));

        let mut builder =
            ForkParamsBuilder::new(echo_file_bytes(), echo_arg_block("test"), echo_argc(1));
        builder.env_ref = Some(env_ref);
        builder.envc = 2;
        let parameters = builder.build(&gc);

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec_with_dir() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let gc = GarbageCollector::new();

        let temp_dir = std::env::temp_dir();
        let dir_str = temp_dir.to_string_lossy();
        #[expect(clippy::cast_possible_wrap)]
        let dir_bytes: Vec<i8> = dir_str.as_bytes().iter().map(|&b| b as i8).collect();
        let dir_ref = Value::new_object(&gc, Reference::from(dir_bytes));

        let mut builder =
            ForkParamsBuilder::new(echo_file_bytes(), echo_arg_block("dir_test"), echo_argc(1));
        builder.dir_ref = Some(dir_ref);
        let parameters = builder.build(&gc);

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec_redirect_error_stream() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let gc = GarbageCollector::new();

        let mut builder =
            ForkParamsBuilder::new(echo_file_bytes(), echo_arg_block("redirect"), echo_argc(1));
        builder.redirect_error_stream = true;
        let parameters = builder.build(&gc);

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec_null_fds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let gc = GarbageCollector::new();

        let mut builder =
            ForkParamsBuilder::new(echo_file_bytes(), echo_arg_block("null_fds"), echo_argc(1));
        builder.fds_ref = Some(Value::Object(None));
        let parameters = builder.build(&gc);

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[test]
    fn test_bytes_to_string_null_terminated() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"hello\0world".iter().map(|&b| b as i8).collect();
        assert_eq!(bytes_to_string_null_terminated(&bytes), "hello");
    }

    #[test]
    fn test_bytes_to_string_null_terminated_no_null() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"hello".iter().map(|&b| b as i8).collect();
        assert_eq!(bytes_to_string_null_terminated(&bytes), "hello");
    }

    #[test]
    fn test_bytes_to_string_null_terminated_empty() {
        let bytes: Vec<i8> = vec![];
        assert_eq!(bytes_to_string_null_terminated(&bytes), "");
    }

    #[test]
    fn test_bytes_to_string() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"hello".iter().map(|&b| b as i8).collect();
        assert_eq!(bytes_to_string(&bytes), "hello");
    }

    #[test]
    fn test_bytes_to_string_empty() {
        let bytes: Vec<i8> = vec![];
        assert_eq!(bytes_to_string(&bytes), "");
    }

    #[test]
    fn test_split_null_terminated() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"hello\0world\0".iter().map(|&b| b as i8).collect();
        assert_eq!(split_null_terminated(&bytes), vec!["hello", "world"]);
    }

    #[test]
    fn test_split_null_terminated_no_trailing_null() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"hello\0world".iter().map(|&b| b as i8).collect();
        assert_eq!(split_null_terminated(&bytes), vec!["hello", "world"]);
    }

    #[test]
    fn test_split_null_terminated_empty() {
        let bytes: Vec<i8> = vec![];
        let result: Vec<String> = split_null_terminated(&bytes);
        assert!(result.is_empty());
    }

    #[test]
    fn test_split_null_terminated_single() {
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"single\0".iter().map(|&b| b as i8).collect();
        assert_eq!(split_null_terminated(&bytes), vec!["single"]);
    }

    #[test]
    fn test_extract_null_terminated_string_none() {
        let result = extract_null_terminated_string(None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_null_terminated_string_some() {
        let gc = GarbageCollector::new();
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"test\0".iter().map(|&b| b as i8).collect();
        let gc_ref = ristretto_gc::Gc::new(&gc, parking_lot::RwLock::new(Reference::from(bytes)));
        let result = extract_null_terminated_string(Some(&gc_ref));
        assert_eq!(result, "test");
    }

    #[test]
    fn test_extract_null_separated_strings_none() {
        let result = extract_null_separated_strings(None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_null_separated_strings_some() {
        let gc = GarbageCollector::new();
        #[expect(clippy::cast_possible_wrap)]
        let bytes: Vec<i8> = b"one\0two\0".iter().map(|&b| b as i8).collect();
        let gc_ref = ristretto_gc::Gc::new(&gc, parking_lot::RwLock::new(Reference::from(bytes)));
        let result = extract_null_separated_strings(Some(&gc_ref));
        assert_eq!(result, vec!["one", "two"]);
    }

    #[test]
    fn test_extract_null_terminated_string_non_byte_array() {
        let gc = GarbageCollector::new();
        let gc_ref = ristretto_gc::Gc::new(
            &gc,
            parking_lot::RwLock::new(Reference::from(vec![1i32, 2, 3])),
        );
        let result = extract_null_terminated_string(Some(&gc_ref));
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_null_separated_strings_non_byte_array() {
        let gc = GarbageCollector::new();
        let gc_ref = ristretto_gc::Gc::new(
            &gc,
            parking_lot::RwLock::new(Reference::from(vec![1i32, 2, 3])),
        );
        let result = extract_null_separated_strings(Some(&gc_ref));
        assert!(result.is_empty());
    }
}
