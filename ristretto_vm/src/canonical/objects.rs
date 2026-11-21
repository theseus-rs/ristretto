//! Execution of verified closed object methods on a dedicated incremental heap.
use super::layout::{Field, ReferenceType, reference_type};
use super::reference_array::{self, Component, Layout as ReferenceArrayLayout};
mod cast;
mod reference_array_runtime;
use super::primitive_array::{Element, Kind, PrimitiveArray};
use super::statics::States;
use super::value::PrimitiveFrameValue;
use super::{ScalarValue, primitive_array, scalar};
use ristretto_classfile::attributes::ArrayType;

use super::{Error, Layout, Program, Result, exact_symbol};
use ristretto_classfile::{
    BaseType, FieldType, JavaStr, MethodAccessFlags, attributes::Instruction,
};
use ristretto_classloader::Method;
use ristretto_gc::canonical::{Budget, Configuration, Helper, Mutator, Root, Runtime, Work};
use ristretto_gc::{GarbageCollector, Trace};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::task::Poll;
/// Caller policy limits; these bound work counts, never wall-clock latency.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub instructions: usize,
    pub request_polls: usize,
    pub cleanup_polls: usize,
    pub frames: usize,
    /// Maximum requested backing bytes of one primitive array, before preparation.
    pub array_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            instructions: 1_000_000,
            request_polls: 100_000,
            cleanup_polls: 1_000_000,
            frames: 64,
            array_bytes: 8 * 1024 * 1024,
        }
    }
}

/// Work performed before the entry body, included in the full Report counters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InitializationWork {
    pub instructions: usize,
    pub allocations: usize,
    pub helper_polls: usize,
    pub reference_reads: usize,
    pub reference_writes: usize,
    pub calls: usize,
    pub class_initialization_waits: usize,
}
impl InitializationWork {
    pub(super) fn from_report(report: &Report) -> Self {
        Self {
            instructions: report.instructions,
            allocations: report.allocations,
            helper_polls: report.helper_polls,
            reference_reads: report.reference_reads,
            reference_writes: report.reference_writes,
            calls: report.calls,
            class_initialization_waits: report.class_initialization_waits,
        }
    }
}

/// Actual execution counters. This report contains no timing or pause claim.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    pub result: i32,
    /// Exact interpreted visits; unavailable in native execution (reported as zero).
    pub instructions: usize,
    pub instruction_count_available: bool,
    pub allocations: usize,
    pub internal_allocations: usize,
    pub static_storage_allocations: usize,
    pub initialized_classes: usize,
    pub class_initializer_calls: usize,
    pub class_initialization_waits: usize,
    pub entry_initialization: InitializationWork,
    pub reference_array_allocations: usize,
    pub reference_reads: usize,
    pub reference_writes: usize,
    pub calls: usize,
    pub helper_polls: usize,
    pub released_objects: usize,
    pub payload_bytes_after_cleanup: usize,
    pub catalog_bytes_after_drop: usize,
    pub compiled_entries: usize,
    pub compiled_resumes: usize,
    pub compiled_backedges: u64,
    pub native_slices: usize,
    pub array_allocations: usize,
    pub array_reads: usize,
    pub array_writes: usize,
}

pub(super) fn array_kind(component: &FieldType) -> Result<Kind> {
    Ok(match component {
        FieldType::Base(BaseType::Boolean) => Kind::Boolean,
        FieldType::Base(BaseType::Byte) => Kind::Byte,
        FieldType::Base(BaseType::Char) => Kind::Char,
        FieldType::Base(BaseType::Short) => Kind::Short,
        FieldType::Base(BaseType::Int) => Kind::Int,
        FieldType::Base(BaseType::Long) => Kind::Long,
        FieldType::Base(BaseType::Float) => Kind::Float,
        FieldType::Base(BaseType::Double) => Kind::Double,
        _ => {
            return Err(Error::Unsupported(
                "reference or multidimensional arrays".to_owned(),
            ));
        }
    })
}

fn allocation_kind(kind: &ArrayType) -> Kind {
    match kind {
        ArrayType::Boolean => Kind::Boolean,
        ArrayType::Byte => Kind::Byte,
        ArrayType::Char => Kind::Char,
        ArrayType::Short => Kind::Short,
        ArrayType::Int => Kind::Int,
        ArrayType::Long => Kind::Long,
        ArrayType::Float => Kind::Float,
        ArrayType::Double => Kind::Double,
    }
}

fn array_instruction(instruction: &Instruction) -> Option<(Kind, bool)> {
    use Instruction as I;
    Some(match instruction {
        I::Iaload => (Kind::Int, false),
        I::Iastore => (Kind::Int, true),
        I::Laload => (Kind::Long, false),
        I::Lastore => (Kind::Long, true),
        I::Faload => (Kind::Float, false),
        I::Fastore => (Kind::Float, true),
        I::Daload => (Kind::Double, false),
        I::Dastore => (Kind::Double, true),
        I::Baload => (Kind::Byte, false),
        I::Bastore => (Kind::Byte, true),
        I::Caload => (Kind::Char, false),
        I::Castore => (Kind::Char, true),
        I::Saload => (Kind::Short, false),
        I::Sastore => (Kind::Short, true),
        _ => return None,
    })
}

