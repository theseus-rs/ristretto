use crate::error::Error::InvalidReferenceKind;
use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of the `ReferenceKind`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-5.html#jvms-5.4.3.5>
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_reference_kinds() -> Result<()> {
        for (index, reference_kind) in ReferenceKind::all().iter().enumerate() {
            let expected_kind = u8::try_from(index)? + 1;
            assert_eq!(expected_kind, reference_kind.kind());

            let mut bytes = Vec::new();
            reference_kind.clone().to_bytes(&mut bytes)?;
            let mut bytes = Cursor::new(bytes);
            assert_eq!(expected_kind, bytes.read_u8()?);

            let mut bytes = Cursor::new(expected_kind.to_be_bytes().to_vec());
            assert_eq!(*reference_kind, ReferenceKind::from_bytes(&mut bytes)?);
        }
        Ok(())
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
