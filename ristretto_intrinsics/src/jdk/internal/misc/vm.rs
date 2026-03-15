use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_DIFF_SECS: i64 = 0x0100_0000_0000i64;

#[intrinsic_method(
    "jdk/internal/misc/VM.getNanoTimeAdjustment(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_nano_time_adjustment<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let offset_in_seconds = parameters.pop_long()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let now_secs = i64::try_from(now.as_secs()).unwrap_or(i64::MAX);
    let diff = now_secs.saturating_sub(offset_in_seconds);
    if !(-MAX_DIFF_SECS..=MAX_DIFF_SECS).contains(&diff) {
        return Ok(Some(Value::Long(-1)));
    }
    let nanos = diff
        .saturating_mul(1_000_000_000)
        .saturating_add(i64::from(now.subsec_nanos()));
    Ok(Some(Value::Long(nanos)))
}

#[intrinsic_method(
    "jdk/internal/misc/VM.getRuntimeArguments()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_runtime_arguments<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let string_class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((string_class, Vec::<Value>::new()))?;
    let vm = thread.vm()?;
    let array = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(array))
}

#[intrinsic_method("jdk/internal/misc/VM.getegid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getegid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(ristretto_types::Error::InternalError(
        "jdk.internal.misc.VM.getegid()J not implemented".into(),
    ))
}

#[intrinsic_method("jdk/internal/misc/VM.geteuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn geteuid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(ristretto_types::Error::InternalError(
        "jdk.internal.misc.VM.geteuid()J not implemented".into(),
    ))
}

#[intrinsic_method("jdk/internal/misc/VM.getgid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getgid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(ristretto_types::Error::InternalError(
        "jdk.internal.misc.VM.getgid()J not implemented".into(),
    ))
}

#[intrinsic_method("jdk/internal/misc/VM.getuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getuid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(ristretto_types::Error::InternalError(
        "jdk.internal.misc.VM.getuid()J not implemented".into(),
    ))
}

#[intrinsic_method("jdk/internal/misc/VM.initialize()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn initialize<T: ristretto_types::Thread + 'static>(
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
pub async fn initialize_from_archive<T: ristretto_types::Thread + 'static>(
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
pub async fn latest_user_defined_loader_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_nano_time_adjustment() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = get_nano_time_adjustment(thread, params).await?;
        assert!(matches!(result, Some(Value::Long(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_runtime_arguments() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_runtime_arguments(thread, Parameters::default()).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_getegid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = getegid(thread, Parameters::default()).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_geteuid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = geteuid(thread, Parameters::default()).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_getgid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = getgid(thread, Parameters::default()).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_getuid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = getuid(thread, Parameters::default()).await;
        assert!(result.is_err());
        Ok(())
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
        let (_vm, thread) = crate::test::thread().await?;
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
