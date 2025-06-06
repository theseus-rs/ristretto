use crate::util::create_function_with_constant_pool;
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Value;

#[test]
fn ldc_integer() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_integer(42)?;
    let instructions = vec![
        Instruction::Ldc(u8::try_from(constant_index)?),
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn ldc_float() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_float(42.1)?;
    let instructions = vec![
        Instruction::Ldc(u8::try_from(constant_index)?),
        Instruction::Freturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()F", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::F32(42.1));
    Ok(())
}

#[test]
fn ldc_string() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_string("foo")?;
    let instructions = vec![
        Instruction::Ldc(u8::try_from(constant_index)?),
        Instruction::Areturn,
    ];
    let result =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/String;", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc_class() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_class("Foo")?;
    let instructions = vec![
        Instruction::Ldc(u8::try_from(constant_index)?),
        Instruction::Areturn,
    ];
    let result =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/Object;", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc_long_error() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_long(42)?;
    let instructions = vec![
        Instruction::Ldc(u8::try_from(constant_index)?),
        Instruction::Lreturn,
    ];
    let result = create_function_with_constant_pool(constant_pool, "()J", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc_w_integer() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_integer(42)?;
    let instructions = vec![Instruction::Ldc_w(constant_index), Instruction::Ireturn];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn ldc_w_float() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_float(42.1)?;
    let instructions = vec![Instruction::Ldc_w(constant_index), Instruction::Freturn];
    let function = create_function_with_constant_pool(constant_pool, "()F", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::F32(42.1));
    Ok(())
}

#[test]
fn ldc_w_string() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_string("foo")?;
    let instructions = vec![Instruction::Ldc_w(constant_index), Instruction::Areturn];
    let result =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/String;", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc_w_class() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_class("Foo")?;
    let instructions = vec![Instruction::Ldc_w(constant_index), Instruction::Areturn];
    let result =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/Object;", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc_w_long_error() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_long(42)?;
    let instructions = vec![Instruction::Ldc_w(constant_index), Instruction::Lreturn];
    let result = create_function_with_constant_pool(constant_pool, "()J", &instructions);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn ldc2_w_long() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_long(42)?;
    let instructions = vec![Instruction::Ldc2_w(constant_index), Instruction::Lreturn];
    let function = create_function_with_constant_pool(constant_pool, "()J", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I64(42));
    Ok(())
}

#[test]
fn ldc2_w_double() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_double(42.1)?;
    let instructions = vec![Instruction::Ldc2_w(constant_index), Instruction::Dreturn];
    let function = create_function_with_constant_pool(constant_pool, "()D", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::F64(42.1));
    Ok(())
}

#[test]
fn ldc2_w_integer_error() -> ristretto_jit::Result<()> {
    let mut constant_pool = ConstantPool::new();
    let constant_index = constant_pool.add_integer(42)?;
    let instructions = vec![Instruction::Ldc2_w(constant_index), Instruction::Ireturn];
    let result = create_function_with_constant_pool(constant_pool, "()I", &instructions);
    assert!(result.is_err());
    Ok(())
}
