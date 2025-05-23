use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Result, Value};

#[test]
fn float_is_nan() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_name_index = constant_pool.add_class("Float")?;
    let code_index = constant_pool.add_utf8("Code")?;
    let test_name_index = constant_pool.add_utf8("isNan")?;
    let test_descriptor_index = constant_pool.add_utf8("(F)Z")?;

    let mut test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: test_name_index,
        descriptor_index: test_descriptor_index,
        attributes: Vec::new(),
    };
    let test_method_code = vec![
        Instruction::Fload_0,
        Instruction::Fload_0,
        Instruction::Fcmpl,
        Instruction::Ifeq(6),
        Instruction::Iconst_1,
        Instruction::Goto(7),
        Instruction::Iconst_0,
        Instruction::Ireturn,
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
    let value = function.execute(vec![Value::F32(42.1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    // let value = function
    //     .execute(vec![Value::F32(f32::NAN)])?
    //     .expect("value");
    // assert_eq!(value, Value::I32(1));
    Ok(())
}
