#![expect(
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    reason = "test fixtures and assertions validate known call shapes"
)]

//! Exercise cold resolution and synchronous dispatch through the instruction dispatcher.

use super::*;
use crate::Error::JavaError;
use crate::JavaError::{
    AbstractMethodError, IllegalAccessError, IncompatibleClassChangeError, NullPointerException,
};
use crate::instruction::resolve_method_ref;
use crate::method_ref_cache::InvokeKind;
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, FieldAccessFlags, MethodAccessFlags,
};
use ristretto_classloader::Object;

const DESCRIPTOR: &str = "(JD)J";

fn target(name: &str, flags: Option<MethodAccessFlags>, interface: bool) -> Result<Arc<Class>> {
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class(name)?;
    let name_index = constant_pool.add_utf8("m")?;
    let descriptor_index = constant_pool.add_utf8(DESCRIPTOR)?;
    Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            access_flags: if interface {
                ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
            } else {
                ClassAccessFlags::PUBLIC
            },
            methods: flags
                .map(|access_flags| ristretto_classfile::Method {
                    access_flags,
                    name_index,
                    descriptor_index,
                    ..Default::default()
                })
                .into_iter()
                .collect(),
            ..Default::default()
        },
    )
    .map_err(Into::into)
}

fn caller(thread: &Arc<Thread>, target: &Arc<Class>) -> Result<(Frame, u16)> {
    let mut constant_pool = ConstantPool::default();
    // Deliberately use the same caller name and CP layout for distinct class identities.
    let this_class = constant_pool.add_class("SameCallerName")?;
    let target_index = constant_pool.add_class(target.name())?;
    let index = if target.is_interface() {
        constant_pool.add_interface_method_ref(target_index, "m", DESCRIPTOR)?
    } else {
        constant_pool.add_method_ref(target_index, "m", DESCRIPTOR)?
    };
    let name_index = constant_pool.add_utf8("caller")?;
    let descriptor_index = constant_pool.add_utf8("()V")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool,
            this_class,
            methods: vec![ristretto_classfile::Method {
                name_index,
                descriptor_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )?;
    let method = class.try_get_method("caller", "()V")?;
    Ok((Frame::new(&Arc::downgrade(thread), &class, &method), index))
}

fn arguments(vm: &crate::VM, receiver: Option<&Arc<Class>>) -> Result<OperandStack> {
    let mut stack = OperandStack::with_max_size(4);
    if let Some(receiver) = receiver {
        stack.push(Value::from_object(
            vm.garbage_collector(),
            Object::new(receiver.clone())?,
        ))?;
    }
    stack.push_long(17)?;
    stack.push_double(2.0)?;
    Ok(stack)
}

async fn dispatch(
    frame: &Frame,
    stack: &mut OperandStack,
    instruction: Instruction,
    sync: bool,
) -> Result<MethodCall> {
    let mut locals = LocalVariables::new(Vec::new());
    let result = frame.process(&mut locals, stack, &instruction)?;
    let result = match result {
        InstructionResult::Sync(result) => {
            assert!(sync, "unexpected cache hit");
            result
        }
        InstructionResult::Async(instruction) => {
            assert!(!sync, "resolved invocation must finish synchronously");
            frame.process_async(stack, &instruction).await?
        }
    };
    let ExecutionResult::Call(call) = result else {
        panic!("expected method call");
    };
    assert!(stack.is_empty());
    assert!(call.has_return_type);
    assert_eq!(
        call.parameters[call.parameters.len() - 2..],
        [Value::Long(17), Value::Double(2.0)]
    );
    Ok(call)
}

#[tokio::test]
async fn caller_identity_shared_records_and_invocation_kind() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let a = target(
        "StaticA",
        Some(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
        false,
    )?;
    let b = target(
        "StaticB",
        Some(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
        false,
    )?;
    thread.register_class(a.clone()).await?;
    thread.register_class(b.clone()).await?;
    let (first, index) = caller(&thread, &a)?;
    let (second, other_index) = caller(&thread, &b)?;
    assert_eq!(index, other_index);
    for (frame, expected) in [(&first, &a), (&second, &b)] {
        for sync in [false, true] {
            let call = dispatch(
                frame,
                &mut arguments(&vm, None)?,
                Instruction::Invokestatic(index),
                sync,
            )
            .await?;
            assert!(Arc::ptr_eq(&call.class, expected));
            assert_eq!(call.parameters.len(), 2);
        }
        // Static resolution cannot validate the same CP index for non-static bytecodes.
        for kind in [
            InvokeKind::Virtual,
            InvokeKind::Special,
            InvokeKind::Interface,
        ] {
            assert!(matches!(
                resolve_method_ref(frame, index, kind).await,
                Err(JavaError(IncompatibleClassChangeError(_)))
            ));
        }
    }
    let another_frame = Frame::new(&Arc::downgrade(&thread), first.class(), first.method());
    let one = resolve_method_ref(&first, index, InvokeKind::Static).await?;
    let two = resolve_method_ref(&another_frame, index, InvokeKind::Static).await?;
    assert!(Arc::ptr_eq(&one, &two));
    Ok(())
}

#[tokio::test]
async fn receiver_identity_polymorphism_and_overflow() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let base = target("VirtualBase", Some(MethodAccessFlags::PUBLIC), false)?;
    thread.register_class(base.clone()).await?;
    let (frame, index) = caller(&thread, &base)?;
    let mut receivers = Vec::new();
    for _ in 0..5 {
        // These classes are deliberately unregistered and have identical names. Dispatch must
        // use the object's class, and may never reload it or accept a name-only guard.
        let receiver = target("SameReceiverName", Some(MethodAccessFlags::PUBLIC), false)?;
        receiver.set_parent(Some(base.clone()))?;
        let call = dispatch(
            &frame,
            &mut arguments(&vm, Some(&receiver))?,
            Instruction::Invokevirtual(index),
            false,
        )
        .await?;
        assert!(Arc::ptr_eq(&call.class, &receiver));
        receivers.push(receiver);
    }
    for (slot, receiver) in receivers.iter().enumerate().rev() {
        let call = dispatch(
            &frame,
            &mut arguments(&vm, Some(receiver))?,
            Instruction::Invokevirtual(index),
            slot < 4,
        )
        .await?;
        assert!(Arc::ptr_eq(&call.class, receiver));
    }
    // The same CP entry can also be used by invokespecial, which must select the base
    // implementation rather than borrowing the invokevirtual receiver target.
    for sync in [false, true] {
        let call = dispatch(
            &frame,
            &mut arguments(&vm, Some(&receivers[0]))?,
            Instruction::Invokespecial(index),
            sync,
        )
        .await?;
        assert!(Arc::ptr_eq(&call.class, &base));
    }
    assert!(matches!(
        resolve_method_ref(&frame, index, InvokeKind::Static).await,
        Err(JavaError(IncompatibleClassChangeError(_)))
    ));

    let mut stack = OperandStack::with_max_size(3);
    stack.push_object(None)?;
    stack.push_long(17)?;
    stack.push_double(2.0)?;
    assert!(matches!(
        frame.process(
            &mut LocalVariables::new(vec![]),
            &mut stack,
            &Instruction::Invokevirtual(index)
        ),
        Err(JavaError(NullPointerException(_)))
    ));
    Ok(())
}

#[tokio::test]
async fn interface_default_override_and_rejected_targets() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let interface = target("DefaultInterface", Some(MethodAccessFlags::PUBLIC), true)?;
    thread.register_class(interface.clone()).await?;
    let child_interface = target("ChildInterface", None, true)?;
    child_interface.set_interfaces(vec![interface.clone()])?;
    thread.register_class(child_interface.clone()).await?;
    let (frame, index) = caller(&thread, &child_interface)?;
    let defaults = target("DefaultReceiver", None, false)?;
    defaults.set_interfaces(vec![child_interface.clone()])?;
    let overrides = target("OverrideReceiver", Some(MethodAccessFlags::PUBLIC), false)?;
    overrides.set_interfaces(vec![child_interface.clone()])?;
    for (receiver, expected) in [(&defaults, &interface), (&overrides, &overrides)] {
        for sync in [false, true] {
            let call = dispatch(
                &frame,
                &mut arguments(&vm, Some(receiver))?,
                Instruction::Invokeinterface(index, 5),
                sync,
            )
            .await?;
            assert!(Arc::ptr_eq(&call.class, expected));
            assert_eq!(call.parameters.len(), 3);
        }
    }
    let parent_only = target("ParentOnly", None, false)?;
    parent_only.set_interfaces(vec![interface])?;
    let bad_targets = [
        (parent_only, 0),
        (
            target(
                "AbstractReceiver",
                Some(MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                false,
            )?,
            1,
        ),
        (
            target("PrivateReceiver", Some(MethodAccessFlags::PRIVATE), false)?,
            2,
        ),
        (
            target(
                "StaticReceiver",
                Some(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                false,
            )?,
            0,
        ),
    ];
    for (receiver, error_kind) in bad_targets {
        if error_kind != 0 || receiver.name() == "StaticReceiver" {
            receiver.set_interfaces(vec![child_interface.clone()])?;
        }
        for _ in 0..2 {
            let mut stack = arguments(&vm, Some(&receiver))?;
            let result = frame
                .process_async(&mut stack, &Instruction::Invokeinterface(index, 5))
                .await;
            match error_kind {
                0 => assert!(matches!(
                    result,
                    Err(JavaError(IncompatibleClassChangeError(_)))
                )),
                1 => assert!(matches!(result, Err(JavaError(AbstractMethodError(_))))),
                _ => assert!(matches!(result, Err(JavaError(IllegalAccessError(_))))),
            }
            let resolution = resolve_method_ref(&frame, index, InvokeKind::Interface).await?;
            assert!(resolution.dispatch.get(&receiver).is_none());
        }
    }
    Ok(())
}

#[tokio::test]
async fn cached_static_method_waits_for_initialization_and_preserves_failure() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let class = target(
        "InitializingTarget",
        Some(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
        false,
    )?;
    thread.register_class(class.clone()).await?;
    class.begin_initialization(thread.id())?;
    let (frame, index) = caller(&thread, &class)?;
    dispatch(
        &frame,
        &mut arguments(&vm, None)?,
        Instruction::Invokestatic(index),
        false,
    )
    .await?;
    assert!(!class.is_initialized()?);
    let other_thread = Thread::new(&Arc::downgrade(&vm), thread.id() + 1);
    let other_frame = Frame::new(
        &Arc::downgrade(&other_thread),
        frame.class(),
        frame.method(),
    );
    let mut stack = arguments(&vm, None)?;
    assert!(matches!(
        other_frame.process(
            &mut LocalVariables::new(vec![]),
            &mut stack,
            &Instruction::Invokestatic(index)
        )?,
        InstructionResult::Async(_)
    ));
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(20),
            other_frame.process_async(&mut stack, &Instruction::Invokestatic(index))
        )
        .await
        .is_err()
    );
    assert_eq!(stack.len(), 2);
    class.fail_initialization("test initialization failed".to_string())?;
    assert!(matches!(
        other_frame
            .process_async(&mut stack, &Instruction::Invokestatic(index))
            .await,
        Err(JavaError(crate::JavaError::NoClassDefFoundError(_)))
    ));
    Ok(())
}

