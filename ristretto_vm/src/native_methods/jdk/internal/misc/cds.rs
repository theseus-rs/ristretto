use crate::native_methods::registry::{MethodRegistry, JAVA_22};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/misc/CDS";

/// Register all native methods for `jdk.internal.misc.CDS`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_22 {
        registry.register(CLASS_NAME, "isDumpingArchive0", "()Z", is_dumping_archive_0);
        registry.register(
            CLASS_NAME,
            "isDumpingClassList0",
            "()Z",
            is_dumping_class_list_0,
        );
        registry.register(CLASS_NAME, "isSharingEnabled0", "()Z", is_sharing_enabled_0);
    } else {
        registry.register(
            CLASS_NAME,
            "getCDSConfigStatus",
            "()I",
            get_cds_config_status,
        );
    }

    registry.register(
        CLASS_NAME,
        "defineArchivedModules",
        "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V",
        define_archived_modules,
    );
    registry.register(
        CLASS_NAME,
        "dumpClassList",
        "(Ljava/lang/String;)V",
        dump_class_list,
    );
    registry.register(
        CLASS_NAME,
        "dumpDynamicArchive",
        "(Ljava/lang/String;)V",
        dump_dynamic_archive,
    );
    registry.register(
        CLASS_NAME,
        "getRandomSeedForDumping",
        "()J",
        get_random_seed_for_dumping,
    );
    registry.register(
        CLASS_NAME,
        "initializeFromArchive",
        "(Ljava/lang/Class;)V",
        initialize_from_archive,
    );
    registry.register(
        CLASS_NAME,
        "logLambdaFormInvoker",
        "(Ljava/lang/String;)V",
        log_lambda_form_invoker,
    );
}

#[async_recursion(?Send)]
async fn define_archived_modules(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V")
}

#[async_recursion(?Send)]
async fn dump_class_list(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn dump_dynamic_archive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn get_cds_config_status(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
async fn get_random_seed_for_dumping(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let version = env!("CARGO_PKG_VERSION");
    let mut hasher = DefaultHasher::new();
    version.hash(&mut hasher);
    let hash = hasher.finish() as i64;
    Ok(Some(Value::Long(hash)))
}

#[async_recursion(?Send)]
async fn initialize_from_archive(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class = parameters.pop_reference()?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_dumping_archive_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_dumping_class_list_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_sharing_enabled_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn log_lambda_form_invoker(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.logLambdaFormInvoker(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let hash = result.unwrap_or(Value::Long(0)).to_long()?;
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
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.CDS.logLambdaFormInvoker(Ljava/lang/String;)V"
    )]
    async fn test_log_lambda_form_invoker() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log_lambda_form_invoker(thread, Parameters::default()).await;
    }
}
