//! Image File Header
//!
//! ## Header Fields
//!
//! ```text
//! ╭──────────────────┬────────┬──────┬──────────────────────────────────────╮
//! │ Field            │ Offset │ Size │ Description                          │
//! ├──────────────────┼────────┼──────┼──────────────────────────────────────┤
//! │ magic            │ 0      │ 4    │ 0xCAFEDADA identifying a Image file  │
//! │ version          │ 4      │ 4    │ Format version (major and minor)     │
//! │ flags            │ 8      │ 4    │ Reserved/flags (0 in current images) │
//! │ resource count   │ 12     │ 4    │ Number of resources                  │
//! │ table length     │ 16     │ 4    │ Total byte length of all tables      │
//! │ locations size   │ 24     │ 4    │ File offset to the location table    │
//! │ strings size     │ 28     │ 4    │ File offset to the strings table     │
//! ╰──────────────────┴────────┴──────┴──────────────────────────────────────╯
//! ```

use crate::{Error, Result};
use byteorder::ByteOrder;
use std::io::ErrorKind;

/// The magic number for Image files.
pub(crate) const IMAGE_MAGIC: [u8; 4] = [0xCA, 0xFE, 0xDA, 0xDA];
/// The inverted magic number for Image files.
pub(crate) const IMAGE_MAGIC_INVERTED: [u8; 4] = [0xDA, 0xDA, 0xFE, 0xCA];
/// The key size in bytes.
pub(crate) const KEY_SIZE: usize = 4;
/// Size of the Image header in bytes.
const HEADER_SIZE: usize = 28;
/// Supported Image major version.
const SUPPORTED_MAJOR_VERSION: u16 = 1;
/// Supported Image minor version.
const SUPPORTED_MINOR_VERSION: u16 = 0;

/// Header structure for the Image file format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Header {
    version_major: u16,
    version_minor: u16,
    flags: u32,
    resource_count: usize,
    table_length: usize,
    locations_size: usize,
    strings_size: usize,
}

