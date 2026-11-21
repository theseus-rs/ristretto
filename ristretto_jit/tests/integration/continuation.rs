//! Native scalar suspension without any runtime-helper symbol registrations.
use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, Method, MethodAccessFlags};
use ristretto_jit::{Compiler, Error, ExecutionStep, Function, Result, Value};
use std::num::NonZeroU32;

pub(super) fn fixture(descriptor: &str, code: &[Instruction]) -> Result<ClassFile<'static>> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("ScalarContinuation")?;
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
        code: code.to_vec(),
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

fn compile(class: &ClassFile) -> Result<Function> {
    Compiler::new()?.compile_requests(class, &class.methods[0], &[])
}

fn sum() -> Result<ClassFile<'static>> {
    fixture(
        "(I)I",
        &[
            Instruction::Iconst_0,
            Instruction::Istore_1, // 0, 1
            Instruction::Iload_0,
            Instruction::Ifle(13), // 2, 3 -> 16
            Instruction::Iload_1,
            Instruction::Iload_0,
            Instruction::Iadd,
            Instruction::Istore_1, // 6..9
            Instruction::Iinc(0, -1),
            Instruction::Goto(-11), // 10, 13 -> 2
            Instruction::Iload_1,
            Instruction::Ireturn, // 16, 17
        ],
    )
}

#[test]
fn scalar_yields_and_resumes_without_replaying_bytecodes() -> Result<()> {
    let class = sum()?;
    assert!(!Compiler::can_compile(&class.methods[0]));
    let function = compile(&class)?;
    assert!(function.requests_only());
    assert!(function.supports_cooperation());
    assert!(
        function
            .execute(&[Value::I32(7)], std::ptr::null())
            .is_err()
    );
    let mut activation = function.activation(&[Value::I32(7)])?;
    assert_eq!(activation.suspended_bci(), None);
    let budget = NonZeroU32::new(3).expect("positive budget");
    for _ in 0..2 {
        assert_eq!(activation.resume(budget)?, ExecutionStep::Yielded);
        assert_eq!(activation.suspended_bci(), Some(2));
    }
    assert_eq!(
        activation.resume(budget)?,
        ExecutionStep::Returned(Some(Value::I32(28)))
    );
    assert_eq!(activation.suspended_bci(), None);
    assert!(activation.resume(budget).is_err());
    Ok(())
}

#[test]
fn operands_survive_a_backward_edge() -> Result<()> {
    let class = fixture(
        "(I)I",
        &[
            Instruction::Iconst_0,
            Instruction::Iload_0,
            Instruction::Ifle(11), // 0, 1, 2 -> 13
            Instruction::Iload_0,
            Instruction::Iadd,
            Instruction::Iinc(0, -1), // 5, 6, 7
            Instruction::Goto(-9),
            Instruction::Ireturn, // 10 -> 1, 13
        ],
    )?;
    let mut activation = compile(&class)?.activation(&[Value::I32(9)])?;
    for _ in 0..9 {
        assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    }
    assert_eq!(
        activation.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(45)))
    );
    Ok(())
}

#[test]
fn independent_activations_own_code_and_cancel_without_resuming() -> Result<()> {
    let function = compile(&sum()?)?;
    assert!(matches!(
        function.activation(&[]),
        Err(Error::InvalidArgumentCount { .. })
    ));
    assert!(matches!(
        function.activation(&[Value::I64(7)]),
        Err(Error::InvalidArgumentType { .. })
    ));
    let mut first = function.activation(&[Value::I32(1)])?;
    let mut second = function.activation(&[Value::I32(2)])?;
    let cancelled = function.activation(&[Value::I32(100)])?;
    drop(function);
    drop(cancelled);
    assert_eq!(first.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    assert_eq!(second.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    assert_eq!(
        first.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(1)))
    );
    assert_eq!(second.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    assert_eq!(
        second.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(3)))
    );
    let class = fixture("()V", &[Instruction::Goto(0)])?;
    let mut infinite = compile(&class)?.activation(&[])?;
    for _ in 0..3 {
        assert_eq!(infinite.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    }
    assert_eq!(infinite.suspended_bci(), Some(0));
    drop(infinite);
    Ok(())
}

