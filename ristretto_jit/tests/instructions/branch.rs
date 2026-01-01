use crate::util::create_function;
use indexmap::IndexMap;
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use ristretto_jit::{Result, Value};

#[test]
fn ifeq() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifeq(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if not equal to zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifne() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifne(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if not equal to zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn iflt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iflt(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if less than zero
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if equal to or greater than zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifge() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifge(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if greater than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if less than zero
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifgt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifgt(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if greater than zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if less than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifle() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifle(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if less than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if greater than zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpeq() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpeq(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if values are equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if values are not equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpne() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpne(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if values are not equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if values are equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmplt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmplt(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is less than the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is greater than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpge() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpge(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is greater than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is less than the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpgt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpgt(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is greater than the second
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is less than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmple() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmple(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is less than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is greater than the second
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn goto() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Goto(2),
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn goto_w() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Goto_w(2),
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn jsr_and_ret() -> Result<()> {
    let instructions = vec![
        Instruction::Jsr(2),
        Instruction::Ireturn,
        Instruction::Istore_0,
        Instruction::Iconst_2,
        Instruction::Ret(0),
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn jsr_w_and_ret_w() -> Result<()> {
    let instructions = vec![
        Instruction::Jsr_w(2),
        Instruction::Ireturn,
        Instruction::Istore_0,
        Instruction::Iconst_2,
        Instruction::Ret_w(0),
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn tableswitch() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Tableswitch(TableSwitch {
            default: 7,
            low: 0,
            high: 2,
            offsets: vec![1, 3, 5],
        }),
        Instruction::Iconst_0,
        Instruction::Ireturn,
        Instruction::Iconst_1,
        Instruction::Ireturn,
        Instruction::Iconst_2,
        Instruction::Ireturn,
        Instruction::Iconst_m1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 0 (within range)
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test case 1 (within range)
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // Test case 2 (within range)
    let value = function.execute(vec![Value::I32(2)])?.expect("value");
    assert_eq!(value, Value::I32(2));

    // Test default case (below range)
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    // Test default case (above range)
    let value = function.execute(vec![Value::I32(3)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    // Test default case (far above range)
    let value = function.execute(vec![Value::I32(100)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    Ok(())
}

/// Test tableswitch with negative offsets (backward jumps)
#[test]
fn tableswitch_negative_offset() -> Result<()> {
    // Structure: Entry jumps to switch, switch can jump backward
    let instructions = vec![
        Instruction::Goto(4),  // 0: jump to the switch at instruction 4
        Instruction::Iconst_0, // 1: target of backward jump - push 0
        Instruction::Ireturn,  // 2: return 0
        Instruction::Nop,      // 3: padding (unreachable)
        Instruction::Iload_0,  // 4: load input for switch
        Instruction::Tableswitch(TableSwitch {
            default: 3, // forward to instruction 8 (5 + 3)
            low: 0,
            high: 1,
            offsets: vec![-4, 3], // case 0: backward to instruction 1 (5 + (-4) = 1), case 1: forward to instruction 8
        }), // 5: switch
        Instruction::Nop,      // 6: padding
        Instruction::Nop,      // 7: padding
        Instruction::Iconst_1, // 8: default/case 1 - push 1
        Instruction::Ireturn,  // 9: return 1
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 0 - should take the backward jump and return 0
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test case 1 - should go forward and return 1
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // Test default case - should go forward and return 1
    let value = function.execute(vec![Value::I32(99)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    Ok(())
}

/// Test tableswitch with negative default offset (backward jump for default case)
#[test]
fn tableswitch_negative_default_offset() -> Result<()> {
    // Structure: Entry jumps to switch, default case jumps backward
    let instructions = vec![
        Instruction::Goto(4),   // 0: jump to the switch at instruction 4
        Instruction::Iconst_m1, // 1: target of backward jump (default) - push -1
        Instruction::Ireturn,   // 2: return -1
        Instruction::Nop,       // 3: padding (unreachable)
        Instruction::Iload_0,   // 4: load input for switch
        Instruction::Tableswitch(TableSwitch {
            default: -4, // backward to instruction 1 (5 + (-4) = 1)
            low: 0,
            high: 0,
            offsets: vec![3], // case 0: forward to instruction 8 (5 + 3)
        }), // 5: switch
        Instruction::Nop,       // 6: padding
        Instruction::Nop,       // 7: padding
        Instruction::Iconst_0,  // 8: case 0 result - push 0
        Instruction::Ireturn,   // 9: return 0
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 0 - should go forward and return 0
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test default case (value not in range) - should take backward jump and return -1
    let value = function.execute(vec![Value::I32(99)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    Ok(())
}

#[test]
fn test_lookupswitch() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Lookupswitch(LookupSwitch {
            default: 7,
            pairs: IndexMap::from_iter([(0, 1), (1, 3), (2, 5)]),
        }),
        Instruction::Iconst_0,
        Instruction::Ireturn,
        Instruction::Iconst_1,
        Instruction::Ireturn,
        Instruction::Iconst_2,
        Instruction::Ireturn,
        Instruction::Iconst_m1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 0 (within range)
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test case 1 (within range)
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // Test case 2 (within range)
    let value = function.execute(vec![Value::I32(2)])?.expect("value");
    assert_eq!(value, Value::I32(2));

    // Test default case (below range)
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    // Test default case (above range)
    let value = function.execute(vec![Value::I32(3)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    // Test default case (far above range)
    let value = function.execute(vec![Value::I32(100)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    Ok(())
}

/// Test lookupswitch with negative offsets (backward jumps)
#[test]
fn lookupswitch_negative_offset() -> Result<()> {
    // Structure: Entry jumps to switch, switch can jump backward
    let instructions = vec![
        Instruction::Goto(4),  // 0: jump to the switch at instruction 4
        Instruction::Iconst_0, // 1: target of backward jump - push 0
        Instruction::Ireturn,  // 2: return 0
        Instruction::Nop,      // 3: padding (unreachable)
        Instruction::Iload_0,  // 4: load input for switch
        Instruction::Lookupswitch(LookupSwitch {
            default: 3, // forward to instruction 8 (5 + 3)
            pairs: IndexMap::from_iter([
                (42, -4), // case 42: backward to instruction 1 (5 + (-4) = 1)
                (100, 3), // case 100: forward to instruction 8
            ]),
        }), // 5: switch
        Instruction::Nop,      // 6: padding
        Instruction::Nop,      // 7: padding
        Instruction::Iconst_1, // 8: default/case 100 result - push 1
        Instruction::Ireturn,  // 9: return 1
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 42 - should take the backward jump and return 0
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test case 100 - should go forward and return 1
    let value = function.execute(vec![Value::I32(100)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // Test default case - should go forward and return 1
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    Ok(())
}

/// Test lookupswitch with negative default offset (backward jump for default case)
#[test]
fn lookupswitch_negative_default_offset() -> Result<()> {
    // Structure: Entry jumps to switch, default case jumps backward
    let instructions = vec![
        Instruction::Goto(4),   // 0: jump to the switch at instruction 4
        Instruction::Iconst_m1, // 1: target of backward jump (default) - push -1
        Instruction::Ireturn,   // 2: return -1
        Instruction::Nop,       // 3: padding (unreachable)
        Instruction::Iload_0,   // 4: load input for switch
        Instruction::Lookupswitch(LookupSwitch {
            default: -4, // backward to instruction 1 (5 + (-4) = 1)
            pairs: IndexMap::from_iter([
                (42, 3), // case 42: forward to instruction 8 (5 + 3)
            ]),
        }), // 5: switch
        Instruction::Nop,       // 6: padding
        Instruction::Nop,       // 7: padding
        Instruction::Iconst_0,  // 8: case 42 result - push 0
        Instruction::Ireturn,   // 9: return 0
    ];
    let function = create_function("(I)I", &instructions)?;

    // Test case 42 - should go forward and return 0
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(0));

    // Test default case - should take backward jump and return -1
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(-1));

    Ok(())
}

#[test]
fn r#return() -> Result<()> {
    let instructions = vec![Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}
