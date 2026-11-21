//! Compiler protocol only; runtime fault and GC ownership checks belong to VM consumers.
use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, Method, MethodAccessFlags};
use ristretto_jit::{
    AllocationKind, AllocationRequest, Compiler, ExecutionStep, FieldAccess, FieldRequest,
    Function, Result, Value,
};
use std::num::NonZeroU32;

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}
fn pointer() -> *const u8 {
    std::ptr::from_ref(&0u8)
}
fn fixture(
    descriptor: &str,
    field: &str,
    mut code: Vec<Instruction>,
) -> Result<(Function, u16, u16)> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("HeapRequests")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let target = pool.add_class("Node")?;
    let member = pool.add_field_ref(target, "value", field)?;
    for instruction in &mut code {
        *instruction = match instruction {
            Instruction::New(0) => Instruction::New(target),
            Instruction::Getfield(0) => Instruction::Getfield(member),
            Instruction::Putfield(0) => Instruction::Putfield(member),
            _ => instruction.clone(),
        };
    }
    let mut method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: pool.add_utf8("run")?,
        descriptor_index: pool.add_utf8(descriptor)?,
        attributes: vec![],
    };
    method.attributes.push(Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: code.max_stack(&pool)?,
        max_locals: code.max_locals(&pool, &method)?,
        code,
        exception_table: vec![],
        attributes: vec![],
    });
    let class = ClassFile {
        version: ristretto_classfile::JAVA_5,
        constant_pool: pool,
        this_class,
        super_class,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        methods: vec![method],
        ..Default::default()
    };
    class.verify()?;
    let function = Compiler::new()?.compile_requests(
        &class,
        &class.methods[0],
        &[("jit_publish_frame_roots", publish as *const u8)],
    )?;
    Ok((function, target, member))
}

#[test]
fn new_and_instance_fields_suspend_at_exact_byte_offsets() -> Result<()> {
    let (function, target, member) = fixture(
        "()I",
        "I",
        vec![
            Instruction::New(0),
            Instruction::Dup,
            Instruction::Bipush(31),
            Instruction::Putfield(0),
            Instruction::Getfield(0),
            Instruction::Ireturn,
        ],
    )?;
    let mut activation = function.activation(&[])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Allocation(AllocationRequest {
            kind: AllocationKind::Object(target),
            bci: 0
        })
    );
    assert_eq!(activation.call_arguments()?, []);
    activation.complete_call(41)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::PutField,
            index: member,
            bci: 6
        })
    );
    assert_eq!(activation.call_arguments()?, [41, 31]);
    activation.complete_call(0)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::GetField,
            index: member,
            bci: 9
        })
    );
    assert_eq!(activation.call_arguments()?, [41]);
    activation.complete_call(31)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Returned(Some(Value::I32(31)))
    );
    Ok(())
}

#[test]
fn field_values_keep_primitive_bits_and_reference_tokens() -> Result<()> {
    for (field, value, load, ret, bits) in [
        (
            "I",
            Value::I32(-17),
            Instruction::Iload_1,
            Instruction::Ireturn,
            -17,
        ),
        (
            "J",
            Value::I64(i64::MIN),
            Instruction::Lload_1,
            Instruction::Lreturn,
            i64::MIN,
        ),
        (
            "F",
            Value::F32(-0.0),
            Instruction::Fload_1,
            Instruction::Freturn,
            i64::from((-0.0f32).to_bits()),
        ),
        (
            "D",
            Value::F64(f64::from_bits(0x7ff8_1234_5678_abcd)),
            Instruction::Dload_1,
            Instruction::Dreturn,
            0x7ff8_1234_5678_abcd,
        ),
        (
            "Ljava/lang/Object;",
            Value::Ptr(83),
            Instruction::Aload_1,
            Instruction::Areturn,
            83,
        ),
    ] {
        let descriptor = format!("(Ljava/lang/Object;{field}){field}");
        let (function, _, member) = fixture(
            &descriptor,
            field,
            vec![
                Instruction::Aload_0,
                load,
                Instruction::Putfield(0),
                Instruction::Aload_0,
                Instruction::Getfield(0),
                ret,
            ],
        )?;
        let mut activation = function.activation(&[Value::Ptr(71), value.clone()])?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Field(FieldRequest {
                kind: FieldAccess::PutField,
                index: member,
                bci: 2
            })
        );
        assert_eq!(activation.call_arguments()?, [71, bits]);
        activation.complete_call(0)?;
        assert!(matches!(
            activation.resume_with_context(NonZeroU32::MIN, pointer())?,
            ExecutionStep::Field(_)
        ));
        activation.complete_call(bits)?;
        let ExecutionStep::Returned(Some(result)) =
            activation.resume_with_context(NonZeroU32::MIN, pointer())?
        else {
            panic!("field result");
        };
        match result {
            Value::F32(result) => assert_eq!(i64::from(result.to_bits()), bits),
            Value::F64(result) => {
                assert_eq!(i64::from_ne_bytes(result.to_bits().to_ne_bytes()), bits);
            }
            result => assert_eq!(result, value),
        }
    }
    Ok(())
}

#[test]
fn null_receiver_and_pending_allocation_remain_host_requests() -> Result<()> {
    let (function, _, member) = fixture(
        "()I",
        "I",
        vec![
            Instruction::Aconst_null,
            Instruction::Getfield(0),
            Instruction::Ireturn,
        ],
    )?;
    let mut activation = function.activation(&[])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Field(FieldRequest {
            kind: FieldAccess::GetField,
            index: member,
            bci: 1
        })
    );
    assert_eq!(activation.call_arguments()?, [0]);
    drop(activation); // host reports the null fault without ever resuming a placeholder
    let (function, _, _) = fixture(
        "()Ljava/lang/Object;",
        "I",
        vec![Instruction::New(0), Instruction::Areturn],
    )?;
    let mut activation = function.activation(&[])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, pointer())?,
        ExecutionStep::Allocation(_)
    ));
    drop(activation); // cancellation does not invoke a legacy allocator or replay New
    Ok(())
}
