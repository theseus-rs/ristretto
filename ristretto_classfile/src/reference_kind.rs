use crate::Error;
use crate::error::Error::InvalidReferenceKind;
use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents the behavior of a dynamic call site in the Java Virtual Machine.
///
/// The `ReferenceKind` enum is used in `MethodHandle` constant pool entries to indicate how the
/// method handle should be interpreted by the JVM. Each variant represents a different kind of
/// access or invocation that can be performed on fields and methods.
///
/// These reference kinds are essential for method handles and invokedynamic instructions  in the
/// JVM, which enable advanced language features like lambda expressions and  method references in
/// Java.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::ReferenceKind;
/// use std::io::Cursor;
///
/// // Create a reference kind
/// let reference_kind = ReferenceKind::InvokeVirtual;
///
/// // Serialize to bytes
/// let mut bytes = Vec::new();
/// reference_kind.to_bytes(&mut bytes)?;
/// assert_eq!(bytes, vec![5]); // InvokeVirtual has value 5
///
/// // Deserialize from bytes
/// let mut cursor = Cursor::new(vec![6]);
/// let deserialized = ReferenceKind::from_bytes(&mut cursor)?;
/// assert_eq!(deserialized, ReferenceKind::InvokeStatic);
///
/// // Get all reference kinds
/// let all_kinds = ReferenceKind::all();
/// assert_eq!(all_kinds.len(), 9);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.5>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    /// Reference kind 1: Read a non-static field.
    /// Used to get the value of an instance field.
    GetField,
    /// Reference kind 2: Read a static field.
    /// Used to get the value of a static field.
    GetStatic,
    /// Reference kind 3: Write a non-static field.
    /// Used to set the value of an instance field.
    PutField,
    /// Reference kind 4: Write a static field.
    /// Used to set the value of a static field.
    PutStatic,
    /// Reference kind 5: Invoke a virtual method.
    /// Used for regular method calls on object instances, with virtual method resolution.
    InvokeVirtual,
    /// Reference kind 6: Invoke a static method.
    /// Used for calls to static methods.
    InvokeStatic,
    /// Reference kind 7: Invoke a special method.
    /// Used for calls to instance initialization methods, private methods, and superclass methods.
    InvokeSpecial,
    /// Reference kind 8: Invoke a constructor.
    /// Used specifically for constructor invocation with the new keyword.
    NewInvokeSpecial,
    /// Reference kind 9: Invoke an interface method.
    /// Used for calls to interface methods.
    InvokeInterface,
}

