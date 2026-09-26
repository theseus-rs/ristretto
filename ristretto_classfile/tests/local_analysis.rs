//! JVMS regressions for local-slot sizing, descriptors, and retained Code metadata.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with Result-returning helpers"
)]

use ristretto_classfile::attributes::{
    Attribute, Instruction, LocalVariableTable, LocalVariableTypeTable, MaxLocals, StackFrame,
    VerificationType,
};
use ristretto_classfile::{
    ConstantPool, Error, FieldType, JavaString, Method, MethodAccessFlags, Result,
};

fn calculate(descriptor: &str, flags: MethodAccessFlags, code: &[Instruction]) -> Result<u16> {
    let mut pool = ConstantPool::new();
    let method = Method {
        descriptor_index: pool.add_utf8(descriptor)?,
        access_flags: flags,
        ..Default::default()
    };
    code.max_locals(&pool, &method)
}

fn with_metadata(
    pool: &mut ConstantPool<'_>,
    descriptor: &str,
    flags: MethodAccessFlags,
    attributes: Vec<Attribute>,
) -> Result<Method> {
    Ok(Method {
        descriptor_index: pool.add_utf8(descriptor)?,
        access_flags: flags,
        attributes: vec![Attribute::Code {
            name_index: pool.add_utf8("Code")?,
            max_stack: 0,
            max_locals: 0,
            code: vec![Instruction::Return],
            exception_table: vec![],
            attributes,
        }],
        ..Default::default()
    })
}

fn frame_requirement(
    descriptor: &str,
    flags: MethodAccessFlags,
    frames: Vec<StackFrame>,
) -> Result<u16> {
    let mut pool = ConstantPool::new();
    let attribute = Attribute::StackMapTable {
        name_index: pool.add_utf8("StackMapTable")?,
        frames,
    };
    let method = with_metadata(&mut pool, descriptor, flags, vec![attribute])?;
    [Instruction::Return].max_locals(&pool, &method)
}

fn full_frame(locals: Vec<VerificationType>) -> StackFrame {
    StackFrame::FullFrame {
        frame_type: 255,
        offset_delta: 0,
        locals,
        stack: vec![],
    }
}

#[test]
fn wide_category_one_indices_and_capacity_boundary() -> Result<()> {
    for index in [0, 255, 256, 65533, 65534] {
        for instruction in [
            Instruction::Iload_w(index),
            Instruction::Fload_w(index),
            Instruction::Aload_w(index),
            Instruction::Istore_w(index),
            Instruction::Fstore_w(index),
            Instruction::Astore_w(index),
            Instruction::Iinc_w(index, i16::MIN),
            Instruction::Ret_w(index),
        ] {
            assert_eq!(
                instruction.max_locals_index()?,
                Some(index),
                "{instruction:?}"
            );
            assert_eq!(
                calculate(
                    "()V",
                    MethodAccessFlags::STATIC,
                    std::slice::from_ref(&instruction)
                )?,
                index + 1,
                "{instruction:?}"
            );
        }
    }
    Ok(())
}

#[test]
fn wide_category_one_overflow_is_an_error() {
    for instruction in [
        Instruction::Iload_w(65535),
        Instruction::Fload_w(65535),
        Instruction::Aload_w(65535),
        Instruction::Istore_w(65535),
        Instruction::Fstore_w(65535),
        Instruction::Astore_w(65535),
        Instruction::Iinc_w(65535, i16::MAX),
        Instruction::Ret_w(65535),
    ] {
        assert!(
            matches!(
                instruction.max_locals_index(),
                Err(Error::TryFromIntError(_))
            ),
            "{instruction:?}"
        );
        assert!(
            matches!(
                calculate(
                    "()V",
                    MethodAccessFlags::STATIC,
                    std::slice::from_ref(&instruction)
                ),
                Err(Error::TryFromIntError(_))
            ),
            "{instruction:?}"
        );
    }
}

