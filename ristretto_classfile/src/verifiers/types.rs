use std::fmt::Display;

/// Represents the verification types used in the JVM bytecode verification process. These types
/// correspond to the types defined in the Java Virtual Machine Specification (JVMS) for stack map
/// frames.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VerificationType {
    /// Top type, used for the second slot of Long and Double
    Top,
    /// Integer type
    Integer,
    /// Float type
    Float,
    /// Long type
    Long,
    /// Double type
    Double,
    /// Null type
    Null,
    /// Uninitialized "this" type
    UninitializedThis,
    /// Object type with class name
    Object(String),
    /// Uninitialized type with offset of the new instruction
    // Long and Double take two slots, represented by Top in the second slot in some implementations,
    // or handled by logic. JVMS suggests using "Long" and "Double" as single types that consume 2 entries in locals/stack calculations.
    // But for type checking, we might need 2nd slot marker.
    // Let's stick to JVMS 4.10.1.2:
    // "Long and Double are considered to be of category 2... they occupy two local variables or two stack entries."
    // We will follow the convention of having specific types and managing the 2-slot aspect in the Frame logic.
    Uninitialized(u16),
}

/// Implements the Display trait for `VerificationType` to provide a human-readable representation.
impl Display for VerificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationType::Top => write!(f, "top"),
            VerificationType::Integer => write!(f, "int"),
            VerificationType::Float => write!(f, "float"),
            VerificationType::Long => write!(f, "long"),
            VerificationType::Double => write!(f, "double"),
            VerificationType::Null => write!(f, "null"),
            VerificationType::UninitializedThis => write!(f, "uninitialized_this"),
            VerificationType::Object(name) => write!(f, "class {name}"),
            VerificationType::Uninitialized(offset) => write!(f, "uninitialized({offset})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(VerificationType::Top.to_string(), "top");
        assert_eq!(VerificationType::Integer.to_string(), "int");
        assert_eq!(VerificationType::Float.to_string(), "float");
        assert_eq!(VerificationType::Long.to_string(), "long");
        assert_eq!(VerificationType::Double.to_string(), "double");
        assert_eq!(VerificationType::Null.to_string(), "null");
        assert_eq!(
            VerificationType::UninitializedThis.to_string(),
            "uninitialized_this"
        );
        assert_eq!(
            VerificationType::Object("java/lang/Object".to_string()).to_string(),
            "class java/lang/Object"
        );
        assert_eq!(
            VerificationType::Uninitialized(42).to_string(),
            "uninitialized(42)"
        );
    }
}
