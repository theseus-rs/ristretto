//! Native activations restricted to the admitted canonical Java program.
use super::compiled_frame::{Context, publish_roots};
use super::frame_table::Table;
use super::{
    Budget, CallTarget, Configuration, Error, Execution, GarbageCollector, Limits, Method,
    MethodAccessFlags, Program, Report, Runtime, Value, exact_symbol, help,
};
use ristretto_classfile::{BaseType, FieldType};
use ristretto_jit::{
    Activation, AllocationKind, ArrayAccessKind, CallKind, Compiler, ExecutionStep, FieldAccess,
    Function,
};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;

use super::{Kind, allocation_kind};

/// Prepared request-only code. It cannot receive an ordinary VM runtime context.
#[derive(Debug)]
pub struct CompiledProgram<'p> {
    program: &'p Program,
    functions: Vec<HashMap<String, HashMap<String, Function>>>,
    roots_per_frame: usize,
    running: Cell<bool>,
}

/// Native work policy; native slices do not pretend to count interpreted instructions.
#[derive(Clone, Copy, Debug)]
pub struct CompiledLimits {
    pub request_polls: usize,
    pub cleanup_polls: usize,
    pub frames: usize,
    pub native_slices: usize,
    pub array_bytes: usize,
}
impl Default for CompiledLimits {
    fn default() -> Self {
        let limits = Limits::default();
        Self {
            request_polls: limits.request_polls,
            cleanup_polls: limits.cleanup_polls,
            frames: limits.frames,
            native_slices: 100_000,
            array_bytes: limits.array_bytes,
        }
    }
}
impl CompiledLimits {
    fn execution_limits(self) -> Limits {
        Limits {
            instructions: 0,
            request_polls: self.request_polls,
            cleanup_polls: self.cleanup_polls,
            frames: self.frames,
            array_bytes: self.array_bytes,
        }
    }
}

impl Program {
    /// Prepare the intersection of verified canonical and request-only compiler capabilities.
    /// # Errors
    /// Rejects unsupported codegen, invalid metadata or excessive frame storage.
    pub fn compile(&self) -> Result<CompiledProgram<'_>, Error> {
        if self.classes.iter().any(|layout| layout.statics.needed()) {
            return Err(Error::Unsupported(
                "native class initialization is not connected".to_owned(),
            ));
        }
        let compiler = Compiler::new()?;
        let mut functions = Vec::with_capacity(self.classes.len());
        let mut roots_per_frame = 1;
        for layout in &self.classes {
            let mut methods = HashMap::<String, HashMap<String, Function>>::new();
            let file = layout.class.class_file();
            for definition in &file.methods {
                let name = exact_symbol(file.constant_pool.try_get_utf8(definition.name_index)?)
                    .map_err(|e| Error::Invalid(e.to_string()))?
                    .into_owned();
                let descriptor = exact_symbol(
                    file.constant_pool
                        .try_get_utf8(definition.descriptor_index)?,
                )
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned();
                let method = layout.class.try_get_method(&name, &descriptor)?;
                let capacity = method
                    .max_locals()
                    .checked_add(method.max_stack())
                    .and_then(|v| v.checked_add(method.parameters().len()))
                    .and_then(|v| v.checked_add(2))
                    .ok_or(Error::Limit("compiled frame layout"))?;
                roots_per_frame = roots_per_frame.max(capacity);
                let function = compiler.compile_requests(
                    file,
                    definition,
                    &[("jit_publish_frame_roots", publish_roots as *const u8)],
                )?;
                if !function.requests_only() {
                    return Err(Error::Boundary("non-request canonical code"));
                }
                if methods
                    .entry(name)
                    .or_default()
                    .insert(descriptor, function)
                    .is_some()
                {
                    return Err(Error::Boundary("duplicate compiled method"));
                }
            }
            functions.push(methods);
        }
        Ok(CompiledProgram {
            program: self,
            functions,
            roots_per_frame,
            running: Cell::new(false),
        })
    }
}