#[test]
fn wide_category_two_indices_and_capacity_boundary() -> Result<()> {
    for index in [0, 255, 256, 65532, 65533] {
        for instruction in [
            Instruction::Lload_w(index),
            Instruction::Dload_w(index),
            Instruction::Lstore_w(index),
            Instruction::Dstore_w(index),
        ] {
            assert_eq!(
                instruction.max_locals_index()?,
                Some(index + 1),
                "{instruction:?}"
            );
            assert_eq!(
                calculate(
                    "()V",
                    MethodAccessFlags::STATIC,
                    std::slice::from_ref(&instruction)
                )?,
                index + 2,
                "{instruction:?}"
            );
        }
    }
    Ok(())
}

#[test]
fn wide_category_two_overflow_is_an_error() {
    for index in [65534, 65535] {
        for instruction in [
            Instruction::Lload_w(index),
            Instruction::Dload_w(index),
            Instruction::Lstore_w(index),
            Instruction::Dstore_w(index),
        ] {
            assert!(
                matches!(
                    instruction.max_locals_index(),
                    Err(Error::TryFromIntError(_))
                ),
                "{instruction:?}"
            );
            assert!(
                matches!(
                    calculate(
                        "()V",
                        MethodAccessFlags::STATIC,
                        std::slice::from_ref(&instruction)
                    ),
                    Err(Error::TryFromIntError(_))
                ),
                "{instruction:?}"
            );
        }
    }
}

#[test]
fn narrow_local_operands_can_require_more_than_255_slots() -> Result<()> {
    for index in [0, 1, 254, 255] {
        for instruction in [
            Instruction::Iload(index),
            Instruction::Fload(index),
            Instruction::Aload(index),
            Instruction::Istore(index),
            Instruction::Fstore(index),
            Instruction::Astore(index),
            Instruction::Iinc(index, -1),
            Instruction::Ret(index),
        ] {
            assert_eq!(instruction.max_locals_index()?, Some(u16::from(index)));
            assert_eq!(
                calculate("()V", MethodAccessFlags::STATIC, &[instruction])?,
                u16::from(index) + 1
            );
        }
        for instruction in [
            Instruction::Lload(index),
            Instruction::Dload(index),
            Instruction::Lstore(index),
            Instruction::Dstore(index),
        ] {
            assert_eq!(instruction.max_locals_index()?, Some(u16::from(index) + 1));
            assert_eq!(
                calculate("()V", MethodAccessFlags::STATIC, &[instruction])?,
                u16::from(index) + 2
            );
        }
    }
    Ok(())
}

#[test]
fn implicit_local_operands_count_both_category_two_slots() -> Result<()> {
    for (expected, instructions) in [
        (
            1,
            vec![
                Instruction::Iload_0,
                Instruction::Istore_0,
                Instruction::Fload_0,
                Instruction::Fstore_0,
                Instruction::Aload_0,
                Instruction::Astore_0,
            ],
        ),
        (
            2,
            vec![
                Instruction::Iload_1,
                Instruction::Istore_1,
                Instruction::Fload_1,
                Instruction::Fstore_1,
                Instruction::Aload_1,
                Instruction::Astore_1,
                Instruction::Lload_0,
                Instruction::Lstore_0,
                Instruction::Dload_0,
                Instruction::Dstore_0,
            ],
        ),
        (
            3,
            vec![
                Instruction::Iload_2,
                Instruction::Istore_2,
                Instruction::Fload_2,
                Instruction::Fstore_2,
                Instruction::Aload_2,
                Instruction::Astore_2,
                Instruction::Lload_1,
                Instruction::Lstore_1,
                Instruction::Dload_1,
                Instruction::Dstore_1,
            ],
        ),
        (
            4,
            vec![
                Instruction::Iload_3,
                Instruction::Istore_3,
                Instruction::Fload_3,
                Instruction::Fstore_3,
                Instruction::Aload_3,
                Instruction::Astore_3,
                Instruction::Lload_2,
                Instruction::Lstore_2,
                Instruction::Dload_2,
                Instruction::Dstore_2,
            ],
        ),
        (
            5,
            vec![
                Instruction::Lload_3,
                Instruction::Lstore_3,
                Instruction::Dload_3,
                Instruction::Dstore_3,
            ],
        ),
    ] {
        for instruction in instructions {
            assert_eq!(instruction.max_locals_index()?, Some(expected - 1));
            assert_eq!(
                calculate("()V", MethodAccessFlags::STATIC, &[instruction])?,
                expected
            );
        }
    }
    Ok(())
}

