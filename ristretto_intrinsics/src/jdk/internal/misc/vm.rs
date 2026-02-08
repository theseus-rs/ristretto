use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/VM.getNanoTimeAdjustment(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_nano_time_adjustment<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getNanoTimeAdjustment(J)J")
}

#[intrinsic_method(
    "jdk/internal/misc/VM.getRuntimeArguments()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_runtime_arguments<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;")
}

#[intrinsic_method("jdk/internal/misc/VM.getegid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getegid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getegid()J")
}

#[intrinsic_method("jdk/internal/misc/VM.geteuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn geteuid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.geteuid()J")
}

#[intrinsic_method("jdk/internal/misc/VM.getgid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getgid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getgid()J")
}

#[intrinsic_method("jdk/internal/misc/VM.getuid()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn getuid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getuid()J")
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
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.VM.getNanoTimeAdjustment(J)J"
    )]
    async fn test_get_nano_time_adjustment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nano_time_adjustment(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;"
    )]
    async fn test_get_runtime_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_runtime_arguments(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getegid()J")]
    async fn test_getegid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getegid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.geteuid()J")]
    async fn test_geteuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = geteuid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getgid()J")]
    async fn test_getgid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getgid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getuid()J")]
    async fn test_getuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getuid(thread, Parameters::default()).await;
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