impl Header {
    /// Parses a `Header` from a byte slice.
    pub(crate) fn from_bytes<T: ByteOrder>(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != HEADER_SIZE {
            return Err(std::io::Error::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "Invalid Image header length; expected 28, actual {}",
                    bytes.len()
                ),
            )
            .into());
        }

        let version = T::read_u32(&bytes[4..8]);
        let version_major = u16::try_from(version >> 16)?;
        let version_minor = u16::try_from(version & 0xFFFF)?;
        if version_major != SUPPORTED_MAJOR_VERSION || version_minor != SUPPORTED_MINOR_VERSION {
            return Err(Error::VersionNotSupported(version_major, version_minor));
        }

        let flags = T::read_u32(&bytes[8..12]);
        let resource_count = T::read_u32(&bytes[12..16]) as usize;
        let table_length = T::read_u32(&bytes[16..20]) as usize;
        let locations_size = T::read_u32(&bytes[20..24]) as usize;
        let strings_size = T::read_u32(&bytes[24..28]) as usize;

        let header = Self {
            version_major,
            version_minor,
            flags,
            resource_count,
            table_length,
            locations_size,
            strings_size,
        };
        Ok(header)
    }

    /// Returns the version of the Image file.
    pub(crate) fn version(&self) -> (&u16, &u16) {
        (&self.version_major, &self.version_minor)
    }

    /// Returns the flags of the Image file.
    pub(crate) fn flags(&self) -> u32 {
        self.flags
    }

    /// Returns the number of resources in the Image file.
    pub(crate) fn resources_count(&self) -> usize {
        self.resource_count
    }

    /// Returns the table length in the Image file.
    pub(crate) fn table_length(&self) -> usize {
        self.table_length
    }

    /// Returns the locations size in the Image file.
    pub(crate) fn locations_size(&self) -> usize {
        self.locations_size
    }

    /// Returns the strings size in the Image file.
    pub(crate) fn strings_size(&self) -> usize {
        self.strings_size
    }

    /// Returns the offset where the redirect table begins.
    #[expect(clippy::unused_self)]
    pub(crate) fn redirect_offset(&self) -> usize {
        HEADER_SIZE
    }

    /// Returns the redirect size in bytes.
    pub(crate) fn redirect_size(&self) -> usize {
        self.table_length * KEY_SIZE
    }

    /// Returns the offset where the attributes table begins.
    pub(crate) fn attribute_offsets(&self) -> usize {
        let redirect_offset = self.redirect_offset();
        let redirect_size = self.redirect_size();
        redirect_offset + redirect_size
    }

    /// Returns the attribute offsets size in bytes.
    pub(crate) fn attribute_offsets_size(&self) -> usize {
        self.table_length * KEY_SIZE
    }

    /// Returns the offset where the attributes table begins.
    pub(crate) fn attribute_data_offset(&self) -> usize {
        let attribute_offsets = self.attribute_offsets();
        let attribute_offsets_size = self.attribute_offsets_size();
        attribute_offsets + attribute_offsets_size
    }

    /// Returns the attribute data size in bytes.
    pub(crate) fn attribute_data_size(&self) -> usize {
        self.table_length * KEY_SIZE
    }

    /// Returns the offset where the attributes table begins.
    pub(crate) fn strings_offset(&self) -> usize {
        let attribute_data_offset = self.attribute_data_offset();
        let locations_size = self.locations_size();
        attribute_data_offset + locations_size
    }

    /// Returns the offset where the data section begins.
    pub(crate) fn data_offset(&self) -> usize {
        self.strings_offset() + self.strings_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::BigEndian;

    #[test]
    fn test_header_parsing() -> Result<()> {
        let bytes: [u8; 28] = [
            0xCA, 0xFE, 0xDA, 0xDA, // magic
            0x00, 0x01, 0x00, 0x00, // version
            0x00, 0x00, 0x00, 0x00, // flags
            0x00, 0x00, 0x00, 0x10, // resource_count
            0x00, 0x00, 0x00, 0x20, // table_length
            0x00, 0x00, 0x00, 0x02, // locations_offset
            0x00, 0x00, 0x00, 0x30, // strings_offset
        ];

        let header = Header::from_bytes::<BigEndian>(&bytes)?;

        assert_eq!(header.version(), (&1, &0));
        assert_eq!(header.flags(), 0);
        assert_eq!(header.resources_count(), 16);
        assert_eq!(header.table_length(), 32);
        assert_eq!(header.locations_size(), 2);
        assert_eq!(header.strings_size(), 48);
        // Redirect index table
        assert_eq!(header.redirect_offset(), 28);
        assert_eq!(header.redirect_size(), 128);
        // Attribute offset index table
        assert_eq!(header.attribute_offsets(), 156);
        assert_eq!(header.attribute_offsets_size(), 128);
        // Attribute data table
        assert_eq!(header.attribute_data_offset(), 284);
        assert_eq!(header.attribute_data_size(), 128);
        // Strings table
        assert_eq!(header.strings_offset(), 286);
        // Data section
        assert_eq!(header.data_offset(), 334);
        Ok(())
    }

    #[test]
    fn test_invalid_length() {
        let bytes: [u8; 10] = [
            0xCA, 0xFE, 0xDA, 0xDA, // magic
            0x00, 0x01, 0x00, 0x00, // version
            0x00, 0x00, // incomplete header
        ];

        let result = Header::from_bytes::<BigEndian>(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_version() {
        let bytes: [u8; 28] = [
            0xCA, 0xFE, 0xDA, 0xDA, // magic
            0x00, 0x02, 0x00, 0x00, // unsupported version
            0x00, 0x00, 0x00, 0x00, // flags
            0x00, 0x00, 0x00, 0x10, // resource_count
            0x00, 0x00, 0x00, 0x20, // table_length
            0x00, 0x00, 0x00, 0x02, // locations_offset
            0x00, 0x00, 0x00, 0x30, // strings_offset
        ];

        let result = Header::from_bytes::<BigEndian>(&bytes);
        assert!(matches!(result, Err(Error::VersionNotSupported(2, 0))));
    }
}
