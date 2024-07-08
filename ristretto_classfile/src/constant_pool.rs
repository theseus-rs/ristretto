use crate::constant::Constant;
use crate::error::Result;
use crate::Error::{InvalidConstantPoolIndex, InvalidConstantPoolIndexType};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Constant pool.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConstantPool {
    constants: Vec<ConstantEntry>,
}

impl ConstantPool {
    /// Add a constant to the pool.
    pub fn add(&mut self, constant: Constant) {
        let add_placeholder = matches!(constant, Constant::Long(_) | Constant::Double(_));
        self.constants.push(ConstantEntry::Constant(constant));
        if add_placeholder {
            self.constants.push(ConstantEntry::Placeholder);
        }
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
        let constant_entry = self.constants.get(index.saturating_sub(1) as usize);
        match constant_entry {
            Some(ConstantEntry::Constant(constant)) => Ok(constant),
            _ => Err(InvalidConstantPoolIndex(index)),
        }
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

    /// Get the number of constants in the pool.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constants.len()
    }

    /// Check if the pool is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constants.is_empty()
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
            constant_pool.add(constant);
        }

        Ok(constant_pool)
    }

    /// Serialize the `ConstantPool` to bytes.
    ///
    /// # Errors
    /// If there are more than 65,535 constants in the pool.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        let constant_pool_count = u16::try_from(self.constants.len())? + 1;
        bytes.write_u16::<BigEndian>(constant_pool_count)?;
        for constant_entry in &self.constants {
            if let ConstantEntry::Constant(constant) = constant_entry {
                constant.to_bytes(bytes)?;
            }
        }
        Ok(())
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

#[allow(clippy::module_name_repetitions)]
pub struct ConstantPoolIterator<'a> {
    constant_pool: &'a ConstantPool,
    index: usize,
}

impl<'a> ConstantPoolIterator<'a> {
    pub fn new(constant_pool: &'a ConstantPool) -> Self {
        Self {
            constant_pool,
            index: 0,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::constant::Constant;

    #[test]
    fn test_get_zero_none() {
        let constant_pool = ConstantPool::default();
        assert!(constant_pool.get(0).is_none());
    }

    #[test]
    fn test_get() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.get(1).is_none());
        constant_pool.add(Constant::Utf8("foo".to_string()));
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
        constant_pool.add(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.try_get(1).is_ok());
    }

    #[test]
    fn test_try_get_utf8_zero_error() {
        let constant_pool = ConstantPool::default();
        assert_eq!(
            Err(InvalidConstantPoolIndex(0)),
            constant_pool.try_get_utf8(0)
        );
    }

    #[test]
    fn test_try_get_utf8_type_error() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Integer(42));
        assert_eq!(
            Err(InvalidConstantPoolIndexType(1)),
            constant_pool.try_get_utf8(1)
        );
    }

    #[test]
    fn test_try_get_utf8() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.try_get(1).is_err());
        constant_pool.add(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.try_get(1).is_ok());
    }

    #[test]
    fn test_utf8() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("foo".to_string()));
        assert!(constant_pool.get(1).is_some());
        assert_eq!(1, constant_pool.constants.len());
    }

    #[test]
    fn test_integer() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Integer(42));
        assert!(constant_pool.get(1).is_some());
        assert_eq!(1, constant_pool.constants.len());
    }

    #[test]
    fn test_long() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Long(1_234_567_890));
        assert!(constant_pool.get(1).is_some());
        assert!(constant_pool.get(2).is_none());
        assert_eq!(2, constant_pool.constants.len());
    }

    #[test]
    fn test_len() {
        let mut constant_pool = ConstantPool::default();
        assert_eq!(0, constant_pool.len());
        constant_pool.add(Constant::Integer(42));
        assert_eq!(1, constant_pool.len());
    }

    #[test]
    fn test_is_empty() {
        let mut constant_pool = ConstantPool::default();
        assert!(constant_pool.is_empty());
        constant_pool.add(Constant::Integer(42));
        assert!(!constant_pool.is_empty());
    }

    #[test]
    fn test_iter() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("foo".to_string()));
        constant_pool.add(Constant::Integer(42));
        constant_pool.add(Constant::Long(1_234_567_890));
        let mut iter = constant_pool.iter();
        assert_eq!(Some(&Constant::Utf8("foo".to_string())), iter.next());
        assert_eq!(Some(&Constant::Integer(42)), iter.next());
        assert_eq!(Some(&Constant::Long(1_234_567_890)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_into_iter() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("foo".to_string()));
        for constant in &constant_pool {
            assert_eq!(Constant::Utf8("foo".to_string()), *constant);
        }
    }

    #[test]
    fn test_double() {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Double(std::f64::consts::PI));
        assert!(constant_pool.get(1).is_some());
        assert!(constant_pool.get(2).is_none());
        assert_eq!(2, constant_pool.constants.len());
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        let integer_constant = Constant::Integer(42);
        let long_constant = Constant::Long(1_234_567_890);

        constant_pool.add(integer_constant.clone());
        constant_pool.add(long_constant.clone());

        let expected_bytes = [0, 4, 3, 0, 0, 0, 42, 5, 0, 0, 0, 0, 73, 150, 2, 210];
        let mut bytes = Vec::new();
        constant_pool.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(constant_pool, ConstantPool::from_bytes(&mut bytes)?);
        Ok(())
    }
}
