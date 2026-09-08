use super::*;
use crate::Error::JavaError;
use crate::JavaError::{NoClassDefFoundError, NullPointerException};
use crate::instruction::{execute_field, try_field};
use crate::{OperandStack, Thread};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_classloader::{ClassLoader, ClassPath};

fn definition(
    name: &str,
    fields: &[(&str, &str, FieldAccessFlags)],
    parent: Option<&Arc<Class>>,
) -> Result<ClassFile<'static>> {
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class(name)?;
    let super_class = parent.map_or(Ok(0), |parent| pool.add_class(parent.name()))?;
    let fields = fields
        .iter()
        .map(|(name, descriptor, access_flags)| {
            Ok(ristretto_classfile::Field {
                access_flags: *access_flags,
                name_index: pool.add_utf8(name)?,
                descriptor_index: pool.add_utf8(descriptor)?,
                field_type: FieldType::parse(descriptor)?,
                attributes: vec![],
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut methods = Vec::new();
    for name in ["test", "<init>", "<clinit>"] {
        methods.push(ristretto_classfile::Method {
            access_flags: if name == "<clinit>" {
                MethodAccessFlags::STATIC
            } else {
                MethodAccessFlags::PUBLIC
            },
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8("()V")?,
            attributes: vec![Attribute::Code {
                name_index: pool.add_utf8("Code")?,
                max_stack: 0,
                max_locals: 1,
                code: vec![Instruction::Return],
                exception_table: vec![],
                attributes: vec![],
            }],
        });
    }
    Ok(ClassFile {
        access_flags: ClassAccessFlags::PUBLIC,
        constant_pool: pool,
        this_class,
        super_class,
        fields,
        methods,
        ..Default::default()
    })
}

fn frame(thread: &Arc<Thread>, class: &Arc<Class>, method: &str) -> Result<Frame> {
    Ok(Frame::new(
        &Arc::downgrade(thread),
        class,
        &class.try_get_method(method, "()V")?,
    ))
}

async fn reference(
    thread: &Arc<Thread>,
    mut definition: ClassFile<'static>,
    target: &str,
    name: &str,
    descriptor: &str,
) -> Result<(Frame, u16)> {
    let pool = &mut definition.constant_pool;
    let target = pool.add_class(target)?;
    let index = pool.add_field_ref(target, name, descriptor)?;
    let class = Class::from(None, definition)?;
    thread
        .load_referenced_class(&class, class.java_name())
        .await?;
    Ok((frame(thread, &class, "test")?, index))
}

#[tokio::test]
async fn instance_slots_distinguish_descriptors_and_hidden_fields() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let initial_cache_size = vm.field_ref_cache().len();
    let base = Class::from(
        None,
        definition("Base", &[("value", "I", FieldAccessFlags::PUBLIC)], None)?,
    )?;
    thread.register_class(base.clone()).await?;
    let child = Class::from(
        None,
        definition(
            "Child",
            &[("value", "J", FieldAccessFlags::PUBLIC)],
            Some(&base),
        )?,
    )?;
    thread.register_class(child.clone()).await?;
    thread
        .load_referenced_class(&child, child.java_name())
        .await?;
    let mut object = Object::new(child.clone())?;
    for (descriptor, value) in [("I", Value::Int(42)), ("J", Value::Long(99))] {
        let (caller, index) = reference(
            &thread,
            definition("Caller", &[], None)?,
            "Child",
            "value",
            descriptor,
        )
        .await?;
        let resolved = resolve_field_ref(&caller, index).await?;
        resolved.put(&mut object, value.clone())?;
        assert_eq!(resolved.get(&object)?, value);
        let second_frame = frame(&thread, caller.class(), "test")?;
        assert!(Arc::ptr_eq(
            &resolved,
            &resolve_field_ref(&second_frame, index).await?
        ));
        assert!(Arc::ptr_eq(
            &resolved.declaring_class,
            if descriptor == "I" { &base } else { &child }
        ));
    }
    assert_eq!(object.value_at_slot(0)?, Value::Int(42));
    assert_eq!(object.value_at_slot(1)?, Value::Long(99));
    assert!(!base.is_initialized()?);
    assert!(!child.is_initialized()?);
    assert_eq!(vm.field_ref_cache().len(), initial_cache_size + 2);
    Ok(())
}

#[tokio::test]
async fn missing_descriptor_and_invalid_indices_fail_resolution() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let (caller, index) = reference(
        &thread,
        definition("Target", &[("value", "I", FieldAccessFlags::PUBLIC)], None)?,
        "Target",
        "value",
        "J",
    )
    .await?;
    for _ in 0..2 {
        assert!(matches!(
            resolve_field_ref(&caller, index).await,
            Err(JavaError(NoSuchFieldError(_)))
        ));
    }
    for index in [0, u16::MAX] {
        assert!(resolve_field_ref(&caller, index).await.is_err());
    }
    Ok(())
}

#[tokio::test]
async fn inherited_static_initializes_only_declaring_class() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC;
    let base = Class::from(None, definition("Base", &[("value", "I", flags)], None)?)?;
    thread.register_class(base.clone()).await?;
    let child = Class::from(None, definition("Child", &[], Some(&base))?)?;
    thread.register_class(child.clone()).await?;
    let (caller, index) = reference(
        &thread,
        definition("Caller", &[], None)?,
        "Child",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    assert!(!base.is_initialized()?);
    assert!(!child.is_initialized()?);
    let mut stack = OperandStack::with_max_size(1);
    let get = Instruction::Getstatic(index);
    assert!(try_field(&caller, &mut stack, index, &get)?.is_none());
    execute_field(&caller, &mut stack, index, get.clone()).await?;
    assert_eq!(stack.pop_int()?, 0);
    assert!(base.is_initialized()?);
    assert!(!child.is_initialized()?);
    base.set_static_value("value", Value::Int(17))?;
    assert!(try_field(&caller, &mut stack, index, &get)?.is_some());
    assert_eq!(stack.pop_int()?, 17);
    resolved.put_static(Value::Int(42))?;
    assert_eq!(base.static_value("value")?, Value::Int(42));
    Ok(())
}

#[tokio::test]
async fn interface_field_precedes_superclass_field() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC;
    let base = Class::from(None, definition("Base", &[("value", "I", flags)], None)?)?;
    thread.register_class(base.clone()).await?;
    let mut interface_definition = definition(
        "Fields",
        &[("value", "I", flags | FieldAccessFlags::FINAL)],
        None,
    )?;
    interface_definition.access_flags |= ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT;
    let interface = Class::from(None, interface_definition)?;
    thread.register_class(interface.clone()).await?;
    let mut child_definition = definition("Child", &[], Some(&base))?;
    child_definition
        .interfaces
        .push(child_definition.constant_pool.add_class("Fields")?);
    let (caller, index) = reference(&thread, child_definition, "Child", "value", "I").await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    assert!(Arc::ptr_eq(&resolved.declaring_class, &interface));
    Ok(())
}

