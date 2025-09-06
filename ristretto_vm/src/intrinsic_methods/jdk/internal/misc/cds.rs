use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use tracing::debug;

#[intrinsic_method(
    "jdk/internal/misc/CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_archived_modules(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.dumpClassList(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn dump_class_list(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.dumpDynamicArchive(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn dump_dynamic_archive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V")
}

#[intrinsic_method("jdk/internal/misc/CDS.getCDSConfigStatus()I", GreaterThan(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn get_cds_config_status(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.getRandomSeedForDumping()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
pub(crate) async fn get_random_seed_for_dumping(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let version = env!("CARGO_PKG_VERSION");
    let mut hasher = DefaultHasher::new();
    version.hash(&mut hasher);
    let hash = hasher.finish() as i64;
    Ok(Some(Value::Long(hash)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.initializeFromArchive(Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn initialize_from_archive(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class = parameters.pop_reference()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isDumpingArchive0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_dumping_archive_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isDumpingClassList0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_dumping_class_list_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.isSharingEnabled0()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_sharing_enabled_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/CDS.logLambdaFormInvoker(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn log_lambda_form_invoker(
    _thread: Arc<Thread>,
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
#[async_recursion(?Send)]
pub(crate) async fn needs_class_init_barrier_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class = parameters.pop_reference()?;
    todo!("jdk.internal.misc.CDS.needsClassInitBarrier0(Ljava/lang/Class;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V"
    )]
    async fn test_define_archived_modules() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_archived_modules(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V"
    )]
    async fn test_dump_class_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_class_list(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V"
    )]
    async fn test_dump_dynamic_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_dynamic_archive(thread, Parameters::default()).await;
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
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_dumping_archive_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_dumping_class_list_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_dumping_class_list_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_sharing_enabled_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
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
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.CDS.needsClassInitBarrier0(Ljava/lang/Class;)Z"
    )]
    async fn test_needs_class_init_barrier_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let parameters = Parameters::new(vec![Value::Object(None)]);
        let _ = needs_class_init_barrier_0(thread, parameters).await;
    }
}
