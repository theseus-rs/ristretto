//! Shared array initialization must yield and release cancelled storage on native/WASI.
use ristretto_classfile::attributes::{ArrayType, Attribute, Instruction};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, JAVA_5, Method, MethodAccessFlags,
};
use ristretto_classloader::{ClassPath, ClassPathEntry, Memory, Value};
use ristretto_gc::GarbageCollector;
use ristretto_vm::{ConfigurationBuilder, Result, Thread, VM};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Waker};

async fn class_path() -> Result<ClassPath> {
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("PortableAllocations")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let wide = pool.add_class("[[I")?;
    let deep = pool.add_class("[[[I")?;
    let mut methods = Vec::new();
    for (name, descriptor, code, slots) in [
        (
            "bytes",
            "(I)[B",
            vec![
                Instruction::Iload_0,
                Instruction::Newarray(ArrayType::Byte),
                Instruction::Areturn,
            ],
            1,
        ),
        (
            "wide",
            "(II)[[I",
            vec![
                Instruction::Iload_0,
                Instruction::Iload_1,
                Instruction::Multianewarray(wide, 2),
                Instruction::Areturn,
            ],
            2,
        ),
        (
            "deep",
            "(III)[[[I",
            vec![
                Instruction::Iload_0,
                Instruction::Iload_1,
                Instruction::Iload_2,
                Instruction::Multianewarray(deep, 3),
                Instruction::Areturn,
            ],
            3,
        ),
    ] {
        methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: pool.add_utf8(name)?,
            descriptor_index: pool.add_utf8(descriptor)?,
            attributes: vec![Attribute::Code {
                name_index: pool.add_utf8("Code")?,
                max_stack: slots,
                max_locals: slots,
                code,
                exception_table: vec![],
                attributes: vec![],
            }],
        });
    }
    let file = ClassFile {
        version: JAVA_5,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        constant_pool: pool,
        this_class,
        super_class,
        methods,
        ..Default::default()
    };
    let mut bytes = Vec::new();
    file.to_bytes(&mut bytes)?;
    let memory = Memory::new("portable-allocations");
    memory.add_class(&bytes).await?;
    Ok(ClassPath::new(vec![ClassPathEntry::Memory(memory)]))
}

fn collect(gc: &GarbageCollector) -> Result<()> {
    gc.collect_full()?;
    gc.collect_full()?;
    Ok(())
}

#[expect(
    clippy::panic_in_result_fn,
    clippy::too_many_lines,
    reason = "integration assertions report cancellation and content invariants across configurations"
)]
#[tokio::test]
async fn array_initialization_yields_cancels_and_completes() -> Result<()> {
    for concurrent in [false, true] {
        for compact in [false, true] {
            let gc = GarbageCollector::with_config(
                ristretto_gc::Configuration::builder()
                    .threads(0)
                    .concurrent_publication(concurrent)
                    .compact_heap(compact)
                    .root_storage(ristretto_gc::RootStorage::Contexts)
                    .max_heap_bytes(32 * 1024 * 1024)
                    .allocation_threshold(usize::MAX)
                    .build(),
            );
            let mut config = ConfigurationBuilder::new()
                .garbage_collector(gc.clone())
                .interpreted(true)
                .class_path(class_path().await?);
            if let Ok(home) = std::env::var("RISTRETTO_GC_JAVA_HOME") {
                config = config.java_home(home.into());
            }
            let vm = VM::new(config.build()?).await?;
            gc.stop()?;
            assert_eq!(gc.registered_workers(), 0);
            let thread = Thread::new(&Arc::downgrade(&vm), 444);
            let class = vm.class("PortableAllocations").await?;
            for (name, descriptor, arguments, small, bci) in [
                (
                    "bytes",
                    "(I)[B",
                    vec![Value::Int(1024 * 1024)],
                    vec![Value::Int(1)],
                    1,
                ),
                (
                    "wide",
                    "(II)[[I",
                    vec![Value::Int(512), Value::Int(0)],
                    vec![Value::Int(1), Value::Int(0)],
                    2,
                ),
                (
                    "deep",
                    "(III)[[[I",
                    vec![Value::Int(16), Value::Int(8), Value::Int(0)],
                    vec![Value::Int(1), Value::Int(1), Value::Int(0)],
                    3,
                ),
            ] {
                let method = class.try_get_method(name, descriptor)?;
                // Complete class/array resolution first. Tiny leaf arrays do no
                // byte-based initialization yields; node-budget yields must remain.
                drop(thread.execute(&class, &method, &small).await?);
                collect(&gc)?;
                let bytes = gc.managed_bytes();
                let occupied = gc
                    .region_heap()
                    .map(|heap| heap.statistics().occupied_bytes);
                let mut execution = Box::pin(thread.execute(&class, &method, &arguments));
                let mut cx = Context::from_waker(Waker::noop());
                assert!(
                    execution.as_mut().poll(&mut cx).is_pending(),
                    "{name} monopolized one poll"
                );
                assert_eq!(thread.current_frame()?.program_counter(), bci);
                assert!(
                    execution.as_mut().poll(&mut cx).is_pending(),
                    "{name} failed to share budget across recursion"
                );
                drop(execution);
                assert!(thread.frames()?.is_empty());
                collect(&gc)?;
                assert!(
                    gc.managed_bytes() <= bytes,
                    "{name} leaked cancelled logical storage"
                );
                if let Some(occupied) = occupied {
                    assert!(
                        gc.region_heap()
                            .is_some_and(|heap| heap.statistics().occupied_bytes <= occupied),
                        "{name} leaked cancelled ranges"
                    );
                }
                let result = thread
                    .execute(&class, &method, &arguments)
                    .await?
                    .ok_or_else(|| {
                        ristretto_vm::Error::InternalError("missing array".to_owned())
                    })?;
                match name {
                    "bytes" => {
                        let elements = result.as_byte_vec_ref()?;
                        assert_eq!(elements.len(), 1024 * 1024);
                        assert!(elements.iter().all(|value| *value == 0));
                    }
                    "wide" => {
                        let (_, children) = result.as_class_vec_ref()?;
                        assert_eq!(children.len(), 512);
                        for child in children.iter() {
                            assert!(child.as_int_vec_ref()?.is_empty());
                        }
                    }
                    "deep" => {
                        let (_, children) = result.as_class_vec_ref()?;
                        assert_eq!(children.len(), 16);
                        for child in children.iter() {
                            let (_, leaves) = child.as_class_vec_ref()?;
                            assert_eq!(leaves.len(), 8);
                            for leaf in leaves.iter() {
                                assert!(leaf.as_int_vec_ref()?.is_empty());
                            }
                        }
                    }
                    _ => {
                        return Err(ristretto_vm::Error::InternalError(
                            "unknown allocation fixture".to_owned(),
                        ));
                    }
                }
                drop(result);
                collect(&gc)?;
                assert!(gc.managed_bytes() <= bytes);
            }
            println!(
                "portable allocation cooperation passed concurrent={concurrent} compact={compact}"
            );
        }
    }
    Ok(())
}
