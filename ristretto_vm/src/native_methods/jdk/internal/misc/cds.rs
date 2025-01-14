use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

const JAVA_22: Version = Version::Java22 { minor: 0 };

/// Register all native methods for `jdk.internal.misc.CDS`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/CDS";
    let java_version = registry.java_version();

    if java_version <= &JAVA_22 {
        registry.register(class_name, "isDumpingArchive0", "()Z", is_dumping_archive_0);
        registry.register(
            class_name,
            "isDumpingClassList0",
            "()Z",
            is_dumping_class_list_0,
        );
        registry.register(class_name, "isSharingEnabled0", "()Z", is_sharing_enabled_0);
    } else {
        registry.register(
            class_name,
            "getCDSConfigStatus",
            "()I",
            get_cds_config_status,
        );
    }

    registry.register(
        class_name,
        "defineArchivedModules",
        "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V",
        define_archived_modules,
    );
    registry.register(
        class_name,
        "dumpClassList",
        "(Ljava/lang/String;)V",
        dump_class_list,
    );
    registry.register(
        class_name,
        "dumpDynamicArchive",
        "(Ljava/lang/String;)V",
        dump_dynamic_archive,
    );
    registry.register(
        class_name,
        "getRandomSeedForDumping",
        "()J",
        get_random_seed_for_dumping,
    );
    registry.register(
        class_name,
        "initializeFromArchive",
        "(Ljava/lang/Class;)V",
        initialize_from_archive,
    );
    registry.register(
        class_name,
        "logLambdaFormInvoker",
        "(Ljava/lang/String;)V",
        log_lambda_form_invoker,
    );
}

#[async_recursion(?Send)]
async fn define_archived_modules(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V")
}

#[async_recursion(?Send)]
async fn dump_class_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn dump_dynamic_archive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn get_cds_config_status(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
async fn get_random_seed_for_dumping(
    _thread: Arc<Thread>,
    _arguments: Arguments,
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
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _class = arguments.pop_reference()?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_dumping_archive_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_dumping_class_list_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_sharing_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn log_lambda_form_invoker(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.CDS.logLambdaFormInvoker(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java22 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/CDS";
        assert!(registry
            .method(class_name, "isDumpingArchive0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "isDumpingClassList0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "isSharingEnabled0", "()Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "defineArchivedModules",
                "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "dumpClassList", "(Ljava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "dumpDynamicArchive", "(Ljava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "getRandomSeedForDumping", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "initializeFromArchive", "(Ljava/lang/Class;)V")
            .is_some());
        assert!(registry
            .method(class_name, "logLambdaFormInvoker", "(Ljava/lang/String;)V")
            .is_some());
    }

    #[test]
    fn test_register_java_23() {
        let mut registry = MethodRegistry::new(&Version::Java23 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/CDS";
        assert!(registry
            .method(class_name, "getCDSConfigStatus", "()I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.misc.CDS.defineArchivedModules(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V"
    )]
    async fn test_define_archived_modules() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_archived_modules(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.CDS.dumpClassList(Ljava/lang/String;)V")]
    async fn test_dump_class_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_class_list(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.CDS.dumpDynamicArchive(Ljava/lang/String;)V")]
    async fn test_dump_dynamic_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_dynamic_archive(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_get_cds_config_status() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cds_config_status(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_random_seed_for_dumping() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_random_seed_for_dumping(thread, Arguments::default()).await?;
        let hash = result.unwrap_or(Value::Long(0)).to_long()?;
        assert_ne!(0, hash);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_from_archive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arguments = Arguments::new(vec![Value::Object(None)]);
        let result = initialize_from_archive(thread, arguments).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_dumping_archive_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_dumping_archive_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_dumping_class_list_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_dumping_class_list_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_sharing_enabled_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_sharing_enabled_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.CDS.logLambdaFormInvoker(Ljava/lang/String;)V")]
    async fn test_log_lambda_form_invoker() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = log_lambda_form_invoker(thread, Arguments::default()).await;
    }
}
