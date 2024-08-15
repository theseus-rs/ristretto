use crate::attributes::annotation_value_pair::AnnotationValuePair;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of Annotation.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.16>
#[derive(Clone, Debug, PartialEq)]
pub struct Annotation {
    pub type_index: u16,
    pub elements: Vec<AnnotationValuePair>,
}

impl Annotation {
    /// Deserialize the annotation from bytes.
    ///
    /// # Errors
    /// If there is an issue deserializing an `AnnotationValuePair`.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Annotation> {
        let type_index = bytes.read_u16::<BigEndian>()?;
        let elements_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut elements = Vec::with_capacity(elements_count);
        for _ in 0..elements_count {
            let annotation_element = AnnotationValuePair::from_bytes(bytes)?;
            elements.push(annotation_element);
        }
        let annotation = Annotation {
            type_index,
            elements,
        };
        Ok(annotation)
    }

    /// Serialize the annotation to bytes.
    ///
    /// # Errors
    /// - If there are more than 65,534 elements.
    /// - If there is an issue serializing an `AnnotationValuePair`.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.type_index)?;

        let elements_length = u16::try_from(self.elements.len())?;
        bytes.write_u16::<BigEndian>(elements_length)?;
        for element in &self.elements {
            element.to_bytes(bytes)?;
        }
        Ok(())
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "type_index: {}, elements: {:?}",
            self.type_index, self.elements
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::AnnotationElement;

    #[test_log::test]
    fn test_to_string() {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![annotation_value_pair],
        };

        assert_eq!(
            "type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }]",
            annotation.to_string()
        );
    }

    #[test_log::test]
    fn test_serialization() -> Result<()> {
        let annotation_value_pair = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let annotation = Annotation {
            type_index: 3,
            elements: vec![annotation_value_pair],
        };
        let expected_value = [0, 3, 0, 1, 0, 1, 66, 0, 42];

        let mut bytes = Vec::new();
        annotation.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(annotation, Annotation::from_bytes(&mut bytes)?);
        Ok(())
    }
}
