use crate::attributes::Attribute;
use crate::class_access_flags::ClassAccessFlags;
use crate::constant_pool::ConstantPool;
use crate::display::indent_lines;
use crate::error::Error::{InvalidMagicNumber, VerificationError};
use crate::error::Result;
use crate::field::Field;
use crate::method::Method;
use crate::verifiers::verifier;
use crate::version::Version;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// The magic number that identifies a valid Java class file.
///
/// Every Java class file begins with this 4-byte value (`0xCAFE_BABE`) in big-endian format. This
/// signature helps JVM identify valid class files and reject invalid ones.
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
/// use std::io::Cursor;
///
/// // Read the bytes of a class file
/// let bytes = fs::read("path/to/Example.class")?;
///
/// // Parse the bytes into a ClassFile
/// let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
///
/// // Now you can inspect the class
/// println!("Class name: {}", class_file.class_name()?);
/// println!("Class version: {}", class_file.version);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
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
    /// # use std::io::Cursor;
    /// # let bytes = vec![];
    /// # let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
    /// let class_name = class_file.class_name().expect("Failed to get class name");
    /// println!("Class name: {class_name}"); // e.g., "java.lang.String"
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn class_name(&self) -> Result<&str> {
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
        match verifier::verify(self) {
            Ok(()) => Ok(()),
            Err(error) => {
                let context = self.class_name()?;
                let verification_error = VerificationError {
                    context: context.to_string(),
                    message: error.to_string(),
                };
                Err(verification_error)
            }
        }
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
    /// use std::io::Cursor;
    ///
    /// // Read the bytes of a class file
    /// let bytes = fs::read("path/to/Example.class")?;
    ///
    /// // Parse the bytes into a ClassFile
    /// let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
    ///
    /// // Now you can inspect the class
    /// println!("Class name: {}", class_file.class_name()?);
    /// println!("Class version: {}", class_file.version);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ClassFile> {
        let magic = bytes.read_u32::<BigEndian>()?;
        if magic != MAGIC {
            return Err(InvalidMagicNumber(magic));
        }

        let version = Version::from_bytes(bytes)?;
        let constant_pool = ConstantPool::from_bytes(bytes)?;
        let access_flags = ClassAccessFlags::from_bytes(bytes)?;
        let this_class = bytes.read_u16::<BigEndian>()?;
        let super_class = bytes.read_u16::<BigEndian>()?;

        let interfaces_count = bytes.read_u16::<BigEndian>()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(bytes.read_u16::<BigEndian>()?);
        }

        let field_count = bytes.read_u16::<BigEndian>()?;
        let mut fields = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            let field = Field::from_bytes(&constant_pool, bytes)?;
            fields.push(field);
        }

        let method_count = bytes.read_u16::<BigEndian>()?;
        let mut methods = Vec::with_capacity(method_count as usize);
        for _ in 0..method_count {
            let method = Method::from_bytes(&constant_pool, bytes)?;
            methods.push(method);
        }

        let attribute_count = bytes.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(&constant_pool, bytes)?;
            attributes.push(attribute);
        }

        let class_file = ClassFile {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        };
        Ok(class_file)
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
    /// use std::io::{Cursor, Write};
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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

impl fmt::Display for ClassFile {
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
    /// use std::io::Cursor;
    ///
    /// // Load a class file
    /// let bytes = fs::read("HelloWorld.class")?;
    /// let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class_name = self.class_name().map_err(|_| fmt::Error)?;
        writeln!(f, "{} {class_name}", self.access_flags.as_code())?;
        writeln!(f, "  minor version: {}", self.version.minor())?;
        writeln!(
            f,
            "  major version: {} ({})",
            self.version.major(),
            self.version
        )?;
        writeln!(f, "  flags: {}", self.access_flags)?;
        writeln!(f, "  this_class: #{}", self.this_class)?;
        writeln!(f, "  super_class: #{}", self.super_class)?;
        writeln!(
            f,
            "  interfaces: {}, fields: {}, methods: {}, attributes: {}",
            self.interfaces.len(),
            self.fields.len(),
            self.methods.len(),
            self.attributes.len()
        )?;

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
            writeln!(f, "{}", &attribute.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::{InvalidConstantPoolIndexType, IoError};
    use crate::error::Result;
    use crate::{Constant, JAVA_8, JAVA_21};
    use indoc::indoc;

    #[test]
    fn test_invalid_magic() {
        let invalid_magic: u32 = 0x0102_0304;
        let mut bytes = Cursor::new(invalid_magic.to_be_bytes().to_vec());
        assert_eq!(
            Err(InvalidMagicNumber(invalid_magic)),
            ClassFile::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_class_name() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;
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
    fn test_verify() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;
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
            Err(VerificationError {
                context: "Test".to_string(),
                message: "Invalid constant pool index 2".to_string()
            }),
            class_file.verify()
        );
        Ok(())
    }

    #[test]
    fn test_minimum_to_string() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;
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
    fn test_minimum_serialization() -> Result<()> {
        let class_bytes = include_bytes!("../../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

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
        let mut cursor = Cursor::new(bytes);
        assert_eq!(
            Err(IoError("Invalid constant pool count".to_string())),
            ClassFile::from_bytes(&mut cursor)
        );
    }
}
