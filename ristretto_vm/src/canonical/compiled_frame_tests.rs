//! Actual canonical owners across emitted native entry, backedges and result handoff.
use super::super::{
    Budget, Configuration, GarbageCollector, Helper, Mutator, Payload, Report, Runtime, complete,
};
use super::*;
use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, Method, MethodAccessFlags};
use ristretto_jit::{Compiler, ExecutionStep, Function};
use std::num::NonZeroU32;

fn compile(operand: bool) -> Result<Function> {
    let code = if operand {
        vec![
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Ifle(9),
            Instruction::Iinc(1, -1),
            Instruction::Goto(-7),
            Instruction::Areturn,
        ]
    } else {
        vec![
            Instruction::Aload_0,
            Instruction::Astore_2,
            Instruction::Iload_1,
            Instruction::Ifle(9),
            Instruction::Iinc(1, -1),
            Instruction::Goto(-7),
            Instruction::Aload_2,
            Instruction::Areturn,
        ]
    };
    let mut pool = ConstantPool::new();
    let this_class = pool.add_class("CanonicalFrameOwner")?;
    let super_class = pool.add_class("java/lang/Object")?;
    let mut method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: pool.add_utf8("run")?,
        descriptor_index: pool.add_utf8("(Ljava/lang/Object;I)Ljava/lang/Object;")?,
        attributes: vec![],
    };
    method.attributes.push(Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: code.max_stack(&pool)?,
        max_locals: code.max_locals(&pool, &method)?,
        code,
        exception_table: vec![],
        attributes: vec![],
    });
    let class = ClassFile {
        version: ristretto_classfile::JAVA_5,
        constant_pool: pool,
        this_class,
        super_class,
        access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
        methods: vec![method],
        ..Default::default()
    };
    class.verify()?;
    // Only the typed canonical publication callback is linked. No legacy heap ABI.
    Ok(Compiler::new()?.compile_requests(
        &class,
        class
            .methods
            .first()
            .ok_or(Error::Boundary("test method"))?,
        &[("jit_publish_frame_roots", publish_roots as *const u8)],
    )?)
}

fn configuration() -> Configuration {
    Configuration {
        objects: 1,
        roots: 4,
        mutators: 1,
        helpers: 1,
        edges_per_object: 1,
    }
}

fn collect_cycles(worker: &mut Mutator<'_>, helper: &mut Helper<'_>) -> Result<()> {
    let target = worker.primary_cycles()?.upper + 2;
    for _ in 0..100_000 {
        helper.poll()?;
        if worker.primary_cycles()?.lower >= target {
            return Ok(());
        }
    }
    Err(Error::Limit("test collection cycles"))
}

fn reclaim(helper: &mut Helper<'_>, collector: &GarbageCollector) -> Result<()> {
    for _ in 0..100_000 {
        helper.poll()?;
        if collector.managed_bytes() == 0 {
            return Ok(());
        }
    }
    Err(Error::Limit("test cleanup"))
}

#[tokio::test]
async fn native_local_and_operand_owners_survive_collection_and_return_handoff() -> Result<()> {
    for operand in [false, true] {
        let function = compile(operand)?;
        let collector = GarbageCollector::new();
        let config = configuration();
        let budget = Budget::new(config.charged_bytes()?);
        let runtime = Runtime::new(&collector, config, &budget)?;
        let mut worker = runtime.mutator()?;
        let mut helper = runtime.helper()?;
        let mut report = Report::default();
        let root = {
            let mut request = worker.allocate(Payload::object(0))?;
            complete(&mut helper, &mut report, 100_000, || request.poll()).await?
        };
        let table = Rc::new(RefCell::new(Table::new(1, 1, 4)?));
        let context = Context::new(table.clone())?;
        let argument = Value::Reference(Some(root));
        let token = context.export(&argument, 100_000).await?;
        let JitValue::Ptr(raw) = token else {
            return Err(Error::Boundary("test token"));
        };
        let mut activation = function.activation(&[token, JitValue::I32(1)])?;
        drop(argument); // the native frame snapshot is now the sole root owner
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Yielded
        );
        collect_cycles(&mut worker, &mut helper)?;
        assert!(collector.managed_bytes() > 0);
        assert_eq!(table.borrow().live_objects(), 1);
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Returned(Some(JitValue::Ptr(raw)))
        );
        assert!(context.take_error().is_none());
        let returned = context.reference(raw)?; // acquire owner before retiring the native frame
        drop((activation, context));
        assert_eq!(table.borrow().live_objects(), 0);
        assert!(table.borrow().resolve(raw).is_err());
        collect_cycles(&mut worker, &mut helper)?;
        assert!(collector.managed_bytes() > 0);
        assert!(matches!(returned, Value::Reference(Some(_))));
        drop(returned);
        reclaim(&mut helper, &collector)?;
        drop((table, worker, helper, runtime));
        assert_eq!(budget.used(), 0);
    }
    Ok(())
}

#[tokio::test]
async fn native_cancellation_drops_owners_while_table_is_borrowed() -> Result<()> {
    let function = compile(false)?;
    let collector = GarbageCollector::new();
    let config = configuration();
    let budget = Budget::new(config.charged_bytes()?);
    let runtime = Runtime::new(&collector, config, &budget)?;
    let mut worker = runtime.mutator()?;
    let mut helper = runtime.helper()?;
    let mut report = Report::default();
    let root = {
        let mut request = worker.allocate(Payload::object(0))?;
        complete(&mut helper, &mut report, 100_000, || request.poll()).await?
    };
    let table = Rc::new(RefCell::new(Table::new(1, 1, 4)?));
    let context = Context::new(table.clone())?;
    let argument = Value::Reference(Some(root));
    let token = context.export(&argument, 100_000).await?;
    let mut activation = function.activation(&[token, JitValue::I32(10)])?;
    drop(argument);
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Yielded
    );
    collect_cycles(&mut worker, &mut helper)?;
    let guard = table.borrow_mut();
    drop((activation, context));
    assert_eq!(guard.live_objects(), 0);
    drop(guard);
    reclaim(&mut helper, &collector)?;
    drop((table, worker, helper, runtime));
    assert_eq!(budget.used(), 0);
    Ok(())
}

#[test]
fn native_foreign_token_fails_publication_before_return() -> Result<()> {
    let function = compile(false)?;
    let table = Rc::new(RefCell::new(Table::new(1, 1, 4)?));
    let context = Context::new(table)?;
    let mut activation = function.activation(&[JitValue::Ptr(i64::MAX), JitValue::I32(0)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(matches!(
        context.take_error(),
        Some(Error::Boundary("stale or foreign reference token"))
    ));
    assert!(context.failed.get());
    Ok(())
}
