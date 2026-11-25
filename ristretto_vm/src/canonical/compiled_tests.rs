//! Real generated-frame cancellation while canonical heap ownership is live.
use super::*;
use ristretto_classfile::ClassFile;
use std::future::Future;
use std::task::{Poll, Waker};

#[tokio::test]
async fn cancelling_live_native_graph_releases_frames_tokens_and_payload_under_table_borrow()
-> Result<(), Error> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    let compiled = program.compile()?;
    let configuration = Configuration {
        objects: 4,
        roots: 16,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
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
    let class = program.class_index("CanonicalGraphWorkload")?;
    let method = program.layout(class)?.class.try_get_method("run", "(I)I")?;
    let mut future = Box::pin(execution.call(class, method, vec![Value::Int(64)], 0));
    let mut context = std::task::Context::from_waker(Waker::noop());
    let mut found = false;
    for _ in 0..2000 {
        assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
        if table.borrow().live_objects() > 0 {
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
