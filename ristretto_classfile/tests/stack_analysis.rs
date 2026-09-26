//! Regressions for JVMS operand-slot accounting and control-flow stack analysis.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with Result-returning helpers"
)]
use ristretto_classfile::attributes::{
    ExceptionTableEntry, Instruction, LookupSwitch, MaxStack, TableSwitch,
};
use ristretto_classfile::{ConstantPool, Error, FieldType, JavaStr, Result};

#[test]
fn long_constant() -> Result<()> {
    let code = [Instruction::Lconst_0, Instruction::Lreturn];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn double_constant() -> Result<()> {
    let code = [Instruction::Dconst_0, Instruction::Dreturn];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn integer_negation() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Ineg,
        Instruction::Iconst_1,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn float_negation() -> Result<()> {
    let code = [
        Instruction::Fconst_0,
        Instruction::Fneg,
        Instruction::Fconst_0,
        Instruction::Fadd,
        Instruction::Freturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn long_negation() -> Result<()> {
    let code = [
        Instruction::Lconst_0,
        Instruction::Lneg,
        Instruction::Lconst_0,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 4);
    Ok(())
}
#[test]
fn double_negation() -> Result<()> {
    let code = [
        Instruction::Dconst_0,
        Instruction::Dneg,
        Instruction::Dconst_0,
        Instruction::Dadd,
        Instruction::Dreturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 4);
    Ok(())
}
#[test]
fn widening_conversion() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::I2l,
        Instruction::Lreturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn conversion_chain() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::I2l,
        Instruction::L2f,
        Instruction::F2d,
        Instruction::D2i,
        Instruction::Ireturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn pop_two_category_one_values() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Pop2,
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Pop2,
        Instruction::Return,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn pop_one_category_two_value() -> Result<()> {
    let code = [
        Instruction::Lconst_0,
        Instruction::Pop2,
        Instruction::Lconst_0,
        Instruction::Pop2,
        Instruction::Return,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn reordered_blocks() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Goto(4),
        Instruction::Pop,
        Instruction::Return,
        Instruction::Iconst_1,
        Instruction::Iadd,
        Instruction::Goto(2),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn reordered_wide_branches() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Goto_w(4),
        Instruction::Pop,
        Instruction::Return,
        Instruction::Iconst_1,
        Instruction::Iadd,
        Instruction::Goto_w(2),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn balanced_diamond() -> Result<()> {
    let code = [
        Instruction::Iload_0,
        Instruction::Ifeq(5),
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Goto(7),
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Iadd,
        Instruction::Ireturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn balanced_loop() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Istore_0,
        Instruction::Iload_0,
        Instruction::Ifeq(7),
        Instruction::Iinc(0, -1),
        Instruction::Goto(2),
        Instruction::Nop,
        Instruction::Return,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}
#[test]
fn return_discards_remaining_stack() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Return,
        Instruction::Lconst_0,
        Instruction::Lconst_0,
        Instruction::Return,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn typed_return_ends_path() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Ireturn,
        Instruction::Lconst_0,
        Instruction::Lconst_0,
        Instruction::Lreturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}
#[test]
fn throw_ends_path() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Aconst_null,
        Instruction::Athrow,
        Instruction::Lconst_0,
        Instruction::Lconst_0,
        Instruction::Lreturn,
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn unreachable_bad_constant_pool_reference() -> Result<()> {
    let code = [Instruction::Return, Instruction::Invokevirtual(42)];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 0);
    Ok(())
}
#[test]
fn shared_legacy_subroutine() -> Result<()> {
    let code = [
        Instruction::Jsr(6),
        Instruction::Nop,
        Instruction::Jsr(6),
        Instruction::Nop,
        Instruction::Return,
        Instruction::Nop,
        Instruction::Astore_0,
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Pop2,
        Instruction::Ret(0),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn nested_legacy_subroutines() -> Result<()> {
    let code = [
        Instruction::Jsr(4),
        Instruction::Return,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Astore_0,
        Instruction::Jsr(8),
        Instruction::Ret(0),
        Instruction::Nop,
        Instruction::Astore_1,
        Instruction::Ret(1),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}
#[test]
fn legacy_return_to_outer_call() -> Result<()> {
    let code = [
        Instruction::Jsr(4),
        Instruction::Return,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Astore_0,
        Instruction::Jsr(8),
        Instruction::Ret(0),
        Instruction::Nop,
        Instruction::Astore_1,
        Instruction::Ret(0),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}
#[test]
fn wide_legacy_subroutine() -> Result<()> {
    let code = [
        Instruction::Jsr_w(2),
        Instruction::Return,
        Instruction::Astore_w(300),
        Instruction::Ret_w(300),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}
#[test]
fn duplicated_return_address() -> Result<()> {
    let code = [
        Instruction::Jsr(3),
        Instruction::Return,
        Instruction::Nop,
        Instruction::Dup,
        Instruction::Astore_0,
        Instruction::Pop,
        Instruction::Ret(0),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}
#[test]
fn swapped_return_address() -> Result<()> {
    let code = [
        Instruction::Jsr(3),
        Instruction::Return,
        Instruction::Nop,
        Instruction::Iconst_0,
        Instruction::Swap,
        Instruction::Astore_0,
        Instruction::Pop,
        Instruction::Ret(0),
    ];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    Ok(())
}

#[test]
fn local_stores_do_not_accumulate_phantom_slots() -> Result<()> {
    let cases = [
        (
            Instruction::Iconst_0,
            vec![
                Instruction::Istore(4),
                Instruction::Istore_0,
                Instruction::Istore_1,
                Instruction::Istore_2,
                Instruction::Istore_3,
                Instruction::Istore_w(300),
            ],
            1,
        ),
        (
            Instruction::Fconst_0,
            vec![
                Instruction::Fstore(4),
                Instruction::Fstore_0,
                Instruction::Fstore_1,
                Instruction::Fstore_2,
                Instruction::Fstore_3,
                Instruction::Fstore_w(300),
            ],
            1,
        ),
        (
            Instruction::Aconst_null,
            vec![
                Instruction::Astore(4),
                Instruction::Astore_0,
                Instruction::Astore_1,
                Instruction::Astore_2,
                Instruction::Astore_3,
                Instruction::Astore_w(300),
            ],
            1,
        ),
        (
            Instruction::Lconst_0,
            vec![
                Instruction::Lstore(4),
                Instruction::Lstore_0,
                Instruction::Lstore_1,
                Instruction::Lstore_2,
                Instruction::Lstore_3,
                Instruction::Lstore_w(300),
            ],
            2,
        ),
        (
            Instruction::Dconst_0,
            vec![
                Instruction::Dstore(4),
                Instruction::Dstore_0,
                Instruction::Dstore_1,
                Instruction::Dstore_2,
                Instruction::Dstore_3,
                Instruction::Dstore_w(300),
            ],
            2,
        ),
    ];
    for (constant, stores, expected) in cases {
        for store in stores {
            let code = [
                constant.clone(),
                store.clone(),
                constant.clone(),
                store.clone(),
                Instruction::Return,
            ];
            assert_eq!(code.max_stack(&ConstantPool::new())?, expected, "{store}");
        }
    }
    Ok(())
}

#[test]
fn array_loads_consume_reference_and_index() -> Result<()> {
    for (load, pop) in [
        (Instruction::Iaload, Instruction::Pop),
        (Instruction::Faload, Instruction::Pop),
        (Instruction::Aaload, Instruction::Pop),
        (Instruction::Baload, Instruction::Pop),
        (Instruction::Caload, Instruction::Pop),
        (Instruction::Saload, Instruction::Pop),
        (Instruction::Laload, Instruction::Pop2),
        (Instruction::Daload, Instruction::Pop2),
    ] {
        let code = [
            Instruction::Aload_0,
            Instruction::Iconst_0,
            load.clone(),
            pop,
            Instruction::Iconst_0,
            Instruction::Iconst_1,
            Instruction::Pop2,
            Instruction::Return,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, 2, "{load}");
    }
    Ok(())
}

#[test]
fn array_stores_consume_reference_index_and_value() -> Result<()> {
    for (store, value, expected) in [
        (Instruction::Iastore, Instruction::Iconst_0, 3),
        (Instruction::Fastore, Instruction::Fconst_0, 3),
        (Instruction::Aastore, Instruction::Aconst_null, 3),
        (Instruction::Bastore, Instruction::Iconst_0, 3),
        (Instruction::Castore, Instruction::Iconst_0, 3),
        (Instruction::Sastore, Instruction::Iconst_0, 3),
        (Instruction::Lastore, Instruction::Lconst_0, 4),
        (Instruction::Dastore, Instruction::Dconst_0, 4),
    ] {
        let code = [
            Instruction::Aload_0,
            Instruction::Iconst_0,
            value.clone(),
            store.clone(),
            Instruction::Aload_0,
            Instruction::Iconst_0,
            value,
            store.clone(),
            Instruction::Return,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, expected, "{store}");
    }
    Ok(())
}

#[test]
fn category_two_arithmetic_and_comparisons() -> Result<()> {
    for operation in [
        Instruction::Ladd,
        Instruction::Lsub,
        Instruction::Lmul,
        Instruction::Ldiv,
        Instruction::Lrem,
        Instruction::Land,
        Instruction::Lor,
        Instruction::Lxor,
    ] {
        let code = [
            Instruction::Lconst_0,
            Instruction::Lconst_0,
            operation.clone(),
            Instruction::Lconst_0,
            operation.clone(),
            Instruction::Lreturn,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, 4, "{operation}");
    }
    for operation in [
        Instruction::Dadd,
        Instruction::Dsub,
        Instruction::Dmul,
        Instruction::Ddiv,
        Instruction::Drem,
    ] {
        let code = [
            Instruction::Dconst_0,
            Instruction::Dconst_0,
            operation.clone(),
            Instruction::Dconst_0,
            operation.clone(),
            Instruction::Dreturn,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, 4, "{operation}");
    }
    for (constant, operation) in [
        (Instruction::Lconst_0, Instruction::Lcmp),
        (Instruction::Dconst_0, Instruction::Dcmpl),
        (Instruction::Dconst_0, Instruction::Dcmpg),
    ] {
        let code = [
            constant.clone(),
            constant.clone(),
            operation.clone(),
            Instruction::Pop,
            constant.clone(),
            constant,
            operation.clone(),
            Instruction::Ireturn,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, 4, "{operation}");
    }
    Ok(())
}

#[test]
fn all_field_widths_and_reference_types() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Fields")?;
    for (descriptor, slots) in [
        ("Z", 1),
        ("B", 1),
        ("C", 1),
        ("S", 1),
        ("I", 1),
        ("F", 1),
        ("J", 2),
        ("D", 2),
        ("Ljava/lang/Object;", 1),
        ("[J", 1),
        ("[[D", 1),
    ] {
        let field = pool.add_field_ref(class, "value", descriptor)?;
        for (instruction, expected) in [
            (Instruction::Getstatic(field), slots),
            (Instruction::Putstatic(field), -slots),
            (Instruction::Getfield(field), slots - 1),
            (Instruction::Putfield(field), -slots - 1),
        ] {
            assert_eq!(
                instruction.stack_delta(&pool)?,
                expected,
                "{descriptor}: {instruction}"
            );
        }
        let code = [
            Instruction::Getstatic(field),
            Instruction::Putstatic(field),
            Instruction::Getstatic(field),
            Instruction::Putstatic(field),
            Instruction::Return,
        ];
        assert_eq!(code.max_stack(&pool)?, u16::try_from(slots)?);
    }
    Ok(())
}

#[test]
fn field_operands_and_descriptors_are_validated() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Fields")?;
    for descriptor in ["", "V", "JI", "Ljava/lang/Object;I", "[V"] {
        let field = pool.add_field_ref(class, "bad", descriptor)?;
        for instruction in [
            Instruction::Getstatic(field),
            Instruction::Putstatic(field),
            Instruction::Getfield(field),
            Instruction::Putfield(field),
        ] {
            assert!(
                instruction.stack_delta(&pool).is_err(),
                "{descriptor}: {instruction}"
            );
        }
    }
    for index in [0, class, u16::MAX] {
        assert!(Instruction::Getfield(index).stack_delta(&pool).is_err());
    }
    Ok(())
}

#[test]
fn calls_account_for_arguments_receivers_and_returns() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Calls")?;
    for (result, slots) in [
        ("V", 0),
        ("I", 1),
        ("J", 2),
        ("D", 2),
        ("[J", 1),
        ("Ljava/lang/Object;", 1),
    ] {
        let descriptor = format!("(IJ[D){result}"); // 1 + 2 + 1 argument slots
        let method = pool.add_method_ref(class, "m", &descriptor)?;
        let interface = pool.add_interface_method_ref(class, "m", &descriptor)?;
        let dynamic = pool.add_invoke_dynamic(0, "m", &descriptor)?;
        for instruction in [
            Instruction::Invokestatic(method),
            Instruction::Invokestatic(interface),
            Instruction::Invokedynamic(dynamic),
        ] {
            assert_eq!(
                instruction.stack_delta(&pool)?,
                slots - 4,
                "{instruction}: {descriptor}"
            );
        }
        for instruction in [
            Instruction::Invokevirtual(method),
            Instruction::Invokespecial(method),
            Instruction::Invokespecial(interface),
            Instruction::Invokeinterface(interface, 5),
        ] {
            assert_eq!(
                instruction.stack_delta(&pool)?,
                slots - 5,
                "{instruction}: {descriptor}"
            );
        }
        assert!(
            Instruction::Invokevirtual(interface)
                .stack_delta(&pool)
                .is_err()
        );
        assert!(
            Instruction::Invokeinterface(method, 5)
                .stack_delta(&pool)
                .is_err()
        );
        assert!(
            Instruction::Invokestatic(dynamic)
                .stack_delta(&pool)
                .is_err()
        );
    }
    Ok(())
}

#[test]
fn invokeinterface_count_includes_receiver_and_wide_arguments() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Calls")?;
    let method = pool.add_interface_method_ref(class, "m", "(JD)V")?;
    assert_eq!(
        Instruction::Invokeinterface(method, 5).stack_delta(&pool)?,
        -5
    );
    for count in [0, 1, 3, 4, 6, 255] {
        assert!(
            Instruction::Invokeinterface(method, count)
                .stack_delta(&pool)
                .is_err()
        );
    }
    Ok(())
}

#[test]
fn method_parameter_slot_limits_and_receiver_boundary() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Calls")?;
    for arguments in ["I".repeat(255), format!("{}I", "J".repeat(127))] {
        let descriptor = format!("({arguments})V");
        let method = pool.add_method_ref(class, "m", &descriptor)?;
        let interface = pool.add_interface_method_ref(class, "m", &descriptor)?;
        assert_eq!(Instruction::Invokestatic(method).stack_delta(&pool)?, -255);
        assert_eq!(
            Instruction::Invokestatic(interface).stack_delta(&pool)?,
            -255
        );
        assert!(
            Instruction::Invokevirtual(method)
                .stack_delta(&pool)
                .is_err()
        );
        assert!(
            Instruction::Invokespecial(interface)
                .stack_delta(&pool)
                .is_err()
        );
        assert!(
            Instruction::Invokeinterface(interface, 255)
                .stack_delta(&pool)
                .is_err()
        );
    }
    let descriptor = format!("({})J", "D".repeat(127));
    let method = pool.add_interface_method_ref(class, "m", &descriptor)?;
    assert_eq!(
        Instruction::Invokeinterface(method, 255).stack_delta(&pool)?,
        -253
    );
    for arguments in ["I".repeat(256), "J".repeat(128), "J".repeat(16_385)] {
        let descriptor = format!("({arguments})V");
        let method = pool.add_method_ref(class, "bad", &descriptor)?;
        assert!(FieldType::parse_method_descriptor(JavaStr::try_from_str(&descriptor)?).is_err());
        assert!(
            Instruction::Invokestatic(method)
                .stack_delta(&pool)
                .is_err()
        );
        assert!(
            [Instruction::Invokestatic(method), Instruction::Return]
                .max_stack(&pool)
                .is_err()
        );
    }
    Ok(())
}

#[test]
fn malformed_method_descriptors_are_rejected_without_panicking() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Calls")?;
    for descriptor in [
        "",
        "(",
        "()",
        "(I",
        "I)V",
        "(V)V",
        "([V)V",
        "(L;)V",
        "()VI",
        "()II",
        "()Ljava/lang/Object;I",
        "(I)Vextra",
    ] {
        let method = pool.add_method_ref(class, "bad", descriptor)?;
        assert!(
            FieldType::parse_method_descriptor(JavaStr::try_from_str(descriptor)?).is_err(),
            "{descriptor}"
        );
        assert!(
            Instruction::Invokestatic(method)
                .stack_delta(&pool)
                .is_err(),
            "{descriptor}"
        );
    }
    Ok(())
}

#[test]
fn array_descriptor_dimension_limit_and_reference_width() -> Result<()> {
    for dimensions in [1, 255] {
        let array = format!("{}J", "[".repeat(dimensions));
        assert_eq!(FieldType::parse(&array)?.slot_count(), 1);
        let descriptor = format!("({array}){array}");
        let (parameters, returned) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str(&descriptor)?)?;
        assert_eq!(parameters.len(), 1);
        assert_eq!(returned.map(|field| field.slot_count()), Some(1));
    }
    for dimensions in [256, 16_385] {
        let array = format!("{}J", "[".repeat(dimensions));
        assert!(FieldType::parse(&array).is_err());
        for descriptor in [format!("({array})V"), format!("(){array}")] {
            assert!(
                FieldType::parse_method_descriptor(JavaStr::try_from_str(&descriptor)?).is_err()
            );
        }
    }
    Ok(())
}

#[test]
fn multianewarray_validates_dimensions_and_descriptor() -> Result<()> {
    let mut pool = ConstantPool::new();
    let array = pool.add_class("[[[J")?;
    for dimensions in 1..=3 {
        assert_eq!(
            Instruction::Multianewarray(array, dimensions).stack_delta(&pool)?,
            1 - i16::from(dimensions)
        );
    }
    for dimensions in [0, 4, 255] {
        assert!(
            Instruction::Multianewarray(array, dimensions)
                .stack_delta(&pool)
                .is_err()
        );
    }
    let object = pool.add_class("java/lang/Object")?;
    assert!(
        Instruction::Multianewarray(object, 1)
            .stack_delta(&pool)
            .is_err()
    );
    assert!(
        Instruction::Multianewarray(0, 1)
            .stack_delta(&pool)
            .is_err()
    );
    let code = [
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Multianewarray(array, 2),
        Instruction::Pop,
        Instruction::Return,
    ];
    assert_eq!(code.max_stack(&pool)?, 2);
    Ok(())
}

#[test]
fn athrow_requires_control_flow_context() {
    assert_eq!(
        Instruction::Athrow.stack_delta(&ConstantPool::new()),
        Err(Error::InvalidStackEffect(191))
    );
}

#[test]
fn reserved_opcodes_and_standalone_wide_are_rejected() {
    for instruction in [
        Instruction::Wide,
        Instruction::Breakpoint,
        Instruction::Impdep1,
        Instruction::Impdep2,
    ] {
        assert!(instruction.stack_delta(&ConstantPool::new()).is_err());
        assert!(
            [instruction, Instruction::Return]
                .max_stack(&ConstantPool::new())
                .is_err()
        );
    }
}

#[test]
fn both_switch_forms_follow_default_and_case_targets() -> Result<()> {
    let switches = [
        Instruction::Tableswitch(Box::new(TableSwitch {
            default: 3,
            low: 0,
            high: 0,
            offsets: vec![1],
        })),
        Instruction::Lookupswitch(Box::new(LookupSwitch {
            default: 3,
            pairs: [(10, 1)].into_iter().collect(),
        })),
    ];
    for switch in switches {
        let code = [
            Instruction::Iload_0,
            switch,
            Instruction::Iconst_0,
            Instruction::Ireturn,
            Instruction::Iconst_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ];
        assert_eq!(code.max_stack(&ConstantPool::new())?, 2);
    }
    Ok(())
}

#[test]
fn switch_back_edge_preserves_stack_height() -> Result<()> {
    let switch = Instruction::Tableswitch(Box::new(TableSwitch {
        default: 1,
        low: 0,
        high: 0,
        offsets: vec![-1],
    }));
    let code = [Instruction::Iload_0, switch, Instruction::Return];
    assert_eq!(code.max_stack(&ConstantPool::new())?, 1);
    Ok(())
}

#[test]
fn incompatible_merges_and_unbalanced_loops_are_rejected() {
    for code in [
        vec![
            Instruction::Iconst_0,
            Instruction::Ifeq(4),
            Instruction::Iconst_1,
            Instruction::Goto(5),
            Instruction::Nop,
            Instruction::Return,
        ],
        vec![Instruction::Iconst_0, Instruction::Goto(0)],
        vec![
            Instruction::Iconst_0,
            Instruction::Pop,
            Instruction::Goto(1),
        ],
    ] {
        assert!(code.max_stack(&ConstantPool::new()).is_err(), "{code:?}");
    }
}

#[test]
fn invalid_branch_targets_and_fallthrough_are_rejected() {
    for code in [
        vec![Instruction::Goto(1)],
        vec![Instruction::Goto_w(-1)],
        vec![Instruction::Goto_w(i32::MAX)],
        vec![Instruction::Nop],
        vec![Instruction::Iconst_0, Instruction::Ifeq(0)],
        vec![
            Instruction::Iconst_0,
            Instruction::Ifeq(3),
            Instruction::Return,
        ],
        vec![
            Instruction::Iconst_0,
            Instruction::Tableswitch(Box::new(TableSwitch {
                default: -2,
                low: 0,
                high: 0,
                offsets: vec![1],
            })),
            Instruction::Return,
        ],
        vec![
            Instruction::Iconst_0,
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: 1,
                pairs: [(0, 20)].into_iter().collect(),
            })),
            Instruction::Return,
        ],
    ] {
        assert!(code.max_stack(&ConstantPool::new()).is_err(), "{code:?}");
    }
}

#[test]
fn stack_underflow_checks_consumed_operands_not_net_delta() {
    for code in [
        vec![Instruction::Pop, Instruction::Return],
        vec![
            Instruction::Iconst_0,
            Instruction::Iadd,
            Instruction::Return,
        ],
        vec![Instruction::Ineg, Instruction::Return],
        vec![Instruction::Dup, Instruction::Return],
        vec![
            Instruction::Iconst_0,
            Instruction::Pop2,
            Instruction::Return,
        ],
        vec![
            Instruction::Iconst_0,
            Instruction::Dup_x1,
            Instruction::Return,
        ],
        vec![
            Instruction::Lconst_0,
            Instruction::Ladd,
            Instruction::Return,
        ],
        vec![
            Instruction::Aload_0,
            Instruction::Iaload,
            Instruction::Return,
        ],
        vec![
            Instruction::Aload_0,
            Instruction::Iconst_0,
            Instruction::Iastore,
            Instruction::Return,
        ],
        vec![Instruction::Iconst_0, Instruction::Lreturn],
        vec![Instruction::Athrow],
        vec![
            Instruction::Iconst_0,
            Instruction::If_icmpeq(2),
            Instruction::Return,
        ],
    ] {
        assert!(code.max_stack(&ConstantPool::new()).is_err(), "{code:?}");
    }
}

#[test]
fn invocation_underflow_with_zero_or_positive_delta() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("Calls")?;
    for descriptor in ["(I)I", "(I)J", "(J)J"] {
        let method = pool.add_method_ref(class, "m", descriptor)?;
        assert!(
            [Instruction::Invokestatic(method), Instruction::Return]
                .max_stack(&pool)
                .is_err()
        );
    }
    let method = pool.add_method_ref(class, "m", "()J")?;
    assert!(
        [Instruction::Invokevirtual(method), Instruction::Return]
            .max_stack(&pool)
            .is_err()
    );
    Ok(())
}