struct RunLease<'a>(&'a Cell<bool>);
impl Drop for RunLease<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}
impl CompiledProgram<'_> {
    /// Run one static (I)I entry on its own canonical heap, including complete cleanup.
    /// # Errors
    /// Reports capability, runtime, ownership and explicit work-limit failures.
    pub async fn run(
        &self,
        class: &str,
        method: &str,
        argument: i32,
        configuration: Configuration,
        limits: CompiledLimits,
    ) -> Result<Report, Error> {
        if self.running.replace(true) {
            return Err(Error::Boundary("compiled program already running"));
        }
        let _lease = RunLease(&self.running);
        let class = self.program.class_index(class)?;
        let method = self
            .program
            .layout(class)?
            .class
            .try_get_method(method, "(I)I")?;
        if !method.access_flags().contains(MethodAccessFlags::STATIC) {
            return Err(Error::Unsupported("entry must be static (I)I".to_owned()));
        }
        if configuration.mutators != 1
            || configuration.helpers != 1
            || self
                .program
                .classes
                .iter()
                .any(|layout| layout.references > configuration.edges_per_object)
        {
            return Err(Error::Unsupported(
                "single caller/helper or insufficient edge positions".to_owned(),
            ));
        }
        let budget = Budget::new(configuration.charged_bytes()?);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let execution = Execution {
            program: self.program,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            runtime: &runtime,
            limits: limits.execution_limits(),
            report: Report::default(),
            statics: crate::canonical::statics::States::default(),
        };
        let table = Rc::new(RefCell::new(Table::new(
            configuration.objects,
            limits.frames,
            self.roots_per_frame,
        )?));
        let mut execution = CompiledExecution {
            compiled: self,
            execution,
            table,
            slice_limit: limits.native_slices,
        };
        let result = execution
            .call(class, method, vec![Value::Int(argument)], 0)
            .await;
        execution.execution.cleanup(&collector).await?;
        let mut report = execution.execution.report.clone();
        drop(execution);
        drop(runtime);
        report.catalog_bytes_after_drop = budget.used();
        if report.catalog_bytes_after_drop != 0 {
            return Err(Error::Boundary("catalog credit retained after drop"));
        }
        report.result = result?
            .ok_or(Error::Boundary("missing compiled result"))?
            .int()?;
        Ok(report)
    }
}

struct CompiledExecution<'c, 'p, 'r, 'gc> {
    compiled: &'c CompiledProgram<'p>,
    execution: Execution<'p, 'r, 'gc>,
    table: Rc<RefCell<Table>>,
    slice_limit: usize,
}
struct Frame {
    class: usize,
    method: Arc<Method>,
    context: Box<Context>,
    activation: Activation,
    started: bool,
}

