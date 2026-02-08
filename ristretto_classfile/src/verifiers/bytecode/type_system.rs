//! # JVM Verification Type System
//!
//! This module implements a rigorous JVM type system for bytecode verification
//! according to [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2) (Type Checking Verification).
//!
//! The type system supports all verification types:
//! - Top: The universal supertype (used for second slot of category 2 types)
//! - Integer: Represents int, boolean, byte, char, short
//! - Float: Single precision floating point
//! - Long: 64-bit integer (category 2)
//! - Double: Double precision floating point (category 2)
//! - Null: The null reference type
//! - `UninitializedThis`: Reference to 'this' before superclass constructor is called
//! - Uninitialized(offset): Reference to a newly created but uninitialized object
//! - `Object(class_name)`: Reference to an instance of a class
//! - `Array(component_type)`: Reference to an array with specific component type
//!
//! # References
//!
//! - [JVMS §4.10.1.2 - Verification Type System](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2)

use std::fmt::Display;
use std::sync::Arc;

use crate::verifiers::context::VerificationContext;
use crate::verifiers::error::{Result, VerifyError};
use crate::{BaseType, FieldType};

/// Represents the verification types used in the JVM bytecode verification process.
///
/// These types correspond to the types defined in [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2) for stack map frames.
/// The type system is designed for strict type checking during bytecode verification.
///
/// # Type Categories
///
/// Types are divided into two categories:
/// - **Category 1**: Takes one slot in locals/stack (`Integer`, `Float`, `Object`, etc.)
/// - **Category 2**: Takes two slots (`Long`, `Double`)
///
/// For category 2 types, the second slot is represented by `Top`.
///
/// # References
///
/// - [JVMS §4.10.1.2 - Verification Type System](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2)
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum VerificationType {
    /// Top type, used for:
    /// - The second slot of `Long` and `Double` values
    /// - Represents an unusable type in the type lattice
    ///
    /// # JVMS Reference
    /// Top is the universal supertype; it can absorb any type during merging.
    #[default]
    Top,

    /// Integer type - represents all JVM integer types that use a single stack slot:
    /// - `int`
    /// - `boolean`
    /// - `byte`
    /// - `char`
    /// - `short`
    ///
    /// # JVMS Reference
    /// All these types are treated as `int` for verification purposes.
    Integer,

    /// Float type - single precision floating point.
    Float,

    /// Long type - 64-bit integer (category 2).
    ///
    /// Takes two slots in locals/stack; the second slot must be `Top`.
    Long,

    /// Double type - double precision floating point (category 2).
    ///
    /// Takes two slots in locals/stack; the second slot must be `Top`.
    Double,

    /// Null type - the type of the null reference.
    ///
    /// Assignable to any reference type (Object or Array).
    Null,

    /// Uninitialized "this" type - used in constructors.
    ///
    /// Represents `this` before the superclass constructor has been invoked.
    /// After constructor invocation, becomes `Object(class_name)`.
    UninitializedThis,

    /// Uninitialized type - represents a newly created but uninitialized object.
    ///
    /// The `u16` is the bytecode offset of the `new` instruction that created this object.
    /// This allows tracking which uninitialized reference becomes which initialized type.
    Uninitialized(u16),

    /// Object reference type.
    ///
    /// Contains the internal class name (e.g., "java/lang/String").
    /// For arrays represented as Object, the name is the array descriptor.
    Object(Arc<str>),

    /// Array reference type with explicit component type.
    ///
    /// This provides proper array type checking including:
    /// - Covariance: `String[]` is a subtype of `Object[]`
    /// - Common interfaces: All arrays implement `Cloneable` and `Serializable`
    /// - Primitive arrays: `int[]`, `byte[]`, etc.
    Array(Box<VerificationType>),
}

impl VerificationType {
    /// Creates the `java/lang/Object` type.
    #[inline]
    #[must_use]
    pub fn java_lang_object() -> Self {
        Self::Object(Arc::from("java/lang/Object"))
    }

    /// Creates the `java/lang/String` type.
    #[inline]
    #[must_use]
    pub fn java_lang_string() -> Self {
        Self::Object(Arc::from("java/lang/String"))
    }

    /// Creates the `java/lang/Class` type.
    #[inline]
    #[must_use]
    pub fn java_lang_class() -> Self {
        Self::Object(Arc::from("java/lang/Class"))
    }

    /// Creates the `java/lang/Throwable` type.
    #[inline]
    #[must_use]
    pub fn java_lang_throwable() -> Self {
        Self::Object(Arc::from("java/lang/Throwable"))
    }

