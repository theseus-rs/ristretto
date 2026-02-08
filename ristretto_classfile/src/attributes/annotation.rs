use crate::attributes::annotation_value_pair::AnnotationValuePair;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a single annotation in a class file.
///
/// Annotations are a form of metadata that can be added to Java code. They are read by the JVM at
/// runtime and can be used to alter program behavior or provide additional information.
///
/// See the [JVMS §4.1.16](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16)
/// for more details on the `RuntimeVisibleAnnotations` attribute and the `annotation` structure.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{Annotation, AnnotationValuePair, AnnotationElement};
/// use std::io::Cursor;
///
/// // Create an annotation
/// let annotation = Annotation {
///     type_index: 10, // Index into the constant pool for the annotation type
///     elements: vec![
///         AnnotationValuePair {
///             name_index: 5, // Index into the constant pool for the element name
///             value: AnnotationElement::Int { const_value_index: 12 }
///         }
///     ]
/// };
///
/// // Serialize the annotation to bytes
/// let mut bytes = Vec::new();
/// annotation.to_bytes(&mut bytes)?;
///
/// // Deserialize the annotation from bytes
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_annotation = Annotation::from_bytes(&mut cursor)?;
///
/// assert_eq!(annotation, deserialized_annotation);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    /// The `type_index` item is an unsigned short. The value of the `type_index` item must be a
    /// valid index into the `constant_pool` table. The `constant_pool` entry at that index must be
    /// a `CONSTANT_Utf8_info` structure representing a field descriptor. This field descriptor
    /// denotes the type of the annotation.
    pub type_index: u16,
    /// Each value of the `element_value_pairs` table represents a single element-value pair in the
    /// annotation.
    pub elements: Vec<AnnotationValuePair>,
}

impl Annotation {
    /// Deserializes an `Annotation` structure from a byte stream.
    ///
    /// The `bytes` cursor should be positioned at the start of the `annotation` structure.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails or if the data
    /// is not a valid `Annotation` structure (e.g., incorrect element count).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Annotation, AnnotationValuePair, AnnotationElement};
    /// use std::io::Cursor;
    ///
    /// // Create a byte array representing an annotation
    /// let annotation_bytes = vec![
    ///     0, 10,             // type_index: 10
    ///     0, 1,              // num_elements: 1
    ///     0, 5,              // element name_index: 5
    ///     b'I',              // element tag: 'I' (integer)
    ///     0, 12              // const_value_index: 12
    /// ];
    ///
    /// // Deserialize the annotation from bytes
    /// let mut cursor = Cursor::new(annotation_bytes);
    /// let annotation = Annotation::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(annotation.type_index, 10);
    /// assert_eq!(annotation.elements.len(), 1);
    /// assert_eq!(annotation.elements[0].name_index, 5);
    ///
    /// // Check the value is an integer with the correct const_value_index
    /// if let AnnotationElement::Int { const_value_index } = &annotation.elements[0].value {
    ///     assert_eq!(*const_value_index, 12);
    /// } else {
    ///     panic!("Expected Int element");
    /// }
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<Annotation> {
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

    /// Serializes the `Annotation` structure to a byte vector.
    ///
    /// # Errors
    ///
    /// - If the number of `elements` exceeds 65,535 (the maximum for a `u16`).
    /// - If serializing any of its `AnnotationValuePair` elements fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Annotation, AnnotationValuePair, AnnotationElement};
    ///
    /// // Create an annotation
    /// let annotation = Annotation {
    ///     type_index: 10, // Index into the constant pool for the annotation type
    ///     elements: vec![
    ///         AnnotationValuePair {
    ///             name_index: 5, // Index into the constant pool for the element name
    ///             value: AnnotationElement::Int { const_value_index: 12 }
    ///         }
    ///     ]
    /// };
    ///
    /// // Serialize the annotation to bytes
    /// let mut bytes = Vec::new();
    /// annotation.to_bytes(&mut bytes)?;
    ///
    /// // The resulting bytes will contain:
    /// // - type_index (2 bytes): 0, 10
    /// // - num_elements (2 bytes): 0, 1
    /// // - For each element:
    /// //   - name_index (2 bytes): 0, 5
    /// //   - tag (1 byte): 'I' (73 in ASCII for Int)
    /// //   - const_value_index (2 bytes): 0, 12
    ///
    /// assert_eq!(bytes, vec![0, 10, 0, 1, 0, 5, 73, 0, 12]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    /// Implements the `Display` trait for `Annotation`.
    ///
    /// This provides a human-readable text representation of the annotation, showing the type index
    /// and elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Annotation, AnnotationValuePair, AnnotationElement};
    ///
    /// let annotation = Annotation {
    ///     type_index: 10,
    ///     elements: vec![
    ///         AnnotationValuePair {
    ///             name_index: 5,
    ///             value: AnnotationElement::Int { const_value_index: 12 }
    ///         }
    ///     ]
    /// };
    ///
    /// // Use the Display implementation directly with to_string()
    /// let output = annotation.to_string();
    /// assert_eq!(
    ///     output,
    ///     "type_index: 10, elements: [AnnotationValuePair { name_index: 5, value: Int { const_value_index: 12 } }]",
    /// );
    /// ```
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

        assert_eq!(
            "type_index: 3, elements: [AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }]",
            annotation.to_string()
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
        let expected_value = [0, 3, 0, 1, 0, 1, 66, 0, 42];

        let mut bytes = Vec::new();
        annotation.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(annotation, Annotation::from_bytes(&mut bytes)?);
        Ok(())
    }
}
