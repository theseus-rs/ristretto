use crate::error::Result;
use crate::method_access_flags::MethodAccessFlags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `MethodParameter`.
///
/// The `MethodParameter` struct represents a parameter in a method's signature, including its name
/// index in the constant pool and the access flags that apply to it.
///
/// This information is stored in the `MethodParameters` attribute of a method, which was introduced
/// in Java 8 to provide formal parameter information.
///
/// # Fields
///
/// * `name_index` - Index into the constant pool pointing to a UTF-8 string representing
///   the parameter name, or zero if the parameter is unnamed.
/// * `access_flags` - Access flags for this parameter (e.g., `final`, `synthetic`).
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::MethodParameter;
/// use ristretto_classfile::MethodAccessFlags;
///
/// // Create a method parameter representing a final parameter named "value"
/// // where the name is at constant pool index 5
/// let parameter = MethodParameter {
///     name_index: 5,
///     access_flags: MethodAccessFlags::FINAL,
/// };
///
/// // Access parameter information
/// assert_eq!(parameter.name_index, 5);
/// assert!(parameter.access_flags.contains(MethodAccessFlags::FINAL));
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.24](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.24)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: MethodAccessFlags,
}

impl MethodParameter {
    /// Deserialize the method parameters from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::MethodParameter;
    /// use ristretto_classfile::MethodAccessFlags;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = Cursor::new(vec![0x00, 0x03, 0x00, 0x01]); // name_index: 3, access_flags: ACC_PUBLIC
    /// let method_parameter = MethodParameter::from_bytes(&mut bytes)?;
    /// assert_eq!(method_parameter.name_index, 3);
    /// assert_eq!(method_parameter.access_flags, MethodAccessFlags::PUBLIC);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<MethodParameter> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let access_flags = MethodAccessFlags::from_bytes(bytes)?;
        let bootstrap_method = MethodParameter {
            name_index,
            access_flags,
        };
        Ok(bootstrap_method)
    }

    /// Serialize the method parameters to bytes.
    ///
    /// # Errors
    ///
    /// If method access flags cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::MethodParameter;
    /// use ristretto_classfile::MethodAccessFlags;
    ///
    /// let method_parameter = MethodParameter {
    ///     name_index: 3,
    ///     access_flags: MethodAccessFlags::PUBLIC,
    /// };
    /// let mut bytes = Vec::new();
    /// method_parameter.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x03, 0x00, 0x01]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.access_flags.to_bytes(bytes)
    }
}

impl fmt::Display for MethodParameter {
    /// Implements the `Display` trait for `MethodParameter`.
    ///
    /// This implementation formats the method parameter as a human-readable string, showing both
    /// the name index into the constant pool and the access flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::MethodParameter;
    /// use ristretto_classfile::MethodAccessFlags;
    ///
    /// let method_parameter = MethodParameter {
    ///     name_index: 7,
    ///     access_flags: MethodAccessFlags::FINAL | MethodAccessFlags::SYNTHETIC,
    /// };
    ///
    /// let output = method_parameter.to_string();
    /// assert_eq!(
    ///     output,
    ///     "name_index: #7, access_flags: (0x1010) ACC_FINAL, ACC_SYNTHETIC"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name_index: #{}, access_flags: {}",
            self.name_index, self.access_flags
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let method_parameter = MethodParameter {
            name_index: 3,
            access_flags: MethodAccessFlags::PUBLIC,
        };
        assert_eq!(
            "name_index: #3, access_flags: (0x0001) ACC_PUBLIC",
            method_parameter.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let method_parameter = MethodParameter {
            name_index: 3,
            access_flags: MethodAccessFlags::PUBLIC,
        };
        let expected_value = [0, 3, 0, 1];
        let mut bytes = Vec::new();
        method_parameter.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(method_parameter, MethodParameter::from_bytes(&mut bytes)?);
        Ok(())
    }
}
