use crate::attributes::Annotation;
use crate::error::Error::InvalidAnnotationElementTag;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents the value of an element in an annotation.
///
/// This enum corresponds to the `element_value` structure in the JVM specification. Each variant
/// represents a different type of value that an annotation element can have.
///
/// See the [JVMS §4.7.16.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16.1)
/// for more details on the `element_value` structure.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{AnnotationElement, Annotation};
/// use std::io::Cursor;
///
/// // Integer element
/// let int_element = AnnotationElement::Int { const_value_index: 10 };
///
/// // String element
/// let string_element = AnnotationElement::String { const_value_index: 20 };
///
/// // Enum element
/// let enum_element = AnnotationElement::Enum {
///     type_name_index: 30, // Index to Utf8 for enum type name
///     const_name_index: 31, // Index to Utf8 for enum constant name
/// };
///
/// // Array element
/// let array_element = AnnotationElement::Array {
///     values: vec![
///         AnnotationElement::Char { const_value_index: 5 },
///         AnnotationElement::Boolean { const_value_index: 6 },
///     ]
/// };
///
/// // Nested Annotation element
/// let nested_annotation = AnnotationElement::Annotation {
///     annotation: Annotation {
///         type_index: 40,
///         elements: vec![],
///     }
/// };
///
/// // Serialize and deserialize an element (e.g., Int)
/// let mut bytes = Vec::new();
/// int_element.to_bytes(&mut bytes)?;
///
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_element = AnnotationElement::from_bytes(&mut cursor)?;
/// assert_eq!(int_element, deserialized_element);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnnotationElement {
    /// A `byte` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Integer_info` structure which represents the `byte` value.
    Byte { const_value_index: u16 },
    /// A `char` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Integer_info` structure which represents the `char` value.
    Char { const_value_index: u16 },
    /// A `double` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Double_info` structure.
    Double { const_value_index: u16 },
    /// A `float` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Float_info` structure.
    Float { const_value_index: u16 },
    /// An `int` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Integer_info` structure.
    Int { const_value_index: u16 },
    /// A `long` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Long_info` structure.
    Long { const_value_index: u16 },
    /// A `short` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Integer_info` structure
    /// which represents the `short` value.
    Short { const_value_index: u16 },
    /// A `boolean` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Integer_info` structure which represents the `boolean` value (0 for false, 1 for
    /// true).
    Boolean { const_value_index: u16 },
    /// A `String` constant value.
    /// The `const_value_index` is an index into the `constant_pool` table to a
    /// `CONSTANT_Utf8_info` structure.
    String { const_value_index: u16 },
    /// An enum constant value.
    /// `type_name_index` points to a `CONSTANT_Utf8_info` for the enum type's binary name.
    /// `const_name_index` points to a `CONSTANT_Utf8_info` for the enum constant's simple name.
    Enum {
        type_name_index: u16,
        const_name_index: u16,
    },
    /// A class literal value.
    /// `class_info_index` points to a `CONSTANT_Utf8_info` for the return descriptor of the class.
    Class { class_info_index: u16 },
    /// A nested annotation value.
    /// Contains another `Annotation` structure.
    Annotation { annotation: Annotation },
    /// An array value, where each element is an `AnnotationElement`.
    Array { values: Vec<AnnotationElement> },
}

