use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::Write;
use tempfile::NamedTempFile;

use crate::header::{IMAGE_MAGIC, IMAGE_MAGIC_INVERTED};

pub(crate) const CLASS_DATA: &[u8] = b"class-data";
pub(crate) const MODULE_INFO_DATA: &[u8] = b"module-info-data";
pub(crate) const MODULE_INFO_NAME: &str = "/java.base/module-info.class";
pub(crate) const EXT_DATA: &[u8] = b"base-data";
pub(crate) const EXT_NAME: &str = "/mod/base.ext";
pub(crate) const STRING_NAME: &str = "/java.base/java/lang/String.class";

const JAVA_BASE_OFFSET: usize = 1;
const JAVA_LANG_OFFSET: usize = 11;
const STRING_OFFSET: usize = 21;
const CLASS_OFFSET: usize = 28;
const MODULE_INFO_OFFSET: usize = 34;
const MOD_OFFSET: usize = 46;
const BASE_OFFSET: usize = 50;
const EXT_OFFSET: usize = 55;

pub(crate) fn write_standard_big_endian_image() -> std::io::Result<NamedTempFile> {
    let bytes = standard_image_bytes::<BigEndian>(IMAGE_MAGIC);
    write_image(&bytes)
}

pub(crate) fn write_standard_little_endian_image() -> std::io::Result<NamedTempFile> {
    let bytes = standard_image_bytes::<LittleEndian>(IMAGE_MAGIC_INVERTED);
    write_image(&bytes)
}

pub(crate) fn write_extensionless_big_endian_image() -> std::io::Result<NamedTempFile> {
    let strings = b"\0java.base\0README\0".to_vec();
    let data = b"readme".to_vec();
    let attributes = attributes(JAVA_BASE_OFFSET, 0, 11, 0, 0, data.len());
    let bytes = image_bytes::<BigEndian>(IMAGE_MAGIC, &[0], &[0], &attributes, &strings, &data);
    write_image(&bytes)
}

fn standard_image_bytes<T: ByteOrder>(magic: [u8; 4]) -> Vec<u8> {
    let strings = b"\0java.base\0java/lang\0String\0class\0module-info\0mod\0base\0ext\0".to_vec();
    let mut data = Vec::new();
    let module_info_data_offset = data.len();
    data.extend_from_slice(MODULE_INFO_DATA);
    let ext_data_offset = data.len();
    data.extend_from_slice(EXT_DATA);
    let class_data_offset = data.len();
    data.extend_from_slice(CLASS_DATA);

    let mut attribute_data = Vec::new();
    let module_info_attribute_offset = attribute_data.len();
    attribute_data.extend(attributes(
        JAVA_BASE_OFFSET,
        0,
        MODULE_INFO_OFFSET,
        CLASS_OFFSET,
        module_info_data_offset,
        MODULE_INFO_DATA.len(),
    ));
    let ext_attribute_offset = attribute_data.len();
    attribute_data.extend(attributes(
        MOD_OFFSET,
        0,
        BASE_OFFSET,
        EXT_OFFSET,
        ext_data_offset,
        EXT_DATA.len(),
    ));
    let class_attribute_offset = attribute_data.len();
    attribute_data.extend(attributes(
        JAVA_BASE_OFFSET,
        JAVA_LANG_OFFSET,
        STRING_OFFSET,
        CLASS_OFFSET,
        class_data_offset,
        CLASS_DATA.len(),
    ));

    image_bytes::<T>(
        magic,
        &[0, 2, -3],
        &[
            module_info_attribute_offset,
            ext_attribute_offset,
            class_attribute_offset,
        ],
        &attribute_data,
        &strings,
        &data,
    )
}

fn image_bytes<T: ByteOrder>(
    magic: [u8; 4],
    redirects: &[i32],
    attribute_offsets: &[usize],
    attribute_data: &[u8],
    strings: &[u8],
    data: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&magic);
    push_u32::<T>(&mut bytes, 0x0001_0000);
    push_u32::<T>(&mut bytes, 0);
    push_u32::<T>(&mut bytes, to_u32(redirects.len()));
    push_u32::<T>(&mut bytes, to_u32(attribute_offsets.len()));
    push_u32::<T>(&mut bytes, to_u32(attribute_data.len()));
    push_u32::<T>(&mut bytes, to_u32(strings.len()));
    for redirect in redirects {
        push_i32::<T>(&mut bytes, *redirect);
    }
    for attribute_offset in attribute_offsets {
        push_u32::<T>(&mut bytes, to_u32(*attribute_offset));
    }
    bytes.extend_from_slice(attribute_data);
    bytes.extend_from_slice(strings);
    bytes.extend_from_slice(data);
    bytes
}

fn attributes(
    module_offset: usize,
    parent_offset: usize,
    base_offset: usize,
    extension_offset: usize,
    data_offset: usize,
    uncompressed_size: usize,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_attribute(&mut bytes, 1, module_offset);
    push_attribute(&mut bytes, 2, parent_offset);
    push_attribute(&mut bytes, 3, base_offset);
    push_attribute(&mut bytes, 4, extension_offset);
    push_attribute(&mut bytes, 5, data_offset);
    push_attribute(&mut bytes, 7, uncompressed_size);
    bytes.push(0);
    bytes
}

fn push_attribute(bytes: &mut Vec<u8>, attribute_type: u8, value: usize) {
    bytes.push((attribute_type << 3) | 3);
    bytes.extend_from_slice(&to_u32(value).to_be_bytes());
}

fn push_i32<T: ByteOrder>(bytes: &mut Vec<u8>, value: i32) {
    let mut buffer = [0; 4];
    T::write_i32(&mut buffer, value);
    bytes.extend_from_slice(&buffer);
}

fn push_u32<T: ByteOrder>(bytes: &mut Vec<u8>, value: u32) {
    let mut buffer = [0; 4];
    T::write_u32(&mut buffer, value);
    bytes.extend_from_slice(&buffer);
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("test fixture value fits in u32")
}

fn write_image(bytes: &[u8]) -> std::io::Result<NamedTempFile> {
    ristretto_test_util::init_wasi_tempdir();
    let mut file = NamedTempFile::new()?;
    file.write_all(bytes)?;
    file.flush()?;
    Ok(file)
}
