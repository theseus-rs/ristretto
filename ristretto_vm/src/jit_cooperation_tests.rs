//! Exercise infinite native loops in a bounded child: a broken poll cannot hang CI.
use super::*;
use crate::java_object::JavaObject;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classloader::Reference;
use std::future::Future;
use std::task::{Context, Waker};

#[expect(
    clippy::too_many_lines,
    reason = "keep cancellation, roots and monitor lifetime checks in one ordered fixture"
)]
async fn lifecycle(concurrent: bool, workers: usize, mixed: bool) -> Result<()> {
    let (vm, thread) = static_call_tests::vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let mut pool = ConstantPool::new();
    let mut definition = method(
        &mut pool,
        "run",
        "(Ljava/lang/Object;)V",
        vec![Instruction::Nop, Instruction::Goto(0)],
    )?;
    definition
        .access_flags
        .insert(MethodAccessFlags::SYNCHRONIZED);
    let target = pool.add_class("CooperativeInfiniteLoop")?;
    let target = pool.add_method_ref(target, "run", "(Ljava/lang/Object;)V")?;
    let caller = method(
        &mut pool,
        "caller",
        "(Ljava/lang/Object;)V",
        vec![
            Instruction::Aload_0,
            Instruction::Invokestatic(target),
            Instruction::Return,
        ],
    )?;
    let done = method(
        &mut pool,
        "done",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    let class = register(
        &thread,
        class_from_methods(
            "CooperativeInfiniteLoop",
            "java/lang/Object",
            pool,
            vec![definition, caller, done],
        )?,
    )
    .await?;
    let mirror = class.to_object(&thread).await?;
    let Value::Object(Some(mirror_owner)) = &mirror else {
        return Err(InternalError("missing class mirror".to_owned()));
    };
    let monitor_id = ristretto_intrinsics::get_monitor_id(mirror_owner)
        .ok_or_else(|| InternalError("missing monitor identity".to_owned()))?;
    let monitor = vm.monitor_registry().monitor(monitor_id);
    let method = class.try_get_method("run", "(Ljava/lang/Object;)V")?;
    let function = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?
        .compile(&class, &method)
        .await?
        .ok_or_else(|| InternalError("production loop compilation failed".to_owned()))?;
    assert!(function.supports_cooperation());
    let method = if mixed {
        class.try_get_method("caller", "(Ljava/lang/Object;)V")?
    } else {
        method
    };
    let argument = Value::try_new_object_async(gc, Reference::IntArray(vec![77].into())).await?;
    let Value::Object(Some(owner)) = &argument else {
        return Err(InternalError("missing argument owner".to_owned()));
    };
    let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    let call_thread = thread.clone();
    let call_class = class.clone();
    let mut execution = Box::pin(async move {
        let parameters = [argument];
        call_thread.execute(&call_class, &method, &parameters).await
    });
    assert!(
        execution
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(thread.frames()?.len(), if mixed { 2 } else { 1 });
    assert_eq!(thread.current_frame()?.program_counter(), 1);
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        assert_eq!(function.resume_count(), 0);
    }
    // A paused frame owns its unused reference argument, but does not pin unrelated garbage.
    let unrelated = Value::try_new_object_async(gc, Reference::IntArray(vec![99].into())).await?;
    let Value::Object(Some(owner)) = &unrelated else {
        return Err(InternalError("missing unrelated owner".to_owned()));
    };
    let unrelated_weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    drop(unrelated);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    assert!(unrelated_weak.upgrade().is_none());
    #[cfg(feature = "execution-profile")]
    let checkpoint = vm.execution_profile_checkpoint("between native slices");
    let runtime = tokio::runtime::Handle::current();
    let execution = std::thread::spawn(move || {
        let _runtime = runtime.enter();
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        execution
    })
    .join()
    .map_err(|_| InternalError("continuation migration panicked".to_owned()))?;
    assert_eq!(thread.current_frame()?.program_counter(), 1);
    #[cfg(feature = "execution-profile")]
    {
        let interval = vm.execution_profile_since(checkpoint)?;
        let record = interval
            .methods
            .iter()
            .find(|record| {
                record.activity.class == "CooperativeInfiniteLoop" && record.activity.name == "run"
            })
            .ok_or_else(|| InternalError("missing loop interval".to_owned()))?;
        assert_eq!(record.activity.compiled_entries, 0);
        assert_eq!(record.activity.compiled_resumes, 1);
        assert_eq!(
            record.activity.compiled_backedges,
            u64::from(function.cooperation_budget().get())
        );
    }
    let mut acquire = Box::pin(monitor.acquire(thread.id() + 1));
    assert!(
        acquire
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(execution); // Cancel with no generated frame active on either OS thread.
    assert!(thread.frames()?.is_empty());
    assert!(
        acquire
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_ready()
    );
    monitor.release(thread.id() + 1)?;
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_none());
    assert_eq!(
        thread
            .execute(
                &class,
                &class.try_get_method("done", "()I")?,
                &[] as &[Value]
            )
            .await?,
        Some(Value::Int(42))
    );
    assert!(thread.frames()?.is_empty());
    Ok(())
}

async fn task_abort() -> Result<()> {
    let (vm, thread) = crate::test::thread().await?;
    let mut pool = ConstantPool::new();
    let definition = method(&mut pool, "run", "()V", vec![Instruction::Goto_w(0)])?;
    let class = register(
        &thread,
        class_from_methods(
            "CooperativeTaskAbort",
            "java/lang/Object",
            pool,
            vec![definition],
        )?,
    )
    .await?;
    let method = class.try_get_method("run", "()V")?;
    let function = vm
        .compiler()
        .ok_or_else(|| InternalError("missing compiler".to_owned()))?
        .compile(&class, &method)
        .await?
        .ok_or_else(|| InternalError("missing native loop".to_owned()))?;
    assert!(function.supports_cooperation());
    let started = Arc::new(tokio::sync::Notify::new());
    let entered = started.clone();
    let call_thread = thread.clone();
    let task = tokio::spawn(async move {
        let arguments: [Value; 0] = [];
        let mut execution = Box::pin(call_thread.execute(&class, &method, &arguments));
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        entered.notify_one();
        execution.await
    });
    started.notified().await;
    // This child has one Tokio worker: the infinite compiled loop must yield it.
    assert_eq!(
        tokio::spawn(async { 42 })
            .await
            .map_err(|error| InternalError(error.to_string()))?,
        42
    );
    task.abort();
    let result = task.await;
    assert!(result.is_err_and(|error| error.is_cancelled()));
    assert!(thread.frames()?.is_empty());
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    Ok(())
}

#[test]
fn native_loop_cancellation_migration_and_reclamation() -> Result<()> {
    const CHILD: &str = "RISTRETTO_PRIMITIVE_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?;
        return runtime.block_on(async {
            for concurrent in [false, true] {
                for workers in [0, 1] {
                    for mixed in [false, true] {
                        lifecycle(concurrent, workers, mixed).await?;
                    }
                }
            }
            task_abort().await
        });
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit::primitive_control_flow_tests::cooperation::native_loop_cancellation_migration_and_reclamation", "--nocapture"])
        .env(CHILD, "1");
    ristretto_test_util::process::run(&mut command, Duration::from_secs(60))?.require_success()?;
    Ok(())
}
