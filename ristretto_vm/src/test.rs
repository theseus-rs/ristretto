use crate::frame::Frame;
use crate::{Class, ConfigurationBuilder, Result, Thread, VM};
use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_classloader::ClassPath;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) async fn thread() -> Result<(Arc<VM>, Arc<Thread>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("..").join("classes");
    let class_path = ClassPath::from(classes_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    let weak_vm = Arc::downgrade(&vm);
    let thread = Thread::new(&weak_vm, 3)?;
    Ok((vm, thread))
}

/// Get a test class for testing.
pub(crate) async fn class() -> Result<(Arc<VM>, Arc<Thread>, Arc<Class>)> {
    let (vm, thread) = thread().await?;
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("Test")?;
    let test_index = constant_pool.add_utf8("test")?;
    let test_descriptor_index = constant_pool.add_utf8("()V")?;
    let test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: test_index,
        descriptor_index: test_descriptor_index,
        ..Default::default()
    };

    let class_file = ClassFile {
        constant_pool,
        this_class,
        methods: vec![test_method],
        ..Default::default()
    };
    let class = Class::from(None, class_file)?;
    Ok((vm, thread, class))
}

/// Get a test frame for testing.
pub(crate) async fn frame() -> Result<(Arc<VM>, Arc<Thread>, Frame)> {
    let (vm, thread, class) = class().await?;
    let method = class.try_get_method("test", "()V")?;
    let frame = Frame::new(&Arc::downgrade(&thread), &class, &method);
    Ok((vm, thread, frame))
}
