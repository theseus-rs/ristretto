use crate::attributes::{AnnotationValuePair, TargetPath, TargetType};
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of a type annotation, which provides information about types in a class file,
/// such as annotations on generic type arguments or array components.
///
/// Type annotations extend the Java annotation system to allow annotations on any use of a type,
/// including generic type arguments, method return types, throws clauses, type casts, and more.
/// They were introduced in Java 8 as part of JSR 308.
///
/// # Structure
///
/// A `TypeAnnotation` consists of:
/// - `target_type`: Specifies what kind of program element is annotated.
/// - `type_path`: Specifies which part of the type is annotated (e.g., a type argument of a generic
///   type).
/// - `type_index`: An index into the constant pool that points to a `CONSTANT_Utf8_info`
///   structure representing the annotation type.
/// - `elements`: A list of `AnnotationValuePair`s, representing the annotation's key-value pairs.
///
/// # Common Target Types
///
/// - Class type parameter declarations (0x00)
/// - Method type parameter declarations (0x01)
/// - Class extends/implements clauses (0x10)
/// - Method return types (0x14)
/// - Method parameter types (0x15)
/// - Field types (0x16)
/// - Local variable types (0x40)
/// - Type casts (0x47)
/// - Type test (instanceof) (0x48)
///
/// # Examples
///
/// This example creates a type annotation that might represent a `@NotNull` annotation
/// on a field type:
///
/// ```rust
/// use ristretto_classfile::attributes::{
///     AnnotationElement, AnnotationValuePair, TargetPath, TargetType, TypeAnnotation
/// };
///
/// let element = AnnotationValuePair {
///     name_index: 1,
///     value: AnnotationElement::Byte {
///         const_value_index: 42, // Constant pool index 42 points to a byte value
///     },
/// };
/// let type_annotation = TypeAnnotation {
///     target_type: TargetType::Empty { target_type: 19 },
///     type_path: vec![TargetPath {
///         type_path_kind: 1,
///         type_argument_index: 2,
///     }],
///     type_index: 42,
///     elements: vec![element],
/// };
///
/// let mut bytes = Vec::new();
/// type_annotation.to_bytes(&mut bytes)?;
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVMS §4.7.20](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeAnnotation {
    pub target_type: TargetType,
    pub type_path: Vec<TargetPath>,
    pub type_index: u16,
    pub elements: Vec<AnnotationValuePair>,
}

impl TypeAnnotation {
    /// Deserialize the type annotation from bytes.
    ///
    /// The byte stream is read according to the JVM specification for type annotations.
    ///
    /// # Errors
    ///
    /// - If the target type fails to deserialize.
    /// - If reading any part of the type annotation (type path, type index, elements) fails,
    ///   for example due to an unexpected end of the byte stream or invalid data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{
    ///     AnnotationElement, AnnotationValuePair, TargetPath, TargetType, TypeAnnotation,
    /// };
    /// use std::io::Cursor;
    ///
    /// // These bytes represent a serialized TypeAnnotation
    /// let bytes = vec![
    ///     0x00, 0x00,       // target_type (0x00), type_parameter_index (0)
    ///     0x01,             // type_path_length
    ///     0x00, 0x00,       // type_path[0] (kind 0, index 0)
    ///     0x00, 0x0A,       // type_index (10)
    ///     0x00, 0x01,       // num_element_value_pairs
    ///     0x00, 0x0B,       // element_name_index (11)
    ///     b's',             // tag for Utf8
    ///     0x00, 0x0C,       // const_value_index (12)
    /// ];
    ///
    /// let mut cursor = Cursor::new(bytes);
    /// let type_annotation = TypeAnnotation::from_bytes(&mut cursor)?;
    ///
    /// // The deserialized TypeAnnotation should match this expected value
    /// let expected = TypeAnnotation {
    ///     target_type: TargetType::TypeParameter { target_type: 0x00, type_parameter_index: 0 },
    ///     type_path: vec![TargetPath {
    ///         type_path_kind: 0,
    ///         type_argument_index: 0,
    ///     }],
    ///     type_index: 10,
    ///     elements: vec![AnnotationValuePair {
    ///         name_index: 11,
    ///         value: AnnotationElement::String { const_value_index: 12 },
    ///     }],
    /// };
    /// assert_eq!(type_annotation, expected);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<TypeAnnotation> {
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
    /// The type annotation is written to the byte vector according to the JVM specification.
    ///
    /// # Errors
    ///
    /// - If an annotation element fails to serialize.
    /// - If the number of type path entries or elements exceeds `u8::MAX` or `u16::MAX` respectively.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{
    ///     AnnotationElement, AnnotationValuePair, TargetPath, TargetType, TypeAnnotation,
    /// };
    ///
    /// let type_annotation = TypeAnnotation {
    ///     target_type: TargetType::TypeParameter { target_type: 0x00, type_parameter_index: 0 },
    ///     type_path: vec![TargetPath {
    ///         type_path_kind: 0,
    ///         type_argument_index: 0,
    ///     }],
    ///     type_index: 10,
    ///     elements: vec![AnnotationValuePair {
    ///         name_index: 11,
    ///         value: AnnotationElement::String { const_value_index: 12 },
    ///     }],
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// type_annotation.to_bytes(&mut bytes)?;
    ///
    /// // Expected byte representation (matches the from_bytes example)
    /// let expected_bytes = vec![
    ///     0x00, 0x00,       // target_type (0x00), type_parameter_index (0)
    ///     0x01,             // type_path_length
    ///     0x00, 0x00,       // type_path[0] (kind 0, index 0)
    ///     0x00, 0x0A,       // type_index (10)
    ///     0x00, 0x01,       // num_element_value_pairs
    ///     0x00, 0x0B,       // element_name_index (11)
    ///     b's',             // tag for Utf8
    ///     0x00, 0x0C,       // const_value_index (12)
    /// ];
    /// assert_eq!(bytes, expected_bytes);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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

impl fmt::Display for TypeAnnotation {
    /// Formats the `TypeAnnotation` for display purposes.
    ///
    /// This implementation creates a string representation of the type annotation including its
    /// target type, type path, type index, and elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{
    ///     AnnotationElement, AnnotationValuePair, TargetPath, TargetType, TypeAnnotation
    /// };
    ///
    /// let type_annotation = TypeAnnotation {
    ///     target_type: TargetType::Empty { target_type: 19 },
    ///     type_path: vec![TargetPath {
    ///         type_path_kind: 1,
    ///         type_argument_index: 2,
    ///     }],
    ///     type_index: 42,
    ///     elements: vec![AnnotationValuePair {
    ///         name_index: 1,
    ///         value: AnnotationElement::Byte { const_value_index: 42 },
    ///     }],
    /// };
    ///
    /// let output = type_annotation.to_string();
    /// assert_eq!(
    ///     output,
    ///     "TypeAnnotation[target_type=Empty[target_type=19], type_path=[TargetPath { type_path_kind: 1, type_argument_index: 2 }], type_index=42, elements=[AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }]]"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TypeAnnotation[target_type={}, type_path={:?}, type_index={}, elements={:?}]",
            self.target_type, self.type_path, self.type_index, self.elements
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::{AnnotationElement, AnnotationValuePair};

    #[test]
    fn test_to_string() {
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
        assert_eq!(
            "TypeAnnotation[target_type=Empty[target_type=19], type_path=[TargetPath { type_path_kind: 1, type_argument_index: 2 }], type_index=42, elements=[AnnotationValuePair { name_index: 1, value: Byte { const_value_index: 42 } }]]",
            type_annotation.to_string()
        );
    }

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
