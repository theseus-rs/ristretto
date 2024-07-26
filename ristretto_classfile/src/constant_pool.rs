use crate::constant::Constant;
use crate::error::Result;
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use crate::ReferenceKind;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Constant pool.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4>
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantPool {
    constants: Vec<ConstantEntry>,
}

impl ConstantPool {
    /// Create a new constant pool.
    #[must_use]
    pub fn new() -> Self {
        // The constant pool is 1-based, so the first entry is a placeholder.
        Self {
            constants: vec![ConstantEntry::Placeholder],
        }
    }

    /// Push a constant to the pool.
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
    /// If there are more than 65,534 constants in the pool.
    pub fn add(&mut self, constant: Constant) -> Result<u16> {
        // Logically the index is self.len() + 1.  However, since the constant pool is one based and
        // a placeholder is added as the first entry, we can just use the length of the constants
        // vector to obtain the new index value.
        let index = u16::try_from(self.constants.len())?;
        self.push(constant);
        Ok(index)
    }

    /// Get a constant from the pool by index; indexes are 1-based.
    /// Returns None if the index is out of bounds.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.1:~:text=The%20constant_pool%20table%20is%20indexed%20from%201%20to%20constant_pool_count%20%2D%201.>
    #[must_use]
    pub fn get(&self, index: u16) -> Option<&Constant> {
        match self.try_get(index) {
            Ok(constant) => Some(constant),
            Err(_) => None,
        }
    }

    /// Get a constant from the pool by index; indexes are 1-based.
    /// Returns an error if the index is out of bounds.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.1:~:text=The%20constant_pool%20table%20is%20indexed%20from%201%20to%20constant_pool_count%20%2D%201.>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds.
    pub fn try_get(&self, index: u16) -> Result<&Constant> {
        let constant_entry = self.constants.get(index as usize);
        match constant_entry {
            Some(ConstantEntry::Constant(constant)) => Ok(constant),
            _ => Err(InvalidConstantPoolIndex(index)),
        }
    }

