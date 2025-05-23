use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn iconst_m1() -> Result<()> {
    let instructions = vec![Instruction::Iconst_m1, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(-1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_0() -> Result<()> {
    let instructions = vec![Instruction::Iconst_0, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_1() -> Result<()> {
    let instructions = vec![Instruction::Iconst_1, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_2() -> Result<()> {
    let instructions = vec![Instruction::Iconst_2, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(2);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_3() -> Result<()> {
    let instructions = vec![Instruction::Iconst_3, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(3);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_4() -> Result<()> {
    let instructions = vec![Instruction::Iconst_4, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(4);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iconst_5() -> Result<()> {
    let instructions = vec![Instruction::Iconst_5, Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(5);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload() -> Result<()> {
    let instructions = vec![Instruction::Iload(0), Instruction::Ireturn];
    let function = create_function("(I)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload_w() -> Result<()> {
    let instructions = vec![Instruction::Iload_w(0), Instruction::Ireturn];
    let function = create_function("(I)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload_0() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::Ireturn];
    let function = create_function("(I)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload_1() -> Result<()> {
    let instructions = vec![Instruction::Iload_1, Instruction::Ireturn];
    let function = create_function("(II)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload_2() -> Result<()> {
    let instructions = vec![Instruction::Iload_2, Instruction::Ireturn];
    let function = create_function("(III)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn iload_3() -> Result<()> {
    let instructions = vec![Instruction::Iload_3, Instruction::Ireturn];
    let function = create_function("(IIII)I", &instructions)?;
    let expected_value = Value::I32(42);
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
fn istore() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore(0),
        Instruction::Iload(0),
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn istore_w() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore_w(0),
        Instruction::Iload_w(0),
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn istore_0() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore_0,
        Instruction::Iload_0,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn istore_1() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore_1,
        Instruction::Iload_1,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn istore_2() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore_2,
        Instruction::Iload_2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn istore_3() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Istore_3,
        Instruction::Iload_3,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(1);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn ireturn() -> Result<()> {
    let instructions = vec![Instruction::Iload_0, Instruction::Ireturn];
    let function = create_function("(I)I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}