fn array_error(error: primitive_array::Error) -> Error {
    use primitive_array::Error as E;
    match error {
        E::NegativeLength => Error::ArrayFault("NegativeArraySizeException"),
        E::Index => Error::ArrayFault("ArrayIndexOutOfBoundsException"),
        E::Type => Error::ArrayFault("primitive array type mismatch"),
        E::ByteLimit => Error::Limit("array backing bytes"),
        E::SizeOverflow => Error::Limit("array size"),
        E::Allocation => Error::Heap(ristretto_gc::Error::OutOfMemory),
    }
}

impl Program {
    pub(super) fn layout(&self, class: usize) -> Result<&Layout> {
        self.classes
            .get(class)
            .ok_or_else(|| Error::Invalid("class index".into()))
    }
    pub(super) fn class_index(&self, name: &str) -> Result<usize> {
        self.names
            .get(JavaStr::cow_from_str(name).as_ref())
            .copied()
            .ok_or_else(|| Error::Unsupported(format!("class {name}")))
    }
    fn class_constant(&self, owner: usize, index: u16) -> Result<usize> {
        let name = self
            .layout(owner)?
            .class
            .constant_pool()
            .try_get_class(index)?;
        let target = self
            .names
            .get(name)
            .copied()
            .ok_or_else(|| Error::Unsupported(format!("class {name}")))?;
        self.admit_class_access(owner, target)?;
        Ok(target)
    }
    pub(super) fn member(
        &self,
        owner: usize,
        index: u16,
        field: bool,
    ) -> Result<(String, String, String)> {
        let pool = self.layout(owner)?.class.constant_pool();
        let (class, name_type) = if field {
            pool.try_get_field_ref(index)?
        } else {
            pool.try_get_method_ref(index)?
        };
        let (name, descriptor) = pool.try_get_name_and_type(*name_type)?;
        Ok((
            exact_symbol(pool.try_get_class(*class)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
            exact_symbol(pool.try_get_utf8(*name)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
            exact_symbol(pool.try_get_utf8(*descriptor)?)
                .map_err(|e| Error::Invalid(e.to_string()))?
                .into_owned(),
        ))
    }

    fn field(&self, owner: usize, index: u16) -> Result<(usize, Field)> {
        let (name, field, descriptor) = self.member(owner, index, true)?;
        let class = self.class_index(&name)?;
        let slot = self
            .layout(class)?
            .field(&field, &descriptor)
            .ok_or_else(|| Error::Unsupported(format!("field {name}.{field} {descriptor}")))?;
        Ok((class, slot))
    }

    fn reference_component(&self, owner: usize, index: u16) -> Result<Component> {
        if self
            .layout(owner)?
            .class
            .constant_pool()
            .try_get_class(index)?
            == "java/lang/Object"
        {
            Ok(Component::Object)
        } else {
            self.class_constant(owner, index).map(Component::Class)
        }
    }
    fn admit_type(&self, field: &FieldType) -> Result<()> {
        match field {
            FieldType::Base(_) => Ok(()),
            _ => reference_type(field, &self.names).map(|_| ()),
        }
    }
    #[expect(
        clippy::too_many_lines,
        reason = "single explicit opcode admission table"
    )]
    pub(super) fn admit_object_method(&self, class: usize, method: &Method) -> Result<()> {
        use Instruction as I;
        if method.name() == "finalize"
            || method.access_flags().intersects(
                MethodAccessFlags::NATIVE
                    | MethodAccessFlags::ABSTRACT
                    | MethodAccessFlags::SYNCHRONIZED,
            )
            || !method.exception_table().is_empty()
            || method.code().is_empty()
        {
            return Err(Error::Unsupported(format!("method {}", method.signature())));
        }
        if method.name() == "<clinit>" && (!method.is_static() || method.descriptor() != "()V") {
            return Err(Error::Unsupported("initializer signature".into()));
        }
        for field in method.parameters().iter().chain(method.return_type()) {
            self.admit_type(field)?;
        }
        for instruction in method.code() {
            match instruction {
                I::New(index) => {
                    self.class_constant(class, *index)?;
                }
                I::Getstatic(index) => {
                    self.static_field(class, *index)?;
                }
                I::Putstatic(index) => {
                    let (owner, field) = self.static_field(class, *index)?;
                    Self::admit_static_write(class, method, owner, &field)?;
                }
                I::Getfield(index) | I::Putfield(index) => {
                    self.field(class, *index)?;
                }
                I::Invokestatic(index) | I::Invokespecial(index) => {
                    let (owner, name, descriptor) = self.member(class, *index, false)?;
                    if owner == "java/lang/Object"
                        && name == "<init>"
                        && descriptor == "()V"
                        && matches!(instruction, I::Invokespecial(_))
                    {
                        continue;
                    }
                    if name == "<clinit>" {
                        return Err(Error::Unsupported("explicit initializer call".into()));
                    }
                    let target = self.class_index(&owner)?;
                    let target = self
                        .layout(target)?
                        .class
                        .try_get_method(&name, &descriptor)?;
                    let is_static = target.access_flags().contains(MethodAccessFlags::STATIC);
                    if is_static != matches!(instruction, I::Invokestatic(_))
                        || (!is_static && name != "<init>")
                    {
                        return Err(Error::Unsupported("call dispatch".to_owned()));
                    }
                }
                I::Ldc(index) => {
                    scalar::constant(
                        self.layout(class)?.class.constant_pool(),
                        u16::from(*index),
                        false,
                    )?;
                }
                I::Ldc_w(index) => {
                    scalar::constant(self.layout(class)?.class.constant_pool(), *index, false)?;
                }
                I::Ldc2_w(index) => {
                    scalar::constant(self.layout(class)?.class.constant_pool(), *index, true)?;
                }
                instruction
                    if scalar::supports(instruction)
                        || array_instruction(instruction).is_some() => {}
                I::Anewarray(index) => {
                    self.reference_component(class, *index)?;
                }
                I::Checkcast(_)
                | I::Aaload
                | I::Aastore
                | I::Newarray(_)
                | I::Arraylength
                | I::Lreturn
                | I::Freturn
                | I::Dreturn
                | I::Nop
                | I::Aconst_null
                | I::Iconst_m1
                | I::Iconst_0
                | I::Iconst_1
                | I::Iconst_2
                | I::Iconst_3
                | I::Iconst_4
                | I::Iconst_5
                | I::Bipush(_)
                | I::Sipush(_)
                | I::Iload(_)
                | I::Iload_0
                | I::Iload_1
                | I::Iload_2
                | I::Iload_3
                | I::Aload(_)
                | I::Aload_0
                | I::Aload_1
                | I::Aload_2
                | I::Aload_3
                | I::Istore(_)
                | I::Istore_0
                | I::Istore_1
                | I::Istore_2
                | I::Istore_3
                | I::Astore(_)
                | I::Astore_0
                | I::Astore_1
                | I::Astore_2
                | I::Astore_3
                | I::Iinc(_, _)
                | I::Iadd
                | I::Isub
                | I::Imul
                | I::Ineg
                | I::Iand
                | I::Ior
                | I::Ixor
                | I::Dup
                | I::Pop
                | I::Swap
                | I::Goto(_)
                | I::Ifeq(_)
                | I::Ifne(_)
                | I::Iflt(_)
                | I::Ifge(_)
                | I::Ifgt(_)
                | I::Ifle(_)
                | I::If_icmpeq(_)
                | I::If_icmpne(_)
                | I::If_icmplt(_)
                | I::If_icmpge(_)
                | I::If_icmpgt(_)
                | I::If_icmple(_)
                | I::If_acmpeq(_)
                | I::If_acmpne(_)
                | I::Ifnull(_)
                | I::Ifnonnull(_)
                | I::Ireturn
                | I::Areturn
                | I::Return => {}
                _ => return Err(Error::Unsupported(format!("bytecode {instruction:?}"))),
            }
        }
        Ok(())
    }

