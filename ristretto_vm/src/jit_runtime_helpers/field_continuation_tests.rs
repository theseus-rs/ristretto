//! Real field requests: primitive bits, checked resolution, roots and initialization lifetime.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ConstantPool, FieldAccessFlags, FieldType};
use ristretto_classloader::InitializationState;
use ristretto_jit::{ExecutionStep, FieldAccess};
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

#[path = "field_resolution_tests.rs"]
mod resolution;

fn field(
    pool: &mut ConstantPool,
    name: &str,
    descriptor: &str,
    flags: FieldAccessFlags,
) -> Result<ristretto_classfile::Field> {
    Ok(ristretto_classfile::Field {
        access_flags: flags,
        name_index: pool.add_utf8(name)?,
        descriptor_index: pool.add_utf8(descriptor)?,
        field_type: FieldType::parse(descriptor)?,
        attributes: vec![],
    })
}

async fn register_fields(
    thread: &Thread,
    name: &str,
    parent: &str,
    pool: ConstantPool<'static>,
    methods: Vec<ristretto_classfile::Method>,
    fields: Vec<ristretto_classfile::Field>,
) -> Result<Arc<Class>> {
    let mut definition = class_from_methods(name, parent, pool, methods)?
        .class_file()
        .clone();
    definition.fields = fields;
    register(thread, Class::from(None, definition)?).await
}

async fn vm(concurrent: bool, compact: bool, workers: usize) -> Result<(Arc<VM>, Arc<Thread>)> {
    let gc = GarbageCollector::with_config(
        ristretto_gc::Configuration::builder()
            .threads(workers)
            .concurrent_publication(concurrent)
            .compact_heap(compact)
            .root_storage(ristretto_gc::RootStorage::Contexts)
            .max_heap_bytes(32 * 1024 * 1024)
            .allocation_threshold(usize::MAX)
            .build(),
    );
    let vm = VM::new(
        crate::ConfigurationBuilder::new()
            .garbage_collector(gc)
            .batch_compilation(false)
            .build()?,
    )
    .await?;
    if workers == 0 {
        vm.garbage_collector().stop()?;
    }
    assert_eq!(vm.garbage_collector().registered_workers(), workers);
    let thread = Thread::new(&Arc::downgrade(&vm), 555);
    Ok((vm, thread))
}

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    if let Value::Object(Some(owner)) = value {
        Ok(owner)
    } else {
        Err(InternalError("missing field root".to_owned()))
    }
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

async fn typed_fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let own = pool.add_class("FieldContinuations")?;
    let mut fields = Vec::new();
    let mut methods = Vec::new();
    let object = pool.add_class("java/lang/Object")?;
    let constructor = pool.add_method_ref(object, "<init>", "()V")?;
    let mut constructor = method(
        &mut pool,
        "<init>",
        "()V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokespecial(constructor),
            Instruction::Return,
        ],
    )?;
    constructor
        .access_flags
        .remove(ristretto_classfile::MethodAccessFlags::STATIC);
    methods.push(constructor);
    for (label, descriptor, stack_type, load, ret) in [
        ("Z", "Z", "I", Instruction::Iload_0, Instruction::Ireturn),
        ("B", "B", "I", Instruction::Iload_0, Instruction::Ireturn),
        ("C", "C", "I", Instruction::Iload_0, Instruction::Ireturn),
        ("S", "S", "I", Instruction::Iload_0, Instruction::Ireturn),
        ("I", "I", "I", Instruction::Iload_0, Instruction::Ireturn),
        ("J", "J", "J", Instruction::Lload_0, Instruction::Lreturn),
        ("F", "F", "F", Instruction::Fload_0, Instruction::Freturn),
        ("D", "D", "D", Instruction::Dload_0, Instruction::Dreturn),
        ("R", "[I", "[I", Instruction::Aload_0, Instruction::Areturn),
    ] {
        for instance in [false, true] {
            let name = format!("{}{label}", if instance { "i" } else { "s" });
            let flags = FieldAccessFlags::PUBLIC
                | FieldAccessFlags::VOLATILE
                | if instance {
                    FieldAccessFlags::empty()
                } else {
                    FieldAccessFlags::STATIC
                };
            fields.push(field(&mut pool, &name, descriptor, flags)?);
            let index = pool.add_field_ref(own, name.as_str(), descriptor)?;
            let (signature, code) = if instance {
                let load = match load {
                    Instruction::Iload_0 => Instruction::Iload_1,
                    Instruction::Lload_0 => Instruction::Lload_1,
                    Instruction::Fload_0 => Instruction::Fload_1,
                    Instruction::Dload_0 => Instruction::Dload_1,
                    _ => Instruction::Aload_1,
                };
                (
                    format!("(LFieldContinuations;{stack_type}){stack_type}"),
                    vec![
                        Instruction::Aload_0,
                        load,
                        Instruction::Putfield(index),
                        Instruction::Aload_0,
                        Instruction::Getfield(index),
                        ret.clone(),
                    ],
                )
            } else {
                (
                    format!("({stack_type}){stack_type}"),
                    vec![
                        load.clone(),
                        Instruction::Putstatic(index),
                        Instruction::Getstatic(index),
                        ret.clone(),
                    ],
                )
            };
            methods.push(method(
                &mut pool,
                &format!("{}{}", if instance { "instance" } else { "static" }, label),
                &signature,
                code,
            )?);
        }
    }
    let class = register_fields(
        thread,
        "FieldContinuations",
        "java/lang/Object",
        pool,
        methods,
        fields,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok(class)
}

