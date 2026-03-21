use ristretto_classfile::VerifyMode;
use ristretto_classloader::{
    ClassPath, DEFAULT_JAVA_VERSION, JAVA_8_VERSION, JAVA_11_VERSION, JAVA_17_VERSION,
    JAVA_21_VERSION, JAVA_25_VERSION,
};
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_vm::{ConfigurationBuilder, Thread, VM};
use std::path::PathBuf;
use std::sync::Arc;

/// Create a VM and thread configured for the default Java version.
pub(crate) async fn thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(DEFAULT_JAVA_VERSION).await
}

/// Create a VM and thread configured for Java 25.
pub(crate) async fn java25_thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(JAVA_25_VERSION).await
}

/// Create a VM and thread configured for Java 21.
pub(crate) async fn java21_thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(JAVA_21_VERSION).await
}

/// Create a VM and thread configured for Java 17.
pub(crate) async fn java17_thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(JAVA_17_VERSION).await
}

/// Create a VM and thread configured for Java 11.
pub(crate) async fn java11_thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(JAVA_11_VERSION).await
}

/// Create a VM and thread configured for Java 8.
pub(crate) async fn java8_thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    version_thread(JAVA_8_VERSION).await
}

/// Create a VM and thread configured for the specified Java version.
pub(crate) async fn version_thread(
    version: &str,
) -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("..").join("classes");
    let class_path = ClassPath::from(&[classes_path]);
    let gc_config = GcConfigurationBuilder::new().threads(1).build();
    let garbage_collector = GarbageCollector::with_config(gc_config);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .java_version(version)
        .verify_mode(VerifyMode::None)
        .garbage_collector(garbage_collector)
        .batch_compilation(false)
        .build()?;
    let vm = VM::new(configuration).await?;
    let weak_vm = Arc::downgrade(&vm);
    let thread = Thread::new(&weak_vm, 3);
    Ok((vm, thread))
}