#[tokio::test]
async fn final_writes_are_checked_in_each_frame_on_cache_hits() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    for (version, is_static) in [
        (ristretto_classfile::JAVA_8, false),
        (ristretto_classfile::JAVA_8, true),
        (ristretto_classfile::JAVA_9, false),
        (ristretto_classfile::JAVA_9, true),
    ] {
        let mut flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::FINAL;
        if is_static {
            flags |= FieldAccessFlags::STATIC;
        }
        let mut target = definition("Target", &[("value", "I", flags)], None)?;
        target.version = version;
        let (caller, index) = reference(&thread, target, "Target", "value", "I").await?;
        let initializer = frame(
            &thread,
            caller.class(),
            if is_static { "<clinit>" } else { "<init>" },
        )?;
        let resolved = resolve_field_ref(&initializer, index).await?;
        resolved.check_write(&initializer)?;
        assert!(Arc::ptr_eq(
            &resolved,
            &resolve_field_ref(&caller, index).await?
        ));
        if caller.class().class_file().version < ristretto_classfile::JAVA_9 {
            // Legacy class files may write final fields from other declaring-class methods.
            resolved.check_write(&caller)?;
            continue;
        }
        assert!(matches!(
            resolved.check_write(&caller),
            Err(JavaError(IllegalAccessError(_)))
        ));
        let mut stack = OperandStack::with_max_size(2);
        let instruction = if is_static {
            Instruction::Putstatic(index)
        } else {
            Instruction::Putfield(index)
        };
        assert!(matches!(
            execute_field(&caller, &mut stack, index, instruction).await,
            Err(JavaError(IllegalAccessError(_)))
        ));
        assert!(!caller.class().is_initialized()?);
    }
    Ok(())
}

