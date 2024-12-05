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