    /// Creates the `java/lang/Cloneable` type.
    #[inline]
    #[must_use]
    pub fn java_lang_cloneable() -> Self {
        Self::Object(Arc::from("java/lang/Cloneable"))
    }

    /// Creates the `java/io/Serializable` type.
    #[inline]
    #[must_use]
    pub fn java_io_serializable() -> Self {
        Self::Object(Arc::from("java/io/Serializable"))
    }

    /// Returns `true` if this is a category 1 type (takes one slot).
    #[inline]
    #[must_use]
    pub const fn is_category1(&self) -> bool {
        !matches!(self, Self::Long | Self::Double)
    }

    /// Returns `true` if this is a category 2 type (takes two slots).
    #[inline]
    #[must_use]
    pub const fn is_category2(&self) -> bool {
        matches!(self, Self::Long | Self::Double)
    }

    /// Returns `true` if this is a reference type (Object, Array, Null, or Uninitialized).
    #[inline]
    #[must_use]
    pub const fn is_reference(&self) -> bool {
        matches!(
            self,
            Self::Object(_)
                | Self::Array(_)
                | Self::Null
                | Self::Uninitialized(_)
                | Self::UninitializedThis
        )
    }

    /// Returns `true` if this is an initialized reference type.
    #[inline]
    #[must_use]
    pub const fn is_initialized_reference(&self) -> bool {
        matches!(self, Self::Object(_) | Self::Array(_) | Self::Null)
    }

    /// Returns `true` if this is an uninitialized type.
    #[inline]
    #[must_use]
    pub const fn is_uninitialized(&self) -> bool {
        matches!(self, Self::Uninitialized(_) | Self::UninitializedThis)
    }

