//! Image file index
//!
//! ## File layout
//!
//! ```text
//! ╭───────────────────╮
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

use crate::Error::InvalidIndex;
use crate::attribute::Attributes;
use crate::header::KEY_SIZE;
use crate::{Image, Result};
use byteorder::ByteOrder;
use std::borrow::Cow;

/// Hash multiplier constant used in the hash function.
///
/// # References
///
/// - [A Practical Minimal Perfect Hashing Method](http://homepages.dcc.ufmg.br/~nivio/papers/wea05.pdf)
const HASH_MULTIPLIER: usize = 0x0100_0193;

/// Provides methods to access the index section of a Image file.
pub(crate) struct Index {}

impl Index {
    /// Retrieves the redirect table entry for the given index. Returns the offset into the
    /// attribute offsets table.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    fn get_redirect_offset<T: ByteOrder>(image: &Image, index: usize) -> Result<isize> {
        let header = image.header();
        let redirect_offset = header.redirect_offset();
        let offset_index = redirect_offset + (index * KEY_SIZE);
        let byte_source = image.byte_source();
        let offset_bytes = byte_source.get_bytes(offset_index..(offset_index + KEY_SIZE))?;
        let offset = T::read_i32(&offset_bytes) as isize;
        Ok(offset)
    }

    /// Retrieves the resource index for a given resource name.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource name is not found.
    pub(crate) fn get_resource_offset<T: ByteOrder>(image: &Image, name: &str) -> Result<usize> {
        let header = image.header();
        let resources_count = header.resources_count();
        let hash = Self::hash(name, HASH_MULTIPLIER);
        let redirect_index = hash % resources_count;
        let redirect_offset = Self::get_redirect_offset::<T>(image, redirect_index)?;
        let resource_index = match redirect_offset {
            value if value < 0 => {
                #[expect(clippy::cast_sign_loss)]
                {
                    (-1 - value) as usize
                }
            }
            value if value > 0 => {
                #[expect(clippy::cast_sign_loss)]
                {
                    let hash = Self::hash(name, value as usize);
                    hash % resources_count
                }
            }
            _ => redirect_index,
        };

        // Validate that the resource exists by checking if we can get valid attributes
        let attribute_offset = Self::get_attribute_offset::<T>(image, resource_index)?;
        let attributes = Self::get_attributes(image, attribute_offset)?;

        // Reconstruct the resource name from attributes and validate
        let module = Self::get_string(image, attributes.module_offset())?;
        let parent = Self::get_string(image, attributes.parent_offset())?;
        let base = Self::get_string(image, attributes.base_offset())?;
        let extension = Self::get_string(image, attributes.extension_offset())?;

        // Build the reconstructed name. Since it must match the input name for success,
        // we can use name.len() as the exact capacity needed.
        let mut reconstructed_name = String::with_capacity(name.len());
        reconstructed_name.push('/');
        reconstructed_name.push_str(&module);
        reconstructed_name.push('/');
        if !parent.is_empty() {
            reconstructed_name.push_str(&parent);
            reconstructed_name.push('/');
        }
        reconstructed_name.push_str(&base);
        reconstructed_name.push('.');
        reconstructed_name.push_str(&extension);

        if reconstructed_name == name {
            Ok(resource_index)
        } else {
            Err(InvalidIndex(resource_index))
        }
    }

    /// Computes the hash of a string using a given seed.
    fn hash(value: &str, seed: usize) -> usize {
        let hash = value.bytes().fold(seed, |hash, byte| {
            hash.overflowing_mul(HASH_MULTIPLIER).0 ^ byte as usize
        });
        hash & 0x7FFF_FFFF
    }

    /// Retrieves the attribute offset for the given redirect index. Returns the offset into the
    /// attribute data section.
    ///
    /// # Errors
    ///
    /// Returns an error if the redirect index is out of bounds.
    pub(crate) fn get_attribute_offset<T: ByteOrder>(image: &Image, index: usize) -> Result<usize> {
        let header = image.header();
        let attribute_offset = header.attribute_offsets();
        let attribute_offset_index = attribute_offset + (index * KEY_SIZE);
        let byte_source = image.byte_source();
        let attribute_offset_bytes =
            byte_source.get_bytes(attribute_offset_index..(attribute_offset_index + KEY_SIZE))?;
        #[expect(clippy::cast_sign_loss)]
        let attribute_index = T::read_i32(&attribute_offset_bytes) as usize;
        Ok(attribute_index)
    }

    /// Retrieves the attributes for a resource at the given index.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or if there is an issue reading the
    /// attributes.
    pub(crate) fn get_attributes(image: &Image, index: usize) -> Result<Attributes> {
        let byte_source = image.byte_source();
        let header = image.header();
        let attribute_data_offset = header.attribute_data_offset();
        let offset = attribute_data_offset + index;
        let limit = byte_source.len()?;
        Attributes::from_bytes(byte_source, offset, limit)
    }

    /// Retrieves a string from the strings table at the given index.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or if there is an issue reading the string.
    pub(crate) fn get_string(image: &'_ Image, index: usize) -> Result<Cow<'_, str>> {
        let header = image.header();
        let strings_offset = header.strings_offset();
        let string_offset = strings_offset + index;
        let byte_source = image.byte_source();
        let bytes = byte_source.get_bytes_to_null(string_offset)?;

        match &bytes {
            Cow::Borrowed(borrowed_bytes) => match std::str::from_utf8(borrowed_bytes) {
                Ok(value) => Ok(Cow::Borrowed(value)),
                Err(_) => Ok(Cow::Owned(
                    String::from_utf8_lossy(borrowed_bytes).into_owned(),
                )),
            },
            Cow::Owned(owned_bytes) => match std::str::from_utf8(owned_bytes) {
                Ok(value) => Ok(Cow::Owned(value.to_string())),
                Err(_) => Ok(Cow::Owned(
                    String::from_utf8_lossy(owned_bytes).into_owned(),
                )),
            },
        }
    }

    /// Retrieves raw data from the data section for the specified attributes.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue reading the data.
    pub(crate) fn get_data<'a>(image: &'a Image, attributes: &Attributes) -> Result<Cow<'a, [u8]>> {
        let header = image.header();
        let data_offset = header.data_offset();
        let offset = data_offset + attributes.offset();
        let byte_source = image.byte_source();

        if attributes.compressed_size() == 0 {
            let length = attributes.uncompressed_size();
            let bytes = byte_source.get_bytes(offset..(offset + length))?;
            return Ok(bytes);
        }

        let length = attributes.compressed_size();
        let _bytes = byte_source.get_bytes(offset..(offset + length))?;
        todo!("compressed jimage support");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::runtime::default_class_loader;

    /// Loads the default Image for testing.
    async fn get_test_image() -> Result<Image> {
        let (java_home, _java_version, _class_loader) =
            default_class_loader().await.expect("java home");
        let path = java_home.join("lib").join("modules");
        let image = Image::from_file(&path)?;
        Ok(image)
    }

    #[test]
    fn test_hash() {
        assert_eq!(Index::hash("a", 0), 97);
        assert_eq!(Index::hash("a", 42), 704_660_095);
        assert_eq!(Index::hash("42", 0), 872_436_206);
        assert_eq!(Index::hash("Hello, World!", 0), 530_056_657);
    }

    #[tokio::test]
    async fn test_get_resource_offset() -> Result<()> {
        let image = get_test_image().await?;
        #[cfg(target_endian = "little")]
        let result = Index::get_resource_offset::<byteorder::LittleEndian>(
            &image,
            "/java.base/java/lang/Object.class",
        );
        #[cfg(target_endian = "big")]
        let result = Index::get_resource_offset::<byteorder::BigEndian>(
            &image,
            "/java.base/java/lang/Object.class",
        );
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_resource_offset_invalid() -> Result<()> {
        let image = get_test_image().await?;
        let result = Index::get_resource_offset::<byteorder::LittleEndian>(&image, "/foo/42");
        assert!(matches!(result, Err(InvalidIndex(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_attribute_offset() -> Result<()> {
        let image = get_test_image().await?;
        #[cfg(target_endian = "little")]
        let result = {
            let resource_offset = Index::get_resource_offset::<byteorder::LittleEndian>(
                &image,
                "/java.base/java/lang/Object.class",
            )?;
            Index::get_attribute_offset::<byteorder::LittleEndian>(&image, resource_offset)
        };
        #[cfg(target_endian = "big")]
        let result = {
            let resource_offset = Index::get_resource_offset::<byteorder::BigEndian>(
                &image,
                "/java.base/java/lang/Object.class",
            )?;
            Index::get_attribute_offset::<byteorder::BigEndian>(&image, resource_offset)
        };
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_attributes() -> Result<()> {
        let image = get_test_image().await?;
        #[cfg(target_endian = "little")]
        let attribute_offset = {
            let resource_offset = Index::get_resource_offset::<byteorder::LittleEndian>(
                &image,
                "/java.base/java/lang/Object.class",
            )?;
            Index::get_attribute_offset::<byteorder::LittleEndian>(&image, resource_offset)?
        };
        #[cfg(target_endian = "big")]
        let attribute_offset = {
            let resource_offset = Index::get_resource_offset::<byteorder::BigEndian>(
                &image,
                "/java.base/java/lang/Object.class",
            )?;
            Index::get_attribute_offset::<byteorder::BigEndian>(&image, resource_offset)?
        };
        let result = Index::get_attributes(&image, attribute_offset);
        assert!(result.is_ok());
        Ok(())
    }
}