    /// Run an admitted static `(I)I` entry on a dedicated canonical heap, then reclaim
    /// every object before returning. Polls and bytecode batches yield to the host.
    ///
    /// # Errors
    /// Rejects configuration/entry errors before allocation; reports Java faults,
    /// work limits, heap exhaustion, collector failures or incomplete cleanup.
    pub async fn run(
        &self,
        class: &str,
        method: &str,
        argument: i32,
        configuration: Configuration,
        limits: Limits,
    ) -> Result<Report> {
        let (class, method) = self.entry_method(class, method)?;
        if configuration.mutators != 1
            || configuration.helpers != 1
            || self.classes.iter().any(|layout| {
                layout.references > configuration.edges_per_object
                    || layout.statics.references > configuration.edges_per_object
            })
        {
            return Err(Error::Unsupported(
                "single caller/helper or insufficient edge positions".to_owned(),
            ));
        }
        self.static_capacity(configuration)?;
        let charge = configuration.charged_bytes()?;
        let budget = Budget::new(charge);
        let collector = GarbageCollector::new();
        let runtime = Runtime::new(&collector, configuration, &budget)?;
        let mut execution = Execution {
            program: self,
            mutator: runtime.mutator()?,
            helper: runtime.helper()?,
            runtime: &runtime,
            limits,
            report: Report::default(),
            statics: States::default(),
        };
        let result = execution.entry(class, method, argument).await;
        // Release static storage and all frame roots before dedicated cleanup.
        execution.statics.close();
        execution.cleanup(&collector).await?;
        let mut report = execution.report.clone();
        drop(execution);
        drop(runtime);
        report.catalog_bytes_after_drop = budget.used();
        if report.catalog_bytes_after_drop != 0 {
            return Err(Error::Invalid(
                "catalog credit retained after drop".to_owned(),
            ));
        }
        report.result = result?
            .ok_or_else(|| Error::Invalid("missing result".to_owned()))?
            .int()?;
        Ok(report)
    }
}
#[derive(Debug)]
pub(super) enum Payload {
    Object {
        class: usize,
        integers: [AtomicI32; 8],
    },
    Array(PrimitiveArray),
    Statics {
        class: usize,
        integers: [AtomicI32; 8],
    },
    ReferenceArray {
        component: Component,
        layout: ReferenceArrayLayout,
    },
    ReferencePage {
        height: u8,
    },
}

impl Payload {
    fn object(class: usize) -> Self {
        Self::Object {
            class,
            integers: std::array::from_fn(|_| AtomicI32::new(0)),
        }
    }
    fn integer(&self, index: usize) -> Result<&AtomicI32> {
        if let Self::Object { integers, .. } = self {
            integers
                .get(index)
                .ok_or(Error::Boundary("integer field index"))
        } else {
            Err(Error::Boundary("object field on primitive array"))
        }
    }
    fn array(&self) -> Result<&PrimitiveArray> {
        if let Self::Array(array) = self {
            Ok(array)
        } else {
            Err(Error::ArrayFault("primitive array type mismatch"))
        }
    }
}