    /// Get the number of constants in the pool.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constants.len() - 1
    }

    /// Check if the pool is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get an iterator over the constants in the pool.
    #[must_use]
    pub fn iter(&self) -> ConstantPoolIterator {
        ConstantPoolIterator::new(self)
    }

    /// Deserialize the `ConstantPool` from bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes are not a valid constant pool.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ConstantPool> {
        let mut constant_pool = ConstantPool::default();
        let constant_pool_count = bytes.read_u16::<BigEndian>()? - 1;
        while constant_pool.len() < constant_pool_count as usize {
            let constant = Constant::from_bytes(bytes)?;
            constant_pool.push(constant);
        }

        Ok(constant_pool)
    }

    /// Serialize the `ConstantPool` to bytes.
    ///
    /// # Errors
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
    /// If there are more than 65,534 constants in the pool.
    pub fn add_utf8<S: AsRef<str>>(&mut self, value: S) -> Result<u16> {
        let value = value.as_ref().to_string();
        self.add(Constant::Utf8(value))
    }

    /// Get a UTF-8 constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a UTF-8 constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.7>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a UTF-8 constant.
    pub fn try_get_utf8(&self, index: u16) -> Result<&String> {
        match self.try_get(index) {
            Ok(Constant::Utf8(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add an integer constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_integer(&mut self, value: i32) -> Result<u16> {
        self.add(Constant::Integer(value))
    }

    /// Get an integer constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not an integer constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.4>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not an integer constant.
    pub fn try_get_integer(&self, index: u16) -> Result<&i32> {
        match self.try_get(index) {
            Ok(Constant::Integer(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a float constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_float(&mut self, value: f32) -> Result<u16> {
        self.add(Constant::Float(value))
    }

    /// Get a float constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a float constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.4>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a float constant.
    pub fn try_get_float(&self, index: u16) -> Result<&f32> {
        match self.try_get(index) {
            Ok(Constant::Float(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a long constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_long(&mut self, value: i64) -> Result<u16> {
        self.add(Constant::Long(value))
    }

    /// Get a long constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a long constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.5>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a long constant.
    pub fn try_get_long(&self, index: u16) -> Result<&i64> {
        match self.try_get(index) {
            Ok(Constant::Long(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a double constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_double(&mut self, value: f64) -> Result<u16> {
        self.add(Constant::Double(value))
    }

    /// Get a double constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a double constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.5>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a double constant.
    pub fn try_get_double(&self, index: u16) -> Result<&f64> {
        match self.try_get(index) {
            Ok(Constant::Double(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a class constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_class<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Class(utf8_index))
    }

    /// Get a class constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a class constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.1>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a class constant.
    pub fn try_get_class(&self, index: u16) -> Result<&String> {
        match self.try_get(index) {
            Ok(Constant::Class(value)) => self.try_get_utf8(*value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a string constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_string<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::String(utf8_index))
    }

    /// Get a string constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a string constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.3>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a string constant.
    pub fn try_get_string(&self, index: u16) -> Result<&String> {
        match self.try_get(index) {
            Ok(Constant::String(value)) => self.try_get_utf8(*value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a field reference constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not a field constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.2>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a field constant.
    pub fn try_get_field_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::FieldRef {
                class_index,
                name_and_type_index,
            }) => Ok((class_index, name_and_type_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method reference constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not a method constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.2>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a method constant.
    pub fn try_get_method_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::MethodRef {
                class_index,
                name_and_type_index,
            }) => Ok((class_index, name_and_type_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add an interface method reference constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not an interface method constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.2>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not an interface method
    /// constant.
    pub fn try_get_interface_method_ref(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            }) => Ok((class_index, name_and_type_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a name and type constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_name_and_type<S: AsRef<str>>(&mut self, name: S, descriptor: S) -> Result<u16> {
        let name_index = self.add_utf8(name)?;
        let descriptor_index = self.add_utf8(descriptor)?;
        self.add(Constant::NameAndType {
            name_index,
            descriptor_index,
        })
    }

    /// Get a name and type constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a name and type constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.2>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a name and type
    /// constant.
    pub fn try_get_name_and_type(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::NameAndType {
                name_index,
                descriptor_index,
            }) => Ok((name_index, descriptor_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method handle constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not a method handle constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.8>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a method handle
    /// constant.
    pub fn try_get_method_handle(&self, index: u16) -> Result<(&ReferenceKind, &u16)> {
        match self.try_get(index) {
            Ok(Constant::MethodHandle {
                reference_kind,
                reference_index,
            }) => Ok((reference_kind, reference_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a method type constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_method_type<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::MethodType(utf8_index))
    }

    /// Get a method type constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a method type constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.9>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a method type
    /// constant.
    pub fn try_get_method_type(&self, index: u16) -> Result<&u16> {
        match self.try_get(index) {
            Ok(Constant::MethodType(value)) => Ok(value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a dynamic constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not a dynamic constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.10>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a dynamic constant.
    pub fn try_get_dynamic(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            }) => Ok((bootstrap_method_attr_index, name_and_type_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a invoke dynamic constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
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
    /// Returns an error if the constant is not an invoke dynamic constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.10>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not an invoke
    /// dynamic constant.
    pub fn try_get_invoke_dynamic(&self, index: u16) -> Result<(&u16, &u16)> {
        match self.try_get(index) {
            Ok(Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            }) => Ok((bootstrap_method_attr_index, name_and_type_index)),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a module constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_module<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Module(utf8_index))
    }

    /// Get a module constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a module constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.11>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a module constant.
    pub fn try_get_module(&self, index: u16) -> Result<&String> {
        match self.try_get(index) {
            Ok(Constant::Module(value)) => self.try_get_utf8(*value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }

    /// Add a package constant to the pool.
    ///
    /// # Errors
    /// If there are more than 65,534 constants in the pool.
    pub fn add_package<S: AsRef<str>>(&mut self, name: S) -> Result<u16> {
        let utf8_index = self.add_utf8(name)?;
        self.add(Constant::Package(utf8_index))
    }

    /// Get a package constant from the pool by index; indexes are 1-based.
    /// Returns an error if the constant is not a package constant.
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.12>
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds or the constant is not a package constant.
    pub fn try_get_package(&self, index: u16) -> Result<&String> {
        match self.try_get(index) {
            Ok(Constant::Package(value)) => self.try_get_utf8(*value),
            Err(_) => Err(InvalidConstantPoolIndex(index)),
            _ => Err(InvalidConstantPoolIndexType(index)),
        }
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}

/// All 8 byte constants (long and double) take up two entries in the constant pool; a placeholder
/// is used to facilitate efficient indexed access of constants in the pool. See the JVM spec for:
/// <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4.5>
#[derive(Clone, Debug, PartialEq)]
enum ConstantEntry {
    Constant(Constant),
    Placeholder,
}

impl fmt::Display for ConstantEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantEntry::Constant(constant) => write!(f, "{constant}"),
            ConstantEntry::Placeholder => Ok(()),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ConstantPoolIterator<'a> {
    constant_pool: &'a ConstantPool,
    index: usize,
}

impl<'a> ConstantPoolIterator<'a> {
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
                ConstantEntry::Placeholder => continue,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        T: Debug + PartialEq,
    {
        let mut constant_pool = ConstantPool::default();
        if matches!(constant, Constant::Utf8(_)) {
            constant_pool.push(Constant::Integer(42));
        } else {
            constant_pool.push(Constant::Utf8("foo".to_string()));
        }
        constant_pool.push(Constant::Integer(42));
        assert_eq!(Err(InvalidConstantPoolIndex(0)), f(&constant_pool, 0));
        assert_eq!(Err(InvalidConstantPoolIndexType(1)), f(&constant_pool, 1));

        let mut constant_pool = ConstantPool::default();
        constant_pool.push(constant);
        assert!(constant_pool.try_get(1).is_ok());
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
        constant_pool.push(Constant::Integer(42));
        assert_eq!(Err(InvalidConstantPoolIndex(0)), f(&constant_pool, 0));
        assert_eq!(Err(InvalidConstantPoolIndexType(1)), f(&constant_pool, 1));

        let mut constant_pool = ConstantPool::default();
        constant_pool.push(constant);
        assert!(constant_pool.try_get(1).is_ok());
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
    fn test_add_string() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let index = constant_pool.add_string("foo")?;
        assert_eq!(2, index);
        assert_eq!(Some(&Constant::String(1)), constant_pool.get(index));
        Ok(())
    }

    #[test]
    fn test_try_get_string() {
        test_try_get_constant(ConstantPool::try_get_string, Constant::String(42));
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
}
