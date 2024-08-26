use crate::frame::Frame;
use crate::{CallStack, Class, ConfigurationBuilder, Result, VM};
use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_classloader::{ClassPath, Method};
use std::path::PathBuf;
use std::sync::Arc;

/// Get the specific class for testing.
pub(crate) fn load_class(class_name: &str) -> Result<(VM, CallStack, Arc<Class>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("../classes");
    let class_path = ClassPath::from(classes_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .build();
    let vm = VM::new(configuration)?;
    let mut call_stack = CallStack::new();
    let class = vm.class(&mut call_stack, class_name)?;
    Ok((vm, call_stack, class))
}

/// Get a test class for testing.
pub(crate) fn class() -> Result<(VM, CallStack, Arc<Class>)> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_path = cargo_manifest.join("../classes");
    let class_path = ClassPath::from(classes_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .build();
    let vm = VM::new(configuration)?;
    let call_stack = CallStack::new();
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("Test")?;
    let class_file = ClassFile {
        constant_pool,
        this_class,
        ..Default::default()
    };
    let class = Class::from(class_file)?;
    Ok((vm, call_stack, Arc::new(class)))
}

/// Get a test frame for testing.
pub(crate) fn frame() -> Result<(VM, CallStack, Frame)> {
    let (vm, call_stack, class) = class()?;
    let method = Method::new(
        MethodAccessFlags::STATIC,
        "test",
        "()V",
        10,
        10,
        Vec::new(),
        Vec::new(),
    )?;
    let arguments = Vec::new();
    let frame = Frame::new(&class, &Arc::new(method), arguments)?;
    Ok((vm, call_stack, frame))
}