// SAFETY: Payload contains only numeric metadata and primitive atomic storage.
// Every managed field is owned exclusively by the catalog, never by this payload.
#[expect(
    unsafe_code,
    reason = "audited private reference-free canonical Java payload"
)]
impl Trace for Payload {
    fn canonical_payload() -> Option<ristretto_gc::CanonicalPayload<Self>> {
        // SAFETY: every field is primitive storage. Managed edges live in the catalog.
        Some(unsafe { ristretto_gc::CanonicalPayload::new() })
    }

    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
    fn needs_write_barrier(&self) -> bool {
        false
    }
    fn external_size(&self) -> usize {
        match self {
            Self::Object { .. }
            | Self::Statics { .. }
            | Self::ReferenceArray { .. }
            | Self::ReferencePage { .. } => 0,
            Self::Array(array) => array.backing_bytes(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Reference(Option<Root<Payload>>),
    Unused,
}

impl Value {
    fn width(&self) -> usize {
        if matches!(self, Self::Long(_) | Self::Double(_)) {
            2
        } else {
            1
        }
    }
    fn long(self) -> Result<i64> {
        if let Self::Long(value) = self {
            Ok(value)
        } else {
            Err(Error::Boundary("expected long"))
        }
    }
    fn float(self) -> Result<f32> {
        if let Self::Float(value) = self {
            Ok(value)
        } else {
            Err(Error::Boundary("expected float"))
        }
    }
    fn double(self) -> Result<f64> {
        if let Self::Double(value) = self {
            Ok(value)
        } else {
            Err(Error::Boundary("expected double"))
        }
    }
    pub(super) fn int(self) -> Result<i32> {
        if let Self::Int(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected int".to_owned()))
        }
    }
    pub(super) fn reference(self) -> Result<Option<Root<Payload>>> {
        if let Self::Reference(value) = self {
            Ok(value)
        } else {
            Err(Error::Invalid("expected reference".to_owned()))
        }
    }
    fn object(self) -> Result<Root<Payload>> {
        self.reference()?
            .ok_or_else(|| Error::Invalid("NullPointerException".to_owned()))
    }
}

impl From<ScalarValue> for Value {
    fn from(v: ScalarValue) -> Self {
        match v {
            ScalarValue::Int(n) => Self::Int(n),
            ScalarValue::Long(n) => Self::Long(n),
            ScalarValue::Float(n) => Self::Float(n),
            ScalarValue::Double(n) => Self::Double(n),
        }
    }
}
impl PrimitiveFrameValue for Value {
    type Slot = Self;
    fn primitive(self) -> Result<ScalarValue> {
        match self {
            Self::Int(n) => Ok(ScalarValue::Int(n)),
            Self::Long(n) => Ok(ScalarValue::Long(n)),
            Self::Float(n) => Ok(ScalarValue::Float(n)),
            Self::Double(n) => Ok(ScalarValue::Double(n)),
            _ => Err(Error::Invalid("expected primitive value".into())),
        }
    }
    fn load(locals: &[Self], stack: &mut Vec<Self>, index: usize) -> Result<()> {
        load(locals, stack, index)
    }
    fn store(locals: &mut [Self], stack: &mut Vec<Self>, index: usize) -> Result<()> {
        store(locals, stack, index)
    }
}
pub(super) struct Execution<'p, 'r, 'gc> {
    pub(super) program: &'p Program,
    pub(super) runtime: &'r Runtime<'gc>,
    pub(super) mutator: Mutator<'gc>,
    pub(super) helper: Helper<'gc>,
    pub(super) limits: Limits,
    pub(super) report: Report,
    pub(super) statics: States,
}

pub(super) async fn complete<T>(
    helper: &mut Helper<'_>,
    report: &mut Report,
    limit: usize,
    mut poll: impl FnMut() -> std::result::Result<Poll<T>, ristretto_gc::canonical::Error>,
) -> Result<T> {
    for _ in 0..limit {
        if let Poll::Ready(value) = poll()? {
            return Ok(value);
        }
        help(helper, report)?;
        tokio::task::yield_now().await;
    }
    Err(Error::Limit("request poll"))
}

pub(super) fn help(helper: &mut Helper<'_>, report: &mut Report) -> Result<()> {
    let work = helper.poll()?;
    report.helper_polls += 1;
    if work == Work::ReleasedObjectSlot {
        report.released_objects += 1;
    }
    Ok(())
}

impl Execution<'_, '_, '_> {
    async fn allocate_array(&mut self, kind: Kind, length: i32) -> Result<Root<Payload>> {
        let array =
            PrimitiveArray::new(kind, length, self.limits.array_bytes).map_err(array_error)?;
        let mut request = self.mutator.allocate(Payload::Array(array))?;
        let root = complete(
            &mut self.helper,
            &mut self.report,
            self.limits.request_polls,
            || request.poll(),
        )
        .await?;
        self.report.allocations += 1;
        self.report.array_allocations += 1;
        Ok(root)
    }

    fn array_length(&mut self, root: &Root<Payload>) -> Result<Value> {
        let view = self.runtime.borrow(root)?;
        let length = match &*view {
            Payload::Array(array) => array.len(),
            Payload::ReferenceArray { layout, .. } => {
                i32::try_from(layout.length).map_err(|_| Error::Boundary("array length"))?
            }
            _ => return Err(Error::ArrayFault("array type mismatch")),
        };
        self.report.array_reads += 1;
        Ok(Value::Int(length))
    }

    fn read_array(&mut self, root: &Root<Payload>, index: i32, kind: Kind) -> Result<Value> {
        let view = self.runtime.borrow(root)?;
        let array = view.array()?;
        if array.kind() != kind && !(kind == Kind::Byte && array.kind() == Kind::Boolean) {
            return Err(Error::ArrayFault("primitive array type mismatch"));
        }
        let value = match array.load(index).map_err(array_error)? {
            Element::Int(value) => Value::Int(value),
            Element::Long(value) => Value::Long(value),
            Element::Float(bits) => Value::Float(f32::from_bits(bits)),
            Element::Double(bits) => Value::Double(f64::from_bits(bits)),
        };
        self.report.array_reads += 1;
        Ok(value)
    }

    fn write_array(
        &mut self,
        root: &Root<Payload>,
        index: i32,
        kind: Kind,
        value: &Value,
    ) -> Result<()> {
        let view = self.runtime.borrow(root)?;
        let array = view.array()?;
        if array.kind() != kind && !(kind == Kind::Byte && array.kind() == Kind::Boolean) {
            return Err(Error::ArrayFault("primitive array type mismatch"));
        }
        let value = match value {
            Value::Int(value) => Element::Int(*value),
            Value::Long(value) => Element::Long(*value),
            Value::Float(value) => Element::Float(value.to_bits()),
            Value::Double(value) => Element::Double(value.to_bits()),
            _ => return Err(Error::ArrayFault("primitive array value type mismatch")),
        };
        array.store(index, value).map_err(array_error)?;
        self.report.array_writes += 1;
        Ok(())
    }

    async fn allocate_object(&mut self, class: usize) -> Result<Root<Payload>> {
        let payload = Payload::object(class);
        let mut request = self.mutator.allocate(payload)?;
        let root = complete(
            &mut self.helper,
            &mut self.report,
            self.limits.request_polls,
            || request.poll(),
        )
        .await?;
        self.report.allocations += 1;
        Ok(root)
    }

    async fn read_field(
        &mut self,
        class: usize,
        index: u16,
        root: &Root<Payload>,
    ) -> Result<Value> {
        let (owner, field) = self.program.field(class, index)?;
        self.check_class(root, owner)?;
        Ok(match field {
            Field::Int(index) => Value::Int(
                self.runtime
                    .borrow(root)?
                    .integer(index)?
                    .load(Ordering::Relaxed),
            ),
            Field::Reference { edge, .. } => {
                let mut request = self.mutator.child::<Payload, Payload>(root, edge)?;
                let child = complete(
                    &mut self.helper,
                    &mut self.report,
                    self.limits.request_polls,
                    || request.poll(),
                )
                .await?;
                self.report.reference_reads += 1;
                Value::Reference(child)
            }
        })
    }

    async fn write_field(
        &mut self,
        class: usize,
        index: u16,
        root: &Root<Payload>,
        value: Value,
    ) -> Result<()> {
        let (owner, field) = self.program.field(class, index)?;
        self.check_class(root, owner)?;
        match field {
            Field::Int(index) => self
                .runtime
                .borrow(root)?
                .integer(index)?
                .store(value.int()?, Ordering::Relaxed),
            Field::Reference {
                edge,
                target: expected,
            } => {
                let target = value.reference()?;
                if let Some(target) = &target {
                    self.check_reference(target, expected)?;
                }
                let mut request = self.mutator.write(root, edge, target.as_ref())?;
                complete(
                    &mut self.helper,
                    &mut self.report,
                    self.limits.request_polls,
                    || request.poll(),
                )
                .await?;
                self.report.reference_writes += 1;
            }
        }
        Ok(())
    }

    pub(super) async fn entry(
        &mut self,
        class: usize,
        method: Arc<Method>,
        argument: i32,
    ) -> Result<Option<Value>> {
        let initialized = self.initialize_class(class, 0).await;
        self.report.entry_initialization = InitializationWork::from_report(&self.report);
        initialized?;
        let pool = self.program.layout(class)?.class.constant_pool();
        if method
            .code()
            .iter()
            .all(|op| super::integer::admitted(op, pool))
        {
            self.report.calls += 1;
            let remaining = self
                .limits
                .instructions
                .checked_sub(self.report.instructions)
                .ok_or(Error::Limit("instruction"))?;
            let (value, count) = super::integer::run_counted(&method, pool, argument, remaining)?;
            self.report.instructions += count;
            Ok(Some(Value::Int(value.int()?)))
        } else {
            self.call(class, method, vec![Value::Int(argument)], 0)
                .await
        }
    }

    async fn cleanup(&mut self, collector: &GarbageCollector) -> Result<()> {
        for index in 0..self.limits.cleanup_polls {
            help(&mut self.helper, &mut self.report)?;
            let bytes = collector.managed_bytes();
            if bytes == 0 {
                self.report.payload_bytes_after_cleanup = bytes;
                return Ok(());
            }
            if index % 64 == 0 {
                tokio::task::yield_now().await;
            }
        }
        Err(Error::Limit("cleanup poll"))
    }

    fn check_type(&self, value: &Value, field: &FieldType) -> Result<()> {
        match (value, field) {
            (
                Value::Int(_),
                FieldType::Base(
                    BaseType::Int
                    | BaseType::Byte
                    | BaseType::Char
                    | BaseType::Short
                    | BaseType::Boolean,
                ),
            )
            | (Value::Long(_), FieldType::Base(BaseType::Long))
            | (Value::Float(_), FieldType::Base(BaseType::Float))
            | (Value::Double(_), FieldType::Base(BaseType::Double))
            | (Value::Reference(None), FieldType::Object(_) | FieldType::Array(_)) => Ok(()),
            (
                Value::Reference(Some(root)),
                field @ (FieldType::Object(_) | FieldType::Array(_)),
            ) => self.check_reference(root, reference_type(field, &self.program.names)?),
            _ => Err(Error::Invalid("method argument/return type".to_owned())),
        }
    }

    pub(super) fn check_class(&self, root: &Root<Payload>, class: usize) -> Result<()> {
        self.check_reference(root, ReferenceType::Object(class))
    }

    fn reference_kind(&self, root: &Root<Payload>) -> Result<ReferenceType> {
        Ok(match &*self.runtime.borrow(root)? {
            Payload::Object { class, .. } => ReferenceType::Object(*class),
            Payload::Statics { .. } => {
                return Err(Error::Boundary("static storage as Java reference"));
            }
            Payload::Array(array) => ReferenceType::Array(array.kind()),
            Payload::ReferenceArray { component, .. } => ReferenceType::ReferenceArray(*component),
            Payload::ReferencePage { .. } => {
                return Err(Error::Boundary("internal page as Java reference"));
            }
        })
    }

    pub(super) fn check_reference(
        &self,
        root: &Root<Payload>,
        expected: ReferenceType,
    ) -> Result<()> {
        if expected.accepts(self.reference_kind(root)?) {
            Ok(())
        } else {
            Err(Error::Invalid("receiver/field class mismatch".to_owned()))
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "single explicit restricted bytecode dispatch table"
    )]
    #[async_recursion::async_recursion(?Send)]
    pub(super) async fn call(
        &mut self,
        class: usize,
        method: Arc<Method>,
        arguments: Vec<Value>,
        depth: usize,
    ) -> Result<Option<Value>> {
        use Instruction as I;
        self.report.instruction_count_available = true;
        if depth >= self.limits.frames {
            return Err(Error::Limit("frame"));
        }
        let is_static = method.access_flags().contains(MethodAccessFlags::STATIC);
        if is_static && method.name() != "<clinit>" {
            self.initialize_class(class, depth).await?;
        }
        let receiver_count = usize::from(!is_static);
        if arguments.len() != method.parameters().len() + receiver_count {
            return Err(Error::Invalid("argument count".to_owned()));
        }
        if !is_static {
            let root = arguments
                .first()
                .cloned()
                .ok_or_else(|| Error::Invalid("receiver".to_owned()))?
                .object()?;
            self.check_class(&root, class)?;
        }
        for (argument, field) in arguments
            .iter()
            .skip(receiver_count)
            .zip(method.parameters())
        {
            self.check_type(argument, field)?;
        }
        let mut locals = vec![Value::Unused; method.max_locals()];
        let mut index = 0;
        for argument in arguments {
            let width = argument.width();
            *locals
                .get_mut(index)
                .ok_or_else(|| Error::Invalid("argument local".to_owned()))? = argument;
            index += width;
            if index > locals.len() {
                return Err(Error::Invalid("wide argument local".to_owned()));
            }
        }
        let mut stack = Vec::with_capacity(method.max_stack());
        let mut pc = 0;
        self.report.calls += 1;
        if method.name() == "<clinit>" {
            self.report.class_initializer_calls += 1;
        }
        loop {
            if self.report.instructions >= self.limits.instructions {
                return Err(Error::Limit("instruction"));
            }
            let instruction = method
                .code()
                .get(pc)
                .ok_or_else(|| Error::Invalid("program counter".to_owned()))?;
            self.report.instructions += 1;
            help(&mut self.helper, &mut self.report)?;
            if self.report.instructions.is_multiple_of(64) {
                tokio::task::yield_now().await;
            }
            match instruction {
                I::Nop => {}
                I::Aconst_null => stack.push(Value::Reference(None)),
                I::Iconst_m1 => stack.push(Value::Int(-1)),
                I::Iconst_0 => stack.push(Value::Int(0)),
                I::Iconst_1 => stack.push(Value::Int(1)),
                I::Iconst_2 => stack.push(Value::Int(2)),
                I::Iconst_3 => stack.push(Value::Int(3)),
                I::Iconst_4 => stack.push(Value::Int(4)),
                I::Iconst_5 => stack.push(Value::Int(5)),
                I::Bipush(value) => stack.push(Value::Int(i32::from(*value))),
                I::Sipush(value) => stack.push(Value::Int(i32::from(*value))),
                I::Ldc(index) => stack.push(Value::from(scalar::constant(
                    self.program.layout(class)?.class.constant_pool(),
                    u16::from(*index),
                    false,
                )?)),
                I::Ldc_w(index) => stack.push(Value::from(scalar::constant(
                    self.program.layout(class)?.class.constant_pool(),
                    *index,
                    false,
                )?)),
                I::Ldc2_w(index) => stack.push(Value::from(scalar::constant(
                    self.program.layout(class)?.class.constant_pool(),
                    *index,
                    true,
                )?)),
                instruction if scalar::supports(instruction) => {
                    scalar::execute(instruction, &mut locals, &mut stack)?;
                }
                I::Checkcast(index) => {
                    let reference = pop(&mut stack)?;
                    stack.push(
                        self.cast_reference(reference, self.program.cast_target(class, *index)?)?,
                    );
                }
                I::Anewarray(index) => {
                    let length = pop(&mut stack)?.int()?;
                    let component = self.program.reference_component(class, *index)?;
                    stack.push(Value::Reference(Some(
                        self.allocate_reference_array(component, length).await?,
                    )));
                }
                I::Aaload => {
                    let index = pop(&mut stack)?.int()?;
                    let root = pop(&mut stack)?.object()?;
                    stack.push(Value::Reference(
                        self.read_reference_array(&root, index).await?,
                    ));
                }
                I::Aastore => {
                    let value = pop(&mut stack)?.reference()?;
                    let index = pop(&mut stack)?.int()?;
                    let root = pop(&mut stack)?.object()?;
                    self.write_reference_array(&root, index, value.as_ref())
                        .await?;
                }
                I::Newarray(kind) => {
                    let length = pop(&mut stack)?.int()?;
                    stack.push(Value::Reference(Some(
                        self.allocate_array(allocation_kind(kind), length).await?,
                    )));
                }
                I::Arraylength => {
                    let root = pop(&mut stack)?.object()?;
                    stack.push(self.array_length(&root)?);
                }
                instruction if array_instruction(instruction).is_some() => {
                    let (kind, store) =
                        array_instruction(instruction).ok_or(Error::Boundary("array opcode"))?;
                    let value = if store { Some(pop(&mut stack)?) } else { None };
                    let index = pop(&mut stack)?.int()?;
                    let root = pop(&mut stack)?.object()?;
                    if let Some(value) = value {
                        self.write_array(&root, index, kind, &value)?;
                    } else {
                        stack.push(self.read_array(&root, index, kind)?);
                    }
                }
                I::Iload(index) | I::Aload(index) => {
                    load(&locals, &mut stack, usize::from(*index))?;
                }
                I::Iload_0 | I::Aload_0 => load(&locals, &mut stack, 0)?,
                I::Iload_1 | I::Aload_1 => load(&locals, &mut stack, 1)?,
                I::Iload_2 | I::Aload_2 => load(&locals, &mut stack, 2)?,
                I::Iload_3 | I::Aload_3 => load(&locals, &mut stack, 3)?,
                I::Istore(index) | I::Astore(index) => {
                    store(&mut locals, &mut stack, usize::from(*index))?;
                }
                I::Istore_0 | I::Astore_0 => store(&mut locals, &mut stack, 0)?,
                I::Istore_1 | I::Astore_1 => store(&mut locals, &mut stack, 1)?,
                I::Istore_2 | I::Astore_2 => store(&mut locals, &mut stack, 2)?,
                I::Istore_3 | I::Astore_3 => store(&mut locals, &mut stack, 3)?,
                I::Iinc(index, amount) => {
                    let value = locals
                        .get_mut(usize::from(*index))
                        .ok_or_else(|| Error::Invalid("local index".to_owned()))?;
                    *value = Value::Int(value.clone().int()?.wrapping_add(i32::from(*amount)));
                }
                I::Iadd | I::Isub | I::Imul | I::Iand | I::Ior | I::Ixor => {
                    let right = pop(&mut stack)?.int()?;
                    let left = pop(&mut stack)?.int()?;
                    let value = match instruction {
                        I::Iadd => left.wrapping_add(right),
                        I::Isub => left.wrapping_sub(right),
                        I::Imul => left.wrapping_mul(right),
                        I::Iand => left & right,
                        I::Ior => left | right,
                        _ => left ^ right,
                    };
                    stack.push(Value::Int(value));
                }
                I::Ineg => {
                    let value = pop(&mut stack)?.int()?;
                    stack.push(Value::Int(value.wrapping_neg()));
                }
                I::Dup => {
                    let value = stack
                        .last()
                        .cloned()
                        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))?;
                    stack.push(value);
                }
                I::Pop => {
                    pop(&mut stack)?;
                }
                I::Swap => {
                    let a = pop(&mut stack)?;
                    let b = pop(&mut stack)?;
                    stack.push(a);
                    stack.push(b);
                }
                I::New(index) => {
                    let class = self.program.class_constant(class, *index)?;
                    self.initialize_class(class, depth + 1).await?;
                    let root = self.allocate_object(class).await?;
                    stack.push(Value::Reference(Some(root)));
                }
                I::Getstatic(index) => {
                    let (owner, field) = self.program.static_field(class, *index)?;
                    stack.push(self.read_static(owner, &field, depth + 1).await?);
                }
                I::Putstatic(index) => {
                    let (owner, field) = self.program.static_field(class, *index)?;
                    let value = pop(&mut stack)?;
                    self.write_static(class, &method, owner, &field, value, depth + 1)
                        .await?;
                }
                I::Getfield(index) => {
                    let root = pop(&mut stack)?.object()?;
                    let value = self.read_field(class, *index, &root).await?;
                    stack.push(value);
                }
                I::Putfield(index) => {
                    let value = pop(&mut stack)?;
                    let root = pop(&mut stack)?.object()?;
                    self.write_field(class, *index, &root, value).await?;
                }
                I::Invokespecial(index) | I::Invokestatic(index) => {
                    let (owner, name, descriptor) = self.program.member(class, *index, false)?;
                    if owner == "java/lang/Object" {
                        // Admission permits only Object.<init>()V, whose body is empty.
                        pop(&mut stack)?.object()?;
                    } else {
                        let owner = self.program.class_index(&owner)?;
                        let target = self
                            .program
                            .layout(owner)?
                            .class
                            .try_get_method(&name, &descriptor)?;
                        let count = target.parameters().len()
                            + usize::from(matches!(instruction, I::Invokespecial(_)));
                        let start = stack
                            .len()
                            .checked_sub(count)
                            .ok_or_else(|| Error::Invalid("call stack underflow".to_owned()))?;
                        let arguments = stack.split_off(start);
                        if let Some(value) = self.call(owner, target, arguments, depth + 1).await? {
                            stack.push(value);
                        }
                    }
                }
                I::Return | I::Ireturn | I::Areturn | I::Lreturn | I::Freturn | I::Dreturn => {
                    let mut value = if matches!(instruction, I::Return) {
                        None
                    } else {
                        Some(pop(&mut stack)?)
                    };
                    match (value.as_ref(), method.return_type()) {
                        (None, None) => {}
                        (Some(value), Some(field)) => self.check_type(value, field)?,
                        _ => return Err(Error::Invalid("return type".to_owned())),
                    }
                    if let (Some(result), Some(field)) = (value.take(), method.return_type()) {
                        value = Some(match result {
                            Value::Int(v) => {
                                Value::from(scalar::normalize_return(ScalarValue::Int(v), field))
                            }
                            other => other,
                        });
                    }
                    return Ok(value);
                }
                instruction => {
                    if branch(instruction, &mut stack)? {
                        pc = method
                            .branch_target_index(pc)
                            .ok_or_else(|| Error::Invalid("branch target".to_owned()))?;
                        continue;
                    }
                }
            }
            if stack.iter().map(Value::width).sum::<usize>() > method.max_stack() {
                return Err(Error::Invalid("operand stack overflow".to_owned()));
            }
            pc += 1;
        }
    }
}

