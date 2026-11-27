//! The Java reference handler must survive returns from WASI host API calls.
#![cfg(target_os = "wasi")]
#![expect(clippy::panic_in_result_fn)]
use ristretto_classloader::{Reference, Value};
use ristretto_vm::{ConfigurationBuilder, Result, VM};

fn compact_builder() -> ConfigurationBuilder {
    ConfigurationBuilder::new().garbage_collector(ristretto_gc::GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .compact_heap(true)
            .region_layout(1024 * 1024, 4096)
            .build(),
    ))
}

#[tokio::test]
async fn reference_handler_survives_host_calls() -> Result<()> {
    let config = if let Ok(home) = std::env::var("RISTRETTO_GC_JAVA_HOME") {
        compact_builder().java_home(home.into()).build()?
    } else {
        compact_builder().build()?
    };
    let vm = VM::new(config).await?;
    let gc = vm.garbage_collector().clone();
    let queue = vm
        .object("java/lang/ref/ReferenceQueue", "", &[] as &[Value])
        .await?;
    let target = Value::try_new_object(&gc, Reference::ByteArray(vec![7; 1024].into()))?;
    let weak = vm
        .object(
            "java/lang/ref/WeakReference",
            "Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;",
            &[target.clone(), queue.clone()],
        )
        .await?;
    drop(target);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.as_object_ref()?.value("referent")?.is_null());
    let mut enqueued = false;
    for _ in 0..100 {
        let value = vm
            .invoke(
                "java/lang/ref/ReferenceQueue",
                "poll()Ljava/lang/ref/Reference;",
                std::slice::from_ref(&queue),
            )
            .await?;
        if value.is_some_and(|value| !value.is_null()) {
            enqueued = true;
            break;
        }
    }
    assert!(enqueued, "reference handler disappeared between host calls");
    let weak_vm = std::sync::Arc::downgrade(&vm);
    let weak_gc = std::sync::Arc::downgrade(&gc);
    drop((weak, queue, vm, gc));
    assert!(weak_vm.upgrade().is_none());
    assert!(
        weak_gc.upgrade().is_none(),
        "scheduler retained cancelled task roots"
    );
    Ok(())
}

async fn finalizable_class_path() -> Result<ristretto_classloader::ClassPath> {
    use ristretto_classfile::attributes::{Attribute, Instruction};
    use ristretto_classfile::{
        ClassAccessFlags, ClassFile, ConstantPool, Field, FieldAccessFlags, FieldType, JAVA_8,
        Method, MethodAccessFlags,
    };
    use ristretto_classloader::{ClassPath, ClassPathEntry, Memory};
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("GcFinalizable")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let constructor = pool.add_method_ref(super_class, "<init>", "()V")?;
    let finalized = pool.add_field_ref(this_class, "finalized", "I")?;
    let resurrected = pool.add_field_ref(this_class, "resurrected", "LGcFinalizable;")?;
    let code_name = pool.add_utf8("Code")?;
    let mut fields = Vec::new();
    for (name, descriptor) in [("finalized", "I"), ("resurrected", "LGcFinalizable;")] {
        fields.push(Field {
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8(descriptor)?,
            field_type: FieldType::parse(descriptor)?,
            attributes: Vec::new(),
        });
    }
    let mut methods = Vec::new();
    for (name, descriptor, flags, locals, code) in [
        (
            "<init>",
            "()V",
            MethodAccessFlags::PUBLIC,
            1,
            vec![
                Instruction::Aload_0,
                Instruction::Invokespecial(constructor),
                Instruction::Return,
            ],
        ),
        (
            "finalize",
            "()V",
            MethodAccessFlags::PROTECTED,
            1,
            vec![
                Instruction::Getstatic(finalized),
                Instruction::Iconst_1,
                Instruction::Iadd,
                Instruction::Putstatic(finalized),
                Instruction::Aload_0,
                Instruction::Putstatic(resurrected),
                Instruction::Return,
            ],
        ),
        (
            "count",
            "()I",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            0,
            vec![Instruction::Getstatic(finalized), Instruction::Ireturn],
        ),
    ] {
        methods.push(Method {
            access_flags: flags,
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8(descriptor)?,
            attributes: vec![Attribute::Code {
                name_index: code_name,
                max_stack: 2,
                max_locals: locals,
                code,
                exception_table: Vec::new(),
                attributes: Vec::new(),
            }],
        });
    }
    let file = ClassFile {
        version: JAVA_8,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        constant_pool: pool,
        this_class,
        super_class,
        fields,
        methods,
        ..Default::default()
    };
    let mut bytes = Vec::new();
    file.to_bytes(&mut bytes)?;
    let memory = Memory::new("gc-finalization");
    memory.add_class(&bytes).await?;
    Ok(ClassPath::new(vec![ClassPathEntry::Memory(memory)]))
}

#[tokio::test]
async fn finalization_resurrection_and_phantom_ordering() -> Result<()> {
    let mut config = compact_builder().class_path(finalizable_class_path().await?);
    if let Ok(home) = std::env::var("RISTRETTO_GC_JAVA_HOME") {
        config = config.java_home(home.into());
    }
    let vm = VM::new(config.build()?).await?;
    let gc = vm.garbage_collector();
    let target = vm.object("GcFinalizable", "", &[] as &[Value]).await?;
    let weak = vm
        .object(
            "java/lang/ref/WeakReference",
            "Ljava/lang/Object;",
            std::slice::from_ref(&target),
        )
        .await?;
    let queue = vm
        .object("java/lang/ref/ReferenceQueue", "", &[] as &[Value])
        .await?;
    let phantom = vm
        .object(
            "java/lang/ref/PhantomReference",
            "Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;",
            &[target.clone(), queue.clone()],
        )
        .await?;
    drop(target);
    gc.collect_full()?;
    gc.collect_full()?;
    for _ in 0..100 {
        if vm
            .invoke("GcFinalizable", "count()I", &[] as &[Value])
            .await?
            == Some(Value::Int(1))
        {
            break;
        }
    }
    let class = vm.class("GcFinalizable").await?;
    assert_eq!(class.static_value("finalized")?, Value::Int(1));
    assert!(!class.static_value("resurrected")?.is_null());
    assert!(weak.as_object_ref()?.value("referent")?.is_null());
    class.set_static_value("resurrected", Value::Object(None))?;
    gc.collect_full()?;
    gc.collect_full()?;
    let mut enqueued = false;
    for _ in 0..100 {
        if vm
            .invoke(
                "java/lang/ref/ReferenceQueue",
                "poll()Ljava/lang/ref/Reference;",
                std::slice::from_ref(&queue),
            )
            .await?
            .is_some_and(|value| !value.is_null())
        {
            enqueued = true;
            break;
        }
    }
    assert!(
        enqueued,
        "phantom processing must follow finalization and resurrection release"
    );
    assert_eq!(class.static_value("finalized")?, Value::Int(1));
    assert!(phantom.as_object_ref()?.value("referent")?.is_null());
    Ok(())
}
