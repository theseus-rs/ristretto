use ristretto_classfile::VerifyMode;
use ristretto_classloader::ClassPath;
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_vm::{ConfigurationBuilder, Thread, VM};
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) async fn thread() -> ristretto_types::Result<(Arc<VM>, Arc<Thread>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("..").join("classes");
    let class_path = ClassPath::from(&[classes_path]);
    let gc_config = GcConfigurationBuilder::new().threads(1).build();
    let garbage_collector = GarbageCollector::with_config(gc_config);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .verify_mode(VerifyMode::None)
        .garbage_collector(garbage_collector)
        .batch_compilation(false)
        .build()?;
    let vm = VM::new(configuration).await?;
    let weak_vm = Arc::downgrade(&vm);
    let thread = Thread::new(&weak_vm, 3);
    Ok((vm, thread))
}
