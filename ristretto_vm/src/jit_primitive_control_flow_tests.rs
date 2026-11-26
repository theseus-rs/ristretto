//! Finite primitive control flow through the production compiler and dispatcher.
//! No test cache injection: a compiled function must be selected by normal execution.
use super::*;
use indexmap::IndexMap;
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{Instruction, LookupSwitch, TableSwitch};
use static_call_tests::{class_from_methods, method, register};

#[path = "jit_cooperation_tests.rs"]
mod cooperation;

#[path = "jit_reference_cooperation_tests.rs"]
mod reference_cooperation;

type Predicate = (fn(i16) -> Instruction, fn(i32, i32) -> bool);

struct Harness {
    vm: Arc<VM>,
    thread: Arc<Thread>,
    interpreted_vm: Arc<VM>,
    interpreted_thread: Arc<Thread>,
}

impl Harness {
    async fn new() -> Result<Self> {
        let (vm, thread) = crate::test::thread().await?;
        let interpreted_vm = VM::new(
            crate::ConfigurationBuilder::new()
                .interpreted(true)
                .build()?,
        )
        .await?;
        let interpreted_thread = Thread::new(&Arc::downgrade(&interpreted_vm), 3);
        Ok(Self {
            vm,
            thread,
            interpreted_vm,
            interpreted_thread,
        })
    }