#[test]
fn stack_u16_limit_is_checked_without_saturating() -> Result<()> {
    let mut code = vec![Instruction::Lconst_0; 32_767];
    code.extend([Instruction::Iconst_0, Instruction::Return]);
    assert_eq!(code.max_stack(&ConstantPool::new())?, u16::MAX);
    let mut overflow = vec![Instruction::Lconst_0; 32_768];
    overflow.push(Instruction::Return);
    assert!(overflow.max_stack(&ConstantPool::new()).is_err());
    let mut unreachable = vec![Instruction::Return];
    unreachable.extend(overflow);
    assert_eq!(unreachable.max_stack(&ConstantPool::new())?, 0);
    Ok(())
}

#[test]
fn exception_handler_contributes_its_initial_slot() -> Result<()> {
    let mut pool = ConstantPool::new();
    let class = pool.add_class("java/lang/System")?;
    let gc = pool.add_method_ref(class, "gc", "()V")?;
    let code = [
        Instruction::Invokestatic(gc),
        Instruction::Return,
        Instruction::Pop,
        Instruction::Return,
    ];
    let handlers = [ExceptionTableEntry {
        range_pc: 0..1,
        handler_pc: 2,
        catch_type: 0,
    }];
    assert_eq!(code.max_stack(&pool)?, 0);
    assert_eq!(code.max_stack_with_exception_table(&pool, &handlers)?, 1);
    Ok(())
}

