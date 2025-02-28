use crate::attributes::Annotation;
use crate::error::Error::InvalidAnnotationElementTag;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `AnnotationElement`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.16>
#[derive(Clone, Debug, PartialEq)]
pub enum AnnotationElement {
    Byte {
        const_value_index: u16,
    },
    Char {
        const_value_index: u16,
    },
    Double {
        const_value_index: u16,
    },
    Float {
        const_value_index: u16,
    },
    Int {
        const_value_index: u16,
    },
    Long {
        const_value_index: u16,
    },
    Short {
        const_value_index: u16,
    },
    Boolean {
        const_value_index: u16,
    },
    String {
        const_value_index: u16,
    },
    Enum {
        type_name_index: u16,
        const_name_index: u16,
    },
    Class {
        class_info_index: u16,
    },
    Annotation {
        annotation: Annotation,
    },
    Array {
        values: Vec<AnnotationElement>,
    },
}

impl AnnotationElement {
    /// Return the tag for the annotation element.
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

    /// Deserialize the annotation element from bytes.
    ///
    /// # Errors
    /// Returns an error if the tag is invalid.
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

    /// Serialize the annotation element to bytes.
    ///
    /// # Errors
    /// If there are more than 65,534 values in the array.
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