impl CompiledExecution<'_, '_, '_, '_> {
    async fn frame(
        &mut self,
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
        depth: usize,
    ) -> Result<Frame, Error> {
        if depth >= self.execution.limits.frames {
            return Err(Error::Limit("frame"));
        }
        let receiver = usize::from(!method.access_flags().contains(MethodAccessFlags::STATIC));
        if arguments.len() != receiver + method.parameters().len() {
            return Err(Error::Boundary("compiled argument count"));
        }
        if receiver != 0 {
            let root = arguments
                .first()
                .cloned()
                .ok_or(Error::Boundary("compiled receiver"))?
                .object()?;
            self.execution.check_class(&root, class)?;
        }
        for (value, field) in arguments.iter().skip(receiver).zip(method.parameters()) {
            self.execution.check_type(value, field)?;
        }
        let context = Context::new(self.table.clone())?;
        let mut exported = Vec::with_capacity(arguments.len());
        for value in &arguments {
            exported.push(
                context
                    .export(value, self.execution.limits.request_polls)
                    .await?,
            );
        }
        let function = self
            .compiled
            .functions
            .get(class)
            .and_then(|methods| methods.get(method.name()))
            .and_then(|overloads| overloads.get(method.descriptor()))
            .ok_or(Error::Boundary("missing compiled method"))?;
        let activation = function.activation(&exported)?;
        drop(arguments);
        self.execution.report.calls += 1;
        Ok(Frame {
            class,
            method,
            context,
            activation,
            started: false,
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "explicit bounded typed activation stack"
    )]
    async fn call(
        &mut self,
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
        depth: usize,
    ) -> Result<Option<Value>, Error> {
        if depth != 0 {
            return Err(Error::Boundary("non-root driver invocation"));
        }
        let mut frames = Vec::new();
        frames
            .try_reserve_exact(self.execution.limits.frames)
            .map_err(|_| Error::Limit("compiled frames allocation"))?;
        frames.push(self.frame(class, method, arguments, 0).await?);
        loop {
            if self.execution.report.native_slices >= self.slice_limit {
                return Err(Error::Limit("native slice"));
            }
            let frame = frames
                .last_mut()
                .ok_or(Error::Boundary("missing compiled frame"))?;
            help(&mut self.execution.helper, &mut self.execution.report)?;
            let step = frame
                .activation
                .resume_with_context(NonZeroU32::MIN, frame.context.pointer())?;
            self.execution.report.native_slices += 1;
            if frame.started {
                self.execution.report.compiled_resumes += 1;
            } else {
                self.execution.report.compiled_entries += 1;
                frame.started = true;
            }
            if let Some(error) = frame.context.take_error() {
                return Err(error);
            }
            match step {
                ExecutionStep::Returned(result) => {
                    let value = match result {
                        None => None,
                        Some(ristretto_jit::Value::I32(v)) => Some(Value::Int(v)),
                        Some(ristretto_jit::Value::I64(v)) => Some(Value::Long(v)),
                        Some(ristretto_jit::Value::F32(v)) => Some(Value::Float(v)),
                        Some(ristretto_jit::Value::F64(v)) => Some(Value::Double(v)),
                        Some(ristretto_jit::Value::Ptr(v)) => Some(frame.context.reference(v)?),
                    };
                    match (value.as_ref(), frame.method.return_type()) {
                        (None, None) => {}
                        (Some(value), Some(field)) => self.execution.check_type(value, field)?,
                        _ => return Err(Error::Boundary("compiled return type")),
                    }
                    let value = match (value, frame.method.return_type()) {
                        (Some(Value::Int(v)), Some(field)) => Some(Value::from(
                            super::scalar::normalize_return(super::ScalarValue::Int(v), field),
                        )),
                        (value, _) => value,
                    };
                    // Resolve the returned token before retiring the callee snapshot.
                    frames.pop();
                    let Some(parent) = frames.last_mut() else {
                        return Ok(value);
                    };
                    let bits = result_bits(
                        &parent.context,
                        value.as_ref(),
                        self.execution.limits.request_polls,
                    )
                    .await?;
                    parent.activation.complete_call(bits)?;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::Yielded => {
                    self.execution.report.compiled_backedges += 1;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::Allocation(request) => {
                    let arguments = frame.activation.call_arguments()?;
                    let root = match request.kind {
                        AllocationKind::Object(index) if arguments.is_empty() => {
                            let class = self.compiled.program.class_constant(frame.class, index)?;
                            self.execution.allocate_object(class).await?
                        }
                        AllocationKind::PrimitiveArray(kind) if arguments.len() == 1 => {
                            let length = int_bits(
                                *arguments
                                    .first()
                                    .ok_or(Error::Boundary("array length argument"))?,
                            );
                            self.execution
                                .allocate_array(allocation_kind(&kind), length)
                                .await?
                        }
                        AllocationKind::ReferenceArray(index) if arguments.len() == 1 => {
                            let component = self
                                .compiled
                                .program
                                .reference_component(frame.class, index)?;
                            let length = int_bits(
                                *arguments
                                    .first()
                                    .ok_or(Error::Boundary("reference array length argument"))?,
                            );
                            self.execution
                                .allocate_reference_array(component, length)
                                .await?
                        }
                        _ => return Err(Error::Boundary("compiled allocation arity")),
                    };
                    let value = Value::Reference(Some(root));
                    let bits = result_bits(
                        &frame.context,
                        Some(&value),
                        self.execution.limits.request_polls,
                    )
                    .await?;
                    frame.activation.complete_call(bits)?;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::ArrayAccess(request) => {
                    let arguments = frame.activation.call_arguments()?;
                    if arguments.len() != request.kind.argument_count() {
                        return Err(Error::Boundary("compiled array arity"));
                    }
                    let root = frame
                        .context
                        .reference(
                            *arguments
                                .first()
                                .ok_or(Error::Boundary("array receiver argument"))?,
                        )?
                        .object()?;
                    let value = if request.kind == ArrayAccessKind::Length {
                        Some(self.execution.array_length(&root)?)
                    } else if request.kind == ArrayAccessKind::ReferenceLoad {
                        let index = int_bits(
                            *arguments
                                .get(1)
                                .ok_or(Error::Boundary("reference array index argument"))?,
                        );
                        Some(Value::Reference(
                            self.execution.read_reference_array(&root, index).await?,
                        ))
                    } else {
                        let kind = array_kind(request.kind)?;
                        let index = int_bits(
                            *arguments
                                .get(1)
                                .ok_or(Error::Boundary("array index argument"))?,
                        );
                        if arguments.len() == 3 {
                            let bits = *arguments
                                .get(2)
                                .ok_or(Error::Boundary("array store argument"))?;
                            let value = match kind {
                                Kind::Long => Value::Long(bits),
                                Kind::Float => Value::Float(f32::from_bits(u32::from_ne_bytes(
                                    int_bits(bits).to_ne_bytes(),
                                ))),
                                Kind::Double => Value::Double(f64::from_bits(u64::from_ne_bytes(
                                    bits.to_ne_bytes(),
                                ))),
                                _ => Value::Int(int_bits(bits)),
                            };
                            self.execution.write_array(&root, index, kind, &value)?;
                            None
                        } else {
                            Some(self.execution.read_array(&root, index, kind)?)
                        }
                    };
                    let bits = result_bits(
                        &frame.context,
                        value.as_ref(),
                        self.execution.limits.request_polls,
                    )
                    .await?;
                    frame.activation.complete_call(bits)?;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::ArrayStore(_) => {
                    let arguments = frame.activation.call_arguments()?;
                    if arguments.len() != 3 {
                        return Err(Error::Boundary("reference array store arity"));
                    }
                    let root = frame
                        .context
                        .reference(
                            *arguments
                                .first()
                                .ok_or(Error::Boundary("reference array receiver argument"))?,
                        )?
                        .object()?;
                    let index = int_bits(
                        *arguments
                            .get(1)
                            .ok_or(Error::Boundary("reference array index argument"))?,
                    );
                    let Value::Reference(value) = frame.context.reference(
                        *arguments
                            .get(2)
                            .ok_or(Error::Boundary("reference array value argument"))?,
                    )?
                    else {
                        return Err(Error::Boundary("reference array store value"));
                    };
                    self.execution
                        .write_reference_array(&root, index, value.as_ref())
                        .await?;
                    frame.activation.complete_call(0)?;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::Field(request) => {
                    let arguments = frame.activation.call_arguments()?;
                    let expected = if request.kind == FieldAccess::GetField {
                        1
                    } else {
                        2
                    };
                    if arguments.len() != expected {
                        return Err(Error::Boundary("compiled field arity"));
                    }
                    let root = frame
                        .context
                        .reference(
                            *arguments
                                .first()
                                .ok_or(Error::Boundary("compiled field receiver"))?,
                        )?
                        .object()?;
                    let value = match request.kind {
                        FieldAccess::GetField => Some(
                            self.execution
                                .read_field(frame.class, request.index, &root)
                                .await?,
                        ),
                        FieldAccess::PutField => {
                            let (_, field) =
                                self.compiled.program.field(frame.class, request.index)?;
                            let bits = *arguments
                                .get(1)
                                .ok_or(Error::Boundary("compiled field value"))?;
                            let value = match field {
                                super::Field::Int(_) => Value::Int(int_bits(bits)),
                                super::Field::Reference { .. } => frame.context.reference(bits)?,
                            };
                            self.execution
                                .write_field(frame.class, request.index, &root, value)
                                .await?;
                            None
                        }
                    };
                    let bits = result_bits(
                        &frame.context,
                        value.as_ref(),
                        self.execution.limits.request_polls,
                    )
                    .await?;
                    frame.activation.complete_call(bits)?;
                    tokio::task::yield_now().await;
                }
                ExecutionStep::Call(request) => {
                    let bits = frame.activation.call_arguments()?;
                    match self
                        .compiled
                        .program
                        .call_target(frame.class, request.index)?
                    {
                        CallTarget::ObjectConstructor => {
                            if request.kind != CallKind::Special || bits.len() != 1 {
                                return Err(Error::Boundary("compiled Object constructor"));
                            }
                            frame
                                .context
                                .reference(
                                    *bits.first().ok_or(Error::Boundary("Object receiver"))?,
                                )?
                                .object()?;
                            frame.activation.complete_call(0)?;
                        }
                        CallTarget::Method {
                            class: owner,
                            method: target,
                        } => {
                            let receiver = usize::from(
                                !target.access_flags().contains(MethodAccessFlags::STATIC),
                            );
                            if (receiver == 0 && request.kind != CallKind::Static)
                                || (receiver == 1
                                    && (request.kind != CallKind::Special
                                        || target.name() != "<init>"))
                                || bits.len() != receiver + target.parameters().len()
                            {
                                return Err(Error::Boundary("compiled call dispatch"));
                            }
                            let mut values = Vec::with_capacity(bits.len());
                            if receiver != 0 {
                                values.push(frame.context.reference(
                                    *bits.first().ok_or(Error::Boundary("call receiver"))?,
                                )?);
                            }
                            for (bits, field) in bits.iter().skip(receiver).zip(target.parameters())
                            {
                                values.push(decode(&frame.context, *bits, field)?);
                            }
                            let child = self.frame(owner, target, values, frames.len()).await?;
                            frames.push(child);
                        }
                    }
                    // A request consumes the single-unit slice. Scheduling and
                    // cancellation never wait for a recursive Rust future chain.
                    tokio::task::yield_now().await;
                }
            }
        }
    }
}

fn array_kind(kind: ArrayAccessKind) -> Result<Kind, Error> {
    Ok(match kind {
        ArrayAccessKind::IntLoad | ArrayAccessKind::IntStore => Kind::Int,
        ArrayAccessKind::LongLoad | ArrayAccessKind::LongStore => Kind::Long,
        ArrayAccessKind::FloatLoad | ArrayAccessKind::FloatStore => Kind::Float,
        ArrayAccessKind::DoubleLoad | ArrayAccessKind::DoubleStore => Kind::Double,
        ArrayAccessKind::ByteLoad | ArrayAccessKind::ByteStore => Kind::Byte,
        ArrayAccessKind::CharLoad | ArrayAccessKind::CharStore => Kind::Char,
        ArrayAccessKind::ShortLoad | ArrayAccessKind::ShortStore => Kind::Short,
        ArrayAccessKind::Length | ArrayAccessKind::ReferenceLoad => {
            return Err(Error::Boundary("unsupported array request kind"));
        }
    })
}

fn int_bits(bits: i64) -> i32 {
    let [a, b, c, d, _, _, _, _] = bits.to_le_bytes();
    i32::from_le_bytes([a, b, c, d])
}
fn decode(context: &Context, bits: i64, field: &FieldType) -> Result<Value, Error> {
    Ok(match field {
        FieldType::Object(_) | FieldType::Array(_) => context.reference(bits)?,
        FieldType::Base(BaseType::Long) => Value::Long(bits),
        FieldType::Base(BaseType::Double) => {
            Value::Double(f64::from_bits(u64::from_ne_bytes(bits.to_ne_bytes())))
        }
        FieldType::Base(BaseType::Float) => Value::Float(f32::from_bits(u32::from_ne_bytes(
            int_bits(bits).to_ne_bytes(),
        ))),
        FieldType::Base(_) => Value::Int(int_bits(bits)),
    })
}
async fn result_bits(
    context: &Context,
    value: Option<&Value>,
    attempts: usize,
) -> Result<i64, Error> {
    Ok(match value {
        None => 0,
        Some(value) => match context.export(value, attempts).await? {
            ristretto_jit::Value::I32(v) => i64::from(v),
            ristretto_jit::Value::I64(v) | ristretto_jit::Value::Ptr(v) => v,
            ristretto_jit::Value::F32(v) => i64::from(v.to_bits()),
            ristretto_jit::Value::F64(v) => i64::from_ne_bytes(v.to_bits().to_ne_bytes()),
        },
    })
}

#[cfg(test)]
#[path = "compiled_tests.rs"]
mod tests;