#[tokio::test]
async fn fields_have_sync_hits_and_check_static_kind() -> Result<()> {
    let (vm, thread, mut frame) = crate::test::frame().await?;
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("FieldTarget")?;
    let name_index = pool.add_utf8("value")?;
    let descriptor_index = pool.add_utf8("I")?;
    let static_name_index = pool.add_utf8("staticValue")?;
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
            this_class,
            fields: vec![
                ristretto_classfile::Field {
                    access_flags: FieldAccessFlags::PUBLIC,
                    name_index,
                    descriptor_index,
                    field_type: ristretto_classfile::FieldType::Base(
                        ristretto_classfile::BaseType::Int,
                    ),
                    attributes: Vec::new(),
                },
                ristretto_classfile::Field {
                    access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
                    name_index: static_name_index,
                    descriptor_index,
                    field_type: ristretto_classfile::FieldType::Base(
                        ristretto_classfile::BaseType::Int,
                    ),
                    attributes: Vec::new(),
                },
            ],
            ..Default::default()
        },
    )?;
    thread.register_class(class.clone()).await?;
    let pool = Arc::get_mut(frame.class_mut())
        .expect("unique caller")
        .constant_pool_mut();
    let class_index = pool.add_class(class.name())?;
    let field_index = pool.add_field_ref(class_index, "value", "I")?;
    let static_index = pool.add_field_ref(class_index, "staticValue", "I")?;
    let object = Value::from_object(vm.garbage_collector(), Object::new(class.clone())?);
    let mut stack = OperandStack::with_max_size(2);
    let mut locals = LocalVariables::new(vec![]);
    for (index, is_static) in [(field_index, false), (static_index, true)] {
        let put = if is_static {
            Instruction::Putstatic(index)
        } else {
            Instruction::Putfield(index)
        };
        let get = if is_static {
            Instruction::Getstatic(index)
        } else {
            Instruction::Getfield(index)
        };
        for sync in [false, true] {
            if !is_static {
                stack.push(object.clone())?;
            }
            stack.push_int(42)?;
            match frame.process(&mut locals, &mut stack, &put)? {
                InstructionResult::Async(instruction) => {
                    assert!(!sync);
                    frame.process_async(&mut stack, &instruction).await?;
                }
                InstructionResult::Sync(_) => assert!(sync),
            }
            if !is_static {
                stack.push(object.clone())?;
            }
            assert!(matches!(
                frame.process(&mut locals, &mut stack, &get)?,
                InstructionResult::Sync(ExecutionResult::Continue)
            ));
            assert_eq!(stack.pop_int()?, 42);
        }
        let wrong_kind = if is_static {
            Instruction::Getfield(index)
        } else {
            Instruction::Getstatic(index)
        };
        assert!(matches!(
            frame.process(&mut locals, &mut stack, &wrong_kind),
            Err(JavaError(IncompatibleClassChangeError(_)))
        ));
        if !is_static {
            assert!(
                !class.is_initialized()?,
                "instance field resolution must not initialize the class"
            );
        }
    }
    Ok(())
}
