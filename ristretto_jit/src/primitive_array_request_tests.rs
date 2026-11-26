//! Primitive-array protocol checks with no synchronous heap helper symbols.
use super::*;
use crate::{AllocationKind, ArrayAccessKind};
use ristretto_classfile::attributes::{ArrayType, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ConstantPool, Method, MethodAccessFlags};

pub(super) fn fixture(descriptor: &str, code: Vec<Instruction>) -> Result<ClassFile<'static>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("PrimitiveArrayRequestFixture")?;
    let super_class = pool.add_class("java/lang/Object")?;
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
    Ok(class)
}

struct Case {
    kind: ArrayType,
    descriptor: &'static str,
    array_local: u8,
    value_load: Instruction,
    store: Instruction,
    load: Instruction,
    ret: Instruction,
    store_kind: ArrayAccessKind,
    load_kind: ArrayAccessKind,
    value: Value,
    store_bits: i64,
    result_bits: i64,
    expected: Value,
}

impl Case {
    fn integer(kind: ArrayType, value: i32, expected: i32) -> Self {
        let (store, load, store_kind, load_kind) = match kind {
            ArrayType::Boolean | ArrayType::Byte => (
                Instruction::Bastore,
                Instruction::Baload,
                ArrayAccessKind::ByteStore,
                ArrayAccessKind::ByteLoad,
            ),
            ArrayType::Char => (
                Instruction::Castore,
                Instruction::Caload,
                ArrayAccessKind::CharStore,
                ArrayAccessKind::CharLoad,
            ),
            ArrayType::Short => (
                Instruction::Sastore,
                Instruction::Saload,
                ArrayAccessKind::ShortStore,
                ArrayAccessKind::ShortLoad,
            ),
            _ => (
                Instruction::Iastore,
                Instruction::Iaload,
                ArrayAccessKind::IntStore,
                ArrayAccessKind::IntLoad,
            ),
        };
        Self {
            kind,
            descriptor: "(II)I",
            array_local: 2,
            value_load: Instruction::Iload_1,
            store,
            load,
            ret: Instruction::Ireturn,
            store_kind,
            load_kind,
            value: Value::I32(value),
            store_bits: i64::from(value),
            result_bits: i64::from(expected),
            expected: Value::I32(expected),
        }
    }
    fn long(value: i64) -> Self {
        Self {
            kind: ArrayType::Long,
            descriptor: "(IJ)J",
            array_local: 3,
            value_load: Instruction::Lload_1,
            store: Instruction::Lastore,
            load: Instruction::Laload,
            ret: Instruction::Lreturn,
            store_kind: ArrayAccessKind::LongStore,
            load_kind: ArrayAccessKind::LongLoad,
            value: Value::I64(value),
            store_bits: value,
            result_bits: value,
            expected: Value::I64(value),
        }
    }
    fn float(bits: u32) -> Self {
        let value = Value::F32(f32::from_bits(bits));
        Self {
            kind: ArrayType::Float,
            descriptor: "(IF)F",
            array_local: 2,
            value_load: Instruction::Fload_1,
            store: Instruction::Fastore,
            load: Instruction::Faload,
            ret: Instruction::Freturn,
            store_kind: ArrayAccessKind::FloatStore,
            load_kind: ArrayAccessKind::FloatLoad,
            value: value.clone(),
            store_bits: i64::from(bits),
            result_bits: i64::from(bits),
            expected: value,
        }
    }
    fn double(bits: u64) -> Self {
        let value = Value::F64(f64::from_bits(bits));
        let bits = i64::from_ne_bytes(bits.to_ne_bytes());
        Self {
            kind: ArrayType::Double,
            descriptor: "(ID)D",
            array_local: 3,
            value_load: Instruction::Dload_1,
            store: Instruction::Dastore,
            load: Instruction::Daload,
            ret: Instruction::Dreturn,
            store_kind: ArrayAccessKind::DoubleStore,
            load_kind: ArrayAccessKind::DoubleLoad,
            value: value.clone(),
            store_bits: bits,
            result_bits: bits,
            expected: value,
        }
    }
}

