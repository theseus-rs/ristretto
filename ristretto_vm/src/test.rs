use crate::frame::Frame;
use crate::{Class, ConfigurationBuilder, Result, Thread, VM};
use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_classloader::{ClassPath, Method};
use std::path::PathBuf;
use std::sync::Arc;

/// Get the specific class for testing.
pub(crate) async fn load_class(class_name: &str) -> Result<(Arc<VM>, Arc<Thread>, Arc<Class>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("../classes");
    let class_path = ClassPath::from(classes_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    let thread = Thread::new(&Arc::downgrade(&vm));
    let class = thread.class(class_name).await?;
    Ok((vm, thread, class))
}

/// Get a test class for testing.
pub(crate) async fn class() -> Result<(Arc<VM>, Arc<Thread>, Arc<Class>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("../classes");
    let class_path = ClassPath::from(classes_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    let thread = Thread::new(&Arc::downgrade(&vm));
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("Test")?;
    let class_file = ClassFile {
        constant_pool,
        this_class,
        ..Default::default()
    };
    let class = Class::from(class_file)?;
    Ok((vm, thread, Arc::new(class)))
}

/// Get a test frame for testing.
pub(crate) async fn frame() -> Result<(Arc<VM>, Arc<Thread>, Frame)> {
    let (vm, thread, class) = class().await?;
    let method = Method::new(
        MethodAccessFlags::STATIC,
        "test",
        "()V",
        10,
        10,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )?;
    let arguments = Vec::new();
    let frame = Frame::new(
        &Arc::downgrade(&thread),
        &class,
        &Arc::new(method),
        arguments,
    )?;
    Ok((vm, thread, frame))
}
