//! Constants must resolve with no generated frame active.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, install_experimental, method, register};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn reference_constants_return_owning_requests() -> Result<()> {
    let (vm, thread) = crate::jit::static_call_tests::vm_with_workers(false, 0).await?;
    let mut pool = ConstantPool::new();
    let string = pool.add_string("constant request")?;
    let class = pool.add_class("[I")?;
    let mut methods = Vec::new();
    for (name, index) in [("string", string), ("class", class)] {
        methods.push(method(
            &mut pool,
            name,
            "()Ljava/lang/Object;",
            vec![Instruction::Ldc_w(index), Instruction::Areturn],
        )?);
    }
    let class = register(
        &thread,
        class_from_methods("ConstantRequests", "java/lang/Object", pool, methods)?,
    )
    .await?;
    for name in ["string", "class"] {
        let function = install_experimental(&vm, &class, name, "()Ljava/lang/Object;")?;
        assert!(function.supports_cooperation(), "{name} cannot suspend");
    }
    Ok(())
}

use ristretto_jit::ExecutionStep;
use std::future::Future;
use std::num::NonZeroU32;
use std::task::{Context, Waker};
use std::time::Duration;

const OBJECT: &str = "()Ljava/lang/Object;";

fn owner(value: &Value) -> Result<&Gc<RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(InternalError("missing constant fixture owner".to_owned())),
    }
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
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
    let thread = Thread::new(&Arc::downgrade(&vm), 560);
    Ok((vm, thread))
}

#[expect(
    clippy::too_many_lines,
    reason = "exact exported bytecode for independent VM differential checks"
)]
async fn fixture(thread: &Thread) -> Result<(Arc<Class>, Arc<Class>)> {
    let mut pool = ConstantPool::new();
    let initializer = method(&mut pool, "<clinit>", "()V", vec![Instruction::Goto(0)])?;
    let target = register(
        thread,
        class_from_methods(
            "UninitializedConstantTarget",
            "java/lang/Object",
            pool,
            vec![initializer],
        )?,
    )
    .await?;
    let mut pool = ConstantPool::new();
    let text = pool.add_string("a\0b😀")?;
    let class = pool.add_class("UninitializedConstantTarget")?;
    let array = pool.add_class("[[I")?;
    let missing = pool.add_class("MissingConstantTarget")?;
    let raw =
        ristretto_classfile::JavaStr::from_mutf8(&[0xed, 0xa0, 0x80, b'a', 0xed, 0xbf, 0xbf])?;
    let utf8 = pool.add(ristretto_classfile::Constant::Utf8(
        std::borrow::Cow::Owned(raw.to_java_string()),
    ))?;
    let surrogates = pool.add(ristretto_classfile::Constant::String(utf8))?;
    let mut methods = Vec::new();
    for (name, index) in [
        ("text", text),
        ("target", class),
        ("array", array),
        ("surrogates", surrogates),
        ("missing", missing),
    ] {
        methods.push(method(
            &mut pool,
            name,
            OBJECT,
            vec![Instruction::Ldc(u8::try_from(index)?), Instruction::Areturn],
        )?);
    }
    for i in 0..300 {
        pool.add_integer(i)?;
    }
    let wide_text = pool.add_string("wide constant")?;
    let wide_class = pool.add_class("java/lang/Cloneable")?;
    assert!(wide_text > 255 && wide_class > 255);
    for (name, index) in [("wideText", wide_text), ("wideClass", wide_class)] {
        methods.push(method(
            &mut pool,
            name,
            OBJECT,
            vec![Instruction::Ldc_w(index), Instruction::Areturn],
        )?);
    }
    methods.push(method(
        &mut pool,
        "wideResidual",
        "(J)J",
        vec![
            Instruction::Lload_0,
            Instruction::Ldc_w(wide_text),
            Instruction::Pop,
            Instruction::Lload_0,
            Instruction::Ladd,
            Instruction::Lreturn,
        ],
    )?);
    methods.push(method(
        &mut pool,
        "loop",
        "()V",
        vec![
            Instruction::Ldc_w(wide_text),
            Instruction::Pop,
            Instruction::Goto(-4),
        ],
    )?);
    let error = pool.add_class("java/lang/NoClassDefFoundError")?;
    let mut caught = method(
        &mut pool,
        "caught",
        "()I",
        vec![
            Instruction::Ldc_w(missing),
            Instruction::Pop,
            Instruction::Iconst_1,
            Instruction::Ireturn,
            Instruction::Pop,
            Instruction::Iconst_0,
            Instruction::Ireturn,
        ],
    )?;
    for attribute in &mut caught.attributes {
        if let ristretto_classfile::attributes::Attribute::Code {
            exception_table, ..
        } = attribute
        {
            exception_table.push(ristretto_classfile::attributes::ExceptionTableEntry {
                range_pc: 0..6,
                handler_pc: 6,
                catch_type: error,
            });
        }
    }
    methods.push(caught);
    let class = register(
        thread,
        class_from_methods("ConstantSemantics", "java/lang/Object", pool, methods)?,
    )
    .await?;
    thread.initialize_class(&class).await?;
    Ok((class, target))
}

