//! Real helper/root ownership between native slices; production remains gated.
use super::*;
use crate::jit::static_call_tests::{class_from_methods, method, register, vm_with_workers};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ArrayType, Instruction};
use ristretto_jit::{Activation, ExecutionStep};
use std::num::NonZeroU32;

#[path = "handler_continuation_tests.rs"]
mod handlers;

type WeakReference = ristretto_gc::GcWeak<RwLock<Reference>>;

fn only_root(context: &RuntimeContext) -> Result<WeakReference> {
    let roots = context.reference_roots.lock();
    assert_eq!(
        roots.len(),
        1,
        "dead helpers or primitive cells retained as roots"
    );
    let owner = roots
        .values()
        .next()
        .ok_or_else(|| InternalError("missing root".to_owned()))?;
    Ok(ristretto_gc::GcWeak::new(
        owner,
        ristretto_gc::ReferenceKind::Weak,
        0,
    ))
}

fn collect_twice(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

/// Drive real runtime requests while these earlier tests inspect loop/handler polls.
async fn resume_allocations(
    activation: &mut Activation,
    context: &RuntimeContext,
) -> Result<ExecutionStep> {
    loop {
        match activation.resume_with_context(NonZeroU32::MIN, context.as_ptr())? {
            ExecutionStep::ExceptionMatch(request) => {
                let bits = context
                    .invoke_exception_match(request, activation.call_arguments()?)
                    .await?;
                activation.complete_call(bits)?;
            }
            ExecutionStep::Allocation(request) => {
                let bits = context
                    .invoke_allocation(request, activation.call_arguments()?)
                    .await?;
                activation.complete_call(bits)?;
            }
            ExecutionStep::ArrayAccess(request) => {
                let bits = context
                    .invoke_array_access(request, activation.call_arguments()?)
                    .await?;
                activation.complete_call(bits)?;
            }
            ExecutionStep::Field(request) => {
                let bits = context
                    .invoke_field(request, activation.call_arguments()?)
                    .await?;
                activation.complete_call(bits)?;
            }
            step => return Ok(step),
        }
    }
}

fn migrate(
    mut activation: Activation,
    context: RuntimeContext,
) -> Result<(Activation, RuntimeContext, ExecutionStep)> {
    let runtime = Handle::current();
    std::thread::spawn(move || {
        let _runtime = runtime.enter();
        let result = runtime.block_on(resume_allocations(&mut activation, &context))?;
        Ok((activation, context, result))
    })
    .join()
    .map_err(|_| InternalError("reference continuation migration panicked".to_owned()))?
}

fn allocating_loop(operand: bool, fail: bool) -> Vec<Instruction> {
    let mut code = if operand {
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Dup,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore, // 7; retain the original reference operand
            Instruction::Iload_0,
            Instruction::Ifeq(15), // 8, 9 -> 24
            Instruction::Sipush(2048),
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iinc(0, -1),
            Instruction::Goto(-13),
        ]
    } else {
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int),
            Instruction::Astore_1,
            Instruction::Aload_1,
            Instruction::Iconst_0,
            Instruction::Bipush(77),
            Instruction::Iastore, // 8
            Instruction::Iload_0,
            Instruction::Ifeq(15), // 9, 10 -> 25
            Instruction::Sipush(2048),
            Instruction::Newarray(ArrayType::Int),
            Instruction::Pop,
            Instruction::Iinc(0, -1),
            Instruction::Goto(-13),
            Instruction::Aload_1,
        ]
    };
    if fail {
        code.extend([
            Instruction::Pop,
            Instruction::Iconst_m1,
            Instruction::Newarray(ArrayType::Int),
        ]);
    }
    code.push(Instruction::Areturn);
    code
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn live_local_and_operand_survive_collection_migration_and_return() -> Result<()> {
    for concurrent in [false, true] {
        for workers in [0, 1] {
            for operand in [false, true] {
                live_reference(concurrent, workers, operand).await?;
            }
        }
    }
    Ok(())
}