    /// Returns `true` if this is an array type.
    #[inline]
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns `true` if this is an integer type (int, boolean, byte, char, short).
    #[inline]
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer)
    }

    /// Returns `true` if this is the Top type.
    #[inline]
    #[must_use]
    pub const fn is_top(&self) -> bool {
        matches!(self, Self::Top)
    }

    /// Returns `true` if this is the Null type.
    #[inline]
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Gets the component type if this is an array.
    ///
    /// Returns `None` if not an array type.
    #[must_use]
    pub fn component_type(&self) -> Option<&VerificationType> {
        match self {
            Self::Array(component) => Some(component.as_ref()),
            _ => None,
        }
    }

    /// Gets the array dimension depth.
    ///
    /// Returns 0 for non-array types.
    #[must_use]
    pub fn array_dimensions(&self) -> usize {
        match self {
            Self::Array(component) => 1 + component.array_dimensions(),
            _ => 0,
        }
    }

    /// Gets the innermost element type of an array.
    ///
    /// For `int[][][]`, returns `Integer`.
    /// For non-arrays, returns `self`.
    #[must_use]
    pub fn element_type(&self) -> &VerificationType {
        match self {
            Self::Array(component) => component.element_type(),
            _ => self,
        }
    }

    /// Creates an array type with this type as the component.
    #[must_use]
    pub fn make_array(self) -> Self {
        Self::Array(Box::new(self))
    }

    /// Converts a `FieldType` to a `VerificationType`.
    ///
    /// This is used when parsing method descriptors to determine parameter and return types.
    ///
    /// # Arguments
    ///
    /// * `field_type` - The field type from a descriptor
    ///
    /// # Returns
    ///
    /// The corresponding verification type.
    #[must_use]
    pub fn from_field_type(field_type: &FieldType) -> Self {
        match field_type {
            FieldType::Base(base) => match base {
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Short
                | BaseType::Char
                | BaseType::Int => Self::Integer,
                BaseType::Float => Self::Float,
                BaseType::Long => Self::Long,
                BaseType::Double => Self::Double,
            },
            FieldType::Object(class_name) => Self::Object(Arc::from(class_name.as_str())),
            FieldType::Array(component) => Self::Array(Box::new(Self::from_field_type(component))),
        }
    }

    /// Converts a base type code to a `VerificationType`.
    ///
    /// Used for `newarray` instruction type codes.
    ///
    /// # Arguments
    ///
    /// * `atype` - The array type code (4-11)
    ///
    /// # Returns
    ///
    /// The corresponding verification type for the array component.
    ///
    /// # Errors
    ///
    /// Returns an error if the type code is invalid.
    pub fn from_array_type_code(atype: u8) -> Result<Self> {
        match atype {
            4 | 5 | 8 | 9 | 10 => Ok(Self::Integer), // T_BOOLEAN, T_CHAR, T_BYTE, T_SHORT, T_INT
            6 => Ok(Self::Float),                    // T_FLOAT
            7 => Ok(Self::Double),                   // T_DOUBLE
            11 => Ok(Self::Long),                    // T_LONG
            _ => Err(VerifyError::InvalidArrayTypeCode(atype)),
        }
    }

    /// Converts to a field type descriptor string.
    ///
    /// This is the inverse of `from_field_type`.
    #[must_use]
    pub fn to_descriptor(&self) -> Option<String> {
        match self {
            Self::Integer => Some("I".to_string()),
            Self::Float => Some("F".to_string()),
            Self::Long => Some("J".to_string()),
            Self::Double => Some("D".to_string()),
            Self::Object(name) => Some(format!("L{name};")),
            Self::Array(component) => component.to_descriptor().map(|desc| format!("[{desc}")),
            _ => None,
        }
    }

    /// Checks if this type is assignable to the target type.
    ///
    /// Implements the type assignability rules from [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2).
    ///
    /// # Type Hierarchy
    ///
    /// The type assignability rules are:
    /// - `Top` is assignable from everything
    /// - `Null` is assignable to any reference type
    /// - Subtype relationships for objects
    /// - Array covariance for reference arrays
    /// - Array types implement `Cloneable` and `Serializable`
    ///
    /// # Arguments
    ///
    /// * `target` - The target type to check assignability to
    /// * `context` - The verification context for class hierarchy checks
    ///
    /// # Errors
    ///
    /// Returns an error if the class hierarchy cannot be resolved.
    #[expect(clippy::match_same_arms)]
    pub fn is_assignable_to<C: VerificationContext>(
        &self,
        target: &VerificationType,
        context: &C,
    ) -> Result<bool> {
        // Same type is always assignable
        if self == target {
            return Ok(true);
        }

        match (self, target) {
            // Top absorbs everything
            (_, VerificationType::Top) => Ok(true),

            // Nothing is assignable to a more specific type from Top
            (VerificationType::Top, _) => Ok(false),

            // Null is assignable to any reference type
            (VerificationType::Null, target) => Ok(target.is_initialized_reference()),

            // Object-to-Object assignability
            (VerificationType::Object(source), VerificationType::Object(target)) => {
                context.is_assignable(target.as_ref(), source.as_ref())
            }

            // Array-to-Object assignability
            (VerificationType::Array(_), VerificationType::Object(target)) => {
                // Arrays are assignable to Object, Cloneable, and Serializable
                Ok(target.as_ref() == "java/lang/Object"
                    || target.as_ref() == "java/lang/Cloneable"
                    || target.as_ref() == "java/io/Serializable")
            }

            // Array-to-Array assignability (covariance for reference arrays)
            (VerificationType::Array(source_comp), VerificationType::Array(target_comp)) => {
                // Primitive arrays are only assignable to the same type
                if source_comp.is_integer()
                    || source_comp.is_category2()
                    || matches!(source_comp.as_ref(), VerificationType::Float)
                {
                    Ok(source_comp == target_comp)
                } else {
                    // Reference arrays are covariant
                    source_comp.is_assignable_to(target_comp, context)
                }
            }

            // Uninitialized types are only assignable to themselves (checked above)
            (VerificationType::Uninitialized(_) | VerificationType::UninitializedThis, _) => {
                Ok(false)
            }

            // Everything else is not assignable
            _ => Ok(false),
        }
    }

    /// Computes the Least Upper Bound (LUB) of two types.
    ///
    /// The LUB is the most specific type that is a supertype of both types.
    /// This is used for merging types at control flow merge points.
    ///
    /// # JVMS Reference
    ///
    /// - [JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2) for the type merging rules.
    ///
    /// # Arguments
    ///
    /// * `other` - The other type to merge with
    /// * `context` - The verification context for class hierarchy checks
    ///
    /// # Returns
    ///
    /// The merged type, or `Top` if types are incompatible.
    ///
    /// # Errors
    ///
    /// Returns an error if the class hierarchy cannot be resolved.
    pub fn merge<C: VerificationContext>(
        &self,
        other: &VerificationType,
        context: &C,
    ) -> Result<VerificationType> {
        // Same types merge to themselves
        if self == other {
            return Ok(self.clone());
        }

        match (self, other) {
            // Top absorbs everything
            (VerificationType::Top, _) | (_, VerificationType::Top) => Ok(VerificationType::Top),

            // Null merges with reference types to produce the reference type
            (VerificationType::Null, other) | (other, VerificationType::Null) => {
                if other.is_initialized_reference() {
                    Ok(other.clone())
                } else {
                    Ok(VerificationType::Top)
                }
            }

            // Object-to-Object merge: find common superclass
            (VerificationType::Object(c1), VerificationType::Object(c2)) => {
                let common = context.common_superclass(c1.as_ref(), c2.as_ref())?;
                Ok(VerificationType::Object(Arc::from(common)))
            }

            // Array-to-Object merge
            (VerificationType::Array(_), VerificationType::Object(obj))
            | (VerificationType::Object(obj), VerificationType::Array(_)) => {
                // The common type of an array and an object is Object
                // (unless the object is Cloneable or Serializable)
                if obj.as_ref() == "java/lang/Cloneable" || obj.as_ref() == "java/io/Serializable" {
                    Ok(VerificationType::Object(obj.clone()))
                } else {
                    Ok(VerificationType::java_lang_object())
                }
            }

            // Array-to-Array merge
            (VerificationType::Array(comp1), VerificationType::Array(comp2)) => {
                // For primitive arrays, if they differ, merge to Object
                if comp1.is_integer()
                    || comp1.is_category2()
                    || matches!(comp1.as_ref(), VerificationType::Float)
                    || comp2.is_integer()
                    || comp2.is_category2()
                    || matches!(comp2.as_ref(), VerificationType::Float)
                {
                    if comp1 == comp2 {
                        Ok(self.clone())
                    } else {
                        Ok(VerificationType::java_lang_object())
                    }
                } else {
                    // Reference arrays: merge component types
                    let merged_comp = comp1.merge(comp2, context)?;
                    Ok(VerificationType::Array(Box::new(merged_comp)))
                }
            }

            // Uninitialized types of different origins merge to Top
            (VerificationType::Uninitialized(_), VerificationType::Uninitialized(_))
            | (VerificationType::UninitializedThis, VerificationType::UninitializedThis) => {
                // Same uninitialized types were handled at the top
                Ok(VerificationType::Top)
            }

            // Incompatible primitive types merge to Top
            _ => Ok(VerificationType::Top),
        }
    }
}

