use super::*;
type Result<T> = std::result::Result<T, Error>;
use ristretto_classfile::ClassFile;
use std::future::Future;
use std::task::{Context as TaskContext, Waker};

fn program() -> Result<Program> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsSmall.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsSmall$Node.class"
        ))?,
    ])
}
fn configuration() -> Configuration {
    Configuration {
        objects: 32,
        roots: 32,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}
fn execution<'c, 'p, 'r, 'gc>(
    compiled: &'c CompiledProgram<'p>,
    runtime: &'r Runtime<'gc>,
    limits: CompiledLimits,
) -> Result<CompiledExecution<'c, 'p, 'r, 'gc>> {
    Ok(CompiledExecution {
        compiled,
        execution: Execution {
            program: compiled.program,
            runtime,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            limits: limits.execution_limits(),
            report: Report::default(),
            recording: Recording::default(),
            statics: crate::canonical::statics::States::default(),
        },
        table: Rc::new(RefCell::new(Table::new(
            configuration().objects,
            limits.frames,
            compiled.roots_per_frame,
        )?)),
        slice_limit: limits.native_slices,
    })
}

#[tokio::test]
async fn cancelling_native_initializer_drops_frame_roots_under_table_borrow_and_poison_is_sticky()
-> Result<()> {
    let program = program()?;
    let compiled = program.compile()?;
    let class = program.class_index("CanonicalStaticRootsSmall")?;
    for with_published_token in [false, true] {
        let collector = GarbageCollector::new();
        let budget = Budget::new(configuration().charged_bytes()?);
        let runtime = Runtime::new(&collector, configuration(), &budget)?;
        let mut execution = execution(&compiled, &runtime, CompiledLimits::default())?;
        let table = execution.table.clone();
        let mut initializing = Box::pin(entry(&mut execution, class, "prepared"));
        let mut context = TaskContext::from_waker(Waker::noop());
        let mut found = false;
        for _ in 0..20_000 {
            assert!(initializing.as_mut().poll(&mut context).is_pending());
            if !with_published_token || table.borrow().live_objects() > 0 {
                found = true;
                break;
            }
        }
        assert!(found, "initializer never reached selected pending stage");
        if with_published_token {
            assert!(collector.managed_bytes() > 0);
        }
        {
            let borrowed = table.borrow_mut();
            drop(initializing);
            assert_eq!(borrowed.live_objects(), 0);
        }
        let entries = compiled
            .method_profiles()?
            .iter()
            .map(|profile| profile.entries)
            .sum::<u64>();
        assert!(matches!(
            entry(&mut execution, class, "prepared").await,
            Err(Error::Boundary(
                "class initialization was aborted by embedding"
            ))
        ));
        assert_eq!(
            compiled
                .method_profiles()?
                .iter()
                .map(|profile| profile.entries)
                .sum::<u64>(),
            entries
        );
        if with_published_token {
            assert_eq!(execution.execution.report.class_initializer_calls, 1);
            assert!(execution.execution.report.native_slices > 0);
            // The unfinished initializer's installed statics remain registry-owned.
            assert!(collector.managed_bytes() > 0);
        }
        execution.execution.statics.close();
        execution.execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        let report = &execution.execution.report;
        assert!(report.released_objects >= report.allocations);
        assert!(report.released_objects <= report.allocations + 1);
        drop((execution, table));
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn native_initializer_frame_and_slice_limits_abort_without_interpreter_fallback() -> Result<()>
{
    let program = Program::from_classfiles([ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalStaticProtocol$Bad.class"
    ))?])?;
    let compiled = program.compile()?;
    let class = program.class_index("CanonicalStaticProtocol$Bad")?;
    for (limits, _depth, expected) in [
        (
            CompiledLimits {
                frames: 1,
                ..CompiledLimits::default()
            },
            1,
            "frame",
        ),
        (
            CompiledLimits {
                native_slices: 0,
                ..CompiledLimits::default()
            },
            0,
            "native slice",
        ),
        (
            CompiledLimits {
                native_slices: 1,
                ..CompiledLimits::default()
            },
            0,
            "native slice",
        ),
    ] {
        let collector = GarbageCollector::new();
        let budget = Budget::new(configuration().charged_bytes()?);
        let runtime = Runtime::new(&collector, configuration(), &budget)?;
        let mut execution = execution(&compiled, &runtime, limits)?;
        assert!(
            matches!(entry(&mut execution, class, "run").await,Err(Error::Limit(reason)) if reason==expected)
        );
        assert!(matches!(
            entry(&mut execution, class, "run").await,
            Err(Error::Boundary(
                "class initialization was aborted by embedding"
            ))
        ));
        assert_eq!(execution.table.borrow().live_objects(), 0);
        assert!(!execution.execution.report.instruction_count_available);
        assert_eq!(execution.execution.report.instructions, 0);
        execution.execution.statics.close();
        execution.execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

async fn entry(
    execution: &mut CompiledExecution<'_, '_, '_, '_>,
    class: usize,
    method: &str,
) -> Result<Option<Value>> {
    let method = execution
        .compiled
        .program
        .layout(class)?
        .class
        .try_get_method(method, "(I)I")?;
    execution.call(class, method, vec![Value::Int(0)], 0).await
}