#[tokio::test]
async fn protected_receiver_is_checked_on_every_access() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let base = Class::from(
        None,
        definition(
            "base/Base",
            &[("value", "I", FieldAccessFlags::PROTECTED)],
            None,
        )?,
    )?;
    thread.register_class(base.clone()).await?;
    let (caller, index) = reference(
        &thread,
        definition("child/Child", &[], Some(&base))?,
        "base/Base",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    let mut valid = Object::new(caller.class().clone())?;
    resolved.put(&mut valid, Value::Int(42))?;
    assert_eq!(resolved.get(&valid)?, Value::Int(42));
    let invalid = Object::new(base)?;
    assert!(matches!(
        resolved.get(&invalid),
        Err(JavaError(IllegalAccessError(_)))
    ));
    Ok(())
}

#[tokio::test]
async fn private_fields_and_packages_respect_class_loader_identity() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let target = Class::from(
        None,
        definition("Target", &[("value", "I", FieldAccessFlags::PRIVATE)], None)?,
    )?;
    thread.register_class(target).await?;
    let (caller, index) = reference(
        &thread,
        definition("Caller", &[], None)?,
        "Target",
        "value",
        "I",
    )
    .await?;
    assert!(matches!(
        resolve_field_ref(&caller, index).await,
        Err(JavaError(IllegalAccessError(_)))
    ));
    let first_loader = ClassLoader::new("first", ClassPath::new(vec![]));
    let second_loader = ClassLoader::new("second", ClassPath::new(vec![]));
    let first = Class::from(
        Some(Arc::downgrade(&first_loader)),
        definition("pkg/Same", &[], None)?,
    )?;
    let second = Class::from(
        Some(Arc::downgrade(&second_loader)),
        definition("pkg/Same", &[], None)?,
    )?;
    assert!(!same_runtime_package(&first, &second)?);
    assert!(!is_subclass_or_same(&first, &second)?);
    let cache = FieldRefCache::new();
    assert!(!Arc::ptr_eq(
        &cache.for_class(&first),
        &cache.for_class(&second)
    ));
    assert!(Arc::ptr_eq(
        &cache.for_class(&first),
        &cache.for_class(&first)
    ));
    Ok(())
}

#[tokio::test]
async fn null_receivers_and_invalid_values_fail_on_cached_access() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let (caller, index) = reference(
        &thread,
        definition("Target", &[("value", "I", FieldAccessFlags::PUBLIC)], None)?,
        "Target",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    let mut stack = OperandStack::with_max_size(1);
    stack.push_object(None)?;
    assert!(matches!(
        try_field(&caller, &mut stack, index, &Instruction::Getfield(index)),
        Err(JavaError(NullPointerException(_)))
    ));
    let mut object = Object::new(caller.class().clone())?;
    assert!(resolved.put(&mut object, Value::Long(42)).is_err());
    assert_eq!(resolved.get(&object)?, Value::Int(0));
    assert!(matches!(
        resolved.check_kind(true),
        Err(JavaError(IncompatibleClassChangeError(_)))
    ));
    Ok(())
}