#[test]
fn unreachable_local_accesses_still_require_storage() -> Result<()> {
    assert_eq!(
        calculate(
            "()V",
            MethodAccessFlags::STATIC,
            &[Instruction::Return, Instruction::Lload_w(65533)]
        )?,
        65535
    );
    assert!(
        calculate(
            "()V",
            MethodAccessFlags::STATIC,
            &[Instruction::Return, Instruction::Iload_w(65535)]
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn nonlocal_instructions_do_not_allocate_locals() -> Result<()> {
    for instruction in [
        Instruction::Nop,
        Instruction::Iconst_0,
        Instruction::Lconst_0,
        Instruction::Dconst_0,
        Instruction::Iaload,
        Instruction::Lastore,
        Instruction::Return,
    ] {
        assert_eq!(instruction.max_locals_index()?, None);
        assert_eq!(
            calculate("()V", MethodAccessFlags::STATIC, &[instruction])?,
            0
        );
    }
    Ok(())
}

#[test]
fn static_parameter_slot_limit() -> Result<()> {
    for slots in [254, 255] {
        assert_eq!(
            calculate(
                &format!("({})V", "I".repeat(slots)),
                MethodAccessFlags::STATIC,
                &[]
            )?,
            u16::try_from(slots)?
        );
    }
    assert!(matches!(
        calculate(
            &format!("({})V", "I".repeat(256)),
            MethodAccessFlags::STATIC,
            &[]
        ),
        Err(Error::InvalidMethodDescriptor(_))
    ));
    Ok(())
}

#[test]
fn instance_parameter_slot_limit_includes_receiver() -> Result<()> {
    for flags in [
        MethodAccessFlags::empty(),
        MethodAccessFlags::PUBLIC,
        MethodAccessFlags::PRIVATE | MethodAccessFlags::SYNCHRONIZED,
    ] {
        assert_eq!(
            calculate(&format!("({})V", "I".repeat(254)), flags, &[])?,
            255
        );
        assert!(matches!(
            calculate(&format!("({})V", "I".repeat(255)), flags, &[]),
            Err(Error::InvalidMethodDescriptor(_))
        ));
    }
    Ok(())
}

#[test]
fn category_two_parameters_obey_slot_limit_not_argument_count() -> Result<()> {
    for wide in ["J", "D"] {
        let descriptor = format!("({})V", wide.repeat(127));
        assert_eq!(calculate(&descriptor, MethodAccessFlags::STATIC, &[])?, 254);
        assert_eq!(
            calculate(&descriptor, MethodAccessFlags::empty(), &[])?,
            255
        );
        let descriptor = format!("({}I)V", wide.repeat(127));
        assert_eq!(calculate(&descriptor, MethodAccessFlags::STATIC, &[])?, 255);
        assert!(matches!(
            calculate(&descriptor, MethodAccessFlags::empty(), &[]),
            Err(Error::InvalidMethodDescriptor(_))
        ));
        assert!(
            calculate(
                &format!("({})V", wide.repeat(128)),
                MethodAccessFlags::STATIC,
                &[]
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn arrays_of_category_two_parameters_use_one_slot() -> Result<()> {
    for array in ["[J", "[[D", "[[Ljava/lang/String;"] {
        assert_eq!(
            calculate(
                &format!("({})V", array.repeat(255)),
                MethodAccessFlags::STATIC,
                &[]
            )?,
            255
        );
        assert_eq!(
            calculate(
                &format!("({})V", array.repeat(254)),
                MethodAccessFlags::empty(),
                &[]
            )?,
            255
        );
        assert!(
            calculate(
                &format!("({})V", array.repeat(255)),
                MethodAccessFlags::empty(),
                &[]
            )
            .is_err()
        );
    }
    Ok(())
}

#[test]
fn parameter_limit_does_not_limit_body_locals() -> Result<()> {
    let descriptor = format!("({})V", "I".repeat(254));
    assert_eq!(
        calculate(
            &descriptor,
            MethodAccessFlags::empty(),
            &[Instruction::Iload_w(65534)]
        )?,
        65535
    );
    assert_eq!(
        calculate(
            "(ZBCSIJFD[DLjava/lang/Object;)J",
            MethodAccessFlags::STATIC,
            &[]
        )?,
        12
    );
    assert_eq!(
        calculate(
            "(ZBCSIJFD[DLjava/lang/Object;)D",
            MethodAccessFlags::empty(),
            &[]
        )?,
        13
    );
    Ok(())
}

#[test]
fn malformed_internal_names_are_rejected_in_all_descriptor_positions() {
    for name in [
        "",
        "java.lang.String",
        "/foo",
        "foo/",
        "foo//bar",
        "foo[bar",
        "foo/[bar",
        ".",
        "foo/../bar",
        "foo;bar",
    ] {
        for field in [
            format!("L{name};"),
            format!("[L{name};"),
            format!("[[L{name};"),
        ] {
            let java_field = JavaString::from(field.as_str());
            assert!(FieldType::parse(&field).is_err(), "{field}");
            assert!(FieldType::parse_java_str(&java_field).is_err(), "{field}");
            for descriptor in [
                format!("({field})V"),
                format!("(){field}"),
                format!("(I{field}J)V"),
            ] {
                let java_descriptor = JavaString::from(descriptor.as_str());
                assert!(
                    FieldType::parse_method_descriptor(&java_descriptor).is_err(),
                    "{descriptor}"
                );
                assert!(
                    calculate(&descriptor, MethodAccessFlags::STATIC, &[]).is_err(),
                    "{descriptor}"
                );
            }
        }
    }
}

#[test]
fn valid_internal_names_are_not_restricted_to_java_source_identifiers() -> Result<()> {
    for name in [
        "Foo",
        "java/lang/String",
        "pkg/Outer$Inner",
        "包/类型",
        "pkg/𐐀",
        "pkg/name-with-dashes",
        "pkg/<strange>",
        "1/2",
    ] {
        for field in [
            format!("L{name};"),
            format!("[L{name};"),
            format!("[[L{name};"),
        ] {
            let java_field = JavaString::from(field.as_str());
            assert_eq!(
                FieldType::parse(&field)?,
                FieldType::parse_java_str(&java_field)?,
                "{field}"
            );
            assert_eq!(
                calculate(&format!("({field}){field}"), MethodAccessFlags::STATIC, &[])?,
                1
            );
            assert_eq!(
                calculate(&format!("({field})V"), MethodAccessFlags::empty(), &[])?,
                2
            );
        }
    }
    Ok(())
}

#[test]
fn invalid_method_descriptor_pool_entries_are_errors() -> Result<()> {
    let mut pool = ConstantPool::new();
    let integer = pool.add_integer(1)?;
    for descriptor_index in [0, integer, u16::MAX] {
        let method = Method {
            descriptor_index,
            ..Default::default()
        };
        assert!([Instruction::Return].max_locals(&pool, &method).is_err());
    }
    Ok(())
}

fn debug_variable(pool: &mut ConstantPool<'_>, descriptor: &str, index: u16) -> Result<Attribute> {
    Ok(Attribute::LocalVariableTable {
        name_index: pool.add_utf8("LocalVariableTable")?,
        variables: vec![LocalVariableTable {
            start_pc: 0,
            length: 0,
            name_index: pool.add_utf8("unused")?,
            descriptor_index: pool.add_utf8(descriptor)?,
            index,
        }],
    })
}

#[test]
fn unused_debug_locals_count_even_with_empty_scope() -> Result<()> {
    for (descriptor, width) in [
        ("I", 1),
        ("J", 2),
        ("D", 2),
        ("Ljava/lang/Object;", 1),
        ("[J", 1),
        ("[[D", 1),
    ] {
        let mut pool = ConstantPool::new();
        let attribute = debug_variable(&mut pool, descriptor, 42)?;
        let method = with_metadata(&mut pool, "()V", MethodAccessFlags::STATIC, vec![attribute])?;
        assert_eq!(
            [Instruction::Return].max_locals(&pool, &method)?,
            42 + width
        );
    }
    Ok(())
}

#[test]
fn debug_local_capacity_boundaries() -> Result<()> {
    for (descriptor, last_index) in [("I", 65534), ("[D", 65534), ("J", 65533), ("D", 65533)] {
        for index in last_index..=65535 {
            let mut pool = ConstantPool::new();
            let attribute = debug_variable(&mut pool, descriptor, index)?;
            let method =
                with_metadata(&mut pool, "()V", MethodAccessFlags::STATIC, vec![attribute])?;
            let result = [Instruction::Return].max_locals(&pool, &method);
            if index == last_index {
                assert_eq!(result?, 65535);
            } else {
                assert!(matches!(result, Err(Error::TryFromIntError(_))));
            }
        }
    }
    Ok(())
}

#[test]
fn invalid_debug_descriptors_are_rejected() -> Result<()> {
    for descriptor in ["V", "JJ", "L;", "Ljava.lang.String;", "L/foo;", "[V", ""] {
        let mut pool = ConstantPool::new();
        let attribute = debug_variable(&mut pool, descriptor, 0)?;
        let method = with_metadata(&mut pool, "()V", MethodAccessFlags::STATIC, vec![attribute])?;
        assert!(
            [Instruction::Return].max_locals(&pool, &method).is_err(),
            "{descriptor}"
        );
    }
    Ok(())
}

#[test]
fn invalid_debug_descriptor_pool_entries_are_errors() -> Result<()> {
    let mut pool = ConstantPool::new();
    let integer = pool.add_integer(1)?;
    for descriptor_index in [0, integer, u16::MAX] {
        let attribute = Attribute::LocalVariableTable {
            name_index: pool.add_utf8("LocalVariableTable")?,
            variables: vec![LocalVariableTable {
                start_pc: 0,
                length: 0,
                name_index: 0,
                descriptor_index,
                index: 0,
            }],
        };
        let method = with_metadata(&mut pool, "()V", MethodAccessFlags::STATIC, vec![attribute])?;
        assert!([Instruction::Return].max_locals(&pool, &method).is_err());
    }
    Ok(())
}

#[test]
fn generic_debug_locals_use_reference_slots_and_check_capacity() -> Result<()> {
    for signature in [
        "TT;",
        "Ljava/util/List<Ljava/lang/String;>;",
        "[TT;",
        "[J",
        "[[D",
    ] {
        for index in [0, 42, 65534, 65535] {
            let mut pool = ConstantPool::new();
            let attribute = Attribute::LocalVariableTypeTable {
                name_index: pool.add_utf8("LocalVariableTypeTable")?,
                variable_types: vec![LocalVariableTypeTable {
                    start_pc: 0,
                    length: 0,
                    name_index: pool.add_utf8("generic")?,
                    signature_index: pool.add_utf8(signature)?,
                    index,
                }],
            };
            let method =
                with_metadata(&mut pool, "()V", MethodAccessFlags::STATIC, vec![attribute])?;
            let result = [Instruction::Return].max_locals(&pool, &method);
            if index == 65535 {
                assert!(matches!(result, Err(Error::TryFromIntError(_))));
            } else {
                assert_eq!(result?, index + 1);
            }
        }
    }
    Ok(())
}

#[test]
fn full_frames_count_top_and_all_verification_types() -> Result<()> {
    let frame = full_frame(vec![
        VerificationType::Top,
        VerificationType::Integer,
        VerificationType::Float,
        VerificationType::Long,
        VerificationType::Double,
        VerificationType::Null,
        VerificationType::UninitializedThis,
        VerificationType::Object { cpool_index: 1 },
        VerificationType::Uninitialized { offset: 0 },
    ]);
    assert_eq!(
        frame_requirement("()V", MethodAccessFlags::STATIC, vec![frame])?,
        11
    );
    assert_eq!(
        frame_requirement(
            "()V",
            MethodAccessFlags::STATIC,
            vec![full_frame(vec![VerificationType::Top; 3])]
        )?,
        3
    );
    Ok(())
}

#[test]
fn append_frames_start_with_receiver_and_parameter_entries() -> Result<()> {
    let frame = StackFrame::AppendFrame {
        frame_type: 254,
        offset_delta: 0,
        locals: vec![
            VerificationType::Long,
            VerificationType::Double,
            VerificationType::Top,
        ],
    };
    assert_eq!(
        frame_requirement("(IJD[J)V", MethodAccessFlags::STATIC, vec![frame.clone()])?,
        11
    );
    assert_eq!(
        frame_requirement("(IJD[J)V", MethodAccessFlags::empty(), vec![frame])?,
        12
    );
    Ok(())
}

#[test]
fn chop_frames_remove_entries_instead_of_slots() -> Result<()> {
    let frames = vec![
        StackFrame::ChopFrame {
            frame_type: 249,
            offset_delta: 0,
        },
        StackFrame::AppendFrame {
            frame_type: 254,
            offset_delta: 0,
            locals: vec![VerificationType::Double; 3],
        },
    ];
    assert_eq!(
        frame_requirement("(IJD)V", MethodAccessFlags::STATIC, frames.clone())?,
        7
    );
    assert_eq!(
        frame_requirement("(IJD)V", MethodAccessFlags::empty(), frames)?,
        8
    );
    Ok(())
}

#[test]
fn chop_frames_remove_one_two_or_three_entries() -> Result<()> {
    for (frame_type, expected) in [(250, 10), (249, 8), (248, 7)] {
        let frames = vec![
            full_frame(vec![
                VerificationType::Top,
                VerificationType::Integer,
                VerificationType::Long,
                VerificationType::Double,
            ]),
            StackFrame::ChopFrame {
                frame_type,
                offset_delta: 0,
            },
            StackFrame::AppendFrame {
                frame_type: 254,
                offset_delta: 0,
                locals: vec![VerificationType::Long; 3],
            },
        ];
        assert_eq!(
            frame_requirement("()V", MethodAccessFlags::STATIC, frames)?,
            expected
        );
    }
    Ok(())
}

#[test]
fn full_frames_replace_locals_and_preserve_earlier_peak() -> Result<()> {
    let frames = vec![
        full_frame(vec![VerificationType::Top; 9]),
        full_frame(vec![VerificationType::Long]),
        StackFrame::AppendFrame {
            frame_type: 254,
            offset_delta: 0,
            locals: vec![VerificationType::Double; 3],
        },
    ];
    assert_eq!(
        frame_requirement("(I)V", MethodAccessFlags::empty(), frames)?,
        9
    );
    assert_eq!(
        frame_requirement(
            "(JD)V",
            MethodAccessFlags::empty(),
            vec![full_frame(vec![])]
        )?,
        5
    );
    Ok(())
}

#[test]
fn same_frames_preserve_locals_without_counting_operand_stack() -> Result<()> {
    for same in [
        StackFrame::SameFrame { frame_type: 0 },
        StackFrame::SameFrame { frame_type: 63 },
        StackFrame::SameFrameExtended {
            frame_type: 251,
            offset_delta: 0,
        },
        StackFrame::SameLocals1StackItemFrame {
            frame_type: 64,
            stack: vec![VerificationType::Double],
        },
        StackFrame::SameLocals1StackItemFrame {
            frame_type: 127,
            stack: vec![VerificationType::Long],
        },
        StackFrame::SameLocals1StackItemFrameExtended {
            frame_type: 247,
            offset_delta: 0,
            stack: vec![VerificationType::Long],
        },
    ] {
        let frames = vec![
            full_frame(vec![VerificationType::Long, VerificationType::Top]),
            same,
            StackFrame::AppendFrame {
                frame_type: 252,
                offset_delta: 0,
                locals: vec![VerificationType::Double],
            },
        ];
        assert_eq!(
            frame_requirement("()V", MethodAccessFlags::STATIC, frames)?,
            5
        );
    }
    let frame = StackFrame::FullFrame {
        frame_type: 255,
        offset_delta: 0,
        locals: vec![],
        stack: vec![VerificationType::Long, VerificationType::Double],
    };
    assert_eq!(
        frame_requirement("()V", MethodAccessFlags::STATIC, vec![frame])?,
        0
    );
    Ok(())
}

#[test]
fn full_frame_capacity_boundaries() -> Result<()> {
    assert_eq!(
        frame_requirement(
            "()V",
            MethodAccessFlags::STATIC,
            vec![full_frame(vec![VerificationType::Top; 65535])]
        )?,
        65535
    );
    assert!(matches!(
        frame_requirement(
            "()V",
            MethodAccessFlags::STATIC,
            vec![full_frame(vec![VerificationType::Top; 65536])]
        ),
        Err(Error::TryFromIntError(_))
    ));
    for wide in [VerificationType::Long, VerificationType::Double] {
        let mut locals = vec![wide.clone(); 32767];
        locals.push(VerificationType::Top);
        assert_eq!(
            frame_requirement("()V", MethodAccessFlags::STATIC, vec![full_frame(locals)])?,
            65535
        );
        assert!(matches!(
            frame_requirement(
                "()V",
                MethodAccessFlags::STATIC,
                vec![full_frame(vec![wide; 32768])]
            ),
            Err(Error::TryFromIntError(_))
        ));
    }
    Ok(())
}

#[test]
fn append_frame_capacity_boundaries_and_transient_overflow() -> Result<()> {
    for (local, initial) in [
        (VerificationType::Integer, 65534),
        (VerificationType::Long, 65533),
        (VerificationType::Double, 65533),
    ] {
        let frames = vec![
            full_frame(vec![VerificationType::Top; initial]),
            StackFrame::AppendFrame {
                frame_type: 252,
                offset_delta: 0,
                locals: vec![local.clone()],
            },
        ];
        assert_eq!(
            frame_requirement("()V", MethodAccessFlags::STATIC, frames)?,
            65535
        );
        let frames = vec![
            full_frame(vec![VerificationType::Top; initial + 1]),
            StackFrame::AppendFrame {
                frame_type: 252,
                offset_delta: 0,
                locals: vec![local],
            },
            StackFrame::ChopFrame {
                frame_type: 250,
                offset_delta: 0,
            },
        ];
        assert!(matches!(
            frame_requirement("()V", MethodAccessFlags::STATIC, frames),
            Err(Error::TryFromIntError(_))
        ));
    }
    Ok(())
}

#[test]
fn malformed_frame_kinds_and_append_counts_are_errors() {
    for frame in [
        StackFrame::SameFrame { frame_type: 64 },
        StackFrame::SameFrameExtended {
            frame_type: 250,
            offset_delta: 0,
        },
        StackFrame::SameLocals1StackItemFrame {
            frame_type: 63,
            stack: vec![VerificationType::Integer],
        },
        StackFrame::SameLocals1StackItemFrameExtended {
            frame_type: 246,
            offset_delta: 0,
            stack: vec![VerificationType::Integer],
        },
        StackFrame::ChopFrame {
            frame_type: 247,
            offset_delta: 0,
        },
        StackFrame::ChopFrame {
            frame_type: 251,
            offset_delta: 0,
        },
        StackFrame::AppendFrame {
            frame_type: 251,
            offset_delta: 0,
            locals: vec![],
        },
        StackFrame::AppendFrame {
            frame_type: 255,
            offset_delta: 0,
            locals: vec![VerificationType::Top; 4],
        },
        StackFrame::AppendFrame {
            frame_type: 252,
            offset_delta: 0,
            locals: vec![],
        },
        StackFrame::AppendFrame {
            frame_type: 253,
            offset_delta: 0,
            locals: vec![VerificationType::Top],
        },
        StackFrame::AppendFrame {
            frame_type: 254,
            offset_delta: 0,
            locals: vec![VerificationType::Top; 4],
        },
        StackFrame::FullFrame {
            frame_type: 254,
            offset_delta: 0,
            locals: vec![],
            stack: vec![],
        },
    ] {
        assert!(matches!(
            frame_requirement("()V", MethodAccessFlags::STATIC, vec![frame]),
            Err(Error::InvalidStackFrameType(_))
        ));
    }
}

#[test]
fn chop_frame_underflow_is_an_error() {
    for (descriptor, frame_type) in [("()V", 250), ("(J)V", 249), ("(JD)V", 248)] {
        assert!(matches!(
            frame_requirement(
                descriptor,
                MethodAccessFlags::STATIC,
                vec![StackFrame::ChopFrame {
                    frame_type,
                    offset_delta: 0
                }]
            ),
            Err(Error::VerificationError(_))
        ));
    }
}

#[test]
fn independent_debug_tables_and_frames_contribute_their_maximum() -> Result<()> {
    let mut pool = ConstantPool::new();
    let first = debug_variable(&mut pool, "J", 30)?;
    let second = debug_variable(&mut pool, "I", 4)?;
    let frames = Attribute::StackMapTable {
        name_index: pool.add_utf8("StackMapTable")?,
        frames: vec![full_frame(vec![VerificationType::Top; 20])],
    };
    let method = with_metadata(
        &mut pool,
        "(D)V",
        MethodAccessFlags::empty(),
        vec![first, frames, second],
    )?;
    assert_eq!([Instruction::Return].max_locals(&pool, &method)?, 32);
    assert_eq!([Instruction::Iload_w(40)].max_locals(&pool, &method)?, 41);
    Ok(())
}

#[test]
fn old_code_and_declared_capacity_do_not_override_replacement_instructions() -> Result<()> {
    let mut pool = ConstantPool::new();
    let method = Method {
        descriptor_index: pool.add_utf8("()V")?,
        access_flags: MethodAccessFlags::STATIC,
        attributes: vec![
            Attribute::Deprecated {
                name_index: pool.add_utf8("Deprecated")?,
            },
            Attribute::Code {
                name_index: pool.add_utf8("Code")?,
                max_stack: 1,
                max_locals: 65535,
                code: vec![Instruction::Iload_w(65534), Instruction::Return],
                exception_table: vec![],
                attributes: vec![Attribute::LineNumberTable {
                    name_index: pool.add_utf8("LineNumberTable")?,
                    line_numbers: vec![],
                }],
            },
        ],
        ..Default::default()
    };
    assert_eq!([Instruction::Return].max_locals(&pool, &method)?, 0);
    assert_eq!([Instruction::Lstore(10)].max_locals(&pool, &method)?, 12);
    Ok(())
}
