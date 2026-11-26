//! Protocol checks with opaque tokens. Collector ownership is tested by VM adapters.
#![expect(
    clippy::cast_ptr_alignment,
    reason = "the opaque pointer originates from a live aligned test Context"
)]
use super::*;
use crate::{ExecutionStep, Value};
use std::num::NonZeroU32;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

#[path = "primitive_array_request_tests.rs"]
mod primitive_arrays;

#[path = "reference_array_request_tests.rs"]
mod reference_arrays;

#[path = "checkcast_request_tests.rs"]
mod checkcasts;

#[path = "static_request_tests.rs"]
mod statics;

#[path = "numeric_conversion_request_tests.rs"]
mod numeric_conversions;

#[path = "narrow_return_request_tests.rs"]
mod narrow_returns;

struct Context {
    roots: Mutex<Vec<i64>>,
    failed: AtomicBool,
    rejected_token: AtomicI64,
}

impl Context {
    fn new() -> Self {
        Self {
            roots: Mutex::new(Vec::with_capacity(64)),
            failed: AtomicBool::new(false),
            rejected_token: AtomicI64::new(0),
        }
    }
    fn pointer(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
}

extern "C" fn publish(pointer: *const u8, handles: *const i64, count: i32) -> i32 {
    // SAFETY: tests pass a live, aligned Context for this native slice; generated
    // publication supplies exactly count initialized cells during this callback.
    let context = unsafe { &*pointer.cast::<Context>() };
    if context.failed.load(Ordering::Relaxed) || !(0..=64).contains(&count) {
        context.failed.store(true, Ordering::Relaxed);
        return 0;
    }
    let Ok(count) = usize::try_from(count) else {
        return 0;
    };
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    let rejected = context.rejected_token.load(Ordering::Relaxed);
    if rejected != 0 && handles.contains(&rejected) {
        context.failed.store(true, Ordering::Relaxed);
        return 0;
    }
    let mut roots = context
        .roots
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    roots.clear();
    roots.extend_from_slice(handles);
    1
}

extern "C" fn pending(pointer: *const u8) -> i64 {
    // SAFETY: same Context lifetime/alignment contract as publish.
    let context = unsafe { &*pointer.cast::<Context>() };
    i64::from(context.failed.load(Ordering::Relaxed))
}

fn compile(class: &ClassFile, name: &str) -> Result<Function> {
    let method = class
        .methods
        .iter()
        .find(|method| {
            class
                .constant_pool
                .try_get_utf8(method.name_index)
                .is_ok_and(|value| value == name)
        })
        .ok_or_else(|| InternalError("missing fixture method".to_owned()))?;
    // Deliberately supply no legacy helpers. Even an unreachable synchronous
    // fallback would fail symbol resolution when the native module is finalized.
    let function = Compiler::new()?.compile_requests(
        class,
        method,
        &[
            ("jit_publish_frame_roots", publish as *const u8),
            ("jit_pending_exception", pending as *const u8),
        ],
    )?;
    assert!(function.requests_only());
    assert!(function.supports_cooperation());
    Ok(function)
}

fn graph() -> Result<ClassFile<'static>> {
    Ok(ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.class"
    ))?)
}

fn node() -> Result<ClassFile<'static>> {
    Ok(ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload$Node.class"
    ))?)
}

fn script(
    function: &Function,
    arguments: &[Value],
    requests: &[(&str, &[i64], i64)],
    expected: Option<Value>,
) -> Result<()> {
    let context = Context::new();
    assert!(function.execute(arguments, context.pointer()).is_err());
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 0);
    let mut activation = function.activation(arguments)?;
    let mut budget = NonZeroU32::new(64).ok_or_else(|| InternalError("zero budget".to_owned()))?;
    for (kind, expected_arguments, result) in requests {
        let step = activation.resume_with_context(budget, context.pointer())?;
        assert!(matches!(
            (step, *kind),
            (ExecutionStep::Allocation(_), "allocation")
                | (ExecutionStep::Call(_), "call")
                | (ExecutionStep::Field(_), "field")
        ));
        assert_eq!(activation.call_arguments()?, *expected_arguments);
        budget = activation
            .remaining_budget()?
            .ok_or_else(|| InternalError("short script exhausted fuel".to_owned()))?;
        activation.complete_call(*result)?;
    }
    assert_eq!(
        activation.resume_with_context(budget, context.pointer())?,
        ExecutionStep::Returned(expected)
    );
    assert!(!context.failed.load(Ordering::Relaxed));
    assert!(
        activation
            .resume_with_context(budget, context.pointer())
            .is_err()
    );
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.execution_count(), 1);
    Ok(())
}

#[test]
fn graph_requests_link_and_execute_without_any_legacy_helper_symbol() -> Result<()> {
    let class = graph()?;
    let function = compile(&class, "run")?;
    script(
        &function,
        &[Value::I32(1)],
        &[
            ("allocation", &[], 1),
            ("call", &[1, 0], 0),
            ("allocation", &[], 2),
            ("call", &[2, 1], 0),
            ("field", &[1, 2], 0),
            ("field", &[2, 1], 0),
            ("call", &[1, 2], 1),
        ],
        Some(Value::I32(1)),
    )?;
    #[cfg(feature = "execution-profile")]
    assert_eq!(function.backedge_count(), 1);
    Ok(())
}