#[test]
fn athrow_resets_handler_stack_and_handler_body_contributes_peak() -> Result<()> {
    let code = [
        Instruction::Iconst_0,
        Instruction::Aconst_null,
        Instruction::Athrow,
        Instruction::Iconst_0,
        Instruction::Iconst_1,
        Instruction::Iadd,
        Instruction::Pop,
        Instruction::Pop,
        Instruction::Return,
    ];
    let handlers = [ExceptionTableEntry {
        range_pc: 1..3,
        handler_pc: 3,
        catch_type: 0,
    }];
    assert_eq!(
        code.max_stack_with_exception_table(&ConstantPool::new(), &handlers)?,
        3
    );
    Ok(())
}

#[test]
fn handlers_can_rethrow_into_another_handler() -> Result<()> {
    let code = [
        Instruction::Aconst_null,
        Instruction::Athrow,
        Instruction::Athrow,
        Instruction::Lconst_0,
        Instruction::Pop2,
        Instruction::Pop,
        Instruction::Return,
    ];
    let handlers = [
        ExceptionTableEntry {
            range_pc: 0..2,
            handler_pc: 2,
            catch_type: 0,
        },
        ExceptionTableEntry {
            range_pc: 2..3,
            handler_pc: 3,
            catch_type: 0,
        },
    ];
    assert_eq!(
        code.max_stack_with_exception_table(&ConstantPool::new(), &handlers)?,
        3
    );
    Ok(())
}