async fn live_reference(concurrent: bool, workers: usize, operand: bool) -> Result<()> {
    let (vm, thread) = vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let definition = method(&mut pool, "run", "(I)[I", allocating_loop(operand, false))?;
    let name = if operand {
        "SuspendedOperand"
    } else {
        "SuspendedLocal"
    };
    let class = register(
        &thread,
        class_from_methods(name, "java/lang/Object", pool, vec![definition])?,
    )
    .await?;
    let method = class.try_get_method("run", "(I)[I")?;
    assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
    let function = ristretto_jit::Compiler::new()?.compile(
        class.class_file(),
        method.definition(),
        &symbols(),
    )?;
    assert!(function.cooperation_requires_context());
    let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
    let mut activation = function.activation(&[ristretto_jit::Value::I32(3)])?;
    // A rejected context does not consume the first slice or count an invocation.
    assert!(activation.resume(NonZeroU32::MIN).is_err());
    assert_eq!(activation.suspended_bci(), None);
    assert_eq!(
        resume_allocations(&mut activation, &context).await?,
        ExecutionStep::Yielded
    );
    let weak = only_root(&context)?;
    collect_twice(gc)?;
    assert!(weak.upgrade().is_some());
    let (mut activation, context, step) = migrate(activation, context)?;
    assert_eq!(step, ExecutionStep::Yielded);
    assert_eq!(
        activation.suspended_bci(),
        Some(if operand { 8 } else { 9 })
    );
    only_root(&context)?;
    collect_twice(gc)?;
    assert!(weak.upgrade().is_some());
    assert_eq!(
        resume_allocations(&mut activation, &context).await?,
        ExecutionStep::Yielded
    );
    collect_twice(gc)?;
    let ExecutionStep::Returned(Some(ristretto_jit::Value::Ptr(handle))) =
        resume_allocations(&mut activation, &context).await?
    else {
        return Err(InternalError(
            "missing continued reference result".to_owned(),
        ));
    };
    let value = context.into_reference_result(handle)?;
    drop(activation);
    collect_twice(gc)?;
    assert_eq!(&*value.as_int_vec_ref()?, &[77]);
    assert!(weak.upgrade().is_some());
    drop(value);
    collect_twice(gc)?;
    assert!(weak.upgrade().is_none());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        // The initial iastore adds one runtime request before the backedges.
        assert_eq!(function.resume_count(), 8);
        assert_eq!(function.backedge_count(), 3);
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn consumed_reference_condition_is_released_before_suspension() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = vm_with_workers(concurrent, 0).await?;
        let gc = vm.garbage_collector();
        let mut pool = ConstantPool::new();
        let definition = method(
            &mut pool,
            "run",
            "()V",
            vec![
                Instruction::Goto(4),
                Instruction::Return, // 0, 3
                Instruction::Iconst_1,
                Instruction::Newarray(ArrayType::Int), // 4, 5
                Instruction::Goto(7),                  // 7 -> 14
                Instruction::Ifnonnull(-7),
                Instruction::Return,   // 10 -> 3, 13
                Instruction::Goto(-4), // 14 -> 10, the only reference operand
            ],
        )?;
        let class = register(
            &thread,
            class_from_methods(
                "ConsumedReference",
                "java/lang/Object",
                pool,
                vec![definition],
            )?,
        )
        .await?;
        let method = class.try_get_method("run", "()V")?;
        let function = ristretto_jit::Compiler::new()?.compile(
            class.class_file(),
            method.definition(),
            &symbols(),
        )?;
        let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
        let mut activation = function.activation(&[])?;
        assert_eq!(
            resume_allocations(&mut activation, &context).await?,
            ExecutionStep::Yielded
        );
        let weak = only_root(&context)?;
        collect_twice(gc)?;
        assert!(weak.upgrade().is_some());
        assert_eq!(
            resume_allocations(&mut activation, &context).await?,
            ExecutionStep::Yielded
        );
        assert_eq!(activation.suspended_bci(), Some(3));
        assert!(context.reference_roots.lock().is_empty());
        collect_twice(gc)?;
        assert!(
            weak.upgrade().is_none(),
            "consumed condition retained while suspended"
        );
        assert_eq!(
            resume_allocations(&mut activation, &context).await?,
            ExecutionStep::Returned(None)
        );
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn resumed_helper_exception_owns_its_throwable_after_context_drop() -> Result<()> {
    for concurrent in [false, true] {
        let (vm, thread) = vm_with_workers(concurrent, 0).await?;
        let gc = vm.garbage_collector();
        let mut pool = ConstantPool::new();
        let definition = method(&mut pool, "run", "(I)[I", allocating_loop(true, true))?;
        let class = register(
            &thread,
            class_from_methods(
                "SuspendedException",
                "java/lang/Object",
                pool,
                vec![definition],
            )?,
        )
        .await?;
        let method = class.try_get_method("run", "(I)[I")?;
        let function = ristretto_jit::Compiler::new()?.compile(
            class.class_file(),
            method.definition(),
            &symbols(),
        )?;
        let context = RuntimeContext::new(gc, &vm, &thread, &class)?;
        let mut activation = function.activation(&[ristretto_jit::Value::I32(1)])?;
        assert_eq!(
            resume_allocations(&mut activation, &context).await?,
            ExecutionStep::Yielded
        );
        let weak = only_root(&context)?;
        assert_eq!(
            resume_allocations(&mut activation, &context).await?,
            ExecutionStep::Returned(None)
        );
        let exception = context
            .take_pending_exception_owner()
            .ok_or_else(|| InternalError("missing resumed exception".to_owned()))?;
        let throwable_weak =
            ristretto_gc::GcWeak::new(&exception, ristretto_gc::ReferenceKind::Weak, 0);
        drop(context);
        drop(activation);
        collect_twice(gc)?;
        assert!(weak.upgrade().is_none());
        assert!(throwable_weak.upgrade().is_some());
        let value = Value::Object(Some(exception));
        assert_eq!(
            value.as_object_ref()?.class().name(),
            "java/lang/NegativeArraySizeException"
        );
        drop(value);
        collect_twice(gc)?;
        assert!(throwable_weak.upgrade().is_none());
    }
    Ok(())
}
