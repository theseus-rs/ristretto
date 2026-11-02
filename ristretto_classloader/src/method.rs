use crate::Result;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction, LineNumber};
use ristretto_classfile::{ClassFile, FieldType, MethodAccessFlags};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

/// Methods borrow immutable definitions from their class; standalone/synthetic methods own theirs.
#[derive(Clone)]
enum MethodDefinition {
    Owned(Box<ristretto_classfile::Method>),
    Shared {
        class_file: Arc<ClassFile<'static>>,
        index: usize,
    },
}

impl Deref for MethodDefinition {
    type Target = ristretto_classfile::Method;

    #[expect(
        clippy::expect_used,
        reason = "from_shared validates the index and retains an immutable classfile"
    )]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(definition) => definition,
            Self::Shared { class_file, index } => class_file
                .methods
                .get(*index)
                .expect("validated method index"),
        }
    }
}

impl Debug for MethodDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, formatter)
    }
}

impl PartialEq for MethodDefinition {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    definition: MethodDefinition,
    access_flags: MethodAccessFlags,
    name: String,
    descriptor: String,
    parameters: Vec<FieldType>,
    return_type: Option<FieldType>,
    max_stack: usize,
    max_locals: usize,
    code_attribute: Option<usize>,
    instruction_offsets: Vec<u16>,
    instruction_indices: Vec<u16>,
    branch_targets: Vec<u16>,
    line_numbers: Vec<LineNumber>,
    exception_table: Vec<ExceptionTableEntry>,
}

impl Method {
    /// Create a new class method with the given definition.
    ///
    /// # Errors
    ///
    /// if the method name cannot be read.
    pub fn from(
        class_file: &ClassFile<'static>,
        definition: &ristretto_classfile::Method,
    ) -> Result<Self> {
        Self::from_definition(
            class_file,
            definition,
            MethodDefinition::Owned(Box::new(definition.clone())),
        )
    }

    pub(crate) fn from_shared(class_file: &Arc<ClassFile<'static>>, index: usize) -> Result<Self> {
        let definition = class_file.methods.get(index).ok_or_else(|| {
            crate::Error::InternalError("Invalid method definition index".to_owned())
        })?;
        Self::from_definition(
            class_file,
            definition,
            MethodDefinition::Shared {
                class_file: class_file.clone(),
                index,
            },
        )
    }

    fn from_definition(
        class_file: &ClassFile<'static>,
        definition: &ristretto_classfile::Method,
        owner: MethodDefinition,
    ) -> Result<Self> {
        let constant_pool = &class_file.constant_pool;
        let name = constant_pool.try_get_utf8(definition.name_index)?;
        let descriptor = constant_pool.try_get_utf8(definition.descriptor_index)?;
        let code_attribute = definition
            .attributes
            .iter()
            .position(|attribute| matches!(attribute, Attribute::Code { .. }));
        let (max_stack, max_locals, code, line_numbers, exception_table) =
            match code_attribute.and_then(|index| definition.attributes.get(index)) {
                Some(Attribute::Code {
                    max_stack,
                    max_locals,
                    code,
                    attributes,
                    exception_table,
                    ..
                }) => {
                    let line_numbers = match attributes
                        .iter()
                        .find(|attribute| matches!(attribute, Attribute::LineNumberTable { .. }))
                    {
                        Some(Attribute::LineNumberTable { line_numbers, .. }) => {
                            // TODO: avoid cloning line numbers
                            line_numbers.clone()
                        }
                        _ => Vec::new(),
                    };
                    (
                        usize::from(*max_stack),
                        usize::from(*max_locals),
                        code.as_slice(),
                        line_numbers,
                        exception_table.clone(), // TODO: avoid cloning exception_table
                    )
                }
                _ => (0, 0, &[][..], Vec::new(), Vec::new()),
            };

        // Keep bytecode operands intact. These tables only locate decoded instructions.
        let mut instruction_offsets = Vec::with_capacity(code.len() + 1);
        let mut instruction_indices = Vec::new();
        for (index, (offset, instruction)) in Instruction::with_offsets(code).enumerate() {
            instruction_offsets.push(u16::try_from(offset)?);
            instruction_indices.push(u16::try_from(index)?);
            instruction_indices.resize(offset + instruction.byte_len(offset), u16::MAX);
        }
        instruction_offsets.push(u16::try_from(instruction_indices.len())?);

        // Execution-only cache. The classfile instructions retain signed byte displacements.
        let branch_targets = code
            .iter()
            .zip(&instruction_offsets)
            .map(|(instruction, &offset)| {
                instruction
                    .branch_offsets()
                    .next()
                    .and_then(|relative| {
                        usize::try_from(i64::from(offset) + i64::from(relative)).ok()
                    })
                    .and_then(|target| instruction_indices.get(target).copied())
                    .unwrap_or(u16::MAX)
            })
            .collect();

        let (parameters, return_type) = FieldType::parse_method_descriptor(descriptor)?;
        Ok(Self {
            definition: owner,
            access_flags: definition.access_flags,
            name: name.to_rust_string(),
            descriptor: descriptor.to_rust_string(),
            parameters,
            return_type,
            max_stack,
            max_locals,
            code_attribute,
            instruction_offsets,
            instruction_indices,
            branch_targets,
            line_numbers,
            exception_table,
        })
    }