fn pop(stack: &mut Vec<Value>) -> Result<Value> {
    stack
        .pop()
        .ok_or_else(|| Error::Invalid("stack underflow".to_owned()))
}
fn load(locals: &[Value], stack: &mut Vec<Value>, index: usize) -> Result<()> {
    let value = locals
        .get(index)
        .ok_or_else(|| Error::Invalid("local index".to_owned()))?
        .clone();
    if matches!(value, Value::Unused) {
        return Err(Error::Invalid("uninitialized local".to_owned()));
    }
    stack.push(value);
    Ok(())
}
fn store(locals: &mut [Value], stack: &mut Vec<Value>, index: usize) -> Result<()> {
    let value = pop(stack)?;
    if value.width() == 2 {
        *locals
            .get_mut(index + 1)
            .ok_or(Error::Boundary("wide local index"))? = Value::Unused;
    }
    if index > 0
        && locals
            .get(index - 1)
            .is_some_and(|value| value.width() == 2)
    {
        *locals
            .get_mut(index - 1)
            .ok_or(Error::Boundary("local index"))? = Value::Unused;
    }
    *locals
        .get_mut(index)
        .ok_or_else(|| Error::Invalid("local index".to_owned()))? = value;
    Ok(())
}
fn branch(instruction: &Instruction, stack: &mut Vec<Value>) -> Result<bool> {
    use Instruction as I;
    Ok(match instruction {
        I::Goto(_) => true,
        I::Ifeq(_) => pop(stack)?.int()? == 0,
        I::Ifne(_) => pop(stack)?.int()? != 0,
        I::Iflt(_) => pop(stack)?.int()? < 0,
        I::Ifge(_) => pop(stack)?.int()? >= 0,
        I::Ifgt(_) => pop(stack)?.int()? > 0,
        I::Ifle(_) => pop(stack)?.int()? <= 0,
        I::Ifnull(_) => pop(stack)?.reference()?.is_none(),
        I::Ifnonnull(_) => pop(stack)?.reference()?.is_some(),
        I::If_icmpeq(_)
        | I::If_icmpne(_)
        | I::If_icmplt(_)
        | I::If_icmpge(_)
        | I::If_icmpgt(_)
        | I::If_icmple(_) => {
            let right = pop(stack)?.int()?;
            let left = pop(stack)?.int()?;
            match instruction {
                I::If_icmpeq(_) => left == right,
                I::If_icmpne(_) => left != right,
                I::If_icmplt(_) => left < right,
                I::If_icmpge(_) => left >= right,
                I::If_icmpgt(_) => left > right,
                _ => left <= right,
            }
        }
        I::If_acmpeq(_) | I::If_acmpne(_) => {
            let right = pop(stack)?.reference()?;
            let left = pop(stack)?.reference()?;
            let same = match (&left, &right) {
                (None, None) => true,
                (Some(a), Some(b)) => a.same_object(b),
                _ => false,
            };
            if matches!(instruction, I::If_acmpeq(_)) {
                same
            } else {
                !same
            }
        }
        _ => return Err(Error::Unsupported(format!("bytecode {instruction:?}"))),
    })
}
