use crate::Error::InvalidMethodDescriptor;
use crate::Result;
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction, LineNumber};
use ristretto_classfile::{BaseType, ClassFile, FieldType, MethodAccessFlags};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    access_flags: MethodAccessFlags,
    name: String,
    descriptor: String,
    parameters: Vec<FieldType>,
    return_type: Option<FieldType>,
    max_stack: usize,
    max_locals: usize,
    code: Vec<Instruction>,
    line_numbers: Vec<LineNumber>,
    exception_table: Vec<ExceptionTableEntry>,
}

impl Method {
    /// Create a new class method
    ///
    /// # Errors
    /// if the method descriptor cannot be parsed
    #[expect(clippy::too_many_arguments)]
    pub fn new<S: AsRef<str>>(
        access_flags: MethodAccessFlags,
        name: S,
        descriptor: S,
        max_stack: usize,
        max_locals: usize,
        code: Vec<Instruction>,
        line_numbers: Vec<LineNumber>,
        exception_table: Vec<ExceptionTableEntry>,
    ) -> Result<Self> {
        let (parameters, return_type) = Method::parse_descriptor(descriptor.as_ref())?;
        Ok(Self {
            access_flags,
            name: name.as_ref().to_string(),
            descriptor: descriptor.as_ref().to_string(),
            parameters,
            return_type,
            max_stack,
            max_locals,
            code,
            line_numbers,
            exception_table,
        })
    }