#[tokio::test]
async fn cached_static_access_waits_for_initialization_and_preserves_failure() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC;
    let (caller, index) = reference(
        &thread,
        definition("Target", &[("value", "I", flags)], None)?,
        "Target",
        "value",
        "I",
    )
    .await?;
    resolve_field_ref(&caller, index).await?;
    caller.class().begin_initialization(thread.id())?;
    let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let other_frame = frame(&other_thread, caller.class(), "test")?;
    let mut stack = OperandStack::with_max_size(1);
    let get = Instruction::Getstatic(index);
    assert!(try_field(&other_frame, &mut stack, index, &get)?.is_none());
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(20),
            execute_field(&other_frame, &mut stack, index, get.clone())
        )
        .await
        .is_err()
    );
    assert!(stack.is_empty());
    caller
        .class()
        .fail_initialization("test failure".to_owned())?;
    assert!(matches!(
        execute_field(&other_frame, &mut stack, index, get).await,
        Err(JavaError(NoClassDefFoundError(_)))
    ));
    Ok(())
}

#[tokio::test]
async fn private_access_requires_reciprocal_nest_membership() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let mut host_definition = definition(
        "pkg/Host",
        &[("value", "I", FieldAccessFlags::PRIVATE)],
        None,
    )?;
    host_definition.attributes.push(Attribute::NestMembers {
        name_index: host_definition.constant_pool.add_utf8("NestMembers")?,
        class_indexes: vec![host_definition.constant_pool.add_class("pkg/Member")?],
    });
    let host = Class::from(None, host_definition)?;
    thread.register_class(host.clone()).await?;
    for name in ["pkg/Member", "pkg/Intruder"] {
        let mut member = definition(name, &[], None)?;
        member.attributes.push(Attribute::NestHost {
            name_index: member.constant_pool.add_utf8("NestHost")?,
            host_class_index: member.constant_pool.add_class("pkg/Host")?,
        });
        let (caller, index) = reference(&thread, member, "pkg/Host", "value", "I").await?;
        let result = resolve_field_ref(&caller, index).await;
        if name == "pkg/Member" {
            assert!(Arc::ptr_eq(&result?.declaring_class, &host));
        } else {
            assert!(matches!(result, Err(JavaError(IllegalAccessError(_)))));
        }
    }
    Ok(())
}

