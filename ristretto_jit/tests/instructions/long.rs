use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn lconst_0() -> Result<()> {
    let instructions = vec![Instruction::Lconst_0, Instruction::Lreturn];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lconst_1() -> Result<()> {
    let instructions = vec![Instruction::Lconst_1, Instruction::Lreturn];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload() -> Result<()> {
    let instructions = vec![Instruction::Lload(0), Instruction::Lreturn];
    let function = create_function("(J)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload_w() -> Result<()> {
    let instructions = vec![Instruction::Lload_w(0), Instruction::Lreturn];
    let function = create_function("(J)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload_0() -> Result<()> {
    let instructions = vec![Instruction::Lload_0, Instruction::Lreturn];
    let function = create_function("(J)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload_1() -> Result<()> {
    let instructions = vec![Instruction::Lload_1, Instruction::Lreturn];
    let function = create_function("(IJ)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload_2() -> Result<()> {
    let instructions = vec![Instruction::Lload_2, Instruction::Lreturn];
    let function = create_function("(IIJ)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lload_3() -> Result<()> {
    let instructions = vec![Instruction::Lload_3, Instruction::Lreturn];
    let function = create_function("(IIIJ)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![
            Value::I32(0),
            Value::I32(0),
            Value::I32(0),
            expected_value.clone(),
        ])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore(0),
        Instruction::Lload(0),
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore_w() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore_w(0),
        Instruction::Lload_w(0),
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore_0() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore_0,
        Instruction::Lload_0,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore_1() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore_1,
        Instruction::Lload_1,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore_2() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore_2,
        Instruction::Lload_2,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lstore_3() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lstore_3,
        Instruction::Lload_3,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let expected_value = Value::I64(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn lreturn() -> Result<()> {
    let instructions = vec![Instruction::Lload_0, Instruction::Lreturn];
    let function = create_function("(J)J", &instructions)?;
    let expected_value = Value::I64(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}