impl AnnotationElement {
    /// Returns the single byte tag that identifies the type of this annotation element.
    ///
    /// Each variant of `AnnotationElement` has a corresponding tag character as defined in the JVM
    /// specification (e.g., 'B' for Byte, 'I' for Int, '[' for Array).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::AnnotationElement;
    ///
    /// let byte_element = AnnotationElement::Byte { const_value_index: 1 };
    /// assert_eq!(byte_element.tag(), b'B');
    ///
    /// let array_element = AnnotationElement::Array { values: vec![] };
    /// assert_eq!(array_element.tag(), b'[');
    /// ```
    #[must_use]
    pub fn tag(&self) -> u8 {
        match self {
            AnnotationElement::Byte { .. } => b'B',
            AnnotationElement::Char { .. } => b'C',
            AnnotationElement::Double { .. } => b'D',
            AnnotationElement::Float { .. } => b'F',
            AnnotationElement::Int { .. } => b'I',
            AnnotationElement::Long { .. } => b'J',
            AnnotationElement::Short { .. } => b'S',
            AnnotationElement::Boolean { .. } => b'Z',
            AnnotationElement::String { .. } => b's',
            AnnotationElement::Enum { .. } => b'e',
            AnnotationElement::Class { .. } => b'c',
            AnnotationElement::Annotation { .. } => b'@',
            AnnotationElement::Array { .. } => b'[',
        }
    }

    /// Deserializes an `AnnotationElement` from a byte stream.
    ///
    /// The method first reads a tag byte to determine the type of the element, then deserializes
    /// the specific data for that element type.
    ///
    /// The `bytes` cursor should be positioned at the start of the `element_value` structure.
    ///
    /// # Errors
    ///
    /// Returns an `Error::InvalidAnnotationElementTag` if the tag byte read from the stream does
    /// not correspond to any known annotation element type. Also propagates I/O errors or errors
    /// from deserializing nested structures (like `Annotation` or inner `AnnotationElement`s for
    /// arrays).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::AnnotationElement;
    /// use std::io::Cursor;
    ///
    /// // Byte representation of AnnotationElement::Int { const_value_index: 100 }
    /// // Tag 'I' (73), const_value_index: 0x0064
    /// let data = vec![b'I', 0, 100];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let element = AnnotationElement::from_bytes(&mut cursor)?;
    /// match element {
    ///     AnnotationElement::Int { const_value_index } => assert_eq!(const_value_index, 100),
    ///     _ => panic!("Deserialized to incorrect type"),
    /// }
    ///
    /// // Example of an invalid tag
    /// let invalid_data = vec![0xFF]; // 0xFF is not a valid tag
    /// let mut cursor = Cursor::new(invalid_data);
    /// assert!(AnnotationElement::from_bytes(&mut cursor).is_err());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<AnnotationElement> {
        let tag = bytes.read_u8()?;

