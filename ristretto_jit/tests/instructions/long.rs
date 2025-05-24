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
fn ladd() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(1), Value::I64(2)])?
        .expect("value");
    assert_eq!(value, Value::I64(3));
    Ok(())
}

#[test]
fn lsub() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lsub,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(2), Value::I64(1)])?
        .expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn lmul() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lmul,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(2), Value::I64(3)])?
        .expect("value");
    assert_eq!(value, Value::I64(6));
    Ok(())
}

#[test]
fn ldiv() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Ldiv,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(6), Value::I64(3)])?
        .expect("value");
    assert_eq!(value, Value::I64(2));
    Ok(())
}

#[test]
fn lrem() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lrem,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(5), Value::I64(2)])?
        .expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn lneg() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lneg,
        Instruction::Lreturn,
    ];
    let function = create_function("(J)J", &instructions)?;
    let value = function.execute(vec![Value::I64(3)])?.expect("value");
    assert_eq!(value, Value::I64(-3));
    Ok(())
}

#[test]
fn lshl() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Iload_2,
        Instruction::Lshl,
        Instruction::Lreturn,
    ];
    let function = create_function("(JI)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(2), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I64(4));
    Ok(())
}

#[test]
fn lshr() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Iload_2,
        Instruction::Lshr,
        Instruction::Lreturn,
    ];
    let function = create_function("(JI)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(8), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I64(4));
    Ok(())
}

#[test]
fn lushr() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Iload_2,
        Instruction::Lushr,
        Instruction::Lreturn,
    ];
    let function = create_function("(JI)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(8), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I64(4));
    Ok(())
}

#[test]
fn land() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Land,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(3), Value::I64(2)])?
        .expect("value");
    assert_eq!(value, Value::I64(2));
    Ok(())
}

#[test]
fn lor() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lor,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(3), Value::I64(2)])?
        .expect("value");
    assert_eq!(value, Value::I64(3));
    Ok(())
}

#[test]
fn lxor() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lxor,
        Instruction::Lreturn,
    ];
    let function = create_function("(JJ)J", &instructions)?;
    let value = function
        .execute(vec![Value::I64(3), Value::I64(2)])?
        .expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn lcmp() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lcmp,
        Instruction::Ireturn,
    ];
    let function = create_function("(JJ)I", &instructions)?;

    // 0 if values are equal
    let value = function
        .execute(vec![Value::I64(0), Value::I64(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));

    // 1 if first value is greater than the second
    let value = function
        .execute(vec![Value::I64(1), Value::I64(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // -1 if first value is lesser than the second
    let value = function
        .execute(vec![Value::I64(0), Value::I64(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(-1));
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