    /// Create a synthetic method for intrinsics that don't have class file entries. This is used
    /// for holder class methods that are dynamically generated by the JVM.
    #[must_use]
    pub fn new_synthetic(
        definition: ristretto_classfile::Method,
        name: String,
        descriptor: String,
        parameters: Vec<FieldType>,
        return_type: Option<FieldType>,
    ) -> Self {
        let access_flags = definition.access_flags;
        Self {
            definition: MethodDefinition::Owned(Box::new(definition)),
            access_flags,
            name,
            descriptor,
            parameters,
            return_type,
            max_stack: 0,
            max_locals: 0,
            code_attribute: None,
            instruction_offsets: vec![0],
            instruction_indices: Vec::new(),
            branch_targets: Vec::new(),
            line_numbers: Vec::new(),
            exception_table: Vec::new(),
        }
    }

    /// Get the method definition.
    #[must_use]
    pub fn definition(&self) -> &ristretto_classfile::Method {
        &self.definition
    }

    /// Get the method access flags.
    #[must_use]
    pub fn access_flags(&self) -> &MethodAccessFlags {
        &self.access_flags
    }

    /// Check if the method is public.
    #[must_use]
    pub fn is_public(&self) -> bool {
        self.access_flags.contains(MethodAccessFlags::PUBLIC)
    }

    /// Check if the method is private.
    #[must_use]
    pub fn is_private(&self) -> bool {
        self.access_flags.contains(MethodAccessFlags::PRIVATE)
    }

