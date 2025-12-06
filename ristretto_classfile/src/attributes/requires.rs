use crate::attributes::RequiresFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a module dependency entry in the `requires` table of a Java module descriptor.
///
/// In the Java Platform Module System, the `Requires` attribute specifies the modules
/// that the current module depends on, along with flags indicating the nature of the
/// dependency (such as whether it's transitive, static, etc.).
///
/// # Fields
///
/// * `index` - Index into the constant pool pointing to a `CONSTANT_Module_info` structure
///   that represents the required module.
/// * `flags` - Access flags for this dependency, indicating properties like whether the
///   dependency is transitive, static, or mandated by the JVM.
/// * `version_index` - Index into the constant pool pointing to a `CONSTANT_Utf8_info`
///   structure that represents the version of the required module, or `0` if no version is specified.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{Requires, RequiresFlags};
/// use ristretto_classfile::Result;
/// use std::io::Cursor;
///
/// fn main() -> Result<()> {
///     let requires_entry = Requires {
///         index: 1, // Index to a Module constant in the constant pool
///         flags: RequiresFlags::TRANSITIVE | RequiresFlags::MANDATED,
///         version_index: 0, // Optional: Index to a Utf8 for module version, 0 if no version
///     };
///
///     // Serialize
///     let mut bytes = Vec::new();
///     requires_entry.to_bytes(&mut bytes)?;
///
///     // Deserialize
///     let mut cursor = Cursor::new(bytes);
///     let deserialized_requires = Requires::from_bytes(&mut cursor)?;
///
///     assert_eq!(requires_entry, deserialized_requires);
///     Ok(())
/// }
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requires {
    /// Index into the constant pool referencing a `CONSTANT_Module_info` structure
    /// that identifies the required module.
    pub index: u16,

    /// Flags that characterize this module dependency.
    /// Common flags include `TRANSITIVE`, `STATIC_PHASE`, and `MANDATED`.
    pub flags: RequiresFlags,

    /// Index into the constant pool referencing a `CONSTANT_Utf8_info` structure
    /// specifying the version of the required module, or `0` if no version is specified.
    pub version_index: u16,
}

impl Requires {
    /// Deserialize `Requires` from bytes.
    ///
    /// Reads a `Requires` entry from the provided byte stream in the format
    /// specified by the JVM specification:
    /// - 2 bytes for the module index
    /// - 2 bytes for the flags
    /// - 2 bytes for the version index
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ristretto_classfile::attributes::{Requires, RequiresFlags};
    /// # use ristretto_classfile::Result;
    /// # use std::io::Cursor;
    /// # fn example() -> Result<()> {
    /// // Create a byte array representing a serialized Requires entry
    /// let raw_bytes = vec![0, 5, 0, 32, 0, 0]; // module_index=5, flags=STATIC_PHASE, no version
    /// let mut cursor = Cursor::new(raw_bytes);
    ///
    /// // Deserialize
    /// let requires = Requires::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(requires.index, 5);
    /// assert_eq!(requires.flags, RequiresFlags::STATIC_PHASE);
    /// assert_eq!(requires.version_index, 0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Requires> {
        let index = bytes.read_u16::<BigEndian>()?;
        let flags = RequiresFlags::from_bytes(bytes)?;
        let version_index = bytes.read_u16::<BigEndian>()?;
        let require = Requires {
            index,
            flags,
            version_index,
        };
        Ok(require)
    }

    /// Serialize `Requires` to bytes.
    ///
    /// Writes this `Requires` entry to the provided byte vector in the format
    /// specified by the JVM specification:
    /// - 2 bytes for the module index
    /// - 2 bytes for the flags
    /// - 2 bytes for the version index
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ristretto_classfile::attributes::{Requires, RequiresFlags};
    /// # use ristretto_classfile::Result;
    /// # fn example() -> Result<()> {
    /// // Create a Requires entry
    /// let requires = Requires {
    ///     index: 7,
    ///     flags: RequiresFlags::TRANSITIVE,
    ///     version_index: 12,
    /// };
    ///
    /// // Serialize to bytes
    /// let mut buffer = Vec::new();
    /// requires.to_bytes(&mut buffer)?;
    ///
    /// // The buffer now contains the binary representation of the Requires entry
    /// assert_eq!(buffer, vec![0, 7, 0, 32, 0, 12]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte stream fails or if the flags cannot be serialized.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;
        self.flags.to_bytes(bytes)?;
        bytes.write_u16::<BigEndian>(self.version_index)?;
        Ok(())
    }
}

impl fmt::Display for Requires {
    /// Formats the `Requires` instance as a human-readable string.
    ///
    /// The formatted string includes the module index, flags, and version index for easy inspection
    /// and debugging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Requires, RequiresFlags};
    ///
    /// let requires = Requires {
    ///     index: 5,
    ///     flags: RequiresFlags::TRANSITIVE | RequiresFlags::STATIC_PHASE,
    ///     version_index: 8,
    /// };
    ///
    /// let output = requires.to_string();
    /// assert_eq!(
    ///     output,
    ///     "Requires[index=5, flags=(0x0060) ACC_TRANSITIVE, ACC_STATIC_PHASE, version_index=8]",
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Requires[index={}, flags={}, version_index={}]",
            self.index, self.flags, self.version_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let requires = Requires {
            index: 1,
            flags: RequiresFlags::MANDATED,
            version_index: 3,
        };
        assert_eq!(
            "Requires[index=1, flags=(0x8000) ACC_MANDATED, version_index=3]",
            requires.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let requires = Requires {
            index: 1,
            flags: RequiresFlags::MANDATED,
            version_index: 3,
        };
        let expected_value = [0, 1, 128, 0, 0, 3];
        let mut bytes = Vec::new();
        requires.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(requires, Requires::from_bytes(&mut bytes)?);
        Ok(())
    }
}
