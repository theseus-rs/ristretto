use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/VM.getNanoTimeAdjustment(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_nano_time_adjustment<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let offset_secs = parameters.pop_long()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;
    let now_secs = i64::try_from(now.as_secs())
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;
    let nanos_of_sec = i64::from(now.subsec_nanos());
    // Return nanoseconds adjustment: (now_secs - offset_secs) * 1_000_000_000 + nanos_of_sec
    let diff_secs = now_secs.saturating_sub(offset_secs);
    let result = diff_secs
        .saturating_mul(1_000_000_000)
        .saturating_add(nanos_of_sec);
    Ok(Some(Value::Long(result)))
}

#[intrinsic_method(
    "jdk/internal/misc/VM.getRuntimeArguments()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_runtime_arguments<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/internal/misc/VM.getegid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getegid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.VM.getegid()J".to_string()).into())
}

#[intrinsic_method("jdk/internal/misc/VM.geteuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn geteuid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.VM.geteuid()J".to_string()).into())
}

#[intrinsic_method("jdk/internal/misc/VM.getgid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getgid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.VM.getgid()J".to_string()).into())
}

#[intrinsic_method("jdk/internal/misc/VM.getuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getuid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.VM.getuid()J".to_string()).into())
}

#[intrinsic_method("jdk/internal/misc/VM.initialize()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/VM.initializeFromArchive(Ljava/lang/Class;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn initialize_from_archive<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn latest_user_defined_loader_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_nano_time_adjustment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![Value::Long(0)]);
        let result = get_nano_time_adjustment(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(matches!(value, Some(Value::Long(_))));
    }

    #[tokio::test]
    async fn test_get_runtime_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_runtime_arguments(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_getegid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getegid(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.misc.VM.getegid()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_geteuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = geteuid(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.misc.VM.geteuid()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_getgid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getgid(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.misc.VM.getgid()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_getuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getuid(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.misc.VM.getuid()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = initialize(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_from_archive() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let value = initialize_from_archive(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_latest_user_defined_loader_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = latest_user_defined_loader_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