impl Display for VerificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationType::Top => write!(f, "top"),
            VerificationType::Integer => write!(f, "int"),
            VerificationType::Float => write!(f, "float"),
            VerificationType::Long => write!(f, "long"),
            VerificationType::Double => write!(f, "double"),
            VerificationType::Null => write!(f, "null"),
            VerificationType::UninitializedThis => write!(f, "uninitializedThis"),
            VerificationType::Uninitialized(offset) => write!(f, "uninitialized({offset})"),
            VerificationType::Object(name) => write!(f, "{name}"),
            VerificationType::Array(component) => write!(f, "{component}[]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockContext;

    impl VerificationContext for MockContext {
        fn is_subclass(&self, subclass: &str, superclass: &str) -> Result<bool> {
            // Simple mock: String is a subclass of Object
            if (subclass == "java/lang/String" && superclass == "java/lang/Object")
                || subclass == superclass
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }

        fn is_assignable(&self, target: &str, source: &str) -> Result<bool> {
            self.is_subclass(source, target)
        }

        fn common_superclass(&self, class1: &str, class2: &str) -> Result<String> {
            if class1 == class2 {
                Ok(class1.to_string())
            } else {
                Ok("java/lang/Object".to_string())
            }
        }
    }

    #[test]
    fn test_is_category1() {
        assert!(VerificationType::Integer.is_category1());
        assert!(VerificationType::Float.is_category1());
        assert!(VerificationType::Null.is_category1());
        assert!(VerificationType::java_lang_object().is_category1());
        assert!(!VerificationType::Long.is_category1());
        assert!(!VerificationType::Double.is_category1());
    }

    #[test]
    fn test_is_category2() {
        assert!(VerificationType::Long.is_category2());
        assert!(VerificationType::Double.is_category2());
        assert!(!VerificationType::Integer.is_category2());
    }

    #[test]
    fn test_is_reference() {
        assert!(VerificationType::Null.is_reference());
        assert!(VerificationType::java_lang_object().is_reference());
        assert!(VerificationType::Array(Box::new(VerificationType::Integer)).is_reference());
        assert!(VerificationType::UninitializedThis.is_reference());
        assert!(VerificationType::Uninitialized(0).is_reference());
        assert!(!VerificationType::Integer.is_reference());
        assert!(!VerificationType::Top.is_reference());
    }

    #[test]
    fn test_array_operations() {
        let int_array = VerificationType::Integer.make_array();
        assert!(int_array.is_array());
        assert_eq!(int_array.array_dimensions(), 1);
        assert_eq!(int_array.component_type(), Some(&VerificationType::Integer));
        assert_eq!(int_array.element_type(), &VerificationType::Integer);

        let int_2d_array = int_array.clone().make_array();
        assert_eq!(int_2d_array.array_dimensions(), 2);
        assert_eq!(int_2d_array.element_type(), &VerificationType::Integer);
    }

    #[test]
    fn test_from_field_type() {
        assert_eq!(
            VerificationType::from_field_type(&FieldType::Base(BaseType::Int)),
            VerificationType::Integer
        );
        assert_eq!(
            VerificationType::from_field_type(&FieldType::Base(BaseType::Long)),
            VerificationType::Long
        );
        assert_eq!(
            VerificationType::from_field_type(&FieldType::Object("java/lang/String".to_string())),
            VerificationType::Object(Arc::from("java/lang/String"))
        );
    }

    #[test]
    fn test_assignability_same_type() {
        let ctx = MockContext;
        assert!(
            VerificationType::Integer
                .is_assignable_to(&VerificationType::Integer, &ctx)
                .unwrap()
        );
    }

    #[test]
    fn test_assignability_to_top() {
        let ctx = MockContext;
        assert!(
            VerificationType::Integer
                .is_assignable_to(&VerificationType::Top, &ctx)
                .unwrap()
        );
        assert!(
            VerificationType::java_lang_object()
                .is_assignable_to(&VerificationType::Top, &ctx)
                .unwrap()
        );
    }

    #[test]
    fn test_null_assignability() {
        let ctx = MockContext;
        assert!(
            VerificationType::Null
                .is_assignable_to(&VerificationType::java_lang_object(), &ctx)
                .unwrap()
        );
        assert!(
            VerificationType::Null
                .is_assignable_to(
                    &VerificationType::Array(Box::new(VerificationType::Integer)),
                    &ctx
                )
                .unwrap()
        );
        assert!(
            !VerificationType::Null
                .is_assignable_to(&VerificationType::Integer, &ctx)
                .unwrap()
        );
    }

    #[test]
    fn test_array_to_object_assignability() {
        let ctx = MockContext;
        let int_array = VerificationType::Array(Box::new(VerificationType::Integer));

        assert!(
            int_array
                .is_assignable_to(&VerificationType::java_lang_object(), &ctx)
                .unwrap()
        );
        assert!(
            int_array
                .is_assignable_to(&VerificationType::java_lang_cloneable(), &ctx)
                .unwrap()
        );
        assert!(
            int_array
                .is_assignable_to(&VerificationType::java_io_serializable(), &ctx)
                .unwrap()
        );
    }

    #[test]
    fn test_merge_same_types() {
        let ctx = MockContext;
        assert_eq!(
            VerificationType::Integer
                .merge(&VerificationType::Integer, &ctx)
                .unwrap(),
            VerificationType::Integer
        );
    }

    #[test]
    fn test_merge_with_top() {
        let ctx = MockContext;
        assert_eq!(
            VerificationType::Integer
                .merge(&VerificationType::Top, &ctx)
                .unwrap(),
            VerificationType::Top
        );
    }

    #[test]
    fn test_merge_null_with_reference() {
        let ctx = MockContext;
        assert_eq!(
            VerificationType::Null
                .merge(&VerificationType::java_lang_object(), &ctx)
                .unwrap(),
            VerificationType::java_lang_object()
        );
    }

    #[test]
    fn test_merge_incompatible_primitives() {
        let ctx = MockContext;
        assert_eq!(
            VerificationType::Integer
                .merge(&VerificationType::Float, &ctx)
                .unwrap(),
            VerificationType::Top
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(VerificationType::Top.to_string(), "top");
        assert_eq!(VerificationType::Integer.to_string(), "int");
        assert_eq!(VerificationType::Long.to_string(), "long");
        assert_eq!(VerificationType::Null.to_string(), "null");
        assert_eq!(
            VerificationType::Object(Arc::from("java/lang/String")).to_string(),
            "java/lang/String"
        );
        assert_eq!(
            VerificationType::Array(Box::new(VerificationType::Integer)).to_string(),
            "int[]"
        );
    }
}
