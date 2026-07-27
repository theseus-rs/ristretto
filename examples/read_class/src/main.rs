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
    if args.len() != 2 {
        bail!("Usage: {} <file_name>", args[0]);
    }
    let file_name = &args[1];
    let bytes = fs::read(file_name)?;
    let class_file = ClassFile::from_bytes(&bytes)?;
    class_file.verify()?;
    Ok(class_file)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_class() -> Result<()> {
        let args = vec![
            "read_class".to_string(),
            "classes/HelloWorld.class".to_string(),
        ];
        let class_file = read_class(&args)?;

        assert_eq!("HelloWorld", class_file.class_name()?);
        let method_names = class_file
            .methods
            .iter()
            .map(|method| class_file.constant_pool.try_get_utf8(method.name_index))
            .collect::<ristretto_classfile::Result<Vec<_>>>()?;
        assert_eq!(vec!["<init>", "main"], method_names);
        Ok(())
    }

    #[test]
    fn test_read_class_requires_file_name() {
        let args = vec!["read_class".to_string()];
        let error = read_class(&args).expect_err("missing file name should fail");

        assert_eq!("Usage: read_class <file_name>", error.to_string());
    }
}
