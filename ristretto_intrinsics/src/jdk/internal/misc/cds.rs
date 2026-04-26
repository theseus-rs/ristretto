use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use tracing::debug;

#[intrinsic_method(
    "jdk/internal/misc/CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn define_archived_modules<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _system_loader = parameters.pop_reference()?;
    let _platform_loader = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.dumpClassList(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn dump_class_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _file_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.dumpDynamicArchive(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn dump_dynamic_archive<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _archive_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/internal/misc/CDS.getCDSConfigStatus()I", GreaterThan(JAVA_21))]
#[async_method]
pub async fn get_cds_config_status<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.getRandomSeedForDumping()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_random_seed_for_dumping<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let version = env!("CARGO_PKG_VERSION");
    let mut hasher = DefaultHasher::new();
    version.hash(&mut hasher);
    let hash: i64 = zerocopy::transmute!(hasher.finish());
    Ok(Some(Value::Long(hash)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.initializeFromArchive(Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn initialize_from_archive<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class = parameters.pop_reference()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isDumpingArchive0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn is_dumping_archive_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isDumpingClassList0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn is_dumping_class_list_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isSharingEnabled0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn is_sharing_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.logLambdaFormInvoker(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn log_lambda_form_invoker<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let log_line = parameters.pop()?.as_string()?;
    debug!("CDS.logLambdaFormInvoker: {log_line}");
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.needsClassInitBarrier0(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn needs_class_init_barrier_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _c = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.misc.CDS.needsClassInitBarrier0(Ljava/lang/Class;)Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_define_archived_modules() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = define_archived_modules(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dump_class_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dump_class_list(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dump_dynamic_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dump_dynamic_archive(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cds_config_status() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cds_config_status(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_random_seed_for_dumping() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_random_seed_for_dumping(thread, Parameters::default()).await?;
        let hash = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert_ne!(0, hash);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_from_archive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Object(None)]);
        let result = initialize_from_archive(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_dumping_archive_0() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = is_dumping_archive_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_dumping_class_list_0() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = is_dumping_class_list_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_sharing_enabled_0() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = is_sharing_enabled_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_log_lambda_form_invoker() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let line = "Test log from logLambdaFormInvoker"
            .to_object(&thread)
            .await?;
        let parameters = Parameters::new(vec![line]);
        let result = log_lambda_form_invoker(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_needs_class_init_barrier_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            needs_class_init_barrier_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.misc.CDS.needsClassInitBarrier0(Ljava/lang/Class;)Z",
            result.unwrap_err().to_string()
        );
    }
}
