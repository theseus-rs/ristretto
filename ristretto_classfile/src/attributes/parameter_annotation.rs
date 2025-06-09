use crate::attributes::Annotation;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of a parameter annotation.
///
/// A `ParameterAnnotation` represents a collection of annotations on a single parameter of a method
/// in the Java class file format. It is used in the `RuntimeVisibleParameterAnnotations` and
/// `RuntimeInvisibleParameterAnnotations` attributes, where each parameter of a method can have
/// multiple annotations.
///
/// Each parameter annotation contains a list of individual `Annotation` instances
/// that are applied to that specific parameter.
///
/// # Structure
///
/// The binary format of a parameter annotation consists of:
/// - A 2-byte `num_annotations` field indicating the number of annotations
/// - A series of annotation structures
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{
///     Annotation, AnnotationElement, AnnotationValuePair, ParameterAnnotation
/// };
///
/// // Create a parameter annotation for a method parameter
/// let parameter_annotation = ParameterAnnotation {
///     annotations: vec![
///         // Annotation with type index 3 (e.g., "Ljavax/annotation/Nonnull;")
///         Annotation {
///             type_index: 3,
///             elements: vec![
///                 // Element with name "value" (assuming name_index 1 points to "value")
///                 AnnotationValuePair {
///                     name_index: 1,
///                     value: AnnotationElement::Byte { const_value_index: 42 },
///                 }
///             ],
///         }
///     ],
/// };
///
/// // You can serialize/deserialize this to/from bytes
/// let mut bytes = Vec::new();
/// parameter_annotation.to_bytes(&mut bytes)?;
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.18](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.18)
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterAnnotation {
    pub annotations: Vec<Annotation>,
}

impl ParameterAnnotation {
    /// Deserialize the parameter annotation from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Annotation, AnnotationElement, AnnotationValuePair, ParameterAnnotation};
    /// use std::io::Cursor;
    ///
    /// // annotations: [Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }]
    /// let mut bytes = Cursor::new(vec![0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x42, 0x00, 0x2A]);
    /// let parameter_annotation = ParameterAnnotation::from_bytes(&mut bytes)?;
    /// assert_eq!(parameter_annotation.annotations.len(), 1);
    /// let annotation = &parameter_annotation.annotations[0];
    /// assert_eq!(annotation.type_index, 3);
    /// assert_eq!(annotation.elements.len(), 1);
    /// let element = &annotation.elements[0];
    /// assert_eq!(element.name_index, 1);
    /// assert_eq!(element.value, AnnotationElement::Byte { const_value_index: 42 });
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    ///
    /// - If the number of annotations exceeds 65,534.
    /// - If an annotation fails to serialize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Annotation, AnnotationElement, AnnotationValuePair, ParameterAnnotation};
    ///
    /// let parameter_annotation = ParameterAnnotation {
    ///     annotations: vec![Annotation {
    ///         type_index: 3,
    ///         elements: vec![AnnotationValuePair {
    ///             name_index: 1,
    ///             value: AnnotationElement::Byte { const_value_index: 42 },
    ///         }],
    ///     }],
    /// };
    /// let mut bytes = Vec::new();
    /// parameter_annotation.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x42, 0x00, 0x2A]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        let annotations_length = u16::try_from(self.annotations.len())?;
        bytes.write_u16::<BigEndian>(annotations_length)?;
        for annotation in &self.annotations {
            annotation.to_bytes(bytes)?;
        }

        Ok(())
    }
}

impl fmt::Display for ParameterAnnotation {
    /// Implements the Display trait for `ParameterAnnotation`.
    ///
    /// This formats the parameter annotation in a human-readable string representation showing the
    /// contained annotations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{
    ///     Annotation, AnnotationElement, AnnotationValuePair, ParameterAnnotation
    /// };
    ///
    /// let parameter_annotation = ParameterAnnotation {
    ///     annotations: vec![Annotation {
    ///         type_index: 3,
    ///         elements: vec![AnnotationValuePair {
    ///             name_index: 1,
    ///             value: AnnotationElement::Byte { const_value_index: 42 },
    ///         }],
    ///     }],
    /// };
    ///
    /// let output = parameter_annotation.to_string();
    /// assert_eq!(
    ///     output,
    ///     "ParameterAnnotation[annotations=[Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }]]",
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParameterAnnotation[annotations={:?}]", self.annotations)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::{AnnotationElement, AnnotationValuePair};

    #[test]
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
        let parameter_annotation = ParameterAnnotation {
            annotations: vec![annotation],
        };
        assert_eq!(
            "ParameterAnnotation[annotations=[Annotation { type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }] }]]",
            parameter_annotation.to_string()
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
