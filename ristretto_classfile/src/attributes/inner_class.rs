use crate::attributes::nested_class_access_flags::NestedClassAccessFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents an entry in the `InnerClasses` attribute, describing a nested class or interface.
///
/// The `InnerClasses` attribute of a class file stores information about all nested classes and
/// interfaces that are members of the current class or interface, or are anonymous or local
/// classes within its methods.
///
/// See the [JVMS §4.7.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.6)
/// for more details.
///
/// # Fields
///
/// - `class_info_index`: An index into the `constant_pool` table. The entry at this index must be
///   a `CONSTANT_Class_info` structure representing the inner class C.
/// - `outer_class_info_index`: An index into the `constant_pool` table. If C is not a member of
///   any class or interface (e.g., it is a top-level class or interface, or a local or anonymous
///   class), this value must be zero. Otherwise, the entry at this index must be a
///   `CONSTANT_Class_info` structure representing the outer class or interface O of which C is a
///   member.
/// - `name_index`: An index into the `constant_pool` table. If C is anonymous, this value must be
///   zero. Otherwise, the entry at this index must be a `CONSTANT_Utf8_info` structure representing
///   the original simple name of C, as given in the source code. This name might be different from
///   the binary name if C is a local class.
/// - `access_flags`: A set of `NestedClassAccessFlags` describing the access permissions and
///   properties of the inner class C as declared in the source code.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{InnerClass, NestedClassAccessFlags};
/// use std::io::Cursor;
///
/// // Example: A public static nested class "MyInner"
/// let inner_class_entry = InnerClass {
///     class_info_index: 10,       // CP index for "com/example/Outer$MyInner"
///     outer_class_info_index: 5,  // CP index for "com/example/Outer"
///     name_index: 12,             // CP index for the string "MyInner"
///     access_flags: NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::STATIC,
/// };
///
/// // Serialize the entry
/// let mut bytes = Vec::new();
/// inner_class_entry.to_bytes(&mut bytes)?;
///
/// // Deserialize the entry
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_entry = InnerClass::from_bytes(&mut cursor)?;
///
/// assert_eq!(inner_class_entry, deserialized_entry);
///
/// // Example: An anonymous class (name_index is 0)
/// let anonymous_class_entry = InnerClass {
///     class_info_index: 20,       // CP index for "com/example/Outer$1"
///     outer_class_info_index: 5,  // CP index for "com/example/Outer"
///     name_index: 0,              // Anonymous, so name_index is 0
///     access_flags: NestedClassAccessFlags::FINAL, // Typically final
/// };
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InnerClass {
    pub class_info_index: u16,
    pub outer_class_info_index: u16,
    pub name_index: u16,
    pub access_flags: NestedClassAccessFlags,
}

impl InnerClass {
    /// Deserializes an `InnerClass` structure from a byte stream.
    ///
    /// The `bytes` cursor should be positioned at the start of the `classes` entry within the
    /// `InnerClasses` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails or if deserializing the
    /// `NestedClassAccessFlags` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{InnerClass, NestedClassAccessFlags};
    /// use std::io::Cursor;
    ///
    /// let data = vec![0, 1, 0, 2, 0, 3, 0, 1];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let inner_class = InnerClass::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(inner_class.class_info_index, 1);
    /// assert_eq!(inner_class.outer_class_info_index, 2);
    /// assert_eq!(inner_class.name_index, 3);
    /// assert_eq!(inner_class.access_flags, NestedClassAccessFlags::PUBLIC);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<InnerClass> {
        let class_info_index = bytes.read_u16::<BigEndian>()?;
        let outer_class_info_index = bytes.read_u16::<BigEndian>()?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let access_flags = NestedClassAccessFlags::from_bytes(bytes)?;

        let inner_class = InnerClass {
            class_info_index,
            outer_class_info_index,
            name_index,
            access_flags,
        };
        Ok(inner_class)
    }

    /// Serializes the `InnerClass` structure to a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if serializing the `NestedClassAccessFlags` fails or if writing to the byte
    /// vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{InnerClass, NestedClassAccessFlags};
    ///
    /// let inner_class = InnerClass {
    ///     class_info_index: 10,
    ///     outer_class_info_index: 0, // Not a member of another class (e.g. local class)
    ///     name_index: 12,            // Name "LocalClass"
    ///     access_flags: NestedClassAccessFlags::FINAL | NestedClassAccessFlags::PRIVATE,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// inner_class.to_bytes(&mut bytes)?;
    ///
    /// let expected_bytes = vec![0, 10, 0, 0, 0, 12, 0, 0x12];
    /// assert_eq!(bytes, expected_bytes);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.class_info_index)?;
        bytes.write_u16::<BigEndian>(self.outer_class_info_index)?;
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.access_flags.to_bytes(bytes)
    }
}

impl fmt::Display for InnerClass {
    /// Implements the `Display` trait for the `InnerClass` structure.
    ///
    /// This implementation provides a human-readable string representation of an `InnerClass`
    /// entry, displaying all of its fields in a concise format. This is useful for debugging,
    /// logging, or displaying the inner class information in a readable manner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{InnerClass, NestedClassAccessFlags};
    ///
    /// let inner_class = InnerClass {
    ///     class_info_index: 10,
    ///     outer_class_info_index: 5,
    ///     name_index: 12,
    ///     access_flags: NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::STATIC,
    /// };
    ///
    /// let output = inner_class.to_string();
    /// assert_eq!(
    ///     output,
    ///     "class_info_index: 10, outer_class_info_index: 5, name_index: 12, access_flags: (0x0009) ACC_PUBLIC, ACC_STATIC",
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "class_info_index: {}, outer_class_info_index: {}, name_index: {}, access_flags: {}",
            self.class_info_index, self.outer_class_info_index, self.name_index, self.access_flags
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let inner_class = InnerClass {
            class_info_index: 1,
            outer_class_info_index: 2,
            name_index: 3,
            access_flags: NestedClassAccessFlags::PUBLIC,
        };
        assert_eq!(
            "class_info_index: 1, outer_class_info_index: 2, name_index: 3, access_flags: (0x0001) ACC_PUBLIC",
            inner_class.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let inner_class = InnerClass {
            class_info_index: 1,
            outer_class_info_index: 2,
            name_index: 3,
            access_flags: NestedClassAccessFlags::PUBLIC,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 1];

        let mut bytes = Vec::new();
        inner_class.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(inner_class, InnerClass::from_bytes(&mut bytes)?);
        Ok(())
    }
}
