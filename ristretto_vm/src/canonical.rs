//! Explicit admission and execution of a closed set of restricted Java classes.

mod integer;
mod scalar;
mod value;
pub use value::ScalarValue;
use value::ScalarValue as Value;
use value::{load, pop, store};

use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::{MemberAccess, VerificationContext};
use ristretto_classfile::verifiers::error::{Result as VerificationResult, VerifyError};
use ristretto_classfile::{ClassAccessFlags, ClassFile, JavaStr, JavaString, MethodAccessFlags};
use ristretto_classloader::{Class, Method};
use std::collections::HashMap;
use std::sync::Arc;

/// A format, verifier or capability failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported canonical Java capability: {0}")]
    Unsupported(String),
    #[error("invalid canonical Java execution: {0}")]
    Invalid(String),
    #[error("canonical Java {0} limit reached")]
    Limit(&'static str),
    #[error(transparent)]
    Classfile(#[from] ristretto_classfile::Error),
    #[error(transparent)]
    Classloader(#[from] ristretto_classloader::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Closed set of admitted classfiles. Each class directly extends Object.
#[derive(Debug)]
pub struct Program {
    classes: HashMap<JavaString, Arc<Class>>,
}

impl Program {
    /// Verify all supplied methods before making an entry available.
    ///
    /// # Errors
    /// Reports format, verification, linking and capability errors.
    pub fn from_classfiles(files: impl IntoIterator<Item = ClassFile<'static>>) -> Result<Self> {
        let mut classes = HashMap::new();
        for file in files {
            file.verify()?;
            let name = file.class_name()?.to_owned();
            if file.constant_pool.try_get_class(file.super_class)? != "java/lang/Object"
                || name.as_bytes().starts_with(b"java/")
                || !file.fields.is_empty()
                || !file.interfaces.is_empty()
                || file.access_flags.intersects(
                    ClassAccessFlags::INTERFACE
                        | ClassAccessFlags::ABSTRACT
                        | ClassAccessFlags::MODULE,
                )
            {
                return Err(Error::Unsupported(format!("class layout {name}")));
            }
            if classes.insert(name, Class::from(None, file)?).is_some() {
                return Err(Error::Invalid("duplicate class".into()));
            }
        }
        if classes.is_empty() {
            return Err(Error::Invalid("empty program".into()));
        }
        let program = Self { classes };
        for class in program.classes.values() {
            for method in class.methods() {
                Self::admit_method(class, &method)?;
            }
            verify_class(class.class_file(), &program, &VerifierConfig::strict())
                .map_err(|error| Error::Invalid(error.to_string()))?;
        }
        Ok(program)
    }

    fn admit_method(class: &Class, method: &Method) -> Result<()> {
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
        Err(Error::Unsupported(method.signature()))
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
        let name = JavaString::from(class);
        let owner = self
            .classes
            .get(&name)
            .ok_or_else(|| Error::Unsupported(class.into()))?;
        let entry = owner.try_get_method(method, descriptor)?;
        if !entry.is_static() || !entry.access_flags().contains(MethodAccessFlags::PUBLIC) {
            return Err(Error::Unsupported(entry.signature()));
        }
        integer::run(&entry, owner.constant_pool(), argument, limit)
    }

    fn known(&self, name: &JavaStr) -> VerificationResult<()> {
        if name == "java/lang/Object" || self.classes.contains_key(name) {
            Ok(())
        } else {
            Err(VerifyError::VerifyError(format!("unresolved class {name}")))
        }
    }
}

impl VerificationContext for Program {
    fn verify_member_access(&self, access: &MemberAccess<'_>) -> VerificationResult<()> {
        self.known(access.current_class)?;
        if matches!(access.instruction, Instruction::Invokespecial(_))
            && access.owner == "java/lang/Object"
            && access.name == "<init>"
            && access.descriptor == "()V"
        {
            Ok(())
        } else {
            Err(VerifyError::VerifyError(format!(
                "unsupported member {}",
                access.name
            )))
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