#[test]
fn wide_locals_preserve_double_bits_across_suspension() -> Result<()> {
    let class = fixture(
        "(JDI)D",
        &[
            Instruction::Dload_2,
            Instruction::Dstore(5), // 0, 1
            Instruction::Iload(4),
            Instruction::Ifle(9), // 3, 5 -> 14
            Instruction::Iinc(4, -1),
            Instruction::Goto(-8), // 8, 11 -> 3
            Instruction::Dload(5),
            Instruction::Dreturn, // 14, 16
        ],
    )?;
    let function = compile(&class)?;
    for bits in [0x8000_0000_0000_0000, 0x7ff8_0000_0000_0123] {
        let mut activation = function.activation(&[
            Value::I64(i64::MIN),
            Value::F64(f64::from_bits(bits)),
            Value::I32(2),
        ])?;
        for _ in 0..2 {
            assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
        }
        let ExecutionStep::Returned(Some(Value::F64(result))) =
            activation.resume(NonZeroU32::MIN)?
        else {
            panic!("missing double result");
        };
        assert_eq!(result.to_bits(), bits);
    }
    Ok(())
}

#[test]
fn ordinary_dispatch_and_unsupported_capabilities_stay_separate() -> Result<()> {
    let class = fixture("(I)I", &[Instruction::Iload_0, Instruction::Ireturn])?;
    let compiler = Compiler::new()?;
    assert!(Compiler::can_compile(&class.methods[0]));
    let function = compiler.compile(&class, &class.methods[0], &[])?;
    assert!(!function.supports_cooperation());
    assert!(function.activation(&[Value::I32(9)]).is_err());
    assert_eq!(
        function.execute(&[Value::I32(9)], std::ptr::null())?,
        Some(Value::I32(9))
    );
    let mut rejected = class;
    for instruction in [
        Instruction::Multianewarray(1, 1),
        Instruction::Instanceof(1),
        Instruction::Invokedynamic(1),
        Instruction::Getstatic(1),
        Instruction::Monitorenter,
        Instruction::Idiv,
    ] {
        if let Attribute::Code { code, .. } = &mut rejected.methods[0].attributes[0] {
            *code = vec![instruction];
        }
        assert!(matches!(
            compile(&rejected),
            Err(Error::UnsupportedMethod(_))
        ));
    }
    let reference = fixture(
        "(Ljava/lang/Object;)I",
        &[Instruction::Iconst_0, Instruction::Ireturn],
    )?;
    assert_eq!(
        Compiler::request_rejection_reason(&reference, &reference.methods[0]),
        None
    );
    Ok(())
}

#[test]
fn excessive_state_transfer_is_rejected_before_codegen() -> Result<()> {
    let mut class = sum()?;
    if let Attribute::Code { max_stack, .. } = &mut class.methods[0].attributes[0] {
        *max_stack = u16::MAX;
    }
    assert!(
        matches!(compile(&class), Err(Error::UnsupportedMethod(message)) if message == "continuation state-transfer limit exceeded")
    );
    Ok(())
}

#[test]
fn pre_entry_stack_failure_preserves_the_activation() -> Result<()> {
    let mut activation = compile(&sum()?)?.activation(&[Value::I32(1)])?;
    let rejected = stacker::grow(128 * 1024, || activation.resume(NonZeroU32::MIN));
    assert!(matches!(rejected, Err(Error::NativeStackOverflow { .. })));
    assert_eq!(activation.suspended_bci(), None);
    assert_eq!(activation.resume(NonZeroU32::MIN)?, ExecutionStep::Yielded);
    assert_eq!(
        activation.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(1)))
    );
    Ok(())
}