    /// Check if the method is native.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.access_flags.contains(MethodAccessFlags::NATIVE)
    }

    /// Check if the method is static.
    #[must_use]
    pub fn is_static(&self) -> bool {
        self.access_flags.contains(MethodAccessFlags::STATIC)
    }

    /// Check if the method is abstract.
    #[must_use]
    pub fn is_abstract(&self) -> bool {
        self.access_flags.contains(MethodAccessFlags::ABSTRACT)
    }

    /// Get the method name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the method descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    /// Get the method parameters.
    #[must_use]
    pub fn parameters(&self) -> &Vec<FieldType> {
        &self.parameters
    }

    /// Get the method return type.
    #[must_use]
    pub fn return_type(&self) -> Option<&FieldType> {
        self.return_type.as_ref()
    }

    /// Get the method signature.
    #[must_use]
    pub fn signature(&self) -> String {
        format!("{}{}", self.name, self.descriptor)
    }

    /// Get the maximum stack size.
    #[must_use]
    pub fn max_stack(&self) -> usize {
        self.max_stack
    }

    /// Get the maximum number of local variables.
    #[must_use]
    pub fn max_locals(&self) -> usize {
        self.max_locals
    }

    /// Get the code.
    #[must_use]
    pub fn code(&self) -> &Vec<Instruction> {
        static EMPTY: Vec<Instruction> = Vec::new();
        match self
            .code_attribute
            .and_then(|index| self.definition.attributes.get(index))
        {
            Some(Attribute::Code { code, .. }) => code,
            _ => &EMPTY,
        }
    }

    /// Locate a decoded instruction by its JVM bytecode offset.
    #[inline]
    #[must_use]
    pub fn instruction_index(&self, byte_offset: usize) -> Option<usize> {
        let index = *self.instruction_indices.get(byte_offset)?;
        (index != u16::MAX).then_some(usize::from(index))
    }

    /// Byte offset of an instruction, or the exclusive code end at `code().len()`.
    #[inline]
    #[must_use]
    pub fn instruction_offset(&self, index: usize) -> Option<usize> {
        self.instruction_offsets
            .get(index)
            .copied()
            .map(usize::from)
    }

    /// Cached decoded destination for the instruction's first signed branch operand.
    /// This execution cache never changes the bytecode or its JVM offset representation.
    #[inline]
    #[must_use]
    pub fn branch_target_index(&self, index: usize) -> Option<usize> {
        let target = *self.branch_targets.get(index)?;
        (target != u16::MAX).then_some(usize::from(target))
    }

    /// Get an instruction at a JVM bytecode boundary.
    #[inline]
    #[must_use]
    pub fn instruction_at(&self, byte_offset: usize) -> Option<&Instruction> {
        self.code().get(self.instruction_index(byte_offset)?)
    }

    /// Byte offset immediately after the instruction at `byte_offset`.
    #[inline]
    #[must_use]
    pub fn next_offset(&self, byte_offset: usize) -> Option<usize> {
        self.instruction_offset(self.instruction_index(byte_offset)? + 1)
    }

    /// Get the line number for a given program counter.
    ///
    /// # Errors
    ///
    /// if the program counter does not index into a valid line number
    #[must_use]
    pub fn line_number(&self, program_counter: usize) -> usize {
        let program_counter = u16::try_from(program_counter).unwrap_or(0);
        let index = self
            .line_numbers
            .binary_search_by(|line_number| line_number.start_pc.cmp(&program_counter))
            .unwrap_or_else(|index| index.saturating_sub(1));
        let line_number = self
            .line_numbers
            .get(index)
            .map_or(0, |line_number| line_number.line_number);
        usize::from(line_number)
    }

    /// Get the exception table.
    #[must_use]
    pub fn exception_table(&self) -> &Vec<ExceptionTableEntry> {
        &self.exception_table
    }

    /// Get the attributes.
    #[must_use]
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.definition.attributes
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ");
        let return_type = match &self.return_type {
            Some(field_type) => field_type.to_string(),
            None => "void".to_string(),
        };
        write!(f, "{}({parameters}) -> {return_type}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::ConstantPool;

    #[test]
    fn test_method() -> Result<()> {
        let mut constant_pool = ConstantPool::new();
        let name_index = constant_pool.add_utf8("test")?;
        let descriptor_index = constant_pool.add_utf8("()V")?;
        let code_attribute = Attribute::Code {
            name_index: 0,
            max_stack: 1,
            max_locals: 2,
            code: Vec::new(),
            exception_table: Vec::new(),
            attributes: Vec::new(),
        };
        let method = ristretto_classfile::Method {
            name_index,
            descriptor_index,
            attributes: vec![code_attribute],
            ..Default::default()
        };
        let class_file = ClassFile {
            constant_pool,
            methods: vec![method.clone()],
            ..Default::default()
        };
        let method = Method::from(&class_file, &method)?;
        assert_eq!(method.access_flags(), &MethodAccessFlags::empty());
        assert!(!method.is_public());
        assert!(!method.is_private());
        assert!(!method.is_native());
        assert!(!method.is_static());
        assert!(!method.is_abstract());
        assert_eq!(method.name(), "test");
        assert_eq!(method.descriptor(), "()V");
        assert_eq!(method.signature(), "test()V");
        assert_eq!(method.parameters().as_slice(), []);
        assert_eq!(method.return_type(), None);
        assert_eq!(method.max_stack, 1);
        assert_eq!(method.max_locals, 2);
        assert_eq!(method.code().as_slice(), []);
        assert_eq!(method.line_number(0), 0);
        Ok(())
    }

    #[test]
    fn test_branch_cache_preserves_signed_bytes_and_boundaries() -> Result<()> {
        let mut class_file = ClassFile::default();
        let name_index = class_file.constant_pool.add_utf8("branches")?;
        let descriptor_index = class_file.constant_pool.add_utf8("()V")?;
        for (code, expected) in [
            (
                vec![
                    Instruction::Bipush(1),
                    Instruction::Ifeq(6),
                    Instruction::Goto(-5),
                    Instruction::Return,
                ],
                vec![None, Some(3), Some(0), None],
            ),
            (
                vec![Instruction::Goto_w(5), Instruction::Goto_w(-5)],
                vec![Some(1), Some(0)],
            ),
            (vec![Instruction::Goto(1)], vec![None]),
            (vec![Instruction::Goto(-1)], vec![None]),
            (vec![Instruction::Goto(3)], vec![None]),
        ] {
            let bytes = Instruction::encode(&code)?;
            let definition = ristretto_classfile::Method {
                name_index,
                descriptor_index,
                attributes: vec![Attribute::Code {
                    name_index: 0,
                    max_stack: 1,
                    max_locals: 0,
                    code,
                    exception_table: vec![],
                    attributes: vec![],
                }],
                ..Default::default()
            };
            let method = Method::from(&class_file, &definition)?;
            assert_eq!(Instruction::encode(method.code())?, bytes);
            let Attribute::Code { code, .. } = &method.definition().attributes[0] else {
                panic!("missing code");
            };
            assert!(std::ptr::eq(code, method.code()));
            for (index, target) in expected.into_iter().enumerate() {
                assert_eq!(method.branch_target_index(index), target);
            }
        }
        Ok(())
    }

    #[test]
    fn test_to_string() {
        let method = Method {
            definition: MethodDefinition::Owned(Box::default()),
            access_flags: MethodAccessFlags::empty(),
            name: "test".to_string(),
            descriptor: "()V".to_string(),
            parameters: Vec::new(),
            return_type: None,
            max_stack: 1,
            max_locals: 2,
            code_attribute: None,
            instruction_offsets: vec![0],
            instruction_indices: Vec::new(),
            branch_targets: Vec::new(),
            line_numbers: Vec::new(),
            exception_table: Vec::new(),
        };
        assert_eq!("test() -> void", method.to_string());
    }
}
