use crate::attributes::Attribute;
use crate::byte_reader::ByteReader;
use crate::class_access_flags::ClassAccessFlags;
use crate::constant_pool::ConstantPool;
use crate::display::indent_lines;
use crate::error::Error::InvalidMagicNumber;
use crate::error::Result;
use crate::field::Field;
use crate::java_string::JavaStr;
use crate::method::Method;
use crate::verifiers::verifier;
use crate::version::Version;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt;

/// The magic number that identifies a valid Java class file.
///
/// Every Java class file begins with this 4-byte value (`0xCAFE_BABE`) in big-endian format. This
/// signature allows the JVM to identify valid class files and reject invalid ones.
const MAGIC: u32 = 0xCAFE_BABE;

/// `ClassFile` represents the content of a Java .class file.
///
/// A class file contains the definition of a single class or interface, including:
/// - Version information
/// - Constant pool (containing various string constants, class and field references, etc.)
/// - Access flags (defining visibility and properties)
/// - This class and super class references
/// - Interfaces implemented by the class
/// - Fields and methods of the class
/// - Class attributes
///
/// # Examples
///
/// ```rust,no_run
/// use ristretto_classfile::ClassFile;
/// use std::fs;
///
/// // Read the bytes of a class file
/// let bytes = fs::read("path/to/Example.class")?;
///
/// // Parse the bytes into a ClassFile
/// let class_file = ClassFile::from_bytes(&bytes)?;
///
/// // Now you can inspect the class
/// println!("Class name: {}", class_file.class_name()?);
/// println!("Class version: {}", class_file.version);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassFile<'a> {
    pub version: Version,
    pub constant_pool: ConstantPool<'a>,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    /// The code source URL for this class (e.g., `file:/path/to/classes/`).
    /// Not part of the binary class file format.
    pub code_source_url: Option<String>,
}

