use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Checks if the current process is running with setUID privileges.
///
/// On Unix, this compares the real UID (`getuid`) with the effective UID (`geteuid`).
/// If they differ, the process is running as setUID.
/// On non-Unix platforms (Windows, WASM), setUID does not apply and this always returns `false`.
#[intrinsic_method("java/util/logging/FileHandler.isSetUID()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn is_set_uid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let is_setuid = platform::is_set_uid();
    Ok(Some(Value::from(is_setuid)))
}

#[cfg(target_family = "unix")]
mod platform {
    /// Returns `true` if the real UID differs from the effective UID.
    pub fn is_set_uid() -> bool {
        #[expect(unsafe_code)]
        // SAFETY: getuid and geteuid are standard POSIX functions with no arguments
        // and no side effects; they simply return the real and effective user IDs.
        unsafe {
            libc::getuid() != libc::geteuid()
        }
    }
}

#[cfg(not(target_family = "unix"))]
mod platform {
    /// setUID is not applicable on non-Unix platforms; always returns `false`.
    pub fn is_set_uid() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_set_uid() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = is_set_uid(thread, Parameters::default()).await?;
        // On most test environments, the process is not running as setUID
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }
}
