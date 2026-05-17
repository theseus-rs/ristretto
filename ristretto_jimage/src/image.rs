//! A module for reading and parsing Image files.
//!
//! ## File layout
//!
//! ```text
//! ╭───────────────────╮
//! │      Header       │
//! ├───────────────────┤
//! │   Redirect Table  │
//! ├───────────────────┤
//! │ Attribute Offsets │
//! ├───────────────────┤
//! │   Attribute Data  │
//! ├───────────────────┤
//! │      Strings      │
//! ├───────────────────┤
//! │       Data        │
//! ╰───────────────────╯
//! ```

use crate::byte_source::ByteSource;
use crate::header::{Header, IMAGE_MAGIC, IMAGE_MAGIC_INVERTED};
use crate::index::Index;
use crate::{Error, Resource, Result};
use byteorder::{BigEndian, LittleEndian};
use std::path::Path;

/// Endian detected from the magic number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

/// A representation of an Image file.
#[derive(Debug)]
pub struct Image {
    endian: Endian,
    byte_source: ByteSource,
    header: Header,
}

impl Image {
    /// Creates a new `Image` instance from the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path cannot be read or if it is not a valid Image path.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file(path: &Path) -> Result<Self> {
        let byte_source = ByteSource::from(path)?;
        let bytes = byte_source.get_bytes(0..28)?;
        let endian = endian_from_magic(&bytes[..4])?;
        let header = match endian {
            Endian::Little => Header::from_bytes::<LittleEndian>(&bytes[..28])?,
            Endian::Big => Header::from_bytes::<BigEndian>(&bytes[..28])?,
        };

        let image = Self {
            endian,
            byte_source,
            header,
        };
        Ok(image)
    }

    /// Returns a reference to the underlying byte source.
    pub(crate) fn byte_source(&self) -> &ByteSource {
        &self.byte_source
    }

    /// Returns a reference to the header of the Image file.
    pub(crate) fn header(&self) -> &Header {
        &self.header
    }

    /// Retrieves a resource by name.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource cannot be found or if there is an issue reading it.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     let resource_name = "/java.base/java/lang/Object.class";
    ///     let resource = image.get_resource(resource_name)?;
    ///     assert_eq!(resource_name, resource.full_name());
    ///     Ok(())
    /// }
    /// ```
    pub fn get_resource(&'_ self, name: &str) -> Result<Resource<'_>> {
        let resource = match self.endian {
            Endian::Little => {
                let resource_offset = Index::get_resource_offset::<LittleEndian>(self, name)?;
                Resource::from_bytes::<LittleEndian>(self, resource_offset)?
            }
            Endian::Big => {
                let resource_offset = Index::get_resource_offset::<BigEndian>(self, name)?;
                Resource::from_bytes::<BigEndian>(self, resource_offset)?
            }
        };

        let resource_name = resource.full_name();
        if resource_name != name {
            return Err(Error::ResourceNameMismatch {
                expected: name.to_string(),
                actual: resource_name.clone(),
            });
        }

        Ok(resource)
    }

    /// Returns an iterator over all resources in the Image file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     for (index, resource) in image.iter().enumerate() {
    ///         let resource = resource?;
    ///         println!("Resource [{index}]: {}", resource.full_name());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&self) -> ImageIterator<'_> {
        ImageIterator {
            image: self,
            current_index: 0,
            total_resources: self.header.table_length(),
        }
    }
}

/// Determines the endian from the magic bytes.
///
/// # Errors
/// Returns an error if the magic bytes do not match any known endian.
fn endian_from_magic(bytes: &[u8]) -> Result<Endian> {
    if bytes == IMAGE_MAGIC_INVERTED {
        return Ok(Endian::Little);
    } else if bytes == IMAGE_MAGIC {
        return Ok(Endian::Big);
    }
    Err(Error::InvalidMagicBytes(bytes.to_vec()))
}

