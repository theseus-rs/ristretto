use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Value;

#[test]
fn reused_reference_and_primitive_locals_cross_block_merges() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Aconst_null, // 0
        Instruction::Astore_1,    // 1
        Instruction::Aload_1,     // 2
        Instruction::Pop,         // 3
        Instruction::Iload_0,     // 4
        Instruction::Ifeq(8),     // 5 -> 13
        Instruction::Fconst_1,    // 8
        Instruction::Fstore_1,    // 9
        Instruction::Goto(5),     // 10 -> 15
        Instruction::Fconst_2,    // 13
        Instruction::Fstore_1,    // 14
        Instruction::Fload_1,     // 15
        Instruction::F2i,         // 16
        Instruction::Aconst_null, // 17
        Instruction::Astore_1,    // 18
        Instruction::Aload_1,     // 19
        Instruction::Aload_1,     // 20
        Instruction::Pop2,        // 21
        Instruction::Ireturn,     // 22
    ];
    let function = create_function("(I)I", &instructions)?;
    for (input, expected) in [(0, 2), (1, 1)] {
        assert_eq!(
            function.execute(&[Value::I32(input)], std::ptr::null())?,
            Some(Value::I32(expected))
        );
    }
    Ok(())
}

#[test]
fn references_use_category_one_stack_forms() -> ristretto_jit::Result<()> {
    for (inputs, operation, outputs) in [
        (2, Instruction::Pop2, 0),
        (1, Instruction::Dup, 2),
        (2, Instruction::Dup_x1, 3),
        (3, Instruction::Dup_x2, 4),
        (2, Instruction::Dup2, 4),
        (3, Instruction::Dup2_x1, 5),
        (4, Instruction::Dup2_x2, 6),
        (2, Instruction::Swap, 2),
    ] {
        let mut instructions = vec![Instruction::Iconst_5];
        instructions.extend(std::iter::repeat_n(Instruction::Aconst_null, inputs));
        instructions.push(operation);
        instructions.extend(std::iter::repeat_n(Instruction::Pop, outputs));
        instructions.push(Instruction::Ireturn);
        let function = create_function("()I", &instructions)?;
        assert_eq!(
            function.execute(&[], std::ptr::null())?,
            Some(Value::I32(5))
        );
    }
    Ok(())
}

#[test]
fn reference_categories_survive_block_parameters() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_5,    // 0
        Instruction::Aconst_null, // 1
        Instruction::Aconst_null, // 2
        Instruction::Iload_0,     // 3
        Instruction::Ifeq(6),     // 4 -> 10
        Instruction::Goto(3),     // 7 -> 10
        Instruction::Pop2,        // 10: two references, each category one
        Instruction::Ireturn,     // 11
    ];
    let function = create_function("(I)I", &instructions)?;
    for input in [0, 1] {
        assert_eq!(
            function.execute(&[Value::I32(input)], std::ptr::null())?,
            Some(Value::I32(5))
        );
    }
    Ok(())
}

#[test]
fn reference_categories_survive_helper_continuations() -> ristretto_jit::Result<()> {
    use ristretto_classfile::attributes::ArrayType;
    let instructions = vec![
        Instruction::Iconst_5,
        Instruction::Aconst_null,
        Instruction::Aconst_null,
        Instruction::Iconst_1,
        Instruction::Newarray(ArrayType::Int),
        Instruction::Arraylength, // also crosses the null-check continuation
        Instruction::Pop,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    assert_eq!(
        function.execute(&[], std::ptr::null())?,
        Some(Value::I32(5))
    );
    Ok(())
}

#[test]
fn pop() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Pop,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn pop2_category_1() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_2,
        Instruction::Iconst_1,
        Instruction::Iconst_0,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn pop2_category_2() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Iconst_2,
        Instruction::Lconst_0,
        Instruction::Pop2,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
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
    let value = function.execute(&[], std::ptr::null())?.expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}
