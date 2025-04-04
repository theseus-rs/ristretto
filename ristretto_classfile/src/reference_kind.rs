use crate::error::Error::InvalidReferenceKind;
use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of the `ReferenceKind`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.5>
#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

impl ReferenceKind {
    /// Deserialize the `ReferenceKind` from bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes do not represent a valid `ReferenceKind`.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ReferenceKind> {
        let reference_kind = match bytes.read_u8()? {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeStatic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            reference_kind => return Err(InvalidReferenceKind(reference_kind)),
        };

        Ok(reference_kind)
    }

    /// Serialize the `ReferenceKind` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.kind())?;
        Ok(())
    }

    /// Get the numeric value of the reference kind.
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