fn roundtrip(case: Case) -> Result<()> {
    let class = fixture(
        case.descriptor,
        vec![
            Instruction::Iload_0,
            Instruction::Newarray(case.kind.clone()),
            Instruction::Astore(case.array_local),
            Instruction::Aload(case.array_local),
            Instruction::Iconst_0,
            case.value_load,
            case.store,
            Instruction::Aload(case.array_local),
            Instruction::Iconst_0,
            case.load,
            case.ret,
        ],
    )?;
    let method = class
        .methods
        .first()
        .ok_or_else(|| InternalError("fixture method".to_owned()))?;
    assert!(!Compiler::can_compile(method));
    assert_eq!(
        Compiler::experimental_rejection_reason(&class, method),
        Some("experimental_unqualified_instruction")
    );
    assert_eq!(Compiler::request_rejection_reason(&class, method), None);
    let function = compile(&class, "run")?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::I32(1), case.value])?;
    let ExecutionStep::Allocation(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?
    else {
        return Err(InternalError(
            "missing primitive allocation request".to_owned(),
        ));
    };
    assert_eq!(request.kind, AllocationKind::PrimitiveArray(case.kind));
    assert_eq!(request.bci, 1);
    assert_eq!(activation.call_arguments()?, &[1]);
    activation.complete_call(7)?;
    for (kind, expected_arguments, result) in [
        (case.store_kind, vec![7, 0, case.store_bits], 0),
        (case.load_kind, vec![7, 0], case.result_bits),
    ] {
        let ExecutionStep::ArrayAccess(request) =
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?
        else {
            return Err(InternalError("missing typed array request".to_owned()));
        };
        assert_eq!(request.kind, kind);
        assert_eq!(activation.call_arguments()?, expected_arguments);
        assert!(
            context
                .roots
                .lock()
                .map_err(|error| InternalError(error.to_string()))?
                .contains(&7)
        );
        activation.complete_call(result)?;
    }
    let result = activation.resume_with_context(NonZeroU32::MIN, context.pointer())?;
    match (result, case.expected) {
        (ExecutionStep::Returned(Some(Value::F32(actual))), Value::F32(expected)) => {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        (ExecutionStep::Returned(Some(Value::F64(actual))), Value::F64(expected)) => {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        (actual, expected) => assert_eq!(actual, ExecutionStep::Returned(Some(expected))),
    }
    Ok(())
}

#[test]
fn all_primitive_kinds_preserve_typed_request_bits_without_legacy_helpers() -> Result<()> {
    // Narrowing/boolean semantics belong to the payload runtime; the compiler
    // preserves each full input and faithfully reconstructs the runtime result.
    for case in [
        Case::integer(ArrayType::Boolean, 2, 0),
        Case::integer(ArrayType::Boolean, 3, 1),
        Case::integer(ArrayType::Byte, 511, -1),
        Case::integer(ArrayType::Char, -1, 65535),
        Case::integer(ArrayType::Short, 65535, -1),
        Case::integer(ArrayType::Int, i32::MIN, i32::MIN),
        Case::long(i64::MIN),
        Case::long(i64::MAX),
        Case::float(0x8000_0000),
        Case::float(0x7fc0_1234),
        Case::double(0x8000_0000_0000_0000),
        Case::double(0x7ff8_0000_0000_0042),
    ] {
        roundtrip(case)?;
    }
    Ok(())
}

#[test]
fn primitive_array_length_is_an_owning_request_for_every_kind() -> Result<()> {
    for kind in [
        ArrayType::Boolean,
        ArrayType::Byte,
        ArrayType::Char,
        ArrayType::Short,
        ArrayType::Int,
        ArrayType::Long,
        ArrayType::Float,
        ArrayType::Double,
    ] {
        let class = fixture(
            "(I)I",
            vec![
                Instruction::Iload_0,
                Instruction::Newarray(kind.clone()),
                Instruction::Arraylength,
                Instruction::Ireturn,
            ],
        )?;
        let function = compile(&class, "run")?;
        let context = Context::new();
        let mut activation = function.activation(&[Value::I32(0)])?;
        assert!(
            matches!(activation.resume_with_context(NonZeroU32::MIN, context.pointer())?, ExecutionStep::Allocation(request) if request.kind == AllocationKind::PrimitiveArray(kind))
        );
        assert_eq!(activation.call_arguments()?, &[0]);
        activation.complete_call(7)?;
        assert!(
            matches!(activation.resume_with_context(NonZeroU32::MIN, context.pointer())?, ExecutionStep::ArrayAccess(request) if request.kind == ArrayAccessKind::Length && request.bci == 3)
        );
        assert_eq!(activation.call_arguments()?, &[7]);
        activation.complete_call(0)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(Some(Value::I32(0)))
        );
    }
    Ok(())
}

#[test]
fn null_bounds_and_type_errors_return_before_later_array_requests() -> Result<()> {
    let class = fixture(
        "([II)I",
        vec![
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Iaload,
            Instruction::Pop,
            Instruction::Aload_0,
            Instruction::Arraylength,
            Instruction::Ireturn,
        ],
    )?;
    // Tokens are opaque: the runtime validates null/bounds/payload type and
    // records the error. No native array dereference may precede that decision.
    for (token, index) in [(0, 0), (7, -1), (7, i32::MAX), (8, 0)] {
        let function = compile(&class, "run")?;
        let context = Context::new();
        let mut activation = function.activation(&[Value::Ptr(token), Value::I32(index)])?;
        assert!(
            matches!(activation.resume_with_context(NonZeroU32::MIN, context.pointer())?, ExecutionStep::ArrayAccess(request) if request.kind == ArrayAccessKind::IntLoad && request.bci == 2)
        );
        assert_eq!(activation.call_arguments()?, &[token, i64::from(index)]);
        context.failed.store(true, Ordering::Relaxed);
        activation.complete_call(0)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(None)
        );
    }
    Ok(())
}

