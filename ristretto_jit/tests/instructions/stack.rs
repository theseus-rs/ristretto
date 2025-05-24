use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Value;

#[test]
fn pop() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Pop,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn pop2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_2,
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn dup() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Dup,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn dup_x1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Dup_x1,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn dup_x2_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_2,
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Dup_x2,
        Instruction::Pop2,
        Instruction::Pop,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn dup_x2_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Lconst_0,
        Instruction::Iconst_1,
        Instruction::Dup_x2,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn dup2_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Iconst_2,
        Instruction::Dup2,
        Instruction::Pop2,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(3));
    Ok(())
}

#[test]
fn dup2_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Dup2,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I64(2));
    Ok(())
}

#[test]
fn dup2_x1_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Iconst_2,
        Instruction::Dup2_x1,
        Instruction::Pop2,
        Instruction::Pop,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(3));
    Ok(())
}

#[test]
fn dup2_x1_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_0,
        Instruction::Lconst_1,
        Instruction::Dup2_x1,
        Instruction::Pop,
        Instruction::Pop,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn dup2_x2_value1_category_1_value3_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_4,
        Instruction::Iconst_3,
        Instruction::Iconst_2,
        Instruction::Iconst_1,
        Instruction::Dup2_x2,
        Instruction::Iadd,
        Instruction::Iadd,
        Instruction::Iadd,
        Instruction::Iadd,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(13));
    Ok(())
}

#[test]
fn dup2_x2_value1_category_1_value3_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Lconst_0,
        Instruction::Iconst_2,
        Instruction::Iconst_1,
        Instruction::Dup2_x2,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(3));
    Ok(())
}

#[test]
fn dup2_x2_value1_category_2_value2_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_2,
        Instruction::Iconst_m1,
        Instruction::Lconst_1,
        Instruction::Dup2_x2,
        Instruction::Pop,
        Instruction::Pop,
        Instruction::Pop,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn dup2_x2_value1_category_2_value2_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Lconst_1,
        Instruction::Lconst_0,
        Instruction::Dup2_x2,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    let function = create_function("()J", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I64(1));
    Ok(())
}

#[test]
fn swap() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Swap,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}
