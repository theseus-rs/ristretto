//! Compiled entry validates the descriptor before generated argument loads.
use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn missing_argument_is_rejected_before_generated_entry() -> Result<()> {
    let function = create_function("(I)I", &[Instruction::Iload_0, Instruction::Ireturn])?;
    // The previous inline buffer had eight initialized slots, so this reproduces
    // the missing check without deliberately reading beyond an allocation.
    assert!(function.execute(&[], std::ptr::null()).is_err());
    assert_eq!(
        function.execute(&[Value::I32(42)], std::ptr::null())?,
        Some(Value::I32(42))
    );
    Ok(())
}

#[test]
fn generated_signatures_cover_inline_heap_and_cloned_entries() -> Result<()> {
    for count in [0_usize, 1, 8, 9, 10, 127] {
        let descriptor = format!("({})I", "I".repeat(count));
        let code = if count == 0 {
            vec![Instruction::Bipush(42), Instruction::Ireturn]
        } else {
            vec![
                Instruction::Iload(u8::try_from(count - 1)?),
                Instruction::Ireturn,
            ]
        };
        let function = create_function(&descriptor, &code)?;
        let cloned = function.clone();
        for actual in [0, count.saturating_sub(1), count + 1] {
            if actual == count {
                continue;
            }
            assert!(matches!(
                cloned.execute(&vec![Value::I32(42); actual], std::ptr::null()),
                Err(ristretto_jit::Error::InvalidArgumentCount { expected, actual: found })
                    if expected == count && found == actual
            ));
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 0);
        for function in [&function, &cloned] {
            assert_eq!(
                function.execute(&vec![Value::I32(42); count], std::ptr::null())?,
                Some(Value::I32(42))
            );
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 2);
    }
    Ok(())
}

#[test]
fn descriptor_kinds_and_wide_values_are_checked_before_loads() -> Result<()> {
    let function = create_function(
        "(ZBCSIJFDLjava/lang/Object;[I)I",
        &[Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    let arguments = [
        Value::I32(1),
        Value::I32(-2),
        Value::I32(3),
        Value::I32(4),
        Value::I32(5),
        Value::I64(6),
        Value::F32(7.0),
        Value::F64(8.0),
        Value::Ptr(0),
        Value::Ptr(0),
    ];
    // Ten ABI values occupy twelve JVM local slots; wide values are not padded.
    assert!(matches!(
        function.execute(&vec![Value::I32(0); 12], std::ptr::null()),
        Err(ristretto_jit::Error::InvalidArgumentCount {
            expected: 10,
            actual: 12
        })
    ));
    for index in 0..arguments.len() {
        let mut wrong = arguments.clone();
        wrong[index] = if matches!(arguments[index], Value::I32(_)) {
            Value::I64(0)
        } else {
            Value::I32(0)
        };
        assert!(matches!(
            function.execute(&wrong, std::ptr::null()),
            Err(ristretto_jit::Error::InvalidArgumentType { index: found, .. }) if found == index
        ));
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 0);
    assert_eq!(
        function.execute(&arguments, std::ptr::null())?,
        Some(Value::I32(42))
    );
    Ok(())
}

extern "C" fn publish(_context: *const u8, _roots: *const i64, _count: i32) -> i32 {
    1
}
extern "C" fn pending(_context: *const u8) -> i64 {
    0
}

#[test]
fn instance_receiver_precedes_wide_parameters() -> Result<()> {
    use ristretto_classfile::attributes::{Attribute, MaxLocals};
    use ristretto_classfile::{ClassFile, ConstantPool, Method, MethodAccessFlags};
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("ArgumentReceiver")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let mut method = Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: pool.add_utf8("run")?,
        descriptor_index: pool.add_utf8("(JD)J")?,
        attributes: vec![],
    };
    let code = vec![Instruction::Lload_1, Instruction::Lreturn];
    method.attributes.push(Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: 2,
        max_locals: code.max_locals(&pool, &method)?,
        code,
        exception_table: vec![],
        attributes: vec![],
    });
    let class = ClassFile {
        constant_pool: pool,
        this_class,
        super_class,
        methods: vec![method],
        ..Default::default()
    };
    class.verify()?;
    let function = ristretto_jit::Compiler::new()?.compile(
        &class,
        class.methods.first().expect("instance method"),
        &[
            ("jit_publish_frame_roots", publish as *const u8),
            ("jit_pending_exception", pending as *const u8),
        ],
    )?;
    assert!(matches!(
        function.execute(&[Value::I64(42), Value::F64(1.5)], std::ptr::null()),
        Err(ristretto_jit::Error::InvalidArgumentCount {
            expected: 3,
            actual: 2
        })
    ));
    assert!(matches!(
        function.execute(
            &[Value::I64(0), Value::I64(42), Value::F64(1.5)],
            std::ptr::null()
        ),
        Err(ristretto_jit::Error::InvalidArgumentType { index: 0, .. })
    ));
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 0);
    // This backend-only shape check does not perform Java virtual invocation or
    // validate receiver liveness; its frame-root helper deliberately accepts null.
    assert_eq!(
        function.execute(
            &[Value::Ptr(0), Value::I64(42), Value::F64(1.5)],
            std::ptr::null()
        )?,
        Some(Value::I64(42))
    );
    Ok(())
}
