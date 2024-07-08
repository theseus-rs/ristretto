use crate::attributes::Annotation;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of a parameter annotation.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.18>
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterAnnotation {
    pub annotations: Vec<Annotation>,
}

impl ParameterAnnotation {
    /// Deserialize the parameter annotation from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ParameterAnnotation> {
        let annotations_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut annotations = Vec::with_capacity(annotations_count);
        for _ in 0..annotations_count {
            let annotation = Annotation::from_bytes(bytes)?;
            annotations.push(annotation);
        }

        let parameter_annotation = ParameterAnnotation { annotations };

        Ok(parameter_annotation)
    }

    /// Serialize the parameter annotation to bytes.
    ///
    /// # Errors
    /// - If the number of annotations exceeds 65,535.
    /// - If an annotation fails to serialize.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        let annotations_length = u16::try_from(self.annotations.len())?;
        bytes.write_u16::<BigEndian>(annotations_length)?;
        for annotation in &self.annotations {
            annotation.to_bytes(bytes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::{AnnotationElement, AnnotationValuePair};

    #[test]
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
        let parameter_annotation = ParameterAnnotation {
            annotations: vec![annotation],
        };
        let expected_bytes = [0, 1, 0, 3, 0, 1, 0, 1, 66, 0, 42];

        let mut bytes = Vec::new();
        parameter_annotation.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(
            parameter_annotation,
            ParameterAnnotation::from_bytes(&mut bytes)?
        );
        Ok(())
    }
}