    async fn check(
        &self,
        name: &str,
        descriptor: &str,
        code: Vec<Instruction>,
        cases: Vec<(Vec<Value>, Value)>,
    ) -> Result<()> {
        let mut pool = ConstantPool::new();
        let definition = method(&mut pool, "run", descriptor, code)?;
        let class = class_from_methods(name, "java/lang/Object", pool, vec![definition])?;
        let class = register(&self.thread, class).await?;
        let method = class.try_get_method("run", descriptor)?;
        assert!(
            ristretto_jit::Compiler::can_compile(method.definition()),
            "production gate rejected {name}"
        );
        for (parameters, expected) in &cases {
            let interpreted = self
                .interpreted_thread
                .execute(&class, &method, parameters)
                .await?;
            let compiled = self.thread.execute(&class, &method, parameters).await?;
            assert_eq!(interpreted, Some(expected.clone()), "interpreter {name}");
            assert_eq!(compiled, interpreted, "production JIT {name}");
        }
        let function = self
            .vm
            .compiler()
            .ok_or_else(|| InternalError("missing production compiler".to_owned()))?
            .get_cached(cache_key(&method))
            .flatten()
            .ok_or_else(|| InternalError(format!("missing production compilation for {name}")))?;
        #[cfg(feature = "execution-profile")]
        {
            let calls = u64::try_from(cases.len())?;
            assert_eq!(function.execution_count(), calls, "{name}");
            assert_eq!(function.backedge_count(), 0, "{name}");
            for (vm, compiled, interpreted) in
                [(&self.vm, calls, 0), (&self.interpreted_vm, 0, calls)]
            {
                let profile = vm.execution_profile();
                let record = profile
                    .methods
                    .iter()
                    .find(|record| record.class == name && record.name == "run")
                    .ok_or_else(|| InternalError(format!("missing execution record for {name}")))?;
                assert_eq!(record.compiled_entries, compiled, "{name}");
                assert_eq!(record.interpreter_entries, interpreted, "{name}");
            }
        }
        // These owners also keep both VM contexts alive in uninstrumented builds.
        drop(function);
        assert!(self.interpreted_vm.configuration().interpreted());
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn all_integer_conditions_execute_through_the_production_gate() -> Result<()> {
    let harness = Harness::new().await?;
    let zero: [Predicate; 6] = [
        (Instruction::Ifeq, |a, b| a == b),
        (Instruction::Ifne, |a, b| a != b),
        (Instruction::Iflt, |a, b| a < b),
        (Instruction::Ifge, |a, b| a >= b),
        (Instruction::Ifgt, |a, b| a > b),
        (Instruction::Ifle, |a, b| a <= b),
    ];
    let pair: [Predicate; 6] = [
        (Instruction::If_icmpeq, |a, b| a == b),
        (Instruction::If_icmpne, |a, b| a != b),
        (Instruction::If_icmplt, |a, b| a < b),
        (Instruction::If_icmpge, |a, b| a >= b),
        (Instruction::If_icmpgt, |a, b| a > b),
        (Instruction::If_icmple, |a, b| a <= b),
    ];
    let inputs = [i32::MIN, -1, 0, 1, i32::MAX];
    for (binary, predicates) in [(false, zero), (true, pair)] {
        for (index, (branch, predicate)) in predicates.into_iter().enumerate() {
            let mut code = vec![Instruction::Iload_0];
            if binary {
                code.push(Instruction::Iload_1);
            }
            code.extend([
                branch(7),
                Instruction::Iconst_0,
                Instruction::Goto(4),
                Instruction::Iconst_1,
                Instruction::Ireturn,
            ]);
            let mut cases = Vec::new();
            for a in inputs {
                for b in if binary { inputs.as_slice() } else { &[0] } {
                    let mut parameters = vec![Value::Int(a)];
                    if binary {
                        parameters.push(Value::Int(*b));
                    }
                    cases.push((parameters, Value::Int(i32::from(predicate(a, *b)))));
                }
            }
            harness
                .check(
                    &format!(
                        "Forward{}Condition{index}",
                        if binary { "Pair" } else { "Zero" }
                    ),
                    if binary { "(II)I" } else { "(I)I" },
                    code,
                    cases,
                )
                .await?;
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forward_gotos_merge_primitive_locals() -> Result<()> {
    let harness = Harness::new().await?;
    for wide in [false, true] {
        let code = vec![
            Instruction::Iload_0,
            Instruction::Ifeq(if wide { 11 } else { 9 }),
            Instruction::Iload_1,
            Instruction::Ineg,
            Instruction::Istore_2,
            if wide {
                Instruction::Goto_w(7)
            } else {
                Instruction::Goto(5)
            },
            Instruction::Iload_1,
            Instruction::Istore_2,
            Instruction::Iload_2,
            Instruction::Ireturn,
        ];
        let mut cases = Vec::new();
        for branch in [-1, 0, 1] {
            for value in [i32::MIN, -1, 0, 1, i32::MAX] {
                cases.push((
                    vec![Value::Int(branch), Value::Int(value)],
                    Value::Int(if branch == 0 {
                        value
                    } else {
                        value.wrapping_neg()
                    }),
                ));
            }
        }
        harness
            .check(
                if wide {
                    "ForwardWideGoto"
                } else {
                    "ForwardGoto"
                },
                "(II)I",
                code,
                cases,
            )
            .await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forward_switches_preserve_operands_and_signed_extremes() -> Result<()> {
    let harness = Harness::new().await?;
    for (index, low) in [i32::MIN, -1, i32::MAX - 2].into_iter().enumerate() {
        let mut code = vec![
            Instruction::Iconst_5,
            Instruction::Iload_0,
            Instruction::Tableswitch(Box::new(TableSwitch {
                low,
                high: low + 2,
                offsets: vec![26, 29, 32],
                default: 35,
            })),
        ];
        for value in 1..=4 {
            code.extend([
                match value {
                    1 => Instruction::Iconst_1,
                    2 => Instruction::Iconst_2,
                    3 => Instruction::Iconst_3,
                    _ => Instruction::Iconst_4,
                },
                Instruction::Iadd,
                Instruction::Ireturn,
            ]);
        }
        let cases = [i32::MIN, -2, -1, 0, 1, 2, i32::MAX, low, low + 1, low + 2]
            .into_iter()
            .map(|key| {
                let expected = if (low..=low + 2).contains(&key) {
                    6 + (key - low)
                } else {
                    9
                };
                (vec![Value::Int(key)], Value::Int(expected))
            })
            .collect();
        harness
            .check(&format!("ForwardTable{index}"), "(I)I", code, cases)
            .await?;
    }
    for empty in [false, true] {
        let mut code = vec![
            Instruction::Iconst_5,
            Instruction::Iload_0,
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: if empty { 10 } else { 40 },
                pairs: if empty {
                    IndexMap::new()
                } else {
                    IndexMap::from([(i32::MIN, 34), (-1, 37), (i32::MAX, 34)])
                },
            })),
        ];
        for instruction in if empty {
            vec![Instruction::Iconst_1]
        } else {
            vec![
                Instruction::Iconst_1,
                Instruction::Iconst_2,
                Instruction::Iconst_3,
            ]
        } {
            code.extend([instruction, Instruction::Iadd, Instruction::Ireturn]);
        }
        let cases = [i32::MIN, -2, -1, 0, 1, 2, i32::MAX]
            .into_iter()
            .map(|key| {
                let expected = if empty || key == i32::MIN || key == i32::MAX {
                    6
                } else if key == -1 {
                    7
                } else {
                    8
                };
                (vec![Value::Int(key)], Value::Int(expected))
            })
            .collect();
        harness
            .check(
                if empty {
                    "ForwardEmptyLookup"
                } else {
                    "ForwardLookup"
                },
                "(I)I",
                code,
                cases,
            )
            .await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forward_merges_preserve_category_two_operands_and_reused_locals() -> Result<()> {
    let harness = Harness::new().await?;
    for double in [false, true] {
        let code = vec![
            if double {
                Instruction::Dconst_1
            } else {
                Instruction::Lconst_1
            },
            Instruction::Iload_0,
            Instruction::Ifeq(7),
            if double {
                Instruction::Dconst_0
            } else {
                Instruction::Lconst_0
            },
            Instruction::Goto(4),
            if double {
                Instruction::Dconst_1
            } else {
                Instruction::Lconst_1
            },
            if double {
                Instruction::Dadd
            } else {
                Instruction::Ladd
            },
            if double {
                Instruction::Dreturn
            } else {
                Instruction::Lreturn
            },
        ];
        let cases = [-1, 0, 1]
            .into_iter()
            .map(|key| {
                let result = if key == 0 { 2 } else { 1 };
                (
                    vec![Value::Int(key)],
                    if double {
                        Value::Double(f64::from(result))
                    } else {
                        Value::Long(i64::from(result))
                    },
                )
            })
            .collect();
        harness
            .check(
                if double {
                    "ForwardDoubleMerge"
                } else {
                    "ForwardLongMerge"
                },
                if double { "(I)D" } else { "(I)J" },
                code,
                cases,
            )
            .await?;
    }
    harness
        .check(
            "ForwardLocalReuse",
            "(I)I",
            vec![
                Instruction::Lconst_1, // 0
                Instruction::Lstore_1, // 1: two slots subsequently reused as float
                Instruction::Iload_0,  // 2
                Instruction::Ifeq(8),  // 3 -> 11
                Instruction::Fconst_1, // 6
                Instruction::Fstore_1, // 7
                Instruction::Goto(5),  // 8 -> 13
                Instruction::Fconst_2, // 11
                Instruction::Fstore_1, // 12
                Instruction::Fload_1,  // 13
                Instruction::F2i,
                Instruction::Ireturn,
            ],
            [-1, 0, 1]
                .into_iter()
                .map(|key| {
                    (
                        vec![Value::Int(key)],
                        Value::Int(if key == 0 { 2 } else { 1 }),
                    )
                })
                .collect(),
        )
        .await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forward_comparisons_preserve_long_extremes_nan_and_signed_zero() -> Result<()> {
    let harness = Harness::new().await?;
    let mut code = vec![
        Instruction::Lload_0,
        Instruction::Lload_2,
        Instruction::Lcmp,
    ];
    code.extend([
        Instruction::Ifge(7),
        Instruction::Iconst_0,
        Instruction::Goto(4),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ]);
    let mut cases = Vec::new();
    for a in [i64::MIN, -1, 0, 1, i64::MAX] {
        for b in [i64::MIN, -1, 0, 1, i64::MAX] {
            cases.push((
                vec![Value::Long(a), Value::Long(b)],
                Value::Int(i32::from(a >= b)),
            ));
        }
    }
    harness
        .check("ForwardLongCompare", "(JJ)I", code, cases)
        .await?;
    for double in [false, true] {
        let mut code = if double {
            vec![
                Instruction::Dload_0,
                Instruction::Dload_2,
                Instruction::Dcmpl,
                Instruction::Ifgt(7),
            ]
        } else {
            vec![
                Instruction::Fload_0,
                Instruction::Fload_1,
                Instruction::Fcmpg,
                Instruction::Iflt(7),
            ]
        };
        code.extend([
            Instruction::Iconst_0,
            Instruction::Goto(4),
            Instruction::Iconst_1,
            Instruction::Ireturn,
        ]);
        let mut cases = Vec::new();
        let values = [
            f64::NEG_INFINITY,
            -1.0,
            -0.0,
            0.0,
            1.0,
            f64::INFINITY,
            f64::NAN,
        ];
        for a in values {
            for b in values {
                let (parameters, expected) = if double {
                    (vec![Value::Double(a), Value::Double(b)], a > b)
                } else {
                    #[expect(
                        clippy::cast_possible_truncation,
                        reason = "all fixture values are exactly representable in f32"
                    )]
                    let (a, b) = (a as f32, b as f32);
                    (vec![Value::Float(a), Value::Float(b)], a < b)
                };
                cases.push((parameters, Value::Int(i32::from(expected))));
            }
        }
        harness
            .check(
                if double {
                    "ForwardDoubleCompare"
                } else {
                    "ForwardFloatCompare"
                },
                if double { "(DD)I" } else { "(FF)I" },
                code,
                cases,
            )
            .await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn primitive_loops_cooperate_through_the_production_gate() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let mut pool = ConstantPool::new();
    let definition = method(
        &mut pool,
        "run",
        "(I)I",
        vec![
            Instruction::Iload_0,     // 0
            Instruction::Ifeq(9),     // 1 -> 10
            Instruction::Iinc(0, -1), // 4
            Instruction::Goto(-7),    // 7 -> 0
            Instruction::Iload_0,     // 10
            Instruction::Ireturn,
        ],
    )?;
    assert!(ristretto_jit::Compiler::can_compile(&definition));
    let class = register(
        &thread,
        class_from_methods("PrimitiveLoop", "java/lang/Object", pool, vec![definition])?,
    )
    .await?;
    let method = class.try_get_method("run", "(I)I")?;
    for input in [0, 100_000] {
        assert_eq!(
            thread
                .execute(&class, &method, &[Value::Int(input)])
                .await?,
            Some(Value::Int(0))
        );
    }
    assert!(
        vm.compiler()
            .expect("compiler")
            .get_cached(cache_key(&method))
            .expect("cached decision")
            .is_some_and(|function| function.supports_cooperation())
    );
    #[cfg(feature = "execution-profile")]
    {
        let profile = vm.execution_profile();
        let record = profile
            .methods
            .iter()
            .find(|record| record.class == "PrimitiveLoop")
            .expect("loop record");
        assert_eq!(record.compiled_entries, 2);
        assert_eq!(record.interpreter_entries, 0);
        assert!(record.compiled_resumes > 0);
        assert_eq!(record.compiled_backedges, 100_000);
        assert_eq!(record.rejection_reason, None);
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ordinary_dispatch_compiles_a_volatile_static_long_store() -> Result<()> {
    use ristretto_classfile::{Field, FieldAccessFlags, FieldType};

    let harness = Harness::new().await?;
    let name = "ProductionLongStore";
    let mut pool = ConstantPool::new();
    let owner = pool.add_class(name)?;
    let slot = pool.add_field_ref(owner, "value", "J")?;
    let body = method(
        &mut pool,
        "publish",
        "(J)J",
        vec![
            Instruction::Lload_0,
            Instruction::Putstatic(slot),
            Instruction::Lload_0,
            Instruction::Lreturn,
        ],
    )?;
    let field = Field {
        access_flags: FieldAccessFlags::PUBLIC
            | FieldAccessFlags::STATIC
            | FieldAccessFlags::VOLATILE,
        name_index: pool.add_utf8("value")?,
        descriptor_index: pool.add_utf8("J")?,
        field_type: FieldType::parse("J")?,
        attributes: vec![],
    };
    let mut definition = class_from_methods(name, "java/lang/Object", pool, vec![body])?
        .class_file()
        .clone();
    definition.fields = vec![field];
    let class = register(&harness.thread, Class::from(None, definition)?).await?;
    let publish = class.try_get_method("publish", "(J)J")?;
    for bits in [i64::MIN, 0x7abc_fedc_8765_4321, -1] {
        for thread in [&harness.interpreted_thread, &harness.thread] {
            class.set_static_value("value", Value::Long(!bits))?;
            assert_eq!(
                thread
                    .execute(&class, &publish, &[Value::Long(bits)])
                    .await?,
                Some(Value::Long(bits))
            );
            assert_eq!(class.static_value("value")?, Value::Long(bits));
        }
    }
    let function = harness
        .vm
        .compiler()
        .and_then(|compiler| compiler.get_cached(cache_key(&publish)))
        .flatten()
        .ok_or_else(|| {
            InternalError("ordinary volatile long store ran only in the interpreter".into())
        })?;
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 3);
        let profile = harness.vm.execution_profile();
        let record = profile
            .methods
            .iter()
            .find(|record| record.class == name && record.name == "publish")
            .ok_or_else(|| InternalError("missing production store record".into()))?;
        assert_eq!(
            (record.compiled_entries, record.interpreter_entries),
            (3, 0)
        );
        assert!(record.rejected_bytecode.is_none());
    }
    drop(function);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ordinary_static_long_store_reports_a_missing_field_from_compiled_code() -> Result<()> {
    let harness = Harness::new().await?;
    let name = "ProductionMissingLongStore";
    let mut pool = ConstantPool::new();
    let owner = pool.add_class(name)?;
    let absent = pool.add_field_ref(owner, "absent", "J")?;
    let body = method(
        &mut pool,
        "publish",
        "(J)V",
        vec![
            Instruction::Lload_0,
            Instruction::Putstatic(absent),
            Instruction::Return,
        ],
    )?;
    let class = register(
        &harness.thread,
        class_from_methods(name, "java/lang/Object", pool, vec![body])?,
    )
    .await?;
    let publish = class.try_get_method("publish", "(J)V")?;
    for thread in [&harness.interpreted_thread, &harness.thread] {
        let result = thread
            .execute(&class, &publish, &[Value::Long(0x1234_5678_9abc)])
            .await;
        let Err(crate::Error::Throwable(value)) = result else {
            return Err(InternalError(format!("missing field produced {result:?}")));
        };
        assert_eq!(
            value.as_object_ref()?.class().name(),
            "java/lang/NoSuchFieldError"
        );
    }
    let function = harness
        .vm
        .compiler()
        .and_then(|compiler| compiler.get_cached(cache_key(&publish)))
        .flatten()
        .ok_or_else(|| {
            InternalError("the missing field store ran only in the interpreter".into())
        })?;
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    drop(function);
    Ok(())
}

async fn register_long_handoff(thread: &Thread) -> Result<Arc<Class>> {
    use ristretto_classfile::{Field, FieldAccessFlags, FieldType};

    let name = "ProductionLongHandoff";
    let mut pool = ConstantPool::new();
    let owner = pool.add_class(name)?;
    let payload = pool.add_field_ref(owner, "payload", "J")?;
    let ready = pool.add_field_ref(owner, "ready", "J")?;
    let publish = method(
        &mut pool,
        "publish",
        "(J)V",
        vec![
            Instruction::Lload_0,
            Instruction::Putstatic(payload),
            Instruction::Lconst_1,
            Instruction::Putstatic(ready),
            Instruction::Return,
        ],
    )?;
    let read_flag = method(
        &mut pool,
        "readFlag",
        "()J",
        vec![Instruction::Getstatic(ready), Instruction::Lreturn],
    )?;
    let read_payload = method(
        &mut pool,
        "readPayload",
        "()J",
        vec![Instruction::Getstatic(payload), Instruction::Lreturn],
    )?;
    let fields = [
        ("payload", FieldAccessFlags::empty()),
        ("ready", FieldAccessFlags::VOLATILE),
    ]
    .into_iter()
    .map(|(name, flags)| -> Result<Field> {
        Ok(Field {
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | flags,
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8("J")?,
            field_type: FieldType::parse("J")?,
            attributes: vec![],
        })
    })
    .collect::<Result<Vec<_>>>()?;
    let mut definition = class_from_methods(
        name,
        "java/lang/Object",
        pool,
        vec![publish, read_flag, read_payload],
    )?
    .class_file()
    .clone();
    definition.fields = fields;
    register(thread, Class::from(None, definition)?).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_volatile_long_store_publishes_an_earlier_plain_store_to_a_java_peer() -> Result<()>
{
    const BITS: i64 = 0x7abc_fedc_8765_4321;
    let harness = Harness::new().await?;
    let class = register_long_handoff(&harness.thread).await?;
    harness.thread.initialize_class(&class).await?;
    let peer = Thread::new(&Arc::downgrade(&harness.vm), harness.thread.id() + 1);
    let observed_class = class.clone();
    let (started, reading) = tokio::sync::oneshot::channel();
    let runtime = tokio::runtime::Handle::current();
    let reader = tokio::task::spawn_blocking(move || {
        runtime.block_on(async move {
            let flag = observed_class.try_get_method("readFlag", "()J")?;
            let payload = observed_class.try_get_method("readPayload", "()J")?;
            assert_eq!(
                peer.execute(&observed_class, &flag, &[] as &[Value])
                    .await?,
                Some(Value::Long(0))
            );
            // This signal precedes both stores. Only the Java flag communicates publication to the reader.
            started
                .send(())
                .map_err(|()| InternalError("writer exited before the reader".into()))?;
            let deadline = std::time::Instant::now() + Duration::from_secs(5);
            loop {
                match peer
                    .execute(&observed_class, &flag, &[] as &[Value])
                    .await?
                {
                    Some(Value::Long(1)) => break,
                    Some(Value::Long(0)) if std::time::Instant::now() < deadline => {
                        std::thread::yield_now();
                    }
                    other => {
                        return Err(InternalError(format!(
                            "volatile handoff stalled or produced {other:?}"
                        )));
                    }
                }
            }
            peer.execute(&observed_class, &payload, &[] as &[Value])
                .await
        })
    });
    reading
        .await
        .map_err(|error| InternalError(error.to_string()))?;
    let publish = class.try_get_method("publish", "(J)V")?;
    assert_eq!(
        harness
            .thread
            .execute(&class, &publish, &[Value::Long(BITS)])
            .await?,
        None
    );
    assert_eq!(
        reader
            .await
            .map_err(|error| InternalError(error.to_string()))??,
        Some(Value::Long(BITS))
    );
    let function = harness
        .vm
        .compiler()
        .and_then(|compiler| compiler.get_cached(cache_key(&publish)))
        .flatten()
        .ok_or_else(|| {
            InternalError("the volatile publisher ran only in the interpreter".into())
        })?;
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    drop(function);
    Ok(())
}
