use crate::attributes::AnnotationElement;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of an annotation value pair.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.16>
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationValuePair {
    pub name_index: u16,
    pub value: AnnotationElement,
}

impl AnnotationValuePair {
    /// Deserialize the annotation value pair from bytes.
    ///
    /// # Errors
    /// If the annotation element cannot be deserialized.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<AnnotationValuePair> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let value = AnnotationElement::from_bytes(bytes)?;
        let annotation_value_pair = AnnotationValuePair { name_index, value };

        Ok(annotation_value_pair)
    }

    /// Serialize the annotation element to bytes.
    ///
    /// # Errors
    /// If the annotation element cannot be serialized.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.value.to_bytes(bytes)
    }
}

impl fmt::Display for AnnotationValuePair {
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
                const_value_index: 42,
            },
        };
        let expected_bytes = [0, 1, 66, 0, 42];

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