impl<'a> ClassFile<'a> {
    /// Convert this `ClassFile` into an owned version with `'static` lifetime.
    #[must_use]
    pub fn into_owned(self) -> ClassFile<'static> {
        ClassFile {
            version: self.version,
            constant_pool: self.constant_pool.into_owned(),
            access_flags: self.access_flags,
            this_class: self.this_class,
            super_class: self.super_class,
            interfaces: self.interfaces,
            fields: self.fields,
            methods: self.methods,
            attributes: self.attributes,
            code_source_url: self.code_source_url,
        }
    }

    /// Get the fully qualified class name.
    ///
    /// # Errors
    ///
    /// Returns an error if the class name is not found.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use ristretto_classfile::ClassFile;
    /// # let bytes = vec![];
    /// # let class_file = ClassFile::from_bytes(&bytes)?;
    /// let class_name = class_file.class_name().expect("Failed to get class name");
    /// println!("Class name: {class_name}"); // e.g., "java.lang.String"
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn class_name(&self) -> Result<&JavaStr> {
        self.constant_pool.try_get_class(self.this_class)
    }

    /// Verify the `ClassFile`.
    ///
    /// This method checks that the class file is well-formed according to the JVM specification.
    /// It validates the constant pool entries, method definitions, and other aspects of the class.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_classfile::ClassFile;
    ///
    /// let class_file = ClassFile::default();
    ///
    /// // Verify that the class file is valid
    /// if let Err(error) = class_file.verify() {
    ///     eprintln!("Class verification failed: {error:?}");
    /// } else {
    ///     println!("Class verified successfully");
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns a `VerificationError` if the verification fails.
    pub fn verify(&self) -> Result<()> {
        verifier::verify(self).map_err(crate::Error::from)
    }

    /// Deserialize the `ClassFile` from bytes.
    ///
    /// This function reads a binary class file format and constructs a `ClassFile` instance. It
    /// follows the Java class file format specification to parse:
    /// - Magic number (`0xCAFE_BABE`)
    /// - Version information
    /// - Constant pool
    /// - Access flags
    /// - Class references
    /// - Interfaces
    /// - Fields
    /// - Methods
    /// - Attributes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The bytes do not start with the correct magic number
    /// - The class file format is invalid
    /// - There are IO errors when reading the bytes
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_classfile::ClassFile;
    /// use std::fs;
    ///
    /// // Read the bytes of a class file
    /// let bytes = fs::read("path/to/Example.class")?;
    ///
    /// // Parse the bytes into a ClassFile
    /// let class_file = ClassFile::from_bytes(&bytes)?;
    ///
    /// // Now you can inspect the class
    /// println!("Class name: {}", class_file.class_name()?);
    /// println!("Class version: {}", class_file.version);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<ClassFile<'static>> {
        let mut reader = ByteReader::new(bytes);
        Self::parse_from_reader(&mut reader)
    }

    /// Parse a `ClassFile` directly from a byte slice, borrowing string data where possible.
    ///
    /// Unlike [`from_bytes`](Self::from_bytes), which returns `ClassFile<'static>` with owned
    /// strings, this method returns `ClassFile<'a>` that borrows UTF-8 data directly from the
    /// input slice, avoiding allocation overhead.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes do not represent a valid class file.
    pub fn from_slice(data: &'a [u8]) -> Result<ClassFile<'a>> {
        let mut reader = ByteReader::new(data);
        Self::from_byte_reader(&mut reader)
    }

    /// Parse from a `ByteReader` and return a `ClassFile<'static>` by converting all
    /// borrowed strings to owned.
    fn parse_from_reader(reader: &mut ByteReader<'_>) -> Result<ClassFile<'static>> {
        let magic = reader.read_u32()?;
        if magic != MAGIC {
            return Err(InvalidMagicNumber(magic));
        }

        let version = Version::from_bytes(reader)?;
        let constant_pool = ConstantPool::from_bytes(reader)?.into_owned();
        let access_flags = ClassAccessFlags::from_bits_truncate(reader.read_u16()?);
        let this_class = reader.read_u16()?;
        let super_class = reader.read_u16()?;

        let interfaces_count = reader.read_u16()? as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(reader.read_u16()?);
        }

        let field_count = reader.read_u16()? as usize;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            let field = Field::from_bytes(&constant_pool, reader)?;
            fields.push(field);
        }

        let method_count = reader.read_u16()? as usize;
        let mut methods = Vec::with_capacity(method_count);
        for _ in 0..method_count {
            let method = Method::from_bytes(&constant_pool, reader)?;
            methods.push(method);
        }

        let attribute_count = reader.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attribute_count);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(&constant_pool, reader)?;
            attributes.push(attribute);
        }

        Ok(ClassFile {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
            code_source_url: None,
        })
    }

    /// Internal fast-path parser using `ByteReader` for zero-overhead reads.
    fn from_byte_reader(reader: &mut ByteReader<'a>) -> Result<ClassFile<'a>> {
        let magic = reader.read_u32()?;
        if magic != MAGIC {
            return Err(InvalidMagicNumber(magic));
        }

        let version = Version::from_bytes(reader)?;
        let constant_pool = ConstantPool::from_bytes(reader)?;
        let access_flags = ClassAccessFlags::from_bits_truncate(reader.read_u16()?);
        let this_class = reader.read_u16()?;
        let super_class = reader.read_u16()?;

        let interfaces_count = reader.read_u16()? as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            interfaces.push(reader.read_u16()?);
        }

        let field_count = reader.read_u16()? as usize;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            let field = Field::from_bytes(&constant_pool, reader)?;
            fields.push(field);
        }

        let method_count = reader.read_u16()? as usize;
        let mut methods = Vec::with_capacity(method_count);
        for _ in 0..method_count {
            let method = Method::from_bytes(&constant_pool, reader)?;
            methods.push(method);
        }

        let attribute_count = reader.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attribute_count);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(&constant_pool, reader)?;
            attributes.push(attribute);
        }

        Ok(ClassFile {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
            code_source_url: None,
        })
    }

    /// Serialize the `ClassFile` to bytes.
    ///
    /// This function converts a `ClassFile` instance into its binary representation according to
    /// the Java class file format specification.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_classfile::{ClassFile, ConstantPool, Version, ClassAccessFlags, JAVA_21};
    /// use std::fs;
    /// use std::io::Write;
    ///
    /// // Create a new class file
    /// let mut constant_pool = ConstantPool::default();
    /// let this_class = constant_pool.add_class("HelloWorld")?;
    /// let super_class = constant_pool.add_class("java/lang/Object")?;
    ///
    /// let class_file = ClassFile {
    ///     version: JAVA_21,
    ///     access_flags: ClassAccessFlags::PUBLIC,
    ///     constant_pool,
    ///     this_class,
    ///     super_class,
    ///     ..Default::default()
    /// };
    ///
    /// // Verify the class file is valid
    /// class_file.verify()?;
    ///
    /// // Write the class file to a vector of bytes
    /// let mut buffer = Vec::new();
    /// class_file.to_bytes(&mut buffer)?;
    ///
    /// // Now you can save these bytes to a file
    /// fs::write("HelloWorld.class", buffer)?;
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// - If there are more than 65,534 interfaces, fields, methods, or attributes
    ///   (exceeding u16 capacity)
    /// - If there are IO errors when writing to the output vector
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u32::<BigEndian>(MAGIC)?;
        self.version.to_bytes(bytes)?;
        self.constant_pool.to_bytes(bytes)?;
        self.access_flags.to_bytes(bytes)?;
        bytes.write_u16::<BigEndian>(self.this_class)?;
        bytes.write_u16::<BigEndian>(self.super_class)?;

        let interfaces_length = u16::try_from(self.interfaces.len())?;
        bytes.write_u16::<BigEndian>(interfaces_length)?;
        for interface in &self.interfaces {
            bytes.write_u16::<BigEndian>(*interface)?;
        }

        let fields_length = u16::try_from(self.fields.len())?;
        bytes.write_u16::<BigEndian>(fields_length)?;
        for field in &self.fields {
            field.to_bytes(bytes)?;
        }

        let methods_length = u16::try_from(self.methods.len())?;
        bytes.write_u16::<BigEndian>(methods_length)?;
        for method in &self.methods {
            method.to_bytes(bytes)?;
        }

        let attributes_length = u16::try_from(self.attributes.len())?;
        bytes.write_u16::<BigEndian>(attributes_length)?;
        for attribute in &self.attributes {
            attribute.to_bytes(bytes)?;
        }

        Ok(())
    }
}

