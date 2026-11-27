//! Direct backend differential checks using live VM helpers. Production eligibility
//! remains closed for these methods; this does not qualify compiled-frame GC roots.
use super::*;
use indexmap::IndexMap;
use ristretto_classfile::attributes::{
    ArrayType, Attribute, ExceptionTableEntry, Instruction, LookupSwitch, MaxLocals, TableSwitch,
};
use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
use ristretto_classloader::{Object, Reference};

struct Fixture {
    descriptor: &'static str,
    code: Vec<Instruction>,
    handlers: Vec<ExceptionTableEntry>,
    cases: Vec<(Vec<Value>, i32)>,
}

fn switch_cases(keys: &[(i32, i32)]) -> Vec<(Vec<Value>, i32)> {
    keys.iter()
        .map(|&(key, expected)| (vec![Value::Int(key)], expected))
        .collect()
}

async fn reference_fixtures(gc: &Arc<GarbageCollector>, thread: &Thread) -> Result<Vec<Fixture>> {
    let array = Value::try_new_object_async(gc, Reference::IntArray(vec![1].into())).await?;
    let throwable =
        Object::new_in_async(gc, thread.class("java/lang/RuntimeException").await?).await?;
    let throwable = Value::try_new_object_async(gc, Reference::Object(throwable)).await?;
    let backwards = Fixture {
        descriptor: "(Ljava/lang/Object;)I",
        code: vec![
            Instruction::Goto(10),
            Instruction::Dup,
            Instruction::Ifnull(10),
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Ireturn,
            Instruction::Aload_0,
            Instruction::Goto(-8),
            Instruction::Pop,
            Instruction::Iconst_2,
            Instruction::Ireturn,
        ],
        handlers: vec![],
        cases: vec![(vec![Value::Object(None)], 2), (vec![array], 1)],
    };
    let exception = Fixture {
        descriptor: "(Ljava/lang/Throwable;)I",
        code: vec![
            Instruction::Iconst_5,
            Instruction::Aload_0,
            Instruction::Athrow,
            Instruction::Pop,
            Instruction::Bipush(42),
            Instruction::Ireturn,
        ],
        handlers: vec![ExceptionTableEntry {
            range_pc: 0..3,
            handler_pc: 3,
            catch_type: 0,
        }],
        cases: vec![(vec![throwable], 42)],
    };
    let helper = Fixture {
        descriptor: "()I",
        code: vec![
            Instruction::Iconst_5,
            Instruction::Aconst_null,
            Instruction::Aconst_null,
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Arraylength,
            Instruction::Pop,
            Instruction::Pop2,
            Instruction::Ireturn,
        ],
        handlers: vec![],
        cases: vec![(vec![], 5)],
    };
    Ok(vec![backwards, exception, helper])
}

fn switch_fixtures() -> Vec<Fixture> {
    let mut fixtures = Vec::new();
    for (switch, cases) in [
        (
            Instruction::Tableswitch(Box::new(TableSwitch {
                low: -1,
                high: 1,
                offsets: vec![28, 32, 36],
                default: 40,
            })),
            switch_cases(&[
                (-2, 9),
                (-1, 6),
                (0, 7),
                (1, 8),
                (2, 9),
                (i32::MIN, 9),
                (i32::MAX, 9),
            ]),
        ),
        (
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: 48,
                pairs: IndexMap::from([(i32::MIN, 36), (-1, 40), (i32::MAX, 44)]),
            })),
            switch_cases(&[(i32::MIN, 6), (-1, 7), (i32::MAX, 8), (0, 9), (-2, 9)]),
        ),
    ] {
        let mut code = vec![
            Instruction::Iconst_5,
            Instruction::Aconst_null,
            Instruction::Aconst_null,
            Instruction::Iload_0,
            switch,
        ];
        for result in [
            Instruction::Iconst_1,
            Instruction::Iconst_2,
            Instruction::Iconst_3,
            Instruction::Iconst_4,
        ] {
            code.extend([
                Instruction::Pop2,
                result,
                Instruction::Iadd,
                Instruction::Ireturn,
            ]);
        }
        fixtures.push(Fixture {
            descriptor: "(I)I",
            code,
            handlers: vec![],
            cases,
        });
    }
    fixtures
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn control_flow_and_reference_kinds_match_the_interpreter() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let gc = vm.garbage_collector();
    let mut fixtures = reference_fixtures(gc, &thread).await?;
    fixtures.extend(switch_fixtures());
    let backend = ristretto_jit::Compiler::new()?;
    for (index, fixture) in fixtures.into_iter().enumerate() {
        let class = fixture_class(&format!("ControlFlowFixture{index}"), &fixture)?;
        let method = class.try_get_method("run", fixture.descriptor)?;
        assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
        let function = Arc::new(backend.compile(
            class.class_file(),
            method.definition(),
            &jit_runtime_helpers::symbols(),
        )?);
        #[cfg(feature = "execution-profile")]
        let count = fixture.cases.len();
        for (parameters, expected) in fixture.cases {
            let interpreted = thread.execute(&class, &method, &parameters).await?;
            let compiled = execute(&function, &parameters, gc, &vm, &thread, &class)?;
            assert_eq!(
                interpreted,
                Some(Value::Int(expected)),
                "interpreter fixture {index}"
            );
            assert_eq!(compiled, interpreted, "compiled fixture {index}");
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), u64::try_from(count)?);
    }
    Ok(())
}

