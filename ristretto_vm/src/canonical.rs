//! Explicit admission and execution of a closed set of restricted Java classes.

#[path = "canonical_heap.rs"]
mod heap;
mod integer;
mod layout;
mod objects;
#[path = "canonical_statics.rs"]
mod statics;
pub use heap::{HeapCompletion, Invocation, JavaHeap, JavaThread};
mod primitive_array;
mod reference_array;
use layout::Layout;
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
pub use objects::{CompiledLimits, CompiledProgram};
pub use objects::{InitializationWork, Limits, Report};
mod scalar;
mod value;
pub use value::ScalarValue;
use value::ScalarValue as Value;
use value::{load, pop, store};

use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::{MemberAccess, VerificationContext};
use ristretto_classfile::verifiers::error::{Result as VerificationResult, VerifyError};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, FieldAccessFlags, JavaStr, JavaString, MethodAccessFlags,
};
use ristretto_classloader::{Class, Method};
use std::collections::{HashMap, HashSet};

/// A format, verifier or capability failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported canonical Java capability: {0}")]
    Unsupported(String),
    #[error("invalid canonical Java execution: {0}")]
    Invalid(String),
    #[error("canonical Java initialization {kind}: {cause:?}")]
    Initialization {
        kind: &'static str,
        cause: Option<Box<Error>>,
    },
    #[error("canonical Java {0} limit reached")]
    Limit(&'static str),
    #[error("canonical Java array fault: {0}")]
    ArrayFault(&'static str),
    #[error("canonical Java type fault: {0}")]
    TypeFault(&'static str),
    #[error("canonical ownership boundary: {0}")]
    Boundary(&'static str),
    #[error(transparent)]
    Heap(#[from] ristretto_gc::Error),
    #[error(transparent)]
    Collector(#[from] ristretto_gc::canonical::Error),
    #[error(transparent)]
    Classfile(#[from] ristretto_classfile::Error),
    #[error(transparent)]
    Classloader(#[from] ristretto_classloader::Error),
    #[cfg(feature = "experimental-canonical-jit")]
    #[error(transparent)]
    Jit(#[from] ristretto_jit::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Closed set of admitted classfiles. Each class directly extends Object.
#[derive(Debug)]
pub struct Program {
    classes: Vec<Layout>,
    names: HashMap<JavaString, usize>,
}

impl Program {
    /// Verify all supplied methods before making an entry available.
    ///
    /// # Errors
    /// Reports format, verification, linking and capability errors.
    pub fn from_classfiles(files: impl IntoIterator<Item = ClassFile<'static>>) -> Result<Self> {
        let mut admitted = Vec::new();
        let mut names = HashMap::new();
        for file in files {
            file.verify()?;
            let name = file.class_name()?.to_owned();
            if file.constant_pool.try_get_class(file.super_class)? != "java/lang/Object"
                || name.as_bytes().starts_with(b"java/")
                || !file.interfaces.is_empty()
                || file.access_flags.intersects(
                    ClassAccessFlags::INTERFACE
                        | ClassAccessFlags::ABSTRACT
                        | ClassAccessFlags::MODULE,
                )
            {
                return Err(Error::Unsupported(format!("class layout {name}")));
            }
            if names.insert(name, admitted.len()).is_some() {
                return Err(Error::Invalid("duplicate class".into()));
            }
            admitted.push(file);
        }
        if admitted.is_empty() {
            return Err(Error::Invalid("empty program".into()));
        }
        let classes = admitted
            .into_iter()
            .map(|file| Layout::new(file, &names))
            .collect::<Result<Vec<_>>>()?;
        let mut program = Self { classes, names };
        for (index, layout) in program.classes.iter().enumerate() {
            let class = &layout.class;
            for method in class.methods() {
                program.admit_method(index, class, &method)?;
            }
            verify_class(class.class_file(), &program, &VerifierConfig::strict())
                .map_err(|error| Error::Invalid(error.to_string()))?;
        }
        program.admit_called_targets()?;
        program.link_members()?;
        Ok(program)
    }

    // Scalar entry methods have a wider instruction set than owning object
    // frames. Validate every called target once, without recursively following
    // calls, so admission cannot authorize an unsupported nested dispatch path.
    fn admit_called_targets(&self) -> Result<()> {
        let mut checked = HashSet::new();
        for (owner, layout) in self.classes.iter().enumerate() {
            for method in layout.class.methods() {
                for instruction in method.code() {
                    let (Instruction::Invokestatic(index) | Instruction::Invokespecial(index)) =
                        instruction
                    else {
                        continue;
                    };
                    let (class, name, descriptor) = self.member(owner, *index, false)?;
                    if class == "java/lang/Object" {
                        continue;
                    }
                    let class = self.class_index(&class)?;
                    if checked.insert((class, name.clone(), descriptor.clone())) {
                        let target = self
                            .layout(class)?
                            .class
                            .try_get_method(&name, &descriptor)?;
                        self.admit_object_method(class, &target)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn admit_method(&self, owner: usize, class: &Class, method: &Method) -> Result<()> {
        use Instruction as I;
        if !method.exception_table().is_empty()
            || method.access_flags().intersects(
                MethodAccessFlags::SYNCHRONIZED
                    | MethodAccessFlags::NATIVE
                    | MethodAccessFlags::ABSTRACT,
            )
        {
            return Err(Error::Unsupported(method.signature()));
        }
        if method.name() == "<init>"
            && method.descriptor() == "()V"
            && !method.is_static()
            && let [I::Aload_0, I::Invokespecial(index), I::Return] = method.code().as_slice()
        {
            let pool = class.constant_pool();
            let (owner, signature) = pool.try_get_method_ref(*index)?;
            let (name, descriptor) = pool.try_get_name_and_type(*signature)?;
            if pool.try_get_class(*owner)? == "java/lang/Object"
                && pool.try_get_utf8(*name)? == "<init>"
                && pool.try_get_utf8(*descriptor)? == "()V"
            {
                return Ok(());
            }
        }
        if method.is_static()
            && matches!(
                method.descriptor(),
                "(I)I" | "(I)B" | "(I)S" | "(I)C" | "(I)Z" | "(I)J" | "(I)F" | "(I)D"
            )
            && !method.code().is_empty()
            && method
                .code()
                .iter()
                .all(|op| integer::admitted(op, class.constant_pool()))
        {
            return Ok(());
        }
        self.admit_object_method(owner, method)
    }

    /// Execute a public scalar entry with its single Java int argument.
    ///
    /// # Errors
    /// Reports a missing or inaccessible entry.
    pub fn run_scalar(&self, class: &str, method: &str, argument: i32) -> Result<i32> {
        self.run_scalar_with_limit(class, method, argument, 1_000_000)
    }

    /// Execute a scalar entry with an explicit maximum number of interpreted instructions.
    ///
    /// # Errors
    /// Reports entry resolution or execution errors and instruction-budget exhaustion.
    pub fn run_scalar_with_limit(
        &self,
        class: &str,
        method: &str,
        argument: i32,
        limit: usize,
    ) -> Result<i32> {
        self.run_primitive(class, method, "(I)I", argument, limit)?
            .int()
    }

    /// Run a verified static entry with one int argument and a primitive result.
    ///
    /// # Errors
    /// Reports an inaccessible entry, an unsupported descriptor or exhausted instruction budget.
    pub fn run_primitive(
        &self,
        class: &str,
        method: &str,
        descriptor: &str,
        argument: i32,
        limit: usize,
    ) -> Result<ScalarValue> {
        if self.layout(self.class_index(class)?)?.statics.needed() {
            return Err(Error::Unsupported(
                "class initialization requires a heap entry".into(),
            ));
        }
        let name = JavaString::from(class);
        let owner = self
            .names
            .get(&name)
            .and_then(|index| self.classes.get(*index))
            .map(|layout| &layout.class)
            .ok_or_else(|| Error::Unsupported(class.into()))?;
        let entry = owner.try_get_method(method, descriptor)?;
        if !entry.is_static() || !entry.access_flags().contains(MethodAccessFlags::PUBLIC) {
            return Err(Error::Unsupported(entry.signature()));
        }
        integer::run(&entry, owner.constant_pool(), argument, limit)
    }

    fn known(&self, name: &JavaStr) -> VerificationResult<()> {
        if name == "java/lang/Object" || self.names.contains_key(name) {
            Ok(())
        } else {
            Err(VerifyError::VerifyError(format!("unresolved class {name}")))
        }
    }
}

fn exact_symbol(name: &JavaStr) -> VerificationResult<std::borrow::Cow<'_, str>> {
    let decoded = name.to_str_lossy();
    if JavaStr::cow_from_str(&decoded).as_ref() == name {
        Ok(decoded)
    } else {
        Err(VerifyError::VerifyError(
            "unsupported unpaired surrogate in symbol".into(),
        ))
    }
}

impl VerificationContext for Program {
    fn verify_member_access(&self, access: &MemberAccess<'_>) -> VerificationResult<()> {
        let member_name = exact_symbol(access.name)?;
        let descriptor = exact_symbol(access.descriptor)?;
        if access.owner == "java/lang/Object" {
            // The capability pass has already restricted this to its empty constructor.
            return Ok(());
        }
        let current = self
            .names
            .get(access.current_class)
            .and_then(|index| self.classes.get(*index));
        let owner = self
            .names
            .get(access.owner)
            .and_then(|index| self.classes.get(*index));
        let (Some(current), Some(owner)) = (current, owner) else {
            return Err(VerifyError::VerifyError(
                "unresolved canonical member owner".to_owned(),
            ));
        };
        let same_class = access.current_class == access.owner;
        let package = |name: &str| {
            name.rsplit_once('/')
                .map_or("", |(package, _)| package)
                .to_owned()
        };
        let same_package = package(current.class.name()) == package(owner.class.name());
        if !same_package
            && !owner
                .class
                .class_file()
                .access_flags
                .contains(ClassAccessFlags::PUBLIC)
        {
            return Err(VerifyError::VerifyError(
                "inaccessible canonical class".to_owned(),
            ));
        }
        let (public, private) = if matches!(
            access.instruction,
            Instruction::Getfield(_)
                | Instruction::Putfield(_)
                | Instruction::Getstatic(_)
                | Instruction::Putstatic(_)
        ) {
            let field = owner
                .class
                .declared_fields()
                .into_iter()
                .find(|field| {
                    field.name() == member_name.as_ref()
                        && field.field_type().descriptor() == descriptor.as_ref()
                })
                .ok_or_else(|| VerifyError::VerifyError("unresolved canonical field".to_owned()))?;
            (
                field.access_flags().contains(FieldAccessFlags::PUBLIC),
                field.access_flags().contains(FieldAccessFlags::PRIVATE),
            )
        } else {
            let method = owner
                .class
                .method(member_name.as_ref(), descriptor.as_ref())
                .ok_or_else(|| {
                    VerifyError::VerifyError("unresolved canonical method".to_owned())
                })?;
            (
                method.access_flags().contains(MethodAccessFlags::PUBLIC),
                method.access_flags().contains(MethodAccessFlags::PRIVATE),
            )
        };
        if public || same_class || (same_package && !private) {
            Ok(())
        } else {
            // Private nestmate access and protected cross-package inheritance are not admitted.
            Err(VerifyError::VerifyError(
                "unsupported/inaccessible canonical member".to_owned(),
            ))
        }
    }

    fn is_subclass(&self, subclass: &JavaStr, superclass: &JavaStr) -> VerificationResult<bool> {
        self.known(subclass)?;
        self.known(superclass)?;
        Ok(subclass == superclass || superclass == "java/lang/Object")
    }

    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> VerificationResult<bool> {
        self.is_subclass(source, target)
    }

    fn common_superclass(&self, a: &JavaStr, b: &JavaStr) -> VerificationResult<JavaString> {
        self.known(a)?;
        self.known(b)?;
        Ok(if a == b {
            a.to_owned()
        } else {
            "java/lang/Object".into()
        })
    }
}