#[test]
fn exception_handler_merges_must_have_compatible_heights() {
    let code = [
        Instruction::Iconst_0,
        Instruction::Pop,
        Instruction::Goto(3),
        Instruction::Return,
    ];
    let handlers = [ExceptionTableEntry {
        range_pc: 0..2,
        handler_pc: 3,
        catch_type: 0,
    }];
    assert!(
        code.max_stack_with_exception_table(&ConstantPool::new(), &handlers)
            .is_err()
    );
}

#[test]
fn malformed_exception_tables_are_rejected() {
    let code = [
        Instruction::Nop,
        Instruction::Return,
        Instruction::Pop,
        Instruction::Return,
    ];
    for (start, end, target) in [(0, 0, 2), (2, 1, 2), (0, 5, 2), (0, 1, 4), (4, 5, 2)] {
        let handlers = [ExceptionTableEntry {
            range_pc: start..end,
            handler_pc: target,
            catch_type: 0,
        }];
        assert!(
            code.max_stack_with_exception_table(&ConstantPool::new(), &handlers)
                .is_err()
        );
    }
    let handlers = [ExceptionTableEntry {
        range_pc: 0..1,
        handler_pc: 2,
        catch_type: 1,
    }];
    assert!(
        [].max_stack_with_exception_table(&ConstantPool::new(), &handlers)
            .is_err()
    );
}