#[test]
fn allocation_errors_and_invalid_result_tokens_never_reach_array_length() -> Result<()> {
    let class = fixture(
        "(I)I",
        vec![
            Instruction::Iload_0,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Arraylength,
            Instruction::Ireturn,
        ],
    )?;
    for (length, rejected_token) in [(-1, 0), (1, 7)] {
        let function = compile(&class, "run")?;
        let context = Context::new();
        let mut activation = function.activation(&[Value::I32(length)])?;
        assert!(matches!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Allocation(_)
        ));
        assert_eq!(activation.call_arguments()?, &[i64::from(length)]);
        if rejected_token == 0 {
            context.failed.store(true, Ordering::Relaxed);
        } else {
            context
                .rejected_token
                .store(rejected_token, Ordering::Relaxed);
        }
        activation.complete_call(rejected_token)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(None)
        );
        assert!(context.failed.load(Ordering::Relaxed));
    }
    Ok(())
}

#[test]
fn multidimensional_array_bytecode_remains_rejected() -> Result<()> {
    // Eligibility rejects before constant-pool or operand-stack lowering.
    let mut class = fixture("()I", vec![Instruction::Iconst_0, Instruction::Ireturn])?;
    let method = class
        .methods
        .first_mut()
        .ok_or_else(|| InternalError("fixture method".to_owned()))?;
    for attribute in &mut method.attributes {
        if let Attribute::Code { code, .. } = attribute {
            code.insert(0, Instruction::Multianewarray(1, 2));
        }
    }
    let method = class
        .methods
        .first()
        .ok_or_else(|| InternalError("fixture method".to_owned()))?;
    assert_eq!(
        Compiler::request_rejection_reason(&class, method),
        Some("experimental_unqualified_instruction")
    );
    assert!(
        Compiler::new()?
            .compile_requests(&class, method, &[])
            .is_err()
    );
    Ok(())
}
