//! Experimental reference continuations through real dispatch and cancellation.
use super::*;
use crate::java_object::JavaObject;
use ristretto_classfile::attributes::ArrayType;
use ristretto_classfile::{FieldAccessFlags, FieldType, MethodAccessFlags};
use std::future::Future;
use std::task::{Context, Waker};

async fn fixture(thread: &Thread) -> Result<Arc<Class>> {
    let mut pool = ConstantPool::new();
    let target = pool.add_class("CooperativeReferenceLoop")?;
    let field = ristretto_classfile::Field {
        access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
        name_index: pool.add_utf8("observed")?,
        descriptor_index: pool.add_utf8("[I")?,
        field_type: FieldType::parse("[I")?,
        attributes: vec![],
    };
    let field_ref = pool.add_field_ref(target, "observed", "[I")?;
    let mut run = method(
        &mut pool,
        "run",
        "()V",
        vec![
            Instruction::Iconst_1,
            Instruction::Newarray(ArrayType::Int), // 0, 1
            Instruction::Dup,
            Instruction::Putstatic(field_ref), // 3, 4
            Instruction::Goto(0), // 7: the sole remaining owner must be the frame context
        ],
    )?;
    run.access_flags.insert(MethodAccessFlags::SYNCHRONIZED);
    let target = pool.add_method_ref(target, "run", "()V")?;
    let caller = method(
        &mut pool,
        "caller",
        "()V",
        vec![Instruction::Invokestatic(target), Instruction::Return],
    )?;
    let done = method(
        &mut pool,
        "done",
        "()I",
        vec![Instruction::Bipush(42), Instruction::Ireturn],
    )?;
    let class = class_from_methods(
        "CooperativeReferenceLoop",
        "java/lang/Object",
        pool,
        vec![run, caller, done],
    )?;
    let mut definition = class.class_file().clone();
    definition.fields.push(field);
    register(thread, Class::from(None, definition)?).await
}

#[expect(
    clippy::too_many_lines,
    reason = "ordered suspension, migration and cancellation ownership checks"
)]
async fn lifecycle(concurrent: bool, workers: usize, mixed: bool) -> Result<()> {
    let (vm, thread) = static_call_tests::vm_with_workers(concurrent, workers).await?;
    let gc = vm.garbage_collector();
    let class = fixture(&thread).await?;
    let function = static_call_tests::install_experimental(&vm, &class, "run", "()V")?;
    assert!(function.cooperation_requires_context());
    let mirror = class.to_object(&thread).await?;
    let Value::Object(Some(mirror_owner)) = &mirror else {
        return Err(InternalError("missing class mirror".to_owned()));
    };
    let monitor_id = ristretto_intrinsics::get_monitor_id(mirror_owner)
        .ok_or_else(|| InternalError("missing monitor identity".to_owned()))?;
    let monitor = vm.monitor_registry().monitor(monitor_id);
    let method = class.try_get_method(if mixed { "caller" } else { "run" }, "()V")?;
    let call_thread = thread.clone();
    let call_class = class.clone();
    let mut execution = Box::pin(async move {
        call_thread
            .execute(&call_class, &method, &[] as &[Value])
            .await
    });
    assert!(
        execution
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(thread.frames()?.len(), if mixed { 2 } else { 1 });
    assert_eq!(thread.current_frame()?.program_counter(), 7);
    let observed = class.static_value("observed")?;
    let Value::Object(Some(owner)) = &observed else {
        return Err(InternalError(
            "generated allocation was not observed".to_owned(),
        ));
    };
    let weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    class.set_static_value("observed", Value::Object(None))?;
    drop(observed);
    let unrelated = Value::try_new_object_async(
        gc,
        ristretto_classloader::Reference::IntArray(vec![99].into()),
    )
    .await?;
    let Value::Object(Some(owner)) = &unrelated else {
        return Err(InternalError("missing unrelated owner".to_owned()));
    };
    let unrelated_weak = ristretto_gc::GcWeak::new(owner, ristretto_gc::ReferenceKind::Weak, 0);
    drop(unrelated);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_some(), "suspended operand lost its owner");
    assert!(
        unrelated_weak.upgrade().is_none(),
        "suspended context blocked reclamation"
    );
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
    .map_err(|_| InternalError("reference future migration panicked".to_owned()))?;
    assert_eq!(thread.current_frame()?.program_counter(), 7);
    gc.collect_full()?;
    gc.collect_full()?;
    assert!(weak.upgrade().is_some());
    #[cfg(feature = "execution-profile")]
    {
        assert_eq!(function.execution_count(), 1);
        // Allocation and static publication each resume before the loop's yield.
        assert_eq!(function.resume_count(), 3);
    }
    let mut acquire = Box::pin(monitor.acquire(thread.id() + 1));
    assert!(
        acquire
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(execution);
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
    assert!(
        weak.upgrade().is_none(),
        "cancelled context retained its operand root"
    );
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

#[test]
fn suspended_reference_future_releases_roots_frames_and_monitors() -> Result<()> {
    const CHILD: &str = "RISTRETTO_REFERENCE_CONTINUATION_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(async {
                for concurrent in [false, true] {
                    for workers in [0, 1] {
                        for mixed in [false, true] {
                            lifecycle(concurrent, workers, mixed).await?;
                        }
                    }
                }
                Ok(())
            });
    }
    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args(["--exact", "jit::primitive_control_flow_tests::reference_cooperation::suspended_reference_future_releases_roots_frames_and_monitors", "--nocapture"]).env(CHILD, "1");
    ristretto_test_util::process::run(&mut command, Duration::from_secs(60))?.require_success()?;
    Ok(())
}