#[test]
fn constructor_and_reference_comparisons_use_only_owning_requests() -> Result<()> {
    let class = node()?;
    script(
        &compile(&class, "<init>")?,
        &[Value::Ptr(1), Value::I32(7)],
        &[("call", &[1], 0), ("field", &[1, 7], 0)],
        None,
    )?;
    let class = graph()?;
    script(
        &compile(&class, "checksum")?,
        &[Value::Ptr(1), Value::Ptr(2)],
        &[
            ("field", &[1], 2),
            ("field", &[1, 0], 0),
            ("field", &[2], 1),
            ("field", &[1, 2], 0),
            ("field", &[1], 7),
            ("field", &[1], 2),
            ("field", &[2], 8),
        ],
        Some(Value::I32(15)),
    )?;
    Ok(())
}

#[test]
fn publication_failure_returns_without_dispatching_a_runtime_request() -> Result<()> {
    let function = compile(&graph()?, "run")?;
    let context = Context::new();
    context.failed.store(true, Ordering::Relaxed);
    let mut activation = function.activation(&[Value::I32(1)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(context.failed.load(Ordering::Relaxed));
    Ok(())
}

#[test]
fn pending_failure_after_request_cannot_execute_later_bytecodes() -> Result<()> {
    let function = compile(&graph()?, "run")?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::I32(1)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Allocation(_)
    ));
    context.failed.store(true, Ordering::Relaxed);
    activation.complete_call(0)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    Ok(())
}

#[test]
fn straight_line_primitive_methods_also_require_explicit_activation() -> Result<()> {
    let mut class = graph()?;
    let pool = &class.constant_pool;
    let method = class
        .methods
        .iter_mut()
        .find(|method| {
            pool.try_get_utf8(method.name_index)
                .is_ok_and(|value| value == "run")
        })
        .expect("run method");
    for attribute in &mut method.attributes {
        if let Attribute::Code {
            code, attributes, ..
        } = attribute
        {
            *code = vec![Instruction::Iload_0, Instruction::Ireturn];
            attributes.clear();
        }
    }
    let function = compile(&class, "run")?;
    assert!(!function.cooperation_requires_context());
    assert!(
        function
            .execute(&[Value::I32(47)], std::ptr::null())
            .is_err()
    );
    let mut activation = function.activation(&[Value::I32(47)])?;
    assert_eq!(
        activation.resume(NonZeroU32::MIN)?,
        ExecutionStep::Returned(Some(Value::I32(47)))
    );
    Ok(())
}

#[test]
fn request_result_identity_is_published_before_any_later_use() -> Result<()> {
    let function = compile(&graph()?, "checksum")?;
    let context = Context::new();
    context.rejected_token.store(3, Ordering::Relaxed);
    let mut activation = function.activation(&[Value::Ptr(1), Value::Ptr(2)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(_)
    ));
    let prior = context
        .roots
        .lock()
        .map_err(|error| InternalError(error.to_string()))?
        .clone();
    activation.complete_call(3)?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(context.failed.load(Ordering::Relaxed));
    assert_eq!(
        *context
            .roots
            .lock()
            .map_err(|error| InternalError(error.to_string()))?,
        prior
    );
    Ok(())
}

#[test]
fn request_context_can_move_and_null_rejection_preserves_activation() -> Result<()> {
    let function = compile(&graph()?, "checksum")?;
    let context = Context::new();
    let original_pointer = context.pointer();
    let mut activation = function.activation(&[Value::Ptr(1), Value::Ptr(2)])?;
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, original_pointer)?,
        ExecutionStep::Field(_)
    ));
    activation.complete_call(2)?;
    let context = Box::new(context);
    assert_ne!(original_pointer, context.pointer());
    assert!(activation.resume(NonZeroU32::MIN).is_err());
    assert!(matches!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Field(_)
    ));
    assert_eq!(activation.call_arguments()?, &[1, 0]);
    // Cancellation leaves no generated frame using the moved context.
    drop(activation);
    drop(context);
    Ok(())
}

#[test]
fn request_compiler_rejects_synchronous_only_capabilities_before_linking() -> Result<()> {
    let mut class = graph()?;
    let pool = &class.constant_pool;
    let method = class
        .methods
        .iter_mut()
        .find(|method| {
            pool.try_get_utf8(method.name_index)
                .is_ok_and(|value| value == "run")
        })
        .expect("run method");
    for attribute in &mut method.attributes {
        if let Attribute::Code { code, .. } = attribute {
            *code = vec![
                Instruction::Iconst_1,
                Instruction::Iconst_0,
                Instruction::Idiv,
                Instruction::Ireturn,
            ];
        }
    }
    let method = class
        .methods
        .iter()
        .find(|method| {
            pool.try_get_utf8(method.name_index)
                .is_ok_and(|value| value == "run")
        })
        .expect("run method");
    assert!(
        matches!(Compiler::new()?.compile_requests(&class, method, &[]), Err(crate::Error::UnsupportedMethod(reason)) if reason == "experimental_unqualified_instruction")
    );
    Ok(())
}