#[expect(
    clippy::too_many_lines,
    reason = "interpret, resume and synchronous embedding with identity and initialization checks"
)]
async fn semantics(vm: &Arc<VM>, thread: &Arc<Thread>) -> Result<()> {
    let (class, target) = fixture(thread).await?;
    let gc = vm.garbage_collector();
    for name in [
        "text",
        "target",
        "array",
        "surrogates",
        "wideText",
        "wideClass",
    ] {
        let body = class.try_get_method(name, OBJECT)?;
        let interpreted = thread
            .execute(&class, &body, &[] as &[Value])
            .await?
            .ok_or_else(|| InternalError("missing constant value".to_owned()))?;
        let function = install_experimental(vm, &class, name, OBJECT)?;
        assert!(function.supports_cooperation());
        let compiled = thread
            .execute(&class, &body, &[] as &[Value])
            .await?
            .ok_or_else(|| InternalError("missing constant value".to_owned()))?;
        assert_eq!(
            owner(&interpreted)?.identity(),
            owner(&compiled)?.identity()
        );
        let context = RuntimeContext::new(gc, vm, thread, &class)?;
        let Some(ristretto_jit::Value::Ptr(bits)) = function.execute(&[], context.as_ptr())? else {
            return Err(InternalError(
                "missing synchronous constant result".to_owned(),
            ));
        };
        assert_eq!(context.pending_exception(), 0);
        let synchronous = context.into_reference_result(bits)?;
        assert_eq!(
            owner(&interpreted)?.identity(),
            owner(&synchronous)?.identity()
        );
        collect(gc)?;
        match name {
            "text" => assert_eq!(compiled.as_string()?, "a\0b😀"),
            "wideText" => assert_eq!(compiled.as_string()?, "wide constant"),
            "surrogates" => {
                let bytes = compiled.as_object_ref()?.value("value")?;
                let expected: Vec<i8> = [0xd800_u16, 0x0061, 0xdfff]
                    .into_iter()
                    .flat_map(u16::to_ne_bytes)
                    .map(|byte| i8::from_ne_bytes([byte]))
                    .collect();
                assert_eq!(&*bytes.as_byte_vec_ref()?, &expected);
            }
            _ => assert_eq!(compiled.as_object_ref()?.class().name(), "java/lang/Class"),
        }
        assert!(!target.is_initialized()?, "class literal ran <clinit>");
        #[cfg(feature = "execution-profile")]
        assert!(function.execution_count() >= 2);
    }
    for (name, descriptor) in [
        ("missing", OBJECT),
        ("caught", "()I"),
        ("wideResidual", "(J)J"),
    ] {
        let body = class.try_get_method(name, descriptor)?;
        for compiled in [false, true] {
            let function = if compiled {
                Some(install_experimental(vm, &class, name, descriptor)?)
            } else {
                None
            };
            if name == "wideResidual" {
                for bits in [i64::MIN, -1, 0, 42, i64::MAX] {
                    assert_eq!(
                        thread.execute(&class, &body, &[Value::Long(bits)]).await?,
                        Some(Value::Long(bits.wrapping_mul(2)))
                    );
                }
            } else if name == "caught" {
                assert_eq!(
                    thread.execute(&class, &body, &[] as &[Value]).await?,
                    Some(Value::Int(0))
                );
            } else {
                let Err(crate::Error::Throwable(error)) =
                    thread.execute(&class, &body, &[] as &[Value]).await
                else {
                    return Err(InternalError(
                        "missing constant resolution error".to_owned(),
                    ));
                };
                assert_eq!(
                    error.as_object_ref()?.class().name(),
                    "java/lang/NoClassDefFoundError"
                );
            }
            #[cfg(feature = "execution-profile")]
            if let Some(function) = &function {
                assert!(function.execution_count() > 0);
            }
            drop(function);
        }
    }
    // Already-interned constants must still consume cooperation fuel.
    let function = install_experimental(vm, &class, "loop", "()V")?;
    let body = class.try_get_method("loop", "()V")?;
    let mut execution = Box::pin(thread.execute(&class, &body, &[] as &[Value]));
    for _ in 0..3 {
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
    }
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        assert!(function.resume_count() > 2);
    }
    drop(function);
    assert_eq!(thread.frames()?.len(), 1);
    drop(execution);
    assert_eq!(thread.frames()?.len(), 0);
    Ok(())
}

