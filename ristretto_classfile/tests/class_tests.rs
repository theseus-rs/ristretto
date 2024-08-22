use ristretto_classfile::{ClassFile, Result};
use std::io::Cursor;

pub fn test_class(class_bytes: &[u8]) -> Result<()> {
    let mut original_bytes = Cursor::new(class_bytes.to_vec());
    let class_file = ClassFile::from_bytes(&mut original_bytes)?;
    let mut serde_bytes = Vec::new();
    class_file.to_bytes(&mut serde_bytes)?;
    let _ = class_file.to_string();
    assert_eq!(class_bytes, serde_bytes);
    Ok(())
}

#[test_log::test]
pub fn test_annotations() -> Result<()> {
    test_class(include_bytes!("../../classes/Annotations.class"))
}

#[test_log::test]
pub fn test_constants() -> Result<()> {
    test_class(include_bytes!("../../classes/Constants.class"))
}

#[test_log::test]
pub fn test_expressions() -> Result<()> {
    test_class(include_bytes!("../../classes/Expressions.class"))
}

#[test_log::test]
pub fn test_minimum() -> Result<()> {
    test_class(include_bytes!("../../classes/Minimum.class"))
}

#[test_log::test]
pub fn test_simple() -> Result<()> {
    test_class(include_bytes!("../../classes/Simple.class"))
}
