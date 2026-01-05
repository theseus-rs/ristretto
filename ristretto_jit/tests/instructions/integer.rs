use crate::util::create_function;
use ristretto_classfile::attributes::{ArrayType, Instruction};
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
fn iaload_iastore() -> Result<()> {
    let instructions = vec![
        Instruction::Bipush(10),
        Instruction::Newarray(ArrayType::Int),
        Instruction::Dup,
        Instruction::Iconst_0,
        Instruction::Bipush(42),
        Instruction::Iastore,
        Instruction::Iconst_0,
        Instruction::Iaload,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn iadd() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(1), Value::I32(2)])?
        .expect("value");
    assert_eq!(value, Value::I32(3));
    Ok(())
}

#[test]
fn isub() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Isub,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(2), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn imul() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Imul,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(2), Value::I32(3)])?
        .expect("value");
    assert_eq!(value, Value::I32(6));
    Ok(())
}

#[test]
fn idiv() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Idiv,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(6), Value::I32(3)])?
        .expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn irem() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Irem,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(5), Value::I32(2)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn ineg() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ineg,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(3)])?.expect("value");
    assert_eq!(value, Value::I32(-3));
    Ok(())
}

#[test]
fn ishl() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Ishl,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(2), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(4));
    Ok(())
}

#[test]
fn ishr() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Ishr,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(8), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(4));
    Ok(())
}

#[test]
fn iushr() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Iushr,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(8), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(4));
    Ok(())
}

#[test]
fn iand() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Iand,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(3), Value::I32(2)])?
        .expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn ior() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Ior,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(3), Value::I32(2)])?
        .expect("value");
    assert_eq!(value, Value::I32(3));
    Ok(())
}

#[test]
fn ixor() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Ixor,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(vec![Value::I32(3), Value::I32(2)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn iinc() -> Result<()> {
    let instructions = vec![
        Instruction::Iinc(0, 1),
        Instruction::Iload_0,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn iinc_w() -> Result<()> {
    let instructions = vec![
        Instruction::Iinc_w(0, 1),
        Instruction::Iload_0,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(2));
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

#[test]
fn imul_square() -> Result<()> {
    // This simulates lambda$main$15: n -> n * n
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_0,
        Instruction::Imul,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(7)])?.expect("value");
    assert_eq!(value, Value::I32(49));
    Ok(())
}