impl ReferenceKind {
    /// Deserialize the `ReferenceKind` from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes do not represent a valid `ReferenceKind`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    /// use std::io::Cursor;
    ///
    /// // Deserialize a valid reference kind
    /// let mut cursor = Cursor::new(vec![5]);
    /// let reference_kind = ReferenceKind::from_bytes(&mut cursor)?;
    /// assert_eq!(reference_kind, ReferenceKind::InvokeVirtual);
    ///
    /// // Attempting to deserialize an invalid reference kind
    /// let mut cursor = Cursor::new(vec![10]);
    /// let result = ReferenceKind::from_bytes(&mut cursor);
    /// assert!(result.is_err());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ReferenceKind> {
        let byte = bytes.read_u8()?;
        ReferenceKind::try_from(byte)
    }

    /// Serialize the `ReferenceKind` to bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    ///
    /// // Serialize GetField (kind 1)
    /// let reference_kind = ReferenceKind::GetField;
    /// let mut bytes = Vec::new();
    /// reference_kind.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![1]);
    ///
    /// // Serialize InvokeInterface (kind 9)
    /// let reference_kind = ReferenceKind::InvokeInterface;
    /// let mut bytes = Vec::new();
    /// reference_kind.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![9]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.kind())?;
        Ok(())
    }

    /// Get the numeric value of the reference kind.
    ///
    /// Returns the u8 value that corresponds to this reference kind in the JVM specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    ///
    /// let reference_kind = ReferenceKind::InvokeVirtual;
    /// assert_eq!(reference_kind.kind(), 5);
    ///
    /// let reference_kind = ReferenceKind::GetField;
    /// assert_eq!(reference_kind.kind(), 1);
    ///
    /// // The kind value can be used when working with bytecode directly
    /// let bytecode_value = ReferenceKind::InvokeInterface.kind();
    /// assert_eq!(bytecode_value, 9);
    /// ```
    #[must_use]
    pub fn kind(&self) -> u8 {
        match self {
            ReferenceKind::GetField => 1,
            ReferenceKind::GetStatic => 2,
            ReferenceKind::PutField => 3,
            ReferenceKind::PutStatic => 4,
            ReferenceKind::InvokeVirtual => 5,
            ReferenceKind::InvokeStatic => 6,
            ReferenceKind::InvokeSpecial => 7,
            ReferenceKind::NewInvokeSpecial => 8,
            ReferenceKind::InvokeInterface => 9,
        }
    }

    /// Get all reference kinds.
    ///
    /// Returns a vector containing all possible reference kinds defined in the JVM specification.
    /// This can be useful for iterating through all reference kinds or for validation purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    ///
    /// // Get all reference kinds
    /// let all_kinds = ReferenceKind::all();
    ///
    /// // There should be exactly 9 reference kinds
    /// assert_eq!(all_kinds.len(), 9);
    ///
    /// // Check if a specific reference kind is in the list
    /// assert!(all_kinds.contains(&ReferenceKind::InvokeVirtual));
    ///
    /// // Iterate through all reference kinds
    /// for kind in ReferenceKind::all() {
    ///     // Each kind should have a valid numeric value between 1 and 9
    ///     assert!(kind.kind() >= 1 && kind.kind() <= 9);
    /// }
    /// ```
    #[must_use]
    pub fn all() -> Vec<ReferenceKind> {
        vec![
            ReferenceKind::GetField,
            ReferenceKind::GetStatic,
            ReferenceKind::PutField,
            ReferenceKind::PutStatic,
            ReferenceKind::InvokeVirtual,
            ReferenceKind::InvokeStatic,
            ReferenceKind::InvokeSpecial,
            ReferenceKind::NewInvokeSpecial,
            ReferenceKind::InvokeInterface,
        ]
    }
}

impl fmt::Display for ReferenceKind {
    /// Formats the `ReferenceKind` as a string.
    ///
    /// This implementation returns the name of the reference kind variant as a string,
    /// which is useful for debugging, logging, and generating human-readable output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    /// use std::fmt::Display;
    ///
    /// // Convert a reference kind to a string
    /// let reference_kind = ReferenceKind::InvokeVirtual;
    /// assert_eq!(reference_kind.to_string(), "InvokeVirtual");
    ///
    /// // Use with string formatting
    /// let message = format!("Method handle type: {}", ReferenceKind::GetField);
    /// assert_eq!(message, "Method handle type: GetField");
    ///
    /// // Useful in error messages or logging
    /// let reference_kinds = vec![ReferenceKind::InvokeStatic, ReferenceKind::InvokeInterface];
    /// for kind in reference_kinds {
    ///     println!("Processing reference kind: {kind}");
    ///     // This would print:
    ///     // Processing reference kind: InvokeStatic
    ///     // Processing reference kind: InvokeInterface
    /// }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReferenceKind::GetField => write!(f, "GetField"),
            ReferenceKind::GetStatic => write!(f, "GetStatic"),
            ReferenceKind::PutField => write!(f, "PutField"),
            ReferenceKind::PutStatic => write!(f, "PutStatic"),
            ReferenceKind::InvokeVirtual => write!(f, "InvokeVirtual"),
            ReferenceKind::InvokeStatic => write!(f, "InvokeStatic"),
            ReferenceKind::InvokeSpecial => write!(f, "InvokeSpecial"),
            ReferenceKind::NewInvokeSpecial => write!(f, "NewInvokeSpecial"),
            ReferenceKind::InvokeInterface => write!(f, "InvokeInterface"),
        }
    }
}

impl TryFrom<u8> for ReferenceKind {
    type Error = Error;

