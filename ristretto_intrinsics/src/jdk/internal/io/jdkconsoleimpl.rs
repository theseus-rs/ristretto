use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::JavaError::IoException;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Toggles terminal echo and returns the previous echo state.
///
/// On Unix this uses `tcgetattr`/`tcsetattr` against `STDIN_FILENO` to flip the `ECHO` bit in the
/// terminal's local mode flags. On Windows it uses `GetConsoleMode`/`SetConsoleMode` against the
/// standard input handle to flip the `ENABLE_ECHO_INPUT` bit. On `wasm` targets there is no
/// controllable terminal, so the requested state is returned as the "previous" value (a no-op).
#[intrinsic_method("jdk/internal/io/JdkConsoleImpl.echo(Z)Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn echo<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let on = parameters.pop_bool()?;
    let previous = set_echo(on)?;
    Ok(Some(Value::from(previous)))
}

/// Unix implementation of `set_echo`.
#[cfg(all(target_family = "unix", not(target_family = "wasm")))]
#[expect(unsafe_code)]
fn set_echo(on: bool) -> Result<bool> {
    use std::io::Error;
    use std::mem::MaybeUninit;

    let mut tio = MaybeUninit::<libc::termios>::uninit();
    let fd = libc::STDIN_FILENO;
    // SAFETY: `tio.as_mut_ptr()` is a valid pointer to writable `termios` storage.
    let rc = unsafe { libc::tcgetattr(fd, tio.as_mut_ptr()) };
    if rc != 0 {
        return Err(IoException(format!("tcgetattr failed: {}", Error::last_os_error())).into());
    }
    // SAFETY: `tcgetattr` returned success above, so `tio` is fully initialized.
    let mut tio = unsafe { tio.assume_init() };

    let previous = (tio.c_lflag & libc::ECHO) != 0;
    if on {
        tio.c_lflag |= libc::ECHO;
    } else {
        tio.c_lflag &= !libc::ECHO;
    }

    // SAFETY: `tio` is a valid, fully initialized `termios` struct passed by reference.
    let rc = unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw const tio) };
    if rc != 0 {
        return Err(IoException(format!("tcsetattr failed: {}", Error::last_os_error())).into());
    }
    Ok(previous)
}

/// Windows implementation of `set_echo`.
#[cfg(target_os = "windows")]
#[expect(unsafe_code)]
fn set_echo(on: bool) -> Result<bool> {
    use std::io::Error;
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows_sys::Win32::System::Console::{
        ENABLE_ECHO_INPUT, GetConsoleMode, GetStdHandle, STD_INPUT_HANDLE, SetConsoleMode,
    };

    // SAFETY: `GetStdHandle` is safe to call with a valid standard handle identifier.
    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if handle.is_null() || handle == INVALID_HANDLE_VALUE {
        return Err(IoException(format!(
            "GetStdHandle(STD_INPUT_HANDLE) failed: {}",
            Error::last_os_error()
        ))
        .into());
    }

    let mut mode: u32 = 0;
    // SAFETY: `handle` is a valid console handle and `mode` points to a writable `u32`.
    let rc = unsafe { GetConsoleMode(handle, &raw mut mode) };
    if rc == 0 {
        return Err(
            IoException(format!("GetConsoleMode failed: {}", Error::last_os_error())).into(),
        );
    }

    let previous = (mode & ENABLE_ECHO_INPUT) != 0;
    if on {
        mode |= ENABLE_ECHO_INPUT;
    } else {
        mode &= !ENABLE_ECHO_INPUT;
    }

    // SAFETY: `handle` is a valid console handle.
    let rc = unsafe { SetConsoleMode(handle, mode) };
    if rc == 0 {
        return Err(
            IoException(format!("SetConsoleMode failed: {}", Error::last_os_error())).into(),
        );
    }
    Ok(previous)
}

/// `wasm` fallback: there is no controllable terminal, so report the requested state as the
/// previous state (effectively a no-op).
#[cfg(target_family = "wasm")]
#[expect(clippy::unnecessary_wraps)]
fn set_echo(on: bool) -> Result<bool> {
    Ok(on)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_bool(true);
        // When stdin is not attached to a TTY (e.g., CI), the underlying syscall returns an
        // error; otherwise the call succeeds and returns a boolean. Accept either, but require
        // that a successful result is a boolean `Value`. The `wasm` no-op implementation must
        // never fail.
        match echo(thread, parameters).await {
            Ok(Some(value)) => {
                let _ = value.as_bool()?;
            }
            Ok(None) => panic!("expected a value"),
            Err(_error) => {
                #[cfg(target_family = "wasm")]
                panic!("wasm implementation must not fail: {_error}");
            }
        }
        Ok(())
    }

    #[cfg(target_family = "wasm")]
    #[tokio::test]
    async fn test_echo_wasm_round_trip() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_bool(false);
        let value = echo(thread, parameters).await?.expect("value");
        assert!(!value.as_bool()?);
        Ok(())
    }
}
