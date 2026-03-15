use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

#[intrinsic_method("java/lang/UNIXProcess.destroyProcess(IZ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn destroy_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let force = parameters.pop_bool()?;
    let pid = parameters.pop_int()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    if let Some(process) = system.process(pid) {
        let signal = if force { Signal::Kill } else { Signal::Term };
        process.kill_with(signal);
    }

    Ok(None)
}

#[intrinsic_method(
    "java/lang/UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn fork_and_exec<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
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
    let child = command
        .spawn()
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;

    let child_pid = child.id().unwrap_or(0);

    // Store the file descriptors in the fds array
    // fds[0] = stdin fd, fds[1] = stdout fd, fds[2] = stderr fd
    store_child_fds(fds_ref.as_ref(), &child);

    // We intentionally leak the child handle here. The process will be managed
    // by destroyProcess/waitForProcessExit using the pid.
    std::mem::forget(child);

    let pid = i32::try_from(child_pid)?;
    Ok(Some(Value::Int(pid)))
}

/// Store the child process file descriptors into the fds array reference.
#[cfg(unix)]
fn store_child_fds(
    fds_ref: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
    child: &tokio::process::Child,
) {
    use std::os::unix::io::AsRawFd;
    if let Some(fds_r) = fds_ref {
        let mut fds_guard = fds_r.write();
        if let Reference::IntArray(fds) = &mut *fds_guard {
            if let Some(stdin) = &child.stdin
                && let Some(slot) = fds.first_mut()
            {
                *slot = stdin.as_raw_fd();
            }
            if let Some(stdout) = &child.stdout
                && let Some(slot) = fds.get_mut(1)
            {
                *slot = stdout.as_raw_fd();
            }
            if let Some(stderr) = &child.stderr
                && let Some(slot) = fds.get_mut(2)
            {
                *slot = stderr.as_raw_fd();
            }
        }
    }
}

/// Store the child process file descriptors into the fds array reference.
#[cfg(not(unix))]
fn store_child_fds(
    fds_ref: Option<&ristretto_gc::Gc<parking_lot::RwLock<Reference>>>,
    _child: &tokio::process::Child,
) {
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
}

/// Extract a null-terminated string from an optional byte array reference.
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
#[expect(clippy::cast_sign_loss)]
fn bytes_to_string_null_terminated(bytes: &[i8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let u8_bytes: Vec<u8> = bytes[..end].iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&u8_bytes).into_owned()
}

/// Convert a byte array (i8 values) to a string.
#[expect(clippy::cast_sign_loss)]
fn bytes_to_string(bytes: &[i8]) -> String {
    let u8_bytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&u8_bytes).into_owned()
}

/// Split a byte array (i8 values) at null terminators, returning a vector of strings.
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

#[intrinsic_method("java/lang/UNIXProcess.init()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/UNIXProcess.waitForProcessExit(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn wait_for_process_exit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pid = parameters.pop_int()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let Some(process) = system.process(pid) else {
        return Ok(Some(Value::Int(-1)));
    };

    process.wait();

    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_gc::GarbageCollector;

    /// Returns the platform-appropriate command for echoing text.
    /// On Unix: (`/bin/echo`, `&["echo", "..."]`)
    /// On Windows: (`cmd.exe`, `&["cmd.exe", "/C", "echo", "..."]`)
    fn echo_command() -> (&'static str, &'static str) {
        if cfg!(target_os = "windows") {
            ("cmd.exe", "cmd.exe")
        } else {
            ("/bin/echo", "/bin/echo")
        }
    }

    /// Returns the echo arg block bytes for a given message, suitable for `forkAndExec`.
    /// On Unix: "echo\0<msg>\0"
    /// On Windows: "cmd.exe\0/C\0echo\0<msg>\0"
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
            // cmd.exe /C echo <msg>
            i32::try_from(3 + extra_args).unwrap_or(3)
        } else {
            // echo <msg>
            i32::try_from(1 + extra_args).unwrap_or(1)
        }
    }

    /// Returns the platform-appropriate command that exits immediately with success.
    fn noop_command() -> &'static str {
        if cfg!(target_os = "windows") {
            "cmd.exe"
        } else {
            "/usr/bin/true"
        }
    }

    /// Returns the noop arg block bytes for `forkAndExec`.
    #[expect(clippy::cast_possible_wrap)]
    fn noop_arg_block() -> Vec<i8> {
        if cfg!(target_os = "windows") {
            let s = "cmd.exe\0/C\0exit\x000\0";
            s.as_bytes().iter().map(|&b| b as i8).collect()
        } else {
            let s = "/usr/bin/true\0";
            s.as_bytes().iter().map(|&b| b as i8).collect()
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

    /// Returns the platform-appropriate file bytes for the noop command.
    #[expect(clippy::cast_possible_wrap)]
    fn noop_file_bytes() -> Vec<i8> {
        let path = noop_command();
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
    async fn test_destroy_process_nonexistent() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        parameters.push_bool(false);
        let result = destroy_process(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_destroy_process_force() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        parameters.push_bool(true);
        let result = destroy_process(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_for_process_exit_nonexistent() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        let result = wait_for_process_exit(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_for_process_exit_completed_process() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        // Spawn a short-lived process and wait for it
        let child = tokio::process::Command::new(noop_command())
            .args(if cfg!(target_os = "windows") {
                vec!["/C", "exit", "0"]
            } else {
                vec![]
            })
            .spawn()
            .expect("failed to spawn noop command");
        let child_pid = child.id().expect("expected child pid");
        let child_pid = i32::try_from(child_pid)?;

        let mut parameters = Parameters::default();
        parameters.push_int(child_pid);
        let result = wait_for_process_exit(thread, parameters).await?;
        // The process should have completed (or be completing)
        assert!(result == Some(Value::Int(0)) || result == Some(Value::Int(-1)));
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
