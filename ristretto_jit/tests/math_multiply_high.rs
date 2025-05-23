use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Result, Value};

#[test]
fn math_multiply_high() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_name_index = constant_pool.add_class("Math")?;
    let first_argument_index = constant_pool.add_long(4_294_967_295)?;
    let code_index = constant_pool.add_utf8("Code")?;
    let test_name_index = constant_pool.add_utf8("multiplyHigh")?;
    let test_descriptor_index = constant_pool.add_utf8("(JJ)J")?;

    let mut test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: test_name_index,
        descriptor_index: test_descriptor_index,
        attributes: Vec::new(),
    };
    let test_method_code = vec![
        // Input arguments x: i64, y: i64
        // x1 = x >> 32
        Instruction::Lload_0,
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(4),
        // x2 = x & 0xFFFFFFFF
        Instruction::Lload_0,
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(6),
        // y1 = y >> 32
        Instruction::Lload_2,
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(8),
        // y2 = y & 0xFFFFFFFF
        Instruction::Lload_2,
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(10),
        // z2 = x2 * y2
        Instruction::Lload(6),
        Instruction::Lload(10),
        Instruction::Lmul,
        Instruction::Lstore(12),
        // t = x1 * y2 + (z2 >>> 32)
        Instruction::Lload(4),
        Instruction::Lload(10),
        Instruction::Lmul,
        Instruction::Lload(12),
        Instruction::Bipush(32),
        Instruction::Lushr,
        Instruction::Ladd,
        Instruction::Lstore(14),
        // z1 = t & 0xFFFFFFFF
        Instruction::Lload(14),
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(16),
        // z0 = t >> 32
        Instruction::Lload(14),
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(18),
        // z1 += x2 * y1
        Instruction::Lload(16),
        Instruction::Lload(6),
        Instruction::Lload(8),
        Instruction::Lmul,
        Instruction::Ladd,
        Instruction::Lstore(16),
        // x1 * y1 + z0 + (z1 >> 32)
        Instruction::Lload(4),
        Instruction::Lload(8),
        Instruction::Lmul,
        Instruction::Lload(18),
        Instruction::Ladd,
        Instruction::Lload(16),
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    let test_max_stack = test_method_code.max_stack(&constant_pool)?;
    let test_max_locals = test_method_code.max_locals(&constant_pool, test_descriptor_index)?;
    test_method.attributes.push(Attribute::Code {
        name_index: code_index,
        max_stack: test_max_stack,
        max_locals: test_max_locals,
        code: test_method_code,
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });
    let class_file = ClassFile {
        constant_pool,
        access_flags: ClassAccessFlags::PUBLIC,
        this_class: class_name_index,
        methods: vec![test_method],
        attributes: Vec::new(),
        ..Default::default()
    };
    let test_method = &class_file.methods[0];

    let compiler = Compiler::new()?;
    let function = compiler.compile(&class_file, test_method)?;
    let arguments = vec![Value::I64(32_767), Value::I64(9_223_372_036_854_775_807)];
    let value = function.execute(arguments)?.expect("value");
    assert_eq!(value, Value::I64(16_383));
    Ok(())
}
