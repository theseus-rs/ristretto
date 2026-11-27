//! Execute the shared monitor ownership/cancellation contract on native and WASI.
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, JAVA_5, Method, MethodAccessFlags,
};
use ristretto_classloader::{ClassPath, ClassPathEntry, Memory, Reference, Value};
use ristretto_gc::{GarbageCollector, GcWeak, ReferenceKind};
use ristretto_vm::{ConfigurationBuilder, Result, Thread, VM};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

async fn class_path() -> Result<ClassPath> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("PortableMonitor")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let code_name = pool.add_utf8("Code")?;
    let method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: pool.add_utf8("spin")?,
        descriptor_index: pool.add_utf8("([I)V")?,
        attributes: vec![Attribute::Code {
            name_index: code_name,
            max_stack: 2,
            max_locals: 2,
            code: vec![
                Instruction::Aload_0,
                Instruction::Monitorenter,
                Instruction::Aload_0,
                Instruction::Monitorenter,
                Instruction::Aload_0,
                Instruction::Iconst_0,
                Instruction::Iaload,
                Instruction::Ifeq(-3),
                Instruction::Aload_0,
                Instruction::Monitorexit,
                Instruction::Aload_0,
                Instruction::Monitorexit,
                Instruction::Return,
                Instruction::Astore_1,
                Instruction::Aload_0,
                Instruction::Monitorexit,
                Instruction::Aload_0,
                Instruction::Monitorexit,
                Instruction::Aload_1,
                Instruction::Athrow,
            ],
            exception_table: vec![ExceptionTableEntry {
                range_pc: 4..10,
                handler_pc: 15,
                catch_type: 0,
            }],
            attributes: vec![],
        }],
    };
    let file = ClassFile {
        version: JAVA_5,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        constant_pool: pool,
        this_class,
        super_class,
        methods: vec![method],
        ..Default::default()
    };
    let mut bytes = Vec::new();
    file.to_bytes(&mut bytes)?;
    let memory = Memory::new("portable-monitors");
    memory.add_class(&bytes).await?;
    Ok(ClassPath::new(vec![ClassPathEntry::Memory(memory)]))
}

fn owner(value: &Value) -> Result<&ristretto_gc::Gc<ristretto_gc::sync::RwLock<Reference>>> {
    match value {
        Value::Object(Some(owner)) => Ok(owner),
        _ => Err(ristretto_vm::Error::InternalError(
            "missing monitor object".to_owned(),
        )),
    }
}

fn monitor(vm: &VM, value: &Value) -> Result<Arc<ristretto_types::monitor::Monitor>> {
    let id = ristretto_intrinsics::get_monitor_id(owner(value)?)
        .ok_or_else(|| ristretto_vm::Error::InternalError("missing monitor identity".to_owned()))?;
    Ok(ristretto_types::VM::monitor_registry(vm).monitor(id))
}

#[expect(
    clippy::panic_in_result_fn,
    reason = "integration assertions report invariant failures while setup errors propagate"
)]
#[tokio::test]
async fn monitor_wait_cancel_release_and_reclaim() -> Result<()> {
    for concurrent in [false, true] {
        let gc = GarbageCollector::with_config(
            ristretto_gc::Configuration::builder()
                .threads(0)
                .concurrent_publication(concurrent)
                .root_storage(ristretto_gc::RootStorage::Contexts)
                .max_heap_bytes(32 * 1024 * 1024)
                .build(),
        );
        let mut config = ConfigurationBuilder::new()
            .garbage_collector(gc.clone())
            .class_path(class_path().await?);
        if let Ok(home) = std::env::var("RISTRETTO_GC_JAVA_HOME") {
            config = config.java_home(home.into());
        }
        let vm = VM::new(config.build()?).await?;
        gc.stop()?;
        assert_eq!(gc.registered_workers(), 0);
        let thread = Thread::new(&Arc::downgrade(&vm), 333);
        let class = vm.class("PortableMonitor").await?;
        let method = class.try_get_method("spin", "([I)V")?;
        assert!(!ristretto_jit::Compiler::can_compile(method.definition()));
        let array = Value::try_new_object_async(&gc, Reference::IntArray(vec![0].into())).await?;
        let lock = monitor(&vm, &array)?;
        let weak = GcWeak::new(owner(&array)?, ReferenceKind::Weak, 0);
        let parameters = [array];
        let mut execution = Box::pin(thread.execute(&class, &method, &parameters));
        assert!(
            execution
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert!(
            lock.is_owned_by(thread.id()),
            "the bytecode must acquire its monitor"
        );
        let mut contender = Box::pin(lock.acquire(999));
        assert!(
            contender
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        let garbage = Value::try_new_object_async(&gc, Reference::IntArray(vec![7].into())).await?;
        let garbage_weak = GcWeak::new(owner(&garbage)?, ReferenceKind::Weak, 0);
        drop(garbage);
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert!(garbage_weak.upgrade().is_none());
        drop(execution);
        assert!(thread.frames()?.is_empty());
        assert!(!lock.is_owned_by(thread.id()));
        assert!(matches!(
            contender
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(()))
        ));
        assert!(
            lock.release(999)?,
            "both recursive entries must be released"
        );
        drop(contender);
        drop(parameters);
        gc.collect_full()?;
        gc.collect_full()?;
        assert!(weak.upgrade().is_none());

        let array = Value::try_new_object_async(&gc, Reference::IntArray(vec![1].into())).await?;
        let lock = monitor(&vm, &array)?;
        lock.acquire(999).await?;
        let parameters = [array];
        let mut waiting = Box::pin(thread.execute(&class, &method, &parameters));
        assert!(
            waiting
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        drop(waiting);
        assert!(lock.is_owned_by(999));
        assert!(lock.release(999)?);
        assert_eq!(thread.execute(&class, &method, &parameters).await?, None);
        assert!(!lock.is_owned_by(thread.id()));
        assert!(thread.frames()?.is_empty());

        let empty = Value::try_new_object_async(&gc, Reference::IntArray(vec![].into())).await?;
        let lock = monitor(&vm, &empty)?;
        let result = thread.execute(&class, &method, &[empty]).await;
        assert!(
            matches!(result, Err(ristretto_vm::Error::Throwable(_))),
            "{result:?}"
        );
        assert!(!lock.is_owned_by(thread.id()));
        lock.acquire(999).await?;
        assert!(lock.release(999)?);
        println!("portable monitor cancellation/reclamation passed: concurrent={concurrent}");
    }
    Ok(())
}
