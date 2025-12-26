use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::ReferenceKind;
use crate::constant::Constant;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::{fmt, io};

/// Constant pool.
///
/// The constant pool is a table of structures representing various string constants, class and
/// interface names, field names, and other constants that are referred to within the `ClassFile`
/// structure and its substructures.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{Constant, ConstantPool};
///
/// let mut constant_pool = ConstantPool::new();
/// constant_pool.add_utf8("Hello, World!")?;
/// constant_pool.add_class("java/lang/Object")?;
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
///  # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstantPool {
    constants: Vec<ConstantEntry>,
}

impl ConstantPool {
    /// Create a new constant pool.
    ///
    /// Creates an empty constant pool with a placeholder entry at index 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let constant_pool = ConstantPool::new();
    /// assert_eq!(0, constant_pool.len());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        // The constant pool is 1-based, so the first entry is a placeholder.
        Self {
            constants: vec![ConstantEntry::Placeholder],
        }
    }

    /// Push a constant to the pool.
    ///
    /// Adds a constant to the pool without returning its index. For Long and Double constants, an
    /// additional placeholder entry is added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// assert_eq!(1, constant_pool.len());
    /// ```
    pub fn push(&mut self, constant: Constant) {
        let add_placeholder = matches!(constant, Constant::Long(_) | Constant::Double(_));
        self.constants.push(ConstantEntry::Constant(constant));
        if add_placeholder {
            self.constants.push(ConstantEntry::Placeholder);
        }
    }

    /// Add a constant to the pool and return the index.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add(Constant::Integer(42))?;
    /// assert_eq!(1, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn add(&mut self, constant: Constant) -> Result<u16> {
        // Logically the index is self.len() + 1.  However, since the constant pool is one based a
        // placeholder is added as the first entry, we can just use the length of the constants
        // vector to obtain the new index value.
        let index = u16::try_from(self.constants.len())?;
        self.push(constant);
        Ok(index)
    }

    /// Get a constant from the pool by index; indexes are 1-based.
    ///
    /// Returns None if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// assert_eq!(Some(&Constant::Integer(42)), constant_pool.get(1));
    /// assert_eq!(None, constant_pool.get(0)); // Index 0 is invalid
    /// assert_eq!(None, constant_pool.get(2)); // Index out of bounds
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1:~:text=The%20constant_pool%20table%20is%20indexed%20from%201%20to%20constant_pool_count%20%2D%201.>
    #[must_use]
    pub fn get(&self, index: u16) -> Option<&Constant> {
        self.try_get(index).ok()
    }

    /// Get a constant from the pool by index; indexes are 1-based.
    ///
    /// Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool, Error};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// assert!(constant_pool.try_get(1).is_ok());
    /// assert!(matches!(constant_pool.try_get(0), Err(Error::InvalidConstantPoolIndex(0))));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1:~:text=The%20constant_pool%20table%20is%20indexed%20from%201%20to%20constant_pool_count%20%2D%201.>
    pub fn try_get(&self, index: u16) -> Result<&Constant> {
        let constant_entry = self.constants.get(index as usize);
        match constant_entry {
            Some(ConstantEntry::Constant(constant)) => Ok(constant),
            _ => Err(InvalidConstantPoolIndex(index)),
        }
    }

    /// Get the number of constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// assert_eq!(0, constant_pool.len());
    /// constant_pool.push(Constant::Integer(42));
    /// assert_eq!(1, constant_pool.len());
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.constants.len() - 1
    }

    /// Check if the pool is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// assert!(constant_pool.is_empty());
    /// constant_pool.push(Constant::Integer(42));
    /// assert!(!constant_pool.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get an iterator over the constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// constant_pool.push(Constant::Utf8("Hello".to_string()));
    ///
    /// let mut iterator = constant_pool.iter();
    /// assert_eq!(Some(&Constant::Integer(42)), iterator.next());
    /// assert_eq!(Some(&Constant::Utf8("Hello".to_string())), iterator.next());
    /// assert_eq!(None, iterator.next());
    /// ```
    #[must_use]
    pub fn iter(&self) -> ConstantPoolIterator<'_> {
        ConstantPoolIterator::new(self)
    }

    /// Deserialize the `ConstantPool` from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not a valid constant pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    /// use std::io::Cursor;
    ///
    /// let bytes = vec![0, 2, 3, 0, 0, 0, 42]; // constant_pool_count=2, Integer=42
    /// let mut cursor = Cursor::new(bytes);
    /// let constant_pool = ConstantPool::from_bytes(&mut cursor)?;
    /// assert_eq!(1, constant_pool.len());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ConstantPool> {
        let mut constant_pool = ConstantPool::default();
        let constant_pool_count =
            bytes
                .read_u16::<BigEndian>()?
                .checked_sub(1)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid constant pool count")
                })?;
        while constant_pool.len() < constant_pool_count as usize {
            let constant = Constant::from_bytes(bytes)?;
            constant_pool.push(constant);
        }

        Ok(constant_pool)
    }

    /// Serialize the `ConstantPool` to bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    ///
    /// let mut bytes = Vec::new();
    /// constant_pool.to_bytes(&mut bytes)?;
    /// assert_eq!(&[0, 2, 3, 0, 0, 0, 42], bytes.as_slice()); // constant_pool_count=2, Integer=42
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        let constant_pool_count = u16::try_from(self.len())? + 1;
        bytes.write_u16::<BigEndian>(constant_pool_count)?;
        for constant_entry in &self.constants {
            if let ConstantEntry::Constant(constant) = constant_entry {
                constant.to_bytes(bytes)?;
            }
        }
        Ok(())
    }

    /// Add a UTF-8 constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_utf8("Hello")?;
    /// assert_eq!(1, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7>
    pub fn add_utf8<S: AsRef<str>>(&mut self, value: S) -> Result<u16> {
        let value = value.as_ref().to_string();
        self.add(Constant::Utf8(value))
    }

    /// Get a UTF-8 constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Utf8("Hello".to_string()));
    /// assert_eq!("Hello", constant_pool.try_get_utf8(1)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a UTF-8 constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7>
    pub fn try_get_utf8(&self, index: u16) -> Result<&str> {
        match self.try_get(index)? {
            Constant::Utf8(value) => Ok(value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add an integer constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_integer(42)?;
    /// assert_eq!(1, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    pub fn add_integer(&mut self, value: i32) -> Result<u16> {
        self.add(Constant::Integer(value))
    }

    /// Get an integer constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// assert_eq!(&42, constant_pool.try_get_integer(1)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not an integer constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    pub fn try_get_integer(&self, index: u16) -> Result<&i32> {
        match self.try_get(index)? {
            Constant::Integer(value) => Ok(value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a float constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_float(3.14)?;
    /// assert_eq!(1, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    pub fn add_float(&mut self, value: f32) -> Result<u16> {
        self.add(Constant::Float(value))
    }

    /// Get a float constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Float(3.14));
    /// assert_eq!(&3.14, constant_pool.try_get_float(1)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a float constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    pub fn try_get_float(&self, index: u16) -> Result<&f32> {
        match self.try_get(index)? {
            Constant::Float(value) => Ok(value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a long constant to the pool.
    ///
    /// Note: Long constants take up two entries in the constant pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_long(42i64)?;
    /// assert_eq!(1, index);
    /// assert_eq!(2, constant_pool.len()); // Long takes two slots
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    pub fn add_long(&mut self, value: i64) -> Result<u16> {
        self.add(Constant::Long(value))
    }

    /// Get a long constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Long(42i64));
    /// assert_eq!(&42i64, constant_pool.try_get_long(1)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a long constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    pub fn try_get_long(&self, index: u16) -> Result<&i64> {
        match self.try_get(index)? {
            Constant::Long(value) => Ok(value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a double constant to the pool.
    ///
    /// Note: Double constants take up two entries in the constant pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_double(3.14)?;
    /// assert_eq!(1, index);
    /// assert_eq!(2, constant_pool.len()); // Double takes two slots
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    pub fn add_double(&mut self, value: f64) -> Result<u16> {
        self.add(Constant::Double(value))
    }

    /// Get a double constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Double(3.14));
    /// assert_eq!(&3.14, constant_pool.try_get_double(1)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a double constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    pub fn try_get_double(&self, index: u16) -> Result<&f64> {
        match self.try_get(index)? {
            Constant::Double(value) => Ok(value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a class constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_class("java/lang/Object")?;
    /// assert_eq!(2, index); // Index 1 is the UTF-8 constant, 2 is the class
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.1>
    pub fn add_class<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Class(utf8_index))
    }

    /// Get a class constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/Object")?;
    /// assert_eq!("java/lang/Object", constant_pool.try_get_class(class_index)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a class constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.1>
    pub fn try_get_class(&self, index: u16) -> Result<&str> {
        match self.try_get(index)? {
            Constant::Class(utf8_index) => self.try_get_utf8(*utf8_index),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a string constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_string("Hello, World!")?;
    /// assert_eq!(2, index); // Index 1 is the UTF-8 constant, 2 is the string
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.3>
    pub fn add_string<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::String(utf8_index))
    }

    /// Get a string constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let string_index = constant_pool.add_string("Hello, World!")?;
    /// assert_eq!("Hello, World!", constant_pool.try_get_string(string_index)?);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a string constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.3>
    pub fn try_get_string(&self, index: u16) -> Result<&str> {
        match self.try_get(index)? {
            Constant::String(value) => self.try_get_utf8(*value),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a field reference constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/System")?;
    /// let index = constant_pool.add_field_ref(class_index, "out", "Ljava/io/PrintStream;")?;
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn add_field_ref<S: AsRef<str>>(
        &mut self,
        class_index: u16,
        name: S,
        descriptor: S,
    ) -> Result<u16> {
        let name_and_type_index = self.add_name_and_type(name, descriptor)?;
        self.add(Constant::FieldRef {
            class_index,
            name_and_type_index,
        })
    }

    /// Get a field constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::FieldRef { class_index: 1, name_and_type_index: 2 });
    /// let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(1)?;
    /// assert_eq!(&1u16, class_index);
    /// assert_eq!(&2u16, name_and_type_index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a field constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn try_get_field_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::FieldRef {
                class_index,
                name_and_type_index,
            } => Ok((class_index, name_and_type_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method reference constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/io/PrintStream")?;
    /// let index = constant_pool.add_method_ref(class_index, "println", "(Ljava/lang/String;)V")?;
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn add_method_ref<S: AsRef<str>>(
        &mut self,
        class_index: u16,
        name: S,
        descriptor: S,
    ) -> Result<u16> {
        let name_and_type_index = self.add_name_and_type(name, descriptor)?;
        self.add(Constant::MethodRef {
            class_index,
            name_and_type_index,
        })
    }

    /// Get a method constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::MethodRef { class_index: 1, name_and_type_index: 2 });
    /// let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(1)?;
    /// assert_eq!(&1u16, class_index);
    /// assert_eq!(&2u16, name_and_type_index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a method constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn try_get_method_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::MethodRef {
                class_index,
                name_and_type_index,
            } => Ok((class_index, name_and_type_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add an interface method reference constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/Comparable")?;
    /// let index = constant_pool.add_interface_method_ref(class_index, "compareTo", "(Ljava/lang/Object;)I")?;
    /// assert_eq!(6, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn add_interface_method_ref<S: AsRef<str>>(
        &mut self,
        class_index: u16,
        name: S,
        descriptor: S,
    ) -> Result<u16> {
        let name_and_type_index = self.add_name_and_type(name, descriptor)?;
        self.add(Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        })
    }

    /// Get an interface method constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/Comparable")?;
    /// let index = constant_pool.add_interface_method_ref(class_index, "compareTo", "(Ljava/lang/Object;)I")?;
    /// let (class_idx, name_and_type_idx) = constant_pool.try_get_interface_method_ref(index)?;
    /// assert_eq!(&class_index, class_idx);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not an interface method
    /// constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    pub fn try_get_interface_method_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => Ok((class_index, name_and_type_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a name and type constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_name_and_type("field", "I")?;
    /// assert_eq!(3, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.6>
    pub fn add_name_and_type<S: AsRef<str>>(&mut self, name: S, descriptor: S) -> Result<u16> {
        let name_index = self.add_utf8(name)?;
        let descriptor_index = self.add_utf8(descriptor)?;
        self.add(Constant::NameAndType {
            name_index,
            descriptor_index,
        })
    }

    /// Get a name and type constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_name_and_type("field", "I")?;
    /// let (name_idx, descriptor_idx) = constant_pool.try_get_name_and_type(index)?;
    /// let name = constant_pool.try_get_utf8(*name_idx)?;
    /// assert_eq!("field", name);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a name and type
    /// constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.6>
    pub fn try_get_name_and_type(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => Ok((name_index, descriptor_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method handle constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{ConstantPool, ReferenceKind};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/System")?;
    /// let field_index = constant_pool.add_field_ref(class_index, "out", "Ljava/io/PrintStream;")?;
    /// let index = constant_pool.add_method_handle(ReferenceKind::GetStatic, field_index)?;
    /// assert_eq!(7, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8>
    pub fn add_method_handle(
        &mut self,
        reference_kind: ReferenceKind,
        reference_index: u16,
    ) -> Result<u16> {
        self.add(Constant::MethodHandle {
            reference_kind,
            reference_index,
        })
    }

    /// Get a method handle constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool, ReferenceKind};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/System")?;
    /// let field_index = constant_pool.add_field_ref(class_index, "out", "Ljava/io/PrintStream;")?;
    /// let index = constant_pool.add_method_handle(ReferenceKind::GetStatic, field_index)?;
    /// let (reference_kind, ref_index) = constant_pool.try_get_method_handle(index)?;
    /// assert_eq!(&ReferenceKind::GetStatic, reference_kind);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a method handle
    /// constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8>
    pub fn try_get_method_handle(&self, index: u16) -> Result<(&ReferenceKind, &u16)> {
        match self.try_get(index)? {
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => Ok((reference_kind, reference_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method type constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_method_type("(Ljava/lang/String;)V")?;
    /// assert_eq!(2, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.9>
    pub fn add_method_type<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::MethodType(utf8_index))
    }

    /// Get a method type constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_method_type("(Ljava/lang/String;)V")?;
    /// let descriptor_idx = constant_pool.try_get_method_type(index)?;
    /// let descriptor = constant_pool.try_get_utf8(*descriptor_idx)?;
    /// assert_eq!("(Ljava/lang/String;)V", descriptor);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a method type
    /// constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.9>
    pub fn try_get_method_type(&self, index: u16) -> Result<&u16> {
        match self.try_get(index)? {
            Constant::MethodType(name_and_type_index) => Ok(name_and_type_index),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a dynamic constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let bootstrap_method_attr_index = 1; // Would reference an actual bootstrap method in real usage
    /// let index = constant_pool.add_dynamic(bootstrap_method_attr_index, "value", "I")?;
    /// assert_eq!(4, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    pub fn add_dynamic<S: AsRef<str>>(
        &mut self,
        bootstrap_method_attr_index: u16,
        name: S,
        descriptor: S,
    ) -> Result<u16> {
        let name_and_type_index = self.add_name_and_type(name, descriptor)?;
        self.add(Constant::Dynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }

    /// Get a dynamic constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let bootstrap_method_attr_index = 1; // Would reference an actual bootstrap method in real usage
    /// let index = constant_pool.add_dynamic(bootstrap_method_attr_index, "value", "I")?;
    /// let (bootstrap_idx, name_and_type_idx) = constant_pool.try_get_dynamic(index)?;
    /// assert_eq!(&bootstrap_method_attr_index, bootstrap_idx);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a dynamic constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    pub fn try_get_dynamic(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => Ok((bootstrap_method_attr_index, name_and_type_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a invoke dynamic constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let bootstrap_method_attr_index = 1; // Would reference an actual bootstrap method in real usage
    /// let index = constant_pool.add_invoke_dynamic(bootstrap_method_attr_index, "apply", "()Ljava/util/function/Function;")?;
    /// assert_eq!(4, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    pub fn add_invoke_dynamic<S: AsRef<str>>(
        &mut self,
        bootstrap_method_attr_index: u16,
        name: S,
        descriptor: S,
    ) -> Result<u16> {
        let name_and_type_index = self.add_name_and_type(name, descriptor)?;
        self.add(Constant::InvokeDynamic {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }

    /// Get an invoke dynamic constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let bootstrap_method_attr_index = 1; // Would reference an actual bootstrap method in real usage
    /// let index = constant_pool.add_invoke_dynamic(bootstrap_method_attr_index, "apply", "()Ljava/util/function/Function;")?;
    /// let (bootstrap_idx, name_and_type_idx) = constant_pool.try_get_invoke_dynamic(index)?;
    /// assert_eq!(&bootstrap_method_attr_index, bootstrap_idx);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not an invoke
    /// dynamic constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    pub fn try_get_invoke_dynamic(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index)? {
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => Ok((bootstrap_method_attr_index, name_and_type_index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a module constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_module("java.base")?;
    /// assert_eq!(2, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.11>
    pub fn add_module<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Module(utf8_index))
    }

    /// Get a module constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_module("java.base")?;
    /// let module_name = constant_pool.try_get_module(index)?;
    /// assert_eq!("java.base", module_name);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a module constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.11>
    pub fn try_get_module(&self, index: u16) -> Result<&str> {
        match self.try_get(index)? {
            Constant::Module(name_index) => self.try_get_utf8(*name_index),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a package constant to the pool.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 constants in the pool.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_package("java/lang")?;
    /// assert_eq!(2, index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.12>
    pub fn add_package<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Package(utf8_index))
    }

    /// Get a package constant from the pool by index; indexes are 1-based.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let index = constant_pool.add_package("java/lang")?;
    /// let package_name = constant_pool.try_get_package(index)?;
    /// assert_eq!("java/lang", package_name);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or the constant is not a package constant.
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.12>
    pub fn try_get_package(&self, index: u16) -> Result<&str> {
        match self.try_get(index)? {
            Constant::Package(name_index) => self.try_get_utf8(*name_index),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Get a formatted string constant from the pool by index; indexes are 1-based.
    ///
    /// Returns a human-readable string representation of the constant at the given index.
    /// This is useful for debugging and displaying constant pool entries.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// let class_index = constant_pool.add_class("java/lang/Object")?;
    /// let formatted = constant_pool.try_get_formatted_string(class_index)?;
    /// assert_eq!("Class java/lang/Object", formatted);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn try_get_formatted_string(&self, index: u16) -> Result<String> {
        let value = match self.try_get(index)? {
            Constant::Utf8(value) => value.clone(),
            Constant::Integer(integer) => format!("{integer}"),
            Constant::Float(float) => format!("{float}"),
            Constant::Long(long) => format!("{long}"),
            Constant::Double(double) => format!("{double}"),
            Constant::Class(utf8_index) => {
                format!("Class {}", self.try_get_utf8(*utf8_index)?)
            }
            Constant::String(utf8_index) => {
                format!("String {}", self.try_get_utf8(*utf8_index)?)
            }
            Constant::FieldRef {
                class_index,
                name_and_type_index,
            } => {
                let (name_index, _descriptor_index) =
                    self.try_get_name_and_type(*name_and_type_index)?;
                let class_name = self.try_get_class(*class_index)?;
                let field_name = self.try_get_utf8(*name_index)?;
                format!("Field {class_name}.{field_name}")
            }
            Constant::MethodRef {
                class_index,
                name_and_type_index,
            } => {
                let class_name = self.try_get_class(*class_index)?;
                let (name_index, descriptor_index) =
                    self.try_get_name_and_type(*name_and_type_index)?;
                let method_name = self.try_get_utf8(*name_index)?;
                let method_descriptor = self.try_get_utf8(*descriptor_index)?;
                format!("Method {class_name}.{method_name}{method_descriptor}")
            }
            Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => {
                let class_name = self.try_get_class(*class_index)?;
                let (name_index, descriptor_index) =
                    self.try_get_name_and_type(*name_and_type_index)?;
                let method_name = self.try_get_utf8(*name_index)?;
                let method_descriptor = self.try_get_utf8(*descriptor_index)?;
                format!("Interface method {class_name}.{method_name}{method_descriptor}")
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                let name = self.try_get_utf8(*name_index)?;
                let descriptor = self.try_get_utf8(*descriptor_index)?;
                format!("Name {name}, Descriptor {descriptor}")
            }
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                let reference = self.try_get_formatted_string(*reference_index)?;
                format!("Method handle {reference_kind} {reference}")
            }
            Constant::MethodType(utf8_index) => {
                let method_descriptor = self.try_get_utf8(*utf8_index)?;
                format!("Method type {method_descriptor}")
            }
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                let (name_index, descriptor_index) =
                    self.try_get_name_and_type(*name_and_type_index)?;
                let name = self.try_get_utf8(*name_index)?;
                let descriptor = self.try_get_utf8(*descriptor_index)?;
                format!("Dynamic #{bootstrap_method_attr_index}:{name}:{descriptor}")
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                let (name_index, descriptor_index) =
                    self.try_get_name_and_type(*name_and_type_index)?;
                let name = self.try_get_utf8(*name_index)?;
                let descriptor = self.try_get_utf8(*descriptor_index)?;
                format!("InvokeDynamic #{bootstrap_method_attr_index}:{name}:{descriptor}")
            }
            Constant::Module(name_index) => {
                format!("Module {}", self.try_get_utf8(*name_index)?)
            }
            Constant::Package(name_index) => {
                format!("Package {}", self.try_get_utf8(*name_index)?)
            }
        };
        Ok(value)
    }
}

impl Default for ConstantPool {
    /// Create a new empty constant pool.
    ///
    /// This is equivalent to calling `ConstantPool::new()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ConstantPool;
    /// use std::default::Default;
    ///
    /// let constant_pool = ConstantPool::default();
    /// assert_eq!(0, constant_pool.len());
    /// assert!(constant_pool.is_empty());
    ///
    /// // Using Default trait
    /// let constant_pool: ConstantPool = Default::default();
    /// assert_eq!(0, constant_pool.len());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// Entry in the constant pool.
///
/// The constant pool uses a 1-based indexing scheme, where valid indices are in the range `[1, constant_pool_count-1]`.
/// Additionally, certain constants (long and double) occupy two slots in the constant pool.
///
/// This enum represents either an actual constant or a placeholder entry used for:
/// 1. The 0 index position (since constant pool is 1-based)
/// 2. The slot following a long or double constant
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{Constant, ConstantPool};
///
/// let mut constant_pool = ConstantPool::new();
/// constant_pool.push(Constant::Long(42)); // Adds a constant entry and a placeholder entry
/// assert_eq!(2, constant_pool.len()); // Long takes two slots
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
#[derive(Clone, Debug, Eq, PartialEq)]
enum ConstantEntry {
    /// An actual constant in the pool.
    Constant(Constant),

    /// A placeholder entry in the constant pool.
    ///
    /// Used for two purposes:
    /// 1. The constant pool is one-based; a placeholder is added at position 0 to facilitate
    ///    one-based indexing without needing to calculate an offset.
    /// 2. The JVM specification requires 8-byte constants (double and long) to take two consecutive
    ///    positions in the constant pool. This implementation uses the placeholder as the second
    ///    position for these 8-byte constants.
    Placeholder,
}

impl fmt::Display for ConstantEntry {
    /// Implements the `Display` trait for `ConstantEntry` to provide a string representation.
    ///
    /// This implementation formats a `ConstantEntry` as follows:
    /// - For a `Constant` entry, delegates to the `Display` implementation of the inner `Constant`
    /// - For a `Placeholder` entry, produces an empty string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    /// use std::fmt::Write;
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.push(Constant::Integer(42));
    /// constant_pool.push(Constant::Long(123_456_789));
    ///
    /// for entry in constant_pool.iter() {
    ///     println!("{entry}");
    /// }
    ///
    /// // The output will look like:
    /// // Integer 42
    /// // Long 123456789
    /// # Ok::<(), std::fmt::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4>
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantEntry::Constant(constant) => write!(f, "{constant}"),
            ConstantEntry::Placeholder => Ok(()),
        }
    }
}

/// Iterator over constants in a constant pool.
///
/// This struct provides a convenient way to iterate over all constants in a constant pool, skipping
/// placeholder entries automatically.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{Constant, ConstantPool};
///
/// let mut constant_pool = ConstantPool::new();
/// constant_pool.add_utf8("foo")?;
/// constant_pool.add_long(42)?; // Long constants take two slots
/// constant_pool.add_integer(3)?;
///
/// // Using the iterator directly
/// let mut iterator = constant_pool.iter();
/// assert_eq!(Some(&Constant::Utf8("foo".to_string())), iterator.next());
/// assert_eq!(Some(&Constant::Long(42)), iterator.next());
/// assert_eq!(Some(&Constant::Integer(3)), iterator.next());
/// assert_eq!(None, iterator.next());
///
/// // Or with a for loop
/// for constant in constant_pool.iter() {
///     println!("{constant}");
/// }
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # Implementation Details
///
/// The iterator automatically skips the placeholder entry at index 0 and any placeholder
/// entries following long and double constants.
#[derive(Debug)]
pub struct ConstantPoolIterator<'a> {
    constant_pool: &'a ConstantPool,
    index: usize,
}

impl<'a> ConstantPoolIterator<'a> {
    /// Creates a new iterator over constants in a constant pool.
    ///
    /// This constructor creates an iterator that automatically skips placeholder entries in the
    /// constant pool, including the initial placeholder at index 0 and any placeholders following
    /// long and double constants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.add_utf8("foo")?;
    /// constant_pool.add_long(42)?; // Takes two slots
    /// constant_pool.add_integer(3)?;
    ///
    /// // Create an iterator using new
    /// let mut itererator = constant_pool.iter();
    ///
    /// // The iterator will automatically skip placeholder entries
    /// assert_eq!(Some(&Constant::Utf8("foo".to_string())), itererator.next());
    /// assert_eq!(Some(&Constant::Long(42)), itererator.next());
    /// assert_eq!(Some(&Constant::Integer(3)), itererator.next());
    /// assert_eq!(None, itererator.next());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Notes
    ///
    /// The iterator begins at index 1 rather than 0, as the constant pool uses 1-based indexing
    /// with a placeholder at index 0.
    pub fn new(constant_pool: &'a ConstantPool) -> Self {
        // index is 1-based; skip the first entry, which is a placeholder
        Self {
            constant_pool,
            index: 1,
        }
    }
}

impl<'a> Iterator for ConstantPoolIterator<'a> {
    type Item = &'a Constant;

    /// Returns the next constant in the iteration.
    ///
    /// This method advances the iterator and returns the next constant from the constant pool,
    /// automatically skipping any placeholder entries. It returns `None` when there are no more
    /// constants to iterate over.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.add_utf8("foo")?;
    /// constant_pool.add_long(42)?; // Long constants take two slots
    /// constant_pool.add_integer(3)?;
    ///
    /// let mut iterator = constant_pool.iter();
    ///
    /// // The first constant is the UTF-8 "Hello"
    /// assert_eq!(Some(&Constant::Utf8("foo".to_string())), iterator.next());
    ///
    /// // The second constant is the Long value (the placeholder is skipped)
    /// assert_eq!(Some(&Constant::Long(42)), iterator.next());
    ///
    /// // The third constant is the Integer
    /// assert_eq!(Some(&Constant::Integer(3)), iterator.next());
    ///
    /// // No more constants
    /// assert_eq!(None, iterator.next());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// - Placeholder entries (at index 0 and after Long/Double constants) are automatically skipped
    /// - The iterator advances through the constant pool until it either finds a constant or
    ///   reaches the end of the pool
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.constant_pool.constants.len() {
            match &self.constant_pool.constants[self.index] {
                ConstantEntry::Constant(constant) => {
                    self.index += 1;
                    return Some(constant);
                }
                ConstantEntry::Placeholder => {
                    self.index += 1;
                }
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a ConstantPool {
    type Item = &'a Constant;
    type IntoIter = ConstantPoolIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl fmt::Display for ConstantPool {
    /// Formats the `ConstantPool` for display.
    ///
    /// Produces a human-readable string representation of the constant pool, displaying each
    /// constant with its index and formatted value. The output is formatted as a table with the
    /// following columns:
    /// - Index number (prefixed with #)
    /// - Constant type
    /// - Constant value
    ///
    /// Placeholder entries are skipped in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ConstantPool};
    ///
    /// let mut constant_pool = ConstantPool::new();
    /// constant_pool.add_utf8("Hello, World!")?;
    /// constant_pool.add_class("java/lang/Object")?;
    /// constant_pool.add_string("test")?;
    ///
    /// // When printed, the output looks like:
    /// //    #1 = Utf8             Hello, World!
    /// //    #2 = Utf8             java/lang/Object
    /// //    #3 = Class            java/lang/Object
    /// //    #4 = Utf8             test
    /// //    #5 = String           test
    /// println!("{}", constant_pool);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // skip the first entry, which is a placeholder
        for (index, constant_entry) in self.constants.iter().skip(1).enumerate() {
            match constant_entry {
                ConstantEntry::Constant(constant) => {
                    let index = index + 1;
                    let value = constant.to_string();
                    let (name, value) = value.split_once(' ').unwrap_or_default();
                    writeln!(f, "{:>5} = {name:<18} {value}", format!("#{index}"))?;
                }
                ConstantEntry::Placeholder => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::IoError;
    use crate::constant::Constant;
    use std::fmt::Debug;

    #[test]
    fn test_constant_pool_entry_to_string() {
        assert_eq!(
            "Integer 42",
            ConstantEntry::Constant(Constant::Integer(42)).to_string()
        );
        assert_eq!("", ConstantEntry::Placeholder.to_string());
    }

    #[test]
    fn test_get_zero_none() {
        let constant_pool = ConstantPool::default();
        assert!(constant_pool.get(0).is_none());
    }

    #[test]
    fn test_get() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.get(1).is_none());
        constant_pool.push(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.get(1).is_some());
    }

    #[test]
    fn test_try_get_zero_error() {
        let constant_pool = ConstantPool::default();
        assert_eq!(Err(InvalidConstantPoolIndex(0)), constant_pool.try_get(0));
    }

    #[test]
    fn test_try_get() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.try_get(1).is_err());
        constant_pool.push(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.try_get(1).is_ok());
    }

    #[test]
    fn test_utf8() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.get(1).is_some());
        assert_eq!(1, constant_pool.len());
    }

    #[test]
    fn test_integer() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Integer(42));
        assert!(constant_pool.get(1).is_some());
        assert_eq!(1, constant_pool.len());
    }

    #[test]
    fn test_long() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Long(1_234_567_890));
        assert!(constant_pool.get(1).is_some());
        assert!(constant_pool.get(2).is_none());
        assert_eq!(2, constant_pool.len());
    }

    #[test]
    fn test_len() {
        let mut constant_pool = ConstantPool::default();
        assert_eq!(0, constant_pool.len());
        constant_pool.push(Constant::Integer(42));
        assert_eq!(1, constant_pool.len());
    }

    #[test]
    fn test_is_empty() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.is_empty());
        constant_pool.push(Constant::Integer(42));
        assert!(!constant_pool.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("foo".to_string()));
        constant_pool.push(Constant::Integer(42));
        constant_pool.push(Constant::Long(1_234_567_890));
        let mut iter = constant_pool.iter();
        assert_eq!(Some(&Constant::Utf8("foo".to_string())), iter.next());
        assert_eq!(Some(&Constant::Integer(42)), iter.next());
        assert_eq!(Some(&Constant::Long(1_234_567_890)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_into_iter() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("foo".to_string()));
        for constant in &constant_pool {
            assert_eq!(Constant::Utf8("foo".to_string()), *constant);
        }
    }

    #[test]
    fn test_double() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Double(std::f64::consts::PI));
        assert!(constant_pool.get(1).is_some());
        assert!(constant_pool.get(2).is_none());
        assert_eq!(2, constant_pool.len());
    }

    #[test]
    fn test_to_string() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("foo".to_string()));
        constant_pool.push(Constant::Integer(42));
        constant_pool.push(Constant::Long(1_234_567_890));
        let expected = "   #1 = Utf8               foo\n   #2 = Integer            42\n   #3 = Long               1234567890\n";
        assert_eq!(expected, constant_pool.to_string());
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let integer_constant = Constant::Integer(42);
        let long_constant = Constant::Long(1_234_567_890);

        constant_pool.push(integer_constant.clone());
        constant_pool.push(long_constant.clone());

        let expected_bytes = [0, 4, 3, 0, 0, 0, 42, 5, 0, 0, 0, 0, 73, 150, 2, 210];
        let mut bytes = Vec::new();
        constant_pool.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(constant_pool, ConstantPool::from_bytes(&mut bytes)?);
        Ok(())
    }

    fn test_try_get_constant<T>(f: fn(&ConstantPool, u16) -> Result<&T>, constant: Constant)
    where
        T: Debug + PartialEq + ?Sized,
    {
        let mut constant_pool = ConstantPool::default();
        if matches!(constant, Constant::Utf8(_)) {
            constant_pool.push(Constant::Integer(42));
        } else {
            constant_pool.push(Constant::Utf8("foo".to_string()));
        }
        constant_pool.push(constant);
        assert_eq!(Err(InvalidConstantPoolIndex(0)), f(&constant_pool, 0));
        assert_eq!(Err(InvalidConstantPoolIndexType(1)), f(&constant_pool, 1));
        assert!(f(&constant_pool, 2).is_ok());
    }

    fn test_try_get_constant_tuple<A, B>(
        f: fn(&ConstantPool, u16) -> Result<(&A, &B)>,
        constant: Constant,
    ) where
        A: Debug + PartialEq,
        B: Debug + PartialEq,
    {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("foo".to_string()));
        constant_pool.push(constant);
        assert_eq!(Err(InvalidConstantPoolIndex(0)), f(&constant_pool, 0));
        assert_eq!(Err(InvalidConstantPoolIndexType(1)), f(&constant_pool, 1));
        assert!(f(&constant_pool, 2).is_ok());
    }

    #[test]
    fn test_add_utf8() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_utf8("foo")?;
        assert_eq!(1, index);
        assert_eq!(
            Some(&Constant::Utf8("foo".to_string())),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_utf8() {
        test_try_get_constant(
            ConstantPool::try_get_utf8,
            Constant::Utf8("foo".to_string()),
        );
    }

    #[test]
    fn test_try_get_formatted_string_utf8() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_utf8("foo")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("foo", value);
        Ok(())
    }

    #[test]
    fn test_add_integer() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_integer(42)?;
        assert_eq!(1, index);
        assert_eq!(Some(&Constant::Integer(42)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_integer() {
        test_try_get_constant(ConstantPool::try_get_integer, Constant::Integer(42));
    }

    #[test]
    fn test_try_get_formatted_string_integer() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_integer(42)?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("42", value);
        Ok(())
    }

    #[test]
    fn test_add_float() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_float(std::f32::consts::PI)?;
        assert_eq!(1, index);
        assert_eq!(
            Some(&Constant::Float(std::f32::consts::PI)),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_float() {
        test_try_get_constant(
            ConstantPool::try_get_float,
            Constant::Float(std::f32::consts::PI),
        );
    }

    #[test]
    fn test_try_get_formatted_string_float() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_float(std::f32::consts::PI)?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("3.1415927", value);
        Ok(())
    }

    #[test]
    fn test_add_long() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_long(i64::MAX)?;
        assert_eq!(1, index);
        assert_eq!(Some(&Constant::Long(i64::MAX)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_long() {
        test_try_get_constant(ConstantPool::try_get_long, Constant::Long(i64::MAX));
    }

    #[test]
    fn test_try_get_formatted_string_long() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_long(42)?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("42", value);
        Ok(())
    }

    #[test]
    fn test_add_double() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_double(std::f64::consts::PI)?;
        assert_eq!(1, index);
        assert_eq!(
            Some(&Constant::Double(std::f64::consts::PI)),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_double() {
        test_try_get_constant(
            ConstantPool::try_get_double,
            Constant::Double(std::f64::consts::PI),
        );
    }

    #[test]
    fn test_try_get_formatted_string_double() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_double(std::f64::consts::PI)?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("3.141592653589793", value);
        Ok(())
    }

    #[test]
    fn test_add_class() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_class("java/lang/Object")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::Class(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_class() {
        test_try_get_constant(ConstantPool::try_get_class, Constant::Class(1));
    }

    #[test]
    fn test_try_get_formatted_string_class() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_class("Foo")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Class Foo", value);
        Ok(())
    }

    #[test]
    fn test_try_get_class_name() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.push(Constant::Utf8("java/lang/Object".to_string()));
        constant_pool.push(Constant::Class(1));
        let class_name = constant_pool.try_get_class(2)?;
        assert_eq!("java/lang/Object", class_name);
        Ok(())
    }

    #[test]
    fn test_add_string() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_string("foo")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::String(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_string() {
        test_try_get_constant(ConstantPool::try_get_string, Constant::String(1));
    }

    #[test]
    fn test_try_get_formatted_string_string() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_string("foo")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("String foo", value);
        Ok(())
    }

    #[test]
    fn test_add_field_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_field_ref(1, "out", "Ljava/io/PrintStream;")?;
        assert_eq!(4, index);
        assert_eq!(
            Some(&Constant::FieldRef {
                class_index: 1,
                name_and_type_index: 3
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_field_ref() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_field_ref,
            Constant::FieldRef {
                class_index: 1,
                name_and_type_index: 3,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_field_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let index = constant_pool.add_field_ref(class_index, "x", "I")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Field Foo.x", value);
        Ok(())
    }

    #[test]
    fn test_add_method_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_method_ref(1, "println", "(Ljava/lang/String;)V")?;
        assert_eq!(4, index);
        assert_eq!(
            Some(&Constant::MethodRef {
                class_index: 1,
                name_and_type_index: 3
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_method_ref() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_method_ref,
            Constant::MethodRef {
                class_index: 1,
                name_and_type_index: 3,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_method_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let index = constant_pool.add_method_ref(class_index, "x", "()V")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Method Foo.x()V", value);
        Ok(())
    }

    #[test]
    fn test_add_interface_method_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index =
            constant_pool.add_interface_method_ref(1, "println", "(Ljava/lang/String;)V")?;
        assert_eq!(4, index);
        assert_eq!(
            Some(&Constant::InterfaceMethodRef {
                class_index: 1,
                name_and_type_index: 3
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_interface_method_ref() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_interface_method_ref,
            Constant::InterfaceMethodRef {
                class_index: 1,
                name_and_type_index: 3,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_interface_method_ref() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let index = constant_pool.add_interface_method_ref(class_index, "x", "()V")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Interface method Foo.x()V", value);
        Ok(())
    }

    #[test]
    fn test_add_name_and_type() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_name_and_type("name", "type")?;
        assert_eq!(3, index);
        assert_eq!(
            Some(&Constant::NameAndType {
                name_index: 1,
                descriptor_index: 2
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_name_and_type() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_name_and_type,
            Constant::NameAndType {
                name_index: 1,
                descriptor_index: 2,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_name_and_type() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_name_and_type("x", "I")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Name x, Descriptor I", value);
        Ok(())
    }

    #[test]
    fn test_add_method_handle() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_method_handle(ReferenceKind::GetField, 1)?;
        assert_eq!(1, index);
        assert_eq!(
            Some(&Constant::MethodHandle {
                reference_kind: ReferenceKind::GetField,
                reference_index: 1
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_method_handle() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_method_handle,
            Constant::MethodHandle {
                reference_kind: ReferenceKind::GetField,
                reference_index: 1,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_method_handle() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let field_index = constant_pool.add_field_ref(class_index, "x", "I")?;
        let index = constant_pool.add_method_handle(ReferenceKind::GetField, field_index)?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Method handle GetField Field Foo.x", value);
        Ok(())
    }

    #[test]
    fn test_add_method_type() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_method_type("()V")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::MethodType(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_method_type() {
        test_try_get_constant(ConstantPool::try_get_method_type, Constant::MethodType(1));
    }

    #[test]
    fn test_try_get_formatted_string_method_type() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_method_type("()V")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Method type ()V", value);
        Ok(())
    }

    #[test]
    fn test_add_dynamic() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_dynamic(1, "name", "type")?;
        assert_eq!(4, index);
        assert_eq!(
            Some(&Constant::Dynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 3
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_dynamic() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_dynamic,
            Constant::Dynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 3,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_dynamic() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let method_index = constant_pool.add_method_ref(class_index, "x", "()V")?;
        let index = constant_pool.add_dynamic(method_index, "x", "()I")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Dynamic #6:x:()I", value);
        Ok(())
    }

    #[test]
    fn test_add_invoke_dynamic() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_invoke_dynamic(1, "name", "type")?;
        assert_eq!(4, index);
        assert_eq!(
            Some(&Constant::InvokeDynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 3
            }),
            constant_pool.get(index)
        );
        Ok(())
    }

    #[test]
    fn test_try_get_invoke_dynamic() {
        test_try_get_constant_tuple(
            ConstantPool::try_get_invoke_dynamic,
            Constant::InvokeDynamic {
                bootstrap_method_attr_index: 1,
                name_and_type_index: 3,
            },
        );
    }

    #[test]
    fn test_try_get_formatted_string_invoke_dynamic() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let class_index = constant_pool.add_class("Foo")?;
        let method_index = constant_pool.add_method_ref(class_index, "x", "()V")?;
        let index = constant_pool.add_invoke_dynamic(method_index, "x", "()I")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("InvokeDynamic #6:x:()I", value);
        Ok(())
    }

    #[test]
    fn test_add_module() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_module("module")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::Module(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_module() {
        test_try_get_constant(ConstantPool::try_get_module, Constant::Module(1));
    }

    #[test]
    fn test_try_get_formatted_string_module() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_module("foo")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Module foo", value);
        Ok(())
    }

    #[test]
    fn test_add_package() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_package("package")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::Package(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_package() {
        test_try_get_constant(ConstantPool::try_get_package, Constant::Package(1));
    }

    #[test]
    fn test_try_get_formatted_string_package() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_package("foo")?;
        let value = constant_pool.try_get_formatted_string(index)?;
        assert_eq!("Package foo", value);
        Ok(())
    }

    #[test]
    fn test_from_bytes_invalid_tag() {
        let mut bytes = Cursor::new(vec![0, 0, 10]);
        assert_eq!(
            Err(IoError("Invalid constant pool count".to_string())),
            ConstantPool::from_bytes(&mut bytes)
        );
    }
}
