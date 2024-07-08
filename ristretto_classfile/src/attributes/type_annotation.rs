use crate::attributes::{AnnotationValuePair, TargetPath, TargetType};
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of a type annotation.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.20>
#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub target_type: TargetType,
    pub type_path: Vec<TargetPath>,
    pub type_index: u16,
    pub elements: Vec<AnnotationValuePair>,
}

impl TypeAnnotation {
    /// Deserialize the type annotation from bytes.
    ///
    /// # Errors
    /// - If the target type fails to deserialize.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<TypeAnnotation> {
        let target_type = TargetType::from_bytes(bytes)?;

        let type_path_count = bytes.read_u8()? as usize;
        let mut type_path = Vec::with_capacity(type_path_count);
        for _ in 0..type_path_count {
            let target_path = TargetPath::from_bytes(bytes)?;
            type_path.push(target_path);
        }

        let type_index = bytes.read_u16::<BigEndian>()?;

        let elements_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut elements = Vec::with_capacity(elements_count);
        for _ in 0..elements_count {
            let element = AnnotationValuePair::from_bytes(bytes)?;
            elements.push(element);
        }

        let type_annotation = TypeAnnotation {
            target_type,
            type_path,
            type_index,
            elements,
        };

        Ok(type_annotation)
    }

    /// Serialize the type annotation to bytes.
    ///
    /// # Errors
    /// If an annotation fails to serialize.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        self.target_type.to_bytes(bytes)?;

        let type_path_count = u8::try_from(self.type_path.len())?;
        bytes.write_u8(type_path_count)?;
        for target_path in &self.type_path {
            target_path.to_bytes(bytes)?;
        }

        bytes.write_u16::<BigEndian>(self.type_index)?;

        let elements_length = u16::try_from(self.elements.len())?;
        bytes.write_u16::<BigEndian>(elements_length)?;
        for element in &self.elements {
            element.to_bytes(bytes)?;
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
        let element = AnnotationValuePair {
            name_index: 1,
            value: AnnotationElement::Byte {
                const_value_index: 42,
            },
        };
        let type_annotation = TypeAnnotation {
            target_type: TargetType::Empty { target_type: 19 },
            type_path: vec![TargetPath {
                type_path_kind: 1,
                type_argument_index: 2,
            }],
            type_index: 42,
            elements: vec![element],
        };
        let expected_bytes = [19, 1, 1, 2, 0, 42, 0, 1, 0, 1, 66, 0, 42];

        let mut bytes = Vec::new();
        type_annotation.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(type_annotation, TypeAnnotation::from_bytes(&mut bytes)?);
        Ok(())
    }
}