/// Iterator for Image resources.
#[derive(Debug)]
pub struct ImageIterator<'a> {
    image: &'a Image,
    current_index: usize,
    total_resources: usize,
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = Result<Resource<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.total_resources {
            return None;
        }
        let resource_result = match self.image.endian {
            Endian::Little => Resource::from_bytes::<LittleEndian>(self.image, self.current_index),
            Endian::Big => Resource::from_bytes::<BigEndian>(self.image, self.current_index),
        };
        self.current_index += 1;
        Some(resource_result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_resources.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a Image {
    type Item = Result<Resource<'a>>;
    type IntoIter = ImageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl ExactSizeIterator for ImageIterator<'_> {
    fn len(&self) -> usize {
        self.total_resources.saturating_sub(self.current_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byte_source::ByteSource;
    use crate::header::Header;
    use crate::index::Index;
    use crate::test_utils::{
        CLASS_DATA, EXT_DATA, EXT_NAME, MODULE_INFO_DATA, MODULE_INFO_NAME, STRING_NAME,
        write_extensionless_big_endian_image, write_standard_big_endian_image,
        write_standard_little_endian_image,
    };
    use byteorder::BigEndian;
    use std::borrow::Cow;
    use std::fs::File;
    use std::io::Write;
    use std::sync::Mutex;
    use tempfile::NamedTempFile;

    #[test]
    fn test_endian_from_magic() -> Result<()> {
        assert_eq!(
            endian_from_magic(&[0xDA, 0xDA, 0xFE, 0xCA])?,
            Endian::Little
        );
        assert_eq!(endian_from_magic(&[0xCA, 0xFE, 0xDA, 0xDA])?, Endian::Big);
        assert!(endian_from_magic(&[0x00, 0x00, 0x00, 0x00]).is_err());

        Ok(())
    }

    #[test]
    fn test_from_file_rejects_invalid_magic() -> Result<()> {
        let temp_file = write_bytes(&[0; 28])?;
        assert!(matches!(
            Image::from_file(temp_file.path()),
            Err(Error::InvalidMagicBytes(bytes)) if bytes == vec![0, 0, 0, 0]
        ));
        Ok(())
    }

    #[test]
    fn test_big_endian_get_resource_and_iter() -> Result<()> {
        let temp_file = write_standard_big_endian_image()?;
        let image = Image::from_file(temp_file.path())?;

        let module_info = image.get_resource(MODULE_INFO_NAME)?;
        assert_eq!(module_info.module(), "java.base");
        assert_eq!(module_info.parent(), "");
        assert_eq!(module_info.base(), "module-info");
        assert_eq!(module_info.extension(), "class");
        assert_eq!(module_info.name(), "module-info.class");
        assert_eq!(module_info.full_name(), MODULE_INFO_NAME);
        assert_eq!(module_info.data(), MODULE_INFO_DATA);

        let ext = image.get_resource(EXT_NAME)?;
        assert_eq!(ext.module(), "mod");
        assert_eq!(ext.name(), "base.ext");
        assert_eq!(ext.data(), EXT_DATA);

        let class = image.get_resource(STRING_NAME)?;
        assert_eq!(class.parent(), "java/lang");
        assert_eq!(class.full_name(), STRING_NAME);
        assert_eq!(class.data(), CLASS_DATA);

        assert!(matches!(
            image.get_resource("/java.base/nope.class"),
            Err(Error::InvalidIndex(2))
        ));

        let mut iterator = image.iter();
        assert_eq!(iterator.len(), 3);
        assert_eq!(iterator.size_hint(), (3, Some(3)));
        assert_eq!(next_full_name(&mut iterator)?, MODULE_INFO_NAME);
        assert_eq!(iterator.len(), 2);
        assert_eq!(next_full_name(&mut iterator)?, EXT_NAME);
        assert_eq!(next_full_name(&mut iterator)?, STRING_NAME);
        assert!(iterator.next().is_none());
        assert_eq!(iterator.size_hint(), (0, Some(0)));

        let names = (&image)
            .into_iter()
            .map(|resource| resource.map(|resource| resource.full_name()))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(names, vec![MODULE_INFO_NAME, EXT_NAME, STRING_NAME]);
        Ok(())
    }

    #[test]
    fn test_little_endian_get_resource_and_iter() -> Result<()> {
        let temp_file = write_standard_little_endian_image()?;
        let image = Image::from_file(temp_file.path())?;

        let class = image.get_resource(STRING_NAME)?;
        assert_eq!(class.full_name(), STRING_NAME);
        assert_eq!(class.data(), CLASS_DATA);

        let mut iterator = image.iter();
        assert_eq!(next_full_name(&mut iterator)?, MODULE_INFO_NAME);
        assert_eq!(next_full_name(&mut iterator)?, EXT_NAME);
        assert_eq!(next_full_name(&mut iterator)?, STRING_NAME);
        assert!(iterator.next().is_none());
        Ok(())
    }

    #[test]
    fn test_get_resource_reports_name_mismatch() -> Result<()> {
        let temp_file = write_extensionless_big_endian_image()?;
        let image = Image::from_file(temp_file.path())?;

        assert!(matches!(
            image.get_resource("/java.base/README."),
            Err(Error::ResourceNameMismatch { expected, actual })
                if expected == "/java.base/README." && actual == "/java.base/README"
        ));
        Ok(())
    }

    #[test]
    fn test_index_get_string_decodes_lossy_borrowed_bytes() -> Result<()> {
        let bytes = string_image_bytes(&[0xFF, 0]);
        let image = image_from_byte_source(ByteSource::Bytes(bytes))?;
        let value = Index::get_string(&image, 0)?;
        assert_eq!(value.as_ref(), String::from_utf8_lossy(&[0xFF]).as_ref());
        Ok(())
    }

    #[test]
    fn test_index_get_string_decodes_owned_file_bytes() -> Result<()> {
        let temp_file = write_bytes(&string_image_bytes(b"owned\0"))?;
        let file = File::open(temp_file.path())?;
        let image = image_from_byte_source(ByteSource::File(Mutex::new(file)))?;
        let value = Index::get_string(&image, 0)?;
        assert!(matches!(value, Cow::Owned(value) if value == "owned"));
        Ok(())
    }

    #[test]
    fn test_index_get_string_decodes_lossy_owned_file_bytes() -> Result<()> {
        let temp_file = write_bytes(&string_image_bytes(&[0xFF, 0]))?;
        let file = File::open(temp_file.path())?;
        let image = image_from_byte_source(ByteSource::File(Mutex::new(file)))?;
        let value = Index::get_string(&image, 0)?;
        assert_eq!(value.as_ref(), String::from_utf8_lossy(&[0xFF]).as_ref());
        Ok(())
    }

    fn image_from_byte_source(byte_source: ByteSource) -> Result<Image> {
        let bytes = string_image_bytes(&[]);
        let header = Header::from_bytes::<BigEndian>(&bytes[..28])?;
        Ok(Image {
            endian: Endian::Big,
            byte_source,
            header,
        })
    }

    fn next_full_name(iterator: &mut ImageIterator<'_>) -> Result<String> {
        let resource = iterator
            .next()
            .transpose()?
            .ok_or(Error::InvalidIndex(usize::MAX))?;
        Ok(resource.full_name())
    }

    fn string_image_bytes(string_bytes: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0xCA, 0xFE, 0xDA, 0xDA]);
        bytes.extend_from_slice(&0x0001_0000_u32.to_be_bytes());
        bytes.extend_from_slice(&0_u32.to_be_bytes());
        bytes.extend_from_slice(&0_u32.to_be_bytes());
        bytes.extend_from_slice(&0_u32.to_be_bytes());
        bytes.extend_from_slice(&0_u32.to_be_bytes());
        let strings_size =
            u32::try_from(string_bytes.len()).expect("test string table length fits in u32");
        bytes.extend_from_slice(&strings_size.to_be_bytes());
        bytes.extend_from_slice(string_bytes);
        bytes
    }

    fn write_bytes(bytes: &[u8]) -> std::io::Result<NamedTempFile> {
        ristretto_test_util::init_wasi_tempdir();
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(bytes)?;
        temp_file.flush()?;
        Ok(temp_file)
    }
}
