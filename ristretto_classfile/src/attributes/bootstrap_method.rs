use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a bootstrap method and its static arguments, as used by `invokedynamic` instructions.
///
/// The `BootstrapMethods` attribute in a class file contains an array of these structures. Each
/// `invokedynamic` instruction refers to one of these bootstrap methods by index. The bootstrap
/// method is responsible for resolving the dynamic call site and linking it.
///
/// See the [JVMS §4.7.23](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23)
/// for more details.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::BootstrapMethod;
/// use std::io::Cursor;
///
/// // Create a BootstrapMethod
/// let bsm = BootstrapMethod {
///     bootstrap_method_ref: 10, // Index to a CONSTANT_MethodHandle_info
///     arguments: vec![15, 20], // Indices to static arguments in the constant pool
/// };
///
/// // Serialize to bytes
/// let mut bytes = Vec::new();
/// bsm.to_bytes(&mut bytes)?;
///
/// // Deserialize from bytes
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_bsm = BootstrapMethod::from_bytes(&mut cursor)?;
///
/// assert_eq!(bsm, deserialized_bsm);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub arguments: Vec<u16>,
}

impl BootstrapMethod {
    /// Deserializes a `BootstrapMethod` structure from a byte stream.
    ///
    /// The `bytes` cursor should be positioned at the start of the `bootstrap_method` structure.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails (e.g., unexpected EOF). Currently,
    /// this specific deserialization does not produce other errors beyond I/O issues, but the
    /// `Result` type is used for consistency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::BootstrapMethod;
    /// use std::io::Cursor;
    ///
    /// let data = vec![0, 5, 0, 2, 0, 10, 0, 12];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let bsm = BootstrapMethod::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(bsm.bootstrap_method_ref, 5);
    /// assert_eq!(bsm.arguments.len(), 2);
    /// assert_eq!(bsm.arguments[0], 10);
    /// assert_eq!(bsm.arguments[1], 12);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<BootstrapMethod> {
        let bootstrap_method_ref = bytes.read_u16::<BigEndian>()?;
        let arguments_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut arguments = Vec::with_capacity(arguments_count);
        for _ in 0..arguments_count {
            arguments.push(bytes.read_u16::<BigEndian>()?);
        }
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref,
            arguments,
        };
        Ok(bootstrap_method)
    }

    /// Serializes the `BootstrapMethod` structure to a byte vector.
    ///
    /// # Errors
    ///
    /// - If the number of `arguments` exceeds 65,535 (the maximum for a `u16` length).
    /// - Propagates I/O errors if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::BootstrapMethod;
    ///
    /// let bsm = BootstrapMethod {
    ///     bootstrap_method_ref: 8,
    ///     arguments: vec![22, 33, 44],
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// bsm.to_bytes(&mut bytes)?;
    ///
    /// let expected_bytes = vec![0, 8, 0, 3, 0, 22, 0, 33, 0, 44];
    /// assert_eq!(bytes, expected_bytes);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bootstrap_method_ref)?;

        let arguments_length = u16::try_from(self.arguments.len())?;
        bytes.write_u16::<BigEndian>(arguments_length)?;
        for argument in &self.arguments {
            bytes.write_u16::<BigEndian>(*argument)?;
        }
        Ok(())
    }
}

impl fmt::Display for BootstrapMethod {
    /// Formats the `BootstrapMethod` for display.
    ///
    /// This implementation shows the bootstrap method reference index and the array of argument
    /// indices to the constant pool in a human-readable format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::BootstrapMethod;
    ///
    /// let bootstrap_method = BootstrapMethod {
    ///     bootstrap_method_ref: 3,
    ///     arguments: vec![10, 20, 30],
    /// };
    ///
    /// let output = bootstrap_method.to_string();
    /// assert_eq!(output, "bootstrap_method_ref: 3, arguments: [10, 20, 30]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bootstrap_method_ref: {}, arguments: {:?}",
            self.bootstrap_method_ref, self.arguments
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref: 3,
            arguments: vec![42],
        };
        assert_eq!(
            "bootstrap_method_ref: 3, arguments: [42]",
            bootstrap_method.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref: 3,
            arguments: vec![42],
        };
        let expected_value = [0, 3, 0, 1, 0, 42];

        let mut bytes = Vec::new();
        bootstrap_method.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(bootstrap_method, BootstrapMethod::from_bytes(&mut bytes)?);
        Ok(())
    }
}
