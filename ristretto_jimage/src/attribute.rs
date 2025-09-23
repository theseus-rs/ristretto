use crate::Error::InvalidAttributeData;
use crate::byte_source::ByteSource;
use crate::{Error, Result};

/// Represents the types of attributes in an Image file.
#[derive(Clone, Debug, PartialEq, Eq)]
enum AttributeType {
    End,
    Module,
    Parent,
    Base,
    Extension,
    Offset,
    Compressed,
    Uncompressed,
    Count,
}

impl TryFrom<u8> for AttributeType {
    type Error = Error;

    /// Converts a `u8` to an `AttributeType`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::InvalidAttributeType` if the value does not correspond to any
    /// `AttributeType`.
    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(AttributeType::End),
            1 => Ok(AttributeType::Module),
            2 => Ok(AttributeType::Parent),
            3 => Ok(AttributeType::Base),
            4 => Ok(AttributeType::Extension),
            5 => Ok(AttributeType::Offset),
            6 => Ok(AttributeType::Compressed),
            7 => Ok(AttributeType::Uncompressed),
            8 => Ok(AttributeType::Count),
            _ => Err(Error::InvalidAttributeType(value)),
        }
    }
}

/// Represents the attributes of a resource in an Image file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Attributes {
    /// The module offset.
    module_offset: usize,
    /// The parent offset.
    parent_offset: usize,
    /// The base offset.
    base_offset: usize,
    /// The extension offset.
    extension_offset: usize,
    /// The data offset.
    offset: usize,
    /// The compressed size.
    compressed_size: usize,
    /// The uncompressed size.
    uncompressed_size: usize,
}

impl Attributes {
    /// Parses attributes from a byte slice.
    pub fn from_bytes(byte_source: &ByteSource, mut offset: usize, limit: usize) -> Result<Self> {
        let mut attributes = Attributes {
            module_offset: 0,
            parent_offset: 0,
            base_offset: 0,
            extension_offset: 0,
            offset: 0,
            compressed_size: 0,
            uncompressed_size: 0,
        };

        while offset < limit {
            let bytes = byte_source.get_bytes(offset..=offset)?;
            let data = bytes[0];
            let attribute_type = AttributeType::try_from(data >> 3)?;
            if attribute_type == AttributeType::End {
                break;
            }

            let length = (data as usize & 0x7) + 1;
            offset += 1;
            let value = Self::read_value(byte_source, offset, length)?;

            match attribute_type {
                AttributeType::End => break,
                AttributeType::Module => {
                    attributes.module_offset = value;
                }
                AttributeType::Parent => {
                    attributes.parent_offset = value;
                }
                AttributeType::Base => {
                    attributes.base_offset = value;
                }
                AttributeType::Extension => {
                    attributes.extension_offset = value;
                }
                AttributeType::Offset => {
                    attributes.offset = value;
                }
                AttributeType::Compressed => {
                    attributes.compressed_size = value;
                }
                AttributeType::Uncompressed => {
                    attributes.uncompressed_size = value;
                }
                AttributeType::Count => {
                    // Count attribute is not used in this context
                }
            }

            offset += length;
        }

        Ok(attributes)
    }

    /// Reads a multiple byte value from the byte source.
    #[inline]
    fn read_value(byte_source: &ByteSource, offset: usize, length: usize) -> Result<usize> {
        let end = offset.checked_add(length).ok_or(InvalidAttributeData)?;
        let bytes = byte_source.get_bytes(offset..end)?;
        let mut buffer = [0u8; 8];
        buffer[8 - bytes.len()..].copy_from_slice(&bytes);
        let value = u64::from_be_bytes(buffer);
        let value = usize::try_from(value)?;
        Ok(value)
    }

    /// Returns the module offset.
    pub fn module_offset(&self) -> usize {
        self.module_offset
    }

    /// Returns the parent offset.
    pub fn parent_offset(&self) -> usize {
        self.parent_offset
    }

    /// Returns the base offset.
    pub fn base_offset(&self) -> usize {
        self.base_offset
    }

    /// Returns the extension offset.
    pub fn extension_offset(&self) -> usize {
        self.extension_offset
    }

    /// Returns the data offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the compressed size.
    pub fn compressed_size(&self) -> usize {
        self.compressed_size
    }

    /// Returns the uncompressed size.
    pub fn uncompressed_size(&self) -> usize {
        self.uncompressed_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_type_conversion() -> Result<()> {
        assert_eq!(AttributeType::try_from(0)?, AttributeType::End);
        assert_eq!(AttributeType::try_from(1)?, AttributeType::Module);
        assert_eq!(AttributeType::try_from(2)?, AttributeType::Parent);
        assert_eq!(AttributeType::try_from(3)?, AttributeType::Base);
        assert_eq!(AttributeType::try_from(4)?, AttributeType::Extension);
        assert_eq!(AttributeType::try_from(5)?, AttributeType::Offset);
        assert_eq!(AttributeType::try_from(6)?, AttributeType::Compressed);
        assert_eq!(AttributeType::try_from(7)?, AttributeType::Uncompressed);
        assert_eq!(AttributeType::try_from(8)?, AttributeType::Count);
        assert!(matches!(
            AttributeType::try_from(9),
            Err(Error::InvalidAttributeType(9))
        ));
        Ok(())
    }

    #[test]
    fn test_attributes_from_bytes() -> Result<()> {
        let bytes = vec![
            9, 1, 191, 18, 7, 214, 127, 26, 7, 215, 162, 32, 1, 43, 6, 224, 205, 170, 57, 3, 160, 0,
        ];
        let byte_source = ByteSource::Bytes(bytes);
        let attributes = Attributes::from_bytes(&byte_source, 0, byte_source.len()?)?;
        assert_eq!(attributes.module_offset(), 447);
        assert_eq!(attributes.parent_offset(), 513_663);
        assert_eq!(attributes.base_offset(), 513_954);
        assert_eq!(attributes.extension_offset(), 1);
        assert_eq!(attributes.offset(), 115_396_010);
        assert_eq!(attributes.compressed_size(), 0);
        assert_eq!(attributes.uncompressed_size(), 928);
        Ok(())
    }

    #[test]
    fn test_read_value() -> Result<()> {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let byte_source = ByteSource::Bytes(data);
        assert_eq!(Attributes::read_value(&byte_source, 0, 1)?, 0x01);
        assert_eq!(Attributes::read_value(&byte_source, 0, 2)?, 0x0102);
        assert_eq!(Attributes::read_value(&byte_source, 0, 3)?, 0x01_0203);
        assert_eq!(Attributes::read_value(&byte_source, 0, 4)?, 0x0102_0304);
        assert!(matches!(
            Attributes::read_value(&byte_source, 2, usize::MAX),
            Err(InvalidAttributeData)
        ));
        Ok(())
    }
}
