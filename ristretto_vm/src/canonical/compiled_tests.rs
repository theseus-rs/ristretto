//! Real generated-frame cancellation while canonical heap ownership is live.
use super::*;
use ristretto_classfile::ClassFile;
use std::future::Future;
use std::task::{Context as TaskContext, Poll, Waker};

#[tokio::test]
async fn cancelling_live_native_graph_releases_frames_tokens_and_payload_under_table_borrow()
-> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    cancel_live(
        &program,
        "CanonicalGraphWorkload",
        "run",
        64,
        small_configuration(),
        1,
    )
    .await
}

#[tokio::test]
async fn cancelling_native_array_return_releases_the_owning_frame() -> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload$Holder.class"
        ))?,
    ])?;
    cancel_live(
        &program,
        "CanonicalPrimitiveArrayWorkload",
        "returned",
        7,
        small_configuration(),
        1,
    )
    .await
}

fn small_configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn cancelling_native_reference_tree_releases_pending_pages_and_frame_tokens() -> Result<()> {
    let classes: [&[u8]; 4] = [
        include_bytes!("../../tests/fixtures/canonical/CanonicalReferenceArrayWorkload.class"),
        include_bytes!("../../tests/fixtures/canonical/CanonicalReferenceArrayWorkload$Node.class"),
        include_bytes!(
            "../../tests/fixtures/canonical/CanonicalReferenceArrayWorkload$Other.class"
        ),
        include_bytes!(
            "../../tests/fixtures/canonical/CanonicalReferenceArrayWorkload$Holder.class"
        ),
    ];
    let program = Program::from_classfiles(
        classes
            .map(ClassFile::from_bytes)
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?,
    )?;
    cancel_live(
        &program,
        "CanonicalReferenceArrayWorkload",
        "returned",
        2048,
        Configuration {
            objects: 512,
            roots: 64,
            mutators: 1,
            helpers: 1,
            edges_per_object: 8,
        },
        3 * size_of::<super::super::Payload>(),
    )
    .await
}

async fn cancel_live(
    program: &Program,
    class: &str,
    method: &str,
    argument: i32,
    configuration: Configuration,
    minimum_bytes: usize,
) -> Result<()> {
    let compiled = program.compile()?;
    let budget = Budget::new(configuration.charged_bytes()?);
    let collector = GarbageCollector::new();
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let table = Rc::new(RefCell::new(Table::new(
        configuration.objects,
        64,
        compiled.roots_per_frame,
    )?));
    let mut execution = CompiledExecution {
        compiled: &compiled,
        execution: Execution {
            program,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            runtime: &runtime,
            limits: CompiledLimits::default().execution_limits(),
            report: Report::default(),
            statics: crate::canonical::statics::States::default(),
        },
        table: table.clone(),
        slice_limit: 100_000,
    };
    let class = program.class_index(class)?;
    let method = program
        .layout(class)?
        .class
        .try_get_method(method, "(I)I")?;
    let mut future = Box::pin(execution.call(class, method, vec![Value::Int(argument)], 0));
    let mut context = TaskContext::from_waker(Waker::noop());
    let mut found = false;
    for _ in 0..2000 {
        assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
        if table.borrow().live_objects() > 0 && collector.managed_bytes() >= minimum_bytes {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "actual canonical root was published by a generated invocation"
    );
    assert!(collector.managed_bytes() > 0);
    {
        let table_guard = table.borrow_mut();
        drop(future);
        assert_eq!(table_guard.live_objects(), 0);
    }
    execution.execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(table);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[tokio::test]
async fn cancelling_native_cast_result_releases_its_single_registered_root() -> Result<()> {
    let classes: [&[u8]; 3] = [
        include_bytes!("../../tests/fixtures/canonical/CanonicalCastWorkload.class"),
        include_bytes!("../../tests/fixtures/canonical/CanonicalCastWorkload$Node.class"),
        include_bytes!("../../tests/fixtures/canonical/CanonicalCastWorkload$Other.class"),
    ];
    let program = Program::from_classfiles(
        classes
            .map(ClassFile::from_bytes)
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?,
    )?;
    let compiled = program.compile()?;
    let configuration = Configuration {
        objects: 4,
        roots: 1,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    };
    let budget = Budget::new(configuration.charged_bytes()?);
    let collector = GarbageCollector::new();
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let table = Rc::new(RefCell::new(Table::new(
        configuration.objects,
        64,
        compiled.roots_per_frame,
    )?));
    let mut execution = CompiledExecution {
        compiled: &compiled,
        execution: Execution {
            program: &program,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            runtime: &runtime,
            limits: CompiledLimits::default().execution_limits(),
            report: Report::default(),
            statics: crate::canonical::statics::States::default(),
        },
        table: table.clone(),
        slice_limit: 100_000,
    };
    let node = program.class_index("CanonicalCastWorkload$Node")?;
    let root = execution.execution.allocate_object(node).await?;
    let class = program.class_index("CanonicalCastWorkload")?;
    let method = program.layout(class)?.class.try_get_method(
        "castNode",
        "(Ljava/lang/Object;)LCanonicalCastWorkload$Node;",
    )?;
    let mut future = Box::pin(execution.call(class, method, vec![Value::Reference(Some(root))], 0));
    let mut context = TaskContext::from_waker(Waker::noop());
    for _ in 0..64 {
        assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
        if table.borrow().live_objects() == 1 {
            break;
        }
    }
    assert_eq!(table.borrow().live_objects(), 1);
    {
        let table_guard = table.borrow_mut();
        drop(future);
        assert_eq!(table_guard.live_objects(), 0);
    }
    assert_eq!(execution.execution.report.native_slices, 1);
    execution.execution.cleanup(&collector).await?;
    assert_eq!(collector.managed_bytes(), 0);
    drop(execution);
    drop(table);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}