#[tokio::test]
async fn stores_narrow_integer_values() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    for (descriptor, input, expected) in [
        ("Z", 2, 0),
        ("B", 255, -1),
        ("C", -1, 65_535),
        ("S", 65_535, -1),
    ] {
        for is_static in [false, true] {
            let mut flags = FieldAccessFlags::PUBLIC;
            if is_static {
                flags |= FieldAccessFlags::STATIC;
            }
            let (caller, index) = reference(
                &thread,
                definition("Target", &[("value", descriptor, flags)], None)?,
                "Target",
                "value",
                descriptor,
            )
            .await?;
            let resolved = resolve_field_ref(&caller, index).await?;
            if is_static {
                resolved.put_static(Value::Int(input))?;
                assert_eq!(resolved.get_static()?, Value::Int(expected));
            } else {
                let mut object = Object::new(caller.class().clone())?;
                resolved.put(&mut object, Value::Int(input))?;
                assert_eq!(resolved.get(&object)?, Value::Int(expected));
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn receiver_cache_preserves_protected_checks_across_receiver_types() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let base = Class::from(
        None,
        definition(
            "base/Base",
            &[("value", "I", FieldAccessFlags::PROTECTED)],
            None,
        )?,
    )?;
    thread.register_class(base.clone()).await?;
    let (caller, index) = reference(
        &thread,
        definition("child/Child", &[], Some(&base))?,
        "base/Base",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    let loader = ClassLoader::new("other", ClassPath::new(vec![]));
    let impostor = Class::from(
        Some(Arc::downgrade(&loader)),
        definition("child/Child", &[], Some(&base))?,
    )?;
    impostor.set_parent(Some(base.clone()))?;
    impostor.finalize_field_layout()?;
    let invalid = Object::new(impostor.clone())?;
    // Exercise both the first-receiver fast path and additional polymorphic entries.
    for i in 0..6 {
        let child = Class::from(
            None,
            definition(&format!("child/Receiver{i}"), &[], Some(caller.class()))?,
        )?;
        child.set_parent(Some(caller.class().clone()))?;
        child.finalize_field_layout()?;
        let mut object = Object::new(child.clone())?;
        resolved.put(&mut object, Value::Int(i))?;
        assert!(resolved.receivers.contains(&child));
        assert_eq!(resolved.get(&object)?, Value::Int(i));
        assert!(resolved.put(&mut object, Value::Long(0)).is_err());
        assert_eq!(resolved.get(&object)?, Value::Int(i));
        assert!(matches!(
            resolved.get(&invalid),
            Err(JavaError(IllegalAccessError(_)))
        ));
        assert!(!resolved.receivers.contains(&impostor));
    }
    // A matching class name in another loader must not pass declaring-class validation either.
    let unrelated = Class::from(
        Some(Arc::downgrade(&loader)),
        definition(
            "base/Base",
            &[("value", "I", FieldAccessFlags::PUBLIC)],
            None,
        )?,
    )?;
    unrelated.finalize_field_layout()?;
    assert!(matches!(
        resolved.get(&Object::new(unrelated.clone())?),
        Err(JavaError(IncompatibleClassChangeError(_)))
    ));
    assert!(!resolved.receivers.contains(&unrelated));
    Ok(())
}

#[tokio::test]
async fn mutable_unlinked_receivers_are_rechecked() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let (caller, index) = reference(
        &thread,
        definition("Base", &[("value", "I", FieldAccessFlags::PUBLIC)], None)?,
        "Base",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    let receiver = Class::from(None, definition("Child", &[], Some(caller.class()))?)?;
    receiver.set_parent(Some(caller.class().clone()))?;
    let object = Object::new(receiver.clone())?;
    assert_eq!(resolved.get(&object)?, Value::Int(0));
    assert!(!resolved.receivers.contains(&receiver));
    receiver.set_parent(None)?;
    assert!(matches!(
        resolved.get(&object),
        Err(JavaError(IncompatibleClassChangeError(_)))
    ));
    receiver.set_parent(Some(caller.class().clone()))?;
    receiver.finalize_field_layout()?;
    assert_eq!(resolved.get(&object)?, Value::Int(0));
    assert!(resolved.receivers.contains(&receiver));
    assert!(receiver.set_parent(None).is_err());
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn receiver_cache_supports_concurrent_publication() -> Result<()> {
    let (_vm, thread) = crate::test::thread().await?;
    let (caller, index) = reference(
        &thread,
        definition("Base", &[("value", "I", FieldAccessFlags::PUBLIC)], None)?,
        "Base",
        "value",
        "I",
    )
    .await?;
    let resolved = resolve_field_ref(&caller, index).await?;
    let mut classes = vec![caller.class().clone()];
    for i in 0..3 {
        let class = Class::from(
            None,
            definition(&format!("Child{i}"), &[], Some(caller.class()))?,
        )?;
        class.set_parent(Some(caller.class().clone()))?;
        class.finalize_field_layout()?;
        classes.push(class);
    }
    std::thread::scope(|scope| -> Result<()> {
        let handles = (0..8)
            .map(|_| {
                let classes = &classes;
                let resolved = &resolved;
                scope.spawn(move || -> Result<()> {
                    for class in classes {
                        let mut object = Object::new(class.clone())?;
                        for value in 0..10 {
                            resolved.put(&mut object, Value::Int(value))?;
                            assert_eq!(resolved.get(&object)?, Value::Int(value));
                        }
                    }
                    Ok(())
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().expect("receiver access thread must finish")?;
        }
        Ok(())
    })?;
    for class in classes {
        assert!(resolved.receivers.contains(&class));
    }
    Ok(())
}