#[test]
fn exception_range_can_end_at_code_length() -> Result<()> {
    let handlers = [ExceptionTableEntry {
        range_pc: 0..3,
        handler_pc: 2,
        catch_type: 0,
    }];
    assert_eq!(
        [
            Instruction::Aconst_null,
            Instruction::Athrow,
            Instruction::Athrow
        ]
        .max_stack_with_exception_table(&ConstantPool::new(), &handlers)?,
        1
    );
    Ok(())
}

#[test]
fn handler_preserves_legacy_return_address_locals() -> Result<()> {
    let code = [
        Instruction::Jsr(2),
        Instruction::Return,
        Instruction::Astore_0,
        Instruction::Aconst_null,
        Instruction::Athrow,
        Instruction::Pop,
        Instruction::Ret(0),
    ];
    let handlers = [ExceptionTableEntry {
        range_pc: 3..5,
        handler_pc: 5,
        catch_type: 0,
    }];
    assert_eq!(
        code.max_stack_with_exception_table(&ConstantPool::new(), &handlers)?,
        1
    );
    Ok(())
}

#[test]
fn invalid_legacy_returns_and_recursive_calls_are_rejected() {
    for code in [
        vec![
            Instruction::Jsr(6),
            Instruction::Iconst_0,
            Instruction::Jsr(6),
            Instruction::Pop,
            Instruction::Return,
            Instruction::Nop,
            Instruction::Astore_0,
            Instruction::Iconst_1,
            Instruction::Pop,
            Instruction::Ret(0),
        ],
        vec![Instruction::Ret(0)],
        vec![Instruction::Ret_w(300)],
        vec![Instruction::Jsr(0)],
        vec![Instruction::Jsr_w(-1), Instruction::Return],
        vec![
            Instruction::Jsr(2),
            Instruction::Return,
            Instruction::Astore_0,
            Instruction::Jsr(2),
            Instruction::Ret(0),
        ],
        vec![
            Instruction::Jsr(2),
            Instruction::Return,
            Instruction::Astore_0,
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Ret(0),
        ],
        vec![
            Instruction::Jsr(2),
            Instruction::Return,
            Instruction::Astore_1,
            Instruction::Lconst_0,
            Instruction::Lstore_0,
            Instruction::Ret(1),
        ],
        vec![
            Instruction::Jsr(2),
            Instruction::Return,
            Instruction::Astore_0,
            Instruction::Iload_0,
            Instruction::Ifeq(8),
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Goto(8),
            Instruction::Ret(0),
        ],
    ] {
        assert!(code.max_stack(&ConstantPool::new()).is_err(), "{code:?}");
    }
}
