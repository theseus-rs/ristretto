use crate::attributes::AnnotationElement;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a single element-value pair in an annotation.
///
/// Each pair consists of an element name (an index into the constant pool for a
/// `CONSTANT_Utf8_info` structure) and an element value (an `AnnotationElement`).
///
/// This structure is part of the `annotation` structure defined in the JVM specification.
/// See the [JVM Specification §4.7.16](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16)
/// for more details.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{AnnotationValuePair, AnnotationElement};
/// use std::io::Cursor;
///
/// // Create different types of annotation value pairs
///
/// // 1. A string annotation element (e.g., @MyAnnotation(name = "example"))
/// let string_pair = AnnotationValuePair {
///     name_index: 10, // Points to "name" in constant pool
///     value: AnnotationElement::String {
///         const_value_index: 15, // Points to "example" in constant pool
///     },
/// };
///
/// // 2. An integer annotation element (e.g., @MyAnnotation(count = 42))
/// let int_pair = AnnotationValuePair {
///     name_index: 11, // Points to "count" in constant pool
///     value: AnnotationElement::Int {
///         const_value_index: 16, // Points to Integer 42 in constant pool
///     },
/// };
///
/// // 3. An enum annotation element (e.g., @MyAnnotation(direction = Direction.NORTH))
/// let enum_pair = AnnotationValuePair {
///     name_index: 12, // Points to "direction" in constant pool
///     value: AnnotationElement::Enum {
///         type_name_index: 17,    // Points to "com/example/Direction" in constant pool
///         const_name_index: 18,   // Points to "NORTH" in constant pool
///     },
/// };
///
/// // Serializing and deserializing
/// let mut bytes = Vec::new();
/// string_pair.to_bytes(&mut bytes)?;
///
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_pair = AnnotationValuePair::from_bytes(&mut cursor)?;
///
/// assert_eq!(string_pair, deserialized_pair);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnotationValuePair {
    /// An index into the `constant_pool` table. The entry at this index must be a
    /// `CONSTANT_Utf8_info` structure representing a valid Java field or method name
    /// encoded in internal form. This is the name of the annotation element.
    pub name_index: u16,
    /// The value of the annotation element.
    pub value: AnnotationElement,
}

impl AnnotationValuePair {
    /// Deserializes an `AnnotationValuePair` from a byte stream.
    ///
    /// The `bytes` cursor should be positioned at the start of the `element_value_pair` data.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails or if deserializing
    /// the nested `AnnotationElement` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{AnnotationValuePair, AnnotationElement};
    /// use std::io::Cursor;
    ///
    /// // Create raw bytes representing an annotation value pair
    /// // Format: [name_index (2 bytes)][tag (1 byte)][const_value_index (2 bytes)]
    /// let raw_bytes = vec![
    ///     0, 7,    // name_index: 7 (points to a Utf8 constant pool entry, e.g., "value")
    ///     b'I',    // tag: 'I' for Integer element type
    ///     0, 15,   // const_value_index: 15 (points to an Integer constant pool entry)
    /// ];
    ///
    /// // Create a cursor over the bytes
    /// let mut cursor = Cursor::new(raw_bytes);
    ///
    /// // Deserialize the bytes into an AnnotationValuePair
    /// let pair = AnnotationValuePair::from_bytes(&mut cursor)?;
    ///
    /// // Verify the deserialized data
    /// assert_eq!(pair.name_index, 7);
    /// assert_eq!(
    ///     pair.value,
    ///     AnnotationElement::Int { const_value_index: 15 }
    /// );
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<AnnotationValuePair> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let value = AnnotationElement::from_bytes(bytes)?;
        let annotation_value_pair = AnnotationValuePair { name_index, value };

        Ok(annotation_value_pair)
    }

    /// Serializes the `AnnotationValuePair` to a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if serializing the nested `AnnotationElement` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{AnnotationValuePair, AnnotationElement};
    ///
    /// // Create an AnnotationValuePair
    /// let pair = AnnotationValuePair {
    ///     name_index: 5, // Index to Utf8 in constant pool for the element name
    ///     value: AnnotationElement::Int {
    ///         const_value_index: 10, // Index to a constant pool entry
    ///     },
    /// };
    ///
    /// // Serialize the pair to bytes
    /// let mut bytes = Vec::new();
    /// pair.to_bytes(&mut bytes)?;
    ///
    /// // The serialized bytes now contain the binary representation of the annotation value pair
    /// // The first 2 bytes are the name_index (0x0005)
    /// // The next byte is the tag for Int element ('I')
    /// // The last 2 bytes are the const_value_index (0x000A)
    /// assert_eq!(bytes, vec![0, 5, b'I', 0, 10]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.value.to_bytes(bytes)
    }
}

impl fmt::Display for AnnotationValuePair {
    /// Formats the `AnnotationValuePair` for display.
    ///
    /// This implementation provides a human-readable representation of the annotation value pair,
    /// showing both the name index and the value in a simple text format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{AnnotationValuePair, AnnotationElement};
    ///
    /// // Create an annotation value pair
    /// let pair = AnnotationValuePair {
    ///     name_index: 5,
    ///     value: AnnotationElement::String { const_value_index: 10 },
    /// };
    ///
    /// let output = pair.to_string();
    /// assert_eq!(output, "name_index: 5, value: String { const_value_index: 10 }");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name_index: {}, value: {}", self.name_index, self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };

        assert_eq!(
            "name_index: 1, value: Byte { const_value_index: 42 }",
            annotation_value_pair.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                // 'B' is 66 in ASCII
                const_value_index: 42,
            },
        };
        let expected_bytes = [0, 1, b'B', 0, 42];

        let mut bytes = Vec::new();
        annotation_value_pair.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(
            annotation_value_pair,
            AnnotationValuePair::from_bytes(&mut bytes)?
        );
        Ok(())
    }
}
