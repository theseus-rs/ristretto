use super::*;
use ristretto_classfile::ClassFile;
type Result<T> = std::result::Result<T, Error>;
use std::future::Future;
use std::task::{Context, Waker};

#[tokio::test]
async fn cancelling_parked_java_frames_releases_all_canonical_roots() -> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    };
    for prefix in [1, 32, 512] {
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let class = program.class_index("CanonicalGraphWorkload")?;
        let method = program.layout(class)?.class.try_get_method("run", "(I)I")?;
        let mut execution = Execution {
            program: &program,
            runtime: &runtime,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            limits: Limits::default(),
            report: Report::default(),
            recording: Recording::default(),
            statics: States::default(),
        };
        let mut future = Box::pin(execution.call(class, method, vec![Value::Int(100)], 0));
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..prefix {
            assert!(future.as_mut().poll(&mut context).is_pending());
        }
        assert!(collector.managed_bytes() > 0);
        drop(future);
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
        drop(execution);
        drop(runtime);
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
#[expect(
    clippy::too_many_lines,
    reason = "combined primitive boundary and cancellation lifecycle"
)]
async fn primitive_array_boundary_accounting_bits_and_cancellation() -> Result<()> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalPrimitiveArrayWorkload$Holder.class"
        ))?,
    ])?;
    let configuration = Configuration {
        objects: 4,
        roots: 8,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    };
    let budget = Budget::new(configuration.charged_bytes()?);
    let collector = GarbageCollector::new();
    let runtime = Runtime::new(&collector, configuration, &budget)?;
    let mut execution = Execution {
        program: &program,
        runtime: &runtime,
        mutator: runtime.mutator()?,
        helper: runtime.helper()?,
        limits: Limits::default(),
        report: Report::default(),
        recording: Recording::default(),
        statics: States::default(),
    };
    let empty = execution.allocate_array(Kind::Double, 0).await?;
    let base_bytes = collector.managed_bytes();
    drop(empty);
    execution.cleanup(&collector).await?;
    let root = execution.allocate_array(Kind::Double, 2).await?;
    assert_eq!(collector.managed_bytes(), base_bytes + 16);
    let bits = 0x7ff8_1234_5678_9abc;
    execution.write_array(&root, 0, Kind::Double, &Value::Double(f64::from_bits(bits)))?;
    assert_eq!(
        execution
            .read_array(&root, 0, Kind::Double)?
            .double()?
            .to_bits(),
        bits
    );
    execution.write_array(&root, 1, Kind::Double, &Value::Double(-0.0))?;
    assert_eq!(
        execution
            .read_array(&root, 1, Kind::Double)?
            .double()?
            .to_bits(),
        (-0.0_f64).to_bits()
    );
    assert!(matches!(
        execution.read_array(&root, 0, Kind::Long),
        Err(Error::ArrayFault("primitive array type mismatch"))
    ));
    assert!(matches!(
        execution.write_array(&root, 0, Kind::Double, &Value::Long(1)),
        Err(Error::ArrayFault("primitive array type mismatch"))
    ));
    assert!(matches!(
        execution.read_array(&root, -1, Kind::Double),
        Err(Error::ArrayFault("ArrayIndexOutOfBoundsException"))
    ));
    assert_eq!(
        execution
            .read_array(&root, 0, Kind::Double)?
            .double()?
            .to_bits(),
        bits
    );
    drop(root);
    execution.cleanup(&collector).await?;
    let object = execution.allocate_object(0).await?;
    assert!(matches!(
        execution.array_length(&object),
        Err(Error::ArrayFault("array type mismatch"))
    ));
    drop(object);
    execution.cleanup(&collector).await?;
    for kind in [
        Kind::Boolean,
        Kind::Byte,
        Kind::Char,
        Kind::Short,
        Kind::Int,
        Kind::Long,
        Kind::Float,
        Kind::Double,
    ] {
        let mut future = Box::pin(execution.allocate_array(kind, 64));
        let mut context = Context::from_waker(Waker::noop());
        assert!(future.as_mut().poll(&mut context).is_pending());
        assert!(collector.managed_bytes() > 0);
        drop(future);
        execution.cleanup(&collector).await?;
        assert_eq!(collector.managed_bytes(), 0);
    }
    drop(execution);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    Ok(())
}