fn same_bits(actual: &Value, expected: &Value) {
    match (actual, expected) {
        (Value::Float(a), Value::Float(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        (Value::Double(a), Value::Double(b)) => assert_eq!(a.to_bits(), b.to_bits()),
        _ => assert_eq!(actual, expected),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "all field types share the same interpreted, async and synchronous checks"
)]
async fn typed_access(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let class = typed_fixture(thread).await?;
    let object = Value::try_new_object_async(
        vm.garbage_collector(),
        Reference::Object(Object::new_in_async(vm.garbage_collector(), class.clone()).await?),
    )
    .await?;
    let array =
        Value::try_new_object_async(vm.garbage_collector(), Reference::IntArray(vec![99].into()))
            .await?;
    for (label, descriptor, values) in [
        (
            "Z",
            "I",
            vec![
                (Value::Int(42), Value::Int(0)),
                (Value::Int(-1), Value::Int(1)),
            ],
        ),
        (
            "B",
            "I",
            vec![
                (Value::Int(255), Value::Int(-1)),
                (Value::Int(128), Value::Int(-128)),
            ],
        ),
        (
            "C",
            "I",
            vec![
                (Value::Int(-1), Value::Int(65535)),
                (Value::Int(0xd800), Value::Int(0xd800)),
            ],
        ),
        (
            "S",
            "I",
            vec![
                (Value::Int(65535), Value::Int(-1)),
                (Value::Int(32768), Value::Int(-32768)),
            ],
        ),
        (
            "I",
            "I",
            vec![
                (Value::Int(i32::MIN), Value::Int(i32::MIN)),
                (Value::Int(i32::MAX), Value::Int(i32::MAX)),
            ],
        ),
        (
            "J",
            "J",
            vec![
                (Value::Long(i64::MIN), Value::Long(i64::MIN)),
                (Value::Long(i64::MAX), Value::Long(i64::MAX)),
            ],
        ),
        (
            "F",
            "F",
            vec![
                (
                    Value::Float(f32::from_bits(0x7fc0_0042)),
                    Value::Float(f32::from_bits(0x7fc0_0042)),
                ),
                (Value::Float(-0.0), Value::Float(-0.0)),
            ],
        ),
        (
            "D",
            "D",
            vec![
                (
                    Value::Double(f64::from_bits(0x7ff8_0000_0000_0042)),
                    Value::Double(f64::from_bits(0x7ff8_0000_0000_0042)),
                ),
                (Value::Double(-0.0), Value::Double(-0.0)),
            ],
        ),
        (
            "R",
            "[I",
            vec![
                (array.clone(), array.clone()),
                (Value::Object(None), Value::Object(None)),
            ],
        ),
    ] {
        for instance in [false, true] {
            let name = format!("{}{label}", if instance { "instance" } else { "static" });
            let descriptor = if instance {
                format!("(LFieldContinuations;{descriptor}){descriptor}")
            } else {
                format!("({descriptor}){descriptor}")
            };
            let body = class.try_get_method(&name, &descriptor)?;
            // Interpret before injecting the experimental compiled method.
            for (input, expected) in &values {
                let arguments = if instance {
                    vec![object.clone(), input.clone()]
                } else {
                    vec![input.clone()]
                };
                let result = thread
                    .execute(&class, &body, &arguments)
                    .await?
                    .ok_or_else(|| InternalError("missing interpreted field result".to_owned()))?;
                same_bits(&result, expected);
            }
            let function = install_experimental(vm, &class, &name, &descriptor)?;
            assert!(function.supports_cooperation());
            for (input, expected) in &values {
                let arguments = if instance {
                    vec![object.clone(), input.clone()]
                } else {
                    vec![input.clone()]
                };
                let result = thread
                    .execute(&class, &body, &arguments)
                    .await?
                    .ok_or_else(|| InternalError("missing compiled field result".to_owned()))?;
                same_bits(&result, expected);
                let result = crate::jit::execute(
                    &function,
                    &arguments,
                    vm.garbage_collector(),
                    vm,
                    thread,
                    &class,
                )?
                .ok_or_else(|| InternalError("missing synchronous field result".to_owned()))?;
                same_bits(&result, expected);
            }
            #[cfg(feature = "execution-profile")]
            {
                assert_eq!(function.execution_count(), 4);
                assert_eq!(function.resume_count(), 4);
            }
        }
    }
    assert!(thread.frames()?.is_empty());
    Ok(())
}

async fn initialization_fixture(
    thread: &Thread,
    name: &str,
    put: bool,
) -> Result<(Arc<Class>, Arc<Class>)> {
    let target_name = format!("{name}Target");
    let mut pool = ConstantPool::new();
    let own = pool.add_class(&target_name)?;
    let mut fields = Vec::new();
    for (name, descriptor) in [("count", "I"), ("gate", "I"), ("value", "[I")] {
        fields.push(field(
            &mut pool,
            name,
            descriptor,
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        )?);
    }
    let count = pool.add_field_ref(own, "count", "I")?;
    let gate = pool.add_field_ref(own, "gate", "I")?;
    let initializer = method(
        &mut pool,
        "<clinit>",
        "()V",
        vec![
            Instruction::Getstatic(count),
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Putstatic(count),
            Instruction::Getstatic(gate),
            Instruction::Ifeq(-3),
            Instruction::Return,
        ],
    )?;
    let target = register_fields(
        thread,
        &target_name,
        "java/lang/Object",
        pool,
        vec![initializer],
        fields,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let target_ref = pool.add_class(&target_name)?;
    let index = pool.add_field_ref(
        target_ref,
        if put { "value" } else { "count" },
        if put { "[I" } else { "I" },
    )?;
    let body = method(
        &mut pool,
        "run",
        "([I)[I",
        if put {
            vec![
                Instruction::Aload_0,
                Instruction::Aload_0,
                Instruction::Putstatic(index),
                Instruction::Areturn,
            ]
        } else {
            vec![
                Instruction::Aload_0,
                Instruction::Getstatic(index),
                Instruction::Pop,
                Instruction::Areturn,
            ]
        },
    )?;
    let caller = register(
        thread,
        class_from_methods(name, "java/lang/Object", pool, vec![body])?,
    )
    .await?;
    thread.initialize_class(&caller).await?;
    Ok((target, caller))
}

#[expect(
    clippy::too_many_lines,
    reason = "ordered initialization, suspension, migration and cancellation checks"
)]
async fn initialization_lifetime(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    put: bool,
    cancel: bool,
) -> Result<()> {
    let name = format!(
        "FieldInitialization{}{}",
        if put { "Put" } else { "Get" },
        if cancel { "Cancel" } else { "Complete" }
    );
    let (target, caller) = initialization_fixture(thread, &name, put).await?;
    let function = install_experimental(vm, &caller, "run", "([I)[I")?;
    let gc = vm.garbage_collector();
    let context = RuntimeContext::new(gc, vm, thread, &caller)?;
    let value = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let handle = owner(&value)?.as_handle_i64();
    let weak = ristretto_gc::GcWeak::new(owner(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
    let mut activation = function.activation(&[ristretto_jit::Value::Ptr(handle)])?;
    let ExecutionStep::Field(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing field request".to_owned()));
    };
    assert_eq!(
        request.kind,
        if put {
            FieldAccess::PutStatic
        } else {
            FieldAccess::GetStatic
        }
    );
    assert_eq!(activation.remaining_budget()?, None);
    drop(value);
    let mut operation = Box::pin(context.invoke_field(request, activation.call_arguments()?));
    let mut cx = Context::from_waker(Waker::noop());
    assert!(operation.as_mut().poll(&mut cx).is_pending());
    assert_eq!(target.static_value("count")?, Value::Int(1));
    assert_eq!(
        target.initialization_state()?,
        InitializationState::BeingInitialized {
            thread_id: thread.id()
        }
    );
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let garbage_weak =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    drop(garbage);
    collect(gc)?;
    assert!(weak.upgrade().is_some());
    assert!(garbage_weak.upgrade().is_none());
    let runtime = Handle::current();
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                let _runtime = runtime.enter();
                assert!(
                    operation
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
            })
            .join()
    })
    .map_err(|_| InternalError("field operation migration panicked".to_owned()))?;
    let other = Thread::new(&Arc::downgrade(vm), thread.id() + 1);
    let mut waiter = Box::pin(other.initialize_class(&target));
    assert!(waiter.as_mut().poll(&mut cx).is_pending());
    if cancel {
        drop(operation);
        assert!(matches!(
            target.initialization_state()?,
            InitializationState::Failed { .. }
        ));
        assert!(matches!(
            waiter.await,
            Err(JavaError(crate::JavaError::NoClassDefFoundError(_)))
        ));
        assert!(
            target.static_value("value")?.is_null(),
            "cancelled store was committed"
        );
        assert!(matches!(
            thread.initialize_class(&target).await,
            Err(JavaError(crate::JavaError::NoClassDefFoundError(_)))
        ));
        drop(activation);
        drop(context);
    } else {
        target.set_static_value("gate", Value::Int(1))?;
        let bits = operation.await?;
        waiter.await?;
        activation.complete_call(bits)?;
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(handle)))
        );
        let result = context.into_reference_result(handle)?;
        assert_eq!(&*result.as_int_vec_ref()?, &[77]);
        if put {
            assert_eq!(
                owner(&target.static_value("value")?)?.as_handle_i64(),
                handle
            );
            target.set_static_value("value", Value::Object(None))?;
        }
        drop(result);
    }
    assert_eq!(
        target.static_value("count")?,
        Value::Int(1),
        "field resumption replayed initialization"
    );
    assert!(thread.frames()?.is_empty());
    collect(gc)?;
    assert!(weak.upgrade().is_none());
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
                let (vm, thread) = vm(concurrent, compact, workers).await?;
                typed_access(&vm, &thread).await?;
                resolution::check(&vm, &thread).await?;
                for put in [false, true] {
                    for cancel in [false, true] {
                        initialization_lifetime(&vm, &thread, put, cancel).await?;
                    }
                }
                println!(
                    "field continuation matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn field_requests_preserve_values_roots_and_initialization_lifetime() -> Result<()> {
    const CHILD: &str = "RISTRETTO_FIELD_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact","jit_runtime_helpers::field_continuation_tests::field_requests_preserve_values_roots_and_initialization_lifetime","--nocapture"]).env(CHILD,"1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