impl fmt::Display for ClassFile<'_> {
    /// Implements the Display trait for `ClassFile` to provide a human-readable representation.
    ///
    /// The formatted output includes:
    /// - Class name and access flags
    /// - Version information (minor and major versions)
    /// - Flag descriptions
    /// - Class references (`this_class` and `super_class`)
    /// - Counts of interfaces, fields, methods, and attributes
    /// - Full constant pool listing
    /// - Interface references
    /// - Field definitions
    /// - Method definitions
    /// - Class attributes
    ///
    /// This format is similar to the output of the `javap -v` command.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_classfile::ClassFile;
    /// use std::fs;
    ///
    /// // Load a class file
    /// let bytes = fs::read("HelloWorld.class")?;
    /// let class_file = ClassFile::from_bytes(&bytes)?;
    ///
    /// // Print the class file in a human-readable format
    /// println!("{class_file}");
    ///
    /// // Output will look similar to:
    /// // public class HelloWorld
    /// //   minor version: 0
    /// //   major version: 65 (Java 21)
    /// //   flags: (0x0021) ACC_PUBLIC, ACC_SUPER
    /// //   this_class: #7
    /// //   super_class: #2
    /// //   interfaces: 0, fields: 0, methods: 1, attributes: 1
    /// // Constant pool:
    /// //    #1 = Methodref          #2.#3
    /// //    #2 = Class              #4
    /// //    ...
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class_name = self.class_name().map_err(|_| fmt::Error)?;
        writeln!(f, "{} {class_name}", self.access_flags.as_code())?;
        writeln!(f, "  minor version: {}", self.version.minor())?;
        let major_version = self.version.major();
        let version = &self.version;
        writeln!(f, "  major version: {major_version} ({version})")?;
        writeln!(f, "  flags: {}", self.access_flags)?;
        writeln!(f, "  this_class: #{}", self.this_class)?;
        writeln!(f, "  super_class: #{}", self.super_class)?;
        let interfaces_len = self.interfaces.len();
        let fields_len = self.fields.len();
        let methods_len = self.methods.len();
        let attributes_len = self.attributes.len();
        let summary = format!(
            "  interfaces: {interfaces_len}, fields: {fields_len}, methods: {methods_len}, attributes: {attributes_len}"
        );
        writeln!(f, "{summary}")?;

        writeln!(f, "Constant pool:")?;
        write!(f, "{}", self.constant_pool)?;
        writeln!(f, "{{")?;

        if !self.interfaces.is_empty() {
            writeln!(f, "interfaces:")?;
            for interface in &self.interfaces {
                writeln!(f, "  #{interface}")?;
            }
        }

        if !self.fields.is_empty() {
            writeln!(f, "fields:")?;
            for (index, field) in self.fields.iter().enumerate() {
                if index > 0 {
                    writeln!(f)?;
                }
                writeln!(f, "{}", indent_lines(&field.to_string(), "  "))?;
            }
        }

        writeln!(f, "methods:")?;
        for (index, method) in self.methods.iter().enumerate() {
            if index > 0 {
                writeln!(f)?;
            }
            writeln!(f, "{}", indent_lines(&method.to_string(), "  "))?;
        }

        writeln!(f, "}}")?;

        for attribute in &self.attributes {
            writeln!(f, "{attribute}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::{InvalidConstantPoolIndexType, IoError};
    use crate::error::Result;
    use crate::field_access_flags::FieldAccessFlags;
    use crate::method_access_flags::MethodAccessFlags;
    use crate::{BaseType, Constant, FieldType, JAVA_8, JAVA_21};
    use indoc::indoc;

    fn populated_class_file() -> Result<ClassFile<'static>> {
        let mut constant_pool = ConstantPool::default();
        let this_class = constant_pool.add_class("Populated")?;
        let super_class = constant_pool.add_class("java/lang/Object")?;
        let interface = constant_pool.add_class("java/io/Serializable")?;
        let field_name = constant_pool.add_utf8("value")?;
        let field_descriptor = constant_pool.add_utf8("I")?;
        let method_name = constant_pool.add_utf8("method")?;
        let method_descriptor = constant_pool.add_utf8("()V")?;
        let source_file_name = constant_pool.add_utf8("SourceFile")?;
        let source_file = constant_pool.add_utf8("Populated.java")?;

        Ok(ClassFile {
            version: JAVA_21,
            constant_pool,
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            this_class,
            super_class,
            interfaces: vec![interface],
            fields: vec![Field {
                access_flags: FieldAccessFlags::PRIVATE,
                name_index: field_name,
                descriptor_index: field_descriptor,
                field_type: FieldType::Base(BaseType::Int),
                attributes: Vec::new(),
            }],
            methods: vec![Method {
                access_flags: MethodAccessFlags::PUBLIC,
                name_index: method_name,
                descriptor_index: method_descriptor,
                attributes: Vec::new(),
            }],
            attributes: vec![Attribute::SourceFile {
                name_index: source_file_name,
                source_file_index: source_file,
            }],
            code_source_url: Some("file:/tmp/populated/".to_string()),
        })
    }

    #[test]
    fn test_invalid_magic() {
        let invalid_magic: u32 = 0x0102_0304;
        let bytes = invalid_magic.to_be_bytes();
        assert_eq!(
            Err(InvalidMagicNumber(invalid_magic)),
            ClassFile::from_bytes(&bytes)
        );
    }

    #[test]
    fn test_class_name() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let class_file = ClassFile::from_bytes(class_bytes.as_slice())?;
        assert_eq!("Minimum", class_file.class_name()?);
        Ok(())
    }

    #[test]
    fn test_class_name_invalid_constant_pool() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let utf8_index = constant_pool.add_utf8("Test")?;
        let class_file = ClassFile {
            constant_pool,
            this_class: utf8_index,
            ..Default::default()
        };
        assert_eq!(
            Err(InvalidConstantPoolIndexType(1)),
            class_file.class_name()
        );
        Ok(())
    }

    #[test]
    fn test_into_owned_preserves_all_fields() -> Result<()> {
        let class_file = populated_class_file()?;
        let owned = class_file.clone().into_owned();
        assert_eq!(class_file.version, owned.version);
        assert_eq!(class_file.constant_pool, owned.constant_pool);
        assert_eq!(class_file.access_flags, owned.access_flags);
        assert_eq!(class_file.this_class, owned.this_class);
        assert_eq!(class_file.super_class, owned.super_class);
        assert_eq!(class_file.interfaces, owned.interfaces);
        assert_eq!(class_file.fields, owned.fields);
        assert_eq!(class_file.methods, owned.methods);
        assert_eq!(class_file.attributes, owned.attributes);
        assert_eq!(class_file.code_source_url, owned.code_source_url);
        Ok(())
    }

    #[test]
    fn test_from_slice_populated_class() -> Result<()> {
        let class_file = populated_class_file()?;
        let mut bytes = Vec::new();
        class_file.to_bytes(&mut bytes)?;

        let parsed = ClassFile::from_slice(&bytes)?;
        assert_eq!(JAVA_21, parsed.version);
        assert_eq!("Populated", parsed.class_name()?);
        assert_eq!(vec![class_file.interfaces[0]], parsed.interfaces);
        assert_eq!(1, parsed.fields.len());
        assert_eq!(1, parsed.methods.len());
        assert_eq!(1, parsed.attributes.len());
        assert_eq!(None, parsed.code_source_url);
        Ok(())
    }

    #[test]
    fn test_from_bytes_populated_class() -> Result<()> {
        let class_file = populated_class_file()?;
        let mut bytes = Vec::new();
        class_file.to_bytes(&mut bytes)?;

        let parsed = ClassFile::from_bytes(&bytes)?;
        assert_eq!(class_file.version, parsed.version);
        assert_eq!(class_file.access_flags, parsed.access_flags);
        assert_eq!(class_file.this_class, parsed.this_class);
        assert_eq!(class_file.super_class, parsed.super_class);
        assert_eq!(class_file.interfaces, parsed.interfaces);
        assert_eq!(class_file.fields, parsed.fields);
        assert_eq!(class_file.methods, parsed.methods);
        assert_eq!(class_file.attributes, parsed.attributes);
        Ok(())
    }

    #[test]
    fn test_populated_class_to_string_summary() -> Result<()> {
        let output = populated_class_file()?.to_string();
        assert!(output.contains("major version: 65 (Java 21)"));
        assert!(output.contains("interfaces: 1, fields: 1, methods: 1, attributes: 1"));
        Ok(())
    }

    #[test]
    fn test_from_slice_invalid_magic() {
        let invalid_magic = 0xDEAD_BEEF_u32;
        assert_eq!(
            Err(InvalidMagicNumber(invalid_magic)),
            ClassFile::from_slice(&invalid_magic.to_be_bytes())
        );
    }

    #[test]
    fn test_verify() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let class_file = ClassFile::from_bytes(class_bytes.as_slice())?;
        assert!(class_file.verify().is_ok());
        Ok(())
    }

    #[test]
    fn test_verify_error() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let this_class = constant_pool.add_class("Test")?;
        // Add an invalid constant to trigger a verification error.
        constant_pool.push(Constant::Class(u16::MAX));
        let class_file = ClassFile {
            version: JAVA_21,
            constant_pool: constant_pool.clone(),
            this_class,
            ..Default::default()
        };

        assert_eq!(
            Err(crate::Error::VerificationError(
                crate::verifiers::error::VerifyError::InvalidConstantPoolIndex(3)
            )),
            class_file.verify()
        );
        Ok(())
    }

    #[test]
    fn test_minimum_to_string() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let class_file = ClassFile::from_bytes(class_bytes.as_slice())?;
        let expected = indoc! {r"
            public class Minimum
              minor version: 0
              major version: 52 (Java 8)
              flags: (0x0021) ACC_PUBLIC, ACC_SUPER
              this_class: #7
              super_class: #2
              interfaces: 0, fields: 0, methods: 1, attributes: 1
            Constant pool:
               #1 = Methodref          #2.#3
               #2 = Class              #4
               #3 = NameAndType        #5:#6
               #4 = Utf8               java/lang/Object
               #5 = Utf8               <init>
               #6 = Utf8               ()V
               #7 = Class              #8
               #8 = Utf8               Minimum
               #9 = Utf8               Code
              #10 = Utf8               LineNumberTable
              #11 = Utf8               SourceFile
              #12 = Utf8               Minimum.java
            {
            methods:
              flags: (0x0001) ACC_PUBLIC
              name_index: #5
              descriptor_index: #6
              attributes:
                Code:
                  stack=1, locals=1
                     0: aload_0
                     1: invokespecial #1
                     4: return
                  LineNumberTable:
                    line 1: 0
            }
            SourceFile { name_index: 11, source_file_index: 12 }
        "};

        assert_eq!(expected, class_file.to_string());
        Ok(())
    }

    #[test]
    fn test_to_string_separates_multiple_fields_and_methods() -> Result<()> {
        let mut class_file = populated_class_file()?;
        class_file.fields.push(class_file.fields[0].clone());
        class_file.methods.push(class_file.methods[0].clone());

        let output = class_file.to_string();
        assert!(output.contains("fields:\n  flags"));
        assert!(output.contains("methods:\n  flags"));
        assert!(output.contains("fields: 2, methods: 2"));
        Ok(())
    }

    #[test]
    fn test_minimum_serialization() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&expected_bytes)?;

        assert_eq!(JAVA_8, class_file.version);
        assert_eq!(
            ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            class_file.access_flags
        );
        assert_eq!("Minimum", class_file.class_name()?);

        let mut bytes = Vec::with_capacity(4);
        class_file.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, bytes);
        Ok(())
    }

    #[test]
    fn test_from_bytes_invalid() {
        let bytes = vec![
            202, 254, 186, 190, 254, 0, 0, 48, 0, 0, 160, 93, 37, 0, 212, 186,
        ];
        assert_eq!(
            Err(IoError("Invalid constant pool count".to_string())),
            ClassFile::from_bytes(&bytes)
        );
    }
}