fn fixture_class(name: &str, fixture: &Fixture) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class(name)?;
    let name_index = pool.add_utf8("run")?;
    let descriptor_index = pool.add_utf8(fixture.descriptor)?;
    let code_name = pool.add_utf8("Code")?;
    let mut method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index,
        descriptor_index,
        attributes: vec![],
    };
    method.attributes.push(Attribute::Code {
        name_index: code_name,
        max_stack: 6,
        max_locals: fixture.code.max_locals(&pool, &method)?,
        code: fixture.code.clone(),
        exception_table: fixture.handlers.clone(),
        attributes: vec![],
    });
    let class = Class::from(
        None,
        ClassFile {
            constant_pool: pool,
            this_class,
            methods: vec![method],
            ..Default::default()
        },
    )?;
    Ok(class)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn primitive_local_reuse_executes_through_the_production_gate() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let interpreted_vm = VM::new(
        crate::ConfigurationBuilder::new()
            .interpreted(true)
            .build()?,
    )
    .await?;
    let interpreted_thread = Thread::new(&Arc::downgrade(&interpreted_vm), 3);
    let fixtures = [
        Fixture {
            descriptor: "()I",
            code: vec![
                Instruction::Iconst_2,
                Instruction::Istore_0,
                Instruction::Fconst_1,
                Instruction::Fstore_0,
                Instruction::Fload_0,
                Instruction::F2i,
                Instruction::Iconst_3,
                Instruction::Istore_0,
                Instruction::Iload_0,
                Instruction::Iadd,
                Instruction::Ireturn,
            ],
            handlers: vec![],
            cases: vec![(vec![], 4)],
        },
        Fixture {
            descriptor: "(J)I",
            code: vec![
                Instruction::Lload_0,
                Instruction::L2i,
                Instruction::Fconst_2,
                Instruction::Fstore_1,
                Instruction::Fload_1,
                Instruction::F2i,
                Instruction::Iadd,
                Instruction::Ireturn,
            ],
            handlers: vec![],
            cases: vec![(vec![Value::Long(40)], 42)],
        },
    ];
    for (index, fixture) in fixtures.into_iter().enumerate() {
        let class = fixture_class(&format!("LocalReuseFixture{index}"), &fixture)?;
        let method = class.try_get_method("run", fixture.descriptor)?;
        assert!(ristretto_jit::Compiler::can_compile(method.definition()));
        for (parameters, expected) in fixture.cases {
            assert_eq!(
                interpreted_thread
                    .execute(&class, &method, &parameters)
                    .await?,
                Some(Value::Int(expected))
            );
            assert_eq!(
                thread.execute(&class, &method, &parameters).await?,
                Some(Value::Int(expected))
            );
        }
        let function = vm
            .compiler()
            .expect("compiler")
            .get_cached(cache_key(&method))
            .flatten()
            .expect("compiled function");
        #[cfg(feature = "execution-profile")]
        {
            assert_eq!(function.execution_count(), 1);
            let profile = interpreted_vm.execution_profile();
            let record = profile
                .methods
                .iter()
                .find(|record| record.class == class.name())
                .expect("interpreted method record");
            assert_eq!(record.interpreter_entries, 1);
            assert_eq!(record.compiled_entries, 0);
        }
        drop(function);
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_allocation_loop_reclaims_before_return_and_keeps_a_live_local() -> Result<()> {
    allocating_frame_runs_in_bounded_heap(vec![
        Instruction::Iconst_1,                 // 0
        Instruction::Newarray(ArrayType::Int), // 1
        Instruction::Astore_1,                 // 3
        Instruction::Aload_1,                  // 4
        Instruction::Iconst_0,                 // 5
        Instruction::Bipush(77),               // 6
        Instruction::Iastore,                  // 8
        Instruction::Iload_0,                  // 9: loop header
        Instruction::Ifeq(15),                 // 10 -> 25
        Instruction::Sipush(2048),             // 13
        Instruction::Newarray(ArrayType::Int), // 16
        Instruction::Pop,                      // 18: discard each temporary array
        Instruction::Iinc(0, -1),              // 19
        Instruction::Goto(-13),                // 22 -> 9
        Instruction::Aload_1,                  // 25: original array must survive
        Instruction::Iconst_0,                 // 26
        Instruction::Iaload,                   // 27
        Instruction::Ireturn,                  // 28
    ])
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_allocation_loop_keeps_a_live_operand_across_backedges() -> Result<()> {
    allocating_frame_runs_in_bounded_heap(vec![
        Instruction::Iconst_1,                 // 0
        Instruction::Newarray(ArrayType::Int), // 1
        Instruction::Dup,                      // 3: keep original on the operand stack
        Instruction::Iconst_0,                 // 4
        Instruction::Bipush(77),               // 5
        Instruction::Iastore,                  // 7
        Instruction::Iload_0,                  // 8: loop header
        Instruction::Ifeq(15),                 // 9 -> 24
        Instruction::Sipush(2048),             // 12
        Instruction::Newarray(ArrayType::Int), // 15
        Instruction::Pop,                      // 17
        Instruction::Iinc(0, -1),              // 18
        Instruction::Goto(-13),                // 21 -> 8
        Instruction::Iconst_0,                 // 24
        Instruction::Iaload,                   // 25
        Instruction::Ireturn,                  // 26
    ])
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_frame_reclaims_a_dead_reference_after_primitive_slot_reuse() -> Result<()> {
    allocating_frame_runs_in_bounded_heap(vec![
        Instruction::Sipush(32767),
        Instruction::Sipush(160),
        Instruction::Imul,
        Instruction::Newarray(ArrayType::Int), // About 20 MiB, then abandoned.
        Instruction::Astore_0,
        Instruction::Aload_0,
        Instruction::Pop,
        Instruction::Iconst_0,
        Instruction::Istore_0, // The stale reference SSA variable must not retain the array.
        Instruction::Sipush(32767),
        Instruction::Sipush(160),
        Instruction::Imul,
        Instruction::Newarray(ArrayType::Int), // Both arrays cannot fit in the 32 MiB heap.
        Instruction::Pop,
        Instruction::Bipush(77),
        Instruction::Ireturn,
    ])
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_allocation_loop_keeps_a_local_used_only_by_an_exception_handler() -> Result<()> {
    allocating_frame_with_handlers(
        vec![
            Instruction::Iconst_1,                 // 0
            Instruction::Newarray(ArrayType::Int), // 1
            Instruction::Astore_1,                 // 3
            Instruction::Aload_1,                  // 4
            Instruction::Iconst_0,                 // 5
            Instruction::Bipush(77),               // 6
            Instruction::Iastore,                  // 8
            Instruction::Iload_0,                  // 9
            Instruction::Ifeq(15),                 // 10 -> 25
            Instruction::Sipush(2048),             // 13
            Instruction::Newarray(ArrayType::Int), // 16
            Instruction::Pop,                      // 18
            Instruction::Iinc(0, -1),              // 19
            Instruction::Goto(-13),                // 22 -> 9
            Instruction::Aconst_null,              // 25: produce NullPointerException
            Instruction::Athrow,                   // 26
            Instruction::Pop,                      // 27: discard the owned throwable
            Instruction::Aload_1,                  // 28: only use after the loop is exceptional
            Instruction::Iconst_0,                 // 29
            Instruction::Iaload,                   // 30
            Instruction::Ireturn,                  // 31
        ],
        vec![ExceptionTableEntry {
            range_pc: 25..27,
            handler_pc: 27,
            catch_type: 0,
        }],
    )
    .await
}

async fn allocating_frame_runs_in_bounded_heap(code: Vec<Instruction>) -> Result<()> {
    allocating_frame_with_handlers(code, vec![]).await
}

async fn allocating_frame_with_handlers(
    code: Vec<Instruction>,
    handlers: Vec<ExceptionTableEntry>,
) -> Result<()> {
    for (concurrent, workers) in [(false, 0), (false, 1), (true, 0), (true, 1)] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .max_heap_bytes(32 * 1024 * 1024)
                .concurrent_publication(concurrent)
                .threads(workers)
                .diagnostics(true)
                .build(),
        );
        let vm = VM::new(
            crate::ConfigurationBuilder::new()
                .garbage_collector(gc.clone())
                .batch_compilation(false)
                .build()?,
        )
        .await?;
        if workers == 0 {
            // Native threads=0 selects one worker during VM startup. Stop/join it
            // explicitly before exercising allocation-driven cooperative recovery.
            gc.stop()?;
        }
        assert_eq!(gc.registered_workers(), workers);
        let thread = Thread::new(&Arc::downgrade(&vm), 3);
        let fixture = Fixture {
            descriptor: "(I)I",
            code: code.clone(),
            handlers: handlers.clone(),
            cases: vec![],
        };
        let class = fixture_class("AllocatingFrameRootFixture", &fixture)?;
        let method = class.try_get_method("run", fixture.descriptor)?;
        assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
        let function = Arc::new(ristretto_jit::Compiler::new()?.compile(
            class.class_file(),
            method.definition(),
            &jit_runtime_helpers::symbols(),
        )?);
        let before = gc.statistics()?;
        let recovery_before = gc.diagnostics();
        // Each fixture allocates more array payload than the entire 32 MiB managed heap.
        assert_eq!(
            execute(&function, &[Value::Int(8_000)], &gc, &vm, &thread, &class)?,
            Some(Value::Int(77))
        );
        let after = gc.statistics()?;
        // Recovery can release enough storage before its cycle completes. Prove
        // reclamation during the invocation rather than requiring a full-cycle wait.
        assert!(after.objects_swept > before.objects_swept);
        assert!(after.bytes_freed > before.bytes_freed);
        assert_eq!(gc.registered_workers(), workers);
        if workers == 0 {
            let recovery_after = gc.diagnostics();
            assert!(
                recovery_after.async_assists > recovery_before.async_assists,
                "workerless compiled allocation did not drive cooperative recovery"
            );
            assert_eq!(
                recovery_after.worker_steps.count,
                recovery_before.worker_steps.count
            );
        }
        #[cfg(feature = "execution-profile")]
        assert_eq!(function.execution_count(), 1);
    }
    Ok(())
}

async fn transfer_vm(concurrent: bool) -> Result<(Arc<VM>, Arc<Thread>)> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(0)
            .diagnostics(true)
            .concurrent_publication(concurrent)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .max_heap_bytes(32 * 1024 * 1024)
            .build(),
    );
    let vm = VM::new(
        crate::ConfigurationBuilder::new()
            .garbage_collector(gc)
            .batch_compilation(false)
            .build()?,
    )
    .await?;
    vm.garbage_collector().stop()?;
    assert_eq!(vm.garbage_collector().registered_workers(), 0);
    let thread = Thread::new(&Arc::downgrade(&vm), 3);
    Ok((vm, thread))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_reference_returns_transfer_existing_owners() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = transfer_vm(concurrent).await?;
        let gc = vm.garbage_collector();
        let context = gc.register_context();
        let argument = || async {
            Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await
        };
        for (descriptor, code, parameters, registrations, expected) in [
            (
                "()[I",
                vec![
                    Instruction::Iconst_1,
                    Instruction::Newarray(ArrayType::Int),
                    Instruction::Areturn,
                ],
                vec![],
                1,
                Some(0),
            ),
            (
                "([I)[I",
                vec![Instruction::Aload_0, Instruction::Areturn],
                vec![argument().await?],
                1,
                Some(77),
            ),
            (
                "(IIIIIIII[I)[I",
                vec![Instruction::Aload(8), Instruction::Areturn],
                (1..=8).map(Value::Int).chain([argument().await?]).collect(),
                1,
                Some(77),
            ),
            (
                "(JJJJ[I)[I",
                vec![Instruction::Aload(8), Instruction::Areturn],
                (1..=4)
                    .flat_map(|value| [Value::Long(value), Value::Unused])
                    .chain([argument().await?])
                    .collect(),
                1,
                Some(77),
            ),
            (
                "()[I",
                vec![Instruction::Aconst_null, Instruction::Areturn],
                vec![],
                0,
                None,
            ),
        ] {
            let fixture = Fixture {
                descriptor,
                code,
                handlers: vec![],
                cases: vec![],
            };
            let class = fixture_class("ReferenceReturnFixture", &fixture)?;
            let method = class.try_get_method("run", descriptor)?;
            assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
            let function = Arc::new(ristretto_jit::Compiler::new()?.compile(
                class.class_file(),
                method.definition(),
                &jit_runtime_helpers::symbols(),
            )?);
            // Warm exception metadata and the reusable sentinel before counting.
            drop(RuntimeContext::new(gc, &vm, &thread, &class)?);
            let value = context.with(|| -> Result<_> {
                let before = gc.diagnostics().root_registrations;
                let result = execute(&function, &parameters, gc, &vm, &thread, &class)?;
                assert_eq!(
                    gc.diagnostics().root_registrations - before,
                    registrations,
                    "return added an avoidable root registration: {descriptor}"
                );
                result.ok_or_else(|| InternalError("missing reference return".to_owned()))
            })?;
            drop(parameters);
            #[cfg(feature = "execution-profile")]
            assert_eq!(function.execution_count(), 1);
            if let Some(expected) = expected {
                let Value::Object(Some(owner)) = &value else {
                    return Err(InternalError("missing returned owner".to_owned()));
                };
                let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
                gc.collect_full()?;
                gc.collect_full()?;
                assert_eq!(&[expected], &*value.as_int_vec_ref()?);
                drop(value);
                gc.collect_full()?;
                gc.collect_full()?;
                assert!(weak.upgrade().is_none(), "returned owner leaked after drop");
            } else {
                assert_eq!(value, Value::Object(None));
            }
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn compiled_exception_transfers_reuse_owners_and_survive_context_exit() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = transfer_vm(concurrent).await?;
        let gc = vm.garbage_collector();
        let context = gc.register_context();
        for caught in [true, false] {
            let object =
                Object::new_in_async(gc, thread.class("java/lang/RuntimeException").await?).await?;
            let parameters =
                vec![Value::try_new_object_async(gc, Reference::Object(object)).await?];
            let mut fixture = Fixture {
                descriptor: "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                code: vec![Instruction::Aload_0, Instruction::Athrow],
                handlers: vec![],
                cases: vec![],
            };
            if caught {
                fixture.code.push(Instruction::Areturn);
                fixture.handlers.push(ExceptionTableEntry {
                    range_pc: 0..2,
                    handler_pc: 2,
                    catch_type: 0,
                });
            }
            let class = fixture_class("ExceptionOwnerTransferFixture", &fixture)?;
            let method = class.try_get_method("run", fixture.descriptor)?;
            assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
            let function = Arc::new(ristretto_jit::Compiler::new()?.compile(
                class.class_file(),
                method.definition(),
                &jit_runtime_helpers::symbols(),
            )?);
            drop(RuntimeContext::new(gc, &vm, &thread, &class)?);
            let value = context.with(|| -> Result<_> {
                let before = gc.diagnostics().root_registrations;
                let result = execute(&function, &parameters, gc, &vm, &thread, &class);
                // One entry snapshot and one guarded type check in athrow. The
                // exception-slot, handler and returned value reuse those owners.
                assert_eq!(
                    gc.diagnostics().root_registrations - before,
                    2,
                    "exception transfer added avoidable registrations; caught={caught}"
                );
                match result {
                    Ok(Some(value)) if caught => Ok(value),
                    Err(crate::Error::Throwable(value)) if !caught => Ok(value),
                    other => Err(InternalError(format!(
                        "wrong exception transfer result: {other:?}"
                    ))),
                }
            })?;
            drop(parameters);
            let Value::Object(Some(owner)) = &value else {
                return Err(InternalError("missing exception owner".to_owned()));
            };
            let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
            gc.collect_full()?;
            gc.collect_full()?;
            assert_eq!(
                value.as_object_ref()?.class().name(),
                "java/lang/RuntimeException"
            );
            drop(value);
            gc.collect_full()?;
            gc.collect_full()?;
            assert!(
                weak.upgrade().is_none(),
                "exception owner leaked after drop"
            );
            #[cfg(feature = "execution-profile")]
            assert_eq!(function.execution_count(), 1);
        }
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "ordered argument ownership, rejection and reclamation schedule"
)]
async fn argument_rejection_preserves_vm_roots_and_wide_argument_conversion() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let gc = vm.garbage_collector();
    gc.stop()?;
    assert_eq!(gc.registered_workers(), 0);
    let fixture = Fixture {
        descriptor: "(JDJDJ)J",
        code: vec![Instruction::Lload(8), Instruction::Lreturn],
        handlers: vec![],
        cases: vec![],
    };
    let class = fixture_class("ValidatedArguments", &fixture)?;
    let method = class.try_get_method("run", fixture.descriptor)?;
    assert!(ristretto_jit::Compiler::can_compile(method.definition()));
    let function = vm
        .compiler()
        .expect("native compiler")
        .compile(&class, &method)
        .await?
        .expect("qualified primitive method");
    // Keep the normal cached emergency throwable outside the per-call root delta.
    drop(RuntimeContext::new(gc, &vm, &thread, &class)?);
    let object = Value::try_new_object_async(gc, Reference::IntArray(vec![42].into())).await?;
    let Value::Object(Some(owner)) = &object else {
        panic!("reference argument")
    };
    let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    let roots = || {
        let stats = gc.root_statistics();
        stats.slot_roots + stats.compatibility_roots
    };
    let before = roots();
    assert!(matches!(
        execute(&function, &[], gc, &vm, &thread, &class),
        Err(crate::Error::JitError(
            ristretto_jit::Error::InvalidArgumentCount {
                expected: 5,
                actual: 0
            }
        ))
    ));
    assert_eq!(roots(), before);
    for padded in [false, true] {
        let mut arguments = vec![
            Value::Long(1),
            Value::Double(2.0),
            Value::Long(3),
            Value::Double(4.0),
            object.clone(),
        ];
        if padded {
            arguments = arguments
                .into_iter()
                .flat_map(|value| [value, Value::Unused])
                .collect();
        }
        let call_roots = roots(); // Value::clone deliberately owns another root.
        assert!(matches!(
            execute(&function, &arguments, gc, &vm, &thread, &class),
            Err(crate::Error::JitError(
                ristretto_jit::Error::InvalidArgumentType { index: 4, .. }
            ))
        ));
        assert_eq!(roots(), call_roots, "rejected context retained roots");
        gc.collect_full()?;
        assert!(
            weak.upgrade().is_some(),
            "rejection discarded caller ownership"
        );
        drop(arguments);
        assert_eq!(roots(), before);
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 0);
    for padded in [false, true] {
        let mut arguments = vec![
            Value::Long(1),
            Value::Double(2.0),
            Value::Long(3),
            Value::Double(4.0),
            Value::Long(42),
        ];
        if padded {
            arguments = arguments
                .into_iter()
                .flat_map(|value| [value, Value::Unused])
                .collect();
        }
        assert_eq!(
            execute(&function, &arguments, gc, &vm, &thread, &class)?,
            Some(Value::Long(42))
        );
        assert_eq!(roots(), before);
    }
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 2);
    drop(object);
    gc.collect_full()?;
    assert!(
        weak.upgrade().is_none(),
        "rejected argument remained retained"
    );
    Ok(())
}
