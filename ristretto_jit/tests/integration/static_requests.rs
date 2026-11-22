//! Static requests are parked while the owning host drives class initialization.
#![allow(unsafe_code)]
#![expect(
    clippy::cast_ptr_alignment,
    reason = "test callbacks receive live aligned Context"
)]
use ristretto_classfile::attributes::{Attribute, Instruction, MaxStack};
use ristretto_classfile::{ClassFile, Field, FieldAccessFlags};
use ristretto_jit::{Compiler, ExecutionStep, FieldAccess, FieldRequest, Function, Result, Value};
use std::cell::Cell;
use std::num::NonZeroU32;

const ROOT_CAPACITY: usize = 8;

struct Context {
    roots: Cell<[i64; ROOT_CAPACITY]>,
    count: Cell<usize>,
}
impl Context {
    fn new() -> Self {
        Self {
            roots: Cell::new([0; ROOT_CAPACITY]),
            count: Cell::new(0),
        }
    }

    fn pointer(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
}

extern "C" fn publish(context: *const u8, roots: *const i64, count: i32) -> i32 {
    let context = unsafe { &*context.cast::<Context>() };
    let Ok(count) = usize::try_from(count) else {
        return 0;
    };
    if count > ROOT_CAPACITY {
        return 0;
    }
    let values = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(roots, count) }
    };
    let mut copy = [0; ROOT_CAPACITY];
    copy[..count].copy_from_slice(values);
    context.roots.set(copy);
    context.count.set(count);
    1
}

fn fixture(
    descriptor: &str,
    field: &str,
    code: impl FnOnce(u16) -> Vec<Instruction>,
    initializer: bool,
) -> Result<(Function, u16)> {
    let mut class = super::continuation::fixture("()V", &[Instruction::Return])?;
    let name = class.constant_pool.add_utf8("value")?;
    let field_type = class.constant_pool.add_utf8(field)?;
    class.fields.push(Field {
        access_flags: FieldAccessFlags::STATIC | FieldAccessFlags::PUBLIC,
        name_index: name,
        descriptor_index: field_type,
        field_type: ristretto_classfile::FieldType::parse(field)?,
        attributes: vec![],
    });
    let index = class
        .constant_pool
        .add_field_ref(class.this_class, "value", field)?;
    class.methods[0].descriptor_index = class.constant_pool.add_utf8(descriptor)?;
    if initializer {
        class.methods[0].name_index = class.constant_pool.add_utf8("<clinit>")?;
    }
    let code = code(index);
    let stack = code.max_stack(&class.constant_pool)?;
    if let Attribute::Code {
        code: body,
        max_stack,
        max_locals,
        ..
    } = &mut class.methods[0].attributes[0]
    {
        *body = code;
        *max_stack = stack;
        *max_locals = 2;
    }
    class.verify()?;
    compile(&class, index)
}

fn compile(class: &ClassFile, index: u16) -> Result<(Function, u16)> {
    Ok((
        Compiler::new()?.compile_requests(
            class,
            &class.methods[0],
            &[("jit_publish_frame_roots", publish as *const u8)],
        )?,
        index,
    ))
}

#[test]
fn static_read_and_write_suspend_without_a_receiver() -> Result<()> {
    let (function, index) = fixture(
        "(I)I",
        "I",
        |index| {
            vec![
                Instruction::Iload_0,
                Instruction::Putstatic(index),
                Instruction::Getstatic(index),
                Instruction::Ireturn,
            ]
        },
        false,
    )?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::I32(-37)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::PutStatic,
            index,
            bci: 1
        })
    );
    assert_eq!(activation.call_arguments()?, [-37]);
    activation.complete_call(0)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::GetStatic,
            index,
            bci: 4
        })
    );
    assert_eq!(activation.call_arguments()?, []);
    activation.complete_call(-37)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(Some(Value::I32(-37)))
    );
    Ok(())
}

#[test]
fn putstatic_reference_remains_published_during_initializer_wait() -> Result<()> {
    let (function, index) = fixture(
        "(Ljava/lang/Object;)V",
        "Ljava/lang/Object;",
        |index| {
            vec![
                Instruction::Aload_0,
                Instruction::Putstatic(index),
                Instruction::Return,
            ]
        },
        false,
    )?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::Ptr(71)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::PutStatic,
            index,
            bci: 1
        })
    );
    assert_eq!(activation.call_arguments()?, [71]);
    assert!(context.roots.get()[..context.count.get()].contains(&71));
    assert!(
        activation
            .resume_with_context(NonZeroU32::MIN, context.pointer())
            .is_err()
    );
    assert!(context.roots.get()[..context.count.get()].contains(&71));
    drop(activation); // failed initialization cancels; no placeholder/static pointer is returned
    Ok(())
}

#[test]
fn class_initializer_bytecode_uses_the_same_explicit_static_request_protocol() -> Result<()> {
    let (function, index) = fixture(
        "()V",
        "I",
        |index| {
            vec![
                Instruction::Iconst_1,
                Instruction::Putstatic(index),
                Instruction::Return,
            ]
        },
        true,
    )?;
    let context = Context::new();
    let mut activation = function.activation(&[])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::PutStatic,
            index,
            bci: 1
        })
    );
    assert_eq!(activation.call_arguments()?, [1]);
    activation.complete_call(0)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    Ok(())
}
