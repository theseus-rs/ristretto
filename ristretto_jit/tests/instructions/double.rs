use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn dconst_0() -> Result<()> {
    let instructions = vec![Instruction::Dconst_0, Instruction::Dreturn];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(0.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dconst_1() -> Result<()> {
    let instructions = vec![Instruction::Dconst_1, Instruction::Dreturn];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload() -> Result<()> {
    let instructions = vec![Instruction::Dload(0), Instruction::Dreturn];
    let function = create_function("(D)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload_w() -> Result<()> {
    let instructions = vec![Instruction::Dload_w(0), Instruction::Dreturn];
    let function = create_function("(D)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload_0() -> Result<()> {
    let instructions = vec![Instruction::Dload_0, Instruction::Dreturn];
    let function = create_function("(D)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload_1() -> Result<()> {
    let instructions = vec![Instruction::Dload_1, Instruction::Dreturn];
    let function = create_function("(ID)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload_2() -> Result<()> {
    let instructions = vec![Instruction::Dload_2, Instruction::Dreturn];
    let function = create_function("(IID)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dload_3() -> Result<()> {
    let instructions = vec![Instruction::Dload_3, Instruction::Dreturn];
    let function = create_function("(IIID)D", &instructions)?;
    let expected_value = Value::F64(42.1);
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
fn dstore() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore(0),
        Instruction::Dload(0),
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dstore_w() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore_w(0),
        Instruction::Dload_w(0),
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dstore_0() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore_0,
        Instruction::Dload_0,
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dstore_1() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore_1,
        Instruction::Dload_1,
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dstore_2() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore_2,
        Instruction::Dload_2,
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dstore_3() -> Result<()> {
    let instructions = vec![
        Instruction::Dconst_1,
        Instruction::Dstore_3,
        Instruction::Dload_3,
        Instruction::Dreturn,
    ];
    let function = create_function("()D", &instructions)?;
    let expected_value = Value::F64(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn dreturn() -> Result<()> {
    let instructions = vec![Instruction::Dload_0, Instruction::Dreturn];
    let function = create_function("(D)D", &instructions)?;
    let expected_value = Value::F64(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}