    /// Create a new class method with the given definition.
    ///
    /// # Errors
    /// if the method name cannot be read.
    pub fn from(class_file: &ClassFile, definition: &ristretto_classfile::Method) -> Result<Self> {
        let constant_pool = &class_file.constant_pool;
        let name = constant_pool.try_get_utf8(definition.name_index)?;
        let descriptor = constant_pool.try_get_utf8(definition.descriptor_index)?;
        let (max_stack, max_locals, code, line_numbers, exception_table) = match definition
            .attributes
            .iter()
            .find(|attribute| matches!(attribute, Attribute::Code { .. }))
        {
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
                    code.clone(), // TODO: avoid cloning code
                    line_numbers,
                    exception_table.clone(), // TODO: avoid cloning exception_table
                )
            }
            _ => (0, 0, Vec::new(), Vec::new(), Vec::new()),
        };

        Method::new(
            definition.access_flags,
            name.to_string(),
            descriptor.to_string(),
            max_stack,
            max_locals,
            code,
            line_numbers,
            exception_table,
        )
    }

    /// Get the method access flags.
    #[must_use]
    pub fn access_flags(&self) -> &MethodAccessFlags {
        &self.access_flags
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

    /// Get the method identifier.
    #[must_use]
    pub fn identifier(&self) -> String {
        format!("{}:{}", self.name, self.descriptor)
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
        &self.code
    }

    /// Get the line number for a given program counter.
    ///
    /// # Errors
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

    /// Parse the method descriptor. The descriptor is a string representing the method signature.
    /// The descriptor has the following format:
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.3>
    ///
    /// # Errors
    /// if the descriptor cannot be parsed
    pub fn parse_descriptor(descriptor: &str) -> Result<(Vec<FieldType>, Option<FieldType>)> {
        let mut chars = descriptor.chars().peekable();
        let mut parameters = Vec::new();
        let mut return_type = None;

        if chars.next() != Some('(') {
            return Err(InvalidMethodDescriptor(descriptor.to_string()));
        }

        while let Some(&ch) = chars.peek() {
            if ch == ')' {
                chars.next();
                break;
            }
            parameters.push(Self::parse_field_type(descriptor, &mut chars)?);
        }

        match chars.next() {
            Some('V') => {}
            Some(ch) => {
                return_type = Some(Self::parse_field_type(
                    descriptor,
                    &mut std::iter::once(ch).chain(chars),
                )?);
            }
            None => return Err(InvalidMethodDescriptor(descriptor.to_string())),
        }

        Ok((parameters, return_type))
    }

    /// Parse the field type.
    ///
    /// # Errors
    /// if the field type cannot be parsed
    fn parse_field_type<I>(descriptor: &str, chars: &mut I) -> Result<FieldType>
    where
        I: Iterator<Item = char>,
    {
        match chars.next() {
            Some('L') => {
                let mut class_name = String::new();
                for ch in chars.by_ref() {
                    if ch == ';' {
                        break;
                    }
                    class_name.push(ch);
                }
                Ok(FieldType::Object(class_name))
            }
            Some('[') => {
                let component_type = Self::parse_field_type(descriptor, chars)?;
                Ok(FieldType::Array(Box::new(component_type)))
            }
            Some(value) => {
                let base_type = BaseType::parse(value)?;
                Ok(FieldType::Base(base_type))
            }
            None => Err(InvalidMethodDescriptor(descriptor.to_string())),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(std::string::ToString::to_string)
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
        assert_eq!(method.name(), "test");
        assert_eq!(method.descriptor(), "()V");
        assert_eq!(method.identifier(), "test:()V");
        assert!(method.parameters().is_empty());
        assert_eq!(method.return_type(), None);
        assert_eq!(method.max_stack, 1);
        assert_eq!(method.max_locals, 2);
        assert!(method.code.is_empty());
        assert_eq!(method.line_number(0), 0);
        Ok(())
    }

    #[test]
    fn test_parse_descriptor() -> Result<()> {
        let (parameters, return_type) = Method::parse_descriptor("()V")?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, None);

        let (parameters, return_type) = Method::parse_descriptor("()I")?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, Some(FieldType::Base(BaseType::Int)));

        let (parameters, return_type) = Method::parse_descriptor("(I)V")?;
        assert_eq!(parameters, vec![FieldType::Base(BaseType::Int)]);
        assert_eq!(return_type, None);

        let (parameters, return_type) = Method::parse_descriptor("(Ljava.lang.String;)V")?;
        assert_eq!(
            parameters,
            vec![FieldType::Object("java.lang.String".to_string())]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) = Method::parse_descriptor("(Ljava.lang.String;I)V")?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object("java.lang.String".to_string()),
                FieldType::Base(BaseType::Int)
            ]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) = Method::parse_descriptor("(Ljava.lang.String;I)I")?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object("java.lang.String".to_string()),
                FieldType::Base(BaseType::Int)
            ]
        );
        assert_eq!(return_type, Some(FieldType::Base(BaseType::Int)));

        Ok(())
    }

    #[test]
    fn test_parse_descriptor_invalid() {
        let descriptor = String::new();
        assert!(matches!(
            Method::parse_descriptor(&descriptor),
            Err(InvalidMethodDescriptor(_))
        ));

        let descriptor = "()";
        assert!(matches!(
            Method::parse_descriptor(descriptor),
            Err(InvalidMethodDescriptor(_))
        ));
    }

    #[test]
    fn test_parse_field_type() -> Result<()> {
        assert_eq!(
            Method::parse_field_type("", &mut "I".chars())?,
            FieldType::Base(BaseType::Int)
        );
        assert_eq!(
            Method::parse_field_type("", &mut "J".chars())?,
            FieldType::Base(BaseType::Long)
        );
        assert_eq!(
            Method::parse_field_type("", &mut "S".chars())?,
            FieldType::Base(BaseType::Short)
        );
        assert_eq!(
            Method::parse_field_type("", &mut "Z".chars())?,
            FieldType::Base(BaseType::Boolean)
        );
        assert_eq!(
            Method::parse_field_type("", &mut "Ljava.lang.String;".chars())?,
            FieldType::Object("java.lang.String".to_string())
        );
        assert_eq!(
            Method::parse_field_type("", &mut "[Ljava.lang.String;".chars())?,
            FieldType::Array(Box::new(FieldType::Object("java.lang.String".to_string())))
        );
        Ok(())
    }

    #[test]
    fn test_parse_field_type_invalid() {
        let descriptor = String::new();
        assert!(matches!(
            Method::parse_field_type(&descriptor, &mut descriptor.chars()),
            Err(InvalidMethodDescriptor(_))
        ));
    }

    #[test]
    fn test_to_string() {
        let method = Method {
            access_flags: MethodAccessFlags::empty(),
            name: "test".to_string(),
            descriptor: "()V".to_string(),
            parameters: Vec::new(),
            return_type: None,
            max_stack: 1,
            max_locals: 2,
            code: Vec::new(),
            line_numbers: Vec::new(),
            exception_table: Vec::new(),
        };
        assert_eq!("test() -> void", method.to_string());
    }
}