/// Preserve a reference residual below an argument-free request while the real loader waits.
#[expect(
    clippy::too_many_lines,
    reason = "ordered native return, loader wait, migration, GC, cancellation and completion"
)]
async fn suspended_resolution(
    vm: &Arc<VM>,
    thread: &Arc<Thread>,
    wide: bool,
    string: bool,
    cancel: bool,
) -> Result<()> {
    let mut pool = ConstantPool::new();
    if wide {
        for i in 0..300 {
            pool.add_integer(i)?;
        }
    }
    let index = if string {
        pool.add_string(format!("cold {wide} {cancel}"))?
    } else {
        pool.add_class("[I")?
    };
    let descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;";
    let load = if wide {
        Instruction::Ldc_w(index)
    } else {
        Instruction::Ldc(u8::try_from(index)?)
    };
    let method = method(
        &mut pool,
        "run",
        descriptor,
        vec![
            Instruction::Aload_0,
            Instruction::Aconst_null,
            Instruction::Astore_0,
            load,
            Instruction::Pop,
            Instruction::Areturn,
        ],
    )?;
    let class = register(
        thread,
        class_from_methods(
            &format!("ConstantWait{wide}{string}{cancel}"),
            "java/lang/Object",
            pool,
            vec![method],
        )?,
    )
    .await?;
    let function = install_experimental(vm, &class, "run", descriptor)?;
    let gc = vm.garbage_collector();
    let context = RuntimeContext::new(gc, vm, thread, &class)?;
    let value = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let handle = owner(&value)?.as_handle_i64();
    let weak = ristretto_gc::GcWeak::new(owner(&value)?, ristretto_gc::ReferenceKind::Weak, 0);
    let garbage = Value::try_new_object_async(gc, Reference::IntArray(vec![13].into())).await?;
    let garbage_weak =
        ristretto_gc::GcWeak::new(owner(&garbage)?, ristretto_gc::ReferenceKind::Weak, 0);
    drop(garbage);
    let mut activation = function.activation(&[ristretto_jit::Value::Ptr(handle)])?;
    let ExecutionStep::Constant(request) =
        activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?
    else {
        return Err(InternalError("missing constant request".to_owned()));
    };
    assert_eq!(request.index, index);
    assert_eq!(request.bci, 3);
    assert_eq!(activation.suspended_bci(), Some(3));
    assert_eq!(activation.remaining_budget()?, None);
    assert_eq!(activation.call_arguments()?, &[] as &[i64]);
    assert!(
        activation
            .resume_with_context(NonZeroU32::MIN, context.as_ptr())
            .is_err()
    );
    drop(value);
    let loader = vm.class_loader();
    let gate = loader.write().await;
    let mut operation = Box::pin(context.invoke_constant(request, activation.call_arguments()?));
    assert!(
        operation
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
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
    .map_err(|_| InternalError("constant request migration panicked".to_owned()))?;
    if cancel {
        drop(operation);
        drop(activation);
        drop(context);
        collect(gc)?;
        assert!(weak.upgrade().is_none());
        drop(gate);
    } else {
        drop(gate);
        let bits = operation.await?;
        assert_eq!(context.pending_exception(), 0);
        assert_ne!(bits, 0);
        collect(gc)?; // helper-result ownership before generated resumption
        activation.complete_call(bits)?;
        assert!(activation.complete_call(bits).is_err());
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())?,
            ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(handle)))
        );
        let result = context.into_reference_result(handle)?;
        collect(gc)?;
        assert_eq!(&*result.as_int_vec_ref()?, &[77]);
        drop(result);
        drop(activation);
        collect(gc)?;
        assert!(weak.upgrade().is_none());
    }
    assert_eq!(thread.frames()?.len(), 0);
    Ok(())
}

async fn matrix() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            for workers in [0, 1] {
                let (vm, thread) = vm(concurrent, compact, workers).await?;
                semantics(&vm, &thread).await?;
                for wide in [false, true] {
                    for string in [false, true] {
                        for cancel in [false, true] {
                            suspended_resolution(&vm, &thread, wide, string, cancel).await?;
                        }
                    }
                }
                println!(
                    "constant matrix passed concurrent={concurrent} compact={compact} workers={workers}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn constants_preserve_semantics_and_suspended_roots() -> Result<()> {
    const CHILD: &str = "RISTRETTO_CONSTANT_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(matrix());
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit_runtime_helpers::constant_continuation_tests::constants_preserve_semantics_and_suspended_roots", "--nocapture"]).env(CHILD, "1");
    let output = ristretto_test_util::process::run(&mut command, Duration::from_secs(180))?;
    output.require_success()?;
    println!("{}", output.stdout);
    Ok(())
}
