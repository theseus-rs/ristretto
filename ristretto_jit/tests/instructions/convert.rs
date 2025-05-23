use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn i2l() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2l, Instruction::Lreturn];
    let function = create_function("(I)J", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I64(42));
    Ok(())
}

#[test]
fn i2f() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2f, Instruction::Freturn];
    let function = create_function("(I)F", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::F32(42.0));
    Ok(())
}

#[test]
fn i2d() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2d, Instruction::Dreturn];
    let function = create_function("(I)D", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::F64(42.0));
    Ok(())
}

#[test]
fn l2i() -> Result<()> {
    let instructions = vec![Instruction::Lload_0, Instruction::L2i, Instruction::Ireturn];
    let function = create_function("(J)I", &instructions)?;
    let value = function.execute(vec![Value::I64(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn l2f() -> Result<()> {
    let instructions = vec![Instruction::Lload_0, Instruction::L2f, Instruction::Freturn];
    let function = create_function("(J)F", &instructions)?;
    let value = function.execute(vec![Value::I64(42)])?.expect("value");
    assert_eq!(value, Value::F32(42.0));
    Ok(())
}

#[test]
fn l2d() -> Result<()> {
    let instructions = vec![Instruction::Lload_0, Instruction::L2d, Instruction::Dreturn];
    let function = create_function("(J)D", &instructions)?;
    let value = function.execute(vec![Value::I64(42)])?.expect("value");
    assert_eq!(value, Value::F64(42.0));
    Ok(())
}

#[test]
fn f2i() -> Result<()> {
    let instructions = vec![Instruction::Fload_0, Instruction::F2i, Instruction::Ireturn];
    let function = create_function("(F)I", &instructions)?;
    let value = function.execute(vec![Value::F32(42.0)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn f2l() -> Result<()> {
    let instructions = vec![Instruction::Fload_0, Instruction::F2l, Instruction::Lreturn];
    let function = create_function("(F)J", &instructions)?;
    let value = function.execute(vec![Value::F32(42.0)])?.expect("value");
    assert_eq!(value, Value::I64(42));
    Ok(())
}

#[test]
fn f2d() -> Result<()> {
    let instructions = vec![Instruction::Fload_0, Instruction::F2d, Instruction::Dreturn];
    let function = create_function("(F)D", &instructions)?;
    let value = function.execute(vec![Value::F32(42.0)])?.expect("value");
    assert_eq!(value, Value::F64(42.0));
    Ok(())
}

#[test]
fn d2i() -> Result<()> {
    let instructions = vec![Instruction::Dload_0, Instruction::D2i, Instruction::Ireturn];
    let function = create_function("(D)I", &instructions)?;
    let value = function.execute(vec![Value::F64(42.0)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn d2l() -> Result<()> {
    let instructions = vec![Instruction::Dload_0, Instruction::D2l, Instruction::Lreturn];
    let function = create_function("(D)J", &instructions)?;
    let value = function.execute(vec![Value::F64(42.0)])?.expect("value");
    assert_eq!(value, Value::I64(42));
    Ok(())
}

#[test]
fn d2f() -> Result<()> {
    let instructions = vec![Instruction::Dload_0, Instruction::D2f, Instruction::Freturn];
    let function = create_function("(D)F", &instructions)?;
    let value = function.execute(vec![Value::F64(42.0)])?.expect("value");
    assert_eq!(value, Value::F32(42.0));
    Ok(())
}

#[test]
fn i2b() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2b, Instruction::Ireturn];
    let function = create_function("(I)Z", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn i2c() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2c, Instruction::Ireturn];
    let function = create_function("(I)C", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn i2s() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::I2s, Instruction::Ireturn];
    let function = create_function("(I)S", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}