    /// Converts a `u8` value to a `ReferenceKind`.
    ///
    /// # Errors
    ///
    /// Returns an error if the `u8` value does not correspond to a valid `ReferenceKind`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ReferenceKind;
    ///
    /// // Convert a valid u8 to ReferenceKind
    /// let reference_kind: ReferenceKind = 5.try_into()?;
    /// assert_eq!(reference_kind, ReferenceKind::InvokeVirtual);
    ///
    /// // Attempting to convert an invalid u8
    /// let invalid_reference_kind: Result<ReferenceKind, _> = 10.try_into();
    /// assert!(invalid_reference_kind.is_err());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(ReferenceKind::GetField),
            2 => Ok(ReferenceKind::GetStatic),
            3 => Ok(ReferenceKind::PutField),
            4 => Ok(ReferenceKind::PutStatic),
            5 => Ok(ReferenceKind::InvokeVirtual),
            6 => Ok(ReferenceKind::InvokeStatic),
            7 => Ok(ReferenceKind::InvokeSpecial),
            8 => Ok(ReferenceKind::NewInvokeSpecial),
            9 => Ok(ReferenceKind::InvokeInterface),
            _ => Err(InvalidReferenceKind(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_reference_kind(reference_kind: &ReferenceKind, expected_kind: u8) -> Result<()> {
        assert_eq!(expected_kind, reference_kind.kind());

        let mut bytes = Vec::new();
        reference_kind.clone().to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(expected_kind, bytes.read_u8()?);

        let mut bytes = Cursor::new(expected_kind.to_be_bytes().to_vec());
        assert_eq!(*reference_kind, ReferenceKind::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_get_field() -> Result<()> {
        let reference_kind = ReferenceKind::GetField;

        assert_eq!("GetField", reference_kind.to_string());
        test_reference_kind(&reference_kind, 1)
    }

    #[test]
    fn test_get_static() -> Result<()> {
        let reference_kind = ReferenceKind::GetStatic;

        assert_eq!("GetStatic", reference_kind.to_string());
        test_reference_kind(&reference_kind, 2)
    }

    #[test]
    fn test_put_field() -> Result<()> {
        let reference_kind = ReferenceKind::PutField;

        assert_eq!("PutField", reference_kind.to_string());
        test_reference_kind(&reference_kind, 3)
    }

    #[test]
    fn test_put_static() -> Result<()> {
        let reference_kind = ReferenceKind::PutStatic;

        assert_eq!("PutStatic", reference_kind.to_string());
        test_reference_kind(&reference_kind, 4)
    }

    #[test]
    fn test_invoke_virtual() -> Result<()> {
        let reference_kind = ReferenceKind::InvokeVirtual;

        assert_eq!("InvokeVirtual", reference_kind.to_string());
        test_reference_kind(&reference_kind, 5)
    }

    #[test]
    fn test_invoke_static() -> Result<()> {
        let reference_kind = ReferenceKind::InvokeStatic;

        assert_eq!("InvokeStatic", reference_kind.to_string());
        test_reference_kind(&reference_kind, 6)
    }

    #[test]
    fn test_invoke_special() -> Result<()> {
        let reference_kind = ReferenceKind::InvokeSpecial;

        assert_eq!("InvokeSpecial", reference_kind.to_string());
        test_reference_kind(&reference_kind, 7)
    }

    #[test]
    fn test_new_invoke_special() -> Result<()> {
        let reference_kind = ReferenceKind::NewInvokeSpecial;

        assert_eq!("NewInvokeSpecial", reference_kind.to_string());
        test_reference_kind(&reference_kind, 8)
    }

    #[test]
    fn test_invoke_interface() -> Result<()> {
        let reference_kind = ReferenceKind::InvokeInterface;

        assert_eq!("InvokeInterface", reference_kind.to_string());
        test_reference_kind(&reference_kind, 9)
    }

    #[test]
    fn test_from_bytes_invalid_reference_kind() {
        let mut bytes = Cursor::new(vec![0]);
        assert_eq!(
            Err(InvalidReferenceKind(0)),
            ReferenceKind::from_bytes(&mut bytes)
        );
    }
}