        let element = match tag {
            b'B' => AnnotationElement::Byte {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'C' => AnnotationElement::Char {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'D' => AnnotationElement::Double {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'F' => AnnotationElement::Float {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'I' => AnnotationElement::Int {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'J' => AnnotationElement::Long {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'S' => AnnotationElement::Short {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'Z' => AnnotationElement::Boolean {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b's' => AnnotationElement::String {
                const_value_index: bytes.read_u16::<BigEndian>()?,
            },
            b'e' => AnnotationElement::Enum {
                type_name_index: bytes.read_u16::<BigEndian>()?,
                const_name_index: bytes.read_u16::<BigEndian>()?,
            },
            b'c' => AnnotationElement::Class {
                class_info_index: bytes.read_u16::<BigEndian>()?,
            },
            b'@' => {
                let annotation = Annotation::from_bytes(bytes)?;
                AnnotationElement::Annotation { annotation }
            }
            b'[' => {
                let values_count = bytes.read_u16::<BigEndian>()?;
                let mut values = Vec::with_capacity(values_count as usize);
                for _ in 0..values_count {
                    let value = AnnotationElement::from_bytes(bytes)?;
                    values.push(value);
                }
                AnnotationElement::Array { values }
            }
            _ => return Err(InvalidAnnotationElementTag(tag)),
        };
        Ok(element)
    }

    /// Serializes the `AnnotationElement` to a byte vector.
    ///
    /// This method writes the element's tag byte first, followed by the specific data for the
    /// element's type.
    ///
    /// # Errors
    ///
    /// - If the number of `values` in an `Array` variant exceeds 65,535.
    /// - Propagates I/O errors or errors from serializing nested structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::AnnotationElement;
    ///
    /// let element = AnnotationElement::Boolean { const_value_index: 1 }; // true
    /// let mut bytes = Vec::new();
    /// element.to_bytes(&mut bytes)?;
    ///
    /// // Expected: Tag 'Z' (90), const_value_index: 0x0001
    /// assert_eq!(bytes, vec![b'Z', 0, 1]);
    ///
    /// let array_element = AnnotationElement::Array {
    ///     values: vec![AnnotationElement::Int { const_value_index: 5 }]
    /// };
    /// let mut array_bytes = Vec::new();
    /// array_element.to_bytes(&mut array_bytes)?;
    ///
    /// // Expected: Tag '[' (91), num_values: 0x0001
    /// //           Element 0: Tag 'I' (73), const_value_index: 0x0005
    /// assert_eq!(array_bytes, vec![b'[', 0, 1, b'I', 0, 5]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.tag())?;
        match self {
            AnnotationElement::Byte { const_value_index }
            | AnnotationElement::Char { const_value_index }
            | AnnotationElement::Double { const_value_index }
            | AnnotationElement::Float { const_value_index }
            | AnnotationElement::Int { const_value_index }
            | AnnotationElement::Long { const_value_index }
            | AnnotationElement::Short { const_value_index }
            | AnnotationElement::Boolean { const_value_index }
            | AnnotationElement::String { const_value_index } => {
                bytes.write_u16::<BigEndian>(*const_value_index)?;
            }
            AnnotationElement::Enum {
                type_name_index,
                const_name_index,
            } => {
                bytes.write_u16::<BigEndian>(*type_name_index)?;
                bytes.write_u16::<BigEndian>(*const_name_index)?;
            }
            AnnotationElement::Class { class_info_index } => {
                bytes.write_u16::<BigEndian>(*class_info_index)?;
            }
            AnnotationElement::Annotation { annotation } => {
                annotation.to_bytes(bytes)?;
            }
            AnnotationElement::Array { values } => {
                let values_length = u16::try_from(values.len())?;
                bytes.write_u16::<BigEndian>(values_length)?;
                for value in values {
                    value.to_bytes(bytes)?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for AnnotationElement {
    /// Implements the `Display` trait for `AnnotationElement` to provide a human-readable string
    /// representation of annotation elements.
    ///
    /// This is useful for debugging, logging, and visualization of annotation data in a more
    /// readable format than the debug representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::AnnotationElement;
    ///
    /// let string_element = AnnotationElement::String { const_value_index: 42 };
    ///
    /// let output = string_element.to_string();
    /// assert_eq!(output, "String { const_value_index: 42 }");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnnotationElement::Byte { const_value_index } => {
                write!(f, "Byte {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Char { const_value_index } => {
                write!(f, "Char {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Double { const_value_index } => {
                write!(f, "Double {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Float { const_value_index } => {
                write!(f, "Float {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Int { const_value_index } => {
                write!(f, "Int {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Long { const_value_index } => {
                write!(f, "Long {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Short { const_value_index } => {
                write!(f, "Short {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Boolean { const_value_index } => {
                write!(f, "Boolean {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::String { const_value_index } => {
                write!(f, "String {{ const_value_index: {const_value_index} }}")
            }
            AnnotationElement::Enum {
                type_name_index,
                const_name_index,
            } => write!(
                f,
                "Enum {{ type_name_index: {type_name_index}, const_name_index: {const_name_index} }}"
            ),
            AnnotationElement::Class { class_info_index } => {
                write!(f, "Class {{ class_info_index: {class_info_index} }}")
            }
            AnnotationElement::Annotation { annotation } => {
                write!(f, "Annotation {{ annotation: {annotation:?} }}")
            }
            AnnotationElement::Array { values } => {
                write!(f, "Array {{ values: {values:?} }}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::annotation_value_pair::AnnotationValuePair;

    #[test]
    fn test_invalid_tag() {
        let mut bytes = Cursor::new(vec![0]);
        assert_eq!(
            Err(InvalidAnnotationElementTag(0)),
            AnnotationElement::from_bytes(&mut bytes)
        );
    }

    fn test_element(element: &AnnotationElement, expected_bytes: &[u8], tag: u8) -> Result<()> {
        assert_eq!(tag, element.tag());

        let mut bytes = Vec::new();
        element.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*element, AnnotationElement::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_byte() -> Result<()> {
        let element = AnnotationElement::Byte {
            const_value_index: 42,
        };
        let expected_bytes = [66, 0, 42];

        assert_eq!("Byte { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'B')
    }

    #[test]
    fn test_char() -> Result<()> {
        let element = AnnotationElement::Char {
            const_value_index: 42,
        };
        let expected_bytes = [67, 0, 42];

        assert_eq!("Char { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'C')
    }

    #[test]
    fn test_double() -> Result<()> {
        let element = AnnotationElement::Double {
            const_value_index: 42,
        };
        let expected_bytes = [68, 0, 42];

        assert_eq!("Double { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'D')
    }

    #[test]
    fn test_float() -> Result<()> {
        let element = AnnotationElement::Float {
            const_value_index: 42,
        };
        let expected_bytes = [70, 0, 42];

        assert_eq!("Float { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'F')
    }

    #[test]
    fn test_int() -> Result<()> {
        let element = AnnotationElement::Int {
            const_value_index: 42,
        };
        let expected_bytes = [73, 0, 42];

        assert_eq!("Int { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'I')
    }

    #[test]
    fn test_long() -> Result<()> {
        let element = AnnotationElement::Long {
            const_value_index: 42,
        };
        let expected_bytes = [74, 0, 42];

        assert_eq!("Long { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'J')
    }

    #[test]
    fn test_short() -> Result<()> {
        let element = AnnotationElement::Short {
            const_value_index: 42,
        };
        let expected_bytes = [83, 0, 42];

        assert_eq!("Short { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'S')
    }

    #[test]
    fn test_boolean() -> Result<()> {
        let element = AnnotationElement::Boolean {
            const_value_index: 42,
        };
        let expected_bytes = [90, 0, 42];

        assert_eq!("Boolean { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'Z')
    }

    #[test]
    fn test_string() -> Result<()> {
        let element = AnnotationElement::String {
            const_value_index: 42,
        };
        let expected_bytes = [115, 0, 42];

        assert_eq!("String { const_value_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b's')
    }

    #[test]
    fn test_enum() -> Result<()> {
        let element = AnnotationElement::Enum {
            type_name_index: 3,
            const_name_index: 42,
        };
        let expected_bytes = [101, 0, 3, 0, 42];

        assert_eq!(
            "Enum { type_name_index: 3, const_name_index: 42 }",
            element.to_string()
        );
        test_element(&element, &expected_bytes, b'e')
    }

    #[test]
    fn test_class() -> Result<()> {
        let element = AnnotationElement::Class {
            class_info_index: 42,
        };
        let expected_bytes = [99, 0, 42];

        assert_eq!("Class { class_info_index: 42 }", element.to_string());
        test_element(&element, &expected_bytes, b'c')
    }

    #[test]
    fn test_annotation() -> Result<()> {
        let element = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![element],
        };
        let element = AnnotationElement::Annotation { annotation };
        let expected_bytes = [64, 0, 3, 0, 1, 0, 1, 66, 0, 42];

        assert_eq!(
            "Annotation { annotation: Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] } }",
            element.to_string()
        );
        test_element(&element, &expected_bytes, b'@')
    }

    #[test]
    fn test_array() -> Result<()> {
        let element = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![element],
        };
        let values = vec![AnnotationElement::Annotation { annotation }];
        let element = AnnotationElement::Array { values };
        let expected_bytes = [91, 0, 1, 64, 0, 3, 0, 1, 0, 1, 66, 0, 42];

        assert_eq!(
            "Array { values: [Annotation { annotation: Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] } }] }",
            element.to_string()
        );
        test_element(&element, &expected_bytes, b'[')
    }
}
