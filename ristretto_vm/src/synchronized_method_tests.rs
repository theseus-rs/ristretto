//! GC ownership at the ordinary synchronized-method dispatch boundary.
use super::*;
use crate::ConfigurationBuilder;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool};
use ristretto_gc::{GarbageCollector, GcWeak, ReferenceKind};
use std::task::{Context, Waker};

// An embedding can transfer its only owner during argument conversion.
#[derive(Debug)]
struct MoveArgument(Mutex<Option<Value>>);

impl RustValue for MoveArgument {
    #[expect(
        clippy::expect_used,
        reason = "the fixture transfers its only argument once"
    )]
    fn to_value(&self, _: &Arc<GarbageCollector>) -> Value {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
            .expect("argument already transferred")
    }
}

async fn fixture() -> Result<(Arc<VM>, Arc<Thread>, Arc<Class>)> {
    let config = ConfigurationBuilder::new()
        .interpreted(true)
        .verify_mode(VerifyMode::None);
    #[cfg(target_os = "wasi")]
    let config = config.java_home("/jdk".into());
    let vm = VM::new(config.build()?).await?;
    let thread = Thread::new(&Arc::downgrade(&vm), 1);
    let mut pool = ConstantPool::default();
    let this_class = pool.add_class("ImplicitMonitorOwner")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let held = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        name_index: pool.add_utf8("held")?,
        descriptor_index: pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: pool.add_utf8("Code")?,
            max_stack: 1,
            max_locals: 1,
            code: vec![
                Instruction::Aconst_null,
                Instruction::Astore_0,
                Instruction::Return,
            ],
            exception_table: Vec::new(),
            attributes: Vec::new(),
        }],
    };
    let class = Class::from(
        None,
        ClassFile {
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            constant_pool: pool,
            this_class,
            super_class,
            methods: vec![held],
            ..Default::default()
        },
    )?;
    thread.register_class(class.clone()).await?;
    Ok((vm, thread, class))
}

#[derive(Clone, Copy)]
enum Exit {
    Return,
    CancelHeld,
    CancelWaiting,
    CancelNotified,
}

#[cfg_attr(target_os = "wasi", ignore = "requires a JDK preopened at /jdk")]
#[tokio::test]
async fn synchronized_receiver_survives_local_clear_until_return_or_cancellation() -> Result<()> {
    let (vm, thread, class) = fixture().await?;
    let method = class.try_get_method("held", "()V")?;
    let gc = vm.garbage_collector();
    let other = thread.id() + 1;
    for exit in [
        Exit::Return,
        Exit::CancelHeld,
        Exit::CancelWaiting,
        Exit::CancelNotified,
    ] {
        let object = Value::from_object(gc, Object::new(class.clone())?);
        let Value::Object(Some(reference)) = &object else {
            return Err(InternalError("missing receiver".into()));
        };
        let witness = GcWeak::new(reference, ReferenceKind::Weak, 0);
        let id =
            get_monitor_id(reference).ok_or_else(|| InternalError("missing monitor ID".into()))?;
        let monitor = vm.monitor_registry().monitor(id);
        let waiting = matches!(exit, Exit::CancelWaiting | Exit::CancelNotified);
        if waiting {
            monitor.acquire(other).await?;
        }
        let arguments = [MoveArgument(Mutex::new(Some(object)))];
        thread
            .instruction_yield_count
            .store(INSTRUCTION_YIELD_COUNT - 3, Ordering::Relaxed);
        let mut invocation = Box::pin(thread.execute(&class, &method, &arguments));
        assert!(
            invocation
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert!(arguments[0].0.lock().unwrap().is_none());
        if waiting {
            assert!(thread.frames()?.is_empty());
            assert!(monitor.is_owned_by(other));
        } else {
            assert_eq!(thread.current_frame()?.program_counter(), 2);
            assert!(monitor.is_owned_by(thread.id()));
        }
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(
            witness.upgrade().is_some(),
            "implicit monitor lost its receiver"
        );
        if matches!(exit, Exit::CancelNotified) {
            assert!(monitor.release(other)?);
        }
        if matches!(exit, Exit::Return) {
            assert_eq!(invocation.await?, None);
        } else {
            drop(invocation);
        }
        if matches!(exit, Exit::CancelWaiting) {
            assert!(monitor.release(other)?);
        }
        assert!(!monitor.is_owned_by(thread.id()));
        monitor.acquire(other).await?;
        assert!(monitor.release(other)?);
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(witness.upgrade().is_none());
    }
    Ok(())
}
