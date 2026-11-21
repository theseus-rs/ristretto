#![forbid(unsafe_code)]

use anyhow::{Result, bail};
use ristretto_classfile::ClassFile;
use std::env;
use std::fs;

/// Example that reads a class file and verifies it.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let class_file = read_class(&args)?;
    println!("{class_file}");
    Ok(())
}

fn read_class(args: &[String]) -> Result<ClassFile<'static>> {
    let [_, file_name] = args else {
        let program = args.first().map_or("read_class", String::as_str);
        bail!("Usage: {program} <file_name>");
    };
    let bytes = fs::read(file_name)?;
    let class_file = ClassFile::from_bytes(&bytes)?;
    class_file.verify()?;
    Ok(class_file)
}

#[cfg(test)]
mod test {
    use super::*;
    use ristretto_classfile::CONSTRUCTOR_NAME;

    #[test]
    fn test_read_class() {
        let args = vec![
            "read_class".to_string(),
            "classes/HelloWorld.class".to_string(),
        ];
        let class_file = read_class(&args).unwrap();

        assert_eq!("HelloWorld", class_file.class_name().unwrap());
        let method_names = class_file
            .methods
            .iter()
            .map(|method| class_file.constant_pool.try_get_utf8(method.name_index))
            .collect::<ristretto_classfile::Result<Vec<_>>>()
            .unwrap();
        assert_eq!(vec![CONSTRUCTOR_NAME, "main"], method_names);
    }

    #[test]
    fn test_read_class_requires_file_name() {
        let args = vec!["read_class".to_string()];
        let error = read_class(&args).expect_err("missing file name should fail");

        assert_eq!("Usage: read_class <file_name>", error.to_string());
    }
}
