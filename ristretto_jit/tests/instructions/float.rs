use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn fconst_0() -> Result<()> {
    let instructions = vec![Instruction::Fconst_0, Instruction::Freturn];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(0.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fconst_1() -> Result<()> {
    let instructions = vec![Instruction::Fconst_1, Instruction::Freturn];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fconst_2() -> Result<()> {
    let instructions = vec![Instruction::Fconst_2, Instruction::Freturn];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(2.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload() -> Result<()> {
    let instructions = vec![Instruction::Fload(0), Instruction::Freturn];
    let function = create_function("(F)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload_w() -> Result<()> {
    let instructions = vec![Instruction::Fload_w(0), Instruction::Freturn];
    let function = create_function("(F)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload_0() -> Result<()> {
    let instructions = vec![Instruction::Fload_0, Instruction::Freturn];
    let function = create_function("(F)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload_1() -> Result<()> {
    let instructions = vec![Instruction::Fload_1, Instruction::Freturn];
    let function = create_function("(IF)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload_2() -> Result<()> {
    let instructions = vec![Instruction::Fload_2, Instruction::Freturn];
    let function = create_function("(IIF)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0), expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fload_3() -> Result<()> {
    let instructions = vec![Instruction::Fload_3, Instruction::Freturn];
    let function = create_function("(IIIF)F", &instructions)?;
    let expected_value = Value::F32(42.1);
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
fn fstore() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore(0),
        Instruction::Fload(0),
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fstore_w() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore_w(0),
        Instruction::Fload_w(0),
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fstore_0() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore_0,
        Instruction::Fload_0,
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fstore_1() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore_1,
        Instruction::Fload_1,
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fstore_2() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore_2,
        Instruction::Fload_2,
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fstore_3() -> Result<()> {
    let instructions = vec![
        Instruction::Fconst_1,
        Instruction::Fstore_3,
        Instruction::Fload_3,
        Instruction::Freturn,
    ];
    let function = create_function("()F", &instructions)?;
    let expected_value = Value::F32(1.0);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn fadd() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fadd,
        Instruction::Freturn,
    ];
    let function = create_function("(FF)F", &instructions)?;
    let value = function
        .execute(vec![Value::F32(1.0), Value::F32(2.0)])?
        .expect("value");
    assert_eq!(value, Value::F32(3.0));
    Ok(())
}

#[test]
fn fsub() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fsub,
        Instruction::Freturn,
    ];
    let function = create_function("(FF)F", &instructions)?;
    let value = function
        .execute(vec![Value::F32(2.0), Value::F32(1.0)])?
        .expect("value");
    assert_eq!(value, Value::F32(1.0));
    Ok(())
}

#[test]
fn fmul() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fmul,
        Instruction::Freturn,
    ];
    let function = create_function("(FF)F", &instructions)?;
    let value = function
        .execute(vec![Value::F32(2.0), Value::F32(3.0)])?
        .expect("value");
    assert_eq!(value, Value::F32(6.0));
    Ok(())
}

#[test]
fn fdiv() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fdiv,
        Instruction::Freturn,
    ];
    let function = create_function("(FF)F", &instructions)?;
    let value = function
        .execute(vec![Value::F32(6.0), Value::F32(3.0)])?
        .expect("value");
    assert_eq!(value, Value::F32(2.0));
    Ok(())
}

#[test]
fn frem() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Frem,
        Instruction::Freturn,
    ];
    let function = create_function("(FF)F", &instructions)?;
    let value = function
        .execute(vec![Value::F32(5.0), Value::F32(2.0)])?
        .expect("value");
    assert_eq!(value, Value::F32(1.0));
    Ok(())
}

#[test]
fn fneg() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fneg,
        Instruction::Freturn,
    ];
    let function = create_function("(F)F", &instructions)?;
    let value = function.execute(vec![Value::F32(3.0)])?.expect("value");
    assert_eq!(value, Value::F32(-3.0));
    Ok(())
}

#[test]
fn fcmpl() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fcmpl,
        Instruction::Ireturn,
    ];
    let function = create_function("(FF)I", &instructions)?;

    // 0 if values are equal
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));

    // 1 if first value is greater than second
    let value = function
        .execute(vec![Value::F32(1.0), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // -1 if first value is lesser than the second
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(1.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(-1));

    // -1 if first value is NaN
    let value = function
        .execute(vec![Value::F32(f32::NAN), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(-1));

    // -1 if second value is NaN
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(f32::NAN)])?
        .expect("value");
    assert_eq!(value, Value::I32(-1));
    Ok(())
}

#[test]
fn fcmpg() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_1,
        Instruction::Fcmpg,
        Instruction::Ireturn,
    ];
    let function = create_function("(FF)I", &instructions)?;

    // 0 if values are equal
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));

    // 1 if first value is greater than second
    let value = function
        .execute(vec![Value::F32(1.0), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // -1 if first value is lesser than the second
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(1.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(-1));

    // 1 if first value is NaN
    let value = function
        .execute(vec![Value::F32(f32::NAN), Value::F32(0.0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // 1 if second value is NaN
    let value = function
        .execute(vec![Value::F32(0.0), Value::F32(f32::NAN)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn freturn() -> Result<()> {
    let instructions = vec![Instruction::Fload_0, Instruction::Freturn];
    let function = create_function("(F)F", &instructions)?;
    let expected_value = Value::F32(42.1);
    let value = function
        .execute(vec![expected_value.clone()])?
        .expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}
