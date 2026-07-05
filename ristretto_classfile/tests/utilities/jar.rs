use ristretto_classfile::Error::IoError;
use ristretto_classfile::{ClassFile, Result};
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

pub fn verify(jar_bytes: Vec<u8>) -> Result<()> {
    let reader = Cursor::new(jar_bytes);
    let mut archive = ZipArchive::new(reader).map_err(|error| IoError(error.to_string()))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|error| IoError(error.to_string()))?;
        let file_name = file.name().to_string();
        if !Path::new(&file_name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("class"))
        {
            continue;
        }

        let mut out = Vec::new();
        std::io::copy(&mut file, &mut out)?;

        let class_file = ClassFile::from_bytes(&out)?;
        class_file.verify()?;
    }

    Ok(())
}
